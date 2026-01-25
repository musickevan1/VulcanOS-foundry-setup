/// VulcanOS brand colors and shared CSS module
///
/// This module provides a single source of truth for VulcanOS brand colors and styling.
/// It exports:
/// - Rust constants for programmatic access (colors module)
/// - GTK4 CSS strings for runtime styling (FULL_CSS)
///
/// All color values match branding/vulcan-palette.css exactly.

/// Rust constants for VulcanOS brand colors
///
/// Use these for programmatic color access (validation, swatch generation, etc.)
pub mod colors {
    // Primary accent colors - warm forge palette
    pub const EMBER: &str = "#f97316";
    pub const MOLTEN: &str = "#ea580c";
    pub const GOLD: &str = "#fbbf24";
    pub const FLAME: &str = "#dc2626";

    // Background colors - dark forge workshop
    pub const OBSIDIAN: &str = "#1c1917";
    pub const CHARCOAL: &str = "#292524";
    pub const ASH: &str = "#44403c";
    pub const SMOKE: &str = "#57534e";

    // Text colors
    pub const WHITE: &str = "#fafaf9";
    pub const STONE: &str = "#a8a29e";
    pub const GRAY: &str = "#78716c";

    // Semantic colors
    pub const SUCCESS: &str = "#22c55e";
    pub const WARNING: &str = "#fbbf24";
    pub const ERROR: &str = "#ef4444";
    pub const INFO: &str = "#3b82f6";
}

/// GTK4 brand color definitions
///
/// Uses @define-color syntax (GTK4) NOT CSS custom properties
const BRAND_COLORS_CSS: &str = r#"
/* ═══════════════════════════════════════════════════════════════════════════
 * VulcanOS Brand Colors
 * Single source of truth matching branding/vulcan-palette.css
 * ═══════════════════════════════════════════════════════════════════════════ */

/* Primary accent colors */
@define-color vulcan_ember #f97316;
@define-color vulcan_molten #ea580c;
@define-color vulcan_gold #fbbf24;
@define-color vulcan_flame #dc2626;

/* Background colors */
@define-color vulcan_obsidian #1c1917;
@define-color vulcan_charcoal #292524;
@define-color vulcan_ash #44403c;
@define-color vulcan_smoke #57534e;

/* Text colors */
@define-color vulcan_white #fafaf9;
@define-color vulcan_stone #a8a29e;
@define-color vulcan_gray #78716c;

/* Semantic colors */
@define-color vulcan_success #22c55e;
@define-color vulcan_warning #fbbf24;
@define-color vulcan_error #ef4444;
@define-color vulcan_info #3b82f6;

/* Override Adwaita accent colors with Vulcan ember */
@define-color accent_bg_color @vulcan_ember;
@define-color accent_fg_color @vulcan_white;
@define-color accent_color @vulcan_ember;
"#;

/// Shared widget styles for both theme-manager and wallpaper-manager
///
/// Merged from both old main.rs files to eliminate duplication
const WIDGET_CSS: &str = r#"
/* ═══════════════════════════════════════════════════════════════════════════
 * Window and Layout
 * ═══════════════════════════════════════════════════════════════════════════ */

window, .background {
    background-color: @vulcan_obsidian;
}

/* Header bar styling */
headerbar {
    background: linear-gradient(to bottom, @vulcan_charcoal, shade(@vulcan_obsidian, 1.1));
    border-bottom: 1px solid @vulcan_ash;
}

headerbar title {
    color: @vulcan_white;
    font-weight: 600;
}

/* Frame backgrounds */
frame > border {
    background-color: @vulcan_charcoal;
    border-color: @vulcan_ash;
    border-radius: 8px;
}

/* Paned separator */
paned > separator {
    background-color: @vulcan_ash;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * Typography
 * ═══════════════════════════════════════════════════════════════════════════ */

label {
    color: @vulcan_white;
}

label.dim-label {
    color: @vulcan_stone;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * Buttons
 * ═══════════════════════════════════════════════════════════════════════════ */

/* Suggested action button (Apply) - Vulcan ember */
button.suggested-action {
    background: linear-gradient(to bottom, @vulcan_ember, @vulcan_molten);
    color: @vulcan_white;
    border: none;
    font-weight: 600;
}

button.suggested-action:hover {
    background: linear-gradient(to bottom, shade(@vulcan_ember, 1.1), @vulcan_ember);
}

button.suggested-action:active {
    background: @vulcan_molten;
}

/* Regular buttons */
button {
    background-color: @vulcan_charcoal;
    color: @vulcan_white;
    border: 1px solid @vulcan_ash;
}

button:hover {
    background-color: shade(@vulcan_charcoal, 1.2);
}

/* ═══════════════════════════════════════════════════════════════════════════
 * Form Controls
 * ═══════════════════════════════════════════════════════════════════════════ */

/* Dropdown/ComboBox */
dropdown, combobox {
    background-color: @vulcan_charcoal;
    color: @vulcan_white;
    border: 1px solid @vulcan_ash;
}

dropdown button, combobox button {
    background-color: @vulcan_charcoal;
}

/* Entry fields */
entry {
    background-color: @vulcan_charcoal;
    color: @vulcan_white;
    border: 1px solid @vulcan_ash;
    caret-color: @vulcan_ember;
}

entry:focus {
    border-color: @vulcan_ember;
    box-shadow: 0 0 3px alpha(@vulcan_ember, 0.5);
}

/* Color button */
colorbutton {
    border-radius: 4px;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * Selection and Navigation
 * ═══════════════════════════════════════════════════════════════════════════ */

/* FlowBox selection (wallpaper thumbnails & theme cards) */
flowboxchild:selected {
    background-color: alpha(@vulcan_ember, 0.3);
    border: 2px solid @vulcan_ember;
    border-radius: 8px;
}

/* Scrollbar */
scrollbar trough {
    background-color: @vulcan_obsidian;
}

scrollbar slider {
    background-color: @vulcan_ash;
    border-radius: 4px;
}

scrollbar slider:hover {
    background-color: @vulcan_stone;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * Overlays and Menus
 * ═══════════════════════════════════════════════════════════════════════════ */

/* Popover menus */
popover, popover.background {
    background-color: @vulcan_charcoal;
    border: 1px solid @vulcan_ash;
}

popover modelbutton:hover {
    background-color: alpha(@vulcan_ember, 0.2);
}

/* Tooltip */
tooltip {
    background-color: @vulcan_charcoal;
    color: @vulcan_white;
    border: 1px solid @vulcan_ash;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * Theme Manager Specific Styles
 * ═══════════════════════════════════════════════════════════════════════════ */

/* Theme card styling */
.theme-card {
    padding: 8px;
    border-radius: 8px;
    background-color: @vulcan_charcoal;
    transition: background-color 150ms;
}

.theme-card:hover {
    background-color: shade(@vulcan_charcoal, 1.15);
}

.theme-name {
    font-weight: 600;
    font-size: 13px;
}

.current-badge {
    font-size: 10px;
    padding: 2px 8px;
    border-radius: 4px;
    background-color: @vulcan_ember;
    color: @vulcan_white;
}

.builtin-badge {
    font-size: 10px;
    padding: 2px 8px;
    border-radius: 4px;
    background-color: @vulcan_ash;
    color: @vulcan_stone;
}

/* Color preview frame */
.color-preview-frame {
    border-radius: 6px;
    padding: 2px;
}

/* Preview panel */
.preview-frame {
    border-radius: 8px;
    border: 1px solid @vulcan_ash;
}

/* Expander rows (color groups) */
row.expander {
    background-color: @vulcan_charcoal;
}

row.expander:hover {
    background-color: shade(@vulcan_charcoal, 1.1);
}

/* ListBox boxed-list */
.boxed-list {
    background-color: @vulcan_charcoal;
    border-radius: 8px;
    border: 1px solid @vulcan_ash;
}
"#;

/// Complete CSS combining brand colors and widget styles
///
/// This is the string loaded by relm4::set_global_css() in main.rs
pub const FULL_CSS: &str = const_format::concatcp!(BRAND_COLORS_CSS, WIDGET_CSS);
