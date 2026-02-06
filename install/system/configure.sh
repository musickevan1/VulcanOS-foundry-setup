#!/bin/bash
# =============================================================================
# VulcanOS Installer - System Configuration
# =============================================================================

run_system_configure() {
    is_done "system-configure" && { info "System configuration already done, skipping"; return 0; }

    header "System configuration"

    # Hostname (from profile or default)
    local hostname_file="$VULCAN_REPO/archiso/profiles/${VULCAN_PROFILE}/airootfs/etc/hostname"
    if [[ -f "$hostname_file" ]]; then
        cp "$hostname_file" /etc/hostname
    else
        echo "vulcan-${VULCAN_PROFILE}" > /etc/hostname
    fi
    info "Hostname: $(cat /etc/hostname)"

    # Locale
    if ! grep -q '^en_US.UTF-8' /etc/locale.gen; then
        sed -i 's/^#en_US.UTF-8 UTF-8/en_US.UTF-8 UTF-8/' /etc/locale.gen
        locale-gen
    fi
    echo "LANG=en_US.UTF-8" > /etc/locale.conf
    info "Locale: en_US.UTF-8"

    # Sudoers — wheel NOPASSWD
    local sudoers_src="$VULCAN_REPO/archiso/base/airootfs/etc/sudoers.d/wheel"
    if [[ -f "$sudoers_src" ]]; then
        install -m 440 "$sudoers_src" /etc/sudoers.d/wheel
        info "Sudoers: wheel NOPASSWD"
    fi

    # Enable multilib
    if ! grep -q '^\[multilib\]' /etc/pacman.conf; then
        sed -i '/^#\[multilib\]/{s/^#//;n;s/^#//}' /etc/pacman.conf
        info "Enabled [multilib]"
    fi

    # ParallelDownloads
    if grep -q '^ParallelDownloads' /etc/pacman.conf; then
        sed -i 's/^ParallelDownloads.*/ParallelDownloads = 5/' /etc/pacman.conf
    else
        sed -i 's/^\[options\]/[options]\nParallelDownloads = 5/' /etc/pacman.conf
    fi

    # Pacman candy (Color + ILoveCandy)
    if ! grep -q '^Color' /etc/pacman.conf; then
        sed -i 's/^#Color/Color/' /etc/pacman.conf
    fi
    if ! grep -q '^ILoveCandy' /etc/pacman.conf; then
        sed -i '/^Color/a ILoveCandy' /etc/pacman.conf
    fi
    info "Pacman: Color + ILoveCandy + ParallelDownloads=5"

    # Sync repos
    info "Syncing package databases..."
    pacman -Sy --noconfirm

    mark_done "system-configure"
    success "System configuration complete"
}
