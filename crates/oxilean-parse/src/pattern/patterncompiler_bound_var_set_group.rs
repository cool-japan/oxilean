//! # PatternCompiler - bound_var_set_group Methods
//!
//! This module contains method implementations for `PatternCompiler`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::Pattern;
use std::collections::HashSet;

use super::patterncompiler_type::PatternCompiler;

impl PatternCompiler {
    /// Get all variables bound by a pattern as a set.
    pub fn bound_var_set(&self, pattern: &Pattern) -> HashSet<String> {
        let names = self.extract_bound_names(pattern);
        names.into_iter().collect()
    }
    /// Check if patterns bind the same variables.
    pub fn same_bindings(&self, patterns: &[Pattern]) -> bool {
        if patterns.is_empty() {
            return true;
        }
        let first_set = self.bound_var_set(&patterns[0]);
        patterns
            .iter()
            .skip(1)
            .all(|p| self.bound_var_set(p) == first_set)
    }
}
