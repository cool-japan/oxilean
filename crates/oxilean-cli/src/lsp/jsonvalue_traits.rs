//! # JsonValue - Trait Implementations
//!
//! This module contains trait implementations for `JsonValue`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::fmt;

use super::functions::format_json_value;
use super::types::JsonValue;

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format_json_value(self))
    }
}
