mod app;
mod components;
mod models;
mod services;

use relm4::RelmApp;
use app::App;

/// VulcanOS brand CSS - ember orange accent colors
const VULCAN_CSS: &str = r#"
/* VulcanOS Brand Colors */
@define-color vulcan_ember #f97316;
@define-color vulcan_molten #ea580c;
@define-color vulcan_gold #fbbf24;
@define-color vulcan_obsidian #1c1917;
@define-color vulcan_charcoal #292524;
@define-color vulcan_ash #44403c;
@define-color vulcan_white #fafaf9;
@define-color vulcan_stone #a8a29e;

/* Override Adwaita accent colors with Vulcan ember */
@define-color accent_bg_color @vulcan_ember;
@define-color accent_fg_color @vulcan_white;
@define-color accent_color @vulcan_ember;

/* Window and content backgrounds */
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

/* Labels */
label {
    color: @vulcan_white;
}

label.dim-label {
    color: @vulcan_stone;
}

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

/* Dropdown/ComboBox */
dropdown, combobox {
    background-color: @vulcan_charcoal;
    color: @vulcan_white;
    border: 1px solid @vulcan_ash;
}

dropdown button, combobox button {
    background-color: @vulcan_charcoal;
}

/* FlowBox selection (wallpaper thumbnails) */
flowboxchild:selected {
    background-color: alpha(@vulcan_ember, 0.3);
    border: 2px solid @vulcan_ember;
    border-radius: 8px;
}

/* Paned separator */
paned > separator {
    background-color: @vulcan_ash;
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
"#;

fn main() {
    let app = RelmApp::new("com.vulcanos.appearance-manager");

    // Load Vulcan brand CSS globally
    relm4::set_global_css(VULCAN_CSS);

    app.run::<App>(());
}
