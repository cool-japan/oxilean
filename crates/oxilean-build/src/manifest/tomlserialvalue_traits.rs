//! # TomlSerialValue - Trait Implementations
//!
//! This module contains trait implementations for `TomlSerialValue`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TomlSerialValue;
use std::fmt;

impl std::fmt::Display for TomlSerialValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_toml_string())
    }
}
