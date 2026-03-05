//! # AesopRuleKind - Trait Implementations
//!
//! This module contains trait implementations for `AesopRuleKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AesopRuleKind;

impl std::fmt::Display for AesopRuleKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AesopRuleKind::Apply => write!(f, "apply"),
            AesopRuleKind::Constructor => write!(f, "constructor"),
            AesopRuleKind::Cases => write!(f, "cases"),
            AesopRuleKind::Ext => write!(f, "ext"),
            AesopRuleKind::Forward => write!(f, "forward"),
            AesopRuleKind::Unfold => write!(f, "unfold"),
            AesopRuleKind::Norm => write!(f, "norm"),
        }
    }
}
