//! # SilentPanicHandler - Trait Implementations
//!
//! This module contains trait implementations for `SilentPanicHandler`.
//!
//! ## Implemented Traits
//!
//! - `PanicHandler`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::PanicHandler;
use super::types::{EvalError, SilentPanicHandler};

impl PanicHandler for SilentPanicHandler {
    fn on_panic(&self, _err: &EvalError) {}
}
