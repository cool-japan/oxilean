//! # PrefixEnricher - Trait Implementations
//!
//! This module contains trait implementations for `PrefixEnricher`.
//!
//! ## Implemented Traits
//!
//! - `TraceEventEnricher`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::TraceEventEnricher;
use super::types::{PrefixEnricher, TraceEvent};
use std::fmt;

impl TraceEventEnricher for PrefixEnricher {
    fn enrich(&self, event: &mut TraceEvent) {
        event.message = format!("[{}] {}", self.prefix, event.message);
    }
}
