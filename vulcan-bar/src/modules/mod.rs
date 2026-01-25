//! Module system for VulcanBar
//!
//! This module provides the trait and infrastructure for creating
//! Touch Bar modules similar to Waybar's module system.

pub mod base;
pub mod battery;
pub mod brightness;
pub mod clock;
pub mod fkeys;
pub mod volume;
pub mod workspaces;

// New Waybar-style modules
pub mod audio;
pub mod bluetooth;
pub mod cpu;
pub mod home;
pub mod media;
pub mod memory;
pub mod network;
pub mod tray;
pub mod window_title;

use anyhow::Result;
use cairo::Context;
use crossbeam_channel::Sender;
use std::collections::HashMap;
use std::time::Duration;

/// Events that modules can emit to trigger updates
#[derive(Debug, Clone)]
pub enum ModuleEvent {
    /// Module content has changed and needs redraw
    NeedsRedraw(String),
    /// Module requests an action to be executed
    Action(Action),
}

/// Actions that can be triggered by module interactions
#[derive(Debug, Clone)]
pub enum Action {
    /// Emit a key press/release
    KeyPress(input_linux::Key),
    /// Execute a shell command
    Command(String),
    /// Switch to a specific Hyprland workspace
    Workspace(i32),
    /// Toggle mute
    ToggleMute,
    /// Switch to a specific page (for tap-to-expand)
    SwitchToPage(String),
    /// No action
    None,
}

/// Touch event information passed to modules
#[derive(Debug, Clone, Copy)]
pub struct TouchEvent {
    /// X coordinate relative to module's left edge
    pub x: f64,
    /// Y coordinate relative to module's top edge
    pub y: f64,
    /// Whether this is a press (true) or release (false)
    pub pressed: bool,
    /// Touch slot for multi-touch tracking
    pub slot: i32,
}

/// Context provided to modules for rendering
pub struct RenderContext<'a> {
    /// Cairo context for drawing
    pub cairo: &'a Context,
    /// Available width for this module
    pub width: i32,
    /// Available height for this module
    pub height: i32,
    /// X offset where module should start drawing
    pub x_offset: f64,
    /// Y offset for pixel shift
    pub y_offset: f64,
    /// Whether button outlines should be shown
    pub show_outlines: bool,
    /// Whether the module is currently being touched
    pub is_active: bool,
}

/// The core trait that all Touch Bar modules must implement
pub trait Module: Send {
    /// Returns the unique name/identifier of this module
    fn name(&self) -> &str;

    /// Returns the preferred width of this module in pixels
    /// Return 0 for modules that should stretch to fill available space
    fn width(&self) -> i32;

    /// Render the module's content
    fn render(&self, ctx: &RenderContext) -> Result<()>;

    /// Handle a touch event on this module
    /// Returns an optional action to execute
    fn on_touch(&mut self, event: TouchEvent) -> Option<Action>;

    /// Returns how often this module should be polled for updates
    /// Return None for event-driven modules that don't need polling
    fn update_interval(&self) -> Option<Duration>;

    /// Poll for updates (called based on update_interval)
    /// Returns true if the module content changed and needs redraw
    fn update(&mut self) -> Result<bool>;

    /// Start any background listeners (e.g., IPC connections)
    /// Returns an optional file descriptor for epoll integration
    fn start_listener(&mut self, _tx: Sender<ModuleEvent>) -> Result<Option<i32>> {
        Ok(None)
    }

    /// Called when the module is being removed/cleaned up
    fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }
}

/// Factory function type for creating modules
pub type ModuleFactory = fn(config: &toml::Value) -> Result<Box<dyn Module>>;

/// Registry for module factories
pub struct ModuleRegistry {
    factories: HashMap<String, ModuleFactory>,
}

impl ModuleRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
        }
    }

    /// Create a registry with all built-in modules registered
    pub fn with_builtins() -> Self {
        let mut registry = Self::new();

        // Core polling modules
        registry.register("fkeys", fkeys::FKeysModule::from_config);
        registry.register("clock", clock::ClockModule::from_config);
        registry.register("battery", battery::BatteryModule::from_config);
        registry.register("brightness", brightness::BrightnessModule::from_config);

        // Event-driven modules
        registry.register("volume", volume::VolumeModule::from_config);
        registry.register("workspaces", workspaces::WorkspacesModule::from_config);

        // Waybar-style modules
        registry.register("cpu", cpu::CpuModule::from_config);
        registry.register("memory", memory::MemoryModule::from_config);
        registry.register("network", network::NetworkModule::from_config);
        registry.register("bluetooth", bluetooth::BluetoothModule::from_config);
        registry.register("audio", audio::AudioModule::from_config);
        registry.register("tray", tray::TrayModule::from_config);
        registry.register("window-title", window_title::WindowTitleModule::from_config);
        registry.register("media", media::MediaModule::from_config);
        registry.register("home", home::HomeModule::from_config);

        registry
    }

    /// Register a module factory
    pub fn register(&mut self, name: &str, factory: ModuleFactory) {
        self.factories.insert(name.to_string(), factory);
    }

    /// Create a module instance by name
    pub fn create(&self, name: &str, config: &toml::Value) -> Result<Box<dyn Module>> {
        let factory = self
            .factories
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("Unknown module type: {}", name))?;
        factory(config)
    }

    /// Get list of registered module names
    pub fn available_modules(&self) -> Vec<&str> {
        self.factories.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for ModuleRegistry {
    fn default() -> Self {
        Self::with_builtins()
    }
}
