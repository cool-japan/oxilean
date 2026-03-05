//! # OnceCellThunk - Trait Implementations
//!
//! This module contains trait implementations for `OnceCellThunk`.
//!
//! ## Implemented Traits
//!
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::OnceCellThunk;
use std::fmt;

impl<T: std::fmt::Debug> std::fmt::Debug for OnceCellThunk<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.cell.get() {
            Some(v) => write!(f, "OnceCellThunk::Forced({:?})", v),
            None => write!(f, "OnceCellThunk::Pending"),
        }
    }
}
