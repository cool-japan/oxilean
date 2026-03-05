//! # SIMDTarget - Trait Implementations
//!
//! This module contains trait implementations for `SIMDTarget`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SIMDTarget;
use std::fmt;

impl fmt::Display for SIMDTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SIMDTarget::Generic => write!(f, "generic"),
            SIMDTarget::X86SSE => write!(f, "x86-sse"),
            SIMDTarget::X86AVX => write!(f, "x86-avx"),
            SIMDTarget::X86AVX512 => write!(f, "x86-avx512"),
            SIMDTarget::ArmNeon => write!(f, "arm-neon"),
            SIMDTarget::WasmSimd128 => write!(f, "wasm-simd128"),
        }
    }
}
