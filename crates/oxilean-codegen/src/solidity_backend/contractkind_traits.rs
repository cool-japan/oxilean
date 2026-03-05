//! # ContractKind - Trait Implementations
//!
//! This module contains trait implementations for `ContractKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ContractKind;
use std::fmt;

impl fmt::Display for ContractKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContractKind::Contract => write!(f, "contract"),
            ContractKind::Abstract => write!(f, "abstract contract"),
            ContractKind::Interface => write!(f, "interface"),
            ContractKind::Library => write!(f, "library"),
        }
    }
}
