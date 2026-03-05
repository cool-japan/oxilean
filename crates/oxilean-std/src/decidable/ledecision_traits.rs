//! # LeDecision - Trait Implementations
//!
//! This module contains trait implementations for `LeDecision`.
//!
//! ## Implemented Traits
//!
//! - `Decidable`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::Decidable;
use super::types::{Decision, LeDecision};

impl<T: PartialOrd> Decidable for LeDecision<T> {
    type Proof = ();
    fn decide(&self) -> Decision<()> {
        if self.lhs <= self.rhs {
            Decision::IsTrue(())
        } else {
            Decision::IsFalse("not ≤".to_string())
        }
    }
}
