//! # SortImportsFix - Trait Implementations
//!
//! This module contains trait implementations for `SortImportsFix`.
//!
//! ## Implemented Traits
//!
//! - `AutofixRule`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::AutofixRule;
use super::types::{FixSuggestion, SortImportsFix, TextEdit};

impl AutofixRule for SortImportsFix {
    fn suggest_fix(
        &self,
        source: &str,
        _span_start: usize,
        _span_end: usize,
    ) -> Option<FixSuggestion> {
        let sorted = Self::sort_imports(source);
        if sorted == source {
            return None;
        }
        let mut fix = FixSuggestion::new("Sort import statements");
        fix.add_edit(TextEdit::new(0, source.len(), &sorted));
        Some(fix)
    }
}
