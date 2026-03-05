//! # FutharkConst - Trait Implementations
//!
//! This module contains trait implementations for `FutharkConst`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkConst;
use std::fmt;

impl std::fmt::Display for FutharkConst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FutharkConst::I8(v) => write!(f, "{}i8", v),
            FutharkConst::I16(v) => write!(f, "{}i16", v),
            FutharkConst::I32(v) => write!(f, "{}i32", v),
            FutharkConst::I64(v) => write!(f, "{}i64", v),
            FutharkConst::U8(v) => write!(f, "{}u8", v),
            FutharkConst::U16(v) => write!(f, "{}u16", v),
            FutharkConst::U32(v) => write!(f, "{}u32", v),
            FutharkConst::U64(v) => write!(f, "{}u64", v),
            FutharkConst::F16(v) => write!(f, "{}f16", v),
            FutharkConst::F32(v) => write!(f, "{}f32", v),
            FutharkConst::F64(v) => write!(f, "{}f64", v),
            FutharkConst::Bool(v) => write!(f, "{}", if *v { "true" } else { "false" }),
        }
    }
}
