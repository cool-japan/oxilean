//! # PrologMode - Trait Implementations
//!
//! This module contains trait implementations for `PrologMode`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::fmt_dcg_seq;
use super::types::PrologMode;
use std::fmt;

impl fmt::Display for PrologMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrologMode::In => write!(f, "+"),
            PrologMode::Out => write!(f, "-"),
            PrologMode::InOut => write!(f, "?"),
            PrologMode::Meta => write!(f, ":"),
            PrologMode::NotFurther => write!(f, "@"),
        }
    }
}
