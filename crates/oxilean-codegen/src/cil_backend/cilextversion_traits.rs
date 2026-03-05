//! # CilExtVersion - Trait Implementations
//!
//! This module contains trait implementations for `CilExtVersion`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::CilExtVersion;
use std::fmt;

impl std::fmt::Display for CilExtVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if let Some(ref p) = self.pre {
            write!(f, "-{}", p)?;
        }
        Ok(())
    }
}
