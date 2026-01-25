//! Terminal User Interface for vulcan-vault
//!
//! Provides a visual interface for browsing notes, searching content,
//! and managing agent memories.

#[cfg(feature = "tui")]
mod app;
#[cfg(feature = "tui")]
mod tui;

#[cfg(feature = "tui")]
pub use app::App;
#[cfg(feature = "tui")]
pub use tui::run_tui;
