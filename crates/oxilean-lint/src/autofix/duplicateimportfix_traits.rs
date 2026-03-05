//! # DuplicateImportFix - Trait Implementations
//!
//! This module contains trait implementations for `DuplicateImportFix`.
//!
//! ## Implemented Traits
//!
//! - `AutofixRule`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::AutofixRule;
use super::types::{DuplicateImportFix, FixSuggestion, TextEdit};

impl AutofixRule for DuplicateImportFix {
    fn suggest_fix(
        &self,
        source: &str,
        _span_start: usize,
        _span_end: usize,
    ) -> Option<FixSuggestion> {
        let deduped = Self::deduplicate(source);
        if deduped == source {
            return None;
        }
        let mut fix = FixSuggestion::new("Remove duplicate imports");
        fix.add_edit(TextEdit::new(0, source.len(), &deduped));
        Some(fix)
    }
}
