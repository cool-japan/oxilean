//! # PatternCompiler - patterns_equivalent_group Methods
//!
//! This module contains method implementations for `PatternCompiler`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::Pattern;

use super::patterncompiler_type::PatternCompiler;

impl PatternCompiler {
    /// Check if two patterns are structurally equivalent.
    pub fn patterns_equivalent(&self, p1: &Pattern, p2: &Pattern) -> bool {
        match (p1, p2) {
            (Pattern::Wild, Pattern::Wild) => true,
            (Pattern::Var(v1), Pattern::Var(v2)) => v1 == v2,
            (Pattern::Lit(l1), Pattern::Lit(l2)) => l1 == l2,
            (Pattern::Ctor(n1, a1), Pattern::Ctor(n2, a2)) => {
                n1 == n2
                    && a1.len() == a2.len()
                    && a1
                        .iter()
                        .zip(a2.iter())
                        .all(|(x, y)| self.patterns_equivalent(&x.value, &y.value))
            }
            (Pattern::Or(l1, r1), Pattern::Or(l2, r2)) => {
                self.patterns_equivalent(&l1.value, &l2.value)
                    && self.patterns_equivalent(&r1.value, &r2.value)
            }
            _ => false,
        }
    }
}
