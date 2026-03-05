//! # WatchEventKind - Trait Implementations
//!
//! This module contains trait implementations for `WatchEventKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WatchEventKind;
use std::fmt;

impl fmt::Display for WatchEventKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WatchEventKind::Created => write!(f, "created"),
            WatchEventKind::Modified => write!(f, "modified"),
            WatchEventKind::Deleted => write!(f, "deleted"),
            WatchEventKind::Renamed => write!(f, "renamed"),
        }
    }
}
