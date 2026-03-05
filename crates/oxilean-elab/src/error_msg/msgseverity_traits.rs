//! # MsgSeverity - Trait Implementations
//!
//! This module contains trait implementations for `MsgSeverity`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MsgSeverity;
use std::fmt;

impl std::fmt::Display for MsgSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MsgSeverity::Info => write!(f, "info"),
            MsgSeverity::Hint => write!(f, "hint"),
            MsgSeverity::Warning => write!(f, "warning"),
            MsgSeverity::Error => write!(f, "error"),
            MsgSeverity::Fatal => write!(f, "fatal"),
        }
    }
}
