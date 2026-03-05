//! # KindFilter - Trait Implementations
//!
//! This module contains trait implementations for `KindFilter`.
//!
//! ## Implemented Traits
//!
//! - `CompletionFilter`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::CompletionFilter;
use super::types::{CompletionContext, CompletionItem, KindFilter};
use std::fmt;

impl CompletionFilter for KindFilter {
    fn accepts(&self, item: &CompletionItem, _ctx: &CompletionContext) -> bool {
        self.allowed.contains(&item.kind)
    }
    fn filter_name(&self) -> &str {
        "kind_filter"
    }
}
