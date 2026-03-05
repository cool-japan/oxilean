//! # CilExtDiagMsg - Trait Implementations
//!
//! This module contains trait implementations for `CilExtDiagMsg`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::CilExtDiagMsg;
use std::fmt;

impl std::fmt::Display for CilExtDiagMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}: {}", self.severity, self.pass, self.message)
    }
}
