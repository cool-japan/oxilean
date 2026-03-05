//! # CilMethodRef - Trait Implementations
//!
//! This module contains trait implementations for `CilMethodRef`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::CilMethodRef;
use std::fmt;

impl fmt::Display for CilMethodRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let conv_str = self.call_conv.to_string();
        if conv_str.is_empty() {
            write!(
                f,
                "{} {}::{}(",
                self.return_type, self.declaring_type, self.name
            )?;
        } else {
            write!(
                f,
                "{} {} {}::{}(",
                conv_str, self.return_type, self.declaring_type, self.name
            )?;
        }
        for (i, p) in self.param_types.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", p)?;
        }
        write!(f, ")")
    }
}
