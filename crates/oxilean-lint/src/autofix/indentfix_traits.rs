//! # IndentFix - Trait Implementations
//!
//! This module contains trait implementations for `IndentFix`.
//!
//! ## Implemented Traits
//!
//! - `AutofixRule`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::AutofixRule;
use super::types::{FixSuggestion, IndentFix, TextEdit};

impl AutofixRule for IndentFix {
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
        let modified: String = snippet
            .lines()
            .map(|l| {
                if self.delta >= 0 {
                    format!("{}{}", " ".repeat(self.delta as usize), l)
                } else {
                    let strip = (-self.delta) as usize;
                    let spaces: usize = l.chars().take_while(|c| *c == ' ').count();
                    l[spaces.min(strip)..].to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        if modified == snippet {
            return None;
        }
        let mut fix = FixSuggestion::new("Adjust indentation");
        fix.add_edit(TextEdit::new(start, end, &modified));
        Some(fix)
    }
}
