//! # SystemFType - Trait Implementations
//!
//! This module contains trait implementations for `SystemFType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SystemFType;
use std::fmt;

impl std::fmt::Display for SystemFType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemFType::TyVar(v) => write!(f, "{v}"),
            SystemFType::Forall(v, body) => write!(f, "∀{v}. {body}"),
            SystemFType::Fun(a, b) => write!(f, "({a} → {b})"),
            SystemFType::Base(n) => write!(f, "{n}"),
        }
    }
}
