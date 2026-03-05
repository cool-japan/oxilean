//! # AlgebraicSimplifier - Trait Implementations
//!
//! This module contains trait implementations for `AlgebraicSimplifier`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::{AlgRule, AlgebraicSimplifier};

impl Default for AlgebraicSimplifier {
    fn default() -> Self {
        let mut s = AlgebraicSimplifier {
            rules: Vec::new(),
            total_simplified: 0,
        };
        s.rules.push(AlgRule::new("add_zero"));
        s.rules.push(AlgRule::new("mul_one"));
        s.rules.push(AlgRule::new("sub_self"));
        s.rules.push(AlgRule::new("eq_self"));
        s
    }
}
