/* SwayNC Style - ${THEME_NAME} Theme */

* {
    font-family: "JetBrainsMono Nerd Font", monospace;
    font-size: 13px;
}

.notification-row {
    outline: none;
}

.notification-row:focus,
.notification-row:hover {
    background: ${BG_SECONDARY};
}

.notification {
    border-radius: 10px;
    margin: 6px 12px;
    padding: 0;
    background: ${BG_PRIMARY};
    border: 1px solid ${BG_TERTIARY};
}

.notification-content {
    background: transparent;
    padding: 6px;
}

.close-button {
    background: ${RED};
    color: ${BG_PRIMARY};
    text-shadow: none;
    padding: 0;
    border-radius: 100%;
    margin-top: 10px;
    margin-right: 10px;
    box-shadow: none;
    border: none;
    min-width: 24px;
    min-height: 24px;
}

.close-button:hover {
    background: ${BRIGHT_RED};
}

.notification-default-action,
.notification-action {
    padding: 4px;
    margin: 0;
    background: transparent;
    border: none;
    color: ${FG_PRIMARY};
}

.notification-default-action:hover,
.notification-action:hover {
    background: ${BG_SECONDARY};
    border-radius: 6px;
}

.notification-default-action {
    border-radius: 10px;
}

.notification-action:first-child {
    border-radius: 0 0 0 10px;
}

.notification-action:last-child {
    border-radius: 0 0 10px 0;
}

.inline-reply {
    margin-top: 8px;
}

.inline-reply-entry {
    background: ${BG_SECONDARY};
    color: ${FG_PRIMARY};
    caret-color: ${FG_PRIMARY};
    border: 1px solid ${BG_TERTIARY};
    border-radius: 6px;
    padding: 6px;
}

.inline-reply-entry:focus {
    border-color: ${ACCENT};
}

.inline-reply-button {
    margin-left: 4px;
    background: ${ACCENT};
    color: ${BG_PRIMARY};
    border-radius: 6px;
    border: none;
    padding: 6px 12px;
}

.inline-reply-button:disabled {
    background: ${BG_TERTIARY};
}

.inline-reply-button:hover {
    background: ${ACCENT_ALT};
}

.image {
    border-radius: 6px;
    margin-right: 6px;
}

.summary {
    font-size: 14px;
    font-weight: bold;
    background: transparent;
    color: ${FG_PRIMARY};
}

.time {
    font-size: 12px;
    font-weight: bold;
    background: transparent;
    color: ${FG_MUTED};
    margin-right: 24px;
}

.body {
    font-size: 13px;
    font-weight: normal;
    background: transparent;
    color: ${FG_SECONDARY};
}

.control-center {
    background: ${BG_PRIMARY};
    border: 1px solid ${BG_TERTIARY};
    border-radius: 10px;
}

.control-center-list {
    background: transparent;
}

.control-center-list-placeholder {
    opacity: 0.5;
}

.floating-notifications {
    background: transparent;
}

.blank-window {
    background: alpha(${BG_PRIMARY}, 0.3);
}

.widget-title {
    margin: 8px;
    font-size: 14px;
    font-weight: bold;
    color: ${FG_PRIMARY};
}

.widget-title > button {
    font-size: 12px;
    color: ${FG_PRIMARY};
    text-shadow: none;
    background: ${BG_SECONDARY};
    border: 1px solid ${BG_TERTIARY};
    border-radius: 6px;
    padding: 4px 8px;
}

.widget-title > button:hover {
    background: ${BG_TERTIARY};
    border-color: ${ACCENT};
}

.widget-dnd {
    margin: 8px;
    font-size: 14px;
    color: ${FG_PRIMARY};
}

.widget-dnd > switch {
    font-size: 14px;
    border-radius: 6px;
    background: ${BG_SECONDARY};
    border: 1px solid ${BG_TERTIARY};
}

.widget-dnd > switch:checked {
    background: ${ACCENT};
    border-color: ${ACCENT};
}

.widget-dnd > switch slider {
    background: ${FG_PRIMARY};
    border-radius: 6px;
}

.widget-label {
    margin: 8px;
}

.widget-label > label {
    font-size: 14px;
    color: ${FG_PRIMARY};
}

.widget-mpris {
    background: ${BG_SECONDARY};
    padding: 8px;
    margin: 8px;
    border-radius: 10px;
}

.widget-mpris-player {
    padding: 8px;
    margin: 4px;
}

.widget-mpris-title {
    font-weight: bold;
    font-size: 14px;
    color: ${FG_PRIMARY};
}

.widget-mpris-subtitle {
    font-size: 12px;
    color: ${FG_SECONDARY};
}

.widget-buttons-grid {
    font-size: 14px;
    padding: 4px;
    margin: 8px;
    border-radius: 10px;
    background: ${BG_SECONDARY};
}

.widget-buttons-grid > flowbox > flowboxchild > button {
    margin: 4px;
    background: ${BG_TERTIARY};
    border-radius: 6px;
    color: ${FG_PRIMARY};
    border: none;
    padding: 8px;
}

.widget-buttons-grid > flowbox > flowboxchild > button:hover {
    background: ${ACCENT};
    color: ${BG_PRIMARY};
}

.widget-menubar > box > .menu-button-bar > button {
    border: none;
    background: transparent;
}

.topbar-buttons > button {
    border: none;
    background: transparent;
}

.widget-volume {
    background: ${BG_SECONDARY};
    padding: 8px;
    margin: 8px;
    border-radius: 10px;
}

.widget-volume > box > button {
    background: transparent;
    border: none;
}

.per-app-volume {
    background: ${BG_TERTIARY};
    padding: 4px 8px 8px 8px;
    margin: 4px 8px;
    border-radius: 10px;
}

.widget-backlight {
    background: ${BG_SECONDARY};
    padding: 8px;
    margin: 8px;
    border-radius: 10px;
}

.widget-inhibitors {
    margin: 8px;
    font-size: 14px;
    color: ${FG_PRIMARY};
}

.widget-inhibitors > button {
    font-size: 12px;
    color: ${FG_PRIMARY};
    text-shadow: none;
    background: ${BG_SECONDARY};
    border: 1px solid ${BG_TERTIARY};
    border-radius: 6px;
    padding: 4px 8px;
}

.widget-inhibitors > button:hover {
    background: ${BG_TERTIARY};
}
