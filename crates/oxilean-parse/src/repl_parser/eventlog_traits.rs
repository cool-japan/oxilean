//! # EventLog - Trait Implementations
//!
//! This module contains trait implementations for `EventLog`.
//!
//! ## Implemented Traits
//!
//! - `ReplEventListener`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::ReplEventListener;
use super::types::{EventLog, ReplEvent};

impl ReplEventListener for EventLog {
    fn on_event(&mut self, event: &ReplEvent) {
        self.events.push(event.clone());
    }
}
