//! # CommentOutFix - Trait Implementations
//!
//! This module contains trait implementations for `CommentOutFix`.
//!
//! ## Implemented Traits
//!
//! - `AutofixRule`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::AutofixRule;
use super::types::{CommentOutFix, FixSuggestion, TextEdit};

impl AutofixRule for CommentOutFix {
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
        let commented = format!("{{- {} -}}", snippet);
        let mut fix = FixSuggestion::new("Comment out code");
        fix.add_edit(TextEdit::new(start, end, &commented));
        fix.is_safe = false;
        Some(fix)
    }
}
