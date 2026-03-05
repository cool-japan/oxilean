//! # MetaVarContext - summary_group Methods
//!
//! This module contains method implementations for `MetaVarContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use super::metavarcontext_type::MetaVarContext;

impl MetaVarContext {
    /// Diagnostic summary.
    pub fn summary(&self) -> String {
        format!(
            "MetaVarContext {{ total: {}, solved: {}, unsolved: {}, constraints: {} }}",
            self.count(),
            self.solved_count(),
            self.unsolved_count(),
            self.constraint_count()
        )
    }
}
