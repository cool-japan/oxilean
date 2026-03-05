//! # CoqRecordDef - Trait Implementations
//!
//! This module contains trait implementations for `CoqRecordDef`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoqRecordDef;
use std::fmt;

impl std::fmt::Display for CoqRecordDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<String> = self
            .params
            .iter()
            .map(|(n, t)| format!("({} : {})", n, t))
            .collect();
        write!(f, "Record {}", self.name)?;
        if !params.is_empty() {
            write!(f, " {}", params.join(" "))?;
        }
        write!(f, " : {} :=", self.universe)?;
        if let Some(ctor) = &self.constructor {
            write!(f, " {}", ctor)?;
        }
        writeln!(f, " {{")?;
        for (fname, ftype) in &self.fields {
            writeln!(f, "  {} : {};", fname, ftype)?;
        }
        write!(f, "}}.")
    }
}
