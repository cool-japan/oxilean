//! # WasmFunc - Trait Implementations
//!
//! This module contains trait implementations for `WasmFunc`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::WasmFunc;
use std::fmt;

impl fmt::Display for WasmFunc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "  (func ${}", self.name)?;
        for (pname, pty) in &self.params {
            write!(f, " (param ${} {})", pname, pty)?;
        }
        for rty in &self.results {
            write!(f, " (result {})", rty)?;
        }
        writeln!(f)?;
        for (lname, lty) in &self.locals {
            writeln!(f, "    (local ${} {})", lname, lty)?;
        }
        for instr in &self.body {
            writeln!(f, "    {}", instr)?;
        }
        write!(f, "  )")
    }
}
