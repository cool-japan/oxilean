//! # WasmComponentType - Trait Implementations
//!
//! This module contains trait implementations for `WasmComponentType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WasmComponentType;
use std::fmt;

impl fmt::Display for WasmComponentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WasmComponentType::Bool => write!(f, "bool"),
            WasmComponentType::S8 => write!(f, "s8"),
            WasmComponentType::U8 => write!(f, "u8"),
            WasmComponentType::S16 => write!(f, "s16"),
            WasmComponentType::U16 => write!(f, "u16"),
            WasmComponentType::S32 => write!(f, "s32"),
            WasmComponentType::U32 => write!(f, "u32"),
            WasmComponentType::S64 => write!(f, "s64"),
            WasmComponentType::U64 => write!(f, "u64"),
            WasmComponentType::F32 => write!(f, "f32"),
            WasmComponentType::F64 => write!(f, "f64"),
            WasmComponentType::Char => write!(f, "char"),
            WasmComponentType::String => write!(f, "string"),
            WasmComponentType::Record(fields) => {
                write!(f, "record {{")?;
                for (i, (name, ty)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", name, ty)?;
                }
                write!(f, "}}")
            }
            WasmComponentType::Variant(cases) => {
                write!(f, "variant {{")?;
                for (i, (name, payload)) in cases.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    match payload {
                        Some(ty) => write!(f, "{}({})", name, ty)?,
                        None => write!(f, "{}", name)?,
                    }
                }
                write!(f, "}}")
            }
            WasmComponentType::List(elem) => write!(f, "list<{}>", elem),
            WasmComponentType::Option(inner) => write!(f, "option<{}>", inner),
            WasmComponentType::Result(ok, err) => match (ok.as_ref(), err.as_ref()) {
                (Some(o), Some(e)) => write!(f, "result<{}, {}>", o, e),
                (Some(o), None) => write!(f, "result<{}>", o),
                (None, Some(e)) => write!(f, "result<_, {}>", e),
                (None, None) => write!(f, "result"),
            },
            WasmComponentType::Tuple(elems) => {
                write!(f, "tuple<")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, ">")
            }
            WasmComponentType::Enum(cases) => write!(f, "enum {{{}}}", cases.join(", ")),
            WasmComponentType::Flags(flags) => {
                write!(f, "flags {{{}}}", flags.join(", "))
            }
            WasmComponentType::Resource(name) => write!(f, "resource {}", name),
            WasmComponentType::Borrow(name) => write!(f, "borrow<{}>", name),
            WasmComponentType::TypeRef(name) => write!(f, "{}", name),
            WasmComponentType::Func(params, results) => {
                write!(f, "func(")?;
                for (i, (name, ty)) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", name, ty)?;
                }
                write!(f, ")")?;
                if !results.is_empty() {
                    write!(f, " -> ")?;
                    if results.len() == 1 {
                        write!(f, "{}", results[0].1)?;
                    } else {
                        write!(f, "(")?;
                        for (i, (name, ty)) in results.iter().enumerate() {
                            if i > 0 {
                                write!(f, ", ")?;
                            }
                            write!(f, "{}: {}", name, ty)?;
                        }
                        write!(f, ")")?;
                    }
                }
                Ok(())
            }
        }
    }
}
