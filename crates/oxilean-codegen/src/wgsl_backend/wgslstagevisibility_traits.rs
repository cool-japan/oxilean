//! # WGSLStageVisibility - Trait Implementations
//!
//! This module contains trait implementations for `WGSLStageVisibility`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WGSLStageVisibility;
use std::fmt;

impl fmt::Display for WGSLStageVisibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WGSLStageVisibility::Vertex => write!(f, "VERTEX"),
            WGSLStageVisibility::Fragment => write!(f, "FRAGMENT"),
            WGSLStageVisibility::Compute => write!(f, "COMPUTE"),
            WGSLStageVisibility::VertexFragment => write!(f, "VERTEX | FRAGMENT"),
            WGSLStageVisibility::All => write!(f, "VERTEX | FRAGMENT | COMPUTE"),
        }
    }
}
