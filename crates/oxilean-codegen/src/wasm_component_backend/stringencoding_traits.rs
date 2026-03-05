//! # StringEncoding - Trait Implementations
//!
//! This module contains trait implementations for `StringEncoding`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::StringEncoding;
use std::fmt;

impl fmt::Display for StringEncoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StringEncoding::Utf8 => write!(f, "utf8"),
            StringEncoding::Utf16 => write!(f, "utf16"),
            StringEncoding::Latin1Utf16 => write!(f, "latin1+utf16"),
        }
    }
}
