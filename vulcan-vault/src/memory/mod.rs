//! Agent memory management
//!
//! This module handles:
//! - Memory formation (recording decisions, lessons, preferences)
//! - Memory retrieval (semantic and context-based)
//! - Memory decay and reinforcement

mod formation;
mod retrieval;
mod decay;

pub use formation::{FormationError, FormationResult, LessonSource, MemoryFormation, SessionEvent};
pub use retrieval::{MemoryRetrieval, RetrievalConfig, RetrievalError, RetrievalResult, ScoredMemory};
pub use decay::{CleanupReport, DecayConfig, DecayError, DecayReport, DecayResult, MemoryDecay};
