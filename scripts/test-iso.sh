#!/bin/bash
# =============================================================================
# VulcanOS - ISO Testing Script
# Tests the built ISO using QEMU
# =============================================================================

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
OUT_DIR="$PROJECT_DIR/out"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

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

# Check dependencies
check_deps() {
    info "Checking QEMU dependencies..."

    if ! command -v qemu-system-x86_64 &> /dev/null; then
        error "qemu-system-x86_64 not found. Install with: pacman -S qemu-full"
    fi

    success "QEMU found"
}

# Find ISO file
find_iso() {
    local iso_file=$(ls "$OUT_DIR"/*.iso 2>/dev/null | head -n1)

    if [[ -z "$iso_file" ]]; then
        error "No ISO file found in $OUT_DIR"
    fi

    echo "$iso_file"
}

# Run ISO in BIOS mode
run_bios() {
    local iso_file="$1"

    info "Starting QEMU in BIOS mode..."
    info "ISO: $(basename "$iso_file")"

    qemu-system-x86_64 \
        -boot d \
        -cdrom "$iso_file" \
        -m 4G \
        -smp 2 \
        -enable-kvm \
        -cpu host \
        -vga virtio \
        -display gtk,gl=on \
        -device virtio-net-pci,netdev=net0 \
        -netdev user,id=net0,hostfwd=tcp::2222-:22 \
        -device virtio-rng-pci \
        -device qemu-xhci \
        -device usb-tablet \
        -audiodev pa,id=snd0 \
        -device ich9-intel-hda \
        -device hda-output,audiodev=snd0
}

# Run ISO in UEFI mode
run_uefi() {
    local iso_file="$1"

    info "Starting QEMU in UEFI mode..."
    info "ISO: $(basename "$iso_file")"

    # Check for OVMF
    local ovmf_code="/usr/share/edk2/x64/OVMF_CODE.4m.fd"
    local ovmf_vars="/usr/share/edk2/x64/OVMF_VARS.4m.fd"

    if [[ ! -f "$ovmf_code" ]]; then
        error "OVMF not found. Install with: pacman -S edk2-ovmf"
    fi

    # Create temporary copy of OVMF_VARS
    local temp_vars="/tmp/vulcanos_ovmf_vars.fd"
    cp "$ovmf_vars" "$temp_vars"

    qemu-system-x86_64 \
        -boot d \
        -cdrom "$iso_file" \
        -m 4G \
        -smp 2 \
        -enable-kvm \
        -cpu host \
        -vga virtio \
        -display gtk,gl=on \
        -drive if=pflash,format=raw,readonly=on,file="$ovmf_code" \
        -drive if=pflash,format=raw,file="$temp_vars" \
        -device virtio-net-pci,netdev=net0 \
        -netdev user,id=net0,hostfwd=tcp::2222-:22 \
        -device virtio-rng-pci \
        -device qemu-xhci \
        -device usb-tablet \
        -audiodev pa,id=snd0 \
        -device ich9-intel-hda \
        -device hda-output,audiodev=snd0
}

# Show help
show_help() {
    echo "Usage: $0 [OPTION]"
    echo ""
    echo "Test VulcanOS ISO using QEMU"
    echo ""
    echo "Options:"
    echo "  --bios    Boot in BIOS mode (default)"
    echo "  --uefi    Boot in UEFI mode"
    echo "  --help    Show this help message"
    echo ""
    echo "Example:"
    echo "  $0 --uefi"
    echo ""
}

# Main
main() {
    local mode="bios"

    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --bios)
                mode="bios"
                shift
                ;;
            --uefi)
                mode="uefi"
                shift
                ;;
            --help)
                show_help
                exit 0
                ;;
            *)
                error "Unknown option: $1"
                ;;
        esac
    done

    echo "=========================================="
    echo "  VulcanOS - ISO Testing"
    echo "=========================================="
    echo ""

    check_deps

    local iso_file=$(find_iso)
    info "Found ISO: $(basename "$iso_file")"

    case $mode in
        bios)
            run_bios "$iso_file"
            ;;
        uefi)
            run_uefi "$iso_file"
            ;;
    esac
}

main "$@"
