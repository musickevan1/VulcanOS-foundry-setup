# VulcanOS Control Center Enhancement Plan

## Executive Summary

Transform VulcanOS from a functional system into a **polished, user-friendly distribution** with a comprehensive control center, rich theme library, version-controlled updates, and curated app installation system.

**Target Version:** 0.2.0 "Forge"

---

## Current State Analysis

### ✅ **What Works Well**
- **Vulcan Menu**: 831-line well-structured bash script with 8 categories
- **Theme System**: `vulcan-theme` with 8 themes and template-based config generation
- **Waybar**: Gruvbox-themed, functional modules
- **Keybindings**: Omarchy-inspired, keyboard-first workflow

### ❌ **Gaps Identified**
1. **Waybar**: Orchestra module present, no visual menu launcher
2. **Vulcan Menu Bugs**: Display Settings uses non-existent `info` command
3. **Incomplete Features**: Default Apps and Restore Defaults show "coming soon"
4. **Limited Themes**: Only 8 themes, 7 Vulcan wallpapers (mostly SVG/login-focused)
5. **No Version Control**: No GitHub sync, manual updates only
6. **Install Menu**: Generic package search, not curated/modular
7. **Config Access**: Scattered across multiple submenus

---

## Phase 1: Waybar & Core Menu Fixes (Foundation)

**Priority:** CRITICAL  
**Estimated Time:** 2-3 hours

### 1.1 Waybar Layout Enhancement

**Changes:**
```jsonc
"modules-left": [
    "custom/vulcan-menu",     // NEW: 󰍜 icon
    "custom/separator",       // NEW
    "hyprland/workspaces",
    "custom/separator",
    "hyprland/window",
    "custom/separator",
    "custom/s2t"
],

"modules-right": [
    "tray",
    "bluetooth",
    "network",
    "pulseaudio",
    "cpu",
    "memory",
    "battery"
    // orchestra REMOVED
]
```

**New Module Config:**
```jsonc
"custom/vulcan-menu": {
    "format": "󰍜",
    "on-click": "vulcan-menu",
    "tooltip-format": "Vulcan System Menu (Super+Alt+Space)",
    "interval": "once"
}
```

**CSS Styling:**
```css
#custom-vulcan-menu {
    background-color: #3c3836;
    color: #458588;  /* Gruvbox blue accent */
    padding: 0 12px;
    margin: 4px 2px;
    border-radius: 4px;
    border: 1px solid #504945;
    font-size: 16px;
}

#custom-vulcan-menu:hover {
    background-color: #458588;
    color: #282828;
    border-color: #458588;
}
```

**Files Modified:**
- `dotfiles/waybar/.config/waybar/config.jsonc`
- `dotfiles/waybar/.config/waybar/style.css`

---

### 1.2 Vulcan Menu Bug Fixes

**Bug #1: Display Settings (Line 123)**
- **Issue**: Uses `info` command (doesn't exist)
- **Fix**: Convert to wofi submenu like other menus

**Implementation:**
```bash
show_display_settings_menu() {
    local options="󰍹  HyprMon (TUI)
󰍹  nwg-displays (GUI)  
󰸉  Wallpaper Profiles
$ICON_BACK  Back"
    
    local choice
    choice=$(wofi_menu "Display Settings" "$options" 320 240)
    
    local action
    action=$(echo "$choice" | sed 's/^[^ ]* *//')
    
    case "$action" in
        "HyprMon (TUI)")
            if command -v hyprmon &> /dev/null; then
                run_in_terminal hyprmon
            else
                notify "HyprMon not found. Install with: yay -S hyprmon-bin"
            fi
            ;;
        "nwg-displays (GUI)")
            if command -v nwg-displays &> /dev/null; then
                nwg-displays &
            else
                notify "nwg-displays not found. Install with: pacman -S nwg-displays"
            fi
            ;;
        "Wallpaper Profiles")
            if command -v vulcan-wallpapers &> /dev/null; then
                run_in_terminal vulcan-wallpapers list
            else
                notify "vulcan-wallpapers not found."
            fi
            ;;
        "Back")
            show_system_menu
            ;;
        *)
            exit 0
            ;;
    esac
}
```

**Bug #2: Default Applications (Line 517)**
- **Current**: Shows "coming soon" notification
- **Fix**: Implement xdg-mime-based default app selector

**Implementation:**
```bash
show_defaults_menu() {
    local options="󰆍  Default Terminal
󰖟  Default Browser
󰨞  Default Editor
󰉋  Default File Manager
$ICON_BACK  Back"
    
    local choice
    choice=$(wofi_menu "Default Applications" "$options" 350 280)
    
    local action
    action=$(echo "$choice" | sed 's/^[^ ]* *//')
    
    case "$action" in
        "Default Terminal")     set_default_terminal ;;
        "Default Browser")      set_default_browser ;;
        "Default Editor")       set_default_editor ;;
        "Default File Manager") set_default_file_manager ;;
        "Back")                 show_setup_menu ;;
        *)                      exit 0 ;;
    esac
}

set_default_terminal() {
    local terminals="kitty
alacritty
wezterm
$ICON_BACK  Back"
    
    local choice
    choice=$(wofi_menu "Default Terminal" "$terminals" 280 240)
    
    case "$choice" in
        "kitty")
            xdg-mime default kitty.desktop x-scheme-handler/terminal
            notify "Default terminal set to Kitty"
            ;;
        "alacritty")
            xdg-mime default Alacritty.desktop x-scheme-handler/terminal
            notify "Default terminal set to Alacritty"
            ;;
        "wezterm")
            xdg-mime default org.wezfurlong.wezterm.desktop x-scheme-handler/terminal
            notify "Default terminal set to WezTerm"
            ;;
        "Back")
            show_defaults_menu
            ;;
    esac
}

set_default_browser() {
    local browsers="firefox
chromium
brave
zen-browser
$ICON_BACK  Back"
    
    local choice
    choice=$(wofi_menu "Default Browser" "$browsers" 280 260)
    
    case "$choice" in
        "firefox")
            xdg-mime default firefox.desktop x-scheme-handler/http
            xdg-mime default firefox.desktop x-scheme-handler/https
            notify "Default browser set to Firefox"
            ;;
        "chromium")
            xdg-mime default chromium.desktop x-scheme-handler/http
            xdg-mime default chromium.desktop x-scheme-handler/https
            notify "Default browser set to Chromium"
            ;;
        "brave")
            xdg-mime default brave-browser.desktop x-scheme-handler/http
            xdg-mime default brave-browser.desktop x-scheme-handler/https
            notify "Default browser set to Brave"
            ;;
        "zen-browser")
            xdg-mime default zen-browser.desktop x-scheme-handler/http
            xdg-mime default zen-browser.desktop x-scheme-handler/https
            notify "Default browser set to Zen Browser"
            ;;
        "Back")
            show_defaults_menu
            ;;
    esac
}

set_default_editor() {
    local editors="neovim
code
sublime-text
gedit
$ICON_BACK  Back"
    
    local choice
    choice=$(wofi_menu "Default Editor" "$editors" 280 260)
    
    case "$choice" in
        "neovim")
            xdg-mime default nvim.desktop text/plain
            notify "Default editor set to Neovim"
            ;;
        "code")
            xdg-mime default code.desktop text/plain
            notify "Default editor set to VS Code"
            ;;
        "sublime-text")
            xdg-mime default sublime_text.desktop text/plain
            notify "Default editor set to Sublime Text"
            ;;
        "gedit")
            xdg-mime default gedit.desktop text/plain
            notify "Default editor set to gedit"
            ;;
        "Back")
            show_defaults_menu
            ;;
    esac
}

set_default_file_manager() {
    local managers="thunar
nautilus
dolphin
ranger
$ICON_BACK  Back"
    
    local choice
    choice=$(wofi_menu "Default File Manager" "$managers" 280 240)
    
    case "$choice" in
        "thunar")
            xdg-mime default thunar.desktop inode/directory
            notify "Default file manager set to Thunar"
            ;;
        "nautilus")
            xdg-mime default nautilus.desktop inode/directory
            notify "Default file manager set to Nautilus"
            ;;
        "dolphin")
            xdg-mime default dolphin.desktop inode/directory
            notify "Default file manager set to Dolphin"
            ;;
        "ranger")
            notify "Ranger is CLI-only, cannot be set as GUI default"
            ;;
        "Back")
            show_defaults_menu
            ;;
    esac
}
```

**Bug #3: Restore Defaults (Line 651)**
- **Current**: Shows "coming soon"
- **Fix**: Implement with source selection prompt (both /etc/skel and repo)

**Implementation:**
```bash
show_restore_menu() {
    local options="󰖬  Restore Hyprland Config
󰀻  Restore Waybar Config
󰗼  Restore All Configs
$ICON_BACK  Back"
    
    local choice
    choice=$(wofi_menu "Restore Defaults" "$options" 350 240)
    
    local action
    action=$(echo "$choice" | sed 's/^[^ ]* *//')
    
    case "$action" in
        "Restore Hyprland Config")  restore_config "hypr" ;;
        "Restore Waybar Config")    restore_config "waybar" ;;
        "Restore All Configs")      restore_all_configs ;;
        "Back")                     show_update_menu ;;
        *)                          exit 0 ;;
    esac
}

restore_config() {
    local config_name="$1"
    
    # Ask user for source
    local source_options="󰊢  System Defaults (/etc/skel)
󰊢  VulcanOS Repository
$ICON_BACK  Cancel"
    
    local source_choice
    source_choice=$(wofi_menu "Restore Source" "$source_options" 350 200)
    
    local source_action
    source_action=$(echo "$source_choice" | sed 's/^[^ ]* *//')
    
    case "$source_action" in
        "System Defaults (/etc/skel)")
            restore_from_skel "$config_name"
            ;;
        "VulcanOS Repository")
            restore_from_repo "$config_name"
            ;;
        *)
            return
            ;;
    esac
}

restore_from_skel() {
    local config_name="$1"
    local backup_source="/etc/skel/.config/${config_name}"
    local config_target="${CONFIG_DIR}/${config_name}"
    
    if [[ ! -d "$backup_source" ]]; then
        notify "Source not found: $backup_source"
        return 1
    fi
    
    # Create backup
    cp -r "$config_target" "${config_target}.backup.$(date +%s)" 2>/dev/null || true
    
    # Restore
    cp -r "$backup_source" "$config_target"
    notify "Restored ${config_name} from /etc/skel. Backup saved."
    
    # Reload if Hyprland
    if [[ "$config_name" == "hypr" ]]; then
        hyprctl reload
    elif [[ "$config_name" == "waybar" ]]; then
        pkill -SIGUSR2 waybar
    fi
}

restore_from_repo() {
    local config_name="$1"
    local repo_path="/home/evan/VulcanOS/dotfiles/${config_name}/.config/${config_name}"
    local config_target="${CONFIG_DIR}/${config_name}"
    
    if [[ ! -d "$repo_path" ]]; then
        notify "Repository source not found: $repo_path"
        return 1
    fi
    
    # Create backup
    cp -r "$config_target" "${config_target}.backup.$(date +%s)" 2>/dev/null || true
    
    # Restore
    cp -r "$repo_path" "$config_target"
    notify "Restored ${config_name} from VulcanOS repo. Backup saved."
    
    # Reload
    if [[ "$config_name" == "hypr" ]]; then
        hyprctl reload
    elif [[ "$config_name" == "waybar" ]]; then
        pkill -SIGUSR2 waybar
    fi
}

restore_all_configs() {
    local configs_to_restore="hypr waybar kitty alacritty wofi swaync"
    
    local source_options="󰊢  System Defaults (/etc/skel)
󰊢  VulcanOS Repository
$ICON_BACK  Cancel"
    
    local source_choice
    source_choice=$(wofi_menu "Restore All From" "$source_options" 350 200)
    
    local source_action
    source_action=$(echo "$source_choice" | sed 's/^[^ ]* *//')
    
    case "$source_action" in
        "System Defaults (/etc/skel)")
            for config in $configs_to_restore; do
                restore_from_skel "$config" 2>/dev/null || true
            done
            notify "All configs restored from /etc/skel"
            ;;
        "VulcanOS Repository")
            for config in $configs_to_restore; do
                restore_from_repo "$config" 2>/dev/null || true
            done
            notify "All configs restored from VulcanOS repo"
            ;;
    esac
}
```

---

## Phase 2: Theme System Expansion

**Priority:** HIGH  
**Estimated Time:** 4-6 hours (themes) + 3-4 hours (wallpapers)

### 2.1 Additional Themes to Create

**Target: 15+ curated themes**

**New Themes Needed:**
1. **Solarized Dark** - Classic warm dark theme
2. **Ayu Dark** - Modern minimalist
3. **Material Deep Ocean** - Deep blue tones
4. **Everforest** - Warm green forest tones
5. **Kanagawa** - Japanese-inspired waves
6. **Monokai Pro** - Vibrant coding theme
7. **Palenight** - Material theme variant

**Theme File Template:**
```bash
# /home/evan/VulcanOS/dotfiles/themes/colors/{theme-id}.sh

export THEME_NAME="Theme Name"
export THEME_ID="theme-id"

# Background colors
export BG_PRIMARY="#000000"
export BG_SECONDARY="#111111"
export BG_TERTIARY="#222222"
export BG_SURFACE="#000000"

# Foreground colors
export FG_PRIMARY="#ffffff"
export FG_SECONDARY="#eeeeee"
export FG_MUTED="#888888"

# Accent colors
export ACCENT="#0099ff"
export ACCENT_ALT="#ff0099"

# Semantic colors
export RED="#ff0000"
export GREEN="#00ff00"
export YELLOW="#ffff00"
export BLUE="#0000ff"
export PURPLE="#ff00ff"
export CYAN="#00ffff"
export ORANGE="#ff9900"
export PINK="#ff99cc"

# Bright variants
export BRIGHT_RED="#ff3333"
export BRIGHT_GREEN="#33ff33"
export BRIGHT_YELLOW="#ffff33"
export BRIGHT_BLUE="#3333ff"
export BRIGHT_PURPLE="#ff33ff"
export BRIGHT_CYAN="#33ffff"

# UI specific
export BORDER_ACTIVE="#0099ff"
export BORDER_INACTIVE="#333333"
export SELECTION="#222222"
export CURSOR="#ffffff"

# Gradient colors
export GRADIENT_START="#0099ff"
export GRADIENT_END="#ff0099"

# GTK/Qt theming
export GTK_THEME="Adwaita-dark"
export ICON_THEME="Papirus-Dark"
export CURSOR_THEME="Adwaita"
export KVANTUM_THEME="KvGnomeDark"

# Neovim colorscheme
export NVIM_COLORSCHEME="theme-name"

# Wallpaper (optional)
export THEME_WALLPAPER="theme-wallpaper.png"
```

**Update vulcan-theme THEMES array:**
```bash
THEMES=(
    "vulcan-forge:Vulcan Forge:Warm forge-inspired colors"
    "tokyonight:Tokyo Night:Soft dark theme with vibrant colors"
    "catppuccin-mocha:Catppuccin Mocha:Soothing pastel theme"
    "dracula:Dracula:Dark theme with purple accents"
    "gruvbox-dark:Gruvbox Dark:Retro groove colors"
    "nord:Nord:Arctic, bluish color palette"
    "onedark:One Dark:Atom's iconic dark theme"
    "rosepine:Rose Pine:All natural pine, faux fur and soho vibes"
    "solarized-dark:Solarized Dark:Classic warm dark theme"
    "ayu-dark:Ayu Dark:Modern minimalist dark theme"
    "material-ocean:Material Deep Ocean:Deep blue ocean tones"
    "everforest:Everforest:Warm green forest atmosphere"
    "kanagawa:Kanagawa:Japanese-inspired wave theme"
    "monokai-pro:Monokai Pro:Vibrant coding theme"
    "palenight:Palenight:Material theme elegant variant"
)
```

### 2.2 Wallpaper Library Expansion

**Current**: 7 Vulcan-branded wallpapers (SVG/login-focused)

**Target**: 50+ high-quality wallpapers organized by theme/category

**Directory Structure:**
```
~/Pictures/Wallpapers/
├── landscapes/      # 10 wallpapers
├── abstract/        # 10 wallpapers
├── space/          # 5 wallpapers
├── minimal/        # 10 wallpapers
├── anime/          # 5 wallpapers
└── themes/         # 10 wallpapers (one per theme)
```

**Sources:**
- Unsplash (free, high-quality landscapes)
- r/wallpapers curated collection
- Theme-specific wallpaper repos (TokyoNight, Catppuccin, Gruvbox, etc.)
- Custom Vulcan-branded designs

**Enhanced Wallpaper Menu:**
```bash
show_wallpaper_menu() {
    local options="󰥶  Browse by Category
󰏘  Browse by Theme
󰷊  Random Wallpaper
󰋩  Download More
$ICON_BACK  Back"
    
    local choice
    choice=$(wofi_menu "Wallpaper" "$options" 300 280)
    
    local action
    action=$(echo "$choice" | sed 's/^[^ ]* *//')
    
    case "$action" in
        "Browse by Category")
            show_wallpaper_category_menu
            ;;
        "Browse by Theme")
            show_wallpaper_theme_menu
            ;;
        "Random Wallpaper")
            set_random_wallpaper
            ;;
        "Download More")
            xdg-open "https://unsplash.com/wallpapers" &
            ;;
        "Back")
            show_style_menu
            ;;
    esac
}

show_wallpaper_category_menu() {
    local categories="Landscapes
Abstract
Space
Minimal
Anime
Theme-Matched"
    
    local choice
    choice=$(wofi_menu "Wallpaper Category" "$categories" 280 340)
    
    if [[ -z "$choice" ]]; then
        return
    fi
    
    local category_dir="${HOME}/Pictures/Wallpapers/${choice,,}"
    
    if [[ -d "$category_dir" ]]; then
        local wallpaper
        wallpaper=$(find "$category_dir" -type f \( -name "*.jpg" -o -name "*.png" \) | \
            xargs -I {} basename {} | \
            wofi --dmenu --prompt "Select Wallpaper" --cache-file /dev/null 2>/dev/null)
        
        if [[ -n "$wallpaper" ]]; then
            set_wallpaper "${category_dir}/${wallpaper}"
        fi
    else
        notify "Category not found: $choice"
    fi
}
```

---

## Phase 3: Config Access Menu (NEW)

**Priority:** MEDIUM  
**Estimated Time:** 2 hours

**Add to Main Menu:**
```bash
show_main_menu() {
    local options="$ICON_SYSTEM  System
$ICON_STYLE  Style
󰏗  Configs        # NEW
$ICON_INSTALL  Install
$ICON_REMOVE  Remove
$ICON_SETUP  Setup
$ICON_UPDATE  Update
$ICON_T2  T2 MacBook
$ICON_POWER  Power"
    
    local choice
    choice=$(wofi_menu "Vulcan Menu" "$options")
    
    local action
    action=$(echo "$choice" | sed 's/^[^ ]* *//' | tr '[:upper:]' '[:lower:]')
    
    case "$action" in
        "system")     show_system_menu ;;
        "style")      show_style_menu ;;
        "configs")    show_configs_menu ;;  # NEW
        "install")    show_install_menu ;;
        "remove")     show_remove_menu ;;
        "setup")      show_setup_menu ;;
        "update")     show_update_menu ;;
        "t2 macbook") show_t2_menu ;;
        "power")      exec vulcan-power ;;
        *)            exit 0 ;;
    esac
}
```

**New Configs Menu:**
```bash
show_configs_menu() {
    local options="󰖬  Hyprland
󰀻  Waybar
󰆍  Terminal (Kitty/Alacritty)
󰏘  Themes
󰸉  Wallpapers
󰒒  Autostart
󰌌  Input Devices
󰍹  Displays
󰑓  Reload All Configs
$ICON_BACK  Back"
    
    local choice
    choice=$(wofi_menu "Configuration Files" "$options")
    
    local action
    action=$(echo "$choice" | sed 's/^[^ ]* *//')
    
    case "$action" in
        "Hyprland")
            show_hyprland_config_menu
            ;;
        "Waybar")
            run_in_terminal $EDITOR "${CONFIG_DIR}/waybar/config.jsonc"
            ;;
        "Terminal (Kitty/Alacritty)")
            show_terminal_config_menu
            ;;
        "Themes")
            run_in_terminal $EDITOR "${CONFIG_DIR}/themes/colors/"
            ;;
        "Wallpapers")
            thunar "${HOME}/Pictures/Wallpapers" &
            ;;
        "Autostart")
            run_in_terminal $EDITOR "${CONFIG_DIR}/hypr/autostart.conf"
            ;;
        "Input Devices")
            run_in_terminal $EDITOR "${CONFIG_DIR}/hypr/input.conf"
            ;;
        "Displays")
            run_in_terminal $EDITOR "${CONFIG_DIR}/hypr/monitors.conf"
            ;;
        "Reload All Configs")
            hyprctl reload
            pkill -SIGUSR2 waybar 2>/dev/null || true
            notify "All configs reloaded"
            ;;
        "Back")
            show_main_menu
            ;;
        *)
            exit 0
            ;;
    esac
}

show_hyprland_config_menu() {
    local options="󰖬  Main Config (hyprland.conf)
󰌌  Bindings (bindings.conf)
󰍹  Monitors (monitors.conf)
󰏘  Appearance (looknfeel.conf)
󰒒  Autostart (autostart.conf)
󰑒  Window Rules (windowrules.conf)
󰍹  Input (input.conf)
󰖼  Environment (envs.conf)
$ICON_BACK  Back"
    
    local choice
    choice=$(wofi_menu "Hyprland Config" "$options" 400 480)
    
    local action
    action=$(echo "$choice" | sed 's/^[^ ]* *//' | sed 's/ (.*//')
    
    case "$action" in
        "Main Config")      run_in_terminal $EDITOR "${CONFIG_DIR}/hypr/hyprland.conf" ;;
        "Bindings")         run_in_terminal $EDITOR "${CONFIG_DIR}/hypr/bindings.conf" ;;
        "Monitors")         run_in_terminal $EDITOR "${CONFIG_DIR}/hypr/monitors.conf" ;;
        "Appearance")       run_in_terminal $EDITOR "${CONFIG_DIR}/hypr/looknfeel.conf" ;;
        "Autostart")        run_in_terminal $EDITOR "${CONFIG_DIR}/hypr/autostart.conf" ;;
        "Window Rules")     run_in_terminal $EDITOR "${CONFIG_DIR}/hypr/windowrules.conf" ;;
        "Input")            run_in_terminal $EDITOR "${CONFIG_DIR}/hypr/input.conf" ;;
        "Environment")      run_in_terminal $EDITOR "${CONFIG_DIR}/hypr/envs.conf" ;;
        "Back")             show_configs_menu ;;
    esac
}

show_terminal_config_menu() {
    local options="󰆍  Kitty
󰆍  Alacritty
$ICON_BACK  Back"
    
    local choice
    choice=$(wofi_menu "Terminal Config" "$options" 250 200)
    
    case "$choice" in
        *"Kitty"*)
            if [[ -f "${CONFIG_DIR}/kitty/kitty.conf" ]]; then
                run_in_terminal $EDITOR "${CONFIG_DIR}/kitty/kitty.conf"
            else
                notify "Kitty config not found"
            fi
            ;;
        *"Alacritty"*)
            if [[ -f "${CONFIG_DIR}/alacritty/alacritty.toml" ]]; then
                run_in_terminal $EDITOR "${CONFIG_DIR}/alacritty/alacritty.toml"
            else
                notify "Alacritty config not found"
            fi
            ;;
        *"Back"*)
            show_configs_menu
            ;;
    esac
}
```

---

## Phase 4: Version-Controlled Update System (NEW)

**Priority:** HIGH  
**Estimated Time:** 3-4 hours

**Enhanced Update Menu:**
```bash
show_update_menu() {
    local options="󰚰  Quick Update (pacman)
󰚰  Full Update (yay)
󰊢  Sync from GitHub
󰑐  Refresh Mirrors
󰌼  Check for VulcanOS Updates
󰑓  Restore Defaults
$ICON_BACK  Back"
    
    local choice
    choice=$(wofi_menu "Update" "$options")
    
    local action
    action=$(echo "$choice" | sed 's/^[^ ]* *//')
    
    case "$action" in
        "Quick Update (pacman)")
            run_in_terminal_hold sudo pacman -Syu
            ;;
        "Full Update (yay)")
            run_in_terminal_hold yay -Syu
            ;;
        "Sync from GitHub")
            show_github_sync_menu
            ;;
        "Refresh Mirrors")
            if command -v reflector &> /dev/null; then
                notify "Refreshing mirrors... This may take a moment."
                run_in_terminal_hold sudo reflector --latest 20 --protocol https --sort rate --save /etc/pacman.d/mirrorlist
            else
                notify "reflector not installed. Install with: sudo pacman -S reflector"
            fi
            ;;
        "Check for VulcanOS Updates")
            check_github_updates
            ;;
        "Restore Defaults")
            show_restore_menu
            ;;
        "Back")
            show_main_menu
            ;;
        *)
            exit 0
            ;;
    esac
}
```

**GitHub Sync Menu:**
```bash
show_github_sync_menu() {
    local options="󰇚  Pull Latest Changes
󰒞  Show Current Version
󰘬  View Changelog
󰊢  Check for Updates
$ICON_BACK  Back"
    
    local choice
    choice=$(wofi_menu "GitHub Sync" "$options" 350 280)
    
    local action
    action=$(echo "$choice" | sed 's/^[^ ]* *//')
    
    case "$action" in
        "Pull Latest Changes")
            sync_from_github
            ;;
        "Show Current Version")
            show_current_version
            ;;
        "View Changelog")
            view_changelog
            ;;
        "Check for Updates")
            check_github_updates
            ;;
        "Back")
            show_update_menu
            ;;
    esac
}

sync_from_github() {
    local repo_path="/home/evan/VulcanOS"
    
    if [[ ! -d "${repo_path}/.git" ]]; then
        notify "Not a git repository: ${repo_path}"
        return 1
    fi
    
    run_in_terminal_hold bash -c "
        cd '${repo_path}' &&
        echo '=== Checking current version ===' &&
        cat VERSION &&
        echo '' &&
        echo '=== Fetching from GitHub ===' &&
        git fetch origin &&
        echo '' &&
        echo '=== Current branch status ===' &&
        git status &&
        echo '' &&
        echo '=== Pulling changes ===' &&
        git pull origin main &&
        echo '' &&
        echo '=== Restowing dotfiles ===' &&
        cd dotfiles &&
        for dir in */; do
            echo \"Restowing \${dir%/}...\"
            stow --restow \"\${dir%/}\" 2>/dev/null || true
        done &&
        echo '' &&
        echo '=== Done! Press Enter to continue ===' &&
        read
    "
    
    notify "VulcanOS synced from GitHub"
}

show_current_version() {
    local version_file="/home/evan/VulcanOS/VERSION"
    
    if [[ -f "$version_file" ]]; then
        run_in_terminal_hold bash -c "
            source '$version_file' &&
            echo '=== VulcanOS Version ===' &&
            echo '' &&
            echo \"Version: \${MAJOR}.\${MINOR}.\${PATCH}\" &&
            echo \"Codename: \${CODENAME}\" &&
            echo '' &&
            echo '=== Git Status ===' &&
            cd /home/evan/VulcanOS &&
            git log -1 --pretty=format:'Commit: %h%nDate: %ad%nAuthor: %an%nMessage: %s' --date=short &&
            echo '' &&
            echo '' &&
            echo 'Press Enter to continue' &&
            read
        "
    else
        notify "VERSION file not found"
    fi
}

view_changelog() {
    local repo_path="/home/evan/VulcanOS"
    
    if [[ -d "${repo_path}/.git" ]]; then
        run_in_terminal_hold bash -c "
            cd '${repo_path}' &&
            echo '=== VulcanOS Changelog (Last 20 commits) ===' &&
            echo '' &&
            git log --oneline --decorate --graph -20 &&
            echo '' &&
            echo 'Press Enter to continue' &&
            read
        "
    else
        notify "Not a git repository"
    fi
}

check_github_updates() {
    run_in_terminal_hold bash -c "
        cd /home/evan/VulcanOS &&
        echo '=== Checking for updates ===' &&
        git fetch origin &&
        LOCAL=\$(git rev-parse @) &&
        REMOTE=\$(git rev-parse @{u}) &&
        BASE=\$(git merge-base @ @{u}) &&
        echo '' &&
        if [ \"\$LOCAL\" = \"\$REMOTE\" ]; then
            echo '✓ Your VulcanOS is up to date!'
        elif [ \"\$LOCAL\" = \"\$BASE\" ]; then
            echo '⚠ Updates available!' &&
            echo '' &&
            echo '=== Changes ===' &&
            git log HEAD..@{u} --oneline --graph
        else
            echo '⚠ Your local version has diverged from GitHub.'
            echo 'You may have local changes.'
        fi &&
        echo '' &&
        echo 'Press Enter to continue' &&
        read
    "
}
```

---

## Phase 5: Modular App Install Menu (NEW)

**Priority:** MEDIUM  
**Estimated Time:** 4-5 hours

**Enhanced Install Menu:**
```bash
show_install_menu() {
    local options="󰏗  Essential Tools
󰨞  Editors & IDEs
󰖟  Web Browsers
󰡨  Development Tools
󰺾  Design & Creative
󰗃  Communication
󰐴  Productivity
󰔬  Media & Entertainment
󰣇  Search All Packages
$ICON_BACK  Back"
    
    local choice
    choice=$(wofi_menu "Install" "$options")
    
    local action
    action=$(echo "$choice" | sed 's/^[^ ]* *//')
    
    case "$action" in
        "Essential Tools")          show_essential_tools_menu ;;
        "Editors & IDEs")           show_editors_menu ;;
        "Web Browsers")             show_browser_install_menu ;;
        "Development Tools")        show_devtools_install_menu ;;
        "Design & Creative")        show_design_menu ;;
        "Communication")            show_communication_menu ;;
        "Productivity")             show_productivity_menu ;;
        "Media & Entertainment")    show_media_menu ;;
        "Search All Packages")      show_package_search ;;
        "Back")                     show_main_menu ;;
        *)                          exit 0 ;;
    esac
}

show_essential_tools_menu() {
    # Get installed status
    local git_status=$(pacman -Q git &>/dev/null && echo "[✓]" || echo "[ ]")
    local gh_status=$(pacman -Q github-cli &>/dev/null && echo "[✓]" || echo "[ ]")
    local nvim_status=$(pacman -Q neovim &>/dev/null && echo "[✓]" || echo "[ ]")
    local docker_status=$(pacman -Q docker &>/dev/null && echo "[✓]" || echo "[ ]")
    local tmux_status=$(pacman -Q tmux &>/dev/null && echo "[✓]" || echo "[ ]")
    local ranger_status=$(pacman -Q ranger &>/dev/null && echo "[✓]" || echo "[ ]")
    
    local options="${git_status} Git Version Control
${gh_status} GitHub CLI (gh)
${nvim_status} Neovim
${docker_status} Docker + Docker Compose
${tmux_status} tmux Terminal Multiplexer
${ranger_status} Ranger CLI File Manager
󰏗  Install Selected
$ICON_BACK  Back"
    
    local choice
    choice=$(wofi_menu "Essential Tools" "$options" 400 420)
    
    # Parse and install
    case "$choice" in
        *"Git Version Control"*)
            run_in_terminal_hold sudo pacman -S git
            ;;
        *"GitHub CLI"*)
            run_in_terminal_hold sudo pacman -S github-cli
            ;;
        *"Neovim"*)
            run_in_terminal_hold sudo pacman -S neovim
            ;;
        *"Docker"*)
            run_in_terminal_hold bash -c "sudo pacman -S docker docker-compose && sudo systemctl enable --now docker && sudo usermod -aG docker \$USER"
            ;;
        *"tmux"*)
            run_in_terminal_hold sudo pacman -S tmux
            ;;
        *"Ranger"*)
            run_in_terminal_hold sudo pacman -S ranger
            ;;
        "Back")
            show_install_menu
            ;;
    esac
}

show_editors_menu() {
    local code_status=$(pacman -Q code &>/dev/null && echo "[✓]" || echo "[ ]")
    local cursor_status=$(pacman -Q cursor-bin &>/dev/null && echo "[✓]" || echo "[ ]")
    local zed_status=$(pacman -Q zed &>/dev/null && echo "[✓]" || echo "[ ]")
    local sublime_status=$(pacman -Q sublime-text-4 &>/dev/null && echo "[✓]" || echo "[ ]")
    
    local options="${code_status} VS Code
${cursor_status} Cursor AI Editor
${zed_status} Zed Editor
${sublime_status} Sublime Text
$ICON_BACK  Back"
    
    local choice
    choice=$(wofi_menu "Editors & IDEs" "$options" 350 300)
    
    case "$choice" in
        *"VS Code"*)       run_in_terminal_hold yay -S visual-studio-code-bin ;;
        *"Cursor"*)        run_in_terminal_hold yay -S cursor-bin ;;
        *"Zed"*)           run_in_terminal_hold yay -S zed ;;
        *"Sublime Text"*)  run_in_terminal_hold yay -S sublime-text-4 ;;
        "Back")            show_install_menu ;;
    esac
}

show_design_menu() {
    local gimp_status=$(pacman -Q gimp &>/dev/null && echo "[✓]" || echo "[ ]")
    local inkscape_status=$(pacman -Q inkscape &>/dev/null && echo "[✓]" || echo "[ ]")
    local blender_status=$(pacman -Q blender &>/dev/null && echo "[✓]" || echo "[ ]")
    local krita_status=$(pacman -Q krita &>/dev/null && echo "[✓]" || echo "[ ]")
    
    local options="${gimp_status} GIMP (Photo Editing)
${inkscape_status} Inkscape (Vector Graphics)
${blender_status} Blender (3D Modeling)
${krita_status} Krita (Digital Painting)
$ICON_BACK  Back"
    
    local choice
    choice=$(wofi_menu "Design & Creative" "$options" 400 300)
    
    case "$choice" in
        *"GIMP"*)      run_in_terminal_hold sudo pacman -S gimp ;;
        *"Inkscape"*)  run_in_terminal_hold sudo pacman -S inkscape ;;
        *"Blender"*)   run_in_terminal_hold sudo pacman -S blender ;;
        *"Krita"*)     run_in_terminal_hold sudo pacman -S krita ;;
        "Back")        show_install_menu ;;
    esac
}

show_communication_menu() {
    local discord_status=$(pacman -Q discord &>/dev/null && echo "[✓]" || echo "[ ]")
    local slack_status=$(pacman -Q slack-desktop &>/dev/null && echo "[✓]" || echo "[ ]")
    local telegram_status=$(pacman -Q telegram-desktop &>/dev/null && echo "[✓]" || echo "[ ]")
    local signal_status=$(pacman -Q signal-desktop &>/dev/null && echo "[✓]" || echo "[ ]")
    
    local options="${discord_status} Discord
${slack_status} Slack
${telegram_status} Telegram
${signal_status} Signal
$ICON_BACK  Back"
    
    local choice
    choice=$(wofi_menu "Communication" "$options" 350 300)
    
    case "$choice" in
        *"Discord"*)   run_in_terminal_hold sudo pacman -S discord ;;
        *"Slack"*)     run_in_terminal_hold yay -S slack-desktop ;;
        *"Telegram"*)  run_in_terminal_hold sudo pacman -S telegram-desktop ;;
        *"Signal"*)    run_in_terminal_hold yay -S signal-desktop ;;
        "Back")        show_install_menu ;;
    esac
}

show_productivity_menu() {
    local obsidian_status=$(pacman -Q obsidian &>/dev/null && echo "[✓]" || echo "[ ]")
    local libreoffice_status=$(pacman -Q libreoffice-fresh &>/dev/null && echo "[✓]" || echo "[ ]")
    
    local options="${obsidian_status} Obsidian (Note Taking)
${libreoffice_status} LibreOffice Suite
$ICON_BACK  Back"
    
    local choice
    choice=$(wofi_menu "Productivity" "$options" 350 240)
    
    case "$choice" in
        *"Obsidian"*)     run_in_terminal_hold yay -S obsidian ;;
        *"LibreOffice"*)  run_in_terminal_hold sudo pacman -S libreoffice-fresh ;;
        "Back")           show_install_menu ;;
    esac
}

show_media_menu() {
    local vlc_status=$(pacman -Q vlc &>/dev/null && echo "[✓]" || echo "[ ]")
    local spotify_status=$(pacman -Q spotify &>/dev/null && echo "[✓]" || echo "[ ]")
    local obs_status=$(pacman -Q obs-studio &>/dev/null && echo "[✓]" || echo "[ ]")
    
    local options="${vlc_status} VLC Media Player
${spotify_status} Spotify
${obs_status} OBS Studio
$ICON_BACK  Back"
    
    local choice
    choice=$(wofi_menu "Media & Entertainment" "$options" 350 260)
    
    case "$choice" in
        *"VLC"*)      run_in_terminal_hold sudo pacman -S vlc ;;
        *"Spotify"*)  run_in_terminal_hold yay -S spotify ;;
        *"OBS"*)      run_in_terminal_hold sudo pacman -S obs-studio ;;
        "Back")       show_install_menu ;;
    esac
}

show_package_search() {
    local search_options="󰏗  Search Official Repos (pacman)
󰣇  Search AUR (yay)
$ICON_BACK  Back"
    
    local choice
    choice=$(wofi_menu "Package Search" "$search_options" 350 200)
    
    case "$choice" in
        *"Official Repos"*)
            local pkg
            pkg=$(pacman -Ssq 2>/dev/null | wofi --dmenu --prompt "Install Package" --cache-file /dev/null 2>/dev/null)
            if [[ -n "$pkg" ]]; then
                run_in_terminal_hold sudo pacman -S "$pkg"
            fi
            ;;
        *"AUR"*)
            local pkg
            pkg=$(yay -Ssq 2>/dev/null | wofi --dmenu --prompt "Install AUR Package" --cache-file /dev/null 2>/dev/null)
            if [[ -n "$pkg" ]]; then
                run_in_terminal_hold yay -S "$pkg"
            fi
            ;;
        "Back")
            show_install_menu
            ;;
    esac
}
```

---

## Additional Enhancements (Future Phases)

### 6. Backup & Sync Menu
```bash
show_backup_menu() {
    local options="󰋑  Backup Configs to GitHub
󰘓  Restore from Backup
󰋋  View Backup History
󰱓  Schedule Auto-Backup
$ICON_BACK  Back"
}
```

### 7. Hardware Optimization Menu
```bash
show_optimization_menu() {
    local options="󰓅  Power Profile (Performance/Balanced/Powersave)
󰐾  Thermal Management
󰂃  Battery Health
󰖩  WiFi Optimization
󰋐  Audio Tweaks
$ICON_BACK  Back"
}
```

### 8. Quick Actions Menu
```bash
show_quick_actions() {
    local options="󰌾  Lock Screen
󰤄  Sleep
󰜨  Restart Waybar
󰑓  Reload Hyprland
󰃨  Clear Cache
󰗑  Emergency Restart"
}
```

### 9. Community Resources Menu
```bash
show_resources_menu() {
    local options="󰗧  VulcanOS Documentation
󰊢  GitHub Repository
󰟀  Report Bug
󰳢  Feature Request
󰌠  Join Discord/Matrix
$ICON_BACK  Back"
}
```

---

## Implementation Timeline

| Phase | Task | Priority | Estimated Time |
|-------|------|----------|----------------|
| **1** | Waybar + Menu Bug Fixes | **CRITICAL** | 2-3 hours |
| **2** | Theme Expansion (15 themes) | **HIGH** | 4-6 hours |
| **2.2** | Wallpaper Library (50+ wallpapers) | **HIGH** | 3-4 hours |
| **3** | Config Access Menu | **MEDIUM** | 2 hours |
| **4** | GitHub Sync System | **HIGH** | 3-4 hours |
| **5** | Modular Install Menu | **MEDIUM** | 4-5 hours |
| **6-9** | Additional Enhancements | **LOW** | 6-8 hours |

**Total Estimated Time: 24-32 hours**

---

## File Changes Summary

| File | Changes | Lines Added |
|------|---------|-------------|
| `dotfiles/waybar/.config/waybar/config.jsonc` | Add vulcan-menu, remove orchestra | ~15 |
| `dotfiles/waybar/.config/waybar/style.css` | Add vulcan-menu styling | ~20 |
| `dotfiles/scripts/.local/bin/vulcan-menu` | Bug fixes + 5 new menus | ~600 |
| `dotfiles/themes/colors/*.sh` | 7 new theme files | ~350 |
| `Pictures/Wallpapers/` | 50+ wallpapers organized | N/A (downloads) |
| `archiso/airootfs/etc/skel/.config/waybar/` | Sync changes | ~35 |
| `archiso/airootfs/usr/local/bin/vulcan-menu` | Sync changes | ~600 |
| **TOTAL** | | **~1,620 lines + wallpapers** |

---

## Testing Checklist

**Phase 1: Waybar + Bugs**
- [ ] Vulcan menu button appears at top left
- [ ] Button launches vulcan-menu on click
- [ ] Orchestra module removed
- [ ] Display Settings submenu works (no terminal output)
- [ ] Default Applications sets defaults via xdg-mime
- [ ] Restore Defaults creates backups and restores from both sources
- [ ] Waybar logs clean: `journalctl --user -u waybar`

**Phase 2: Themes**
- [ ] All 15 themes apply successfully
- [ ] Theme switcher menu shows all themes
- [ ] Wallpaper categories browsable
- [ ] Random wallpaper works
- [ ] Theme-matched wallpapers set correctly

**Phase 3: Configs Menu**
- [ ] All config files open in editor
- [ ] Hyprland submenu navigates properly
- [ ] Reload function works without errors

**Phase 4: GitHub Sync**
- [ ] Pull updates works
- [ ] Version display shows correct info
- [ ] Changelog displays recent commits
- [ ] Update checker compares local vs remote

**Phase 5: Install Menu**
- [ ] Package status shows correctly ([✓] vs [ ])
- [ ] Installations complete successfully
- [ ] All categories accessible
- [ ] Package search works for pacman and AUR

**Integration:**
- [ ] All keybindings still work
- [ ] Configs persist after reboot
- [ ] Stow symlinks intact
- [ ] Changes synced to archiso

---

## Pre-Implementation Questions

1. **Theme Sources**: Should I research and create all 7 new themes, or would you like to pick specific ones?
   
2. **Wallpaper Sources**: Should I:
   - Curate from Unsplash/r/wallpapers (requires manual selection)
   - Use existing theme wallpaper repos (automated)
   - Mix of both?

3. **GitHub Sync**: Should this:
   - Auto-commit user changes?
   - Only pull (read-only)?
   - Require manual commit/push?

4. **Install Menu**: Should package installation be:
   - Individual (one at a time)
   - Batch multi-select (checkboxes)
   - Both options?

5. **Version Control**: Should VulcanOS VERSION file:
   - Auto-increment on sync?
   - Manual update only?
   - Semantic versioning (0.1.0 → 0.2.0)?

6. **Priority**: Which phases should I implement first? (Recommended: 1 → 4 → 2 → 5 → 3)

---

## Success Criteria

**VulcanOS 0.2.0 "Forge" will be considered complete when:**

1. ✓ Waybar has visual Vulcan Menu launcher
2. ✓ All menu bugs fixed (Display Settings, Default Apps, Restore)
3. ✓ 15+ themes available with matching wallpapers
4. ✓ 50+ curated wallpapers organized by category
5. ✓ Centralized config access menu
6. ✓ GitHub sync system functional
7. ✓ Modular app installation system with status indicators
8. ✓ All changes synced to archiso for ISO builds
9. ✓ Documentation updated
10. ✓ No regressions in existing functionality

---

## Next Steps

1. Review this plan
2. Answer pre-implementation questions
3. Create orchestrator session to implement
4. Test thoroughly on T2 hardware
5. Sync to archiso
6. Build and test ISO
7. Release VulcanOS 0.2.0 "Forge"
