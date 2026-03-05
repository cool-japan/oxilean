//! # TruncateStage - Trait Implementations
//!
//! This module contains trait implementations for `TruncateStage`.
//!
//! ## Implemented Traits
//!
//! - `CompletionStage`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::CompletionStage;
use super::types::{CompletionContext, CompletionItem, TruncateStage};
use std::fmt;

impl CompletionStage for TruncateStage {
    fn name(&self) -> &'static str {
        "truncate"
    }
    fn process(
        &self,
        _ctx: &CompletionContext,
        mut items: Vec<CompletionItem>,
    ) -> Vec<CompletionItem> {
        items.truncate(self.limit);
        items
    }
}
