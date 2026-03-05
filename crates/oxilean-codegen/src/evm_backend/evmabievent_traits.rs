//! # EvmAbiEvent - Trait Implementations
//!
//! This module contains trait implementations for `EvmAbiEvent`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::EvmAbiEvent;
use std::fmt;

impl std::fmt::Display for EvmAbiEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inputs: Vec<String> = self
            .inputs
            .iter()
            .map(|(n, t, idx)| {
                if *idx {
                    format!("{} indexed {}", t, n)
                } else {
                    format!("{} {}", t, n)
                }
            })
            .collect();
        let anon = if self.is_anonymous { " anonymous" } else { "" };
        write!(f, "event {}({}){}", self.name, inputs.join(", "), anon)
    }
}
