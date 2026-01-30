/* Waybar Style - ${THEME_NAME} Theme */

* {
    font-family: "JetBrainsMono NF Vulcan", "JetBrainsMono Nerd Font", "JetBrains Mono", monospace;
    font-size: 13px;
    font-weight: 500;
    min-height: 0;
    border: none;
    border-radius: 0;
}

window#waybar {
    background-color: ${BG_PRIMARY};
    color: ${FG_PRIMARY};
}

window#waybar.hidden {
    opacity: 0.2;
}

/* Vulcan Menu Button */
#custom-vulcan-menu {
    background-color: ${BG_SECONDARY};
    color: ${ACCENT};
    padding: 0 10px 0 8px;
    margin: 4px 2px 4px 4px;
    border-radius: 4px;
    border: 1px solid ${BG_TERTIARY};
    font-size: 14px;
    font-family: "JetBrainsMono NF Vulcan";
}

#custom-vulcan-menu:hover {
    background-color: ${ACCENT};
    color: ${BG_PRIMARY};
    border-color: ${ACCENT};
}

/* Separator */
#custom-separator {
    color: ${BG_TERTIARY};
    padding: 0 6px;
}

/* Workspaces - no boxes, just icons */
#workspaces {
    background-color: transparent;
    margin: 0;
}

#workspaces button {
    background-color: transparent;
    color: ${FG_MUTED};
    padding: 0 6px;
    margin: 0;
    border: none;
}

#workspaces button:hover {
    color: ${FG_PRIMARY};
}

#workspaces button.active {
    color: ${ACCENT};
}

#workspaces button.urgent {
    color: ${RED};
}

/* Window title */
#window {
    color: ${FG_SECONDARY};
    padding: 0 10px;
    margin: 4px 0;
}

/* Clock */
#clock {
    background-color: ${BG_SECONDARY};
    color: ${ACCENT};
    padding: 0 12px;
    margin: 4px 0;
    border-radius: 4px;
    border: 1px solid ${BG_TERTIARY};
    font-weight: bold;
}

#clock:hover {
    background-color: ${ACCENT};
    color: ${BG_PRIMARY};
    border-color: ${ACCENT};
}

/* Tray */
#tray {
    background-color: ${BG_SECONDARY};
    padding: 0 8px;
    margin: 4px 2px;
    border-radius: 4px;
    border: 1px solid ${BG_TERTIARY};
}

#tray > .passive {
    -gtk-icon-effect: dim;
}

#tray > .needs-attention {
    background-color: ${RED};
}

/* All right-side modules - consistent accent color */
#custom-notification,
#custom-hyprwhspr,
#bluetooth,
#network,
#pulseaudio,
#cpu,
#memory,
#battery {
    background-color: ${BG_SECONDARY};
    color: ${ACCENT};
    padding: 0 10px;
    margin: 4px 2px;
    border-radius: 4px;
    border: 1px solid ${BG_TERTIARY};
}

#custom-notification:hover,
#custom-hyprwhspr:hover,
#bluetooth:hover,
#network:hover,
#pulseaudio:hover,
#cpu:hover,
#memory:hover,
#battery:hover {
    background-color: ${ACCENT};
    color: ${BG_PRIMARY};
    border-color: ${ACCENT};
}

/* State-based colors */
#bluetooth.connected,
#battery.charging,
#battery.plugged {
    color: ${GREEN};
}

#bluetooth.disabled,
#pulseaudio.muted {
    color: ${FG_MUTED};
}

#network.disconnected {
    color: ${RED};
}

#battery.warning:not(.charging) {
    color: ${YELLOW};
}

#battery.critical:not(.charging) {
    background-color: ${RED};
    color: ${BG_PRIMARY};
    animation-name: blink;
    animation-duration: 0.5s;
    animation-timing-function: linear;
    animation-iteration-count: infinite;
    animation-direction: alternate;
}

@keyframes blink {
    to {
        background-color: ${BG_PRIMARY};
        color: ${RED};
    }
}

/* Tooltips */
tooltip {
    background-color: ${BG_PRIMARY};
    border: 1px solid ${BG_TERTIARY};
    border-radius: 4px;
}

tooltip label {
    color: ${FG_PRIMARY};
    padding: 4px;
}
