//! # TailCall - Trait Implementations
//!
//! This module contains trait implementations for `TailCall`.
//!
//! ## Implemented Traits
//!
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TailCall;
use std::fmt;

impl<T: fmt::Debug> fmt::Debug for TailCall<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TailCall::Done(v) => write!(f, "Done({:?})", v),
            TailCall::Call(_) => write!(f, "Call(<fn>)"),
        }
    }
}
