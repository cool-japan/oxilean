//! # InferErrorKind - Trait Implementations
//!
//! This module contains trait implementations for `InferErrorKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::InferErrorKind;
use std::fmt;

#[allow(dead_code)]
impl std::fmt::Display for InferErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InferErrorKind::TypeMismatch { expected, got } => {
                write!(f, "type mismatch: expected {:?}, got {:?}", expected, got)
            }
            InferErrorKind::UnboundVariable(idx) => {
                write!(f, "unbound variable at de Bruijn index {}", idx)
            }
            InferErrorKind::UnknownConstant(n) => write!(f, "unknown constant: {}", n),
            InferErrorKind::MetaVarEscapes(id) => {
                write!(f, "metavariable ?{} escapes its scope", id)
            }
            InferErrorKind::UniverseMismatch(l1, l2) => {
                write!(f, "universe mismatch: {:?} vs {:?}", l1, l2)
            }
            InferErrorKind::NotAFunction(e) => {
                write!(f, "expected a function type, got {:?}", e)
            }
            InferErrorKind::NotASort(e) => write!(f, "expected a Sort, got {:?}", e),
            InferErrorKind::RecursionLimit => {
                write!(f, "type inference recursion limit exceeded")
            }
            InferErrorKind::Custom(s) => write!(f, "{}", s),
        }
    }
}
