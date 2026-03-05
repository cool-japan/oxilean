//! # CudaQualifier - Trait Implementations
//!
//! This module contains trait implementations for `CudaQualifier`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CudaQualifier;
use std::fmt;

impl fmt::Display for CudaQualifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CudaQualifier::Global => write!(f, "__global__"),
            CudaQualifier::Device => write!(f, "__device__"),
            CudaQualifier::Host => write!(f, "__host__"),
            CudaQualifier::Shared => write!(f, "__shared__"),
            CudaQualifier::Constant => write!(f, "__constant__"),
            CudaQualifier::Managed => write!(f, "__managed__"),
            CudaQualifier::Restrict => write!(f, "__restrict__"),
            CudaQualifier::Volatile => write!(f, "volatile"),
        }
    }
}
