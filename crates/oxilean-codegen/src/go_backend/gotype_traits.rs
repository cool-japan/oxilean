//! # GoType - Trait Implementations
//!
//! This module contains trait implementations for `GoType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::format_stmts;
use super::types::{GoFunc, GoType};
use std::fmt;

impl fmt::Display for GoType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GoType::GoBool => write!(f, "bool"),
            GoType::GoInt => write!(f, "int64"),
            GoType::GoFloat => write!(f, "float64"),
            GoType::GoString => write!(f, "string"),
            GoType::GoSlice(inner) => write!(f, "[]{}", inner),
            GoType::GoMap(k, v) => write!(f, "map[{}]{}", k, v),
            GoType::GoFunc(params, rets) => {
                write!(f, "func(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ")")?;
                match rets.len() {
                    0 => Ok(()),
                    1 => write!(f, " {}", rets[0]),
                    _ => {
                        write!(f, " (")?;
                        for (i, r) in rets.iter().enumerate() {
                            if i > 0 {
                                write!(f, ", ")?;
                            }
                            write!(f, "{}", r)?;
                        }
                        write!(f, ")")
                    }
                }
            }
            GoType::GoInterface => write!(f, "interface{{}}"),
            GoType::GoStruct(name) => write!(f, "{}", name),
            GoType::GoPtr(inner) => write!(f, "*{}", inner),
            GoType::GoChan(inner) => write!(f, "chan {}", inner),
            GoType::GoError => write!(f, "error"),
            GoType::GoUnit => write!(f, "struct{{}}"),
        }
    }
}
