//! # TautoTactic - Trait Implementations
//!
//! This module contains trait implementations for `TautoTactic`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TautoTactic;

impl Default for TautoTactic {
    fn default() -> Self {
        Self { max_atoms: 20 }
    }
}
