//! # SwiftType - Trait Implementations
//!
//! This module contains trait implementations for `SwiftType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::{SwiftFunc, SwiftType};
use std::fmt;

impl fmt::Display for SwiftType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SwiftType::SwiftInt => write!(f, "Int"),
            SwiftType::SwiftBool => write!(f, "Bool"),
            SwiftType::SwiftString => write!(f, "String"),
            SwiftType::SwiftDouble => write!(f, "Double"),
            SwiftType::SwiftFloat => write!(f, "Float"),
            SwiftType::SwiftVoid => write!(f, "Void"),
            SwiftType::SwiftAny => write!(f, "Any"),
            SwiftType::SwiftAnyObject => write!(f, "AnyObject"),
            SwiftType::SwiftNever => write!(f, "Never"),
            SwiftType::SwiftArray(inner) => write!(f, "[{}]", inner),
            SwiftType::SwiftDict(k, v) => write!(f, "[{}: {}]", k, v),
            SwiftType::SwiftOptional(inner) => write!(f, "{}?", inner),
            SwiftType::SwiftTuple(elems) => {
                write!(f, "(")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, ")")
            }
            SwiftType::SwiftFunc(params, ret) => {
                write!(f, "(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ") -> {}", ret)
            }
            SwiftType::SwiftEnum(name) => write!(f, "{}", name),
            SwiftType::SwiftStruct(name) => write!(f, "{}", name),
            SwiftType::SwiftClass(name) => write!(f, "{}", name),
            SwiftType::SwiftProtocol(name) => write!(f, "{}", name),
            SwiftType::SwiftNamed(name) => write!(f, "{}", name),
            SwiftType::SwiftGeneric(base, args) => {
                write!(f, "{}<", base)?;
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
