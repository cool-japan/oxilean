//! # KTheoryData - Trait Implementations
//!
//! This module contains trait implementations for `KTheoryData`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::KTheoryData;
use std::fmt;

impl std::fmt::Display for KTheoryData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn group_str(summands: &[u64]) -> String {
            if summands.is_empty() {
                return "0".to_string();
            }
            summands
                .iter()
                .map(|&s| {
                    if s == 0 {
                        "Z".to_string()
                    } else {
                        format!("Z/{}", s)
                    }
                })
                .collect::<Vec<_>>()
                .join(" + ")
        }
        write!(
            f,
            "K({})  K0={},  K1={}",
            self.algebra_name,
            group_str(&self.k0_summands),
            group_str(&self.k1_summands)
        )
    }
}
