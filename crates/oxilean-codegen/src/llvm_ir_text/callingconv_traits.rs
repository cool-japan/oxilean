//! # CallingConv - Trait Implementations
//!
//! This module contains trait implementations for `CallingConv`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CallingConv;
use std::fmt;

impl fmt::Display for CallingConv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CallingConv::C => Ok(()),
            CallingConv::Fast => write!(f, "fastcc "),
            CallingConv::Cold => write!(f, "coldcc "),
            CallingConv::Ghc => write!(f, "ghccc "),
            CallingConv::Wasm => write!(f, "wasm_funcref "),
            CallingConv::Num(n) => write!(f, "cc{} ", n),
        }
    }
}
