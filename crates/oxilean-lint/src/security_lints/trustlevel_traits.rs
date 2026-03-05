//! # TrustLevel - Trait Implementations
//!
//! This module contains trait implementations for `TrustLevel`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TrustLevel;

impl std::fmt::Display for TrustLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrustLevel::Verified => write!(f, "verified"),
            TrustLevel::Reviewed => write!(f, "reviewed"),
            TrustLevel::Untrusted => write!(f, "untrusted"),
            TrustLevel::Compromised => write!(f, "compromised"),
        }
    }
}

