//! # CEmitConfig - Trait Implementations
//!
//! This module contains trait implementations for `CEmitConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::CEmitConfig;

impl Default for CEmitConfig {
    fn default() -> Self {
        CEmitConfig {
            emit_comments: true,
            inline_small: true,
            use_rc: true,
            indent: "  ".to_string(),
            module_name: "oxilean_module".to_string(),
            use_static: true,
        }
    }
}
