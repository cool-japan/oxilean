//! # EqDecision - Trait Implementations
//!
//! This module contains trait implementations for `EqDecision`.
//!
//! ## Implemented Traits
//!
//! - `Decidable`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::Decidable;
use super::types::{Decision, EqDecision};
use std::fmt;

impl<T: PartialEq + std::fmt::Debug> Decidable for EqDecision<T> {
    type Proof = ();
    fn decide(&self) -> Decision<()> {
        if self.lhs == self.rhs {
            Decision::IsTrue(())
        } else {
            Decision::IsFalse(format!("{:?} ≠ {:?}", self.lhs, self.rhs))
        }
    }
}
