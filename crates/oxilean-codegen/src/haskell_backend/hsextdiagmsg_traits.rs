//! # HsExtDiagMsg - Trait Implementations
//!
//! This module contains trait implementations for `HsExtDiagMsg`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::HsExtDiagMsg;
use std::fmt;

impl std::fmt::Display for HsExtDiagMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}: {}", self.severity, self.pass, self.message)
    }
}
