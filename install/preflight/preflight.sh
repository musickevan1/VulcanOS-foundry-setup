#!/bin/bash
# =============================================================================
# VulcanOS Installer - Preflight Checks
# =============================================================================

run_preflight() {
    header "Preflight checks"

    # Must be root (via sudo)
    if [[ $EUID -ne 0 ]]; then
        error "This script must be run with sudo"
        exit 1
    fi

    if [[ -z "${SUDO_USER:-}" || "$SUDO_USER" == "root" ]]; then
        error "Must be run via sudo from a regular user account"
        exit 1
    fi

    info "Target user: $(vulcan_user) (home: $(vulcan_home))"

    # Arch Linux only
    if [[ ! -f /etc/arch-release ]]; then
        error "This installer is designed for Arch Linux only"
        exit 1
    fi

    # Internet connectivity
    if ! ping -c 1 -W 3 archlinux.org &>/dev/null; then
        error "No internet connectivity"
        exit 1
    fi

    # Repo structure
    if [[ ! -d "$VULCAN_REPO" ]]; then
        error "VulcanOS repo not found at: $VULCAN_REPO"
        exit 1
    fi

    # Ensure critical tools
    for cmd in rsync stow; do
        if ! command -v "$cmd" &>/dev/null; then
            info "Installing $cmd..."
            pac_install "$cmd"
        fi
    done

    success "Preflight checks passed"
}
