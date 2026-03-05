//! # BashCondition - Trait Implementations
//!
//! This module contains trait implementations for `BashCondition`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BashCondition;
use std::fmt;

impl fmt::Display for BashCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BashCondition::FileExists(e) => write!(f, "[[ -e {} ]]", e),
            BashCondition::IsFile(e) => write!(f, "[[ -f {} ]]", e),
            BashCondition::IsDir(e) => write!(f, "[[ -d {} ]]", e),
            BashCondition::NonEmpty(e) => write!(f, "[[ -n {} ]]", e),
            BashCondition::Empty(e) => write!(f, "[[ -z {} ]]", e),
            BashCondition::StrEq(a, b) => write!(f, "[[ {} == {} ]]", a, b),
            BashCondition::StrNe(a, b) => write!(f, "[[ {} != {} ]]", a, b),
            BashCondition::StrLt(a, b) => write!(f, "[[ {} < {} ]]", a, b),
            BashCondition::ArithLt(a, b) => write!(f, "(( {} < {} ))", a, b),
            BashCondition::ArithEq(a, b) => write!(f, "(( {} == {} ))", a, b),
            BashCondition::And(a, b) => write!(f, "[[ {} && {} ]]", a, b),
            BashCondition::Or(a, b) => write!(f, "[[ {} || {} ]]", a, b),
            BashCondition::Not(c) => write!(f, "! {}", c),
            BashCondition::Raw(s) => write!(f, "{}", s),
        }
    }
}
