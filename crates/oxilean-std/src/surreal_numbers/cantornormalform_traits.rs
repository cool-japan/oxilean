//! # CantorNormalForm - Trait Implementations
//!
//! This module contains trait implementations for `CantorNormalForm`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CantorNormalForm;
use std::fmt;

impl std::fmt::Display for CantorNormalForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_cnf())
    }
}
