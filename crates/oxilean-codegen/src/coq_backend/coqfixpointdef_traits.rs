//! # CoqFixpointDef - Trait Implementations
//!
//! This module contains trait implementations for `CoqFixpointDef`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoqFixpointDef;
use std::fmt;

impl std::fmt::Display for CoqFixpointDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<String> = self
            .params
            .iter()
            .map(|(n, t)| format!("({} : {})", n, t))
            .collect();
        write!(f, "Fixpoint {} {}", self.name, params.join(" "))?;
        if let Some(st) = &self.struct_arg {
            write!(f, " {{struct {}}}", st)?;
        }
        write!(f, " : {} :=\n  {}.", self.return_type, self.body)
    }
}
