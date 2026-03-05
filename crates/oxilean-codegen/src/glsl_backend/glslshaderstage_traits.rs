//! # GLSLShaderStage - Trait Implementations
//!
//! This module contains trait implementations for `GLSLShaderStage`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::GLSLShaderStage;
use std::fmt;

impl fmt::Display for GLSLShaderStage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GLSLShaderStage::Vertex => write!(f, "vertex"),
            GLSLShaderStage::Fragment => write!(f, "fragment"),
            GLSLShaderStage::Geometry => write!(f, "geometry"),
            GLSLShaderStage::TessControl => write!(f, "tessellation control"),
            GLSLShaderStage::TessEval => write!(f, "tessellation evaluation"),
            GLSLShaderStage::Compute => write!(f, "compute"),
        }
    }
}
