//! # WGSLComputeKernelParams - Trait Implementations
//!
//! This module contains trait implementations for `WGSLComputeKernelParams`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WGSLComputeKernelParams;

impl Default for WGSLComputeKernelParams {
    fn default() -> Self {
        WGSLComputeKernelParams {
            name: "main".to_string(),
            wg_x: 64,
            wg_y: 1,
            wg_z: 1,
            use_local_id: false,
            use_workgroup_id: false,
            use_num_workgroups: false,
        }
    }
}
