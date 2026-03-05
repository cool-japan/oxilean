//! # LlvmLinkage - Trait Implementations
//!
//! This module contains trait implementations for `LlvmLinkage`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::LlvmLinkage;
use std::fmt;

impl fmt::Display for LlvmLinkage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LlvmLinkage::Private => write!(f, "private"),
            LlvmLinkage::Internal => write!(f, "internal"),
            LlvmLinkage::External => write!(f, "external"),
            LlvmLinkage::LinkOnce => write!(f, "linkonce"),
            LlvmLinkage::Weak => write!(f, "weak"),
            LlvmLinkage::Common => write!(f, "common"),
            LlvmLinkage::Appending => write!(f, "appending"),
            LlvmLinkage::ExternWeak => write!(f, "extern_weak"),
            LlvmLinkage::LinkOnceOdr => write!(f, "linkonce_odr"),
            LlvmLinkage::WeakOdr => write!(f, "weak_odr"),
        }
    }
}
