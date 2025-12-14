# Starship Prompt Configuration
# Theme: ${THEME_NAME}
# https://starship.rs

format = """
[](${BLUE})\
$os\
$username\
[](bg:${GREEN} fg:${BLUE})\
$directory\
[](fg:${GREEN} bg:${PURPLE})\
$git_branch\
$git_status\
[](fg:${PURPLE} bg:${YELLOW})\
$c\
$rust\
$golang\
$nodejs\
$python\
[](fg:${YELLOW} bg:${CYAN})\
$docker_context\
[](fg:${CYAN} bg:${BG_PRIMARY})\
$time\
[ ](fg:${BG_PRIMARY})\
"""

[os]
disabled = false
style = "bg:${BLUE} fg:${BG_PRIMARY}"

[os.symbols]
Arch = "󰣇"

[username]
show_always = true
style_user = "bg:${BLUE} fg:${BG_PRIMARY}"
style_root = "bg:${BLUE} fg:${BG_PRIMARY}"
format = '[$user ]($style)'
disabled = false

[directory]
style = "bg:${GREEN} fg:${BG_PRIMARY}"
format = "[ $path ]($style)"
truncation_length = 3
truncation_symbol = "…/"

[git_branch]
symbol = ""
style = "bg:${PURPLE} fg:${BG_PRIMARY}"
format = '[ $symbol $branch ]($style)'

[git_status]
style = "bg:${PURPLE} fg:${BG_PRIMARY}"
format = '[$all_status$ahead_behind ]($style)'

[nodejs]
symbol = ""
style = "bg:${YELLOW} fg:${BG_PRIMARY}"
format = '[ $symbol ($version) ]($style)'

[rust]
symbol = ""
style = "bg:${YELLOW} fg:${BG_PRIMARY}"
format = '[ $symbol ($version) ]($style)'

[golang]
symbol = ""
style = "bg:${YELLOW} fg:${BG_PRIMARY}"
format = '[ $symbol ($version) ]($style)'

[python]
symbol = ""
style = "bg:${YELLOW} fg:${BG_PRIMARY}"
format = '[ $symbol ($version) ]($style)'

[docker_context]
symbol = ""
style = "bg:${CYAN} fg:${BG_PRIMARY}"
format = '[ $symbol $context ]($style)'

[time]
disabled = false
time_format = "%R"
style = "bg:${BG_PRIMARY} fg:${FG_PRIMARY}"
format = '[ $time ]($style)'

[character]
success_symbol = "[❯](bold green)"
error_symbol = "[❯](bold red)"
