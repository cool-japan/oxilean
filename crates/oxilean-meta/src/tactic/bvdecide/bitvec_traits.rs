//! # BitVec - Trait Implementations
//!
//! This module contains trait implementations for `BitVec`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BitVec;
use std::fmt;

impl fmt::Display for BitVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x")?;
        let w = self.width.as_usize();
        let hex_digits = (w + 3) / 4;
        let val = self.to_u128();
        write!(f, "{:0>width$x}", val, width = hex_digits)?;
        write!(f, "#{}", self.width)
    }
}
