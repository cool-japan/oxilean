//! # CtfeValueExt - Trait Implementations
//!
//! This module contains trait implementations for `CtfeValueExt`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CtfeValueExt;
use std::fmt;

impl std::fmt::Display for CtfeValueExt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CtfeValueExt::Unit => write!(f, "()"),
            CtfeValueExt::Bool(b) => write!(f, "{}", b),
            CtfeValueExt::Int(n) => write!(f, "{}", n),
            CtfeValueExt::Uint(n) => write!(f, "{}u", n),
            CtfeValueExt::Float(v) => write!(f, "{}", v),
            CtfeValueExt::Str(s) => write!(f, "\"{}\"", s),
            CtfeValueExt::Tuple(vs) => {
                let ss: Vec<String> = vs.iter().map(|v| v.to_string()).collect();
                write!(f, "({})", ss.join(", "))
            }
            CtfeValueExt::List(vs) => {
                let ss: Vec<String> = vs.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", ss.join(", "))
            }
            CtfeValueExt::Constructor(n, vs) => {
                if vs.is_empty() {
                    write!(f, "{}", n)
                } else {
                    let ss: Vec<String> = vs.iter().map(|v| v.to_string()).collect();
                    write!(f, "{} ({})", n, ss.join(", "))
                }
            }
            CtfeValueExt::Closure { params, .. } => {
                write!(f, "<closure({})>", params.join(", "))
            }
            CtfeValueExt::Opaque => write!(f, "<opaque>"),
        }
    }
}
