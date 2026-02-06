#!/bin/bash
# =============================================================================
# VulcanOS Installer - Service Enablement
# =============================================================================

run_enable_services() {
    is_done "services" && { info "Services already enabled, skipping"; return 0; }

    header "Enabling services"

    # Core services (always enabled)
    local services=(
        sddm
        NetworkManager
        docker
    )

    for svc in "${services[@]}"; do
        if systemctl enable "$svc" 2>/dev/null; then
            info "Enabled: $svc"
        else
            warn "Could not enable: $svc (may not be installed)"
        fi
    done

    # NVIDIA-specific services (suspend/hibernate/resume)
    if [[ "${VULCAN_GPU:-}" == "nvidia" ]]; then
        for svc in nvidia-suspend nvidia-hibernate nvidia-resume; do
            if systemctl enable "$svc.service" 2>/dev/null; then
                info "Enabled: $svc"
            else
                warn "Could not enable: $svc"
            fi
        done
    fi

    mark_done "services"
    success "Services enabled"
}
