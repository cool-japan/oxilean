//! # WasmComponentExpr - Trait Implementations
//!
//! This module contains trait implementations for `WasmComponentExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WasmComponentExpr;
use std::fmt;

impl fmt::Display for WasmComponentExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WasmComponentExpr::Lift {
                func, result_type, ..
            } => {
                write!(f, "(lift {} {})", func, result_type)
            }
            WasmComponentExpr::Lower { func, .. } => write!(f, "(lower {})", func),
            WasmComponentExpr::ResourceNew {
                resource,
                core_func,
            } => {
                write!(f, "(resource.new {} {})", resource, core_func)
            }
            WasmComponentExpr::ResourceDrop { resource } => {
                write!(f, "(resource.drop {})", resource)
            }
            WasmComponentExpr::ResourceRep { resource } => {
                write!(f, "(resource.rep {})", resource)
            }
            WasmComponentExpr::Call {
                instance,
                func,
                args,
            } => {
                write!(f, "({}.{}", instance, func)?;
                for a in args {
                    write!(f, " {}", a)?;
                }
                write!(f, ")")
            }
            WasmComponentExpr::IntLit(n) => write!(f, "{}", n),
            WasmComponentExpr::FloatLit(n) => write!(f, "{}", n),
            WasmComponentExpr::StringLit(s) => write!(f, "\"{}\"", s),
            WasmComponentExpr::BoolLit(b) => write!(f, "{}", b),
            WasmComponentExpr::Var(name) => write!(f, "{}", name),
            WasmComponentExpr::RecordNew(fields) => {
                write!(f, "{{record")?;
                for (name, val) in fields {
                    write!(f, " {}: {}", name, val)?;
                }
                write!(f, "}}")
            }
            WasmComponentExpr::FieldGet(rec, field) => write!(f, "{}.{}", rec, field),
            WasmComponentExpr::VariantNew(tag, payload) => match payload.as_ref() {
                Some(p) => write!(f, "(variant {} {})", tag, p),
                None => write!(f, "(variant {})", tag),
            },
            WasmComponentExpr::OptionSome(v) => write!(f, "(some {})", v),
            WasmComponentExpr::OptionNone => write!(f, "none"),
            WasmComponentExpr::ResultOk(v) => match v.as_ref() {
                Some(inner) => write!(f, "(ok {})", inner),
                None => write!(f, "ok"),
            },
            WasmComponentExpr::ResultErr(v) => match v.as_ref() {
                Some(inner) => write!(f, "(err {})", inner),
                None => write!(f, "err"),
            },
            WasmComponentExpr::ListNew(items) => {
                write!(f, "[list")?;
                for item in items {
                    write!(f, " {}", item)?;
                }
                write!(f, "]")
            }
            WasmComponentExpr::TupleNew(elems) => {
                write!(f, "(tuple")?;
                for e in elems {
                    write!(f, " {}", e)?;
                }
                write!(f, ")")
            }
        }
    }
}
