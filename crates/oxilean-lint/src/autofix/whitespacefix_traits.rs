//! # WhitespaceFix - Trait Implementations
//!
//! This module contains trait implementations for `WhitespaceFix`.
//!
//! ## Implemented Traits
//!
//! - `AutofixRule`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{strip_trailing_whitespace, AutofixRule};
use super::types::{FixSuggestion, TextEdit, WhitespaceFix};

impl AutofixRule for WhitespaceFix {
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
        let snippet = &source[start..end];
        let cleaned = strip_trailing_whitespace(snippet);
        if cleaned == snippet {
            return None;
        }
        let mut fix = FixSuggestion::new("Remove trailing whitespace");
        fix.add_edit(TextEdit::new(start, end, &cleaned));
        Some(fix)
    }
}
