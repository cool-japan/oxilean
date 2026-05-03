//! # BuildProfileKind - Trait Implementations
//!
//! This module contains trait implementations for `BuildProfileKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BuildProfileKind;
use std::fmt;

impl fmt::Display for BuildProfileKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildProfileKind::Debug => write!(f, "debug"),
            BuildProfileKind::Release => write!(f, "release"),
            BuildProfileKind::Test => write!(f, "test"),
            BuildProfileKind::Bench => write!(f, "bench"),
            BuildProfileKind::Doc => write!(f, "doc"),
        }
    }
}
