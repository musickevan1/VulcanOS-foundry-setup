# =============================================================================
# VulcanOS Kitty Terminal Configuration
# Theme: ${THEME_NAME}
# https://sw.kovidgoyal.net/kitty/conf/
# =============================================================================

# -----------------------------------------------------------------------------
# REMOTE CONTROL (for zoom indicator feature)
# -----------------------------------------------------------------------------
allow_remote_control yes

# -----------------------------------------------------------------------------
# FONT CONFIGURATION
# -----------------------------------------------------------------------------
font_family      JetBrainsMono Nerd Font
bold_font        JetBrainsMono Nerd Font Bold
italic_font      JetBrainsMono Nerd Font Italic
bold_italic_font JetBrainsMono Nerd Font Bold Italic
font_size        11.0

# Use VulcanOS patched font for custom glyph (U+F0001)
symbol_map U+F0001 JetBrainsMono NF Vulcan

# Disable ligatures for cleaner code reading (optional - remove to enable)
disable_ligatures never

# -----------------------------------------------------------------------------
# CURSOR
# -----------------------------------------------------------------------------
cursor_shape          block
cursor_blink_interval 0.5
cursor_stop_blinking_after 15.0

# -----------------------------------------------------------------------------
# SCROLLBACK
# -----------------------------------------------------------------------------
scrollback_lines 10000
scrollback_pager less --chop-long-lines --RAW-CONTROL-CHARS +INPUT_LINE_NUMBER
wheel_scroll_multiplier 3.0

# -----------------------------------------------------------------------------
# MOUSE
# -----------------------------------------------------------------------------
mouse_hide_wait 3.0
url_style curly
open_url_with default
url_prefixes file ftp ftps gemini git gopher http https irc ircs kitty mailto news sftp ssh
detect_urls yes
copy_on_select clipboard
strip_trailing_spaces smart

# -----------------------------------------------------------------------------
# PERFORMANCE
# -----------------------------------------------------------------------------
repaint_delay    10
input_delay      3
sync_to_monitor  yes

# -----------------------------------------------------------------------------
# BELL
# -----------------------------------------------------------------------------
enable_audio_bell    no
visual_bell_duration 0.0
window_alert_on_bell yes
bell_on_tab          " "

# -----------------------------------------------------------------------------
# WINDOW
# -----------------------------------------------------------------------------
remember_window_size  yes
initial_window_width  120c
initial_window_height 35c
window_padding_width  8
placement_strategy    center
hide_window_decorations no
confirm_os_window_close 0

# Background opacity (0.0 to 1.0)
background_opacity 0.95
dynamic_background_opacity yes

# -----------------------------------------------------------------------------
# TAB BAR
# -----------------------------------------------------------------------------
tab_bar_edge            top
tab_bar_style           powerline
tab_powerline_style     slanted
tab_bar_align           left
tab_bar_min_tabs        1
tab_switch_strategy     previous
tab_activity_symbol     " "
tab_title_template      " {index}: {title} "
active_tab_title_template " {index}: {title} "

# Tab bar margins
tab_bar_margin_width    4.0
tab_bar_margin_height   4.0 0.0

# Tab fonts
active_tab_font_style   bold
inactive_tab_font_style normal

# -----------------------------------------------------------------------------
# SHELL INTEGRATION
# -----------------------------------------------------------------------------
shell_integration enabled
allow_hyperlinks yes

# -----------------------------------------------------------------------------
# KEYBINDINGS
# -----------------------------------------------------------------------------

# Clipboard
map ctrl+shift+c copy_to_clipboard
map ctrl+shift+v paste_from_clipboard
map ctrl+shift+s paste_from_selection
map shift+insert paste_from_selection

# Scrolling
map ctrl+shift+up        scroll_line_up
map ctrl+shift+down      scroll_line_down
map ctrl+shift+page_up   scroll_page_up
map ctrl+shift+page_down scroll_page_down
map ctrl+shift+home      scroll_home
map ctrl+shift+end       scroll_end
map ctrl+shift+h         show_scrollback

# Window management (splits)
map ctrl+shift+enter new_window
map ctrl+shift+n     new_os_window
map ctrl+shift+w     close_window
map ctrl+shift+]     next_window
map ctrl+shift+[     previous_window
map ctrl+shift+f     move_window_forward
map ctrl+shift+b     move_window_backward
map ctrl+shift+`     move_window_to_top
map ctrl+shift+r     start_resizing_window

# Tab management
map ctrl+shift+t     new_tab
map ctrl+shift+q     close_tab
map ctrl+shift+right next_tab
map ctrl+shift+left  previous_tab
map ctrl+shift+.     move_tab_forward
map ctrl+shift+,     move_tab_backward
map ctrl+shift+alt+t set_tab_title

# Direct tab switching (Ctrl+1-9)
map ctrl+1 goto_tab 1
map ctrl+2 goto_tab 2
map ctrl+3 goto_tab 3
map ctrl+4 goto_tab 4
map ctrl+5 goto_tab 5
map ctrl+6 goto_tab 6
map ctrl+7 goto_tab 7
map ctrl+8 goto_tab 8
map ctrl+9 goto_tab 9

# Layout cycling
map ctrl+shift+l next_layout

# Font size
map ctrl+shift+equal     change_font_size all +1.0
map ctrl+shift+plus      change_font_size all +1.0
map ctrl+shift+minus     change_font_size all -1.0
map ctrl+shift+backspace change_font_size all 0

# Background opacity
map ctrl+shift+a>m set_background_opacity +0.05
map ctrl+shift+a>l set_background_opacity -0.05
map ctrl+shift+a>1 set_background_opacity 1
map ctrl+shift+a>d set_background_opacity default

# Misc
map ctrl+shift+f5 load_config_file
map ctrl+shift+f6 debug_config
map ctrl+shift+u  kitten unicode_input
map ctrl+shift+e  open_url_with_hints
map ctrl+shift+g  show_last_command_output

# -----------------------------------------------------------------------------
# COLORS - ${THEME_NAME}
# Optimized for readability with good contrast ratios
# -----------------------------------------------------------------------------

# Basic colors
foreground              ${FG_PRIMARY}
background              ${BG_PRIMARY}
selection_foreground    ${BG_PRIMARY}
selection_background    ${ACCENT}

# Cursor colors
cursor                  ${ACCENT}
cursor_text_color       ${BG_PRIMARY}

# URL underline color
url_color               ${BRIGHT_BLUE}

# Window borders
active_border_color     ${ACCENT}
inactive_border_color   ${BG_TERTIARY}
bell_border_color       ${RED}

# Tab bar colors
active_tab_foreground   ${BG_PRIMARY}
active_tab_background   ${ACCENT}
inactive_tab_foreground ${FG_SECONDARY}
inactive_tab_background ${BG_SECONDARY}
tab_bar_background      ${BG_PRIMARY}

# Color palette - optimized for readability
# Black (dim visible against dark bg, bright for comments)
color0  ${BG_SECONDARY}
color8  ${FG_MUTED}

# Red
color1  ${RED}
color9  ${BRIGHT_RED}

# Green
color2  ${GREEN}
color10 ${BRIGHT_GREEN}

# Yellow
color3  ${YELLOW}
color11 ${BRIGHT_YELLOW}

# Blue
color4  ${BLUE}
color12 ${BRIGHT_BLUE}

# Magenta/Purple
color5  ${PURPLE}
color13 ${BRIGHT_PURPLE}

# Cyan
color6  ${CYAN}
color14 ${BRIGHT_CYAN}

# White (dim for secondary text, bright for emphasis)
color7  ${FG_SECONDARY}
color15 ${FG_PRIMARY}

# Extended colors for marks
mark1_foreground ${BG_PRIMARY}
mark1_background ${ACCENT}
mark2_foreground ${BG_PRIMARY}
mark2_background ${ACCENT_ALT}
mark3_foreground ${BG_PRIMARY}
mark3_background ${GREEN}
