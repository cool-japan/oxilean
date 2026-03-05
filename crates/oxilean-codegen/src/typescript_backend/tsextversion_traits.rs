//! # TsExtVersion - Trait Implementations
//!
//! This module contains trait implementations for `TsExtVersion`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{format_ts_stmt, format_ts_stmts};
use super::types::TsExtVersion;
use std::fmt;

impl std::fmt::Display for TsExtVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if let Some(ref p) = self.pre {
            write!(f, "-{}", p)?;
        }
        Ok(())
    }
}
