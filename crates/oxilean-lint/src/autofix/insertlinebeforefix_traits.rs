//! # InsertLineBeforeFix - Trait Implementations
//!
//! This module contains trait implementations for `InsertLineBeforeFix`.
//!
//! ## Implemented Traits
//!
//! - `AutofixRule`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::AutofixRule;
use super::types::{FixSuggestion, InsertLineBeforeFix, TextEdit};

impl AutofixRule for InsertLineBeforeFix {
    fn suggest_fix(
        &self,
        source: &str,
        span_start: usize,
        _span_end: usize,
    ) -> Option<FixSuggestion> {
        let line_start = source[..span_start].rfind('\n').map(|p| p + 1).unwrap_or(0);
        let insertion = format!("{}\n", self.line_to_insert);
        let mut fix = FixSuggestion::new("Insert line before");
        fix.add_edit(TextEdit::new(line_start, line_start, &insertion));
        Some(fix)
    }
}
