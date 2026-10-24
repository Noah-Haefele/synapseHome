#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"
source "$SCRIPT_DIR/package_install.sh"

if [[ -r /etc/os-release ]]; then
    source /etc/os-release
else
    error "Cannot detect Linux distribution."
    exit 1
fi

DISTRO_ID="${ID:-}"
DISTRO_LIKE="${ID_LIKE:-}"

if [[ "$DISTRO_ID" =~ ^(ubuntu|debian|linuxmint|pop|elementary)$ ]] || [[ "$DISTRO_LIKE" =~ (ubuntu|debian) ]]; then
    install_packages qt6-base-dev qt6-declarative-dev
elif [[ "$DISTRO_ID" =~ ^(fedora|rhel|centos)$ ]] || [[ "$DISTRO_LIKE" =~ (fedora|rhel) ]]; then
    install_packages qt6-qtbase-devel qt6-qtdeclarative-devel
elif [[ "$DISTRO_ID" =~ ^(arch|manjaro|endeavouros)$ ]] || [[ "$DISTRO_LIKE" =~ arch ]]; then
    install_packages qt6-base qt6-declarative
else
    error "Unsupported distribution: ${DISTRO_ID:-unknown}"
    exit 1
fi

success "Qt and QML dependencies are installed."
