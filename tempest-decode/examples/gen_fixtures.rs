//! Synthetic NEXRAD test fixture generator.
//!
//! This module generates binary test fixtures that follow the NEXRAD Archive2 format.
//! Run with: cargo run --package tempest-decode --example gen_fixtures

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// Generate a synthetic Message 31 radial data block
fn create_msg31_radial(
    station_id: &[u8; 4],
    vcp: u16,
    num_radials: usize,
    gates_per_radial: u16,
    has_vel: bool,
    has_sw: bool,
) -> Vec<u8> {
    let mut data = Vec::new();

    // Message header starts at offset 0
    // We'll fill in message size at the end

    // Message Type: 31 (0x001F)
    data.extend_from_slice(&0x001fu16.to_be_bytes());

    // Date: MJD 60500 (approximately 2025-01-15)
    data.extend_from_slice(&60500u16.to_be_bytes());

    // Time: 12:00:00.000 = 43200000 ms
    data.extend_from_slice(&43200000u32.to_be_bytes());

    // Station ID (4 bytes)
    data.extend_from_slice(station_id);

    // Volume scan number: 1
    data.extend_from_slice(&1u16.to_be_bytes());

    // VCP: provided as parameter
    data.extend_from_slice(&vcp.to_be_bytes());

    // Now add radial data - start with sweep indicator (1 = new sweep)
    data.push(1); // Sweep flag

    // Elevation: 0.5 degrees
    data.extend_from_slice(&0.5f32.to_be_bytes());

    // Number of radials
    data.extend_from_slice(&(num_radials as u16).to_be_bytes());

    // For each radial, add azimuth and moment data
    for i in 0..num_radials {
        let azimuth = (i as f32) * (360.0 / num_radials as f32);

        // Radial header
        // Radial number (1-based)
        data.extend_from_slice(&((i + 1) as u16).to_be_bytes());
        // Calibration (0)
        data.extend_from_slice(&0u32.to_be_bytes());
        // Azimuth angle
        data.extend_from_slice(&azimuth.to_be_bytes());
        // Radial status (0 = normal)
        data.push(0);
        // Elevation angle
        data.extend_from_slice(&0.5f32.to_be_bytes());
        // Elevation number
        data.push(1);
        // Number of data blocks (REF always present, optionally VEL and SW)
        let num_blocks = 1 + if has_vel { 1 } else { 0 } + if has_sw { 1 } else { 0 };
        data.push(num_blocks);

        // Add REF moment block
        let ref_data = create_ref_moment(gates_per_radial);
        data.extend_from_slice(&ref_data);

        // Add VEL moment block if requested
        if has_vel {
            let vel_data = create_vel_moment(gates_per_radial);
            data.extend_from_slice(&vel_data);
        }

        // Add SW moment block if requested
        if has_sw {
            let sw_data = create_sw_moment(gates_per_radial);
            data.extend_from_slice(&sw_data);
        }
    }

    // Now calculate message size (total bytes - 4 for the size field itself)
    let msg_size = (data.len()) as u32;

    // Prepend message size
    let mut full_msg = Vec::new();
    full_msg.extend_from_slice(&msg_size.to_be_bytes());
    full_msg.extend_from_slice(&data);

    full_msg
}

/// Create REF (Reflectivity) moment block
fn create_ref_moment(gate_count: u16) -> Vec<u8> {
    let mut block = Vec::new();

    // Moment name: "REF"
    block.extend_from_slice(b"REF");

    // Reserved
    block.push(0);

    // Data word size: 8 bits
    block.push(8);

    // Scale: 0.5 (stored as raw byte)
    block.push(0);

    // Offset: 0 (stored as raw byte, but we'll use standard formula)
    block.push(0);

    // Reserved
    block.push(0);

    // Number of gates
    block.extend_from_slice(&gate_count.to_be_bytes());

    // Range to first gate: 5000m (standard for REF)
    block.extend_from_slice(&5000.0f32.to_be_bytes());

    // Gate spacing: 250m (standard for REF)
    block.extend_from_slice(&250u16.to_be_bytes());

    // Generate reflectivity data (simulate a storm pattern)
    // Using standard encoding: dBZ = (value * 0.5) - 32.0
    // So value = (dBZ + 32.0) / 0.5
    for i in 0..gate_count {
        // Create a simulated storm pattern
        let gate_range = i as f32 * 250.0;
        let d_bz = if gate_range < 50000.0 {
            // Storm core - high reflectivity
            55.0 - (gate_range / 2000.0)
        } else {
            // Light precipitation
            20.0
        };

        // Convert to NEXRAD value
        let raw_value = ((d_bz + 32.0) / 0.5).round() as u8;
        block.push(raw_value);
    }

    block
}

/// Create VEL (Velocity) moment block
fn create_vel_moment(gate_count: u16) -> Vec<u8> {
    let mut block = Vec::new();

    // Moment name: "VEL"
    block.extend_from_slice(b"VEL");

    // Reserved
    block.push(0);

    // Data word size: 8 bits
    block.push(8);

    // Scale: 2.0
    block.push(0);

    // Offset: 0
    block.push(0);

    // Reserved
    block.push(0);

    // Number of gates
    block.extend_from_slice(&gate_count.to_be_bytes());

    // Range to first gate: 5000m (same as REF)
    block.extend_from_slice(&5000.0f32.to_be_bytes());

    // Gate spacing: 250m
    block.extend_from_slice(&250u16.to_be_bytes());

    // Generate velocity data
    // Standard encoding: m/s = (value * 2.0) - 64.0
    // So value = (m/s + 64.0) / 2.0
    for i in 0..gate_count {
        // Create a simulated velocity pattern (outbound then inbound)
        let gate_range = i as f32 * 250.0;
        let velocity = if gate_range < 25000.0 {
            // Strong outbound in center
            -30.0
        } else if gate_range < 50000.0 {
            // Weaker inbound
            15.0
        } else {
            // No data
            0.0
        };

        // Convert to NEXRAD value
        let raw_value = ((velocity + 64.0_f32) / 2.0_f32).round() as u8;
        block.push(raw_value);
    }

    block
}

/// Create SW (Spectrum Width) moment block
fn create_sw_moment(gate_count: u16) -> Vec<u8> {
    let mut block = Vec::new();

    // Moment name: "SW"
    block.extend_from_slice(b"SW ");

    // Reserved
    block.push(0);

    // Data word size: 8 bits
    block.push(8);

    // Scale: 0.5
    block.push(0);

    // Offset: 0
    block.push(0);

    // Reserved
    block.push(0);

    // Number of gates
    block.extend_from_slice(&gate_count.to_be_bytes());

    // Range to first gate: 5000m
    block.extend_from_slice(&5000.0f32.to_be_bytes());

    // Gate spacing: 250m
    block.extend_from_slice(&250u16.to_be_bytes());

    // Generate spectrum width data
    // Standard encoding: m/s = value * 0.5
    // So value = m/s / 0.5
    for i in 0..gate_count {
        let gate_range = i as f32 * 250.0;
        let sw = if gate_range < 50000.0 {
            // Moderate spectrum width in precipitation
            8.0
        } else {
            2.0
        };

        // Convert to NEXRAD value
        let raw_value = (sw / 0.5_f32).round() as u8;
        block.push(raw_value);
    }

    block
}

/// Create a fixture with specific VCP characteristics
fn create_vcp_fixture(
    name: &str,
    station_id: &[u8; 4],
    vcp: u16,
    description: &str,
    num_radials: usize,
    gates_per_radial: u16,
    moments: (bool, bool),
) -> (Vec<u8>, String) {
    let (has_vel, has_sw) = moments;
    let data = create_msg31_radial(
        station_id,
        vcp,
        num_radials,
        gates_per_radial,
        has_vel,
        has_sw,
    );

    let metadata = format!(
        r#"{{
  "name": "{}",
  "description": "{}",
  "station_id": "{}",
  "vcp": {},
  "message_type": 31,
  "num_radials": {},
  "gates_per_radial": {},
  "has_reflectivity": true,
  "has_velocity": {},
  "has_spectrum_width": {},
  "range_to_first_gate": 5000,
  "gate_spacing": 250,
  "elevation_angle": 0.5
}}"#,
        name,
        description,
        String::from_utf8_lossy(station_id),
        vcp,
        num_radials,
        gates_per_radial,
        has_vel,
        has_sw
    );

    (data, metadata)
}

fn main() {
    let fixtures_dir = PathBuf::from("/workspace/tempest-decode/tests/fixtures");

    // Fixture 1: VCP 215 - Standard clear-air mode (360 radials, 100 gates, all moments)
    let (data1, meta1) = create_vcp_fixture(
        "VCP 215 Clear Air",
        b"KTLX",
        215,
        "Standard VCP 215 clear-air mode volume scan with REF, VEL, and SW moments",
        360,
        100,
        (true, true),
    );

    let mut f1 = File::create(fixtures_dir.join("vcp215_clear_air.bin")).unwrap();
    f1.write_all(&data1).unwrap();

    let mut f1_meta = File::create(fixtures_dir.join("vcp215_clear_air.json")).unwrap();
    f1_meta.write_all(meta1.as_bytes()).unwrap();

    println!("Created vcp215_clear_air.bin ({} bytes)", data1.len());

    // Fixture 2: VCP 35 - Alternate clear-air mode
    let (data2, meta2) = create_vcp_fixture(
        "VCP 35 Clear Air",
        b"KOKC",
        35,
        "VCP 35 clear-air mode with reduced elevation cuts",
        360,
        100,
        (true, true),
    );

    let mut f2 = File::create(fixtures_dir.join("vcp35_clear_air.bin")).unwrap();
    f2.write_all(&data2).unwrap();

    let mut f2_meta = File::create(fixtures_dir.join("vcp35_clear_air.json")).unwrap();
    f2_meta.write_all(meta2.as_bytes()).unwrap();

    println!("Created vcp35_clear_air.bin ({} bytes)", data2.len());

    // Fixture 3: VCP 12 - Severe weather mode
    let (data3, meta3) = create_vcp_fixture(
        "VCP 12 Severe Weather",
        b"KICT",
        12,
        "VCP 12 severe weather mode with rapid scanning",
        360,
        100,
        (true, true),
    );

    let mut f3 = File::create(fixtures_dir.join("vcp12_severe_weather.bin")).unwrap();
    f3.write_all(&data3).unwrap();

    let mut f3_meta = File::create(fixtures_dir.join("vcp12_severe_weather.json")).unwrap();
    f3_meta.write_all(meta3.as_bytes()).unwrap();

    println!("Created vcp12_severe_weather.bin ({} bytes)", data3.len());

    // Fixture 4: Super-resolution (0.5 degree azimuth, 250m gates)
    let (data4, meta4) = create_vcp_fixture(
        "Super Resolution",
        b"KTLX",
        215,
        "Super-resolution data with 0.5 degree azimuthal resolution and 250m gate spacing",
        720, // Double radials for 0.5 degree resolution
        200, // More gates for super-resolution
        (true, true),
    );

    let mut f4 = File::create(fixtures_dir.join("super_resolution.bin")).unwrap();
    f4.write_all(&data4).unwrap();

    let mut f4_meta = File::create(fixtures_dir.join("super_resolution.json")).unwrap();
    f4_meta.write_all(meta4.as_bytes()).unwrap();

    println!("Created super_resolution.bin ({} bytes)", data4.len());

    // Fixture 5: REF only (missing VEL and SW moments)
    let (data5, meta5) = create_vcp_fixture(
        "Reflectivity Only",
        b"KTLX",
        215,
        "Volume scan with only reflectivity moment (VEL and SW missing)",
        360,
        100,
        (false, false), // No VEL, No SW
    );

    let mut f5 = File::create(fixtures_dir.join("reflectivity_only.bin")).unwrap();
    f5.write_all(&data5).unwrap();

    let mut f5_meta = File::create(fixtures_dir.join("reflectivity_only.json")).unwrap();
    f5_meta.write_all(meta5.as_bytes()).unwrap();

    println!("Created reflectivity_only.bin ({} bytes)", data5.len());

    // Fixture 6: High-altitude station
    let (data6, meta6) = create_vcp_fixture(
        "High Altitude Station",
        b"KPUB",
        215,
        "High altitude radar station (Pueblo, CO) with standard VCP 215",
        360,
        100,
        (true, true),
    );

    let mut f6 = File::create(fixtures_dir.join("high_altitude_station.bin")).unwrap();
    f6.write_all(&data6).unwrap();

    let mut f6_meta = File::create(fixtures_dir.join("high_altitude_station.json")).unwrap();
    f6_meta.write_all(meta6.as_bytes()).unwrap();

    println!("Created high_altitude_station.bin ({} bytes)", data6.len());

    // Fixture 7: Strong velocity aliasing (near Nyquist velocity)
    let (_data7, meta7) = create_vcp_fixture(
        "Velocity Aliasing",
        b"KTLX",
        215,
        "Volume scan with strong velocity aliasing near Nyquist velocity",
        360,
        100,
        (true, true),
    );

    // Modify VEL data to show strong aliasing
    // For simplicity, we'll regenerate with special velocity pattern
    let mut data7_mod = create_msg31_radial(b"KTLX", 215, 360, 100, true, true);

    // Find and modify velocity data to show aliasing (values near 0 and 255)
    let mut pos = 24; // After header
    while pos + 16 < data7_mod.len() {
        if &data7_mod[pos..pos + 3] == b"VEL" {
            // Found VEL block, now modify data
            let gate_count = u16::from_be_bytes([data7_mod[pos + 8], data7_mod[pos + 9]]) as usize;
            let data_start = pos + 16;
            for i in 0..gate_count.min(100) {
                // Alternate between max and min to simulate aliasing
                if i % 10 < 5 {
                    data7_mod[data_start + i] = 254; // Near +64 m/s
                } else {
                    data7_mod[data_start + i] = 2; // Near -64 m/s
                }
            }
            break;
        }
        pos += 1;
    }

    let mut f7 = File::create(fixtures_dir.join("velocity_aliasing.bin")).unwrap();
    f7.write_all(&data7_mod).unwrap();

    let mut f7_meta = File::create(fixtures_dir.join("velocity_aliasing.json")).unwrap();
    f7_meta.write_all(meta7.as_bytes()).unwrap();

    println!("Created velocity_aliasing.bin ({} bytes)", data7_mod.len());

    // Create the legacy synthetic_msg31.bin for backward compatibility with existing tests
    let legacy_data = create_msg31_radial(b"KTLX", 215, 360, 100, true, true);
    let mut f_legacy = File::create(fixtures_dir.join("synthetic_msg31.bin")).unwrap();
    f_legacy.write_all(&legacy_data).unwrap();

    let legacy_meta = r#"{
  "name": "Synthetic Message 31",
  "description": "Legacy synthetic test fixture for backward compatibility",
  "station_id": "KTLX",
  "vcp": 215,
  "message_type": 31,
  "num_radials": 360,
  "gates_per_radial": 100,
  "has_reflectivity": true,
  "has_velocity": true,
  "has_spectrum_width": true,
  "range_to_first_gate": 5000,
  "gate_spacing": 250,
  "elevation_angle": 0.5
}"#;

    let mut f_legacy_meta = File::create(fixtures_dir.join("synthetic_msg31.json")).unwrap();
    f_legacy_meta.write_all(legacy_meta.as_bytes()).unwrap();

    println!(
        "Created synthetic_msg31.bin ({} bytes) - legacy fixture",
        legacy_data.len()
    );

    println!("\nAll fixtures generated successfully!");
}
