//! # TargetKind - Trait Implementations
//!
//! This module contains trait implementations for `TargetKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TargetKind;
use std::fmt;

impl fmt::Display for TargetKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TargetKind::Lib => write!(f, "lib"),
            TargetKind::Bin => write!(f, "bin"),
            TargetKind::Test => write!(f, "test"),
            TargetKind::Bench => write!(f, "bench"),
            TargetKind::Doc => write!(f, "doc"),
            TargetKind::BuildScript => write!(f, "build-script"),
        }
    }
}
