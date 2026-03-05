//! # UnusedImportFix - Trait Implementations
//!
//! This module contains trait implementations for `UnusedImportFix`.
//!
//! ## Implemented Traits
//!
//! - `AutofixRule`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::AutofixRule;
use super::types::{FixSuggestion, TextEdit, UnusedImportFix};

impl AutofixRule for UnusedImportFix {
    fn suggest_fix(
        &self,
        source: &str,
        span_start: usize,
        _span_end: usize,
    ) -> Option<FixSuggestion> {
        let line_start = source[..span_start].rfind('\n').map(|p| p + 1).unwrap_or(0);
        let line_end = source[span_start..]
            .find('\n')
            .map(|p| span_start + p + 1)
            .unwrap_or(source.len());
        let mut fix = FixSuggestion::new("Remove unused import");
        fix.add_edit(TextEdit::new(line_start, line_end, ""));
        Some(fix)
    }
}
