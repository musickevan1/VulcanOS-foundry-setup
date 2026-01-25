//! Workspaces Module
//!
//! Displays Hyprland workspaces with touch switching support.

use anyhow::{Context, Result};
use crossbeam_channel::Sender;
use hyprland::data::{Workspace, Workspaces};
use hyprland::dispatch::{Dispatch, DispatchType, WorkspaceIdentifierWithSpecial};
use hyprland::shared::{HyprData, HyprDataActive};
use std::time::Duration;

use super::base::{draw_rounded_rect, get_background_color, CORNER_RADIUS};
use super::{Action, Module, ModuleEvent, RenderContext, TouchEvent};
use crate::config::WorkspacesConfig;

/// Single workspace button state
#[derive(Debug, Clone)]
struct WorkspaceButton {
    id: i32,
    active: bool,
    occupied: bool,
}

/// Workspaces module for Hyprland
pub struct WorkspacesModule {
    name: String,
    config: WorkspacesConfig,
    /// Current workspace buttons
    buttons: Vec<WorkspaceButton>,
    /// Currently active workspace ID
    active_workspace: i32,
    /// Button being touched (-1 if none)
    touched_button: i32,
    /// Spacing between workspace buttons
    button_spacing: i32,
}

impl WorkspacesModule {
    /// Create a new workspaces module with default settings
    pub fn new() -> Self {
        Self::with_config(WorkspacesConfig::default())
    }

    /// Create a workspaces module with specific configuration
    pub fn with_config(config: WorkspacesConfig) -> Self {
        let mut module = Self {
            name: "workspaces".to_string(),
            buttons: Vec::new(),
            active_workspace: 1,
            touched_button: -1,
            button_spacing: 8,
            config,
        };

        // Initialize with default buttons if Hyprland isn't available
        module.init_default_buttons();

        // Try to get actual state (may fail if Hyprland not running)
        let _ = module.update();
        module
    }

    /// Initialize default workspace buttons (used when Hyprland unavailable)
    fn init_default_buttons(&mut self) {
        self.buttons.clear();
        for i in 1..=self.config.persistent_workspaces as i32 {
            self.buttons.push(WorkspaceButton {
                id: i,
                active: i == 1,
                occupied: i == 1,
            });
        }
    }

    /// Check if Hyprland is available
    fn hyprland_available() -> bool {
        std::env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok()
    }

    /// Create from TOML configuration
    pub fn from_config(config: &toml::Value) -> Result<Box<dyn Module>> {
        let ws_config: WorkspacesConfig = config.clone().try_into().unwrap_or_default();
        Ok(Box::new(Self::with_config(ws_config)))
    }

    /// Query current workspace state from Hyprland
    fn query_workspaces(&mut self) -> Result<()> {
        // Check if Hyprland is available
        if !Self::hyprland_available() {
            return Ok(()); // Keep default buttons
        }

        // Use std::panic::catch_unwind to handle hyprland crate panics
        let result = std::panic::catch_unwind(|| {
            let active = Workspace::get_active().ok()?;
            let workspaces = Workspaces::get().ok()?;
            Some((active, workspaces))
        });

        let (active, workspaces) = match result {
            Ok(Some((a, w))) => (a, w),
            _ => return Ok(()), // Hyprland not available or error - keep existing buttons
        };

        self.active_workspace = active.id;
        let occupied: Vec<i32> = workspaces.iter().map(|w| w.id).collect();

        // Build button list
        self.buttons.clear();

        if self.config.active_only {
            // Only show occupied workspaces
            for ws in workspaces.iter() {
                if ws.id > 0 {
                    // Skip special workspaces
                    self.buttons.push(WorkspaceButton {
                        id: ws.id,
                        active: ws.id == self.active_workspace,
                        occupied: true,
                    });
                }
            }
            self.buttons.sort_by_key(|b| b.id);
        } else {
            // Show persistent workspaces
            for i in 1..=self.config.persistent_workspaces as i32 {
                self.buttons.push(WorkspaceButton {
                    id: i,
                    active: i == self.active_workspace,
                    occupied: occupied.contains(&i),
                });
            }
        }

        Ok(())
    }

    /// Switch to a workspace
    fn switch_workspace(&self, workspace_id: i32) -> Result<()> {
        if !Self::hyprland_available() {
            return Ok(()); // Can't switch if Hyprland not running
        }

        // Catch potential panics from hyprland crate
        let result = std::panic::catch_unwind(|| {
            Dispatch::call(DispatchType::Workspace(
                WorkspaceIdentifierWithSpecial::Id(workspace_id),
            ))
        });

        match result {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(anyhow::anyhow!("Failed to switch workspace: {}", e)),
            Err(_) => Err(anyhow::anyhow!("Hyprland IPC panic")),
        }
    }

    /// Get button width
    fn button_width(&self) -> i32 {
        40 // Fixed width per workspace button
    }

    /// Hit test to find which button was touched
    fn hit_test(&self, x: f64, _y: f64) -> Option<i32> {
        let button_width = self.button_width();
        let total_spacing = self.button_spacing * (self.buttons.len() as i32 - 1).max(0);
        let total_width = button_width * self.buttons.len() as i32 + total_spacing;

        // Calculate starting x (centered)
        let mut current_x = 0.0;

        for button in &self.buttons {
            if x >= current_x && x < current_x + button_width as f64 {
                return Some(button.id);
            }
            current_x += (button_width + self.button_spacing) as f64;
        }

        None
    }
}

impl Module for WorkspacesModule {
    fn name(&self) -> &str {
        &self.name
    }

    fn width(&self) -> i32 {
        let button_width = self.button_width();
        let total_spacing = self.button_spacing * (self.buttons.len() as i32 - 1).max(0);
        button_width * self.buttons.len() as i32 + total_spacing
    }

    fn render(&self, ctx: &RenderContext) -> Result<()> {
        let button_width = self.button_width();
        let mut x = ctx.x_offset;

        for button in &self.buttons {
            // Determine color based on state
            let is_touched = self.touched_button == button.id;
            let color = if button.active {
                (0.3, 0.3, 0.5) // Active workspace - blueish
            } else if is_touched {
                (0.4, 0.4, 0.4) // Being touched
            } else if button.occupied {
                get_background_color(false, ctx.show_outlines)
            } else {
                (0.1, 0.1, 0.1) // Empty workspace - darker
            };

            // Draw button background
            draw_rounded_rect(
                ctx.cairo,
                x,
                0.0,
                button_width as f64,
                ctx.height as f64,
                CORNER_RADIUS / 2.0,
                color,
            );

            // Draw workspace number
            ctx.cairo.set_source_rgb(1.0, 1.0, 1.0);
            let label = format!("{}", button.id);
            let extents = ctx.cairo.text_extents(&label)?;
            ctx.cairo.move_to(
                x + (button_width as f64 / 2.0 - extents.width() / 2.0),
                ctx.y_offset + (ctx.height as f64 / 2.0 + extents.height() / 2.0),
            );
            ctx.cairo.show_text(&label)?;

            // Draw indicator dot for occupied workspaces
            if button.occupied && !button.active {
                ctx.cairo.set_source_rgb(0.6, 0.6, 0.6);
                let dot_y = ctx.height as f64 * 0.8;
                ctx.cairo.arc(
                    x + button_width as f64 / 2.0,
                    dot_y,
                    2.0,
                    0.0,
                    2.0 * std::f64::consts::PI,
                );
                ctx.cairo.fill()?;
            }

            x += (button_width + self.button_spacing) as f64;
        }

        Ok(())
    }

    fn on_touch(&mut self, event: TouchEvent) -> Option<Action> {
        if event.pressed {
            // Find which button was touched
            if let Some(workspace_id) = self.hit_test(event.x, event.y) {
                self.touched_button = workspace_id;
            }
        } else {
            // On release, switch to workspace
            if self.touched_button > 0 {
                let ws_id = self.touched_button;
                self.touched_button = -1;

                if let Err(e) = self.switch_workspace(ws_id) {
                    eprintln!("Failed to switch workspace: {}", e);
                }
                return Some(Action::Workspace(ws_id));
            }
        }
        None
    }

    fn update_interval(&self) -> Option<Duration> {
        // Poll for workspace changes (reduced frequency to avoid IPC overload)
        // TODO: Use Hyprland event listener for real-time updates
        Some(Duration::from_millis(1000))
    }

    fn update(&mut self) -> Result<bool> {
        let old_active = self.active_workspace;
        let old_count = self.buttons.len();

        self.query_workspaces()?;

        Ok(old_active != self.active_workspace || old_count != self.buttons.len())
    }

    fn start_listener(&mut self, _tx: Sender<ModuleEvent>) -> Result<Option<i32>> {
        // TODO: Implement Hyprland event listener for real-time updates
        // For now, use polling via update_interval
        Ok(None)
    }
}

impl Default for WorkspacesModule {
    fn default() -> Self {
        Self::new()
    }
}
