//! # YulExpr - Trait Implementations
//!
//! This module contains trait implementations for `YulExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::YulExpr;
use std::fmt;

impl std::fmt::Display for YulExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            YulExpr::Literal(n) => write!(f, "{}", n),
            YulExpr::Variable(n) => write!(f, "{}", n),
            YulExpr::FunctionCall(name, args) => {
                let args_str: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                write!(f, "{}({})", name, args_str.join(", "))
            }
        }
    }
}
