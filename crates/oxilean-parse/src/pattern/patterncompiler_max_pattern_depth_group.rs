//! # PatternCompiler - max_pattern_depth_group Methods
//!
//! This module contains method implementations for `PatternCompiler`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::Pattern;

use super::patterncompiler_type::PatternCompiler;

impl PatternCompiler {
    /// Check if pattern depth is reasonable (prevent stack overflow).
    pub fn max_pattern_depth(&self, pattern: &Pattern) -> usize {
        match pattern {
            Pattern::Wild | Pattern::Var(_) | Pattern::Lit(_) => 0,
            Pattern::Ctor(_, args) => {
                1 + args
                    .iter()
                    .map(|a| self.max_pattern_depth(&a.value))
                    .max()
                    .unwrap_or(0)
            }
            Pattern::Or(left, right) => {
                1 + std::cmp::max(
                    self.max_pattern_depth(&left.value),
                    self.max_pattern_depth(&right.value),
                )
            }
        }
    }
}
