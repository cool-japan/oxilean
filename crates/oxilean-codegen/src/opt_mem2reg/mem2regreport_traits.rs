//! # Mem2RegReport - Trait Implementations
//!
//! This module contains trait implementations for `Mem2RegReport`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::Mem2RegReport;
use std::fmt;

impl fmt::Display for Mem2RegReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Mem2RegReport {{ bindings_promoted={}, phi_nodes_inserted={} }}",
            self.bindings_promoted, self.phi_nodes_inserted
        )
    }
}
