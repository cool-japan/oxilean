//! # SpellingFix - Trait Implementations
//!
//! This module contains trait implementations for `SpellingFix`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `AutofixRule`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::AutofixRule;
use super::types::{FixSuggestion, SpellingFix, TextEdit};

impl Default for SpellingFix {
    fn default() -> Self {
        Self::new()
    }
}

impl AutofixRule for SpellingFix {
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
        for (wrong, right) in &self.corrections {
            if modified.contains(wrong.as_str()) {
                modified = modified.replace(wrong.as_str(), right.as_str());
                changed = true;
            }
        }
        if !changed {
            return None;
        }
        let mut fix = FixSuggestion::new("Fix spelling");
        fix.add_edit(TextEdit::new(start, end, &modified));
        Some(fix)
    }
}
