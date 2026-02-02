#!/bin/bash
# =============================================================================
# VulcanOS T2 - Build Script
# Builds the T2 MacBook Pro ISO using multi-profile architecture
# =============================================================================

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
ARCHISO_DIR="$PROJECT_DIR/archiso"
PROFILE="t2"
WORK_DIR="/tmp/vulcanos-work-$PROFILE"
ASSEMBLED_DIR="/tmp/vulcanos-assembled-$PROFILE"
OUT_DIR="$PROJECT_DIR/out"

# Source shared functions
source "$SCRIPT_DIR/lib/build-common.sh"

# Set work directory for cleanup trap
export WORK_DIR ASSEMBLED_DIR

# Trap errors and clean up
trap cleanup EXIT

# =============================================================================
# Main Execution
# =============================================================================

main() {
    echo "=========================================="
    echo "  VulcanOS T2 - Build"
    echo "=========================================="
    echo ""

    # Validate environment
    check_root
    check_dependencies
    validate_profile "$PROFILE"

    # Build pipeline
    clean_build "$WORK_DIR" "$ASSEMBLED_DIR"
    assemble_profile "$PROFILE" "$ASSEMBLED_DIR"
    run_mkarchiso "$ASSEMBLED_DIR" "$WORK_DIR" "$OUT_DIR"

    # Finalize
    generate_checksums "$OUT_DIR"
    fix_permissions "$OUT_DIR"
    show_info "$OUT_DIR" "$PROFILE"
}

main "$@"
