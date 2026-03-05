//! # HaskellTypeClass - Trait Implementations
//!
//! This module contains trait implementations for `HaskellTypeClass`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::HaskellTypeClass;
use std::fmt;

impl fmt::Display for HaskellTypeClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.superclasses.is_empty() {
            write!(f, "class (")?;
            for (i, sc) in self.superclasses.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", sc)?;
            }
            write!(f, ") => {} ", self.name)?;
        } else {
            write!(f, "class {} ", self.name)?;
        }
        for p in &self.type_params {
            write!(f, "{} ", p)?;
        }
        write!(f, "where")?;
        for (mname, mty, mdefault) in &self.methods {
            write!(f, "\n  {} :: {}", mname, mty)?;
            if let Some(default) = mdefault {
                write!(f, "\n  {} = {}", mname, default)?;
            }
        }
        Ok(())
    }
}
