//! # ReasoningMode - Trait Implementations
//!
//! This module contains trait implementations for `ReasoningMode`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ReasoningMode;
use std::fmt;

impl fmt::Display for ReasoningMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReasoningMode::Backward => write!(f, "backward"),
            ReasoningMode::Forward => write!(f, "forward"),
            ReasoningMode::Both => write!(f, "both"),
        }
    }
}
