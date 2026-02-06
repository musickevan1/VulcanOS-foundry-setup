#!/bin/bash
# =============================================================================
# VulcanOS Installer - Dotfiles via GNU Stow (auto-discovery)
# =============================================================================

run_dotfiles_stow() {
    is_done "dotfiles" && { info "Dotfiles already stowed, skipping"; return 0; }

    header "Dotfiles via GNU Stow"

    local dotfiles_dir="$VULCAN_REPO/dotfiles"
    local user_home
    user_home=$(vulcan_home)

    if [[ ! -d "$dotfiles_dir" ]]; then
        warn "Dotfiles directory not found: $dotfiles_dir"
        return 0
    fi

    # Auto-discover stow packages from dotfiles/*/
    local stow_packages=()
    for d in "$dotfiles_dir"/*/; do
        [[ -d "$d" ]] && stow_packages+=("$(basename "$d")")
    done

    if [[ ${#stow_packages[@]} -eq 0 ]]; then
        warn "No stow packages found in $dotfiles_dir"
        return 0
    fi

    info "Found ${#stow_packages[@]} stow packages: ${stow_packages[*]}"

    local stowed=0
    local failed=0

    for pkg in "${stow_packages[@]}"; do
        # Try --restow first, fall back to --adopt --restow on conflict
        if sudo -u "$(vulcan_user)" stow --restow -d "$dotfiles_dir" -t "$user_home" "$pkg" 2>/dev/null; then
            ((stowed++))
        elif sudo -u "$(vulcan_user)" stow --adopt --restow -d "$dotfiles_dir" -t "$user_home" "$pkg" 2>/dev/null; then
            ((stowed++))
            warn "$pkg: adopted existing files (check for conflicts)"
        else
            error "Failed to stow: $pkg"
            ((failed++))
        fi
    done

    info "Stowed $stowed/${#stow_packages[@]} packages ($failed failed)"

    mark_done "dotfiles"
    success "Dotfiles installed"
}
