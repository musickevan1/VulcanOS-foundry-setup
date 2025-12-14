#!/bin/bash
# VulcanOS - Vulcan Forge Theme
# A warm, forge-inspired color palette evoking molten metal and craftsman aesthetics
#
# Primary: Forge Orange (#f97316) - molten metal glow
# Secondary: Ember Gold (#fbbf24) - warm highlights
# Base: Forge Black (#1c1917) - warm charcoal undertones

# Theme metadata
export THEME_NAME="Vulcan Forge"
export THEME_ID="vulcan-forge"

# Background colors (warm charcoal scale)
export BG_PRIMARY="#1c1917"     # Forge Black - primary background
export BG_SECONDARY="#292524"   # Charcoal - elevated surfaces
export BG_TERTIARY="#44403c"    # Stone Gray - tertiary/borders
export BG_SURFACE="#57534e"     # Ash - subtle borders/dividers

# Foreground/text colors
export FG_PRIMARY="#fafaf9"     # Warm white - main text
export FG_SECONDARY="#a8a29e"   # Stone 400 - secondary text
export FG_MUTED="#78716c"       # Stone 500 - muted/disabled

# Accent colors
export ACCENT="#f97316"         # Forge Orange - primary accent
export ACCENT_ALT="#fbbf24"     # Ember Gold - secondary accent

# ANSI color palette
export RED="#ef4444"            # Flame Red
export GREEN="#22c55e"          # Success Green
export YELLOW="#fbbf24"         # Ember Gold
export BLUE="#3b82f6"           # Info Blue
export PURPLE="#a855f7"         # Violet accent
export CYAN="#06b6d4"           # Cyan accent
export ORANGE="#f97316"         # Forge Orange
export PINK="#ec4899"           # Pink accent

# Bright ANSI colors
export BRIGHT_RED="#f87171"     # Bright flame
export BRIGHT_GREEN="#4ade80"   # Bright success
export BRIGHT_YELLOW="#fcd34d"  # Bright gold
export BRIGHT_BLUE="#60a5fa"    # Bright info
export BRIGHT_PURPLE="#c084fc"  # Bright violet
export BRIGHT_CYAN="#22d3ee"    # Bright cyan

# Window manager colors
export BORDER_ACTIVE="${ACCENT}"        # Active window border
export BORDER_INACTIVE="#44403c"        # Inactive window border
export SELECTION="#44403c"              # Selection background
export CURSOR="#f97316"                 # Cursor color

# Gradient colors (for Hyprland borders, etc.)
export GRADIENT_START="#f97316"         # Forge Orange
export GRADIENT_END="#fbbf24"           # Ember Gold

# GTK/Qt theming
export GTK_THEME="Adwaita-dark"         # GTK theme
export ICON_THEME="Papirus-Dark"        # Icon theme
export CURSOR_THEME="Adwaita"           # Cursor theme
export KVANTUM_THEME="KvArcDark"        # Kvantum theme

# Editor colorscheme (closest match)
export NVIM_COLORSCHEME="tokyonight-night"  # Neovim colorscheme

# Optional wallpaper (uncomment when available)
# export THEME_WALLPAPER="vulcan-forge-dark.png"
export THEME_WALLPAPER="vulcan-gradient.png"
