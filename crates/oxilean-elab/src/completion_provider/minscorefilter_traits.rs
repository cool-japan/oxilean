//! # MinScoreFilter - Trait Implementations
//!
//! This module contains trait implementations for `MinScoreFilter`.
//!
//! ## Implemented Traits
//!
//! - `CompletionFilter`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::CompletionFilter;
use super::types::{CompletionContext, CompletionItem, MinScoreFilter};
use std::fmt;

impl CompletionFilter for MinScoreFilter {
    fn accepts(&self, item: &CompletionItem, _ctx: &CompletionContext) -> bool {
        item.score >= self.min_score
    }
    fn filter_name(&self) -> &str {
        "min_score_filter"
    }
}
