//! # HaskellDataDecl - Trait Implementations
//!
//! This module contains trait implementations for `HaskellDataDecl`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::*;
use super::types::HaskellDataDecl;
use std::fmt;

impl fmt::Display for HaskellDataDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "data {}", self.name)?;
        for p in &self.type_params {
            write!(f, " {}", p)?;
        }
        if self.constructors.is_empty() {
            return Ok(());
        }
        write!(f, "\n  = ")?;
        for (i, (ctor, fields)) in self.constructors.iter().enumerate() {
            if i > 0 {
                write!(f, "\n  | ")?;
            }
            write!(f, "{}", ctor)?;
            for fty in fields {
                write!(f, " {}", paren_type(fty))?;
            }
        }
        if !self.deriving_clauses.is_empty() {
            write!(f, "\n  deriving (")?;
            for (i, d) in self.deriving_clauses.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", d)?;
            }
            write!(f, ")")?;
        }
        Ok(())
    }
}
