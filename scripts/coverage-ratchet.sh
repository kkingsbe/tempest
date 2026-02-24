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
#   0 - Coverage is within threshold (>= baseline - 1.5%) or improved
#   1 - Coverage dropped by more than 1.5%
#
# Coverage History:
#   Coverage data is stored in .coverage_history.json
#   Format: JSON array with entries containing date, branch, commit, and per-crate coverage

set -euo pipefail

# Constants
BASELINE_FILE=".coverage_baseline"
HISTORY_FILE=".coverage_history.json"
THRESHOLD=1.5
TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

# Get git info
get_git_branch() {
    git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown"
}

get_git_commit() {
    git rev-parse HEAD 2>/dev/null | cut -c1-8 || echo "unknown"
}

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

# Function to get current coverage (overall and per-crate)
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
    
    # Extract line coverage percentage from JSON (overall)
    local line_percent
    line_percent=$(echo "$json_output" | grep -o '"line_percent":[0-9.]*' | head -1 | grep -o '[0-9.]*')
    
    if [[ -z "$line_percent" ]]; then
        echo "Error: Could not parse line coverage from tarpaulin output" >&2
        echo "$json_output" >&2
        exit 1
    fi
    
    echo "$line_percent"
}

# Function to get per-crate coverage from tarpaulin JSON output
get_per_crate_coverage() {
    local json_output
    json_output=$(cargo tarpaulin --line --config tarpaulin.toml --out Json 2>&1)
    
    # Extract all package coverage data
    # The JSON format contains "files" array with "package_name" and "line_percent"
    echo "$json_output" | grep -o '"package_name":"[^"]*","line_percent":[0-9.]*' | while read -r line; do
        local pkg_name line_pct
        pkg_name=$(echo "$line" | grep -o '"package_name":"[^"]*"' | cut -d'"' -f4)
        line_pct=$(echo "$line" | grep -o '"line_percent":[0-9.]*' | cut -d':' -f2)
        if [[ -n "$pkg_name" && -n "$line_pct" ]]; then
            echo "  \"$pkg_name\": $line_pct"
        fi
    done | paste -sd ',' | sed 's/^/{/;s/$/}/'
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

# Function to read coverage history
read_history() {
    if [[ -f "$HISTORY_FILE" ]]; then
        cat "$HISTORY_FILE"
    else
        echo "[]"
    fi
}

# Function to write coverage history
write_history() {
    echo "$1" > "$HISTORY_FILE"
}

# Function to add entry to history
add_history_entry() {
    local overall_coverage="$1"
    local branch="$2"
    local commit="$3"
    local timestamp="$4"
    local crate_coverage="$5"
    
    local history
    history=$(read_history)
    
    # Create new entry
    local new_entry
    new_entry="{\"date\":\"$timestamp\",\"branch\":\"$branch\",\"commit\":\"$commit\",\"overall\":$overall_coverage,\"crates\":{$crate_coverage}}"
    
    # Add to history array (prepend)
    if [[ "$history" == "[]" ]]; then
        history="[$new_entry]"
    else
        # Remove trailing ] and add new entry
        history="${history%]},$new_entry]"
    fi
    
    write_history "$history"
    echo "Coverage history updated"
}

# Function to get latest history entry
get_latest_history_entry() {
    local history
    history=$(read_history)
    
    if [[ "$history" == "[]" || -z "$history" ]]; then
        return 1
    fi
    
    # Get first entry (most recent)
    echo "$history" | grep -o '{[^}]*}' | head -1
}

# Main logic
if [[ "$SET_BASELINE" == true ]]; then
    # Set/update baseline to current coverage
    current_coverage=$(get_current_coverage)
    write_baseline "$current_coverage"
    
    # Also add to history
    branch=$(get_git_branch)
    commit=$(get_git_commit)
    crate_coverage=$(get_per_crate_coverage)
    add_history_entry "$current_coverage" "$branch" "$commit" "$TIMESTAMP" "$crate_coverage"
    
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
        
        # Add to history
        branch=$(get_git_branch)
        commit=$(get_git_commit)
        crate_coverage=$(get_per_crate_coverage)
        add_history_entry "$current_coverage" "$branch" "$commit" "$TIMESTAMP" "$crate_coverage"
        
        echo "Initial baseline set to: $current_coverage%"
        exit 0
    fi
    
    # Calculate difference
    difference=$(echo "$current_coverage - $baseline" | bc)
    
    if (( $(echo "$difference > 0" | bc -l) )); then
        # Coverage improved, update baseline
        write_baseline "$current_coverage"
        
        # Add to history
        branch=$(get_git_branch)
        commit=$(get_git_commit)
        crate_coverage=$(get_per_crate_coverage)
        add_history_entry "$current_coverage" "$branch" "$commit" "$TIMESTAMP" "$crate_coverage"
        
        echo "Coverage improved! Updated baseline from ${baseline}% to ${current_coverage}%"
        exit 0
    else
        # Coverage didn't improve, just check against baseline
        # Still add to history for tracking
        branch=$(get_git_branch)
        commit=$(get_git_commit)
        crate_coverage=$(get_per_crate_coverage)
        add_history_entry "$current_coverage" "$branch" "$commit" "$TIMESTAMP" "$crate_coverage"
        
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

# Add to history even if within threshold
branch=$(get_git_branch)
commit=$(get_git_commit)
crate_coverage=$(get_per_crate_coverage)
add_history_entry "$current_coverage" "$branch" "$commit" "$TIMESTAMP" "$crate_coverage"

echo "Coverage is within threshold (>= baseline - ${THRESHOLD}%)"
exit 0
