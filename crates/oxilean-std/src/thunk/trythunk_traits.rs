//! # TryThunk - Trait Implementations
//!
//! This module contains trait implementations for `TryThunk`.
//!
//! ## Implemented Traits
//!
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TryThunk;
use std::fmt;

impl<T: std::fmt::Debug, E: std::fmt::Debug> std::fmt::Debug for TryThunk<T, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            Some(v) => write!(f, "TryThunk::Forced({:?})", v),
            None => write!(f, "TryThunk::Pending"),
        }
    }
}
