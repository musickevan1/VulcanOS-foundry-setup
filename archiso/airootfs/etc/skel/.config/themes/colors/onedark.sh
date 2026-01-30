#!/bin/bash
# One Dark Theme - VulcanOS
# Official palette: https://github.com/atom/one-dark-syntax

export THEME_NAME="One Dark"
export THEME_ID="onedark"
export THEME_DESCRIPTION="Atom's iconic dark theme"

# One Dark official colors
export OD_BG="#282c34"          # bg - main background
export OD_GUTTER="#636d83"      # gutter - line numbers
export OD_ACCENT="#528bff"      # accent
export OD_FG="#abb2bf"          # fg - foreground
export OD_RED="#e06c75"         # red
export OD_ORANGE="#d19a66"      # orange
export OD_YELLOW="#e5c07b"      # yellow
export OD_GREEN="#98c379"       # green
export OD_CYAN="#56b6c2"        # cyan
export OD_BLUE="#61afef"        # blue
export OD_PURPLE="#c678dd"      # purple
export OD_WHITE="#abb2bf"       # white

# Derived background shades
export OD_BG_DARKER="#21252b"   # Darker background
export OD_BG_LIGHTEST="#3e4451" # Lightest background elements

# VulcanOS standard mappings
export BG_PRIMARY="#282c34"     # bg
export BG_SECONDARY="#21252b"   # Derived darker
export BG_TERTIARY="#3e4451"    # Derived lightest
export BG_SURFACE="#1e2227"     # Derived even darker

export FG_PRIMARY="#abb2bf"     # fg
export FG_SECONDARY="#abb2bf"   # fg
export FG_MUTED="#5c6370"       # Derived muted

export ACCENT="#61afef"         # blue
export ACCENT_ALT="#c678dd"     # purple

# Semantic colors
export RED="#e06c75"            # red
export GREEN="#98c379"          # green
export YELLOW="#e5c07b"         # yellow
export BLUE="#61afef"           # blue
export PURPLE="#c678dd"         # purple
export CYAN="#56b6c2"           # cyan
export ORANGE="#d19a66"         # orange
export PINK="#e06c75"           # red (as pink)

# Bright variants
export BRIGHT_RED="#e06c75"
export BRIGHT_GREEN="#98c379"
export BRIGHT_YELLOW="#e5c07b"
export BRIGHT_BLUE="#61afef"
export BRIGHT_PURPLE="#c678dd"
export BRIGHT_CYAN="#56b6c2"

# UI specific
export BORDER_ACTIVE="#61afef"  # blue
export BORDER_INACTIVE="#3e4451" # bg_lightest
export SELECTION="#3e4451"      # bg_lightest
export CURSOR="#abb2bf"         # fg

# Gradient colors (for Hyprland)
export GRADIENT_START="#61afef" # blue
export GRADIENT_END="#c678dd"   # purple

# GTK/Qt themes
export GTK_THEME="Adwaita-dark"
export ICON_THEME="Papirus-Dark"
export CURSOR_THEME="Adwaita"
export KVANTUM_THEME="KvArcDark"

# Neovim colorscheme
export NVIM_COLORSCHEME="onedark"

# Wallpaper
export THEME_WALLPAPER="onedark.png"
