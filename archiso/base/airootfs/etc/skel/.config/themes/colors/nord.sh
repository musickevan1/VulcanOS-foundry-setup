#!/bin/bash
# Nord Theme - VulcanOS
# Official palette: https://www.nordtheme.com/docs/colors-and-palettes

export THEME_NAME="Nord"
export THEME_ID="nord"
export THEME_DESCRIPTION="Arctic, bluish color palette"

# Polar Night (backgrounds)
export NORD0="#2e3440"          # nord0 - darkest
export NORD1="#3b4252"          # nord1 - dark
export NORD2="#434c5e"          # nord2 - medium dark
export NORD3="#4c566a"          # nord3 - medium

# Snow Storm (foregrounds)
export NORD4="#d8dee9"          # nord4 - light
export NORD5="#e5e9f0"          # nord5 - lighter
export NORD6="#eceff4"          # nord6 - lightest

# Frost (blues)
export NORD7="#8fbcbb"          # nord7 - cyan
export NORD8="#88c0d0"          # nord8 - bright cyan
export NORD9="#81a1c1"          # nord9 - blue
export NORD10="#5e81ac"         # nord10 - dark blue

# Aurora (accents)
export NORD11="#bf616a"         # nord11 - red
export NORD12="#d08770"         # nord12 - orange
export NORD13="#ebcb8b"         # nord13 - yellow
export NORD14="#a3be8c"         # nord14 - green
export NORD15="#b48ead"         # nord15 - purple

# VulcanOS standard mappings
export BG_PRIMARY="#2e3440"     # nord0
export BG_SECONDARY="#3b4252"   # nord1
export BG_TERTIARY="#434c5e"    # nord2
export BG_SURFACE="#242933"     # Derived darker

export FG_PRIMARY="#eceff4"     # nord6
export FG_SECONDARY="#e5e9f0"   # nord5
export FG_MUTED="#4c566a"       # nord3

export ACCENT="#88c0d0"         # nord8
export ACCENT_ALT="#81a1c1"     # nord9

# Semantic colors (Aurora)
export RED="#bf616a"            # nord11
export GREEN="#a3be8c"          # nord14
export YELLOW="#ebcb8b"         # nord13
export BLUE="#81a1c1"           # nord9
export PURPLE="#b48ead"         # nord15
export CYAN="#88c0d0"           # nord8
export ORANGE="#d08770"         # nord12
export PINK="#b48ead"           # nord15

# Bright variants
export BRIGHT_RED="#bf616a"
export BRIGHT_GREEN="#a3be8c"
export BRIGHT_YELLOW="#ebcb8b"
export BRIGHT_BLUE="#81a1c1"
export BRIGHT_PURPLE="#b48ead"
export BRIGHT_CYAN="#8fbcbb"    # nord7

# UI specific
export BORDER_ACTIVE="#88c0d0"  # nord8
export BORDER_INACTIVE="#434c5e" # nord2
export SELECTION="#434c5e"      # nord2
export CURSOR="#eceff4"         # nord6

# Gradient colors (for Hyprland)
export GRADIENT_START="#88c0d0" # nord8
export GRADIENT_END="#b48ead"   # nord15

# GTK/Qt themes
export GTK_THEME="Nordic"
export ICON_THEME="Papirus-Dark"
export CURSOR_THEME="Adwaita"
export KVANTUM_THEME="Nordic"

# Neovim colorscheme
export NVIM_COLORSCHEME="nord"

# Wallpaper
export THEME_WALLPAPER="nord.png"
