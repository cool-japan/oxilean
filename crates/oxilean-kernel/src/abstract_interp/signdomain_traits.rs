//! # SignDomain - Trait Implementations
//!
//! This module contains trait implementations for `SignDomain`.
//!
//! ## Implemented Traits
//!
//! - `AbstractDomain`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::AbstractDomain;
use super::types::SignDomain;

impl AbstractDomain for SignDomain {
    fn join(&self, other: &Self) -> Self {
        use SignDomain::*;
        if self == other {
            return *self;
        }
        match (self, other) {
            (Bottom, x) | (x, Bottom) => *x,
            (Top, _) | (_, Top) => Top,
            (Neg, Zero) | (Zero, Neg) => NonPos,
            (Pos, Zero) | (Zero, Pos) => NonNeg,
            (Neg, Pos) | (Pos, Neg) => Nonzero,
            (Neg, NonPos) | (NonPos, Neg) => NonPos,
            (Pos, NonNeg) | (NonNeg, Pos) => NonNeg,
            (Zero, Nonzero) | (Nonzero, Zero) => Top,
            (NonPos, Pos) | (Pos, NonPos) => Top,
            (NonNeg, Neg) | (Neg, NonNeg) => Top,
            (NonPos, NonNeg) | (NonNeg, NonPos) => Top,
            (NonPos, Nonzero) | (Nonzero, NonPos) => Top,
            (NonNeg, Nonzero) | (Nonzero, NonNeg) => Top,
            (Zero, NonPos) | (NonPos, Zero) => NonPos,
            (Zero, NonNeg) | (NonNeg, Zero) => NonNeg,
            _ => Top,
        }
    }
    fn meet(&self, other: &Self) -> Self {
        use SignDomain::*;
        if self == other {
            return *self;
        }
        match (self, other) {
            (Top, x) | (x, Top) => *x,
            (Bottom, _) | (_, Bottom) => Bottom,
            (NonNeg, NonPos) | (NonPos, NonNeg) => Zero,
            (NonNeg, Nonzero) | (Nonzero, NonNeg) => Pos,
            (NonPos, Nonzero) | (Nonzero, NonPos) => Neg,
            (NonNeg, Neg) | (Neg, NonNeg) => Bottom,
            (NonPos, Pos) | (Pos, NonPos) => Bottom,
            (Nonzero, Zero) | (Zero, Nonzero) => Bottom,
            (NonNeg, Pos) | (Pos, NonNeg) => Pos,
            (NonNeg, Zero) | (Zero, NonNeg) => Zero,
            (NonPos, Neg) | (Neg, NonPos) => Neg,
            (NonPos, Zero) | (Zero, NonPos) => Zero,
            _ => Bottom,
        }
    }
    fn is_bottom(&self) -> bool {
        matches!(self, SignDomain::Bottom)
    }
    fn is_top(&self) -> bool {
        matches!(self, SignDomain::Top)
    }
    fn leq(&self, other: &Self) -> bool {
        self.join(other) == *other
    }
}
