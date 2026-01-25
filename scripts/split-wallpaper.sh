#!/bin/bash
# =============================================================================
# VulcanOS Wallpaper Splitter
# Splits a panoramic image into per-monitor wallpapers for the desktop profile
# =============================================================================

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Desktop profile monitor layout (from ~/.config/hyprmon-desc/profiles/desktop.conf)
# ┌─────────┐  ┌─────────┐  ┌─────────┐
# │ DP-10   │  │ DP-12   │  │ DP-5    │
# │  VERT   │  │ 1920×   │  │ 1920×   │
# │ 1080×   │  │  1080   │  │  1080   │
# │  1920   │  ├─────────┤  ├─────────┤
# │         │  │ DP-14   │  │ eDP-1   │
# │         │  │ 1920×   │  │ 1920×   │
# │         │  │  1080   │  │  1200   │
# └─────────┘  └─────────┘  └─────────┘
#
# Positions (for cropping from panoramic):
# DP-10:  0,120    1080x1920 (vertical - rotated 90°)
# DP-12:  1080,0   1920x1080
# DP-14:  1080,1080 1920x1080
# DP-5:   3000,0   1920x1080
# eDP-1:  3000,1080 1920x1200

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WALLPAPER_DIR="${HOME}/Pictures/Wallpapers"
OUTPUT_DIR="${WALLPAPER_DIR}/spanning"

# Monitor definitions: name, width, height, x_offset, y_offset
declare -A MONITORS=(
    ["DP-10"]="1080 1920 0 120"      # Vertical monitor (rotated)
    ["DP-12"]="1920 1080 1080 0"     # Center top
    ["DP-14"]="1920 1080 1080 1080"  # Center bottom (Sceptre)
    ["DP-5"]="1920 1080 3000 0"      # Right top (Float2 Pro)
    ["eDP-1"]="1920 1200 3000 1080"  # MacBook (scaled output)
)

# Total canvas size
CANVAS_WIDTH=4920
CANVAS_HEIGHT=2280

usage() {
    echo -e "${CYAN}VulcanOS Wallpaper Splitter${NC}"
    echo ""
    echo "Usage: $0 <panoramic-image> [output-prefix]"
    echo "       $0 --apply [output-prefix]"
    echo ""
    echo "Arguments:"
    echo "  panoramic-image   Path to the wide panoramic wallpaper image"
    echo "  output-prefix     Optional prefix for output files (default: vulcan-forge)"
    echo ""
    echo "Example:"
    echo "  $0 ~/Downloads/forge-panorama.png vulcan-forge"
    echo "  $0 --apply vulcan-forge"
    echo ""
    echo "Output:"
    echo "  Creates individual wallpapers in ${OUTPUT_DIR}/"
    echo "  - vulcan-forge-DP-10.png (vertical)"
    echo "  - vulcan-forge-DP-12.png (center top)"
    echo "  - vulcan-forge-DP-14.png (center bottom)"
    echo "  - vulcan-forge-DP-5.png (right top)"
    echo "  - vulcan-forge-eDP-1.png (MacBook)"
    echo ""
    echo "Environment variables:"
    echo "  SWWW_TRANSITION   Transition type (default: wipe)"
    echo "  SWWW_DURATION     Transition duration in seconds (default: 1)"
}

check_dependencies() {
    local missing=()

    if ! command -v magick &> /dev/null && ! command -v convert &> /dev/null; then
        missing+=("imagemagick")
    fi

    if ! command -v identify &> /dev/null; then
        missing+=("imagemagick")
    fi

    if [ ${#missing[@]} -gt 0 ]; then
        echo -e "${RED}Error: Missing dependencies: ${missing[*]}${NC}"
        echo "Install with: sudo pacman -S imagemagick"
        exit 1
    fi
}

get_image_dimensions() {
    local image="$1"
    identify -format "%wx%h" "$image" 2>/dev/null
}

split_wallpaper() {
    local input_image="$1"
    local prefix="${2:-vulcan-forge}"

    # Verify input exists
    if [ ! -f "$input_image" ]; then
        echo -e "${RED}Error: Image not found: $input_image${NC}"
        exit 1
    fi

    # Get input dimensions
    local dimensions
    dimensions=$(get_image_dimensions "$input_image")
    local input_width="${dimensions%x*}"
    local input_height="${dimensions#*x}"

    echo -e "${CYAN}Input image: ${NC}$input_image"
    echo -e "${CYAN}Dimensions:  ${NC}${input_width}x${input_height}"
    echo -e "${CYAN}Required:    ${NC}${CANVAS_WIDTH}x${CANVAS_HEIGHT} (minimum)"
    echo ""

    # Warn if image is smaller than canvas
    if [ "$input_width" -lt "$CANVAS_WIDTH" ] || [ "$input_height" -lt "$CANVAS_HEIGHT" ]; then
        echo -e "${YELLOW}Warning: Image is smaller than the monitor canvas.${NC}"
        echo -e "${YELLOW}The image will be scaled up, which may reduce quality.${NC}"
        echo ""
        read -p "Continue anyway? [y/N] " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 1
        fi
    fi

    # Create output directory
    mkdir -p "$OUTPUT_DIR"

    # First, resize/scale the input to match our canvas if needed
    local working_image="${OUTPUT_DIR}/.working-canvas.png"
    echo -e "${CYAN}Preparing canvas...${NC}"

    # Scale to fit canvas while maintaining aspect ratio, then crop to exact size
    magick "$input_image" \
        -resize "${CANVAS_WIDTH}x${CANVAS_HEIGHT}^" \
        -gravity center \
        -extent "${CANVAS_WIDTH}x${CANVAS_HEIGHT}" \
        "$working_image"

    echo -e "${GREEN}Canvas prepared: ${CANVAS_WIDTH}x${CANVAS_HEIGHT}${NC}"
    echo ""

    # Split into individual monitor wallpapers
    echo -e "${CYAN}Splitting into monitor wallpapers...${NC}"

    for monitor in "${!MONITORS[@]}"; do
        IFS=' ' read -r width height x_off y_off <<< "${MONITORS[$monitor]}"
        local output_file="${OUTPUT_DIR}/${prefix}-${monitor}.png"

        echo -n "  ${monitor} (${width}x${height} @ ${x_off},${y_off})... "

        magick "$working_image" \
            -crop "${width}x${height}+${x_off}+${y_off}" \
            +repage \
            "$output_file"

        echo -e "${GREEN}✓${NC}"
    done

    # Clean up working file
    rm -f "$working_image"

    echo ""
    echo -e "${GREEN}Wallpapers created in: ${OUTPUT_DIR}/${NC}"
    echo ""
    echo "Files created:"
    ls -la "${OUTPUT_DIR}/${prefix}-"*.png 2>/dev/null | awk '{print "  " $NF}'
    echo ""
    echo -e "${CYAN}To apply these wallpapers, run:${NC}"
    echo "  $0 --apply $prefix"
}

apply_wallpapers() {
    local prefix="${1:-vulcan-forge}"
    local transition_type="${SWWW_TRANSITION:-wipe}"
    local transition_duration="${SWWW_DURATION:-1}"

    echo -e "${CYAN}Applying wallpapers with prefix: ${prefix}${NC}"

    # Verify wallpapers exist
    local missing=()
    for monitor in "${!MONITORS[@]}"; do
        local wallpaper="${OUTPUT_DIR}/${prefix}-${monitor}.png"
        if [ ! -f "$wallpaper" ]; then
            missing+=("$monitor")
        fi
    done

    if [ ${#missing[@]} -gt 0 ]; then
        echo -e "${RED}Error: Missing wallpapers for: ${missing[*]}${NC}"
        echo "Run split first: $0 <image> $prefix"
        exit 1
    fi

    # Check if swww-daemon is running
    if ! pgrep -x swww-daemon > /dev/null; then
        echo -e "${YELLOW}Starting swww-daemon...${NC}"
        swww-daemon &
        disown
        sleep 0.5
    fi

    # Apply wallpapers to each monitor via swww
    echo -e "${CYAN}Applying wallpapers via swww...${NC}"
    for monitor in "${!MONITORS[@]}"; do
        local wallpaper="${OUTPUT_DIR}/${prefix}-${monitor}.png"
        echo -n "  ${monitor}... "
        swww img -o "$monitor" "$wallpaper" \
            --transition-type "$transition_type" \
            --transition-duration "$transition_duration" \
            --transition-angle 0 &
    done
    wait

    echo -e "${GREEN}All wallpapers applied!${NC}"
    echo ""

    # Show current state
    echo -e "${CYAN}Current wallpaper state:${NC}"
    swww query | while read -r line; do
        echo "  $line"
    done
}

# Main
check_dependencies

case "${1:-}" in
    -h|--help)
        usage
        exit 0
        ;;
    --apply)
        apply_wallpapers "${2:-vulcan-forge}"
        ;;
    "")
        usage
        exit 1
        ;;
    *)
        split_wallpaper "$1" "${2:-vulcan-forge}"
        ;;
esac
