//! # DoElem - Trait Implementations
//!
//! This module contains trait implementations for `DoElem`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DoElem;
use std::fmt;

impl fmt::Display for DoElem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DoElem::Bind { pat, ty, rhs } => {
                if let Some(ty) = ty {
                    write!(f, "{} : {:?} <- {:?}", pat, ty, rhs)
                } else {
                    write!(f, "{} <- {:?}", pat, rhs)
                }
            }
            DoElem::LetBind { pat, ty, val } => {
                if let Some(ty) = ty {
                    write!(f, "let {} : {:?} := {:?}", pat, ty, val)
                } else {
                    write!(f, "let {} := {:?}", pat, val)
                }
            }
            DoElem::Action(expr) => write!(f, "{:?}", expr),
            DoElem::Return(expr) => write!(f, "return {:?}", expr),
            DoElem::For {
                var,
                collection,
                body,
            } => {
                write!(f, "for {} in {:?} do {}", var, collection, body)
            }
            DoElem::If { cond, then_, else_ } => {
                write!(
                    f,
                    "if {:?} then ({} elems) else ({} elems)",
                    cond,
                    then_.len(),
                    else_.len()
                )
            }
            DoElem::Match { scrutinee, arms } => {
                write!(f, "match {:?} with {} arms", scrutinee, arms.len())
            }
            DoElem::TryCatch {
                body,
                catch_var,
                catch_body,
            } => {
                write!(
                    f,
                    "try ({} elems) catch {} => ({} elems)",
                    body.len(),
                    catch_var,
                    catch_body.len()
                )
            }
            DoElem::Unless { cond, body } => {
                write!(f, "unless {:?} do ({} elems)", cond, body.len())
            }
        }
    }
}
