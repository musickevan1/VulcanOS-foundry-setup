#!/bin/bash
# =============================================================================
# VulcanOS - Shared Build Library
# Common functions for multi-profile ISO builds
# =============================================================================

set -e

# =============================================================================
# Directory Configuration (can be overridden by calling script)
# =============================================================================

SCRIPT_DIR="${SCRIPT_DIR:-$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)}"
PROJECT_DIR="${PROJECT_DIR:-$(dirname "$(dirname "$SCRIPT_DIR")")}"
ARCHISO_DIR="${ARCHISO_DIR:-$PROJECT_DIR/archiso}"
BASE_DIR="${BASE_DIR:-$ARCHISO_DIR/base}"
PROFILE_DIR="${PROFILE_DIR:-$ARCHISO_DIR/profiles}"

# =============================================================================
# Color Constants and Logging
# =============================================================================

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

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
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

# =============================================================================
# Dependency Checking
# =============================================================================

check_root() {
    if [[ $EUID -ne 0 ]]; then
        error "This script must be run as root (use sudo)"
        exit 1
    fi
}

check_dependencies() {
    info "Checking dependencies..."

    local deps=("mkarchiso" "git" "mksquashfs" "xorriso" "rsync")
    local missing=()

    for dep in "${deps[@]}"; do
        if ! command -v "$dep" &> /dev/null; then
            missing+=("$dep")
        fi
    done

    if [[ ${#missing[@]} -gt 0 ]]; then
        error "Missing dependencies: ${missing[*]}"
        error "Install with: pacman -S archiso git squashfs-tools rsync"
        exit 1
    fi

    success "All dependencies found"
}

# =============================================================================
# Validation Functions
# =============================================================================

validate_profile() {
    local profile="$1"
    local errors=0

    info "Validating profile: $profile"

    # Required files
    local required_files=(
        "base/packages.base"
        "profiles/$profile/packages.profile"
        "profiles/$profile/pacman.conf"
        "profiles/$profile/profiledef.sh"
        "profiles/$profile/grub/grub.cfg"
        "profiles/$profile/syslinux/syslinux.cfg"
    )

    for file in "${required_files[@]}"; do
        if [[ ! -f "$ARCHISO_DIR/$file" ]]; then
            error "Missing required file: $file"
            ((errors++))
        fi
    done

    # Required directories
    if [[ ! -d "$ARCHISO_DIR/base/airootfs" ]]; then
        warn "Missing base/airootfs directory (will build without base overlay)"
    fi

    if [[ ! -d "$ARCHISO_DIR/profiles/$profile/airootfs" ]]; then
        warn "No profile-specific airootfs (will use base only)"
    fi

    if [[ $errors -gt 0 ]]; then
        error "Validation failed with $errors error(s)"
        exit 1
    fi

    success "Validation passed"
}

# =============================================================================
# Assembly Function
# =============================================================================

assemble_profile() {
    local profile="$1"
    local assembled_dir="$2"

    local base_dir="$ARCHISO_DIR/base"
    local profile_dir="$ARCHISO_DIR/profiles/$profile"

    info "Assembling profile: $profile"

    # Clean assembled directory
    rm -rf "$assembled_dir"
    mkdir -p "$assembled_dir"

    # Step 1: Copy base airootfs (if exists)
    if [[ -d "$base_dir/airootfs" ]]; then
        info "  Copying base airootfs..."
        rsync -a "$base_dir/airootfs/" "$assembled_dir/airootfs/"
    else
        info "  No base airootfs found, creating empty directory..."
        mkdir -p "$assembled_dir/airootfs"
    fi

    # Step 2: Overlay profile airootfs (profile wins on conflict)
    if [[ -d "$profile_dir/airootfs" ]]; then
        info "  Overlaying profile airootfs..."
        rsync -a "$profile_dir/airootfs/" "$assembled_dir/airootfs/"
    fi

    # Step 3: Merge package lists
    info "  Merging package lists..."
    cat "$base_dir/packages.base" "$profile_dir/packages.profile" | \
        grep -v '^[[:space:]]*#' | \
        grep -v '^[[:space:]]*$' | \
        sort -u > "$assembled_dir/packages.x86_64"

    # Step 4: Copy profile-specific configs (not merged)
    info "  Copying profile configs..."
    cp "$profile_dir/pacman.conf" "$assembled_dir/"
    cp "$profile_dir/profiledef.sh" "$assembled_dir/"

    # Step 5: Copy boot configs
    info "  Copying boot configs..."
    cp -r "$profile_dir/grub" "$assembled_dir/"
    cp -r "$profile_dir/syslinux" "$assembled_dir/"

    # Step 6: Copy efiboot (profile-specific if exists, else shared)
    if [[ -d "$profile_dir/efiboot" ]]; then
        info "  Copying profile-specific efiboot..."
        cp -r "$profile_dir/efiboot" "$assembled_dir/"
    elif [[ -d "$ARCHISO_DIR/efiboot" ]]; then
        info "  Copying shared efiboot..."
        cp -r "$ARCHISO_DIR/efiboot" "$assembled_dir/"
    else
        warn "  No efiboot directory found (neither profile-specific nor shared)"
    fi

    success "Profile assembled: $assembled_dir"
}

# =============================================================================
# Build Helpers
# =============================================================================

clean_build() {
    local work_dir="$1"
    local assembled_dir="$2"

    info "Cleaning previous build artifacts..."

    if [[ -d "$work_dir" ]]; then
        rm -rf "$work_dir"
    fi

    if [[ -d "$assembled_dir" ]]; then
        rm -rf "$assembled_dir"
    fi

    success "Build directories cleaned"
}

run_mkarchiso() {
    local assembled_dir="$1"
    local work_dir="$2"
    local out_dir="$3"

    info "Building ISO with mkarchiso..."
    info "  Assembled profile: $assembled_dir"
    info "  Work directory: $work_dir"
    info "  Output directory: $out_dir"

    # Create output directory if it doesn't exist
    mkdir -p "$out_dir"

    # Set SOURCE_DATE_EPOCH for reproducibility
    if [[ -d "$PROJECT_DIR/.git" ]]; then
        export SOURCE_DATE_EPOCH=$(git -C "$PROJECT_DIR" log -1 --format=%ct 2>/dev/null || date +%s)
    else
        export SOURCE_DATE_EPOCH=$(date +%s)
    fi

    # Run mkarchiso
    mkarchiso -v \
        -w "$work_dir" \
        -o "$out_dir" \
        "$assembled_dir"

    success "ISO build completed!"
}

generate_checksums() {
    local out_dir="$1"

    info "Generating checksums..."

    cd "$out_dir"

    if ls *.iso 1> /dev/null 2>&1; then
        sha256sum *.iso > SHA256SUMS
        md5sum *.iso > MD5SUMS
        success "Checksums generated"
    else
        warn "No ISO file found to generate checksums"
    fi
}

fix_permissions() {
    local out_dir="$1"

    info "Fixing output permissions..."

    if [[ -n "$SUDO_USER" ]]; then
        chown -R "$SUDO_USER:$SUDO_USER" "$out_dir"
        success "Permissions fixed for user $SUDO_USER"
    fi
}

cleanup() {
    local exit_code=$?

    # Allow overriding via WORK_DIR and ASSEMBLED_DIR environment variables
    local work_dir="${WORK_DIR:-}"
    local assembled_dir="${ASSEMBLED_DIR:-}"

    if [[ $exit_code -ne 0 ]]; then
        error "Build failed. Cleaning up..."
    fi

    if [[ -n "$work_dir" && -d "$work_dir" ]]; then
        info "Removing work directory: $work_dir"
        rm -rf "$work_dir"
    fi

    if [[ -n "$assembled_dir" && -d "$assembled_dir" ]]; then
        info "Removing assembled directory: $assembled_dir"
        rm -rf "$assembled_dir"
    fi

    if [[ $exit_code -eq 0 ]]; then
        success "Cleanup completed"
    fi
}

show_info() {
    local out_dir="$1"
    local profile="${2:-unknown}"

    echo ""
    echo "=========================================="
    echo "  VulcanOS Build Complete!"
    echo "  Profile: $profile"
    echo "=========================================="
    echo ""

    if ls "$out_dir"/*.iso 1> /dev/null 2>&1; then
        local iso_file=$(ls "$out_dir"/*.iso)
        local iso_size=$(du -h "$iso_file" | cut -f1)
        echo "ISO File: $(basename "$iso_file")"
        echo "Size: $iso_size"
        echo "Location: $iso_file"
        echo ""
        echo "Checksums:"
        cat "$out_dir/SHA256SUMS" 2>/dev/null || echo "  (not generated)"
        echo ""
    fi

    echo "To test the ISO:"
    echo "  ./scripts/test-iso.sh"
    echo ""
    echo "To write to USB:"
    echo "  sudo dd if=\$ISO_FILE of=/dev/sdX bs=4M status=progress oflag=sync"
    echo ""
}
