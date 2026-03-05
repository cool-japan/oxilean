//! # WatchBackend - Trait Implementations
//!
//! This module contains trait implementations for `WatchBackend`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WatchBackend;
use std::fmt;

impl std::fmt::Display for WatchBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WatchBackend::Polling => write!(f, "polling"),
            WatchBackend::Inotify => write!(f, "inotify"),
            WatchBackend::Kqueue => write!(f, "kqueue"),
            WatchBackend::ReadDirChanges => write!(f, "read_dir_changes"),
        }
    }
}
