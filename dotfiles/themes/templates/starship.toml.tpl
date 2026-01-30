# =============================================================================
# VulcanOS Starship Prompt - Minimal Powerline
# Theme: ${THEME_NAME}
# https://starship.rs/config/
# =============================================================================

# Two-line powerline: VulcanOS logo, user, directory, git branch
# Line 2: prompt character
format = """
[](fg:${ACCENT})\
[ 1 ](bg:${ACCENT} fg:${BG_PRIMARY} bold)\
$username\
[](bg:${ACCENT_ALT} fg:${ACCENT})\
$directory\
[](fg:${ACCENT_ALT} bg:${PURPLE})\
$git_branch\
[](fg:${PURPLE})
$character"""

# No right prompt
right_format = ""

# Add newline between prompts
add_newline = true

# Command timeout
command_timeout = 1000

# =============================================================================
# PROMPT COMPONENTS
# =============================================================================

[username]
show_always = true
style_user = "bg:${ACCENT} fg:${BG_PRIMARY} bold"
style_root = "bg:${RED} fg:${BG_PRIMARY} bold"
format = '[$user ]($style)'
disabled = false

[directory]
style = "bg:${ACCENT_ALT} fg:${BG_PRIMARY}"
format = '[ $path ]($style)'
truncation_length = 3
truncation_symbol = "…/"
truncate_to_repo = true
read_only = " 󰌾"
read_only_style = "bg:${ACCENT_ALT} fg:${RED}"
home_symbol = "~"

[git_branch]
symbol = ""
style = "bg:${PURPLE} fg:${BG_PRIMARY}"
format = '[ $symbol $branch ]($style)'
truncation_length = 24
truncation_symbol = "…"

# Character - accent on success, red on error
[character]
success_symbol = '[❯](bold ${ACCENT})'
error_symbol = '[❯](bold ${RED})'
vimcmd_symbol = '[❮](bold ${ACCENT_ALT})'
