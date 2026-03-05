//! # ErrorSource - Trait Implementations
//!
//! This module contains trait implementations for `ErrorSource`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ErrorSource;
use std::fmt;

impl fmt::Display for ErrorSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorSource::Kernel => write!(f, "kernel"),
            ErrorSource::Elaborator => write!(f, "elaborator"),
            ErrorSource::TypeChecker => write!(f, "type-checker"),
            ErrorSource::UserCode { decl_name } => write!(f, "user-code({})", decl_name),
            ErrorSource::IoMonad => write!(f, "io-monad"),
            ErrorSource::BytecodeInterp { chunk_name, ip } => {
                write!(f, "bytecode-interp({}@{})", chunk_name, ip)
            }
            ErrorSource::Unknown => write!(f, "unknown"),
        }
    }
}
