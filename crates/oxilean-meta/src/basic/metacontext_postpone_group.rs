//! # MetaContext - postpone_group Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::Expr;

use super::types::PostponedConstraint;

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Add a postponed constraint.
    pub fn postpone(&mut self, lhs: Expr, rhs: Expr) {
        self.postponed.push(PostponedConstraint {
            lhs,
            rhs,
            depth: self.depth,
        });
    }
}
