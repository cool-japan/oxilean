//! # FfiCallbackType - Trait Implementations
//!
//! This module contains trait implementations for `FfiCallbackType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FfiCallbackType;
use std::fmt;

impl std::fmt::Display for FfiCallbackType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<String> = self.params.iter().map(|p| p.to_string()).collect();
        write!(
            f,
            "typedef {} ({}*{})({})",
            self.ret_type,
            self.calling_conv,
            self.name,
            params.join(", ")
        )
    }
}
