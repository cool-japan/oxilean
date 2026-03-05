//! # HaskellInstance - Trait Implementations
//!
//! This module contains trait implementations for `HaskellInstance`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::*;
use super::types::HaskellInstance;
use std::fmt;

impl fmt::Display for HaskellInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.context.is_empty() {
            write!(f, "instance (")?;
            for (i, c) in self.context.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", c)?;
            }
            write!(f, ") => ")?;
        } else {
            write!(f, "instance ")?;
        }
        write!(
            f,
            "{} {} where",
            self.class,
            paren_type(&self.instance_type)
        )?;
        for func in &self.where_clause {
            let func_str = func.to_string();
            for line in func_str.lines() {
                write!(f, "\n  {}", line)?;
            }
        }
        Ok(())
    }
}
