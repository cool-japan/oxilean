//! # UncommentFix - Trait Implementations
//!
//! This module contains trait implementations for `UncommentFix`.
//!
//! ## Implemented Traits
//!
//! - `AutofixRule`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::AutofixRule;
use super::types::{FixSuggestion, TextEdit, UncommentFix};

impl AutofixRule for UncommentFix {
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
        let snippet = source[start..end].trim();
        if snippet.starts_with("{-") && snippet.ends_with("-}") {
            let inner = snippet[2..snippet.len() - 2].trim();
            let mut fix = FixSuggestion::new("Uncomment code");
            fix.add_edit(TextEdit::new(start, end, inner));
            return Some(fix);
        }
        None
    }
}
