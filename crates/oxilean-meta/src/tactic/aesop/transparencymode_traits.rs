//! # TransparencyMode - Trait Implementations
//!
//! This module contains trait implementations for `TransparencyMode`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TransparencyMode;

impl std::fmt::Display for TransparencyMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransparencyMode::Reducible => write!(f, "reducible"),
            TransparencyMode::Default => write!(f, "default"),
            TransparencyMode::All => write!(f, "all"),
        }
    }
}
