//! # IdrisAttribute - Trait Implementations
//!
//! This module contains trait implementations for `IdrisAttribute`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{IdrisAttribute, Totality};
use std::fmt;

impl fmt::Display for IdrisAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdrisAttribute::Auto => write!(f, "auto"),
            IdrisAttribute::Interface => write!(f, "interface"),
            IdrisAttribute::Search => write!(f, "search"),
            IdrisAttribute::Totality(t) => write!(f, "{}", t),
            IdrisAttribute::Inline => write!(f, "inline"),
            IdrisAttribute::Static => write!(f, "static"),
        }
    }
}
