//! # PatternCompiler - flatten_or_pattern_group Methods
//!
//! This module contains method implementations for `PatternCompiler`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::Pattern;

use super::patterncompiler_type::PatternCompiler;

impl PatternCompiler {
    /// Flatten a nested or-pattern into a flat list of alternatives.
    pub fn flatten_or_pattern(&self, pattern: &Pattern) -> Vec<Pattern> {
        match pattern {
            Pattern::Or(left, right) => {
                let mut result = self.flatten_or_pattern(&left.value);
                result.extend(self.flatten_or_pattern(&right.value));
                result
            }
            other => vec![other.clone()],
        }
    }
}
