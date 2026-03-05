//! # RenameIdentifierFix - Trait Implementations
//!
//! This module contains trait implementations for `RenameIdentifierFix`.
//!
//! ## Implemented Traits
//!
//! - `AutofixRule`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{replace_all_occurrences, AutofixRule};
use super::types::{FixSuggestion, RenameIdentifierFix, TextEdit};

impl AutofixRule for RenameIdentifierFix {
    fn suggest_fix(
        &self,
        source: &str,
        _span_start: usize,
        _span_end: usize,
    ) -> Option<FixSuggestion> {
        if !source.contains(self.old_name.as_str()) {
            return None;
        }
        let renamed = replace_all_occurrences(source, &self.old_name, &self.new_name);
        let mut fix = FixSuggestion::new(&format!(
            "Rename `{}` to `{}`",
            self.old_name, self.new_name
        ));
        fix.add_edit(TextEdit::new(0, source.len(), &renamed));
        Some(fix)
    }
}
