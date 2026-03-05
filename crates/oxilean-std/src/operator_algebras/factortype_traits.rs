//! # FactorType - Trait Implementations
//!
//! This module contains trait implementations for `FactorType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FactorType;
use std::fmt;

impl std::fmt::Display for FactorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FactorType::TypeI(Some(n)) => write!(f, "I_{}", n),
            FactorType::TypeI(None) => write!(f, "I_inf"),
            FactorType::TypeII1 => write!(f, "II_1"),
            FactorType::TypeIIInfty => write!(f, "II_inf"),
            FactorType::TypeIII(lambda) => write!(f, "III_{:.3}", lambda),
            FactorType::TypeIII0 => write!(f, "III_0"),
            FactorType::TypeIII1 => write!(f, "III_1"),
        }
    }
}
