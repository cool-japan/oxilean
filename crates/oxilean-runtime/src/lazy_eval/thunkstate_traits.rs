//! # ThunkState - Trait Implementations
//!
//! This module contains trait implementations for `ThunkState`.
//!
//! ## Implemented Traits
//!
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ThunkState;
use std::fmt;

impl<T: fmt::Debug> fmt::Debug for ThunkState<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThunkState::Unevaluated(_) => write!(f, "Unevaluated(<fn>)"),
            ThunkState::Evaluated(v) => write!(f, "Evaluated({:?})", v),
            ThunkState::BlackHole => write!(f, "BlackHole"),
        }
    }
}
