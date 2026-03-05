//! # StrengthReduceRule - Trait Implementations
//!
//! This module contains trait implementations for `StrengthReduceRule`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::StrengthReduceRule;
use std::fmt;

impl fmt::Display for StrengthReduceRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StrengthReduceRule::MulByPow2(n) => write!(f, "MulByPow2({})", n),
            StrengthReduceRule::DivByPow2(n) => write!(f, "DivByPow2({})", n),
            StrengthReduceRule::ModByPow2(n) => write!(f, "ModByPow2({})", n),
            StrengthReduceRule::MulByConstant(c) => write!(f, "MulByConstant({})", c),
            StrengthReduceRule::DivByConstant(c) => write!(f, "DivByConstant({})", c),
            StrengthReduceRule::Pow2Const => write!(f, "Pow2Const"),
            StrengthReduceRule::Pow3Const => write!(f, "Pow3Const"),
            StrengthReduceRule::NegToSub => write!(f, "NegToSub"),
            StrengthReduceRule::AddSubToInc => write!(f, "AddSubToInc"),
        }
    }
}
