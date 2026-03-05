//! # SortByScoreStage - Trait Implementations
//!
//! This module contains trait implementations for `SortByScoreStage`.
//!
//! ## Implemented Traits
//!
//! - `CompletionStage`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::CompletionStage;
use super::types::{CompletionContext, CompletionItem, SortByScoreStage};
use std::fmt;

impl CompletionStage for SortByScoreStage {
    fn name(&self) -> &'static str {
        "sort_by_score"
    }
    fn process(
        &self,
        _ctx: &CompletionContext,
        mut items: Vec<CompletionItem>,
    ) -> Vec<CompletionItem> {
        items.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        items
    }
}
