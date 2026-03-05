//! # EvmInstruction - Trait Implementations
//!
//! This module contains trait implementations for `EvmInstruction`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::EvmInstruction;
use std::fmt;

impl fmt::Display for EvmInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.opcode.mnemonic())?;
        if let Some(ref data) = self.data {
            write!(f, " 0x")?;
            for b in data {
                write!(f, "{:02x}", b)?;
            }
        }
        if let Some(ref c) = self.comment {
            write!(f, "  ; {}", c)?;
        }
        Ok(())
    }
}
