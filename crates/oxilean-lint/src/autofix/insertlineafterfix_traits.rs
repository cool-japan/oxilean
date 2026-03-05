//! # InsertLineAfterFix - Trait Implementations
//!
//! This module contains trait implementations for `InsertLineAfterFix`.
//!
//! ## Implemented Traits
//!
//! - `AutofixRule`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::AutofixRule;
use super::types::{FixSuggestion, InsertLineAfterFix, TextEdit};

impl AutofixRule for InsertLineAfterFix {
    fn suggest_fix(
        &self,
        source: &str,
        span_start: usize,
        _span_end: usize,
    ) -> Option<FixSuggestion> {
        let line_end = source[span_start..]
            .find('\n')
            .map(|p| span_start + p + 1)
            .unwrap_or(source.len());
        let insertion = format!("{}\n", self.line_to_insert);
        let mut fix = FixSuggestion::new("Insert line after");
        fix.add_edit(TextEdit::new(line_end, line_end, &insertion));
        Some(fix)
    }
}
