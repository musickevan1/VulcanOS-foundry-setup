//! Gesture recognition from touch events
//!
//! Tracks touch points and calculates gestures based on:
//! - Movement distance (tap vs swipe)
//! - Velocity (fast swipe vs slow drag)
//! - Duration (tap vs long-press)

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Direction of a swipe gesture
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

/// Phase of a touch event
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchPhase {
    Start,
    Move,
    End,
    Cancel,
}

/// A recognized gesture
#[derive(Debug, Clone)]
pub enum Gesture {
    /// A quick tap at a position
    Tap {
        x: f64,
        y: f64,
    },
    /// A horizontal swipe gesture
    Swipe {
        direction: Direction,
        velocity: f64,
        start_x: f64,
        end_x: f64,
    },
    /// A long press (held without moving)
    LongPress {
        x: f64,
        y: f64,
    },
}

/// Configuration for gesture detection thresholds
#[derive(Debug, Clone)]
pub struct GestureConfig {
    /// Minimum horizontal distance (pixels) to trigger a swipe
    pub swipe_threshold_px: f64,
    /// Minimum velocity (pixels/second) to trigger a swipe
    pub swipe_velocity_threshold: f64,
    /// Duration (ms) to trigger a long press
    pub long_press_duration_ms: u64,
    /// Maximum movement (pixels) allowed for a tap
    pub tap_max_distance_px: f64,
    /// Maximum duration (ms) for a tap
    pub tap_max_duration_ms: u64,
}

impl Default for GestureConfig {
    fn default() -> Self {
        Self {
            swipe_threshold_px: 80.0,        // Increased: need larger movement for swipe
            swipe_velocity_threshold: 200.0, // Increased: need faster movement for swipe
            long_press_duration_ms: 500,
            tap_max_distance_px: 40.0,       // Increased: Touch Bar is very sensitive
            tap_max_duration_ms: 400,        // Increased: allow slightly longer taps
        }
    }
}

/// Tracking state for a single touch point
#[derive(Debug, Clone)]
struct TouchTrack {
    /// Touch slot identifier
    slot: i32,
    /// Starting position
    start_pos: (f64, f64),
    /// Current/last position
    current_pos: (f64, f64),
    /// When the touch started
    start_time: Instant,
    /// Last update time (for velocity calculation)
    last_update: Instant,
    /// Whether long press has been triggered
    long_press_fired: bool,
}

impl TouchTrack {
    fn new(slot: i32, x: f64, y: f64) -> Self {
        let now = Instant::now();
        Self {
            slot,
            start_pos: (x, y),
            current_pos: (x, y),
            start_time: now,
            last_update: now,
            long_press_fired: false,
        }
    }

    /// Total horizontal distance moved
    fn delta_x(&self) -> f64 {
        self.current_pos.0 - self.start_pos.0
    }

    /// Total vertical distance moved
    fn delta_y(&self) -> f64 {
        self.current_pos.1 - self.start_pos.1
    }

    /// Total distance moved (Euclidean)
    fn total_distance(&self) -> f64 {
        let dx = self.delta_x();
        let dy = self.delta_y();
        (dx * dx + dy * dy).sqrt()
    }

    /// Duration of the touch
    fn duration(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Horizontal velocity in pixels per second
    fn velocity_x(&self) -> f64 {
        let duration_secs = self.duration().as_secs_f64();
        if duration_secs > 0.0 {
            self.delta_x().abs() / duration_secs
        } else {
            0.0
        }
    }
}

/// Gesture detector that processes touch events
#[derive(Debug)]
pub struct GestureDetector {
    /// Configuration thresholds
    config: GestureConfig,
    /// Active touch tracks by slot
    touches: HashMap<i32, TouchTrack>,
    /// Pending gesture to emit (set during motion, emitted on end)
    pending_gesture: Option<Gesture>,
}

impl GestureDetector {
    /// Create a new gesture detector with default configuration
    pub fn new() -> Self {
        Self::with_config(GestureConfig::default())
    }

    /// Create a new gesture detector with custom configuration
    pub fn with_config(config: GestureConfig) -> Self {
        Self {
            config,
            touches: HashMap::new(),
            pending_gesture: None,
        }
    }

    /// Update configuration (e.g., from hot-reload)
    pub fn set_config(&mut self, config: GestureConfig) {
        self.config = config;
    }

    /// Get current configuration
    pub fn config(&self) -> &GestureConfig {
        &self.config
    }

    /// Process a touch event and potentially return a gesture
    ///
    /// Returns `Some(Gesture)` when a gesture is recognized, `None` otherwise.
    pub fn process_touch(
        &mut self,
        phase: TouchPhase,
        slot: i32,
        x: f64,
        y: f64,
    ) -> Option<Gesture> {
        match phase {
            TouchPhase::Start => {
                self.on_touch_start(slot, x, y);
                None
            }
            TouchPhase::Move => {
                self.on_touch_move(slot, x, y)
            }
            TouchPhase::End => {
                self.on_touch_end(slot, x, y)
            }
            TouchPhase::Cancel => {
                self.touches.remove(&slot);
                self.pending_gesture = None;
                None
            }
        }
    }

    /// Check for long press on active touches (call periodically)
    ///
    /// Returns `Some(Gesture::LongPress)` if a touch has been held long enough.
    pub fn check_long_press(&mut self) -> Option<Gesture> {
        let config = &self.config;
        let long_press_duration = Duration::from_millis(config.long_press_duration_ms);

        for track in self.touches.values_mut() {
            if track.long_press_fired {
                continue;
            }

            // Check if held long enough without moving
            if track.duration() >= long_press_duration
                && track.total_distance() < config.tap_max_distance_px
            {
                track.long_press_fired = true;
                return Some(Gesture::LongPress {
                    x: track.start_pos.0,
                    y: track.start_pos.1,
                });
            }
        }
        None
    }

    /// Clear all tracking state
    pub fn reset(&mut self) {
        self.touches.clear();
        self.pending_gesture = None;
    }

    /// Check if any touches are active
    pub fn has_active_touch(&self) -> bool {
        !self.touches.is_empty()
    }

    /// Get the start position of the first active touch (if any)
    pub fn active_touch_start(&self) -> Option<(f64, f64)> {
        self.touches.values().next().map(|t| t.start_pos)
    }

    fn on_touch_start(&mut self, slot: i32, x: f64, y: f64) {
        let track = TouchTrack::new(slot, x, y);
        self.touches.insert(slot, track);
        self.pending_gesture = None;
    }

    fn on_touch_move(&mut self, slot: i32, x: f64, y: f64) -> Option<Gesture> {
        if let Some(track) = self.touches.get_mut(&slot) {
            track.current_pos = (x, y);
            track.last_update = Instant::now();

            // Check for swipe during motion (for early detection)
            let dx = track.delta_x();
            let dy = track.delta_y().abs();

            // Horizontal swipe: significant horizontal movement, limited vertical
            if dx.abs() > self.config.swipe_threshold_px && dx.abs() > dy * 2.0 {
                let velocity = track.velocity_x();
                if velocity >= self.config.swipe_velocity_threshold {
                    let direction = if dx > 0.0 {
                        Direction::Right
                    } else {
                        Direction::Left
                    };

                    // Store pending gesture but don't emit yet
                    // (user might change direction)
                    self.pending_gesture = Some(Gesture::Swipe {
                        direction,
                        velocity,
                        start_x: track.start_pos.0,
                        end_x: track.current_pos.0,
                    });
                }
            }
        }
        None // Don't emit during motion - wait for end
    }

    fn on_touch_end(&mut self, slot: i32, x: f64, y: f64) -> Option<Gesture> {
        let track = self.touches.remove(&slot)?;

        // Update final position if provided
        let final_pos = if x != 0.0 || y != 0.0 {
            (x, y)
        } else {
            track.current_pos
        };

        // Calculate final metrics
        let dx = final_pos.0 - track.start_pos.0;
        let dy = (final_pos.1 - track.start_pos.1).abs();
        let distance = ((dx * dx) + (dy * dy)).sqrt();
        let duration = track.duration();
        let velocity_x = if duration.as_secs_f64() > 0.0 {
            dx.abs() / duration.as_secs_f64()
        } else {
            0.0
        };

        // If long press was fired, don't emit another gesture
        if track.long_press_fired {
            return None;
        }

        // Priority 1: Check for swipe
        if dx.abs() > self.config.swipe_threshold_px
            && dx.abs() > dy * 2.0  // More horizontal than vertical
            && velocity_x >= self.config.swipe_velocity_threshold
        {
            let direction = if dx > 0.0 {
                Direction::Right
            } else {
                Direction::Left
            };

            return Some(Gesture::Swipe {
                direction,
                velocity: velocity_x,
                start_x: track.start_pos.0,
                end_x: final_pos.0,
            });
        }

        // Priority 2: Check for tap
        if distance < self.config.tap_max_distance_px
            && duration < Duration::from_millis(self.config.tap_max_duration_ms)
        {
            return Some(Gesture::Tap {
                x: track.start_pos.0,
                y: track.start_pos.1,
            });
        }

        // No recognized gesture
        None
    }
}

impl Default for GestureDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tap_detection() {
        let mut detector = GestureDetector::new();

        // Simulate a quick tap
        assert!(detector.process_touch(TouchPhase::Start, 0, 100.0, 30.0).is_none());

        // Small movement within threshold
        assert!(detector.process_touch(TouchPhase::Move, 0, 102.0, 31.0).is_none());

        // Release quickly
        std::thread::sleep(Duration::from_millis(50));
        let gesture = detector.process_touch(TouchPhase::End, 0, 102.0, 31.0);

        assert!(matches!(gesture, Some(Gesture::Tap { .. })));
    }

    #[test]
    fn test_swipe_detection() {
        let mut detector = GestureDetector::new();

        // Start touch
        detector.process_touch(TouchPhase::Start, 0, 100.0, 30.0);

        // Move significantly to the right
        detector.process_touch(TouchPhase::Move, 0, 200.0, 32.0);

        // Release
        std::thread::sleep(Duration::from_millis(100));
        let gesture = detector.process_touch(TouchPhase::End, 0, 250.0, 32.0);

        match gesture {
            Some(Gesture::Swipe { direction, .. }) => {
                assert_eq!(direction, Direction::Right);
            }
            _ => panic!("Expected swipe gesture"),
        }
    }

    #[test]
    fn test_swipe_left() {
        let mut detector = GestureDetector::new();

        detector.process_touch(TouchPhase::Start, 0, 200.0, 30.0);
        detector.process_touch(TouchPhase::Move, 0, 100.0, 32.0);

        std::thread::sleep(Duration::from_millis(100));
        let gesture = detector.process_touch(TouchPhase::End, 0, 50.0, 32.0);

        match gesture {
            Some(Gesture::Swipe { direction, .. }) => {
                assert_eq!(direction, Direction::Left);
            }
            _ => panic!("Expected swipe left gesture"),
        }
    }
}
