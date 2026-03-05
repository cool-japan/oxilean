//! # PatternCompiler - collect_ctors_from_pattern_group Methods
//!
//! This module contains method implementations for `PatternCompiler`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::Pattern;

use super::patterncompiler_type::PatternCompiler;

impl PatternCompiler {
    /// Recursively collect constructors from a single pattern.
    pub(super) fn collect_ctors_from_pattern(
        &self,
        pattern: &Pattern,
        seen: &mut Vec<(String, usize)>,
    ) {
        match pattern {
            Pattern::Ctor(name, args) => {
                if !seen.iter().any(|(n, _)| n == name) {
                    seen.push((name.clone(), args.len()));
                }
            }
            Pattern::Or(left, right) => {
                self.collect_ctors_from_pattern(&left.value, seen);
                self.collect_ctors_from_pattern(&right.value, seen);
            }
            Pattern::Wild | Pattern::Var(_) | Pattern::Lit(_) => {}
        }
    }
}
