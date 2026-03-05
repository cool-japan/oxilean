//! # ClosureLayout - Trait Implementations
//!
//! This module contains trait implementations for `ClosureLayout`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use crate::native_backend::*;

use super::types::ClosureLayout;
use std::fmt;

impl fmt::Display for ClosureLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ClosureLayout {{ arity={}, captured={}, env_offset={}, total={} }}",
            self.arity, self.num_captured, self.env_offset, self.object_layout.total_size,
        )
    }
}
