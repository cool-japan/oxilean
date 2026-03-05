//! # MetaVarContext - add_constraint_group Methods
//!
//! This module contains method implementations for `MetaVarContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MetaConstraint;

use super::functions::*;
use super::metavarcontext_type::MetaVarContext;

impl MetaVarContext {
    /// Add a constraint.
    pub fn add_constraint(&mut self, c: MetaConstraint) {
        self.constraints.push(c);
    }
}
