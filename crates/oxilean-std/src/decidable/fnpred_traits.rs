//! # FnPred - Trait Implementations
//!
//! This module contains trait implementations for `FnPred`.
//!
//! ## Implemented Traits
//!
//! - `DecidablePred`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::DecidablePred;
use super::types::{Decision, FnPred};

impl<A, F: Fn(&A) -> bool> DecidablePred<A> for FnPred<A, F> {
    fn decide_pred(&self, a: &A) -> Decision<()> {
        if (self.0)(a) {
            Decision::IsTrue(())
        } else {
            Decision::IsFalse("predicate returned false".to_string())
        }
    }
}
