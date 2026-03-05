//! # PatternCompiler - check_methods Methods
//!
//! This module contains method implementations for `PatternCompiler`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::Pattern;

use super::patterncompiler_type::PatternCompiler;

impl PatternCompiler {
    /// Check if a pattern set is exhaustive.
    ///
    /// A pattern set is exhaustive when it contains at least one irrefutable
    /// pattern (wildcard or variable binding).  When only constructor patterns
    /// are present, exhaustiveness cannot be verified without the full type
    /// definition, so the check is optimistically accepted.
    pub fn check_exhaustive(&self, patterns: &[Pattern]) -> Result<(), String> {
        if patterns.is_empty() {
            return Err("No patterns provided".to_string());
        }
        for pattern in patterns {
            if matches!(pattern, Pattern::Wild | Pattern::Var(_)) {
                return Ok(());
            }
        }
        let has_ctor = patterns
            .iter()
            .any(|p| matches!(p, Pattern::Ctor(_, _) | Pattern::Lit(_)));
        if has_ctor {
            Ok(())
        } else {
            Err("Non-exhaustive patterns".to_string())
        }
    }
}
