//! # RubyPattern - Trait Implementations
//!
//! This module contains trait implementations for `RubyPattern`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::{fmt_ruby_class, fmt_ruby_method, fmt_ruby_module_stmt, fmt_ruby_stmt};
use super::types::RubyPattern;
use std::fmt;

impl std::fmt::Display for RubyPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RubyPattern::Pin(v) => write!(f, "^{}", v),
            RubyPattern::Variable(v) => write!(f, "{}", v),
            RubyPattern::Literal(l) => write!(f, "{}", l),
            RubyPattern::Array(pats) => {
                let ps: Vec<String> = pats.iter().map(|p| p.to_string()).collect();
                write!(f, "[{}]", ps.join(", "))
            }
            RubyPattern::Hash(fields) => {
                let fs: Vec<String> = fields
                    .iter()
                    .map(|(k, v)| {
                        if let Some(p) = v {
                            format!("{}: {}", k, p)
                        } else {
                            format!("{}:", k)
                        }
                    })
                    .collect();
                write!(f, "{{{}}}", fs.join(", "))
            }
            RubyPattern::Guard(p, cond) => write!(f, "{} if {}", p, cond),
            _ => write!(f, "_"),
        }
    }
}
