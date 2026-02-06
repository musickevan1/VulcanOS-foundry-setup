#!/bin/bash
# =============================================================================
# VulcanOS Installer - Intel GPU Configuration
# =============================================================================

run_intel_config() {
    is_done "intel-gpu" && { info "Intel GPU config already done, skipping"; return 0; }

    header "Intel GPU configuration"

    # Intel iGPUs work out of the box with mesa + i915/xe kernel modules.
    # Add specific configuration here as needed.
    info "Intel GPU: using mesa/i915 defaults (no extra config needed)"

    mark_done "intel-gpu"
    success "Intel GPU configuration complete"
}
