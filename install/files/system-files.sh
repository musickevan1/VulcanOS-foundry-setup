#!/bin/bash
# =============================================================================
# VulcanOS Installer - System File Installation
# =============================================================================

run_system_files() {
    is_done "system-files" && { info "System files already installed, skipping"; return 0; }

    header "System files"

    local base_root="$VULCAN_REPO/archiso/base/airootfs"
    local user_home
    user_home=$(vulcan_home)

    # SDDM theme
    if [[ -d "$base_root/usr/share/sddm/themes/vulcanos" ]]; then
        mkdir -p /usr/share/sddm/themes/vulcanos
        rsync -a "$base_root/usr/share/sddm/themes/vulcanos/" /usr/share/sddm/themes/vulcanos/
        info "SDDM theme installed"
    fi

    # SDDM config
    if [[ -f "$base_root/etc/sddm.conf.d/vulcanos.conf" ]]; then
        mkdir -p /etc/sddm.conf.d
        cp "$base_root/etc/sddm.conf.d/vulcanos.conf" /etc/sddm.conf.d/
        info "SDDM config installed"
    fi

    # Custom scripts → /usr/local/bin/
    if [[ -d "$base_root/usr/local/bin" ]]; then
        local count=0
        for script in "$base_root/usr/local/bin/"*; do
            [[ -f "$script" ]] || continue
            install -m 755 "$script" /usr/local/bin/
            ((count++))
        done
        info "Installed $count scripts to /usr/local/bin/"
    fi

    # Desktop files
    if [[ -d "$base_root/usr/share/applications" ]]; then
        mkdir -p /usr/share/applications
        rsync -a "$base_root/usr/share/applications/" /usr/share/applications/
        info "Desktop files installed"
    fi

    # Icons
    if [[ -d "$base_root/usr/share/icons" ]]; then
        rsync -a "$base_root/usr/share/icons/" /usr/share/icons/
        info "Icons installed"
    fi

    # Backgrounds
    if [[ -d "$base_root/usr/share/backgrounds" ]]; then
        mkdir -p /usr/share/backgrounds
        rsync -a "$base_root/usr/share/backgrounds/" /usr/share/backgrounds/
        info "Backgrounds installed"
    fi

    # Skel files → user home (preserve existing)
    if [[ -d "$base_root/etc/skel" ]]; then
        rsync -a --ignore-existing "$base_root/etc/skel/" "$user_home/"
        chown -R "$(vulcan_user):$(vulcan_user)" "$user_home"
        info "Skeleton files synced to $user_home"
    fi

    mark_done "system-files"
    success "System files installed"
}
