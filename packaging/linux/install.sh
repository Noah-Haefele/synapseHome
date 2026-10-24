#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../scripts/logging.sh"
source "$SCRIPT_DIR/helpers/package_install.sh"

TOUCH_DEVICE=false
DOOR_DEVICE=false

usage() {
    info "Usage: $0 [OPTIONS]"
    info
    info "Options:"
    info "  -t      Install Touch-Device"
    info "  -d      Install Door-Device"
    info "  -h      Display Help"
}

while getopts ":tdh" opt; do
    case "$opt" in
    t)
        TOUCH_DEVICE=true
        ;;
    d)
        DOOR_DEVICE=true
        ;;
    h)
        usage
        exit 0
        ;;
    \?)
        error "Unknown option: -$OPTARG"
        usage >&2
        exit 1
        ;;
    :)
        error "Option -$OPTARG requires a value."
        usage
        exit 1
        ;;
    esac
done

# Check if no device was selected
if [[ "$TOUCH_DEVICE" == false && "$DOOR_DEVICE" == false ]]; then
    error "No option provided."
    usage
    exit 1
fi

if command -v apt-get >/dev/null 2>&1; then
    sudo apt-get update
fi

step "Installing dependencies ..."

info "Installing curl ..."
install_packages curl
success "Curl is installed."

# Installing dependencys and compilers
log
bash "$SCRIPT_DIR/install/cpp_install.sh"
log
bash "$SCRIPT_DIR/install/qml_install.sh"
log
bash "$SCRIPT_DIR/install/rust_install.sh"
success "Dependencies are installed."

# Compiling the frontend
log
bash "$SCRIPT_DIR/compile/ui_build.sh"
