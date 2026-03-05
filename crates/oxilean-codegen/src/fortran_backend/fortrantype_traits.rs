//! # FortranType - Trait Implementations
//!
//! This module contains trait implementations for `FortranType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FortranType;
use std::fmt;

impl fmt::Display for FortranType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FortranType::FtInteger => write!(f, "INTEGER"),
            FortranType::FtIntegerK(k) => write!(f, "INTEGER(KIND={})", k),
            FortranType::FtReal => write!(f, "REAL"),
            FortranType::FtDouble => write!(f, "REAL(KIND=8)"),
            FortranType::FtComplex => write!(f, "COMPLEX"),
            FortranType::FtComplexDouble => write!(f, "COMPLEX(KIND=8)"),
            FortranType::FtLogical => write!(f, "LOGICAL"),
            FortranType::FtCharacter(Some(n)) => write!(f, "CHARACTER(LEN={})", n),
            FortranType::FtCharacter(None) => write!(f, "CHARACTER"),
            FortranType::FtCharacterStar => write!(f, "CHARACTER(LEN=*)"),
            FortranType::FtArray(inner, dim) => {
                write!(f, "{}, DIMENSION({})", inner, dim)
            }
            FortranType::FtDerived(name) => write!(f, "TYPE({})", name),
            FortranType::FtClass(name) => write!(f, "CLASS({})", name),
            FortranType::FtClassStar => write!(f, "CLASS(*)"),
            FortranType::FtAssumedType => write!(f, "TYPE(*)"),
            FortranType::FtPointer(inner) => write!(f, "{}, POINTER", inner),
            FortranType::FtAllocatable(inner) => write!(f, "{}, ALLOCATABLE", inner),
            FortranType::FtVoid => write!(f, "! void"),
        }
    }
}
