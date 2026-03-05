//! # MissingDocFix - Trait Implementations
//!
//! This module contains trait implementations for `MissingDocFix`.
//!
//! ## Implemented Traits
//!
//! - `AutofixRule`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::AutofixRule;
use super::types::{FixSuggestion, MissingDocFix, TextEdit};

impl AutofixRule for MissingDocFix {
    fn suggest_fix(
        &self,
        source: &str,
        span_start: usize,
        _span_end: usize,
    ) -> Option<FixSuggestion> {
        let line_start = source[..span_start].rfind('\n').map(|p| p + 1).unwrap_or(0);
        let indent: String = source[line_start..span_start]
            .chars()
            .take_while(|c| c.is_whitespace())
            .collect();
        let placeholder = format!("{}/// TODO: add documentation\n", indent);
        let mut fix = FixSuggestion::new("Add missing doc comment");
        fix.add_edit(TextEdit::new(line_start, line_start, &placeholder));
        fix.is_safe = false;
        Some(fix)
    }
}
