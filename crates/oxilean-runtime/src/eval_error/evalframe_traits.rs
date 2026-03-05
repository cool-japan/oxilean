//! # EvalFrame - Trait Implementations
//!
//! This module contains trait implementations for `EvalFrame`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::EvalFrame;
use std::fmt;

impl fmt::Display for EvalFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tc = if self.is_tail_call { " [tail]" } else { "" };
        write!(f, "  in `{}`{} at {}", self.name, tc, self.call_site)
    }
}
