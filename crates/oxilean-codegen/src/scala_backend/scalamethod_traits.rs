//! # ScalaMethod - Trait Implementations
//!
//! This module contains trait implementations for `ScalaMethod`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ScalaMethod;
use std::fmt;

impl fmt::Display for ScalaMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for m in &self.modifiers {
            write!(f, "{} ", m)?;
        }
        write!(f, "def {}", self.name)?;
        if !self.type_params.is_empty() {
            write!(f, "[")?;
            for (i, tp) in self.type_params.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", tp)?;
            }
            write!(f, "]")?;
        }
        for param_list in &self.params {
            write!(f, "(")?;
            for (i, p) in param_list.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", p)?;
            }
            write!(f, ")")?;
        }
        write!(f, ": {}", self.return_type)?;
        if let Some(body) = &self.body {
            write!(f, " = {}", body)?;
        }
        Ok(())
    }
}
