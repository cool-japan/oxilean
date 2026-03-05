//! # IdrisTactic - Trait Implementations
//!
//! This module contains trait implementations for `IdrisTactic`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IdrisTactic;
use std::fmt;

impl fmt::Display for IdrisTactic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdrisTactic::Intro(x) => write!(f, "intro {}", x),
            IdrisTactic::Intros => write!(f, "intros"),
            IdrisTactic::Exact(e) => write!(f, "exact {}", e),
            IdrisTactic::Refl => write!(f, "refl"),
            IdrisTactic::Trivial => write!(f, "trivial"),
            IdrisTactic::Decide => write!(f, "decide"),
            IdrisTactic::Rewrite(h) => write!(f, "rewrite {}", h),
            IdrisTactic::RewriteBack(h) => write!(f, "rewrite <- {}", h),
            IdrisTactic::Apply(func) => write!(f, "apply {}", func),
            IdrisTactic::Cases(x) => write!(f, "cases {}", x),
            IdrisTactic::Induction(x) => write!(f, "induction {}", x),
            IdrisTactic::Search => write!(f, "search"),
            IdrisTactic::Auto => write!(f, "auto"),
            IdrisTactic::With(e) => write!(f, "with {}", e),
            IdrisTactic::Let(x, e) => write!(f, "let {} = {}", x, e),
            IdrisTactic::Have(x, t) => write!(f, "have {} : {}", x, t),
            IdrisTactic::Focus(n) => write!(f, "focus {}", n),
            IdrisTactic::Claim(n, t) => write!(f, "claim {} : {}", n, t),
            IdrisTactic::Unfold(n) => write!(f, "unfold {}", n),
            IdrisTactic::Compute => write!(f, "compute"),
            IdrisTactic::Normals => write!(f, "normals"),
            IdrisTactic::Fail(msg) => write!(f, "fail \"{}\"", msg),
            IdrisTactic::Seq(ts) => {
                for (i, t) in ts.iter().enumerate() {
                    if i > 0 {
                        write!(f, "; ")?;
                    }
                    write!(f, "{}", t)?;
                }
                Ok(())
            }
        }
    }
}
