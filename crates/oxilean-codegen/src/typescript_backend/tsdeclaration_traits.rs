//! # TsDeclaration - Trait Implementations
//!
//! This module contains trait implementations for `TsDeclaration`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{format_ts_stmt, format_ts_stmts};
use super::types::TsDeclaration;
use std::fmt;

impl fmt::Display for TsDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TsDeclaration::Interface(i) => write!(f, "{}", i),
            TsDeclaration::TypeAlias(t) => write!(f, "{}", t),
            TsDeclaration::Enum(e) => write!(f, "{}", e),
            TsDeclaration::Function(func) => write!(f, "{}", func),
            TsDeclaration::Class(c) => write!(f, "{}", c),
            TsDeclaration::Const(name, ty, expr) => {
                if let Some(t) = ty {
                    write!(f, "export const {}: {} = {};", name, t, expr)
                } else {
                    write!(f, "export const {} = {};", name, expr)
                }
            }
            TsDeclaration::Let(name, ty, expr) => {
                if let Some(t) = ty {
                    write!(f, "export let {}: {} = {};", name, t, expr)
                } else {
                    write!(f, "export let {} = {};", name, expr)
                }
            }
            TsDeclaration::ReExport(path) => write!(f, "export * from \"{}\";", path),
        }
    }
}
