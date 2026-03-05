//! # NamingConventionFix - Trait Implementations
//!
//! This module contains trait implementations for `NamingConventionFix`.
//!
//! ## Implemented Traits
//!
//! - `AutofixRule`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::AutofixRule;
use super::types::{FixSuggestion, NamingConventionFix, TextEdit};

impl AutofixRule for NamingConventionFix {
    fn suggest_fix(
        &self,
        source: &str,
        span_start: usize,
        span_end: usize,
    ) -> Option<FixSuggestion> {
        let start = span_start.min(source.len());
        let end = span_end.min(source.len());
        if start >= end {
            return None;
        }
        let original = &source[start..end];
        let snake = Self::to_snake_case(original);
        if snake == original {
            return None;
        }
        let mut fix = FixSuggestion::new("Convert to snake_case");
        fix.add_edit(TextEdit::new(start, end, &snake));
        Some(fix)
    }
}
