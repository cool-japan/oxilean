//! # HotReloadState - Trait Implementations
//!
//! This module contains trait implementations for `HotReloadState`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::HotReloadState;
use std::fmt;

impl std::fmt::Display for HotReloadState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HotReloadState::Idle => write!(f, "idle"),
            HotReloadState::Pending => write!(f, "pending"),
            HotReloadState::Reloading => write!(f, "reloading"),
            HotReloadState::Done => write!(f, "done"),
            HotReloadState::Failed => write!(f, "failed"),
        }
    }
}
