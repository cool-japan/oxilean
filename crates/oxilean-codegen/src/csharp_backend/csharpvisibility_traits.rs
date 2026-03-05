//! # CSharpVisibility - Trait Implementations
//!
//! This module contains trait implementations for `CSharpVisibility`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::CSharpVisibility;
use std::fmt;

impl fmt::Display for CSharpVisibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CSharpVisibility::Public => write!(f, "public"),
            CSharpVisibility::Private => write!(f, "private"),
            CSharpVisibility::Protected => write!(f, "protected"),
            CSharpVisibility::Internal => write!(f, "internal"),
            CSharpVisibility::ProtectedInternal => write!(f, "protected internal"),
            CSharpVisibility::PrivateProtected => write!(f, "private protected"),
        }
    }
}
