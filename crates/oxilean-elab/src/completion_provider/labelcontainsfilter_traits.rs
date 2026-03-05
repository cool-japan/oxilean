//! # LabelContainsFilter - Trait Implementations
//!
//! This module contains trait implementations for `LabelContainsFilter`.
//!
//! ## Implemented Traits
//!
//! - `CompletionFilter`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::CompletionFilter;
use super::types::{CompletionContext, CompletionItem, LabelContainsFilter};
use std::fmt;

impl CompletionFilter for LabelContainsFilter {
    fn accepts(&self, item: &CompletionItem, _ctx: &CompletionContext) -> bool {
        item.label.contains(self.substring.as_str())
    }
    fn filter_name(&self) -> &str {
        "label_contains_filter"
    }
}
