#!/bin/bash
# =============================================================================
# VulcanOS - Boot Script
# One-liner entry point: wget -qO- https://raw.githubusercontent.com/musickevan1/VulcanOS/main/boot.sh | bash
#
# Clones the VulcanOS repo, detects available profiles, and hands off to the
# modular installer.
# =============================================================================

set -euo pipefail

REPO_URL="${VULCAN_REPO_URL:-https://github.com/musickevan1/VulcanOS-foundry-setup.git}"
CLONE_DIR="$HOME/VulcanOS"
PROFILES_DIR="archiso/profiles"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
BOLD='\033[1m'
NC='\033[0m'

echo -e "${BOLD}"
echo "  ╦  ╦╦ ╦╦  ╔═╗╔═╗╔╗╔╔═╗╔═╗"
echo "  ╚╗╔╝║ ║║  ║  ╠═╣║║║║ ║╚═╗"
echo "   ╚╝ ╚═╝╩═╝╚═╝╩ ╩╝╚╝╚═╝╚═╝"
echo -e "${NC}"
echo -e "${BLUE}VulcanOS Installer Bootstrap${NC}"
echo ""

# -------------------------------------------------------------------
# Step 1: Clone or update the repo
# -------------------------------------------------------------------
if [[ -d "$CLONE_DIR/.git" ]]; then
    echo -e "${YELLOW}[WARN]${NC} VulcanOS repo already exists at $CLONE_DIR"
    echo -e "${BLUE}[INFO]${NC} Pulling latest changes..."
    git -C "$CLONE_DIR" pull --ff-only || true
else
    echo -e "${BLUE}[INFO]${NC} Cloning VulcanOS to $CLONE_DIR..."
    git clone "$REPO_URL" "$CLONE_DIR"
fi

# -------------------------------------------------------------------
# Step 2: Discover available profiles
# -------------------------------------------------------------------
profiles=()
for dir in "$CLONE_DIR/$PROFILES_DIR"/*/; do
    [[ -d "$dir" ]] && profiles+=("$(basename "$dir")")
done

if [[ ${#profiles[@]} -eq 0 ]]; then
    echo -e "${RED}[ERROR]${NC} No profiles found in $CLONE_DIR/$PROFILES_DIR/"
    exit 1
fi

# -------------------------------------------------------------------
# Step 3: Profile selection
# -------------------------------------------------------------------
selected=""
if [[ ${#profiles[@]} -eq 1 ]]; then
    selected="${profiles[0]}"
    echo -e "${BLUE}[INFO]${NC} Single profile found: ${BOLD}$selected${NC}"
else
    echo ""
    echo -e "${BOLD}Available profiles:${NC}"
    for i in "${!profiles[@]}"; do
        echo "  $((i + 1)). ${profiles[$i]}"
    done
    echo ""

    while true; do
        read -rp "Select a profile [1-${#profiles[@]}]: " choice
        if [[ "$choice" =~ ^[0-9]+$ ]] && (( choice >= 1 && choice <= ${#profiles[@]} )); then
            selected="${profiles[$((choice - 1))]}"
            break
        fi
        echo -e "${RED}Invalid selection${NC}"
    done
fi

echo -e "${GREEN}[OK]${NC} Profile: ${BOLD}$selected${NC}"
echo ""

# -------------------------------------------------------------------
# Step 4: Hand off to the modular installer
# -------------------------------------------------------------------
echo -e "${BLUE}[INFO]${NC} Launching installer..."
echo -e "${YELLOW}[WARN]${NC} You will be prompted for your sudo password."
echo ""

export VULCAN_REPO="$CLONE_DIR"
export VULCAN_PROFILE="$selected"

sudo -E bash "$CLONE_DIR/install.sh"
