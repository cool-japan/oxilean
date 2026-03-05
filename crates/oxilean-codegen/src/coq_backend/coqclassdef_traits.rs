//! # CoqClassDef - Trait Implementations
//!
//! This module contains trait implementations for `CoqClassDef`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoqClassDef;
use std::fmt;

impl std::fmt::Display for CoqClassDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<String> = self
            .params
            .iter()
            .map(|(n, t)| format!("({} : {})", n, t))
            .collect();
        write!(f, "Class {}", self.name)?;
        if !params.is_empty() {
            write!(f, " {}", params.join(" "))?;
        }
        writeln!(f, " := {{")?;
        for (mname, mtype) in &self.methods {
            writeln!(f, "  {} : {};", mname, mtype)?;
        }
        write!(f, "}}.")
    }
}
