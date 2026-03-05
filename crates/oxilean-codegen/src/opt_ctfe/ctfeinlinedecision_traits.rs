//! # CtfeInlineDecision - Trait Implementations
//!
//! This module contains trait implementations for `CtfeInlineDecision`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CtfeInlineDecision;
use std::fmt;

impl std::fmt::Display for CtfeInlineDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CtfeInlineDecision::AlwaysInline => write!(f, "always_inline"),
            CtfeInlineDecision::InlineIfSmall(n) => write!(f, "inline_if_small({})", n),
            CtfeInlineDecision::NeverInline => write!(f, "never_inline"),
            CtfeInlineDecision::InlineForCtfe => write!(f, "inline_for_ctfe"),
        }
    }
}
