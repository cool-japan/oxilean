//! # DynkinDiagram - Trait Implementations
//!
//! This module contains trait implementations for `DynkinDiagram`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DynkinDiagram;
use std::fmt;

impl std::fmt::Display for DynkinDiagram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DynkinDiagram({:?}, simply_laced={})",
            self.kind,
            self.is_simply_laced()
        )
    }
}
