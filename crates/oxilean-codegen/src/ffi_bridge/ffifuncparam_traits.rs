//! # FfiFuncParam - Trait Implementations
//!
//! This module contains trait implementations for `FfiFuncParam`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FfiFuncParam;
use std::fmt;

impl std::fmt::Display for FfiFuncParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let attrs: Vec<String> = self.attrs.iter().map(|a| a.to_string()).collect();
        if self.is_vararg {
            write!(f, "...")
        } else if attrs.is_empty() {
            write!(f, "{} {}", self.ffi_type, self.name)
        } else {
            write!(f, "{} {} {}", attrs.join(" "), self.ffi_type, self.name)
        }
    }
}
