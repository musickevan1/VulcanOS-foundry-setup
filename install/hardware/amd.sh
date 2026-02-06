#!/bin/bash
# =============================================================================
# VulcanOS Installer - AMD GPU Configuration
# =============================================================================

run_amd_config() {
    is_done "amd-gpu" && { info "AMD GPU config already done, skipping"; return 0; }

    header "AMD GPU configuration"

    # AMD GPUs generally work out of the box with mesa + amdgpu kernel module.
    # Add specific configuration here as needed.
    info "AMD GPU: using mesa/amdgpu defaults (no extra config needed)"

    mark_done "amd-gpu"
    success "AMD GPU configuration complete"
}
