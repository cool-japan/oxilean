//! # IdentityStep - Trait Implementations
//!
//! This module contains trait implementations for `IdentityStep`.
//!
//! ## Implemented Traits
//!
//! - `MacroTransformStep`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Expr, Level, Literal, Name};
use std::fmt;

use super::functions::MacroTransformStep;
use super::types::{IdentityStep, MacroError};

impl MacroTransformStep for IdentityStep {
    fn step_name(&self) -> &'static str {
        "identity"
    }
    fn transform(&self, expr: Expr) -> Result<Expr, MacroError> {
        Ok(expr)
    }
}
