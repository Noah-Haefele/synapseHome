#!/usr/bin/env bash

set -e

GREEN='\033[0;32m'
NC='\033[0m'

echo
echo ----------------------
echo Start building the ui
echo ----------------------
echo

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"

SOURCE_DIR="$SCRIPT_DIR/src/iface/ui"
BUILD_DIR="$SCRIPT_DIR/build"

# Building process
cmake -S "$SOURCE_DIR" -B "$BUILD_DIR"
cmake --build "$BUILD_DIR"

echo
echo ----------------------
echo -e "${GREEN}Building terminated${NC}"
echo ----------------------
