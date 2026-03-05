//! # HaskellFunction - Trait Implementations
//!
//! This module contains trait implementations for `HaskellFunction`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::*;
use super::types::HaskellFunction;
use std::fmt;

impl fmt::Display for HaskellFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ty) = &self.type_annotation {
            writeln!(f, "{} :: {}", self.name, ty)?;
        }
        for eq in &self.equations {
            write!(f, "{}", self.name)?;
            for p in &eq.patterns {
                write!(f, " {}", paren_pattern(p))?;
            }
            if !eq.guards.is_empty() {
                for g in &eq.guards {
                    write!(f, "\n  | {} = {}", g.condition, g.body)?;
                }
            } else if let Some(body) = &eq.body {
                write!(f, " = {}", body)?;
            }
            if !eq.where_clause.is_empty() {
                write!(f, "\n  where")?;
                for wf in &eq.where_clause {
                    let wf_str = wf.to_string();
                    for line in wf_str.lines() {
                        write!(f, "\n    {}", line)?;
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
