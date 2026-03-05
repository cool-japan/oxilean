//! # ApplyVariant - Trait Implementations
//!
//! This module contains trait implementations for `ApplyVariant`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ApplyVariant;
use std::fmt;

impl fmt::Display for ApplyVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApplyVariant::Sapply => write!(f, "sapply"),
            ApplyVariant::Vapply => write!(f, "vapply"),
            ApplyVariant::Lapply => write!(f, "lapply"),
            ApplyVariant::Apply => write!(f, "apply"),
            ApplyVariant::Tapply => write!(f, "tapply"),
            ApplyVariant::Mapply => write!(f, "mapply"),
        }
    }
}
