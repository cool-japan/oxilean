//! # PatternCompiler - collect_bound_names_group Methods
//!
//! This module contains method implementations for `PatternCompiler`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::Pattern;

use super::patterncompiler_type::PatternCompiler;

impl PatternCompiler {
    /// Helper: recursively collect bound names.
    pub(super) fn collect_bound_names(&self, pattern: &Pattern, names: &mut Vec<String>) {
        match pattern {
            Pattern::Wild => {}
            Pattern::Var(name) => names.push(name.clone()),
            Pattern::Ctor(_, args) => {
                for arg in args {
                    self.collect_bound_names(&arg.value, names);
                }
            }
            Pattern::Lit(_) => {}
            Pattern::Or(left, _right) => {
                self.collect_bound_names(&left.value, names);
            }
        }
    }
}
