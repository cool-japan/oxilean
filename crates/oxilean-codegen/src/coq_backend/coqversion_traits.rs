//! # CoqVersion - Trait Implementations
//!
//! This module contains trait implementations for `CoqVersion`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoqVersion;
use std::fmt;

impl std::fmt::Display for CoqVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoqVersion::V8_14 => write!(f, "8.14"),
            CoqVersion::V8_15 => write!(f, "8.15"),
            CoqVersion::V8_16 => write!(f, "8.16"),
            CoqVersion::V8_17 => write!(f, "8.17"),
            CoqVersion::V8_18 => write!(f, "8.18"),
            CoqVersion::V8_19 => write!(f, "8.19"),
            CoqVersion::V8_20 => write!(f, "8.20"),
            CoqVersion::Rocq0_1 => write!(f, "Rocq 0.1"),
            CoqVersion::Latest => write!(f, "latest"),
        }
    }
}
