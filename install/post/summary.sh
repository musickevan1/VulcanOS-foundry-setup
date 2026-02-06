#!/bin/bash
# =============================================================================
# VulcanOS Installer - Post-Install Summary
# =============================================================================

run_summary() {
    local elapsed="${1:-unknown}"

    echo ""
    echo -e "${GREEN}==========================================${NC}"
    echo -e "${GREEN}  VulcanOS ${VULCAN_PROFILE^} - Setup Complete!${NC}"
    echo -e "${GREEN}==========================================${NC}"
    echo ""
    echo -e "  Profile:  ${BOLD}${VULCAN_PROFILE}${NC}"
    echo -e "  GPU:      ${BOLD}${VULCAN_GPU:-unknown}${NC}"
    echo -e "  CPU:      ${BOLD}${VULCAN_CPU:-unknown}${NC}"
    echo -e "  Duration: ${BOLD}${elapsed}${NC}"
    echo ""
    echo "Next steps:"
    echo ""
    echo "  1. Reboot:"
    echo "       sudo reboot"
    echo ""
    echo "  2. Log in via SDDM → Hyprland"
    echo ""

    # Profile-specific post-install hints
    case "${VULCAN_PROFILE}" in
        foundry)
            echo "  3. Install OpenCode (AI assistant):"
            echo "       curl -fsSL https://opencode.ai/install | bash"
            echo ""
            echo "  4. PyTorch nightly for newer GPUs:"
            echo "       pip install --pre torch torchvision torchaudio \\"
            echo "         --index-url https://download.pytorch.org/whl/nightly/cu128"
            echo ""
            ;;
        t2)
            echo "  3. Connect WiFi via iwctl:"
            echo "       iwctl station wlan0 connect <SSID>"
            echo ""
            ;;
    esac

    echo "  Optional — install yay for AUR packages:"
    echo "       git clone https://aur.archlinux.org/yay.git /tmp/yay"
    echo "       cd /tmp/yay && makepkg -si"
    echo ""
    echo "Verify:"
    echo "  hostname           → $(cat /etc/hostname 2>/dev/null || echo 'vulcan')"

    if [[ "${VULCAN_GPU:-}" == "nvidia" ]]; then
        echo "  nvidia-smi         → GPU detected"
        echo "  echo \$GBM_BACKEND  → nvidia-drm"
    fi

    echo "  ls -la ~/.config/hypr → stow symlink to VulcanOS"
    echo ""
}
