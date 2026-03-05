//! # FutharkMatchExpr - Trait Implementations
//!
//! This module contains trait implementations for `FutharkMatchExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkMatchExpr;
use std::fmt;

impl std::fmt::Display for FutharkMatchExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "match {}", self.scrutinee)?;
        for arm in &self.arms {
            write!(f, "\ncase {} -> {}", arm.pattern, arm.body)?;
        }
        Ok(())
    }
}
