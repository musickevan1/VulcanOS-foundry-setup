#!/bin/bash
# =============================================================================
# VulcanOS Installer - User Configuration
# =============================================================================

run_user_configure() {
    is_done "user-configure" && { info "User configuration already done, skipping"; return 0; }

    header "User configuration"

    local user
    user=$(vulcan_user)
    local groups="docker,video,input,wheel"

    usermod -aG "$groups" "$user"
    info "Added $user to groups: $groups"

    mark_done "user-configure"
    success "User configuration complete"
}
