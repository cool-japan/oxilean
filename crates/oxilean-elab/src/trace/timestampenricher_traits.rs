//! # TimestampEnricher - Trait Implementations
//!
//! This module contains trait implementations for `TimestampEnricher`.
//!
//! ## Implemented Traits
//!
//! - `TraceEventEnricher`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::TraceEventEnricher;
use super::types::{TimestampEnricher, TraceEvent};
use std::fmt;

impl TraceEventEnricher for TimestampEnricher {
    fn enrich(&self, _event: &mut TraceEvent) {}
}
