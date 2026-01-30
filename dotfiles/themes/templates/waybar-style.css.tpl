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
    transition-property: background-color;
    transition-duration: 0.5s;
}

window#waybar.hidden {
    opacity: 0.2;
}

/* Vulcan Menu Button - uses U+E900 from patched Vulcan font */
#custom-vulcan-menu {
    background-color: ${BG_SECONDARY};
    color: ${ACCENT};
    padding: 0 10px;
    margin: 4px 2px 4px 4px;
    border-radius: 4px;
    border: 1px solid ${BG_TERTIARY};
    font-size: 14px;
    font-family: "JetBrainsMono NF Vulcan";
}

#custom-vulcan-menu label {
    font-family: "JetBrainsMono NF Vulcan";
    padding-bottom: 2px;
}

#custom-vulcan-menu:hover {
    background-color: ${ACCENT};
    color: ${BG_PRIMARY};
    border-color: ${ACCENT};
}

#workspaces {
    background-color: transparent;
    margin: 0 4px;
}

#workspaces button {
    background-color: ${BG_SECONDARY};
    color: ${FG_MUTED};
    padding: 0 8px;
    margin: 4px 2px;
    border-radius: 4px;
    border: 1px solid ${BG_TERTIARY};
    transition: all 0.3s ease;
}

#workspaces button:hover {
    background-color: ${ACCENT};
    color: ${BG_PRIMARY};
    border-color: ${ACCENT};
}

#workspaces button.active {
    background-color: ${ACCENT};
    color: ${BG_PRIMARY};
    border-color: ${ACCENT};
    font-weight: bold;
}

#workspaces button.urgent {
    background-color: ${RED};
    color: ${BG_PRIMARY};
    border-color: ${RED};
}

#custom-separator {
    color: ${BG_TERTIARY};
    padding: 0 4px;
}

#window {
    color: ${FG_SECONDARY};
    padding: 0 10px;
    margin: 4px 0;
}

/* Clock - Primary Accent */
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
    -gtk-icon-effect: highlight;
    background-color: ${RED};
}

/* Alternating modules: ACCENT (orange) and ACCENT_ALT (yellow) */

#bluetooth {
    background-color: ${BG_SECONDARY};
    color: ${ACCENT};
    padding: 0 10px;
    margin: 4px 2px;
    border-radius: 4px;
    border: 1px solid ${BG_TERTIARY};
}

#bluetooth:hover {
    background-color: ${ACCENT};
    color: ${BG_PRIMARY};
    border-color: ${ACCENT};
}

#bluetooth.connected {
    color: ${GREEN};
}

#bluetooth.disabled {
    color: ${FG_MUTED};
}

#network {
    background-color: ${BG_SECONDARY};
    color: ${ACCENT_ALT};
    padding: 0 10px;
    margin: 4px 2px;
    border-radius: 4px;
    border: 1px solid ${BG_TERTIARY};
}

#network:hover {
    background-color: ${ACCENT_ALT};
    color: ${BG_PRIMARY};
    border-color: ${ACCENT_ALT};
}

#network.disconnected {
    color: ${RED};
}

#network.linked {
    color: ${ACCENT};
}

#pulseaudio {
    background-color: ${BG_SECONDARY};
    color: ${ACCENT};
    padding: 0 10px;
    margin: 4px 2px;
    border-radius: 4px;
    border: 1px solid ${BG_TERTIARY};
}

#pulseaudio:hover {
    background-color: ${ACCENT};
    color: ${BG_PRIMARY};
    border-color: ${ACCENT};
}

#pulseaudio.muted {
    color: ${FG_MUTED};
}

#cpu {
    background-color: ${BG_SECONDARY};
    color: ${ACCENT_ALT};
    padding: 0 10px;
    margin: 4px 2px;
    border-radius: 4px;
    border: 1px solid ${BG_TERTIARY};
}

#cpu:hover {
    background-color: ${ACCENT_ALT};
    color: ${BG_PRIMARY};
    border-color: ${ACCENT_ALT};
}

#memory {
    background-color: ${BG_SECONDARY};
    color: ${ACCENT};
    padding: 0 10px;
    margin: 4px 2px;
    border-radius: 4px;
    border: 1px solid ${BG_TERTIARY};
}

#memory:hover {
    background-color: ${ACCENT};
    color: ${BG_PRIMARY};
    border-color: ${ACCENT};
}

#battery {
    background-color: ${BG_SECONDARY};
    color: ${ACCENT_ALT};
    padding: 0 10px;
    margin: 4px 2px 4px 2px;
    border-radius: 4px;
    border: 1px solid ${BG_TERTIARY};
}

#battery:hover {
    background-color: ${ACCENT_ALT};
    color: ${BG_PRIMARY};
    border-color: ${ACCENT_ALT};
}

#battery.charging,
#battery.plugged {
    color: ${GREEN};
}

#battery.warning:not(.charging) {
    color: ${ACCENT};
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

tooltip {
    background-color: ${BG_PRIMARY};
    border: 1px solid ${BG_TERTIARY};
    border-radius: 4px;
}

tooltip label {
    color: ${FG_PRIMARY};
    padding: 4px;
}
