//! # FfiFuncSignature - Trait Implementations
//!
//! This module contains trait implementations for `FfiFuncSignature`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FfiFuncSignature;
use std::fmt;

impl std::fmt::Display for FfiFuncSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<String> = self.params.iter().map(|p| p.to_string()).collect();
        let noexcept = if self.is_noexcept { " noexcept" } else { "" };
        write!(
            f,
            "{} {}{}({}){}",
            self.ret_type,
            self.calling_conv,
            self.name,
            params.join(", "),
            noexcept
        )
    }
}
