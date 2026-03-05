//! # MlirDialect - Trait Implementations
//!
//! This module contains trait implementations for `MlirDialect`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MlirDialect;
use std::fmt;

impl fmt::Display for MlirDialect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MlirDialect::Builtin => write!(f, "builtin"),
            MlirDialect::Arith => write!(f, "arith"),
            MlirDialect::Func => write!(f, "func"),
            MlirDialect::CF => write!(f, "cf"),
            MlirDialect::MemRef => write!(f, "memref"),
            MlirDialect::SCF => write!(f, "scf"),
            MlirDialect::Affine => write!(f, "affine"),
            MlirDialect::Tensor => write!(f, "tensor"),
            MlirDialect::Vector => write!(f, "vector"),
            MlirDialect::Linalg => write!(f, "linalg"),
            MlirDialect::GPU => write!(f, "gpu"),
            MlirDialect::LLVM => write!(f, "llvm"),
            MlirDialect::Math => write!(f, "math"),
            MlirDialect::Index => write!(f, "index"),
        }
    }
}
