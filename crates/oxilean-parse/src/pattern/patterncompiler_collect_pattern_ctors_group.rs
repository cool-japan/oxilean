//! # PatternCompiler - collect_pattern_ctors_group Methods
//!
//! This module contains method implementations for `PatternCompiler`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::Pattern;

use super::patterncompiler_type::PatternCompiler;

impl PatternCompiler {
    /// Helper: collect constructor names from a pattern recursively.
    pub(super) fn collect_pattern_ctors(&self, pattern: &Pattern, covered: &mut Vec<String>) {
        match pattern {
            Pattern::Ctor(name, _) => {
                if !covered.contains(name) {
                    covered.push(name.clone());
                }
            }
            Pattern::Or(left, right) => {
                self.collect_pattern_ctors(&left.value, covered);
                self.collect_pattern_ctors(&right.value, covered);
            }
            Pattern::Wild | Pattern::Var(_) | Pattern::Lit(_) => {}
        }
    }
}
