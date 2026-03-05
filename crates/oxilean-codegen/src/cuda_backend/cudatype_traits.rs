//! # CudaType - Trait Implementations
//!
//! This module contains trait implementations for `CudaType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CudaType;
use std::fmt;

impl fmt::Display for CudaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CudaType::Int => write!(f, "int"),
            CudaType::Long => write!(f, "long"),
            CudaType::Float => write!(f, "float"),
            CudaType::Double => write!(f, "double"),
            CudaType::Half => write!(f, "__half"),
            CudaType::Bool => write!(f, "bool"),
            CudaType::Dim3 => write!(f, "dim3"),
            CudaType::DimT => write!(f, "size_t"),
            CudaType::CudaErrorT => write!(f, "cudaError_t"),
            CudaType::Pointer(inner) => write!(f, "{}*", inner),
            CudaType::Shared(inner) => write!(f, "__shared__ {}", inner),
            CudaType::Constant(inner) => write!(f, "__constant__ {}", inner),
            CudaType::Device(inner) => write!(f, "__device__ {}", inner),
            CudaType::Void => write!(f, "void"),
            CudaType::UInt => write!(f, "unsigned int"),
            CudaType::Named(name) => write!(f, "{}", name),
        }
    }
}
