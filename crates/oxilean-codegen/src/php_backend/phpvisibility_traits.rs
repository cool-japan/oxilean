//! # PHPVisibility - Trait Implementations
//!
//! This module contains trait implementations for `PHPVisibility`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::format_param;
use super::types::PHPVisibility;
use std::fmt;

impl fmt::Display for PHPVisibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PHPVisibility::Public => write!(f, "public"),
            PHPVisibility::Protected => write!(f, "protected"),
            PHPVisibility::Private => write!(f, "private"),
        }
    }
}
