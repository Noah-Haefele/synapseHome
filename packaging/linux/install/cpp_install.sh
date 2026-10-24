#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../scripts/logging.sh"
source "$SCRIPT_DIR/../helpers/package_install.sh"

if [[ -r /etc/os-release ]]; then
    source /etc/os-release
else
    error "Cannot detect Linux distribution."
    exit 1
fi

DISTRO_ID="${ID:-}"
DISTRO_LIKE="${ID_LIKE:-}"

if [[ "$DISTRO_ID" =~ ^(ubuntu|debian|linuxmint|pop|elementary)$ ]] ||
    [[ "$DISTRO_LIKE" =~ (ubuntu|debian) ]]; then
    info "Installing C++ dependencies ..."

    install_packages \
        cmake \
        libasound2-dev \
        libclang-dev \
        meson \
        ninja-build \
        build-essential \
        pkg-config

elif [[ "$DISTRO_ID" =~ ^(fedora|rhel|centos)$ ]] ||
    [[ "$DISTRO_LIKE" =~ (fedora|rhel) ]]; then
    info "Installing C++ dependencies ..."

    install_packages \
        cmake \
        alsa-lib-devel \
        clang-devel \
        meson \
        ninja-build \
        gcc-c++ \
        make \
        pkgconf-pkg-config

elif [[ "$DISTRO_ID" =~ ^(arch|manjaro|endeavouros)$ ]] ||
    [[ "$DISTRO_LIKE" =~ arch ]]; then
    info "Installing C++ dependencies ..."

    install_packages \
        cmake \
        alsa-lib \
        clang \
        meson \
        ninja \
        base-devel \
        pkgconf

else
    error "Unsupported distribution: ${DISTRO_ID:-unknown}"
    exit 1
fi

success "C++ build dependencies are installed."
