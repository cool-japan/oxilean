//! # LuaExtDiagSeverity - Trait Implementations
//!
//! This module contains trait implementations for `LuaExtDiagSeverity`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::LuaExtDiagSeverity;
use std::fmt;

impl std::fmt::Display for LuaExtDiagSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LuaExtDiagSeverity::Note => write!(f, "note"),
            LuaExtDiagSeverity::Warning => write!(f, "warning"),
            LuaExtDiagSeverity::Error => write!(f, "error"),
        }
    }
}
