//! # Mem2RegConfig - Trait Implementations
//!
//! This module contains trait implementations for `Mem2RegConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::Mem2RegConfig;
use std::fmt;

impl Default for Mem2RegConfig {
    fn default() -> Self {
        Mem2RegConfig {
            max_phi_nodes: 64,
            conservative: false,
        }
    }
}

impl fmt::Display for Mem2RegConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Mem2RegConfig {{ max_phi_nodes={}, conservative={} }}",
            self.max_phi_nodes, self.conservative
        )
    }
}
