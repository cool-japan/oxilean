//! # FfiExtDiagMsg - Trait Implementations
//!
//! This module contains trait implementations for `FfiExtDiagMsg`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FfiExtDiagMsg;
use std::fmt;

impl std::fmt::Display for FfiExtDiagMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}: {}", self.severity, self.pass, self.message)
    }
}
