#!/bin/bash
# =============================================================================
# VulcanOS Installer - Hardware Detection
# =============================================================================

run_hardware_detect() {
    header "Hardware detection"

    # GPU detection
    if lspci | grep -qi 'nvidia'; then
        export VULCAN_GPU="nvidia"
        info "GPU: NVIDIA detected"
    elif lspci | grep -qi 'amd.*radeon\|amd.*navi\|amd.*vega'; then
        export VULCAN_GPU="amd"
        info "GPU: AMD detected"
    elif lspci | grep -qi 'intel.*graphics\|intel.*iris\|intel.*uhd'; then
        export VULCAN_GPU="intel"
        info "GPU: Intel detected"
    else
        export VULCAN_GPU="unknown"
        warn "GPU: Could not detect — skipping GPU-specific config"
    fi

    # CPU detection
    if grep -qi 'intel' /proc/cpuinfo; then
        export VULCAN_CPU="intel"
        info "CPU: Intel detected"
    elif grep -qi 'amd' /proc/cpuinfo; then
        export VULCAN_CPU="amd"
        info "CPU: AMD detected"
    else
        export VULCAN_CPU="unknown"
        warn "CPU: Could not detect"
    fi

    # Run GPU-specific configuration
    case "$VULCAN_GPU" in
        nvidia) source "$INSTALLER_DIR/hardware/nvidia.sh" && run_nvidia_config ;;
        amd)    source "$INSTALLER_DIR/hardware/amd.sh"    && run_amd_config ;;
        intel)  source "$INSTALLER_DIR/hardware/intel.sh"  && run_intel_config ;;
    esac

    success "Hardware detection complete (GPU=$VULCAN_GPU, CPU=$VULCAN_CPU)"
}
