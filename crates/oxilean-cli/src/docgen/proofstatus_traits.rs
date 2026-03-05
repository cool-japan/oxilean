//! # ProofStatus - Trait Implementations
//!
//! This module contains trait implementations for `ProofStatus`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ProofStatus;
use std::fmt;

impl fmt::Display for ProofStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProofStatus::Proved => write!(f, "proved"),
            ProofStatus::Sorry => write!(f, "sorry"),
            ProofStatus::Axiom => write!(f, "axiom"),
        }
    }
}
