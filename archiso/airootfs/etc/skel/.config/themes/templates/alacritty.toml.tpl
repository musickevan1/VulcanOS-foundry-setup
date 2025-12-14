[window]
opacity = 0.95
padding = { x = 10, y = 10 }
decorations = "Full"
startup_mode = "Windowed"

[font]
size = 12.0

[font.normal]
family = "JetBrainsMono Nerd Font"
style = "Regular"

[font.bold]
family = "JetBrainsMono Nerd Font"
style = "Bold"

[font.italic]
family = "JetBrainsMono Nerd Font"
style = "Italic"

[font.bold_italic]
family = "JetBrainsMono Nerd Font"
style = "Bold Italic"

# ${THEME_NAME} color scheme
[colors.primary]
background = "${BG_PRIMARY}"
foreground = "${FG_PRIMARY}"

[colors.cursor]
cursor = "${CURSOR}"
text = "${BG_PRIMARY}"

[colors.selection]
background = "${SELECTION}"
text = "${FG_PRIMARY}"

[colors.normal]
black = "${BG_SURFACE}"
red = "${RED}"
green = "${GREEN}"
yellow = "${YELLOW}"
blue = "${BLUE}"
magenta = "${PURPLE}"
cyan = "${CYAN}"
white = "${FG_SECONDARY}"

[colors.bright]
black = "${BG_TERTIARY}"
red = "${BRIGHT_RED}"
green = "${BRIGHT_GREEN}"
yellow = "${BRIGHT_YELLOW}"
blue = "${BRIGHT_BLUE}"
magenta = "${BRIGHT_PURPLE}"
cyan = "${BRIGHT_CYAN}"
white = "${FG_PRIMARY}"

[cursor]
style = { shape = "Block", blinking = "On" }
blink_interval = 750

[scrolling]
history = 10000
multiplier = 3
