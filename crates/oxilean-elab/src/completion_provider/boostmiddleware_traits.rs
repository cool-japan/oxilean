//! # BoostMiddleware - Trait Implementations
//!
//! This module contains trait implementations for `BoostMiddleware`.
//!
//! ## Implemented Traits
//!
//! - `CompletionMiddleware`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::CompletionMiddleware;
use super::types::{BoostMiddleware, CompletionContext, CompletionItem};
use std::fmt;

impl CompletionMiddleware for BoostMiddleware {
    fn middleware_name(&self) -> &str {
        "boost_middleware"
    }
    fn post_complete(&self, _ctx: &CompletionContext, items: &mut Vec<CompletionItem>) {
        for item in items.iter_mut() {
            if item.label.contains(self.pattern.as_str()) {
                item.score += self.boost;
            }
        }
    }
}
