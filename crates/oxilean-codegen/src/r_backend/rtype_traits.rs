//! # RType - Trait Implementations
//!
//! This module contains trait implementations for `RType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RType;
use std::fmt;

impl fmt::Display for RType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RType::Numeric => write!(f, "numeric"),
            RType::Integer => write!(f, "integer"),
            RType::Logical => write!(f, "logical"),
            RType::Character => write!(f, "character"),
            RType::Complex => write!(f, "complex"),
            RType::Raw => write!(f, "raw"),
            RType::List => write!(f, "list"),
            RType::DataFrame => write!(f, "data.frame"),
            RType::Matrix(inner) => write!(f, "matrix<{}>", inner),
            RType::Array(inner, dims) => write!(f, "array<{}, {:?}>", inner, dims),
            RType::Function => write!(f, "function"),
            RType::Environment => write!(f, "environment"),
            RType::S3(name) => write!(f, "S3:{}", name),
            RType::S4(name) => write!(f, "S4:{}", name),
            RType::R5(name) => write!(f, "R5:{}", name),
            RType::R6(name) => write!(f, "R6:{}", name),
            RType::Null => write!(f, "NULL"),
            RType::Na => write!(f, "NA"),
            RType::Vector(inner) => write!(f, "vector<{}>", inner),
            RType::Named(name) => write!(f, "{}", name),
        }
    }
}
