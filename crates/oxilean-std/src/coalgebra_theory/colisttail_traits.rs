//! # CoListTail - Trait Implementations
//!
//! This module contains trait implementations for `CoListTail`.
//!
//! ## Implemented Traits
//!
//! - `Clone`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoListTail;

impl<A: Clone + 'static> Clone for CoListTail<A> {
    fn clone(&self) -> Self {
        let val = (self.0)();
        CoListTail(Box::new(move || val.clone()))
    }
}
