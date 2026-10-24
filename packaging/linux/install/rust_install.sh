#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../scripts/logging.sh"

# Check if rust is already installed
if command -v rustc >/dev/null 2>&1 &&
    command -v cargo >/dev/null 2>&1 &&
    command -v rustup >/dev/null 2>&1; then

    log "Rust is already installed."
    info "Updating Rust ..."
    rustup update
    success "Rust is now up to date."

else
    warn "Rust is not installed."
    info "Installing Rust ..."

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

    if [ -f "$HOME/.cargo/env" ]; then
        source "$HOME/.cargo/env"
    fi

    success "Rust is installed."
fi
