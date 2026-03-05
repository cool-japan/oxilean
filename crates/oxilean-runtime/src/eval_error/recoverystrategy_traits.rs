//! # RecoveryStrategy - Trait Implementations
//!
//! This module contains trait implementations for `RecoveryStrategy`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RecoveryStrategy;
use std::fmt;

impl std::fmt::Display for RecoveryStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecoveryStrategy::Abort => write!(f, "abort"),
            RecoveryStrategy::ReturnDefault => write!(f, "return-default"),
            RecoveryStrategy::Retry { max_attempts } => {
                write!(f, "retry(max={})", max_attempts)
            }
            RecoveryStrategy::FallbackToSorry => write!(f, "fallback-to-sorry"),
            RecoveryStrategy::LogAndContinue => write!(f, "log-and-continue"),
        }
    }
}
