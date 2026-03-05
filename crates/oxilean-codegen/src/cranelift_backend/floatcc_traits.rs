//! # FloatCC - Trait Implementations
//!
//! This module contains trait implementations for `FloatCC`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FloatCC;
use std::fmt;

impl fmt::Display for FloatCC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FloatCC::Equal => write!(f, "eq"),
            FloatCC::NotEqual => write!(f, "ne"),
            FloatCC::LessThan => write!(f, "lt"),
            FloatCC::LessThanOrEqual => write!(f, "le"),
            FloatCC::GreaterThan => write!(f, "gt"),
            FloatCC::GreaterThanOrEqual => write!(f, "ge"),
            FloatCC::Ordered => write!(f, "ord"),
            FloatCC::Unordered => write!(f, "uno"),
            FloatCC::UnorderedOrEqual => write!(f, "une"),
            FloatCC::UnorderedOrLessThan => write!(f, "ult"),
            FloatCC::UnorderedOrGreaterThan => write!(f, "ugt"),
        }
    }
}
