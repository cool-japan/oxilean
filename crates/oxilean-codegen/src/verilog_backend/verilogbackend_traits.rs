//! # VerilogBackend - Trait Implementations
//!
//! This module contains trait implementations for `VerilogBackend`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::VerilogBackend;

impl Default for VerilogBackend {
    fn default() -> Self {
        VerilogBackend::new(false)
    }
}
