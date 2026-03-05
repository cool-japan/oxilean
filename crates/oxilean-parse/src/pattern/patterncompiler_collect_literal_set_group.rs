//! # PatternCompiler - collect_literal_set_group Methods
//!
//! This module contains method implementations for `PatternCompiler`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::Pattern;
use std::collections::HashSet;

use super::patterncompiler_type::PatternCompiler;

impl PatternCompiler {
    /// Helper to collect literal values into a set.
    pub(super) fn collect_literal_set(&self, pattern: &Pattern, set: &mut HashSet<u64>) {
        match pattern {
            Pattern::Lit(crate::Literal::Nat(n)) => {
                set.insert(*n);
            }
            Pattern::Ctor(_, args) => {
                for arg in args {
                    self.collect_literal_set(&arg.value, set);
                }
            }
            Pattern::Or(left, right) => {
                self.collect_literal_set(&left.value, set);
                self.collect_literal_set(&right.value, set);
            }
            _ => {}
        }
    }
}
