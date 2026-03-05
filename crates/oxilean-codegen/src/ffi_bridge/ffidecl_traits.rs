//! # FfiDecl - Trait Implementations
//!
//! This module contains trait implementations for `FfiDecl`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FfiDecl;
use std::fmt;

impl fmt::Display for FfiDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@[extern {}] ", self.extern_name)?;
        if self.is_unsafe {
            write!(f, "unsafe ")?;
        }
        write!(f, "fn {}(", self.name)?;
        for (i, (pname, pty)) in self.params.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", pname, pty)?;
        }
        write!(f, ") -> {} [{}]", self.ret_type, self.calling_conv)
    }
}
