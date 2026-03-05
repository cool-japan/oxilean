//! # LimitStep - Trait Implementations
//!
//! This module contains trait implementations for `LimitStep`.
//!
//! ## Implemented Traits
//!
//! - `CompletionPipelineStep`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::CompletionPipelineStep;
use super::types::{CompletionList, LimitStep};
use std::fmt;

impl CompletionPipelineStep for LimitStep {
    fn process(&self, mut list: CompletionList) -> CompletionList {
        if list.items.len() > self.max_items {
            list.items.truncate(self.max_items);
            list.is_incomplete = true;
        }
        list
    }
}
