//! # CoqLetDef - Trait Implementations
//!
//! This module contains trait implementations for `CoqLetDef`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoqLetDef;
use std::fmt;

impl std::fmt::Display for CoqLetDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kw = if self.is_opaque {
            "Opaque"
        } else {
            "Transparent"
        };
        write!(f, "Definition {}", self.name)?;
        for (pn, pt) in &self.params {
            if let Some(t) = pt {
                write!(f, " ({} : {})", pn, t)?;
            } else {
                write!(f, " {}", pn)?;
            }
        }
        if let Some(rt) = &self.return_type {
            write!(f, " : {}", rt)?;
        }
        write!(f, " := {}.", self.body)?;
        if self.is_opaque {
            write!(f, "\n{} {}.", kw, self.name)?;
        }
        Ok(())
    }
}
