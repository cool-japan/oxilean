//! # ComponentExportKind - Trait Implementations
//!
//! This module contains trait implementations for `ComponentExportKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ComponentExportKind;
use std::fmt;

impl fmt::Display for ComponentExportKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComponentExportKind::Func => write!(f, "func"),
            ComponentExportKind::Type => write!(f, "type"),
            ComponentExportKind::Instance => write!(f, "instance"),
            ComponentExportKind::Module => write!(f, "module"),
            ComponentExportKind::Value => write!(f, "value"),
            ComponentExportKind::Component => write!(f, "component"),
        }
    }
}
