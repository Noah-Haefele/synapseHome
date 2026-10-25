#!/usr/bin/env bash
set -eo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../scripts/logging.sh"

WORK_DIR="/tmp/WM8960-Audio-HAT"

step "Setting up WM8960 Audio HAT driver ..."

# Clean up potential old clones in /tmp
if [[ -d "$WORK_DIR" ]]; then
    rm -rf "$WORK_DIR"
fi

info "Cloning Waveshare WM8960-Audio-HAT repository ..."
git clone https://github.com/waveshareteam/WM8960-Audio-HAT "$WORK_DIR"

# Run their installer
info "Running driver installer ..."
cd "$WORK_DIR"
sudo chmod +x install.sh
sudo ./install.sh

cd "$SCRIPT_DIR"
rm -rf "$WORK_DIR"

success "Audio driver installed successfully."
info "When encountering problems, feel free to visit waveshare's page"
info "https://www.waveshare.com/wiki/WM8960_Audio_HAT"

log
warn "A system reboot is required for the audio driver to take effect."
read -rp "Do you want to reboot now? [y/N] " -n 1 -r
log

if [[ $REPLY =~ ^[Yy]$ ]]; then
    info "Rebooting system..."
    sudo reboot
else
    info "Please remember to reboot manually before running the main installation."
fi
