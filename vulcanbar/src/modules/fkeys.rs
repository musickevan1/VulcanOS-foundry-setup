//! F-Keys Module
//!
//! Displays function keys (F1-F12) with configurable labels and actions.
//! This is a port of the original tiny-dfr F-key functionality.

use anyhow::Result;
use crossbeam_channel::Sender;
use input_linux::Key;
use std::time::Duration;

use super::base::{
    draw_centered_text, draw_rounded_rect, get_background_color, CORNER_RADIUS,
};
use super::{Action, Module, ModuleEvent, RenderContext, TouchEvent};

/// Configuration for a single F-key button
#[derive(Debug, Clone)]
pub struct FKeyConfig {
    /// Display label for the key
    pub label: String,
    /// Key to emit when pressed
    pub key: Key,
    /// Stretch factor (how many slots this button occupies)
    pub stretch: usize,
}

impl Default for FKeyConfig {
    fn default() -> Self {
        Self {
            label: String::new(),
            key: Key::F1,
            stretch: 1,
        }
    }
}

/// State of a single button
struct ButtonState {
    config: FKeyConfig,
    active: bool,
    changed: bool,
    /// Start position in virtual button units
    start_pos: usize,
}

/// F-Keys module that displays function keys
pub struct FKeysModule {
    name: String,
    buttons: Vec<ButtonState>,
    /// Total number of virtual button slots
    virtual_button_count: usize,
    /// Spacing between buttons in pixels
    spacing: i32,
    /// Currently active touch slots mapped to button indices
    active_touches: std::collections::HashMap<i32, usize>,
}

impl FKeysModule {
    /// Create a new F-keys module with default F1-F12 keys
    pub fn new() -> Self {
        let keys = vec![
            ("F1", Key::F1),
            ("F2", Key::F2),
            ("F3", Key::F3),
            ("F4", Key::F4),
            ("F5", Key::F5),
            ("F6", Key::F6),
            ("F7", Key::F7),
            ("F8", Key::F8),
            ("F9", Key::F9),
            ("F10", Key::F10),
            ("F11", Key::F11),
            ("F12", Key::F12),
        ];

        Self::with_keys(
            keys.into_iter()
                .map(|(label, key)| FKeyConfig {
                    label: label.to_string(),
                    key,
                    stretch: 1,
                })
                .collect(),
        )
    }

    /// Create a new F-keys module with custom key configuration
    pub fn with_keys(configs: Vec<FKeyConfig>) -> Self {
        let mut virtual_button_count = 0;
        let buttons: Vec<ButtonState> = configs
            .into_iter()
            .map(|config| {
                let start_pos = virtual_button_count;
                virtual_button_count += config.stretch;
                ButtonState {
                    config,
                    active: false,
                    changed: true, // Initially needs drawing
                    start_pos,
                }
            })
            .collect();

        Self {
            name: "fkeys".to_string(),
            buttons,
            virtual_button_count,
            spacing: 16,
            active_touches: std::collections::HashMap::new(),
        }
    }

    /// Create from TOML configuration
    pub fn from_config(config: &toml::Value) -> Result<Box<dyn Module>> {
        let keys = if let Some(keys_array) = config.get("keys").and_then(|v| v.as_array()) {
            keys_array
                .iter()
                .filter_map(|v| {
                    let label = v.get("label")?.as_str()?;
                    let action = v.get("action")?.as_str()?;
                    let stretch = v
                        .get("stretch")
                        .and_then(|s| s.as_integer())
                        .unwrap_or(1) as usize;

                    let key = parse_key(action)?;

                    Some(FKeyConfig {
                        label: label.to_string(),
                        key,
                        stretch: stretch.max(1),
                    })
                })
                .collect()
        } else {
            // Default F1-F12
            (1..=12)
                .map(|i| FKeyConfig {
                    label: format!("F{}", i),
                    key: match i {
                        1 => Key::F1,
                        2 => Key::F2,
                        3 => Key::F3,
                        4 => Key::F4,
                        5 => Key::F5,
                        6 => Key::F6,
                        7 => Key::F7,
                        8 => Key::F8,
                        9 => Key::F9,
                        10 => Key::F10,
                        11 => Key::F11,
                        _ => Key::F12,
                    },
                    stretch: 1,
                })
                .collect()
        };

        Ok(Box::new(Self::with_keys(keys)))
    }

    /// Calculate button width and position
    fn calculate_button_geometry(&self, total_width: i32) -> (f64, Vec<(f64, f64)>) {
        let virtual_button_width = (total_width
            - (self.spacing * (self.virtual_button_count - 1) as i32)) as f64
            / self.virtual_button_count as f64;

        let positions: Vec<(f64, f64)> = self
            .buttons
            .iter()
            .enumerate()
            .map(|(i, button)| {
                let start = button.start_pos;
                let end = if i + 1 < self.buttons.len() {
                    self.buttons[i + 1].start_pos
                } else {
                    self.virtual_button_count
                };

                let left_edge =
                    (start as f64 * (virtual_button_width + self.spacing as f64)).floor();
                let button_width = virtual_button_width
                    + ((end - start - 1) as f64 * (virtual_button_width + self.spacing as f64))
                        .floor();

                (left_edge, button_width)
            })
            .collect();

        (virtual_button_width, positions)
    }

    /// Find which button (if any) is at the given coordinates
    fn hit_test(&self, x: f64, y: f64, width: i32, height: i32) -> Option<usize> {
        let (_, positions) = self.calculate_button_geometry(width);

        for (i, (left_edge, button_width)) in positions.iter().enumerate() {
            if x >= *left_edge
                && x <= left_edge + button_width
                && y >= height as f64 * 0.1
                && y <= height as f64 * 0.9
            {
                return Some(i);
            }
        }
        None
    }
}

impl Module for FKeysModule {
    fn name(&self) -> &str {
        &self.name
    }

    fn width(&self) -> i32 {
        0 // Stretch to fill available space
    }

    fn render(&self, ctx: &RenderContext) -> Result<()> {
        let (_, positions) = self.calculate_button_geometry(ctx.width);

        // Set up font
        ctx.cairo.set_source_rgb(1.0, 1.0, 1.0);
        ctx.cairo.set_font_size(32.0);

        for (i, button) in self.buttons.iter().enumerate() {
            let (left_edge, button_width) = positions[i];
            let x = ctx.x_offset + left_edge;

            // Draw background
            let color = get_background_color(button.active, ctx.show_outlines);
            draw_rounded_rect(
                ctx.cairo,
                x,
                0.0,
                button_width,
                ctx.height as f64,
                CORNER_RADIUS,
                color,
            );

            // Draw label
            ctx.cairo.set_source_rgb(1.0, 1.0, 1.0);
            draw_centered_text(
                ctx.cairo,
                &button.config.label,
                x,
                0.0,
                button_width,
                ctx.height as f64,
                ctx.y_offset,
            )?;
        }

        Ok(())
    }

    fn on_touch(&mut self, event: TouchEvent) -> Option<Action> {
        if event.pressed {
            // Touch down - find button and activate
            // Note: x is relative to module, need full width for hit test
            // This simplified version assumes full-width module
            if let Some(button_idx) = self.hit_test(event.x, event.y, 2008, 60) {
                self.active_touches.insert(event.slot, button_idx);
                self.buttons[button_idx].active = true;
                self.buttons[button_idx].changed = true;
                return Some(Action::KeyPress(self.buttons[button_idx].config.key));
            }
        } else {
            // Touch up - deactivate button
            if let Some(button_idx) = self.active_touches.remove(&event.slot) {
                if button_idx < self.buttons.len() {
                    self.buttons[button_idx].active = false;
                    self.buttons[button_idx].changed = true;
                }
            }
        }
        None
    }

    fn update_interval(&self) -> Option<Duration> {
        None // F-keys don't need polling
    }

    fn update(&mut self) -> Result<bool> {
        Ok(false) // No polling updates needed
    }

    fn start_listener(&mut self, _tx: Sender<ModuleEvent>) -> Result<Option<i32>> {
        Ok(None) // No background listener needed
    }
}

impl Default for FKeysModule {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse a key name string into a Key enum
fn parse_key(name: &str) -> Option<Key> {
    match name.to_uppercase().as_str() {
        "F1" => Some(Key::F1),
        "F2" => Some(Key::F2),
        "F3" => Some(Key::F3),
        "F4" => Some(Key::F4),
        "F5" => Some(Key::F5),
        "F6" => Some(Key::F6),
        "F7" => Some(Key::F7),
        "F8" => Some(Key::F8),
        "F9" => Some(Key::F9),
        "F10" => Some(Key::F10),
        "F11" => Some(Key::F11),
        "F12" => Some(Key::F12),
        "ESC" | "ESCAPE" => Some(Key::Esc),
        "BRIGHTNESSDOWN" => Some(Key::BrightnessDown),
        "BRIGHTNESSUP" => Some(Key::BrightnessUp),
        "MICMUTE" => Some(Key::MicMute),
        "SEARCH" => Some(Key::Search),
        "ILLUMDOWN" => Some(Key::IllumDown),
        "ILLUMUP" => Some(Key::IllumUp),
        "PREVIOUSSONG" => Some(Key::PreviousSong),
        "PLAYPAUSE" => Some(Key::PlayPause),
        "NEXTSONG" => Some(Key::NextSong),
        "MUTE" => Some(Key::Mute),
        "VOLUMEDOWN" => Some(Key::VolumeDown),
        "VOLUMEUP" => Some(Key::VolumeUp),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_fkeys() {
        let module = FKeysModule::new();
        assert_eq!(module.buttons.len(), 12);
        assert_eq!(module.virtual_button_count, 12);
    }

    #[test]
    fn test_parse_key() {
        assert_eq!(parse_key("F1"), Some(Key::F1));
        assert_eq!(parse_key("f12"), Some(Key::F12));
        assert_eq!(parse_key("ESC"), Some(Key::Esc));
        assert_eq!(parse_key("invalid"), None);
    }
}
