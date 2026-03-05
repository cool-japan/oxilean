//! # HMType - Trait Implementations
//!
//! This module contains trait implementations for `HMType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::HMType;
use std::fmt;

impl std::fmt::Display for HMType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HMType::Var(v) => write!(f, "α{}", v),
            HMType::Base(s) => write!(f, "{}", s),
            HMType::Arrow(a, b) => write!(f, "({} → {})", a, b),
            HMType::App(c, args) => {
                if args.is_empty() {
                    write!(f, "{}", c)
                } else {
                    write!(
                        f,
                        "{}[{}]",
                        c,
                        args.iter()
                            .map(|a| a.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
            HMType::Tuple(ts) => {
                write!(
                    f,
                    "({})",
                    ts.iter()
                        .map(|t| t.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
    }
}
