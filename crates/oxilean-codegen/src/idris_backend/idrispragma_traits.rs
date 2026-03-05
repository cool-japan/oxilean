//! # IdrisPragma - Trait Implementations
//!
//! This module contains trait implementations for `IdrisPragma`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IdrisPragma;
use std::fmt;

impl fmt::Display for IdrisPragma {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdrisPragma::Name(var, names) => {
                write!(f, "%name {} {}", var, names.join(", "))
            }
            IdrisPragma::AutoImplicit => write!(f, "%auto_implicit"),
            IdrisPragma::DefaultTotal => write!(f, "%default total"),
            IdrisPragma::DefaultPartial => write!(f, "%default partial"),
            IdrisPragma::DefaultCovering => write!(f, "%default covering"),
            IdrisPragma::Inline => write!(f, "%inline"),
            IdrisPragma::NoInline => write!(f, "%noinline"),
            IdrisPragma::Hint => write!(f, "%hint"),
            IdrisPragma::Extern => write!(f, "%extern"),
            IdrisPragma::Builtin(b) => write!(f, "%builtin {}", b),
            IdrisPragma::Foreign { backend, impl_str } => {
                write!(f, "%foreign \"{}\" \"{}\"", backend, impl_str)
            }
            IdrisPragma::Transform(rule) => write!(f, "%transform {}", rule),
            IdrisPragma::Deprecate(None) => write!(f, "%deprecate"),
            IdrisPragma::Deprecate(Some(msg)) => write!(f, "%deprecate \"{}\"", msg),
            IdrisPragma::Hide(name) => write!(f, "%hide {}", name),
            IdrisPragma::UnboundImplicitsOff => write!(f, "%unbound_implicits off"),
            IdrisPragma::AmbiguityDepth(n) => write!(f, "%ambiguity_depth {}", n),
            IdrisPragma::SearchTimeout(n) => write!(f, "%search_timeout {}", n),
            IdrisPragma::Logging { topic, level } => {
                write!(f, "%logging {} {}", topic, level)
            }
            IdrisPragma::Language(ext) => write!(f, "%language {}", ext),
            IdrisPragma::RunElab(expr) => write!(f, "%runElab {}", expr),
        }
    }
}
