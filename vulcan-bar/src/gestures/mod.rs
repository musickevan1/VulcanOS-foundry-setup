//! Gesture detection system for VulcanBar
//!
//! Detects tap, swipe, and long-press gestures from raw touch events.
//! Used for page switching (horizontal swipe) and module interactions (tap).

mod recognizer;

pub use recognizer::{GestureDetector, GestureConfig, Gesture, Direction, TouchPhase};
