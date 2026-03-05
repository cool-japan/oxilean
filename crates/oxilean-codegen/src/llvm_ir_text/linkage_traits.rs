//! # Linkage - Trait Implementations
//!
//! This module contains trait implementations for `Linkage`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Linkage;
use std::fmt;

impl fmt::Display for Linkage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Linkage::External => Ok(()),
            Linkage::Internal => write!(f, "internal "),
            Linkage::Private => write!(f, "private "),
            Linkage::Weak => write!(f, "weak "),
            Linkage::Linkonce => write!(f, "linkonce "),
            Linkage::Common => write!(f, "common "),
            Linkage::AvailableExternally => write!(f, "available_externally "),
            Linkage::WeakOdr => write!(f, "weak_odr "),
            Linkage::LinkonceOdr => write!(f, "linkonce_odr "),
            Linkage::ExternalWeak => write!(f, "extern_weak "),
            Linkage::Appending => write!(f, "appending "),
        }
    }
}
