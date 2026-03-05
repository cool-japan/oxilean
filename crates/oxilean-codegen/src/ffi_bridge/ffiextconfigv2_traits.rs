//! # FfiExtConfigV2 - Trait Implementations
//!
//! This module contains trait implementations for `FfiExtConfigV2`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::{FfiCallingConv, FfiExtConfigV2, FfiPlatformTarget};

impl Default for FfiExtConfigV2 {
    fn default() -> Self {
        Self {
            platform: FfiPlatformTarget::Linux,
            emit_c_header: true,
            emit_rust_bindings: true,
            emit_python_cffi: false,
            emit_zig_bindings: false,
            header_guard_prefix: "OXILEAN_".to_string(),
            lib_name: "oxilean".to_string(),
            calling_conv_default: FfiCallingConv::C,
            enable_null_checks: true,
            enable_bounds_checks: false,
        }
    }
}
