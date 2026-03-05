//! # MetalParamAttr - Trait Implementations
//!
//! This module contains trait implementations for `MetalParamAttr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MetalParamAttr;
use std::fmt;

impl fmt::Display for MetalParamAttr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MetalParamAttr::Buffer(i) => write!(f, " [[buffer({})]]", i),
            MetalParamAttr::Texture(i) => write!(f, " [[texture({})]]", i),
            MetalParamAttr::Sampler(i) => write!(f, " [[sampler({})]]", i),
            MetalParamAttr::StageIn => write!(f, " [[stage_in]]"),
            MetalParamAttr::Builtin(b) => write!(f, " {}", b.attribute()),
            MetalParamAttr::None => write!(f, ""),
        }
    }
}
