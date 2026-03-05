//! # OJoinDiagMsg - Trait Implementations
//!
//! This module contains trait implementations for `OJoinDiagMsg`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::OJoinDiagMsg;
use std::fmt;

impl std::fmt::Display for OJoinDiagMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}: {}", self.severity, self.pass, self.message)
    }
}
