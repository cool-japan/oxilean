//! # HahnSeriesRepr - Trait Implementations
//!
//! This module contains trait implementations for `HahnSeriesRepr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::HahnSeriesRepr;
use std::fmt;

impl std::fmt::Display for HahnSeriesRepr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.terms.is_empty() {
            return write!(f, "0");
        }
        let parts: Vec<String> = self
            .terms
            .iter()
            .map(|(e, c)| {
                if (*e - 0.0).abs() < 1e-14 {
                    format!("{:.4}", c)
                } else {
                    format!("{:.4}·x^{:.4}", c, e)
                }
            })
            .collect();
        write!(f, "{}", parts.join(" + "))
    }
}
