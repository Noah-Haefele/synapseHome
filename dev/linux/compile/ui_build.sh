#!/usr/bin/env bash
set -eo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../scripts/logging.sh"

PROJECT_ROOT="$(cd -- "$SCRIPT_DIR/../../.." && pwd)"
SOURCE_DIR="$PROJECT_ROOT/src/iface/ui"
BUILD_DIR="$PROJECT_ROOT/build"
mkdir -p "$BUILD_DIR"

CONFIG_LOG="$BUILD_DIR/configure.log"
BUILD_LOG="$BUILD_DIR/build.log"

# Print log locations automatically if any command fails
trap 'error "Build failed. Check logs at: $CONFIG_LOG and $BUILD_LOG"; exit 1' ERR

VERBOSE=false
for arg in "$@"; do
    [[ "$arg" == "-v" || "$arg" == "--verbose" ]] && VERBOSE=true
done

step "Building the UI ..."

# Configure project
if [ "$VERBOSE" = true ]; then
    cmake -S "$SOURCE_DIR" -B "$BUILD_DIR" 2>&1 | tee "$CONFIG_LOG"
else
    cmake -S "$SOURCE_DIR" -B "$BUILD_DIR" >"$CONFIG_LOG" 2>&1
fi

# Compile binaries
if [ "$VERBOSE" = true ]; then
    cmake --build "$BUILD_DIR" 2>&1 | tee "$BUILD_LOG"
else
    cmake --build "$BUILD_DIR" >"$BUILD_LOG" 2>&1
fi

# Clear the error trap on success
trap - ERR

info "Config-Log: $CONFIG_LOG"
info "Build-Log: $BUILD_LOG"
success "UI built successfully."
