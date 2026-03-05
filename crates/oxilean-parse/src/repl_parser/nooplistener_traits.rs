//! # NoopListener - Trait Implementations
//!
//! This module contains trait implementations for `NoopListener`.
//!
//! ## Implemented Traits
//!
//! - `ReplEventListener`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::ReplEventListener;
use super::types::{NoopListener, ReplEvent};

impl ReplEventListener for NoopListener {
    fn on_event(&mut self, _event: &ReplEvent) {}
}
