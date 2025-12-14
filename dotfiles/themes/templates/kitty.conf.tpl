# VulcanOS Kitty Configuration
# Theme: ${THEME_NAME}

# Font
font_family      JetBrainsMono Nerd Font
bold_font        JetBrainsMono Nerd Font Bold
italic_font      JetBrainsMono Nerd Font Italic
bold_italic_font JetBrainsMono Nerd Font Bold Italic
font_size        12.0

# Window
window_padding_width 10
background_opacity   0.95
confirm_os_window_close 0

# Cursor
cursor_shape block
cursor_blink_interval 0.75

# Scrollback
scrollback_lines 10000

# Mouse
mouse_hide_wait 3.0
copy_on_select  clipboard

# Bell
enable_audio_bell no
visual_bell_duration 0.0

# Tab bar
tab_bar_style powerline
tab_powerline_style slanted

# ${THEME_NAME} color scheme
foreground ${FG_PRIMARY}
background ${BG_PRIMARY}
selection_foreground ${FG_PRIMARY}
selection_background ${SELECTION}

# Cursor colors
cursor ${CURSOR}
cursor_text_color ${BG_PRIMARY}

# Normal colors
color0  ${BG_SURFACE}
color1  ${RED}
color2  ${GREEN}
color3  ${YELLOW}
color4  ${BLUE}
color5  ${PURPLE}
color6  ${CYAN}
color7  ${FG_SECONDARY}

# Bright colors
color8  ${BG_TERTIARY}
color9  ${BRIGHT_RED}
color10 ${BRIGHT_GREEN}
color11 ${BRIGHT_YELLOW}
color12 ${BRIGHT_BLUE}
color13 ${BRIGHT_PURPLE}
color14 ${BRIGHT_CYAN}
color15 ${FG_PRIMARY}

# URL styling
url_color ${BLUE}
url_style curly

# Keyboard shortcuts
map ctrl+shift+c copy_to_clipboard
map ctrl+shift+v paste_from_clipboard
map ctrl+shift+equal change_font_size all +1.0
map ctrl+shift+minus change_font_size all -1.0
map ctrl+shift+backspace change_font_size all 0
map ctrl+shift+t new_tab
map ctrl+shift+q close_tab
map ctrl+shift+right next_tab
map ctrl+shift+left previous_tab
