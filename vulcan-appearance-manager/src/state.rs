use std::collections::HashMap;
use std::path::PathBuf;
use anyhow::{bail, Result};

/// Snapshot of state before preview, used for revert
#[derive(Debug, Clone, PartialEq)]
pub struct PreviewSnapshot {
    /// Monitor -> wallpaper path mapping before preview started
    pub wallpapers: HashMap<String, PathBuf>,
    /// Theme ID that was active before preview (if any)
    pub theme_id: Option<String>,
}

/// Application state machine with explicit transitions.
/// Invalid transitions return Result::Err.
#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    /// No preview active, showing live system state
    Idle,
    /// User is previewing a change but hasn't applied
    Previewing {
        /// State before preview started (for revert)
        previous: PreviewSnapshot,
    },
    /// Currently applying changes to live system
    Applying,
    /// An error occurred during apply/preview
    Error {
        /// Human-readable error description
        message: String,
        /// State to return to after acknowledging error
        recovery: Box<AppState>,
    },
}

impl AppState {
    /// Start a preview session. Only valid from Idle state.
    pub fn start_preview(self, snapshot: PreviewSnapshot) -> Result<AppState> {
        match self {
            AppState::Idle => Ok(AppState::Previewing { previous: snapshot }),
            AppState::Previewing { .. } => {
                bail!("Cannot start preview from state: Previewing (must be Idle)")
            }
            AppState::Applying => {
                bail!("Cannot start preview from state: Applying (must be Idle)")
            }
            AppState::Error { .. } => {
                bail!("Cannot start preview from state: Error (must be Idle)")
            }
        }
    }

    /// Start applying changes to the live system. Valid from Idle or Previewing.
    pub fn start_apply(self) -> Result<AppState> {
        match self {
            AppState::Idle | AppState::Previewing { .. } => Ok(AppState::Applying),
            AppState::Applying => {
                bail!("Cannot start apply from state: Applying (already applying)")
            }
            AppState::Error { .. } => {
                bail!("Cannot start apply from state: Error (must recover first)")
            }
        }
    }

    /// Finish an operation and return to Idle. Valid from Previewing or Applying.
    pub fn finish(self) -> Result<AppState> {
        match self {
            AppState::Previewing { .. } | AppState::Applying => Ok(AppState::Idle),
            AppState::Idle => {
                bail!("Cannot finish from state: Idle (nothing to finish)")
            }
            AppState::Error { .. } => {
                bail!("Cannot finish from state: Error (must recover first)")
            }
        }
    }

    /// Cancel a preview and return to Idle. Only valid from Previewing state.
    pub fn cancel_preview(self) -> Result<AppState> {
        match self {
            AppState::Previewing { .. } => Ok(AppState::Idle),
            AppState::Idle => {
                bail!("Cannot cancel preview from state: Idle (no preview active)")
            }
            AppState::Applying => {
                bail!("Cannot cancel preview from state: Applying (cannot cancel during apply)")
            }
            AppState::Error { .. } => {
                bail!("Cannot cancel preview from state: Error (must recover first)")
            }
        }
    }

    /// Enter error state with recovery path. Valid from any state (infallible).
    pub fn fail(self, message: String) -> AppState {
        AppState::Error {
            message,
            recovery: Box::new(AppState::Idle),
        }
    }

    /// Recover from error state. Only valid from Error state.
    pub fn recover(self) -> Result<AppState> {
        match self {
            AppState::Error { recovery, .. } => Ok(*recovery),
            AppState::Idle => {
                bail!("Cannot recover from state: Idle (no error to recover from)")
            }
            AppState::Previewing { .. } => {
                bail!("Cannot recover from state: Previewing (no error to recover from)")
            }
            AppState::Applying => {
                bail!("Cannot recover from state: Applying (no error to recover from)")
            }
        }
    }

    /// Rollback from Applying to Previewing after a failure.
    /// Only valid from Applying state. Requires the snapshot to restore to.
    pub fn rollback(self, snapshot: PreviewSnapshot) -> Result<AppState> {
        match self {
            AppState::Applying => Ok(AppState::Previewing { previous: snapshot }),
            AppState::Idle => {
                bail!("Cannot rollback from state: Idle (not applying)")
            }
            AppState::Previewing { .. } => {
                bail!("Cannot rollback from state: Previewing (not applying)")
            }
            AppState::Error { .. } => {
                bail!("Cannot rollback from state: Error (must recover first)")
            }
        }
    }

    /// Check if the state is Idle
    pub fn is_idle(&self) -> bool {
        matches!(self, AppState::Idle)
    }

    /// Check if the state is Previewing
    pub fn is_previewing(&self) -> bool {
        matches!(self, AppState::Previewing { .. })
    }

    /// Check if the state is Applying
    pub fn is_applying(&self) -> bool {
        matches!(self, AppState::Applying)
    }

    /// Check if the state is Error
    pub fn is_error(&self) -> bool {
        matches!(self, AppState::Error { .. })
    }

    /// Get the previous snapshot if in Previewing state
    pub fn previous_snapshot(&self) -> Option<&PreviewSnapshot> {
        match self {
            AppState::Previewing { previous } => Some(previous),
            _ => None,
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        AppState::Idle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_snapshot() -> PreviewSnapshot {
        let mut wallpapers = HashMap::new();
        wallpapers.insert("eDP-1".to_string(), PathBuf::from("/test/wallpaper.png"));
        PreviewSnapshot {
            wallpapers,
            theme_id: Some("test-theme".to_string()),
        }
    }

    // Valid transitions tests

    #[test]
    fn test_idle_to_previewing() {
        let state = AppState::Idle;
        let snapshot = make_snapshot();
        let result = state.start_preview(snapshot.clone());
        assert!(result.is_ok());
        let new_state = result.unwrap();
        assert!(new_state.is_previewing());
        assert_eq!(new_state.previous_snapshot(), Some(&snapshot));
    }

    #[test]
    fn test_idle_to_applying() {
        let state = AppState::Idle;
        let result = state.start_apply();
        assert!(result.is_ok());
        assert!(result.unwrap().is_applying());
    }

    #[test]
    fn test_previewing_to_applying() {
        let state = AppState::Idle;
        let snapshot = make_snapshot();
        let state = state.start_preview(snapshot).unwrap();
        let result = state.start_apply();
        assert!(result.is_ok());
        assert!(result.unwrap().is_applying());
    }

    #[test]
    fn test_previewing_to_idle_cancel() {
        let state = AppState::Idle;
        let snapshot = make_snapshot();
        let state = state.start_preview(snapshot).unwrap();
        let result = state.cancel_preview();
        assert!(result.is_ok());
        assert!(result.unwrap().is_idle());
    }

    #[test]
    fn test_applying_to_idle_finish() {
        let state = AppState::Idle;
        let state = state.start_apply().unwrap();
        let result = state.finish();
        assert!(result.is_ok());
        assert!(result.unwrap().is_idle());
    }

    #[test]
    fn test_previewing_to_idle_finish() {
        let state = AppState::Idle;
        let snapshot = make_snapshot();
        let state = state.start_preview(snapshot).unwrap();
        let result = state.finish();
        assert!(result.is_ok());
        assert!(result.unwrap().is_idle());
    }

    #[test]
    fn test_any_to_error() {
        let state = AppState::Idle;
        let error_state = state.fail("Test error".to_string());
        assert!(error_state.is_error());
    }

    #[test]
    fn test_error_to_recovery() {
        let state = AppState::Idle;
        let error_state = state.fail("Test error".to_string());
        let result = error_state.recover();
        assert!(result.is_ok());
        assert!(result.unwrap().is_idle());
    }

    // Invalid transitions tests

    #[test]
    fn test_cannot_preview_from_applying() {
        let state = AppState::Applying;
        let snapshot = make_snapshot();
        let result = state.start_preview(snapshot);
        assert!(result.is_err());
    }

    #[test]
    fn test_cannot_preview_from_previewing() {
        let state = AppState::Idle;
        let snapshot = make_snapshot();
        let state = state.start_preview(snapshot.clone()).unwrap();
        let result = state.start_preview(snapshot);
        assert!(result.is_err());
    }

    #[test]
    fn test_cannot_apply_from_error() {
        let state = AppState::Idle;
        let error_state = state.fail("Test error".to_string());
        let result = error_state.start_apply();
        assert!(result.is_err());
    }

    #[test]
    fn test_cannot_finish_from_idle() {
        let state = AppState::Idle;
        let result = state.finish();
        assert!(result.is_err());
    }

    #[test]
    fn test_cannot_cancel_from_idle() {
        let state = AppState::Idle;
        let result = state.cancel_preview();
        assert!(result.is_err());
    }

    #[test]
    fn test_cannot_recover_from_idle() {
        let state = AppState::Idle;
        let result = state.recover();
        assert!(result.is_err());
    }

    // Query methods tests

    #[test]
    fn test_is_idle() {
        let state = AppState::default();
        assert!(state.is_idle());
        assert!(!state.is_previewing());
        assert!(!state.is_applying());
        assert!(!state.is_error());
    }

    #[test]
    fn test_previous_snapshot() {
        let snapshot = make_snapshot();
        let state = AppState::Previewing {
            previous: snapshot.clone(),
        };
        assert_eq!(state.previous_snapshot(), Some(&snapshot));
    }

    #[test]
    fn test_previous_snapshot_none() {
        let state = AppState::Idle;
        assert_eq!(state.previous_snapshot(), None);

        let state = AppState::Applying;
        assert_eq!(state.previous_snapshot(), None);
    }

    #[test]
    fn test_applying_to_previewing_rollback() {
        let state = AppState::Applying;
        let snapshot = make_snapshot();
        let result = state.rollback(snapshot.clone());
        assert!(result.is_ok());
        let new_state = result.unwrap();
        assert!(new_state.is_previewing());
        assert_eq!(new_state.previous_snapshot(), Some(&snapshot));
    }

    #[test]
    fn test_cannot_rollback_from_idle() {
        let state = AppState::Idle;
        let snapshot = make_snapshot();
        let result = state.rollback(snapshot);
        assert!(result.is_err());
    }

    #[test]
    fn test_cannot_rollback_from_previewing() {
        let snapshot = make_snapshot();
        let state = AppState::Previewing { previous: snapshot.clone() };
        let result = state.rollback(snapshot);
        assert!(result.is_err());
    }
}
