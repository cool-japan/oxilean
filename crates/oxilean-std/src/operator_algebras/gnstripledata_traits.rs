//! # GNSTripleData - Trait Implementations
//!
//! This module contains trait implementations for `GNSTripleData`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::GNSTripleData;
use std::fmt;

impl std::fmt::Display for GNSTripleData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dim_str = match self.hilbert_dim {
            Some(n) => format!("dim={}", n),
            None => "dim=inf".to_string(),
        };
        write!(
            f,
            "GNS({}, {}, {}, irred={}, TT={})",
            self.algebra_name,
            self.state_name,
            dim_str,
            self.is_irreducible,
            self.tomita_takesaki_applies()
        )
    }
}
