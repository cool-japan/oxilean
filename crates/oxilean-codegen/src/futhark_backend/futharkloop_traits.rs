//! # FutharkLoop - Trait Implementations
//!
//! This module contains trait implementations for `FutharkLoop`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{FutharkLoop, FutharkLoopKind};
use std::fmt;

impl std::fmt::Display for FutharkLoop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<String> = self
            .params
            .iter()
            .map(|(n, t, i)| format!("({n}: {t} = {i})"))
            .collect();
        let param_str = params.join(" ");
        match &self.kind {
            FutharkLoopKind::For { var, bound } => {
                write!(
                    f,
                    "loop {} for {var} < {bound} do\n  {}",
                    param_str, self.body
                )
            }
            FutharkLoopKind::While { cond } => {
                write!(f, "loop {} while {} do\n  {}", param_str, cond, self.body)
            }
            FutharkLoopKind::ForWhile { var, bound, cond } => {
                write!(
                    f,
                    "loop {} for {var} < {bound} while {} do\n  {}",
                    param_str, cond, self.body
                )
            }
        }
    }
}
