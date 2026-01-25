//! Volume Module
//!
//! Displays and controls audio volume via PulseAudio/PipeWire.

use anyhow::{Context, Result};
use crossbeam_channel::Sender;
use libpulse_binding::{
    callbacks::ListResult,
    context::{Context as PulseContext, FlagSet as ContextFlagSet, State as ContextState},
    mainloop::standard::{IterateResult, Mainloop},
    proplist::Proplist,
    volume::Volume,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::base::{draw_centered_text, draw_rounded_rect, get_background_color, CORNER_RADIUS};
use super::{Action, Module, ModuleEvent, RenderContext, TouchEvent};
use crate::config::VolumeConfig;

/// Shared volume state
#[derive(Debug, Clone)]
struct VolumeState {
    volume_percent: u32,
    muted: bool,
}

/// Volume module that displays and controls audio volume
pub struct VolumeModule {
    name: String,
    config: VolumeConfig,
    /// Current volume state
    state: Arc<Mutex<VolumeState>>,
    /// Whether currently being touched
    active: bool,
}

impl VolumeModule {
    /// Create a new volume module with default settings
    pub fn new() -> Self {
        Self::with_config(VolumeConfig::default())
    }

    /// Create a volume module with specific configuration
    pub fn with_config(config: VolumeConfig) -> Self {
        let state = Arc::new(Mutex::new(VolumeState {
            volume_percent: 100,
            muted: false,
        }));

        let mut module = Self {
            name: "volume".to_string(),
            config,
            state,
            active: false,
        };

        // Try to get initial volume state
        let _ = module.update();
        module
    }

    /// Create from TOML configuration
    pub fn from_config(config: &toml::Value) -> Result<Box<dyn Module>> {
        let volume_config: VolumeConfig = config.clone().try_into().unwrap_or_default();
        Ok(Box::new(Self::with_config(volume_config)))
    }

    /// Get volume icon based on level and mute state
    fn get_volume_icon(&self) -> &'static str {
        let state = self.state.lock().unwrap();
        if state.muted || state.volume_percent == 0 {
            "󰝟" // muted
        } else if state.volume_percent < 30 {
            "󰕿" // low
        } else if state.volume_percent < 70 {
            "󰖀" // medium
        } else {
            "󰕾" // high
        }
    }

    /// Get display text based on config
    fn get_display_text(&self) -> String {
        let state = self.state.lock().unwrap();
        match self.config.display.as_str() {
            "icon" => self.get_volume_icon().to_string(),
            "percentage" => {
                if state.muted {
                    "MUTE".to_string()
                } else {
                    format!("{}%", state.volume_percent)
                }
            }
            "both" | _ => {
                if state.muted {
                    format!("{} MUTE", self.get_volume_icon())
                } else {
                    format!("{} {}%", self.get_volume_icon(), state.volume_percent)
                }
            }
        }
    }

    /// Query current volume from PulseAudio
    fn query_volume(&self) -> Result<()> {
        let state = Arc::clone(&self.state);

        // Create a new mainloop and context for querying
        let mainloop = Rc::new(RefCell::new(
            Mainloop::new().context("Failed to create PulseAudio mainloop")?,
        ));

        let mut proplist = Proplist::new().context("Failed to create proplist")?;
        proplist
            .set_str(
                libpulse_binding::proplist::properties::APPLICATION_NAME,
                "vulcanbar",
            )
            .ok();

        let context = Rc::new(RefCell::new(
            PulseContext::new_with_proplist(&*mainloop.borrow(), "vulcanbar", &proplist)
                .context("Failed to create PulseAudio context")?,
        ));

        context
            .borrow_mut()
            .connect(None, ContextFlagSet::NOFLAGS, None)
            .context("Failed to connect to PulseAudio")?;

        // Wait for connection
        loop {
            match mainloop.borrow_mut().iterate(true) {
                IterateResult::Success(_) => {}
                IterateResult::Quit(_) | IterateResult::Err(_) => {
                    return Err(anyhow::anyhow!("PulseAudio mainloop error"));
                }
            }

            match context.borrow().get_state() {
                ContextState::Ready => break,
                ContextState::Failed | ContextState::Terminated => {
                    return Err(anyhow::anyhow!("PulseAudio connection failed"));
                }
                _ => {}
            }
        }

        // Query default sink
        let state_clone = Arc::clone(&state);
        let introspector = context.borrow().introspect();
        let done = Rc::new(RefCell::new(false));
        let done_clone = Rc::clone(&done);

        introspector.get_sink_info_by_name("@DEFAULT_SINK@", move |result| {
            if let ListResult::Item(sink) = result {
                let volume = sink.volume.avg();
                let percent = (volume.0 as f64 / Volume::NORMAL.0 as f64 * 100.0).round() as u32;

                if let Ok(mut state) = state_clone.lock() {
                    state.volume_percent = percent.min(150); // Cap at 150%
                    state.muted = sink.mute;
                }
            }
            *done_clone.borrow_mut() = true;
        });

        // Wait for result
        while !*done.borrow() {
            match mainloop.borrow_mut().iterate(true) {
                IterateResult::Success(_) => {}
                _ => break,
            }
        }

        Ok(())
    }

    /// Toggle mute via PulseAudio
    fn toggle_mute(&self) -> Result<()> {
        let mainloop = Rc::new(RefCell::new(
            Mainloop::new().context("Failed to create PulseAudio mainloop")?,
        ));

        let mut proplist = Proplist::new().context("Failed to create proplist")?;
        proplist
            .set_str(
                libpulse_binding::proplist::properties::APPLICATION_NAME,
                "vulcanbar",
            )
            .ok();

        let context = Rc::new(RefCell::new(
            PulseContext::new_with_proplist(&*mainloop.borrow(), "vulcanbar", &proplist)
                .context("Failed to create PulseAudio context")?,
        ));

        context
            .borrow_mut()
            .connect(None, ContextFlagSet::NOFLAGS, None)
            .context("Failed to connect to PulseAudio")?;

        // Wait for connection
        loop {
            match mainloop.borrow_mut().iterate(true) {
                IterateResult::Success(_) => {}
                _ => return Err(anyhow::anyhow!("PulseAudio mainloop error")),
            }

            match context.borrow().get_state() {
                ContextState::Ready => break,
                ContextState::Failed | ContextState::Terminated => {
                    return Err(anyhow::anyhow!("PulseAudio connection failed"));
                }
                _ => {}
            }
        }

        // Toggle mute on default sink
        let current_mute = self.state.lock().unwrap().muted;
        let mut introspector = context.borrow().introspect();
        introspector.set_sink_mute_by_name("@DEFAULT_SINK@", !current_mute, None);

        // Process the command
        mainloop.borrow_mut().iterate(true);

        Ok(())
    }
}

impl Module for VolumeModule {
    fn name(&self) -> &str {
        &self.name
    }

    fn width(&self) -> i32 {
        match self.config.display.as_str() {
            "icon" => 60,
            "percentage" => 80,
            _ => 120,
        }
    }

    fn render(&self, ctx: &RenderContext) -> Result<()> {
        // Draw background
        let color = get_background_color(self.active, ctx.show_outlines);
        draw_rounded_rect(
            ctx.cairo,
            ctx.x_offset,
            0.0,
            ctx.width as f64,
            ctx.height as f64,
            CORNER_RADIUS,
            color,
        );

        // Draw volume text/icon
        ctx.cairo.set_source_rgb(1.0, 1.0, 1.0);
        let text = self.get_display_text();
        draw_centered_text(
            ctx.cairo,
            &text,
            ctx.x_offset,
            0.0,
            ctx.width as f64,
            ctx.height as f64,
            ctx.y_offset,
        )?;

        Ok(())
    }

    fn on_touch(&mut self, event: TouchEvent) -> Option<Action> {
        self.active = event.pressed;

        if !event.pressed {
            // On release, perform action
            match self.config.on_click.as_str() {
                "toggle-mute" => {
                    let _ = self.toggle_mute();
                    let _ = self.update(); // Refresh state
                    return Some(Action::ToggleMute);
                }
                _ => {}
            }
        }
        None
    }

    fn update_interval(&self) -> Option<Duration> {
        // Poll periodically to catch external volume changes
        Some(Duration::from_secs(2))
    }

    fn update(&mut self) -> Result<bool> {
        let old_state = {
            let s = self.state.lock().unwrap();
            (s.volume_percent, s.muted)
        };

        self.query_volume()?;

        let new_state = {
            let s = self.state.lock().unwrap();
            (s.volume_percent, s.muted)
        };

        Ok(old_state != new_state)
    }

    fn start_listener(&mut self, _tx: Sender<ModuleEvent>) -> Result<Option<i32>> {
        // For now, use polling. Event-based listening would require
        // running the PulseAudio mainloop in a separate thread.
        Ok(None)
    }
}

impl Default for VolumeModule {
    fn default() -> Self {
        Self::new()
    }
}
