//! # WatchEvent - Trait Implementations
//!
//! This module contains trait implementations for `WatchEvent`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WatchEvent;

impl std::fmt::Display for WatchEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WatchEvent::Modified(p) => write!(f, "modified: {}", p),
            WatchEvent::Created(p) => write!(f, "created: {}", p),
            WatchEvent::Deleted(p) => write!(f, "deleted: {}", p),
            WatchEvent::Renamed(old, new) => write!(f, "renamed: {} -> {}", old, new),
        }
    }
}
