//! # BinderKind - Trait Implementations
//!
//! This module contains trait implementations for `BinderKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BinderKind;
use std::fmt;

impl fmt::Display for BinderKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinderKind::Explicit => write!(f, "explicit"),
            BinderKind::Implicit => write!(f, "implicit"),
            BinderKind::StrictImplicit => write!(f, "strict implicit"),
            BinderKind::Instance => write!(f, "instance"),
        }
    }
}
