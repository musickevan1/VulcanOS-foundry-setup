#!/bin/bash
# =============================================================================
# VulcanOS - Modular Installer Orchestrator
#
# Sources phase scripts in order. Designed to be re-runnable (idempotent) —
# each phase tracks completion state and skips if already done.
#
# Environment (set by boot.sh or manually):
#   VULCAN_REPO     — path to VulcanOS repo (default: script's parent dir)
#   VULCAN_PROFILE  — profile name (default: foundry)
#
# Usage:
#   sudo -E bash install.sh                  # uses env vars from boot.sh
#   sudo VULCAN_PROFILE=foundry bash install.sh  # manual invocation
# =============================================================================

set -euo pipefail

# -------------------------------------------------------------------
# Resolve paths
# -------------------------------------------------------------------
VULCAN_REPO="${VULCAN_REPO:-$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)}"
VULCAN_PROFILE="${VULCAN_PROFILE:-foundry}"
INSTALLER_DIR="$VULCAN_REPO/install"
LOG_FILE="/var/log/vulcanos-install.log"
STATE_DIR="/var/lib/vulcanos/install-state"

export VULCAN_REPO VULCAN_PROFILE INSTALLER_DIR STATE_DIR

# -------------------------------------------------------------------
# Source helpers
# -------------------------------------------------------------------
source "$INSTALLER_DIR/helpers/colors.sh"
source "$INSTALLER_DIR/helpers/errors.sh"
source "$INSTALLER_DIR/helpers/utils.sh"

trap_errors

# -------------------------------------------------------------------
# Logging (tee to both terminal and log file)
# -------------------------------------------------------------------
mkdir -p "$(dirname "$LOG_FILE")"
exec > >(tee -a "$LOG_FILE") 2>&1

# -------------------------------------------------------------------
# Banner
# -------------------------------------------------------------------
echo ""
echo -e "${BOLD}==========================================${NC}"
echo -e "${BOLD}  VulcanOS ${VULCAN_PROFILE^} - Installer${NC}"
echo -e "${BOLD}==========================================${NC}"
echo ""
echo -e "  Repo:    $VULCAN_REPO"
echo -e "  Profile: $VULCAN_PROFILE"
echo -e "  Log:     $LOG_FILE"
echo -e "  State:   $STATE_DIR"
echo ""

START_TIME=$(date +%s)

# -------------------------------------------------------------------
# Source and run each phase in order
# -------------------------------------------------------------------

# Phase 0: Preflight
source "$INSTALLER_DIR/preflight/preflight.sh"
run_preflight

# Phase 1: System configuration
source "$INSTALLER_DIR/system/configure.sh"
run_system_configure

# Phase 2: Package installation
source "$INSTALLER_DIR/packages/install.sh"
run_package_install

# Phase 3: Hardware detection & GPU config
source "$INSTALLER_DIR/hardware/detect.sh"
run_hardware_detect

# Phase 4: System files
source "$INSTALLER_DIR/files/system-files.sh"
run_system_files

# Phase 5: Dotfiles (GNU Stow)
source "$INSTALLER_DIR/dotfiles/stow.sh"
run_dotfiles_stow

# Phase 6: Service enablement
source "$INSTALLER_DIR/services/enable.sh"
run_enable_services

# Phase 7: User configuration
source "$INSTALLER_DIR/user/configure.sh"
run_user_configure

# Phase 8: Summary
END_TIME=$(date +%s)
ELAPSED=$(( END_TIME - START_TIME ))
ELAPSED_FMT="$(( ELAPSED / 60 ))m $(( ELAPSED % 60 ))s"

source "$INSTALLER_DIR/post/summary.sh"
run_summary "$ELAPSED_FMT"
