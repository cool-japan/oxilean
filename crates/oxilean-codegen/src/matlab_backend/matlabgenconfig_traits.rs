//! # MatlabGenConfig - Trait Implementations
//!
//! This module contains trait implementations for `MatlabGenConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MatlabGenConfig;

impl Default for MatlabGenConfig {
    fn default() -> Self {
        MatlabGenConfig {
            suppress_output: true,
            emit_section_markers: false,
            octave_compat: false,
            indent: "  ".to_string(),
            emit_function_end: true,
            prefer_anon_functions: true,
        }
    }
}
