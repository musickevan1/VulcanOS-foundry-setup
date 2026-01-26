pub mod monitor_layout;
pub mod profile_manager;
pub mod split_dialog;
pub mod wallpaper_picker;
pub mod wallpaper_view;

// Theme components (migrated from vulcan-theme-manager)
pub mod theme_card;
pub mod theme_browser;
pub mod preview_panel;
pub mod theme_editor;
pub mod theme_view;
pub mod binding_dialog;

// Re-exports
pub use binding_dialog::{BindingDialogModel, BindingDialogOutput, BindingDialogInit};

// Profile components
pub mod profile_card;
