//! # WatchAction - Trait Implementations
//!
//! This module contains trait implementations for `WatchAction`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WatchAction;
use std::fmt;

impl fmt::Display for WatchAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WatchAction::Recheck => write!(f, "recheck"),
            WatchAction::Rebuild => write!(f, "rebuild"),
            WatchAction::Notify => write!(f, "notify"),
            WatchAction::Custom => write!(f, "custom"),
        }
    }
}
