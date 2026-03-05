//! # CoqInstanceDef - Trait Implementations
//!
//! This module contains trait implementations for `CoqInstanceDef`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoqInstanceDef;
use std::fmt;

impl std::fmt::Display for CoqInstanceDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let nm = self.name.as_deref().unwrap_or("_");
        let args = self.args.join(" ");
        write!(
            f,
            "#[global] Instance {} : {} {} :=\n{{",
            nm, self.class, args
        )?;
        for (mn, mb) in &self.methods {
            write!(f, "\n  {} := {};", mn, mb)?;
        }
        write!(f, "\n}}.")
    }
}
