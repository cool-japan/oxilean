//! # FfiRustBindingKind - Trait Implementations
//!
//! This module contains trait implementations for `FfiRustBindingKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FfiRustBindingKind;
use std::fmt;

impl std::fmt::Display for FfiRustBindingKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FfiRustBindingKind::Function => write!(f, "fn"),
            FfiRustBindingKind::Struct => write!(f, "struct"),
            FfiRustBindingKind::Enum => write!(f, "enum"),
            FfiRustBindingKind::Type => write!(f, "type"),
            FfiRustBindingKind::Const => write!(f, "const"),
        }
    }
}
