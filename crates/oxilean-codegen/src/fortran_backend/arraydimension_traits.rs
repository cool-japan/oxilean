//! # ArrayDimension - Trait Implementations
//!
//! This module contains trait implementations for `ArrayDimension`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ArrayDimension;
use std::fmt;

impl fmt::Display for ArrayDimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArrayDimension::Explicit(n) => write!(f, "{}", n),
            ArrayDimension::Deferred => write!(f, ":"),
            ArrayDimension::Assumed => write!(f, "*"),
            ArrayDimension::Multi(dims) => {
                let s: Vec<String> = dims.iter().map(|d| d.to_string()).collect();
                write!(f, "{}", s.join(", "))
            }
        }
    }
}
