#!/bin/bash
# =============================================================================
# VulcanOS Installer - Package Installation
# =============================================================================

# ISO-only packages that should NOT be installed on a real system
ISO_ONLY_PACKAGES=(
    mkinitcpio-archiso
    syslinux
    archinstall
)

run_package_install() {
    is_done "packages" && { info "Packages already installed, skipping"; return 0; }

    header "Package installation"

    local base_pkgs="$VULCAN_REPO/archiso/base/packages.base"
    local profile_pkgs="$VULCAN_REPO/archiso/profiles/${VULCAN_PROFILE}/packages.profile"

    if [[ ! -f "$base_pkgs" ]]; then
        error "Base package list not found: $base_pkgs"
        return 1
    fi

    # Build exclusion regex
    local exclude_pattern
    exclude_pattern=$(printf '%s\n' "${ISO_ONLY_PACKAGES[@]}" | paste -sd '|')

    # Merge and deduplicate
    local packages
    local sources=("$base_pkgs")
    [[ -f "$profile_pkgs" ]] && sources+=("$profile_pkgs")

    packages=$(cat "${sources[@]}" \
        | sed 's/#.*//' \
        | tr -s '[:space:]' '\n' \
        | grep -v '^$' \
        | grep -Evx "$exclude_pattern" \
        | sort -u)

    local count
    count=$(echo "$packages" | wc -l)
    info "Installing $count packages (base + ${VULCAN_PROFILE} profile)"

    # shellcheck disable=SC2086
    pac_install $packages

    mark_done "packages"
    success "Package installation complete"
}
