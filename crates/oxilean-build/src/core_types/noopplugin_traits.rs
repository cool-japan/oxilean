//! # NoopPlugin - Trait Implementations
//!
//! This module contains trait implementations for `NoopPlugin`.
//!
//! ## Implemented Traits
//!
//! - `BuildPlugin`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::BuildPlugin;
use super::types::NoopPlugin;

impl BuildPlugin for NoopPlugin {
    fn name(&self) -> &str {
        &self.name
    }
}
