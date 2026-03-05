//! # TypeAnnotationFix - Trait Implementations
//!
//! This module contains trait implementations for `TypeAnnotationFix`.
//!
//! ## Implemented Traits
//!
//! - `AutofixRule`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::AutofixRule;
use super::types::{FixSuggestion, TextEdit, TypeAnnotationFix};

impl AutofixRule for TypeAnnotationFix {
    fn suggest_fix(
        &self,
        _source: &str,
        span_start: usize,
        _span_end: usize,
    ) -> Option<FixSuggestion> {
        let annotation = format!(" : {}", self.type_name);
        let mut fix = FixSuggestion::new("Insert type annotation");
        fix.add_edit(TextEdit::new(span_start, span_start, &annotation));
        fix.is_safe = false;
        Some(fix)
    }
}
