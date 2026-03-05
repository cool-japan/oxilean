//! # TraceSpan - Trait Implementations
//!
//! This module contains trait implementations for `TraceSpan`.
//!
//! ## Implemented Traits
//!
//! - `Drop`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TraceSpan;
use std::fmt;

impl<'a> Drop for TraceSpan<'a> {
    fn drop(&mut self) {
        self.tracer.pop();
        self.tracer.info(format!("<<< {}", self.name));
    }
}
