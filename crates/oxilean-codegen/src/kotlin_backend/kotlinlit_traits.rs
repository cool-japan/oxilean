//! # KotlinLit - Trait Implementations
//!
//! This module contains trait implementations for `KotlinLit`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::{fmt_stmt, fmt_stmts};
use super::types::KotlinLit;
use std::fmt;

impl fmt::Display for KotlinLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KotlinLit::Int(n) => write!(f, "{}", n),
            KotlinLit::Long(n) => write!(f, "{}L", n),
            KotlinLit::Bool(b) => write!(f, "{}", b),
            KotlinLit::Str(s) => {
                write!(f, "\"")?;
                for c in s.chars() {
                    match c {
                        '"' => write!(f, "\\\"")?,
                        '\\' => write!(f, "\\\\")?,
                        '\n' => write!(f, "\\n")?,
                        '\r' => write!(f, "\\r")?,
                        '\t' => write!(f, "\\t")?,
                        c => write!(f, "{}", c)?,
                    }
                }
                write!(f, "\"")
            }
            KotlinLit::Null => write!(f, "null"),
        }
    }
}
