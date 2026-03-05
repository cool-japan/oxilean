//! # EvmAbiFunction - Trait Implementations
//!
//! This module contains trait implementations for `EvmAbiFunction`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::EvmAbiFunction;
use std::fmt;

impl std::fmt::Display for EvmAbiFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inputs: Vec<String> = self
            .inputs
            .iter()
            .map(|(n, t)| format!("{} {}", t, n))
            .collect();
        let outputs: Vec<String> = self
            .outputs
            .iter()
            .map(|(n, t)| format!("{} {}", t, n))
            .collect();
        let mut mods = Vec::new();
        if self.is_pure {
            mods.push("pure");
        } else if self.is_view {
            mods.push("view");
        }
        if self.is_payable {
            mods.push("payable");
        }
        write!(
            f,
            "function {}({}) {} returns ({})",
            self.name,
            inputs.join(", "),
            mods.join(" "),
            outputs.join(", ")
        )
    }
}
