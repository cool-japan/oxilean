//! # ScalaModifier - Trait Implementations
//!
//! This module contains trait implementations for `ScalaModifier`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ScalaModifier;
use std::fmt;

impl fmt::Display for ScalaModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScalaModifier::Private => write!(f, "private"),
            ScalaModifier::Protected => write!(f, "protected"),
            ScalaModifier::Override => write!(f, "override"),
            ScalaModifier::Final => write!(f, "final"),
            ScalaModifier::Abstract => write!(f, "abstract"),
            ScalaModifier::Implicit => write!(f, "implicit"),
            ScalaModifier::Inline => write!(f, "inline"),
            ScalaModifier::Lazy => write!(f, "lazy"),
            ScalaModifier::Given => write!(f, "given"),
            ScalaModifier::Extension => write!(f, "extension"),
        }
    }
}
