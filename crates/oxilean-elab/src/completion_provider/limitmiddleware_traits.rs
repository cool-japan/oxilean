//! # LimitMiddleware - Trait Implementations
//!
//! This module contains trait implementations for `LimitMiddleware`.
//!
//! ## Implemented Traits
//!
//! - `CompletionMiddleware`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::CompletionMiddleware;
use super::types::{CompletionContext, CompletionItem, LimitMiddleware};
use std::fmt;

impl CompletionMiddleware for LimitMiddleware {
    fn middleware_name(&self) -> &str {
        "limit_middleware"
    }
    fn post_complete(&self, _ctx: &CompletionContext, items: &mut Vec<CompletionItem>) {
        items.truncate(self.max_items);
    }
}
