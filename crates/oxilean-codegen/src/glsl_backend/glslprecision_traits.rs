//! # GLSLPrecision - Trait Implementations
//!
//! This module contains trait implementations for `GLSLPrecision`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::GLSLPrecision;
use std::fmt;

impl fmt::Display for GLSLPrecision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GLSLPrecision::Low => write!(f, "lowp"),
            GLSLPrecision::Medium => write!(f, "mediump"),
            GLSLPrecision::High => write!(f, "highp"),
        }
    }
}
