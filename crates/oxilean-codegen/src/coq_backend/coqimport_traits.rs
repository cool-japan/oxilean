//! # CoqImport - Trait Implementations
//!
//! This module contains trait implementations for `CoqImport`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoqImport;
use std::fmt;

impl std::fmt::Display for CoqImport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoqImport::Require(mods) => write!(f, "Require {}.", mods.join(" ")),
            CoqImport::RequireImport(mods) => {
                write!(f, "Require Import {}.", mods.join(" "))
            }
            CoqImport::RequireExport(mods) => {
                write!(f, "Require Export {}.", mods.join(" "))
            }
            CoqImport::Import(mods) => write!(f, "Import {}.", mods.join(" ")),
            CoqImport::Open(scope) => write!(f, "Open Scope {}.", scope),
        }
    }
}
