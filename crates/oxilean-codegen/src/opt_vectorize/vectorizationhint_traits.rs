//! # VectorizationHint - Trait Implementations
//!
//! This module contains trait implementations for `VectorizationHint`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::VectorizationHint;
use std::fmt;

impl fmt::Display for VectorizationHint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VectorizationHint::Force => write!(f, "#[vectorize(force)]"),
            VectorizationHint::Disable => write!(f, "#[vectorize(disable)]"),
            VectorizationHint::Unroll(n) => write!(f, "#[vectorize(unroll={})]", n),
            VectorizationHint::Width(w) => write!(f, "#[vectorize(width={})]", w.bits()),
            VectorizationHint::NoAlias => write!(f, "#[vectorize(noalias)]"),
            VectorizationHint::Aligned => write!(f, "#[vectorize(aligned)]"),
        }
    }
}
