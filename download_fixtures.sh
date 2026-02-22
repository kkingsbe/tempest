#!/usr/bin/env bash
# =============================================================================
# Tempest Test Fixture Downloader
# =============================================================================
# Downloads 10 curated NEXRAD Archive2 files from the public S3 bucket for use
# as test fixtures in the tempest-decode crate.
#
# Requirements: curl (or wget), optionally aws cli
# Usage: ./download_fixtures.sh [output_dir]
# =============================================================================

set -euo pipefail

OUTPUT_DIR="${1:-tempest-decode/tests/fixtures}"
BUCKET_URL="https://noaa-nexrad-level2.s3.amazonaws.com"
# Note: The bucket is migrating to unidata-nexrad-level2. If the above stops
# working, switch to: https://unidata-nexrad-level2.s3.amazonaws.com
BUCKET_URL_ALT="https://unidata-nexrad-level2.s3.amazonaws.com"

mkdir -p "$OUTPUT_DIR"

echo "Downloading Tempest test fixtures to $OUTPUT_DIR ..."
echo ""

# Helper function with fallback to alternate bucket
download() {
    local s3_path="$1"
    local local_name="$2"
    local description="$3"
    local output_path="$OUTPUT_DIR/$local_name"

    if [ -f "$output_path" ]; then
        echo "  [SKIP] $local_name (already exists)"
        return 0
    fi

    echo "  [$local_name] $description"
    echo "    Fetching: $s3_path"

    if curl -sfL --retry 3 "$BUCKET_URL/$s3_path" -o "$output_path" 2>/dev/null; then
        local size=$(du -h "$output_path" | cut -f1)
        echo "    OK ($size)"
    elif curl -sfL --retry 3 "$BUCKET_URL_ALT/$s3_path" -o "$output_path" 2>/dev/null; then
        local size=$(du -h "$output_path" | cut -f1)
        echo "    OK ($size) [from alt bucket]"
    else
        echo "    FAILED - could not download from either bucket"
        rm -f "$output_path"
        return 1
    fi
}

echo "=========================================="
echo " Fixture 1: Standard VCP 215 Volume Scan"
echo "=========================================="
echo "Source: KLWX (Sterling, VA) - 2024-06-15"
echo "A typical precipitation-mode scan with all modern moments (REF, VEL, SW, ZDR, CC, KDP)."
echo "VCP 215 is the most common operational VCP — this is the baseline decode test."
echo ""
download \
    "2024/06/15/KLWX/KLWX20240615_120052_V06" \
    "VCP_215_KLWX_20240615.ar2v" \
    "VCP 215, KLWX, 2024-06-15 12:00Z"

echo ""
echo "=========================================="
echo " Fixture 2: VCP 35 (Clear-Air Mode)"
echo "=========================================="
echo "Source: KLWX (Sterling, VA) - 2024-01-15"
echo "Clear-air mode has a slower scan rate, fewer elevation tilts, and 1.0° azimuth"
echo "resolution (vs 0.5° super-res). Tests the decoder's handling of different VCP"
echo "scan strategies and lower-resolution data."
echo ""
download \
    "2024/01/15/KLWX/KLWX20240115_060044_V06" \
    "VCP_35_KLWX_20240115.ar2v" \
    "VCP 35, KLWX, 2024-01-15 06:00Z"

echo ""
echo "=========================================="
echo " Fixture 3: VCP 12 (Severe Weather)"
echo "=========================================="
echo "Source: KTLX (Oklahoma City, OK) - 2013-05-20"
echo "The May 20, 2013 Moore, OK EF5 tornado. KTLX was running VCP 12 (severe weather"
echo "mode) with the fastest scan rate and maximum number of elevation tilts. This scan"
echo "should contain strong reflectivity gradients and intense velocity signatures."
echo ""
download \
    "2013/05/20/KTLX/KTLX20130520_200120_V06.gz" \
    "VCP_12_KTLX_20130520.ar2v.gz" \
    "VCP 12, KTLX, 2013-05-20 20:01Z (Moore EF5 tornado)"

echo ""
echo "=========================================="
echo " Fixture 4: Super-Resolution Scan (0.5° azimuth)"
echo "=========================================="
echo "Source: KTLX (Oklahoma City, OK) - 2024-04-27"
echo "Modern post-2008 data with 0.5° azimuth resolution and 250m gate spacing at"
echo "lower tilts (super-resolution). Tests that the decoder correctly handles the"
echo "higher-resolution data format with twice as many radials per sweep."
echo ""
download \
    "2024/04/27/KTLX/KTLX20240427_210352_V06" \
    "SuperRes_KTLX_20240427.ar2v" \
    "Super-res, KTLX, 2024-04-27 21:03Z"

echo ""
echo "=========================================="
echo " Fixture 5: Legacy Message Type 1 (Pre-2008)"
echo "=========================================="
echo "Source: KTLX (Oklahoma City, OK) - 2005-05-09"
echo "Before 2008, NEXRAD used Message Type 1 (fixed 2432-byte messages) instead of"
echo "the modern Message Type 31. These files only contain REF, VEL, and SW (no dual-pol"
echo "moments). The file naming convention also differs — no _V06 suffix, just .gz."
echo "Critical for backward compatibility with the full 1991-2008 archive."
echo ""
download \
    "2005/05/09/KTLX/KTLX20050509_180040.gz" \
    "Legacy_KTLX_20050509.ar2v.gz" \
    "Message Type 1, KTLX, 2005-05-09 18:00Z"

echo ""
echo "=========================================="
echo " Fixture 6: Bzip2-Compressed Volume Scan"
echo "=========================================="
echo "Source: KFSX (Flagstaff, AZ) - 2024-07-15"
echo "Archive2 files use bzip2 compression for each LDM record (the data blocks between"
echo "the volume header and the individual messages). The decoder must detect and"
echo "decompress each LDM record. This is separate from the .gz outer compression that"
echo "some files have — this tests the *internal* bzip2 compression that all modern"
echo "Archive2 files use."
echo ""
download \
    "2024/07/15/KFSX/KFSX20240715_200018_V06" \
    "Bzip2_KFSX_20240715.ar2v.bz2" \
    "Bzip2-compressed LDM records, KFSX, 2024-07-15 20:00Z"

echo ""
echo "=========================================="
echo " Fixture 7: Truncated / Corrupt File"
echo "=========================================="
echo "This fixture is GENERATED, not downloaded. We take a valid file and truncate it"
echo "mid-stream to simulate a corrupt/incomplete download."
echo ""
echo "  [Truncated.ar2v] Generating from fixture 01..."
if [ -f "$OUTPUT_DIR/VCP_215_KLWX_20240615.ar2v" ]; then
    # Take the first 50KB of the standard file — enough for the volume header
    # and start of data, but truncated mid-message
    head -c 51200 "$OUTPUT_DIR/VCP_215_KLWX_20240615.ar2v" > "$OUTPUT_DIR/Truncated.ar2v"
    echo "    OK (50 KB truncated from fixture 01)"
else
    echo "    DEFERRED - will be generated after fixture 01 is downloaded"
fi

echo ""
echo "=========================================="
echo " Fixture 8: Volume Scan with Missing Moments"
echo "=========================================="
echo "Source: KLWX (Sterling, VA) - 2012-06-01"
echo "Shortly after the dual-pol upgrade rollout (2011-2012), some stations had"
echo "intermittent issues with dual-pol moments. Files from this transitional period"
echo "may contain REF/VEL/SW but have missing or incomplete ZDR/CC/KDP data. The"
echo "decoder must handle partial moment availability gracefully."
echo ""
download \
    "2012/06/01/KLWX/KLWX20120601_120119_V06" \
    "MissingMoments_KLWX_20120601.ar2v" \
    "Partial moments, KLWX, 2012-06-01 12:01Z (early dual-pol era)"

echo ""
echo "=========================================="
echo " Fixture 9: High-Altitude Station (KFSX)"
echo "=========================================="
echo "Source: KFSX (Flagstaff, AZ) - 2024-08-15"
echo "KFSX sits at 7,514 feet (2,290m) elevation — one of the highest NEXRAD stations."
echo "The high altitude affects beam height calculations significantly. At 100km range,"
echo "the lowest tilt's center beam height is substantially different from a sea-level"
echo "radar. Tests the beam height / projection correction logic."
echo ""
download \
    "2024/08/15/KFSX/KFSX20240815_200021_V06" \
    "HighAlt_KFSX_20240815.ar2v" \
    "High-altitude station, KFSX, 2024-08-15 20:00Z (7514 ft elev)"

echo ""
echo "=========================================="
echo " Fixture 10: Strong Velocity Aliasing"
echo "=========================================="
echo "Source: KTLX (Oklahoma City, OK) - 2013-05-31"
echo "The May 31, 2013 El Reno, OK EF5 tornado — the widest tornado ever recorded"
echo "(2.6 miles). This event produced extreme velocity signatures that exceeded the"
echo "Nyquist velocity, causing aliasing (velocities wrap around from +max to -max)."
echo "The velocity data will show abrupt sign reversals that are artifacts of aliasing,"
echo "not actual wind direction changes. Tests that the decoder faithfully preserves"
echo "these aliased values without misinterpreting them."
echo ""
download \
    "2013/05/31/KTLX/KTLX20130531_232515_V06.gz" \
    "VelocityAlias_KTLX_20130531.ar2v.gz" \
    "Velocity aliasing, KTLX, 2013-05-31 23:25Z (El Reno EF5)"

echo ""
echo "=========================================="
echo " Post-Download Steps"
echo "=========================================="

# Decompress .gz files if gzip is available
echo ""
echo "Decompressing .gz fixtures..."
for f in "$OUTPUT_DIR"/*.gz; do
    if [ -f "$f" ]; then
        base="${f%.gz}"
        if [ ! -f "$base" ]; then
            echo "  Decompressing $(basename "$f")..."
            gzip -dk "$f" 2>/dev/null || gunzip -k "$f" 2>/dev/null || {
                echo "    WARNING: Could not decompress $f — install gzip"
            }
        fi
    fi
done

echo ""
echo "=========================================="
echo " Fixture Summary"
echo "=========================================="
echo ""
echo "Downloaded fixtures:"
ls -lhS "$OUTPUT_DIR"/ 2>/dev/null | grep -v "^total" || true
echo ""
echo "Total size: $(du -sh "$OUTPUT_DIR" | cut -f1)"
echo ""
echo "=========================================="
echo " IMPORTANT: Next Steps"
echo "=========================================="
echo ""
echo "1. Add these files to Git LFS:"
echo "   git lfs track 'test-fixtures/*.ar2v*'"
echo "   git add .gitattributes test-fixtures/"
echo ""
echo "2. Generate metadata sidecars by running the verification script"
echo "   (or use Py-ART to cross-reference expected values):"
echo ""
echo "   pip install arm-pyart"
echo "   python generate_fixture_metadata.py"
echo ""
echo "3. For Fixture 07 (truncated), if it wasn't generated above,"
echo "   run: head -c 51200 test-fixtures/01_vcp215_standard.ar2v > test-fixtures/07_truncated.ar2v"
echo ""
echo "Done!"