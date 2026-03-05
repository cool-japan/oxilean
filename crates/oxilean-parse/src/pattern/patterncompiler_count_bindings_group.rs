//! # PatternCompiler - count_bindings_group Methods
//!
//! This module contains method implementations for `PatternCompiler`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::Pattern;

use super::patterncompiler_type::PatternCompiler;

impl PatternCompiler {
    /// Count the number of variable bindings in a pattern.
    #[allow(dead_code)]
    pub fn count_bindings(&self, pattern: &Pattern) -> usize {
        match pattern {
            Pattern::Wild => 0,
            Pattern::Var(_) => 1,
            Pattern::Ctor(_, args) => args.iter().map(|a| self.count_bindings(&a.value)).sum(),
            Pattern::Lit(_) => 0,
            Pattern::Or(left, right) => {
                let left_count = self.count_bindings(&left.value);
                let right_count = self.count_bindings(&right.value);
                left_count.max(right_count)
            }
        }
    }
}
