#!/bin/bash
# =============================================================================
# VulcanOS Installer - Error Handling
# =============================================================================

# Trap ERR to report file and line on failure
trap_errors() {
    trap '__error_handler $? "${BASH_SOURCE[0]}" ${LINENO}' ERR
}

__error_handler() {
    local exit_code=$1
    local source=$2
    local lineno=$3
    error "Command failed (exit $exit_code) at ${source}:${lineno}"
    error "Installation aborted. Check /var/log/vulcanos-install.log for details."
    exit "$exit_code"
}
