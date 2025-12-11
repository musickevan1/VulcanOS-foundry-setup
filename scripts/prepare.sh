#!/bin/bash
# =============================================================================
# VulcanOS - Preparation Script
# Downloads resources and prepares the build environment
# =============================================================================

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Verify project structure
verify_structure() {
    info "Verifying project structure..."

    local required_dirs=(
        "archiso"
        "archiso/airootfs"
        "archiso/airootfs/etc"
        "archiso/grub"
        "archiso/syslinux"
        "archiso/efiboot"
        "dotfiles"
        "scripts"
    )

    local required_files=(
        "archiso/packages.x86_64"
        "archiso/pacman.conf"
        "archiso/profiledef.sh"
    )

    for dir in "${required_dirs[@]}"; do
        if [[ ! -d "$PROJECT_DIR/$dir" ]]; then
            warn "Missing directory: $dir"
            mkdir -p "$PROJECT_DIR/$dir"
            info "  Created: $dir"
        fi
    done

    for file in "${required_files[@]}"; do
        if [[ ! -f "$PROJECT_DIR/$file" ]]; then
            warn "Missing file: $file"
        fi
    done

    success "Project structure verified"
}

# Check for AUR packages that need to be built
check_aur_packages() {
    info "Checking AUR package requirements..."

    # List of AUR packages we want to include
    local aur_packages=(
        "yay"
        # Add more as needed
    )

    info "AUR packages to build: ${aur_packages[*]}"
    warn "Note: AUR packages must be built separately using build-aur-repo.sh"

    success "AUR package check completed"
}

# Create necessary directories in airootfs
prepare_airootfs() {
    info "Preparing airootfs directories..."

    local dirs=(
        "etc/skel/.config"
        "etc/skel/.local/bin"
        "etc/skel/Pictures/Screenshots"
        "etc/skel/Documents"
        "etc/skel/Downloads"
        "etc/skel/Projects"
    )

    for dir in "${dirs[@]}"; do
        mkdir -p "$PROJECT_DIR/archiso/airootfs/$dir"
    done

    success "airootfs directories prepared"
}

# Create pacman mirrorlist
create_mirrorlist() {
    info "Creating pacman mirrorlist..."

    local mirrorlist_dir="$PROJECT_DIR/archiso/airootfs/etc/pacman.d"
    mkdir -p "$mirrorlist_dir"

    cat > "$mirrorlist_dir/mirrorlist" << 'EOF'
## Arch Linux repository mirrorlist
## Worldwide
Server = https://geo.mirror.pkgbuild.com/$repo/os/$arch
Server = https://mirror.rackspace.com/archlinux/$repo/os/$arch
## United States
Server = https://mirrors.kernel.org/archlinux/$repo/os/$arch
Server = https://mirror.arizona.edu/archlinux/$repo/os/$arch
Server = https://mirrors.mit.edu/archlinux/$repo/os/$arch
EOF

    success "Mirrorlist created"
}

# Create hyprpaper config (wallpaper)
create_hyprpaper_config() {
    info "Creating hyprpaper configuration..."

    local hypr_dir="$PROJECT_DIR/dotfiles/hypr"
    mkdir -p "$hypr_dir"

    cat > "$hypr_dir/hyprpaper.conf" << 'EOF'
# Hyprpaper Configuration
# Wallpaper daemon for Hyprland

preload = ~/Pictures/Wallpapers/wallpaper.png

# Set wallpaper for all monitors
wallpaper = ,~/Pictures/Wallpapers/wallpaper.png

# Disable splash
splash = false
EOF

    success "Hyprpaper configuration created"
}

# Create hypridle config
create_hypridle_config() {
    info "Creating hypridle configuration..."

    local hypr_dir="$PROJECT_DIR/dotfiles/hypr"

    cat > "$hypr_dir/hypridle.conf" << 'EOF'
# Hypridle Configuration
# Idle management for Hyprland

general {
    lock_cmd = hyprlock
    before_sleep_cmd = loginctl lock-session
    after_sleep_cmd = hyprctl dispatch dpms on
}

# Screen dimming after 2.5 minutes
listener {
    timeout = 150
    on-timeout = brightnessctl -s set 30
    on-resume = brightnessctl -r
}

# Lock screen after 5 minutes
listener {
    timeout = 300
    on-timeout = loginctl lock-session
}

# Turn off screen after 5.5 minutes
listener {
    timeout = 330
    on-timeout = hyprctl dispatch dpms off
    on-resume = hyprctl dispatch dpms on
}

# Suspend after 15 minutes
listener {
    timeout = 900
    on-timeout = systemctl suspend
}
EOF

    success "Hypridle configuration created"
}

# Create hyprlock config
create_hyprlock_config() {
    info "Creating hyprlock configuration..."

    local hypr_dir="$PROJECT_DIR/dotfiles/hypr"

    cat > "$hypr_dir/hyprlock.conf" << 'EOF'
# Hyprlock Configuration
# Lock screen for Hyprland

general {
    disable_loading_bar = false
    hide_cursor = true
    grace = 0
    no_fade_in = false
}

background {
    monitor =
    path = ~/Pictures/Wallpapers/wallpaper.png
    color = rgba(25, 20, 20, 1.0)

    blur_passes = 3
    blur_size = 8
    noise = 0.0117
    contrast = 0.8916
    brightness = 0.8172
    vibrancy = 0.1696
    vibrancy_darkness = 0.0
}

input-field {
    monitor =
    size = 250, 50
    outline_thickness = 3
    dots_size = 0.33
    dots_spacing = 0.15
    dots_center = true
    outer_color = rgb(89, 180, 250)
    inner_color = rgb(30, 30, 46)
    font_color = rgb(192, 202, 245)
    fade_on_empty = true
    placeholder_text = <i>Password...</i>
    hide_input = false
    rounding = 10
    check_color = rgb(158, 206, 106)
    fail_color = rgb(247, 118, 142)
    fail_text = <i>$FAIL <b>($ATTEMPTS)</b></i>
    fail_transition = 300

    position = 0, -20
    halign = center
    valign = center
}

label {
    monitor =
    text = $TIME
    color = rgba(192, 202, 245, 1.0)
    font_size = 90
    font_family = JetBrainsMono Nerd Font Bold
    position = 0, 80
    halign = center
    valign = center
}

label {
    monitor =
    text = Hi, $USER
    color = rgba(192, 202, 245, 1.0)
    font_size = 20
    font_family = JetBrainsMono Nerd Font
    position = 0, -70
    halign = center
    valign = center
}
EOF

    success "Hyprlock configuration created"
}

# Create starship config
create_starship_config() {
    info "Creating Starship prompt configuration..."

    local config_dir="$PROJECT_DIR/dotfiles/starship"
    mkdir -p "$config_dir"

    cat > "$config_dir/starship.toml" << 'EOF'
# Starship Prompt Configuration
# https://starship.rs

format = """
[](#7aa2f7)\
$os\
$username\
[](bg:#9ece6a fg:#7aa2f7)\
$directory\
[](fg:#9ece6a bg:#bb9af7)\
$git_branch\
$git_status\
[](fg:#bb9af7 bg:#e0af68)\
$c\
$rust\
$golang\
$nodejs\
$python\
[](fg:#e0af68 bg:#7dcfff)\
$docker_context\
[](fg:#7dcfff bg:#1a1b26)\
$time\
[ ](fg:#1a1b26)\
"""

[os]
disabled = false
style = "bg:#7aa2f7 fg:#1a1b26"

[os.symbols]
Arch = "󰣇"

[username]
show_always = true
style_user = "bg:#7aa2f7 fg:#1a1b26"
style_root = "bg:#7aa2f7 fg:#1a1b26"
format = '[$user ]($style)'
disabled = false

[directory]
style = "bg:#9ece6a fg:#1a1b26"
format = "[ $path ]($style)"
truncation_length = 3
truncation_symbol = "…/"

[git_branch]
symbol = ""
style = "bg:#bb9af7 fg:#1a1b26"
format = '[ $symbol $branch ]($style)'

[git_status]
style = "bg:#bb9af7 fg:#1a1b26"
format = '[$all_status$ahead_behind ]($style)'

[nodejs]
symbol = ""
style = "bg:#e0af68 fg:#1a1b26"
format = '[ $symbol ($version) ]($style)'

[rust]
symbol = ""
style = "bg:#e0af68 fg:#1a1b26"
format = '[ $symbol ($version) ]($style)'

[golang]
symbol = ""
style = "bg:#e0af68 fg:#1a1b26"
format = '[ $symbol ($version) ]($style)'

[python]
symbol = ""
style = "bg:#e0af68 fg:#1a1b26"
format = '[ $symbol ($version) ]($style)'

[docker_context]
symbol = ""
style = "bg:#7dcfff fg:#1a1b26"
format = '[ $symbol $context ]($style)'

[time]
disabled = false
time_format = "%R"
style = "bg:#1a1b26 fg:#c0caf5"
format = '[ $time ]($style)'

[character]
success_symbol = "[❯](bold green)"
error_symbol = "[❯](bold red)"
EOF

    success "Starship configuration created"
}

# Main execution
main() {
    echo "=========================================="
    echo "  NewOS - Build Preparation"
    echo "=========================================="
    echo ""

    verify_structure
    check_aur_packages
    prepare_airootfs
    create_mirrorlist
    create_hyprpaper_config
    create_hypridle_config
    create_hyprlock_config
    create_starship_config

    echo ""
    success "Preparation complete!"
    echo ""
    echo "Next steps:"
    echo "  1. (Optional) Build AUR packages: ./scripts/build-aur-repo.sh"
    echo "  2. Build the ISO: sudo ./scripts/build.sh"
    echo ""
}

main "$@"
