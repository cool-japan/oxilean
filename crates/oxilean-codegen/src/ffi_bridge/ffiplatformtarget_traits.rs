//! # FfiPlatformTarget - Trait Implementations
//!
//! This module contains trait implementations for `FfiPlatformTarget`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FfiPlatformTarget;
use std::fmt;

impl std::fmt::Display for FfiPlatformTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FfiPlatformTarget::Linux => write!(f, "linux"),
            FfiPlatformTarget::Windows => write!(f, "windows"),
            FfiPlatformTarget::MacOS => write!(f, "macos"),
            FfiPlatformTarget::FreeBSD => write!(f, "freebsd"),
            FfiPlatformTarget::Wasm32 => write!(f, "wasm32"),
            FfiPlatformTarget::Wasm64 => write!(f, "wasm64"),
            FfiPlatformTarget::Android => write!(f, "android"),
            FfiPlatformTarget::Ios => write!(f, "ios"),
            FfiPlatformTarget::Universal => write!(f, "universal"),
        }
    }
}
