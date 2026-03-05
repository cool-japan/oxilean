//! # StateMutability - Trait Implementations
//!
//! This module contains trait implementations for `StateMutability`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::StateMutability;
use std::fmt;

impl fmt::Display for StateMutability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StateMutability::NonPayable => Ok(()),
            StateMutability::Payable => write!(f, "payable"),
            StateMutability::View => write!(f, "view"),
            StateMutability::Pure => write!(f, "pure"),
        }
    }
}
