//! # RemoveDeprecatedStep - Trait Implementations
//!
//! This module contains trait implementations for `RemoveDeprecatedStep`.
//!
//! ## Implemented Traits
//!
//! - `CompletionPipelineStep`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::CompletionPipelineStep;
use super::types::{CompletionList, RemoveDeprecatedStep};
use std::fmt;

impl CompletionPipelineStep for RemoveDeprecatedStep {
    fn process(&self, mut list: CompletionList) -> CompletionList {
        list.items.retain(|i| !i.deprecated || i.preselect);
        list
    }
}
