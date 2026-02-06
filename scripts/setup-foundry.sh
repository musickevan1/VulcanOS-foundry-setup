#!/bin/bash
# =============================================================================
# VulcanOS Foundry - Post-Install Setup Script
# Transforms a fresh Arch Linux install into a VulcanOS Foundry workstation
#
# Prerequisites:
#   1. Fresh Arch Linux install via archinstall (GRUB, base system, user in wheel)
#   2. VulcanOS repo cloned to ~/VulcanOS
#   3. Internet connectivity
#
# Usage: sudo ./scripts/setup-foundry.sh
# =============================================================================

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

# Source shared logging/color functions
source "$SCRIPT_DIR/lib/build-common.sh"

# Profile identity
PROFILE="foundry"

# Paths to repo config files
BASE_DIR="$PROJECT_DIR/archiso/base"
PROFILE_CONF="$PROJECT_DIR/archiso/profiles/$PROFILE"
DOTFILES_DIR="$PROJECT_DIR/dotfiles"

# ISO-only packages that should NOT be installed on a real system
ISO_ONLY_PACKAGES=(
    mkinitcpio-archiso
    syslinux
    archinstall
)

# Stow packages (all directories in dotfiles/)
STOW_PACKAGES=(
    alacritty applications bash git goimapnotify gtk-4.0 hypr kitty
    nvim nwg-dock-hyprland opencode scripts starship swaync systemd
    themes wallpapers waybar wofi yazi
)

# Services to enable
ENABLE_SERVICES=(
    sddm
    NetworkManager
    docker
)

# =============================================================================
# Phase 0: Preflight Checks
# =============================================================================

preflight() {
    echo ""
    echo "=========================================="
    echo "  VulcanOS Foundry - Setup"
    echo "=========================================="
    echo ""

    # Must be run as root via sudo
    if [[ $EUID -ne 0 ]]; then
        error "This script must be run with sudo"
        error "Usage: sudo ./scripts/setup-foundry.sh"
        exit 1
    fi

    if [[ -z "${SUDO_USER:-}" || "$SUDO_USER" == "root" ]]; then
        error "Must be run via sudo from a regular user account"
        error "Do not run as root directly — \$SUDO_USER is needed for stow"
        exit 1
    fi

    local user_home
    user_home=$(eval echo "~$SUDO_USER")
    info "Running as root, target user: $SUDO_USER (home: $user_home)"

    # Check Arch Linux
    if [[ ! -f /etc/arch-release ]]; then
        error "This script is designed for Arch Linux only"
        exit 1
    fi

    # Validate repo structure
    local required_files=(
        "$BASE_DIR/packages.base"
        "$PROFILE_CONF/packages.profile"
        "$DOTFILES_DIR"
    )
    for f in "${required_files[@]}"; do
        if [[ ! -e "$f" ]]; then
            error "Missing required path: $f"
            error "Is this script being run from the VulcanOS repo root?"
            exit 1
        fi
    done

    # Check internet
    if ! ping -c 1 -W 3 archlinux.org &>/dev/null; then
        error "No internet connectivity — cannot install packages"
        exit 1
    fi

    success "Preflight checks passed"
}

# =============================================================================
# Phase 1: System Configuration
# =============================================================================

configure_system() {
    info "Phase 1: System configuration"

    # Hostname
    local hostname_file="$PROFILE_CONF/airootfs/etc/hostname"
    if [[ -f "$hostname_file" ]]; then
        cp "$hostname_file" /etc/hostname
        info "  Hostname set to: $(cat /etc/hostname)"
    else
        echo "vulcan-foundry" > /etc/hostname
        info "  Hostname set to: vulcan-foundry (default)"
    fi

    # Locale
    if ! grep -q '^en_US.UTF-8' /etc/locale.gen; then
        sed -i 's/^#en_US.UTF-8 UTF-8/en_US.UTF-8 UTF-8/' /etc/locale.gen
        locale-gen
    fi
    echo "LANG=en_US.UTF-8" > /etc/locale.conf
    info "  Locale configured: en_US.UTF-8"

    # Sudoers (wheel NOPASSWD)
    local sudoers_src="$BASE_DIR/airootfs/etc/sudoers.d/wheel"
    if [[ -f "$sudoers_src" ]]; then
        install -m 440 "$sudoers_src" /etc/sudoers.d/wheel
        info "  Sudoers rule installed (wheel NOPASSWD)"
    fi

    # Enable multilib repository
    if grep -q '^\[multilib\]' /etc/pacman.conf; then
        info "  [multilib] already enabled"
    else
        # Uncomment the [multilib] section (header + Include line)
        sed -i '/^#\[multilib\]/{s/^#//;n;s/^#//}' /etc/pacman.conf
        info "  [multilib] enabled in pacman.conf"
    fi

    # ParallelDownloads
    if grep -q '^ParallelDownloads' /etc/pacman.conf; then
        sed -i 's/^ParallelDownloads.*/ParallelDownloads = 5/' /etc/pacman.conf
    else
        sed -i 's/^\[options\]/[options]\nParallelDownloads = 5/' /etc/pacman.conf
    fi
    info "  ParallelDownloads = 5"

    # Sync repos
    info "  Syncing package databases..."
    pacman -Sy --noconfirm

    success "System configuration complete"
}

# =============================================================================
# Phase 2: Package Installation
# =============================================================================

install_packages() {
    info "Phase 2: Package installation"

    # Build exclusion regex
    local exclude_pattern
    exclude_pattern=$(printf '%s\n' "${ISO_ONLY_PACKAGES[@]}" | paste -sd '|')

    # Parse and merge package lists
    local packages
    packages=$(cat "$BASE_DIR/packages.base" "$PROFILE_CONF/packages.profile" \
        | sed 's/#.*//' \
        | tr -s '[:space:]' '\n' \
        | grep -v '^$' \
        | grep -Evx "$exclude_pattern" \
        | sort -u)

    local count
    count=$(echo "$packages" | wc -l)
    info "  Installing $count packages (base + $PROFILE profile, excluding ISO-only)"

    # shellcheck disable=SC2086
    pacman -S --needed --noconfirm $packages

    success "Package installation complete"
}

# =============================================================================
# Phase 3: NVIDIA Configuration
# =============================================================================

configure_nvidia() {
    info "Phase 3: NVIDIA configuration"

    # mkinitcpio.conf — fresh for installed system (NOT the archiso version)
    cat > /etc/mkinitcpio.conf << 'MKINITCPIO'
# =============================================================================
# VulcanOS Foundry - mkinitcpio Configuration
# Generated by setup-foundry.sh — NVIDIA early KMS
# =============================================================================

MODULES=(nvidia nvidia_modeset nvidia_uvm nvidia_drm)
BINARIES=()
FILES=()
HOOKS=(base udev autodetect modconf kms block filesystems keyboard fsck)

COMPRESSION="zstd"
COMPRESSION_OPTIONS=(-c -T0 -19)
MKINITCPIO
    info "  mkinitcpio.conf written (installed-system hooks, NVIDIA early KMS)"

    # NVIDIA modprobe config
    mkdir -p /etc/modprobe.d
    cat > /etc/modprobe.d/nvidia.conf << 'MODPROBE'
# VulcanOS Foundry - NVIDIA DRM modeset + framebuffer
options nvidia_drm modeset=1 fbdev=1
MODPROBE
    info "  /etc/modprobe.d/nvidia.conf written"

    # GRUB kernel parameters
    local grub_default="/etc/default/grub"
    if [[ -f "$grub_default" ]]; then
        if ! grep -q 'nvidia-drm.modeset=1' "$grub_default"; then
            sed -i 's/^GRUB_CMDLINE_LINUX_DEFAULT="\(.*\)"/GRUB_CMDLINE_LINUX_DEFAULT="\1 nvidia-drm.modeset=1"/' "$grub_default"
            info "  Added nvidia-drm.modeset=1 to GRUB cmdline"
        else
            info "  nvidia-drm.modeset=1 already in GRUB cmdline"
        fi
    else
        warn "  /etc/default/grub not found — skipping GRUB kernel params"
    fi

    # NVIDIA pacman hook — auto-rebuild initramfs on nvidia updates
    mkdir -p /etc/pacman.d/hooks
    cat > /etc/pacman.d/hooks/nvidia.hook << 'HOOK'
[Trigger]
Operation = Install
Operation = Upgrade
Operation = Remove
Type = Package
Target = nvidia-open-dkms
Target = nvidia-utils
Target = linux
Target = linux-headers

[Action]
Description = Rebuilding initramfs after NVIDIA/kernel update...
Depends = mkinitcpio
When = PostTransaction
NeedsTargets
Exec = /usr/bin/mkinitcpio -P
HOOK
    info "  NVIDIA pacman hook installed"

    # Rebuild initramfs
    info "  Rebuilding initramfs..."
    mkinitcpio -P

    # Regenerate GRUB config
    if [[ -f "$grub_default" ]]; then
        info "  Regenerating GRUB config..."
        grub-mkconfig -o /boot/grub/grub.cfg
    fi

    success "NVIDIA configuration complete"
}

# =============================================================================
# Phase 4: System Files
# =============================================================================

install_system_files() {
    info "Phase 4: System files"

    local base_root="$BASE_DIR/airootfs"
    local user_home
    user_home=$(eval echo "~$SUDO_USER")

    # SDDM theme
    if [[ -d "$base_root/usr/share/sddm/themes/vulcanos" ]]; then
        mkdir -p /usr/share/sddm/themes/vulcanos
        rsync -a "$base_root/usr/share/sddm/themes/vulcanos/" /usr/share/sddm/themes/vulcanos/
        info "  SDDM theme installed"
    fi

    # SDDM config
    if [[ -f "$base_root/etc/sddm.conf.d/vulcanos.conf" ]]; then
        mkdir -p /etc/sddm.conf.d
        cp "$base_root/etc/sddm.conf.d/vulcanos.conf" /etc/sddm.conf.d/
        info "  SDDM config installed"
    fi

    # Custom scripts → /usr/local/bin/
    if [[ -d "$base_root/usr/local/bin" ]]; then
        local script_count=0
        for script in "$base_root/usr/local/bin/"*; do
            [[ -f "$script" ]] || continue
            install -m 755 "$script" /usr/local/bin/
            ((script_count++))
        done
        info "  Installed $script_count scripts to /usr/local/bin/"
    fi

    # Desktop files → /usr/share/applications/
    if [[ -d "$base_root/usr/share/applications" ]]; then
        mkdir -p /usr/share/applications
        rsync -a "$base_root/usr/share/applications/" /usr/share/applications/
        info "  Desktop files installed"
    fi

    # Icons → /usr/share/icons/
    if [[ -d "$base_root/usr/share/icons" ]]; then
        rsync -a "$base_root/usr/share/icons/" /usr/share/icons/
        info "  Icons installed"
    fi

    # Backgrounds → /usr/share/backgrounds/
    if [[ -d "$base_root/usr/share/backgrounds" ]]; then
        mkdir -p /usr/share/backgrounds
        rsync -a "$base_root/usr/share/backgrounds/" /usr/share/backgrounds/
        info "  Backgrounds installed"
    fi

    # Skel files → user home (don't overwrite existing)
    if [[ -d "$base_root/etc/skel" ]]; then
        rsync -a --ignore-existing "$base_root/etc/skel/" "$user_home/"
        chown -R "$SUDO_USER:$SUDO_USER" "$user_home"
        info "  Skeleton files synced to $user_home (existing files preserved)"
    fi

    success "System files installed"
}

# =============================================================================
# Phase 5: Dotfiles (GNU Stow)
# =============================================================================

install_dotfiles() {
    info "Phase 5: Dotfiles via GNU Stow"

    local user_home
    user_home=$(eval echo "~$SUDO_USER")

    local stowed=0
    local failed=0

    for pkg in "${STOW_PACKAGES[@]}"; do
        if [[ ! -d "$DOTFILES_DIR/$pkg" ]]; then
            warn "  Stow package not found: $pkg (skipping)"
            continue
        fi

        # Try --restow first (clean re-link), fall back to --adopt --restow on conflict
        if sudo -u "$SUDO_USER" stow --restow -d "$DOTFILES_DIR" -t "$user_home" "$pkg" 2>/dev/null; then
            ((stowed++))
        elif sudo -u "$SUDO_USER" stow --adopt --restow -d "$DOTFILES_DIR" -t "$user_home" "$pkg" 2>/dev/null; then
            ((stowed++))
            warn "  $pkg: adopted existing files into stow (check for conflicts)"
        else
            error "  Failed to stow: $pkg"
            ((failed++))
        fi
    done

    info "  Stowed $stowed packages ($failed failed)"
    success "Dotfiles installed"
}

# =============================================================================
# Phase 6: Service Enablement
# =============================================================================

enable_services() {
    info "Phase 6: Enabling services"

    for service in "${ENABLE_SERVICES[@]}"; do
        if systemctl enable "$service" 2>/dev/null; then
            info "  Enabled: $service"
        else
            warn "  Could not enable: $service (may not be installed)"
        fi
    done

    success "Services enabled"
}

# =============================================================================
# Phase 7: User Configuration
# =============================================================================

configure_user() {
    info "Phase 7: User configuration"

    local groups="docker,video,input,wheel"
    usermod -aG "$groups" "$SUDO_USER"
    info "  Added $SUDO_USER to groups: $groups"

    success "User configuration complete"
}

# =============================================================================
# Phase 8: Summary
# =============================================================================

print_summary() {
    echo ""
    echo "=========================================="
    echo "  VulcanOS Foundry - Setup Complete!"
    echo "=========================================="
    echo ""
    echo "Next steps:"
    echo ""
    echo "  1. Reboot:"
    echo "       sudo reboot"
    echo ""
    echo "  2. After reboot, log in via SDDM → Hyprland"
    echo ""
    echo "  3. Install OpenCode (AI assistant):"
    echo "       curl -fsSL https://opencode.ai/install | bash"
    echo ""
    echo "  4. PyTorch nightly for RTX 5070 Ti (sm_120):"
    echo "       pip install --pre torch torchvision torchaudio \\"
    echo "         --index-url https://download.pytorch.org/whl/nightly/cu128"
    echo ""
    echo "  5. Optional — install yay for AUR packages:"
    echo "       git clone https://aur.archlinux.org/yay.git /tmp/yay"
    echo "       cd /tmp/yay && makepkg -si"
    echo ""
    echo "Verify:"
    echo "  hostname           → vulcan-foundry"
    echo "  nvidia-smi         → GPU detected"
    echo "  nvcc --version     → CUDA toolkit"
    echo "  ls -la ~/.config/hypr → stow symlink to VulcanOS"
    echo ""
}

# =============================================================================
# Main
# =============================================================================

main() {
    preflight
    configure_system
    install_packages
    configure_nvidia
    install_system_files
    install_dotfiles
    enable_services
    configure_user
    print_summary
}

main "$@"
