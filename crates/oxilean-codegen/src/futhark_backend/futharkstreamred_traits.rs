//! # FutharkStreamRed - Trait Implementations
//!
//! This module contains trait implementations for `FutharkStreamRed`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{FutharkStreamKind, FutharkStreamRed};
use std::fmt;

impl std::fmt::Display for FutharkStreamRed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kw = match self.kind {
            FutharkStreamKind::Seq => "stream_red_seq",
            FutharkStreamKind::Par => "stream_red",
        };
        write!(
            f,
            "{} ({}) {} ({}) {}",
            kw, self.op, self.neutral, self.func, self.array
        )
    }
}
