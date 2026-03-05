//! # MetaVarContext - constraints_group Methods
//!
//! This module contains method implementations for `MetaVarContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MetaConstraint;

use super::functions::*;
use super::metavarcontext_type::MetaVarContext;

impl MetaVarContext {
    /// Get constraints.
    pub fn constraints(&self) -> &[MetaConstraint] {
        &self.constraints
    }
}
