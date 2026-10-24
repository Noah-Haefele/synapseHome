#!/usr/bin/env bash
set -euo pipefail

install_packages() {
    if [[ $# -eq 0 ]]; then
        return 0
    fi

    if [[ $EUID -eq 0 ]]; then
        SUDO=()
    else
        SUDO=(sudo)
    fi

    if command -v apt-get >/dev/null 2>&1; then
        "${SUDO[@]}" apt-get install -y "$@"

    elif command -v dnf >/dev/null 2>&1; then
        "${SUDO[@]}" dnf install -y "$@"

    elif command -v pacman >/dev/null 2>&1; then
        "${SUDO[@]}" pacman -S --needed --noconfirm "$@"

    elif command -v zypper >/dev/null 2>&1; then
        "${SUDO[@]}" zypper --non-interactive install "$@"

    elif command -v apk >/dev/null 2>&1; then
        "${SUDO[@]}" apk add "$@"

    else
        error "Unsupported package manager." >&2
        return 1
    fi
}
