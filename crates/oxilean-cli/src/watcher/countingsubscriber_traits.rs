//! # CountingSubscriber - Trait Implementations
//!
//! This module contains trait implementations for `CountingSubscriber`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `WatcherSubscriber`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::fmt;
use std::path::{Path, PathBuf};

use super::functions::WatcherSubscriber;
use super::types::{CountingSubscriber, WatchEventKind};

impl Default for CountingSubscriber {
    fn default() -> Self {
        Self::new()
    }
}

impl WatcherSubscriber for CountingSubscriber {
    fn on_events(&self, events: &[(PathBuf, WatchEventKind)]) {
        self.count
            .fetch_add(events.len() as u64, std::sync::atomic::Ordering::Relaxed);
    }
}
