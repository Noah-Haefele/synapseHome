#!/usr/bin/env bash

if [[ -t 1 ]] && [[ -z "${NO_COLOR:-}" ]]; then
    readonly COLOR_BLUE=$'\033[0;34m'
    readonly COLOR_GREEN=$'\033[0;32m'
    readonly COLOR_YELLOW=$'\033[0;33m'
    readonly COLOR_RED=$'\033[0;31m'
    readonly COLOR_CYAN=$'\033[0;36m'
    readonly COLOR_RESET=$'\033[0m'
else
    readonly COLOR_BLUE=''
    readonly COLOR_GREEN=''
    readonly COLOR_YELLOW=''
    readonly COLOR_RED=''
    readonly COLOR_CYAN=''
    readonly COLOR_RESET=''
fi

# Logging
log() {
    printf '%s\n' "$*"
}

info() {
    printf '%s[ INFO ]%s %s\n' "$COLOR_BLUE" "$COLOR_RESET" "$*"
}

success() {
    printf '%s[ OK   ]%s %s\n' "$COLOR_GREEN" "$COLOR_RESET" "$*"
}

warn() {
    printf '%s[ WARN ] %s%s\n' "$COLOR_YELLOW" "$*" "$COLOR_RESET" >&2
}

error() {
    printf '%s[ FAIL ] %s%s\n' "$COLOR_RED" "$*" "$COLOR_RESET" >&2
}

step() {
    printf '\n%s==> %s%s\n' "$COLOR_CYAN" "$*" "$COLOR_RESET"
}
