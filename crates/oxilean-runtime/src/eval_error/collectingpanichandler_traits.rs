//! # CollectingPanicHandler - Trait Implementations
//!
//! This module contains trait implementations for `CollectingPanicHandler`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `PanicHandler`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::PanicHandler;
use super::types::{CollectingPanicHandler, EvalError};

impl Default for CollectingPanicHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl PanicHandler for CollectingPanicHandler {
    fn on_panic(&self, err: &EvalError) {
        self.panics
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .push(err.to_string());
    }
}
