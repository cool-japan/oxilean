//! # AttributeKind - Trait Implementations
//!
//! This module contains trait implementations for `AttributeKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AttributeKind;
use std::fmt;

impl fmt::Display for AttributeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttributeKind::Simp => write!(f, "simp"),
            AttributeKind::Ext => write!(f, "ext"),
            AttributeKind::Instance => write!(f, "instance"),
            AttributeKind::Reducible => write!(f, "reducible"),
            AttributeKind::Irreducible => write!(f, "irreducible"),
            AttributeKind::Inline => write!(f, "inline"),
            AttributeKind::NoInline => write!(f, "noinline"),
            AttributeKind::SpecializeAttr => write!(f, "specialize"),
            AttributeKind::Custom(s) => write!(f, "{}", s),
        }
    }
}
