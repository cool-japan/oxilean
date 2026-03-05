//! # StateData - Trait Implementations
//!
//! This module contains trait implementations for `StateData`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::StateData;
use std::fmt;

impl std::fmt::Display for StateData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kms_str = if let Some(beta) = self.beta {
            format!(", beta={:.2}", beta)
        } else {
            String::new()
        };
        write!(
            f,
            "State[{}](faithful={}, tracial={}, pure={}{})",
            self.name, self.is_faithful, self.is_tracial, self.is_pure, kms_str
        )
    }
}
