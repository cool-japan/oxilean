//! # ClassModifier - Trait Implementations
//!
//! This module contains trait implementations for `ClassModifier`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ClassModifier;
use std::fmt;

impl fmt::Display for ClassModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClassModifier::Sealed => write!(f, "sealed"),
            ClassModifier::Abstract => write!(f, "abstract"),
            ClassModifier::Final => write!(f, "final"),
            ClassModifier::Static => write!(f, "static"),
            ClassModifier::NonSealed => write!(f, "non-sealed"),
        }
    }
}
