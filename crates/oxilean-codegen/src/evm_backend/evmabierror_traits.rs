//! # EvmAbiError - Trait Implementations
//!
//! This module contains trait implementations for `EvmAbiError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::EvmAbiError;
use std::fmt;

impl std::fmt::Display for EvmAbiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inputs: Vec<String> = self
            .inputs
            .iter()
            .map(|(n, t)| format!("{} {}", t, n))
            .collect();
        write!(f, "error {}({})", self.name, inputs.join(", "))
    }
}
