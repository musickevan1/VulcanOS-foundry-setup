//! Configuration system for VulcanBar
//!
//! Provides both the legacy tiny-dfr configuration (for backward compatibility)
//! and the new Waybar-style configuration system.

// Legacy loader (tiny-dfr compatibility)
pub mod loader;

// New VulcanBar configuration system
pub mod schema;
pub mod new_loader;

// Re-export legacy types for backward compatibility
pub use loader::{Config, ConfigManager, ButtonConfig};

// Re-export new configuration types
pub use schema::{
    VulcanBarConfig, GeneralConfig, LayoutConfig,
    ClockConfig, BatteryConfig, VolumeConfig, BrightnessConfig,
    WorkspacesConfig, FKeysConfig, FKeyDefinition,
    PagesConfig, PageDefinition, GesturesConfig,
};
pub use new_loader::ConfigLoader;
