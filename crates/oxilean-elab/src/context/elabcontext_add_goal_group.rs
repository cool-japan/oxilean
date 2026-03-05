//! # ElabContext - add_goal_group Methods
//!
//! This module contains method implementations for `ElabContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Environment, Expr, FVarId, Name};

use super::elabcontext_type::ElabContext;
use super::functions::*;

impl<'env> ElabContext<'env> {
    /// Add a pending goal.
    pub fn add_goal(&mut self, goal: Expr) {
        self.pending_goals.push(goal);
    }
}
