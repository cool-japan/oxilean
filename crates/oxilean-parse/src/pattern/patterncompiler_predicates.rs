//! # PatternCompiler - predicates Methods
//!
//! This module contains method implementations for `PatternCompiler`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::Pattern;

use super::types::PatternRow;

use super::patterncompiler_type::PatternCompiler;

impl PatternCompiler {
    /// Check if a pattern is irrefutable (always matches).
    ///
    /// Wildcards and variable bindings are irrefutable. Constructor and
    /// literal patterns are not. An or-pattern is irrefutable if either
    /// branch is irrefutable.
    pub fn is_irrefutable(&self, pattern: &Pattern) -> bool {
        match pattern {
            Pattern::Wild | Pattern::Var(_) => true,
            Pattern::Ctor(_, _) | Pattern::Lit(_) => false,
            Pattern::Or(left, right) => {
                self.is_irrefutable(&left.value) || self.is_irrefutable(&right.value)
            }
        }
    }
    /// Check if all patterns in a list are irrefutable.
    pub(super) fn all_irrefutable(&self, patterns: &[PatternRow]) -> bool {
        patterns
            .iter()
            .all(|row| row.patterns.iter().all(|p| self.is_irrefutable(p)))
    }
    /// Check if a set of patterns subsumes a new pattern.
    pub(super) fn subsumes_all(&self, rows: &[PatternRow], new_pattern: &[Pattern]) -> bool {
        for row in rows {
            if row.patterns.len() == new_pattern.len()
                && row
                    .patterns
                    .iter()
                    .zip(new_pattern.iter())
                    .all(|(p1, p2)| self.is_irrefutable(p1) || self.patterns_equivalent(p1, p2))
            {
                return true;
            }
        }
        false
    }
}
