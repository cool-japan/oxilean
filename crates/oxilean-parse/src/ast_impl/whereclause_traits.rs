//! # WhereClause - Trait Implementations
//!
//! This module contains trait implementations for `WhereClause`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WhereClause;
use std::fmt;

impl fmt::Display for WhereClause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        for param in &self.params {
            write!(f, " ({} : ...)", param.name)?;
        }
        if let Some(ty) = &self.ty {
            write!(f, " : {}", ty.value)?;
        }
        write!(f, " := ...")
    }
}
