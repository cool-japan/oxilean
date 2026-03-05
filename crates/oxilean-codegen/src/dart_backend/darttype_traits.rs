//! # DartType - Trait Implementations
//!
//! This module contains trait implementations for `DartType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::{fmt_args, fmt_typed_params};
use super::types::DartType;
use std::fmt;

impl fmt::Display for DartType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DartType::DtInt => write!(f, "int"),
            DartType::DtDouble => write!(f, "double"),
            DartType::DtBool => write!(f, "bool"),
            DartType::DtString => write!(f, "String"),
            DartType::DtVoid => write!(f, "void"),
            DartType::DtDynamic => write!(f, "dynamic"),
            DartType::DtObject => write!(f, "Object"),
            DartType::DtNull => write!(f, "Null"),
            DartType::DtNullable(inner) => write!(f, "{}?", inner),
            DartType::DtList(inner) => write!(f, "List<{}>", inner),
            DartType::DtMap(k, v) => write!(f, "Map<{}, {}>", k, v),
            DartType::DtSet(inner) => write!(f, "Set<{}>", inner),
            DartType::DtFuture(inner) => write!(f, "Future<{}>", inner),
            DartType::DtStream(inner) => write!(f, "Stream<{}>", inner),
            DartType::DtFunction(params, ret) => {
                write!(f, "{} Function(", ret)?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ")")
            }
            DartType::DtNamed(name) => write!(f, "{}", name),
            DartType::DtGeneric(name, args) => {
                write!(f, "{}<", name)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ">")
            }
        }
    }
}
