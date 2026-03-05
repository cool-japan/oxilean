//! # AddSemicolonFix - Trait Implementations
//!
//! This module contains trait implementations for `AddSemicolonFix`.
//!
//! ## Implemented Traits
//!
//! - `AutofixRule`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::AutofixRule;
use super::types::{AddSemicolonFix, FixSuggestion, TextEdit};

impl AutofixRule for AddSemicolonFix {
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
        let snippet = source[start..end].trim_end();
        if snippet.ends_with(';') || snippet.ends_with('{') || snippet.ends_with('}') {
            return None;
        }
        let new_text = format!("{};", snippet);
        let mut fix = FixSuggestion::new("Add missing semicolon");
        fix.add_edit(TextEdit::new(start, end, &new_text));
        Some(fix)
    }
}
