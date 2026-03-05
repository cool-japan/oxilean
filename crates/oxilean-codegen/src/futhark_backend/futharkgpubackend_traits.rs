//! # FutharkGpuBackend - Trait Implementations
//!
//! This module contains trait implementations for `FutharkGpuBackend`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkGpuBackend;
use std::fmt;

impl std::fmt::Display for FutharkGpuBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FutharkGpuBackend::OpenCL => write!(f, "opencl"),
            FutharkGpuBackend::CUDA => write!(f, "cuda"),
            FutharkGpuBackend::Hip => write!(f, "hip"),
            FutharkGpuBackend::Sequential => write!(f, "c"),
            FutharkGpuBackend::Multicore => write!(f, "multicore"),
            FutharkGpuBackend::IsPC => write!(f, "ispc"),
            FutharkGpuBackend::WGpu => write!(f, "wgpu"),
        }
    }
}
