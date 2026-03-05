//! # IdentityMetaStep - Trait Implementations
//!
//! This module contains trait implementations for `IdentityMetaStep`.
//!
//! ## Implemented Traits
//!
//! - `MetaElabStep`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{MetaElabStep, MetaResult};
use super::types::{IdentityMetaStep, MetaEnv, QuotedExpr};
use std::fmt;

impl MetaElabStep for IdentityMetaStep {
    fn step_name(&self) -> &str {
        "identity"
    }
    fn apply(&self, expr: QuotedExpr, _env: &mut MetaEnv) -> MetaResult<QuotedExpr> {
        Ok(expr)
    }
}
