#!/usr/bin/env bash
# vulcan-logo-trace.sh - Animated logo tracer screensaver
# Pipes trace out the VulcanOS logo shape
# VulcanOS edition - themed with forge colors (red/yellow/orange)

# VulcanOS forge colors (ANSI): 1=red, 3=yellow
declare -a colors=(1 3 1 3 1 3)

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

# Get terminal dimensions
w=$(tput cols)
h=$(tput lines)

# Calculate logo position (centered)
# Logo is approximately 20 chars wide, 12 lines tall
logo_w=20
logo_h=12
text_h=3  # VULCAN + OS
total_h=$((logo_h + text_h))

start_x=$(( (w - logo_w) / 2 ))
start_y=$(( (h - total_h) / 2 ))

# Pipe characters
pipe_chars="┃━┏┓┗┛"

# Draw a character at position with color
draw() {
    local x=$1 y=$2 char=$3 color=$4
    printf '\033[%d;%dH\033[1;3%dm%s\033[0m' "$y" "$x" "$color" "$char"
}

# Define logo paths as coordinate sequences (relative to start position)
# The flipped Arch logo (V/crucible shape)
# Each path is: x1,y1 x2,y2 x3,y3 ... (space separated coordinate pairs)

# Left outer edge (top to bottom)
path1="0,0 0,1 1,2 1,3 2,4 2,5 3,6 3,7 4,8 4,9 5,10 5,11"
# Left inner edge (top to bottom)
path2="4,0 4,1 5,2 5,3 6,4 6,5 7,6 7,7 8,8 8,9 9,10 9,11"
# Right inner edge (top to bottom)
path3="15,0 15,1 14,2 14,3 13,4 13,5 12,6 12,7 11,8 11,9 10,10 10,11"
# Right outer edge (top to bottom)
path4="19,0 19,1 18,2 18,3 17,4 17,5 16,6 16,7 15,8 15,9 14,10 14,11"
# Top left horizontal
path5="0,0 1,0 2,0 3,0 4,0"
# Top right horizontal
path6="15,0 16,0 17,0 18,0 19,0"

# All paths
declare -a paths=("$path1" "$path2" "$path3" "$path4" "$path5" "$path6")

# Animation speed (seconds between frames)
speed=0.05

# Trace a single path
trace_path() {
    local path=$1
    local color=$2
    local char=$3

    local prev_x=-1
    local prev_y=-1

    for coord in $path; do
        local x=${coord%,*}
        local y=${coord#*,}

        # Calculate actual screen position
        local screen_x=$((start_x + x))
        local screen_y=$((start_y + y))

        # Choose character based on direction
        if [[ $prev_x -eq -1 ]]; then
            # First point
            draw "$screen_x" "$screen_y" "●" "$color"
        elif [[ $x -eq $prev_x ]]; then
            # Vertical movement
            draw "$screen_x" "$screen_y" "┃" "$color"
        elif [[ $y -eq $prev_y ]]; then
            # Horizontal movement
            draw "$screen_x" "$screen_y" "━" "$color"
        else
            # Diagonal - use backslash or forward slash
            if [[ $((x - prev_x)) -eq $((y - prev_y)) ]]; then
                draw "$screen_x" "$screen_y" "\\" "$color"
            else
                draw "$screen_x" "$screen_y" "/" "$color"
            fi
        fi

        prev_x=$x
        prev_y=$y
        sleep "$speed"
    done
}

# Draw the text "VULCAN" and "OS" below the logo
draw_text() {
    local text_y=$((start_y + logo_h + 1))
    local vulcan="VULCAN"
    local os="OS"

    # Center VULCAN
    local vulcan_x=$(( (w - ${#vulcan}) / 2 ))
    local i=0
    for ((i=0; i<${#vulcan}; i++)); do
        draw $((vulcan_x + i)) "$text_y" "${vulcan:$i:1}" 3
        sleep "$speed"
    done

    # Center OS below
    local os_x=$(( (w - ${#os}) / 2 ))
    text_y=$((text_y + 1))
    for ((i=0; i<${#os}; i++)); do
        draw $((os_x + i)) "$text_y" "${os:$i:1}" 1
        sleep "$speed"
    done
}

# Main animation loop
while true; do
    clear

    # Trace each path with alternating colors
    local color_idx=0
    for path in "${paths[@]}"; do
        trace_path "$path" "${colors[$color_idx]}"
        color_idx=$(( (color_idx + 1) % ${#colors[@]} ))
    done

    # Draw the text
    draw_text

    # Hold the completed logo
    sleep 3

    # Fade out effect (optional - just clear for now)
    sleep 1
done
