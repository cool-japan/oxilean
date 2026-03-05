//! # RustType - Trait Implementations
//!
//! This module contains trait implementations for `RustType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::RustType;
use std::fmt;

impl fmt::Display for RustType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RustType::I8 => write!(f, "i8"),
            RustType::I16 => write!(f, "i16"),
            RustType::I32 => write!(f, "i32"),
            RustType::I64 => write!(f, "i64"),
            RustType::I128 => write!(f, "i128"),
            RustType::Isize => write!(f, "isize"),
            RustType::U8 => write!(f, "u8"),
            RustType::U16 => write!(f, "u16"),
            RustType::U32 => write!(f, "u32"),
            RustType::U64 => write!(f, "u64"),
            RustType::U128 => write!(f, "u128"),
            RustType::Usize => write!(f, "usize"),
            RustType::F32 => write!(f, "f32"),
            RustType::F64 => write!(f, "f64"),
            RustType::Bool => write!(f, "bool"),
            RustType::Char => write!(f, "char"),
            RustType::Str => write!(f, "str"),
            RustType::Unit => write!(f, "()"),
            RustType::Never => write!(f, "!"),
            RustType::RustString => write!(f, "String"),
            RustType::Vec(inner) => write!(f, "Vec<{}>", inner),
            RustType::Option(inner) => write!(f, "Option<{}>", inner),
            RustType::Result(ok, err) => write!(f, "Result<{}, {}>", ok, err),
            RustType::Tuple(elems) => {
                write!(f, "(")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                if elems.len() == 1 {
                    write!(f, ",")?;
                }
                write!(f, ")")
            }
            RustType::Slice(inner) => write!(f, "[{}]", inner),
            RustType::Ref(mutable, inner) => {
                if *mutable {
                    write!(f, "&mut {}", inner)
                } else {
                    write!(f, "&{}", inner)
                }
            }
            RustType::Custom(name) => write!(f, "{}", name),
            RustType::Generic(base, args) => {
                write!(f, "{}<", base)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ">")
            }
            RustType::Lifetime(lt) => write!(f, "'{}", lt),
            RustType::Fn(params, ret) => {
                write!(f, "impl Fn(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ") -> {}", ret)
            }
        }
    }
}
