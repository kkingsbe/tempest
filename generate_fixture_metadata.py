#!/usr/bin/env python3
"""
Generate JSON metadata sidecar files for Tempest test fixtures.

Uses Py-ART to decode each Archive2 file and extract the properties needed
for golden-value testing in the tempest-decode crate.

Requirements: pip install arm-pyart numpy

Usage: python generate_fixture_metadata.py [fixtures_dir]
"""

import json
import os
import sys
import traceback
import numpy as np

try:
    import pyart
except ImportError:
    print("ERROR: arm-pyart is required. Install with: pip install arm-pyart")
    sys.exit(1)


FIXTURES_DIR = sys.argv[1] if len(sys.argv) > 1 else "./test-fixtures"

# Map fixture filenames to their expected properties (what we know a priori)
FIXTURE_EXPECTATIONS = {
    "01_vcp215_standard.ar2v": {
        "description": "Standard VCP 215 volume scan",
        "station": "KLWX",
        "expected_vcp": 215,
        "expected_message_type": 31,
        "notes": "Baseline test. All modern moments should be present.",
    },
    "02_vcp35_clearair.ar2v": {
        "description": "VCP 35 clear-air mode",
        "station": "KLWX",
        "expected_vcp": 35,
        "expected_message_type": 31,
        "notes": "Clear-air mode. Fewer tilts, 1.0 deg azimuth resolution.",
    },
    "03_vcp12_severe.ar2v": {
        "description": "VCP 12 severe weather mode (Moore EF5 tornado)",
        "station": "KTLX",
        "expected_vcp": 12,
        "expected_message_type": 31,
        "notes": "May 20, 2013 Moore EF5 tornado. High tilt count.",
    },
    "04_superres.ar2v": {
        "description": "Super-resolution scan (0.5 deg azimuth)",
        "station": "KTLX",
        "expected_vcp": None,  # Will be detected
        "expected_message_type": 31,
        "notes": "0.5 deg azimuth, 250m gate spacing at lower tilts.",
    },
    "05_legacy_msg1.ar2v": {
        "description": "Legacy Message Type 1 (pre-2008)",
        "station": "KTLX",
        "expected_vcp": None,
        "expected_message_type": 1,
        "notes": "Pre-2008 format. Only REF, VEL, SW. No dual-pol.",
    },
    "06_bzip2_compressed.ar2v": {
        "description": "Bzip2-compressed LDM records",
        "station": "KFSX",
        "expected_vcp": None,
        "expected_message_type": 31,
        "notes": "Tests internal bzip2 decompression of LDM records.",
    },
    "07_truncated.ar2v": {
        "description": "Truncated / corrupt file",
        "station": "KLWX",
        "expected_vcp": None,
        "expected_message_type": None,
        "notes": "First 50KB of fixture 01. Should produce DecodeError::Truncated.",
        "expect_failure": True,
    },
    "08_missing_moments.ar2v": {
        "description": "Volume scan with missing/partial moments",
        "station": "KLWX",
        "expected_vcp": None,
        "expected_message_type": 31,
        "notes": "Early dual-pol era. Some moments may be missing.",
    },
    "09_high_altitude_kfsx.ar2v": {
        "description": "High-altitude station (KFSX, 7514 ft)",
        "station": "KFSX",
        "expected_vcp": None,
        "expected_message_type": 31,
        "notes": "Tests beam height correction at high station elevation.",
    },
    "10_velocity_aliasing.ar2v": {
        "description": "Strong velocity aliasing (El Reno EF5 tornado)",
        "station": "KTLX",
        "expected_vcp": None,
        "expected_message_type": 31,
        "notes": "May 31, 2013 El Reno EF5. Extreme velocities cause aliasing.",
    },
}

# Sample points for golden-value testing: (sweep_index, azimuth_deg, range_km)
SAMPLE_POINTS = [
    (0, 0.0, 10.0),
    (0, 90.0, 25.0),
    (0, 180.0, 50.0),
    (0, 270.0, 75.0),
    (0, 45.0, 100.0),
    (1, 0.0, 20.0),
    (1, 180.0, 40.0),
]


def find_nearest_ray_gate(radar, sweep_idx, target_azimuth, target_range_km):
    """Find the nearest ray and gate index for a given azimuth and range."""
    sweep_start = radar.sweep_start_ray_index["data"][sweep_idx]
    sweep_end = radar.sweep_end_ray_index["data"][sweep_idx]

    azimuths = radar.azimuth["data"][sweep_start : sweep_end + 1]
    az_idx = int(np.argmin(np.abs(azimuths - target_azimuth)))
    ray_idx = sweep_start + az_idx

    ranges_km = radar.range["data"] / 1000.0
    gate_idx = int(np.argmin(np.abs(ranges_km - target_range_km)))

    return ray_idx, gate_idx, float(azimuths[az_idx]), float(ranges_km[gate_idx])


def extract_golden_values(radar, sweep_idx, target_azimuth, target_range_km):
    """Extract field values at a specific sweep/azimuth/range for golden-value testing."""
    try:
        ray_idx, gate_idx, actual_az, actual_range = find_nearest_ray_gate(
            radar, sweep_idx, target_azimuth, target_range_km
        )
    except (IndexError, ValueError):
        return None

    values = {
        "sweep_index": sweep_idx,
        "target_azimuth_deg": target_azimuth,
        "target_range_km": target_range_km,
        "actual_azimuth_deg": round(actual_az, 2),
        "actual_range_km": round(actual_range, 3),
        "ray_index": int(ray_idx),
        "gate_index": int(gate_idx),
        "fields": {},
    }

    for field_name in radar.fields:
        val = radar.fields[field_name]["data"][ray_idx, gate_idx]
        if np.ma.is_masked(val):
            values["fields"][field_name] = None  # No data / below threshold
        else:
            values["fields"][field_name] = round(float(val), 4)

    return values


def process_fixture(filepath, expectations):
    """Process a single fixture file and generate its metadata."""
    filename = os.path.basename(filepath)
    print(f"\n{'='*60}")
    print(f"Processing: {filename}")
    print(f"{'='*60}")

    if expectations.get("expect_failure"):
        print("  [SKIP] This fixture is expected to fail decoding.")
        return {
            "filename": filename,
            "description": expectations["description"],
            "notes": expectations["notes"],
            "expect_failure": True,
            "expected_error": "DecodeError::Truncated",
        }

    try:
        # Read with Py-ART
        station = expectations.get("station")
        radar = pyart.io.read_nexrad_archive(filepath, station=station)
    except Exception as e:
        print(f"  ERROR reading file: {e}")
        traceback.print_exc()
        return {
            "filename": filename,
            "description": expectations["description"],
            "error": str(e),
            "notes": "Failed to decode — investigate manually.",
        }

    # Extract basic metadata
    nsweeps = radar.nsweeps
    nrays = radar.nrays
    ngates = radar.ngates
    fields = list(radar.fields.keys())

    elevations = [round(float(e), 2) for e in radar.fixed_angle["data"]]

    sweep_info = []
    for i in range(nsweeps):
        start = int(radar.sweep_start_ray_index["data"][i])
        end = int(radar.sweep_end_ray_index["data"][i])
        n_radials = end - start + 1
        sweep_info.append(
            {
                "sweep_index": i,
                "elevation_deg": elevations[i],
                "num_radials": n_radials,
                "num_gates": int(ngates),
            }
        )

    # Extract site info
    site_lat = round(float(radar.latitude["data"][0]), 6)
    site_lon = round(float(radar.longitude["data"][0]), 6)
    site_alt = round(float(radar.altitude["data"][0]), 2)

    # Time info
    time_start = str(radar.time["units"]).replace("seconds since ", "")

    # Nyquist velocity (for velocity aliasing testing)
    nyquist = None
    if hasattr(radar.instrument_parameters, "__contains__") or True:
        try:
            nv = radar.instrument_parameters.get("nyquist_velocity")
            if nv is not None:
                nyquist = round(float(np.mean(nv["data"])), 2)
        except Exception:
            pass

    # Golden values at sample points
    golden_values = []
    for sweep_idx, az, rng in SAMPLE_POINTS:
        if sweep_idx < nsweeps:
            gv = extract_golden_values(radar, sweep_idx, az, rng)
            if gv is not None:
                golden_values.append(gv)

    # VCP detection (Py-ART doesn't directly expose VCP, but we can infer from sweep count)
    metadata = {
        "filename": filename,
        "description": expectations["description"],
        "station": expectations.get("station"),
        "notes": expectations.get("notes", ""),
        "site": {
            "latitude": site_lat,
            "longitude": site_lon,
            "altitude_m": site_alt,
        },
        "time_start": time_start,
        "num_sweeps": nsweeps,
        "num_rays_total": int(nrays),
        "num_gates": int(ngates),
        "elevation_angles_deg": elevations,
        "available_fields": fields,
        "sweep_details": sweep_info,
        "nyquist_velocity_mps": nyquist,
        "expected_vcp": expectations.get("expected_vcp"),
        "expected_message_type": expectations.get("expected_message_type"),
        "golden_values": golden_values,
    }

    # Print summary
    print(f"  Station:    {expectations.get('station')}")
    print(f"  Time:       {time_start}")
    print(f"  Sweeps:     {nsweeps}")
    print(f"  Rays:       {nrays}")
    print(f"  Gates:      {ngates}")
    print(f"  Fields:     {', '.join(fields)}")
    print(f"  Elevations: {elevations[:5]}{'...' if len(elevations) > 5 else ''}")
    print(f"  Nyquist:    {nyquist} m/s")
    print(f"  Golden pts: {len(golden_values)}")

    return metadata


def main():
    if not os.path.isdir(FIXTURES_DIR):
        print(f"ERROR: Fixtures directory not found: {FIXTURES_DIR}")
        print("Run download_fixtures.sh first.")
        sys.exit(1)

    results = {}

    for fixture_name, expectations in FIXTURE_EXPECTATIONS.items():
        filepath = os.path.join(FIXTURES_DIR, fixture_name)

        if not os.path.isfile(filepath):
            # Check for decompressed version (without .gz)
            alt_path = filepath.replace(".ar2v.gz", ".ar2v")
            if os.path.isfile(alt_path):
                filepath = alt_path
            else:
                print(f"\n  [MISSING] {fixture_name} — skipping")
                continue

        metadata = process_fixture(filepath, expectations)
        results[fixture_name] = metadata

        # Write individual JSON sidecar
        json_path = filepath.rsplit(".", 1)[0] + ".json"
        with open(json_path, "w") as f:
            json.dump(metadata, f, indent=2)
        print(f"  Wrote: {os.path.basename(json_path)}")

    # Write combined metadata file
    combined_path = os.path.join(FIXTURES_DIR, "fixtures_manifest.json")
    with open(combined_path, "w") as f:
        json.dump(results, f, indent=2)
    print(f"\n{'='*60}")
    print(f"Wrote combined manifest: {combined_path}")
    print(f"Processed {len(results)} fixtures.")


if __name__ == "__main__":
    main()