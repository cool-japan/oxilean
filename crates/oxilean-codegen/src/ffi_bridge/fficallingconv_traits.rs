//! # FfiCallingConv - Trait Implementations
//!
//! This module contains trait implementations for `FfiCallingConv`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FfiCallingConv;
use std::fmt;

impl std::fmt::Display for FfiCallingConv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FfiCallingConv::C => write!(f, "C"),
            FfiCallingConv::StdCall => write!(f, "__stdcall"),
            FfiCallingConv::FastCall => write!(f, "__fastcall"),
            FfiCallingConv::ThisCall => write!(f, "__thiscall"),
            FfiCallingConv::VectorCall => write!(f, "__vectorcall"),
            FfiCallingConv::Win64 => write!(f, "win64"),
            FfiCallingConv::SysV64 => write!(f, "sysv64"),
            FfiCallingConv::Swift => write!(f, "swift"),
            FfiCallingConv::Rust => write!(f, "Rust"),
            FfiCallingConv::Custom(s) => write!(f, "{}", s),
        }
    }
}
