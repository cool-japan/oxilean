//! # StderrPanicHandler - Trait Implementations
//!
//! This module contains trait implementations for `StderrPanicHandler`.
//!
//! ## Implemented Traits
//!
//! - `PanicHandler`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::PanicHandler;
use super::types::{EvalError, StderrPanicHandler};

impl PanicHandler for StderrPanicHandler {
    fn on_panic(&self, err: &EvalError) {
        eprintln!("[OxiLean runtime panic] {}", err);
    }
}
