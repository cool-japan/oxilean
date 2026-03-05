//! # MetalStage - Trait Implementations
//!
//! This module contains trait implementations for `MetalStage`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MetalStage;
use std::fmt;

impl fmt::Display for MetalStage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MetalStage::Vertex => write!(f, "[[vertex]]"),
            MetalStage::Fragment => write!(f, "[[fragment]]"),
            MetalStage::Kernel => write!(f, "[[kernel]]"),
            MetalStage::Mesh => write!(f, "[[mesh]]"),
            MetalStage::Device => write!(f, ""),
        }
    }
}
