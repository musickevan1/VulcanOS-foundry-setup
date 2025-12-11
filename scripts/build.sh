#!/bin/bash
# =============================================================================
# VulcanOS - Main ISO Build Script
# Builds the custom Arch Linux ISO using archiso
# =============================================================================

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
ARCHISO_DIR="$PROJECT_DIR/archiso"
WORK_DIR="/tmp/vulcanos-archiso-work"
OUT_DIR="$PROJECT_DIR/out"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

# Check if running as root
check_root() {
    if [[ $EUID -ne 0 ]]; then
        error "This script must be run as root (use sudo)"
    fi
}

# Check dependencies
check_dependencies() {
    info "Checking dependencies..."

    local deps=("mkarchiso" "git" "mksquashfs" "xorriso")
    local missing=()

    for dep in "${deps[@]}"; do
        if ! command -v "$dep" &> /dev/null; then
            missing+=("$dep")
        fi
    done

    if [[ ${#missing[@]} -gt 0 ]]; then
        error "Missing dependencies: ${missing[*]}\nInstall with: pacman -S archiso git squashfs-tools"
    fi

    success "All dependencies found"
}

# Clean previous builds
clean_build() {
    info "Cleaning previous build artifacts..."

    if [[ -d "$WORK_DIR" ]]; then
        rm -rf "$WORK_DIR"
    fi

    if [[ -d "$OUT_DIR" ]]; then
        rm -rf "$OUT_DIR"
    fi

    mkdir -p "$OUT_DIR"
    success "Build directories cleaned"
}

# Copy dotfiles to skel
prepare_skel() {
    info "Preparing user skeleton with dotfiles..."

    local skel_dir="$ARCHISO_DIR/airootfs/etc/skel"

    # Create .config directory in skel
    mkdir -p "$skel_dir/.config"

    # Clear existing configs to prevent nested copies
    rm -rf "$skel_dir/.config/hypr" "$skel_dir/.config/waybar" "$skel_dir/.config/alacritty" "$skel_dir/.config/wofi" "$skel_dir/.config/swaync"

    # Copy Hyprland config
    if [[ -d "$PROJECT_DIR/dotfiles/hypr" ]]; then
        cp -r "$PROJECT_DIR/dotfiles/hypr" "$skel_dir/.config/hypr"
        info "  - Copied Hyprland configuration"
    fi

    # Copy Waybar config
    if [[ -d "$PROJECT_DIR/dotfiles/waybar" ]]; then
        cp -r "$PROJECT_DIR/dotfiles/waybar" "$skel_dir/.config/waybar"
        info "  - Copied Waybar configuration"
    fi

    # Copy Alacritty config
    if [[ -d "$PROJECT_DIR/dotfiles/alacritty" ]]; then
        cp -r "$PROJECT_DIR/dotfiles/alacritty" "$skel_dir/.config/alacritty"
        info "  - Copied Alacritty configuration"
    fi

    # Copy Wofi config
    if [[ -d "$PROJECT_DIR/dotfiles/wofi" ]]; then
        cp -r "$PROJECT_DIR/dotfiles/wofi" "$skel_dir/.config/wofi"
        info "  - Copied Wofi configuration"
    fi

    # Copy SwayNC config
    if [[ -d "$PROJECT_DIR/dotfiles/swaync" ]]; then
        cp -r "$PROJECT_DIR/dotfiles/swaync" "$skel_dir/.config/swaync"
        info "  - Copied SwayNC configuration"
    fi

    # Copy bashrc
    if [[ -f "$PROJECT_DIR/dotfiles/bash/.bashrc" ]]; then
        cp "$PROJECT_DIR/dotfiles/bash/.bashrc" "$skel_dir/.bashrc"
        info "  - Copied .bashrc"
    fi

    # Copy gitconfig
    if [[ -f "$PROJECT_DIR/dotfiles/git/.gitconfig" ]]; then
        cp "$PROJECT_DIR/dotfiles/git/.gitconfig" "$skel_dir/.gitconfig"
        info "  - Copied .gitconfig"
    fi

    success "User skeleton prepared"
}

# Build the ISO
build_iso() {
    info "Building NewOS ISO..."
    info "Work directory: $WORK_DIR"
    info "Output directory: $OUT_DIR"

    # Set SOURCE_DATE_EPOCH for reproducibility
    if [[ -d "$PROJECT_DIR/.git" ]]; then
        export SOURCE_DATE_EPOCH=$(git -C "$PROJECT_DIR" log -1 --format=%ct 2>/dev/null || date +%s)
    else
        export SOURCE_DATE_EPOCH=$(date +%s)
    fi

    # Run mkarchiso
    mkarchiso -v \
        -w "$WORK_DIR" \
        -o "$OUT_DIR" \
        "$ARCHISO_DIR"

    success "ISO build completed!"
}

# Generate checksums
generate_checksums() {
    info "Generating checksums..."

    cd "$OUT_DIR"

    if ls *.iso 1> /dev/null 2>&1; then
        sha256sum *.iso > SHA256SUMS
        md5sum *.iso > MD5SUMS
        success "Checksums generated"
    else
        warn "No ISO file found to generate checksums"
    fi
}

# Clean up
cleanup() {
    info "Cleaning up work directory..."
    rm -rf "$WORK_DIR"
    success "Cleanup completed"
}

# Fix permissions
fix_permissions() {
    info "Fixing output permissions..."

    if [[ -n "$SUDO_USER" ]]; then
        chown -R "$SUDO_USER:$SUDO_USER" "$OUT_DIR"
        success "Permissions fixed for user $SUDO_USER"
    fi
}

# Show build info
show_info() {
    echo ""
    echo "=========================================="
    echo "  VulcanOS Build Complete!"
    echo "=========================================="
    echo ""

    if ls "$OUT_DIR"/*.iso 1> /dev/null 2>&1; then
        local iso_file=$(ls "$OUT_DIR"/*.iso)
        local iso_size=$(du -h "$iso_file" | cut -f1)
        echo "ISO File: $(basename "$iso_file")"
        echo "Size: $iso_size"
        echo "Location: $iso_file"
        echo ""
        echo "Checksums:"
        cat "$OUT_DIR/SHA256SUMS" 2>/dev/null || echo "  (not generated)"
        echo ""
    fi

    echo "To test the ISO:"
    echo "  ./scripts/test-iso.sh"
    echo ""
    echo "To write to USB:"
    echo "  sudo dd if=$iso_file of=/dev/sdX bs=4M status=progress oflag=sync"
    echo ""
}

# Main execution
main() {
    echo "=========================================="
    echo "  VulcanOS - Custom Arch Linux Build"
    echo "=========================================="
    echo ""

    check_root
    check_dependencies
    clean_build
    prepare_skel
    build_iso
    generate_checksums
    fix_permissions
    cleanup
    show_info
}

# Run main
main "$@"
