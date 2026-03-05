//! # DoBlock - Trait Implementations
//!
//! This module contains trait implementations for `DoBlock`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DoBlock;
use std::fmt;

impl fmt::Display for DoBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "do")?;
        if let Some(monad) = &self.monad_type {
            write!(f, " [{:?}]", monad)?;
        }
        for elem in &self.elems {
            write!(f, "\n  {}", elem)?;
        }
        Ok(())
    }
}
