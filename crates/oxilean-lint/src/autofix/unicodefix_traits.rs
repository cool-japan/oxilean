//! # UnicodeFix - Trait Implementations
//!
//! This module contains trait implementations for `UnicodeFix`.
//!
//! ## Implemented Traits
//!
//! - `AutofixRule`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::AutofixRule;
use super::types::{FixSuggestion, TextEdit, UnicodeFix};

impl AutofixRule for UnicodeFix {
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
        let mut modified = snippet.to_string();
        let mut changed = false;
        for (ascii, unicode) in Self::replacements() {
            if modified.contains(ascii) {
                modified = modified.replace(ascii, unicode);
                changed = true;
            }
        }
        if !changed {
            return None;
        }
        let mut fix = FixSuggestion::new("Use Unicode operators");
        fix.add_edit(TextEdit::new(start, end, &modified));
        Some(fix)
    }
}
