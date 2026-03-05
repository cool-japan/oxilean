//! # FutharkVersion - Trait Implementations
//!
//! This module contains trait implementations for `FutharkVersion`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkVersion;
use std::fmt;

impl std::fmt::Display for FutharkVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FutharkVersion::V020 => write!(f, "0.20"),
            FutharkVersion::V021 => write!(f, "0.21"),
            FutharkVersion::V022 => write!(f, "0.22"),
            FutharkVersion::V023 => write!(f, "0.23"),
            FutharkVersion::V024 => write!(f, "0.24"),
            FutharkVersion::V025 => write!(f, "0.25"),
            FutharkVersion::Latest => write!(f, "latest"),
        }
    }
}
