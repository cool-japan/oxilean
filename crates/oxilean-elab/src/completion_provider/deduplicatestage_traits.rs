//! # DeduplicateStage - Trait Implementations
//!
//! This module contains trait implementations for `DeduplicateStage`.
//!
//! ## Implemented Traits
//!
//! - `CompletionStage`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::CompletionStage;
use super::types::{CompletionContext, CompletionItem, DeduplicateStage};
use std::fmt;

impl CompletionStage for DeduplicateStage {
    fn name(&self) -> &'static str {
        "deduplicate"
    }
    fn process(&self, _ctx: &CompletionContext, items: Vec<CompletionItem>) -> Vec<CompletionItem> {
        let mut seen = std::collections::HashSet::new();
        items
            .into_iter()
            .filter(|item| seen.insert(item.label.clone()))
            .collect()
    }
}
