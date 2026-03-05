//! # KotlinType - Trait Implementations
//!
//! This module contains trait implementations for `KotlinType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::{fmt_stmt, fmt_stmts};
use super::types::KotlinType;
use std::fmt;

impl fmt::Display for KotlinType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KotlinType::KtInt => write!(f, "Int"),
            KotlinType::KtLong => write!(f, "Long"),
            KotlinType::KtBool => write!(f, "Boolean"),
            KotlinType::KtString => write!(f, "String"),
            KotlinType::KtUnit => write!(f, "Unit"),
            KotlinType::KtAny => write!(f, "Any"),
            KotlinType::KtList(inner) => write!(f, "List<{}>", inner),
            KotlinType::KtPair(a, b) => write!(f, "Pair<{}, {}>", a, b),
            KotlinType::KtFunc(params, ret) => {
                write!(f, "(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ") -> {}", ret)
            }
            KotlinType::KtNullable(inner) => write!(f, "{}?", inner),
            KotlinType::KtObject(name) => write!(f, "{}", name),
        }
    }
}
