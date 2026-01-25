/// Color field definition for the editor
#[derive(Debug, Clone)]
pub struct ColorField {
    /// Display label for the color
    pub label: &'static str,
    /// Field name in Theme struct (for updates)
    pub field: &'static str,
    /// Description/tooltip
    pub description: &'static str,
}

/// Color group for organizing the editor UI
#[derive(Debug, Clone)]
pub struct ColorGroup {
    pub name: &'static str,
    pub fields: Vec<ColorField>,
}

impl ColorGroup {
    /// Get all color groups for the editor
    pub fn all_groups() -> Vec<ColorGroup> {
        vec![
            ColorGroup {
                name: "Backgrounds",
                fields: vec![
                    ColorField {
                        label: "Primary",
                        field: "bg_primary",
                        description: "Main window/container background",
                    },
                    ColorField {
                        label: "Secondary",
                        field: "bg_secondary",
                        description: "Elevated surfaces, cards",
                    },
                    ColorField {
                        label: "Tertiary",
                        field: "bg_tertiary",
                        description: "Borders, subtle separators",
                    },
                    ColorField {
                        label: "Surface",
                        field: "bg_surface",
                        description: "Dividers, input backgrounds",
                    },
                ],
            },
            ColorGroup {
                name: "Foregrounds",
                fields: vec![
                    ColorField {
                        label: "Primary",
                        field: "fg_primary",
                        description: "Main text color",
                    },
                    ColorField {
                        label: "Secondary",
                        field: "fg_secondary",
                        description: "Secondary/dimmed text",
                    },
                    ColorField {
                        label: "Muted",
                        field: "fg_muted",
                        description: "Disabled/placeholder text",
                    },
                ],
            },
            ColorGroup {
                name: "Accents",
                fields: vec![
                    ColorField {
                        label: "Primary Accent",
                        field: "accent",
                        description: "Primary accent color (links, buttons)",
                    },
                    ColorField {
                        label: "Alternate Accent",
                        field: "accent_alt",
                        description: "Secondary accent for highlights",
                    },
                ],
            },
            ColorGroup {
                name: "ANSI Colors",
                fields: vec![
                    ColorField {
                        label: "Red",
                        field: "red",
                        description: "Errors, warnings",
                    },
                    ColorField {
                        label: "Green",
                        field: "green",
                        description: "Success, additions",
                    },
                    ColorField {
                        label: "Yellow",
                        field: "yellow",
                        description: "Warnings, highlights",
                    },
                    ColorField {
                        label: "Blue",
                        field: "blue",
                        description: "Information, links",
                    },
                    ColorField {
                        label: "Purple",
                        field: "purple",
                        description: "Keywords, special",
                    },
                    ColorField {
                        label: "Cyan",
                        field: "cyan",
                        description: "Strings, constants",
                    },
                    ColorField {
                        label: "Orange",
                        field: "orange",
                        description: "Numbers, operators",
                    },
                    ColorField {
                        label: "Pink",
                        field: "pink",
                        description: "Tags, attributes",
                    },
                ],
            },
            ColorGroup {
                name: "Bright ANSI",
                fields: vec![
                    ColorField {
                        label: "Bright Red",
                        field: "bright_red",
                        description: "Bright variant of red",
                    },
                    ColorField {
                        label: "Bright Green",
                        field: "bright_green",
                        description: "Bright variant of green",
                    },
                    ColorField {
                        label: "Bright Yellow",
                        field: "bright_yellow",
                        description: "Bright variant of yellow",
                    },
                    ColorField {
                        label: "Bright Blue",
                        field: "bright_blue",
                        description: "Bright variant of blue",
                    },
                    ColorField {
                        label: "Bright Purple",
                        field: "bright_purple",
                        description: "Bright variant of purple",
                    },
                    ColorField {
                        label: "Bright Cyan",
                        field: "bright_cyan",
                        description: "Bright variant of cyan",
                    },
                ],
            },
            ColorGroup {
                name: "UI Elements",
                fields: vec![
                    ColorField {
                        label: "Active Border",
                        field: "border_active",
                        description: "Focused window border",
                    },
                    ColorField {
                        label: "Inactive Border",
                        field: "border_inactive",
                        description: "Unfocused window border",
                    },
                    ColorField {
                        label: "Selection",
                        field: "selection",
                        description: "Text selection background",
                    },
                    ColorField {
                        label: "Cursor",
                        field: "cursor",
                        description: "Text cursor color",
                    },
                ],
            },
            ColorGroup {
                name: "Gradients",
                fields: vec![
                    ColorField {
                        label: "Gradient Start",
                        field: "gradient_start",
                        description: "Starting color for gradients",
                    },
                    ColorField {
                        label: "Gradient End",
                        field: "gradient_end",
                        description: "Ending color for gradients",
                    },
                ],
            },
            ColorGroup {
                name: "System Themes",
                fields: vec![
                    ColorField {
                        label: "GTK Theme",
                        field: "gtk_theme",
                        description: "GTK application theme name",
                    },
                    ColorField {
                        label: "Icon Theme",
                        field: "icon_theme",
                        description: "Desktop icon theme",
                    },
                    ColorField {
                        label: "Cursor Theme",
                        field: "cursor_theme",
                        description: "Mouse cursor theme",
                    },
                    ColorField {
                        label: "Kvantum Theme",
                        field: "kvantum_theme",
                        description: "Qt/Kvantum theme name",
                    },
                    ColorField {
                        label: "Neovim Colorscheme",
                        field: "nvim_colorscheme",
                        description: "Neovim colorscheme name",
                    },
                    ColorField {
                        label: "Wallpaper",
                        field: "theme_wallpaper",
                        description: "Associated wallpaper filename",
                    },
                ],
            },
        ]
    }
}
