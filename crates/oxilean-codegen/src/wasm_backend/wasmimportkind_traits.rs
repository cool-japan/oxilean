//! # WasmImportKind - Trait Implementations
//!
//! This module contains trait implementations for `WasmImportKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::WasmImportKind;
use std::fmt;

impl fmt::Display for WasmImportKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WasmImportKind::Func { params, results } => {
                write!(f, "(func")?;
                for p in params {
                    write!(f, " (param {})", p)?;
                }
                for r in results {
                    write!(f, " (result {})", r)?;
                }
                write!(f, ")")
            }
            WasmImportKind::Memory { min_pages } => write!(f, "(memory {})", min_pages),
            WasmImportKind::Global { ty, mutable } => {
                if *mutable {
                    write!(f, "(global (mut {}))", ty)
                } else {
                    write!(f, "(global {})", ty)
                }
            }
        }
    }
}
