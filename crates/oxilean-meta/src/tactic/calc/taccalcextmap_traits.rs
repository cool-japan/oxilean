//! # TacCalcExtMap - Trait Implementations
//!
//! This module contains trait implementations for `TacCalcExtMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TacCalcExtMap;

impl<V: Clone + Default> Default for TacCalcExtMap<V> {
    fn default() -> Self {
        Self::new()
    }
}
