#!/usr/bin/env bash
# vulcan-logo.sh - VulcanOS animated logo screensaver
# Displays the VulcanOS logo with a typing/forge glow effect

# Hide cursor and disable keyboard input
tput civis
stty -echo 2>/dev/null

# Cleanup on exit
cleanup() {
    stty echo 2>/dev/null
    tput cnorm
    tput sgr0
    clear
    exit 0
}
trap cleanup INT TERM EXIT

# Colors (ANSI escape sequences)
ORANGE=$'\033[38;2;249;115;22m'   # Forge Orange #f97316
GOLD=$'\033[38;2;251;191;36m'     # Ember Gold #fbbf24
WHITE=$'\033[38;2;250;250;249m'   # Off-white #fafaf9
DIM=$'\033[38;2;120;113;108m'     # Dim gray #78716c
RESET=$'\033[0m'
BOLD=$'\033[1m'

# Get terminal dimensions
cols=$(tput cols)
lines=$(tput lines)

# The VulcanOS ASCII logo
read -r -d '' LOGO << 'EOF'
██╗   ██╗██╗   ██╗██╗      ██████╗ █████╗ ███╗   ██╗
██║   ██║██║   ██║██║     ██╔════╝██╔══██╗████╗  ██║
██║   ██║██║   ██║██║     ██║     ███████║██╔██╗ ██║
╚██╗ ██╔╝██║   ██║██║     ██║     ██╔══██║██║╚██╗██║
 ╚████╔╝ ╚██████╔╝███████╗╚██████╗██║  ██║██║ ╚████║
  ╚═══╝   ╚═════╝ ╚══════╝ ╚═════╝╚═╝  ╚═╝╚═╝  ╚═══╝
EOF

read -r -d '' LOGO_OS << 'EOF'
                ██████╗ ███████╗
                ██╔═══██╗██╔════╝
                ██║   ██║███████╗
                ██║   ██║╚════██║
                ╚██████╔╝███████║
                 ╚═════╝ ╚══════╝
EOF

TAGLINE="Forge your development environment"

# Calculate center position
logo_width=52
logo_height=6
total_height=$((logo_height + logo_height + 2))

start_y=$(( (lines - total_height) / 2 ))
start_x=$(( (cols - logo_width) / 2 ))

# Move cursor to position
goto() {
    printf '\033[%d;%dH' "$1" "$2"
}

# Type out text with delay
type_text() {
    local text="$1"
    local delay="${2:-0.02}"
    local color="${3:-$WHITE}"

    printf "%s" "$color"
    for ((i=0; i<${#text}; i++)); do
        printf "%s" "${text:$i:1}"
        sleep "$delay"
    done
    printf "%s" "$RESET"
}

# Draw logo line by line
draw_logo() {
    local y=$1
    local x=$2
    local color="$3"

    local line_num=0
    while IFS= read -r line; do
        goto $((y + line_num)) "$x"
        type_text "$line" 0.003 "$color"
        ((line_num++))
    done <<< "$LOGO"
}

# Draw OS text
draw_os() {
    local y=$1
    local x=$2
    local color="$3"

    local line_num=0
    while IFS= read -r line; do
        goto $((y + line_num)) "$x"
        type_text "$line" 0.005 "$color"
        ((line_num++))
    done <<< "$LOGO_OS"
}

# Flicker/glow effect
flicker() {
    local iterations=$1
    for ((i=0; i<iterations; i++)); do
        # Random brightness
        case $((RANDOM % 3)) in
            0) printf "%s" "$ORANGE" ;;
            1) printf "%s" "$GOLD" ;;
            2) printf "%s" "$WHITE" ;;
        esac
        sleep 0.1
    done
}

# Main animation loop
while true; do
    clear

    # Draw VULCAN text
    logo_y=$start_y
    draw_logo "$logo_y" "$start_x" "$GOLD"

    # Draw OS below VULCAN (same X position - OS has built-in padding)
    os_y=$((logo_y + logo_height))
    draw_os "$os_y" "$start_x" "$ORANGE"

    # Draw tagline
    tagline_y=$((os_y + logo_height + 1))
    tagline_x=$(( (cols - ${#TAGLINE}) / 2 ))
    goto "$tagline_y" "$tagline_x"
    type_text "$TAGLINE" 0.03 "$DIM"

    # Hold the display
    sleep 5

    # Brief pause before restart
    sleep 1
done
