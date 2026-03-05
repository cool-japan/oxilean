//! # WellFoundedOrder - Trait Implementations
//!
//! This module contains trait implementations for `WellFoundedOrder`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WellFoundedOrder;
use std::fmt;

impl std::fmt::Display for WellFoundedOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WellFoundedOrder::Lexicographic(args) => write!(f, "lex({:?})", args),
            WellFoundedOrder::Measure(i) => write!(f, "measure(arg{})", i),
            WellFoundedOrder::Structural(i) => write!(f, "structural(arg{})", i),
            WellFoundedOrder::Multiset(args) => write!(f, "multiset({:?})", args),
            WellFoundedOrder::Unknown => write!(f, "unknown"),
        }
    }
}
