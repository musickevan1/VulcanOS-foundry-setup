#!/usr/bin/env bash
# pipes.sh - Animated pipes terminal screensaver
# Based on pipeseroni/pipes.sh (MIT License)
# VulcanOS edition - themed with forge colors (red/yellow)
#
# Usage: pipes.sh [options]
#   -c COLOR  Add a color (1=red, 2=green, 3=yellow, 4=blue, 5=magenta, 6=cyan)
#   -p N      Number of pipes (default: 1)
#   -t N      Pipe type 0-9 (default: 0)
#   -r N      Reset after N characters (default: 2000)
#   -R        Random starting positions
#   -h        Show help

# Default VulcanOS forge colors: red and yellow
declare -a colors=(1 3)
p=1
t=0
r=2000
random_start=0

# Parse arguments
while getopts "c:p:t:r:Rh" opt; do
    case $opt in
        c) colors+=("$OPTARG") ;;
        p) p=$OPTARG ;;
        t) t=$OPTARG ;;
        r) r=$OPTARG ;;
        R) random_start=1 ;;
        h) echo "Usage: pipes.sh [-c color] [-p pipes] [-t type] [-r reset] [-R random]"; exit 0 ;;
    esac
done

# Pipe characters for different styles
declare -a sets=(
    "┃┏ ┓┛━┓  ┗┃┛┗ ┏━"
    "│╭ ╮╯─╮  ╰│╯╰ ╭─"
    "│┌ ┐┘─┐  └│┘└ ┌─"
    "║╔ ╗╝═╗  ╚║╝╚ ╔═"
    "|+ ++-+  +|++ +-"
    "|/ \\/-\\  \\|/\\ /-"
    ".. ....  .... .."
    ".o oo.o" "o. .oo."
    "-ede-ede-"
)

# Hide cursor, clear screen
tput civis
clear

# Cleanup on exit
cleanup() {
    tput cnorm
    tput sgr0
    clear
    exit 0
}
trap cleanup INT TERM EXIT

# Direction changes: 0=up, 1=right, 2=down, 3=left
declare -a dx=(0 1 0 -1)
declare -a dy=(-1 0 1 0)

# Initialize
declare -i w h n=0
declare -a x y d c

init_pipes() {
    w=$(tput cols)
    h=$(tput lines)
    for ((i = 0; i < p; i++)); do
        if ((random_start)); then
            x[i]=$((RANDOM % w))
            y[i]=$((RANDOM % h))
        else
            x[i]=$((w / 2))
            y[i]=$((h / 2))
        fi
        d[i]=$((RANDOM % 4))
        c[i]=${colors[$((RANDOM % ${#colors[@]}))]}
    done
}

# Initialize pipes (launcher script handles fullscreen delay)
init_pipes

# Track last known size for resize detection
last_w=$w
last_h=$h

# Main loop
while :; do
    # Check for resize
    cur_w=$(tput cols)
    cur_h=$(tput lines)
    if ((cur_w != last_w || cur_h != last_h)); then
        last_w=$cur_w
        last_h=$cur_h
        w=$cur_w
        h=$cur_h
        clear
        n=0
    fi

    for ((i = 0; i < p; i++)); do
        # Possibly change direction
        if ((RANDOM % 3 == 0)); then
            case ${d[i]} in
                0|2) d[i]=$((RANDOM % 2 * 2 + 1)) ;;
                1|3) d[i]=$((RANDOM % 2 * 2)) ;;
            esac
        fi

        # Move pipe
        x[i]=$((x[i] + dx[d[i]]))
        y[i]=$((y[i] + dy[d[i]]))

        # Wrap around screen
        ((x[i] < 0)) && x[i]=$((w - 1))
        ((x[i] >= w)) && x[i]=0
        ((y[i] < 0)) && y[i]=$((h - 1))
        ((y[i] >= h)) && y[i]=0

        # Draw pipe character
        printf '\033[%d;%dH\033[1;3%dm%s' \
            $((y[i] + 1)) $((x[i] + 1)) ${c[i]} \
            "${sets[t]:$((d[i] * 2)):1}"
    done

    # Reset after n characters
    ((++n > r)) && { n=0; clear; }

    # Small delay for smooth animation
    sleep 0.03
done
