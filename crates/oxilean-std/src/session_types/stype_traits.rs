//! # SType - Trait Implementations
//!
//! This module contains trait implementations for `SType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SType;
use std::fmt;

impl std::fmt::Display for SType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SType::Send(t, s) => write!(f, "!{}.{}", t, s),
            SType::Recv(t, s) => write!(f, "?{}.{}", t, s),
            SType::End => write!(f, "End"),
            SType::Choice(s1, s2) => write!(f, "({} ⊕ {})", s1, s2),
            SType::Branch(s1, s2) => write!(f, "({} & {})", s1, s2),
            SType::Rec(x, s) => write!(f, "μ{}.{}", x, s),
            SType::Var(x) => write!(f, "{}", x),
        }
    }
}
