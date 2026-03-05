//! # CStarAlgebraData - Trait Implementations
//!
//! This module contains trait implementations for `CStarAlgebraData`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CStarAlgebraData;
use std::fmt;

impl std::fmt::Display for CStarAlgebraData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dim_str = match self.dimension {
            Some(n) => format!("dim={}", n),
            None => "dim=inf".to_string(),
        };
        write!(
            f,
            "C*({}, {}, commutative={}, nuclear={}, simple={})",
            self.name, dim_str, self.is_commutative, self.is_nuclear, self.is_simple
        )
    }
}
