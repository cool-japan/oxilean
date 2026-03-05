//! # SwiftStmt - Trait Implementations
//!
//! This module contains trait implementations for `SwiftStmt`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::*;
use super::types::SwiftStmt;
use std::fmt;

impl fmt::Display for SwiftStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", emit_stmt(self, 0))
    }
}
