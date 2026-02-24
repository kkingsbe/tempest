#!/bin/bash
# Coverage Ratchet Script
# Compares current coverage against baseline and fails if coverage drops by >1.5%
#
# Usage:
#   ./coverage-ratchet.sh              # Compare against baseline
#   ./coverage-ratchet.sh --baseline    # Set/update baseline coverage
#   ./coverage-ratchet.sh --update-baseline  # Update baseline if coverage improved
#
# Exit codes:
#   0 - Coverage is within threshold (>= baseline - 1.5%)
#   1 - Coverage dropped by more than 1.5%

set -euo pipefail

# Constants
BASELINE_FILE=".coverage_baseline"
THRESHOLD=1.5

# Parse arguments
UPDATE_BASELINE=false
SET_BASELINE=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --update-baseline)
            UPDATE_BASELINE=true
            shift
            ;;
        --baseline)
            SET_BASELINE=true
            shift
            ;;
        *)
            echo "Unknown option: $1" >&2
            echo "Usage: $0 [--baseline|--update-baseline]" >&2
            exit 1
            ;;
    esac
done

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

# Function to get current coverage
get_current_coverage() {
    echo "Running cargo tarpaulin to get current coverage..."
    
    # Run tarpaulin and capture JSON output
    local json_output
    json_output=$(cargo tarpaulin --line --config tarpaulin.toml --out Json 2>&1)
    
    # Check if tarpaulin ran successfully
    if [[ $? -ne 0 ]]; then
        echo "Error: cargo tarpaulin failed" >&2
        echo "$json_output" >&2
        exit 1
    fi
    
    # Extract line coverage percentage from JSON
    # The JSON output contains a "line_percent" field
    local line_percent
    line_percent=$(echo "$json_output" | grep -o '"line_percent":[0-9.]*' | head -1 | grep -o '[0-9.]*')
    
    if [[ -z "$line_percent" ]]; then
        echo "Error: Could not parse line coverage from tarpaulin output" >&2
        echo "$json_output" >&2
        exit 1
    fi
    
    echo "$line_percent"
}

# Function to read baseline
read_baseline() {
    if [[ -f "$BASELINE_FILE" ]]; then
        cat "$BASELINE_FILE"
    else
        echo "0.0"
    fi
}

# Function to write baseline
write_baseline() {
    echo "$1" > "$BASELINE_FILE"
    echo "Baseline updated to: $1%"
}

# Main logic
if [[ "$SET_BASELINE" == true ]]; then
    # Set/update baseline to current coverage
    current_coverage=$(get_current_coverage)
    write_baseline "$current_coverage"
    echo "Baseline set to current coverage: $current_coverage%"
    exit 0
fi

if [[ "$UPDATE_BASELINE" == true ]]; then
    # Get current coverage
    current_coverage=$(get_current_coverage)
    baseline=$(read_baseline)
    
    if [[ "$baseline" == "0.0" ]]; then
        # No baseline exists, set it
        write_baseline "$current_coverage"
        echo "Initial baseline set to: $current_coverage%"
        exit 0
    fi
    
    # Calculate difference
    difference=$(echo "$current_coverage - $baseline" | bc)
    
    if (( $(echo "$difference > 0" | bc -l) )); then
        # Coverage improved, update baseline
        write_baseline "$current_coverage"
        echo "Coverage improved! Updated baseline from ${baseline}% to ${current_coverage}%"
        exit 0
    else
        # Coverage didn't improve, just check against baseline
        echo "Coverage did not improve. Checking against baseline: ${baseline}%"
    fi
fi

# Normal mode: compare against baseline
current_coverage=$(get_current_coverage)
baseline=$(read_baseline)

if [[ "$baseline" == "0.0" ]]; then
    echo "No baseline found. Run with --baseline or --update-baseline to set one."
    exit 1
fi

# Calculate the difference
difference=$(echo "$baseline - $current_coverage" | bc)

echo "Current coverage: ${current_coverage}%"
echo "Baseline coverage: ${baseline}%"
echo "Difference: ${difference}%"

# Check if coverage dropped by more than threshold
drop_amount=$(echo "$difference > $THRESHOLD" | bc -l)

if [[ "$drop_amount" == "1" ]]; then
    echo "ERROR: Coverage dropped by more than ${THRESHOLD}%!"
    echo "Coverage must not decrease by more than ${THRESHOLD}% from baseline."
    exit 1
fi

echo "Coverage is within threshold (>= baseline - ${THRESHOLD}%)"
exit 0
