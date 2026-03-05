//! # CtfeMode - Trait Implementations
//!
//! This module contains trait implementations for `CtfeMode`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CtfeMode;
use std::fmt;

impl std::fmt::Display for CtfeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CtfeMode::FullEval => write!(f, "full"),
            CtfeMode::PartialEval => write!(f, "partial"),
            CtfeMode::FoldOnly => write!(f, "fold_only"),
            CtfeMode::Disabled => write!(f, "disabled"),
        }
    }
}
