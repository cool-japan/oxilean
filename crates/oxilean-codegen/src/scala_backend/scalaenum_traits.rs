//! # ScalaEnum - Trait Implementations
//!
//! This module contains trait implementations for `ScalaEnum`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ScalaEnum;
use std::fmt;

impl fmt::Display for ScalaEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "enum {}", self.name)?;
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
        if !self.extends_list.is_empty() {
            write!(f, " extends {}", self.extends_list[0])?;
            for e in &self.extends_list[1..] {
                write!(f, " with {}", e)?;
            }
        }
        write!(f, ":")?;
        let mut i = 0;
        while i < self.cases.len() {
            let case = &self.cases[i];
            if case.fields.is_empty() {
                let mut j = i;
                let mut simple_names = vec![case.name.clone()];
                while j + 1 < self.cases.len() && self.cases[j + 1].fields.is_empty() {
                    j += 1;
                    simple_names.push(self.cases[j].name.clone());
                }
                write!(f, "\n  case {}", simple_names.join(", "))?;
                i = j + 1;
            } else {
                write!(f, "\n  case {}(", case.name)?;
                for (k, field) in case.fields.iter().enumerate() {
                    if k > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", field)?;
                }
                write!(f, ")")?;
                i += 1;
            }
        }
        Ok(())
    }
}
