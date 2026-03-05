//! # PatternCompiler - collect_literals_group Methods
//!
//! This module contains method implementations for `PatternCompiler`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::Pattern;

use super::patterncompiler_type::PatternCompiler;

impl PatternCompiler {
    /// Helper to collect all literal values from a pattern.
    pub(super) fn collect_literals(&self, pattern: &Pattern, values: &mut Vec<i64>) {
        match pattern {
            Pattern::Lit(crate::Literal::Nat(n)) => values.push(*n as i64),
            Pattern::Ctor(_, args) => {
                for arg in args {
                    self.collect_literals(&arg.value, values);
                }
            }
            Pattern::Or(left, right) => {
                self.collect_literals(&left.value, values);
                self.collect_literals(&right.value, values);
            }
            _ => {}
        }
    }
}
