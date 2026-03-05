//! # RemoveDeadCodeFix - Trait Implementations
//!
//! This module contains trait implementations for `RemoveDeadCodeFix`.
//!
//! ## Implemented Traits
//!
//! - `AutofixRule`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::AutofixRule;
use super::types::{FixSuggestion, RemoveDeadCodeFix, TextEdit};

impl AutofixRule for RemoveDeadCodeFix {
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
        let mut fix = FixSuggestion::new("Remove dead code");
        fix.add_edit(TextEdit::new(start, end, ""));
        fix.is_safe = false;
        Some(fix)
    }
}
