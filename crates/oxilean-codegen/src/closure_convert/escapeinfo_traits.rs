//! # EscapeInfo - Trait Implementations
//!
//! This module contains trait implementations for `EscapeInfo`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::EscapeInfo;
use std::fmt;

impl fmt::Display for EscapeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EscapeInfo::NoEscape => write!(f, "no-escape"),
            EscapeInfo::LocalEscape => write!(f, "local-escape"),
            EscapeInfo::GlobalEscape => write!(f, "global-escape"),
        }
    }
}
