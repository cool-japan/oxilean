//! # ComonadCtx - Trait Implementations
//!
//! This module contains trait implementations for `ComonadCtx`.
//!
//! ## Implemented Traits
//!
//! - `Clone`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ComonadCtx;

impl<E: Clone, A: Clone> Clone for ComonadCtx<E, A> {
    fn clone(&self) -> Self {
        Self {
            env: self.env.clone(),
            focus: self.focus.clone(),
        }
    }
}
