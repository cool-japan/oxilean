//! # CoqInductiveDef - Trait Implementations
//!
//! This module contains trait implementations for `CoqInductiveDef`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoqInductiveDef;
use std::fmt;

impl std::fmt::Display for CoqInductiveDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kw = if self.is_coinductive {
            "CoInductive"
        } else {
            "Inductive"
        };
        let params: Vec<String> = self
            .params
            .iter()
            .map(|(n, t)| format!("({} : {})", n, t))
            .collect();
        let indices: Vec<String> = self
            .indices
            .iter()
            .map(|(n, t)| format!("({} : {})", n, t))
            .collect();
        write!(f, "{} {}", kw, self.name)?;
        if !params.is_empty() {
            write!(f, " {}", params.join(" "))?;
        }
        if !indices.is_empty() {
            write!(f, " : {} -> {}", indices.join(" -> "), self.universe)?;
        } else {
            write!(f, " : {}", self.universe)?;
        }
        writeln!(f, " :=")?;
        for (cname, cargs, _ret) in &self.constructors {
            let args: Vec<String> = cargs
                .iter()
                .map(|(n, t)| format!("({} : {})", n, t))
                .collect();
            if args.is_empty() {
                writeln!(f, "| {} : {}", cname, self.name)?;
            } else {
                writeln!(f, "| {} : {} -> {}", cname, args.join(" -> "), self.name)?;
            }
        }
        write!(f, ".")
    }
}
