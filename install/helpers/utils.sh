#!/bin/bash
# =============================================================================
# VulcanOS Installer - Shared Utilities
# =============================================================================

# State directory for idempotent tracking
STATE_DIR="${STATE_DIR:-/var/lib/vulcanos/install-state}"

# Install packages (idempotent, quiet)
pac_install() {
    pacman -S --needed --noconfirm "$@"
}

# Get the real (non-root) user who invoked sudo
vulcan_user() {
    echo "${SUDO_USER:-$USER}"
}

# Get home directory of the install target user
vulcan_home() {
    eval echo "~$(vulcan_user)"
}

# Idempotent phase tracking — skip if already completed
is_done() {
    local phase="$1"
    [[ -f "$STATE_DIR/$phase.done" ]]
}

mark_done() {
    local phase="$1"
    mkdir -p "$STATE_DIR"
    touch "$STATE_DIR/$phase.done"
}
