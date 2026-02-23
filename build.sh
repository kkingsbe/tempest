#!/bin/bash
# Tempest NEXRAD Weather Radar Application - Build Script
# This script builds optimized release binaries for multiple platforms
#
# Usage: ./build.sh [targets...]
#   Without arguments: builds for current platform only
#   With arguments: builds for specified targets (e.g., ./build.sh all)
#
# Available targets:
#   linux      - x86_64-unknown-linux-gnu
#   macos      - x86_64-apple-darwin
#   macos-arm  - aarch64-apple-darwin (Apple Silicon)
#   windows    - x86_64-pc-windows-msvc
#   all        - all supported targets

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Build configuration
CARGO_BUILD_ARGS="--release --locked"
BINARY_NAME="tempest"

echo_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

echo_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

echo_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if target is installed
check_target() {
    local target=$1
    if rustup target list --installed | grep -q "^${target}$"; then
        return 0
    else
        return 1
    fi
}

# Install target if needed
install_target() {
    local target=$1
    echo_info "Installing target: $target"
    rustup target add "$target"
}

# Build for a specific target
build_target() {
    local target=$1
    local output_dir="target/${target}/release"
    
    echo_info "Building for target: $target"
    
    # Check and install target if needed
    if ! check_target "$target"; then
        install_target "$target"
    fi
    
    # Build with target-specific flags
    cargo build --target "$target" $CARGO_BUILD_ARGS
    
    # Strip binary if it's not Windows
    if [[ "$target" != *"windows"* ]]; then
        local binary_path="target/${target}/release/${BINARY_NAME}"
        if [ -f "$binary_path" ]; then
            echo_info "Stripping binary: $binary_path"
            strip "$binary_path"
        fi
    fi
    
    echo_info "Build complete for: $target"
}

# Build for current platform only
build_current() {
    echo_info "Building for current platform..."
    cargo build $CARGO_BUILD_ARGS
    
    # Get binary size
    local binary_path="target/release/${BINARY_NAME}"
    if [ -f "$binary_path" ]; then
        local size=$(du -h "$binary_path" | cut -f1)
        echo_info "Binary size: $size"
    fi
}

# Main build function
main() {
    local build_type="${1:-current}"
    
    echo_info "Tempest Build Script"
    echo_info "====================="
    
    case "$build_type" in
        linux)
            build_target "x86_64-unknown-linux-gnu"
            ;;
        macos)
            build_target "x86_64-apple-darwin"
            ;;
        macos-arm)
            build_target "aarch64-apple-darwin"
            ;;
        windows)
            build_target "x86_64-pc-windows-msvc"
            ;;
        all)
            echo_warn "Building all targets - this may take a while..."
            build_target "x86_64-unknown-linux-gnu"
            build_target "x86_64-apple-darwin"
            build_target "aarch64-apple-darwin"
            build_target "x86_64-pc-windows-msvc"
            ;;
        current|*)
            build_current
            ;;
    esac
    
    echo_info "Build complete!"
}

# Run main function
main "$@"
