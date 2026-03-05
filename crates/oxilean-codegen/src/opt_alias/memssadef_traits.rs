//! # MemSsaDef - Trait Implementations
//!
//! This module contains trait implementations for `MemSsaDef`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{MemSsaDef, MemSsaKind};
use std::fmt;

impl std::fmt::Display for MemSsaDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = match self.kind {
            MemSsaKind::MemDef => "MemDef",
            MemSsaKind::MemPhi => "MemPhi",
            MemSsaKind::MemUse => "MemUse",
            MemSsaKind::LiveOnEntry => "LiveOnEntry",
        };
        write!(f, "{}{}@{}", kind, self.version, self.id)
    }
}
