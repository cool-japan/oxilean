//! # InfoData - Trait Implementations
//!
//! This module contains trait implementations for `InfoData`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::InfoData;
use oxilean_kernel::*;
use std::fmt;

impl fmt::Display for InfoData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InfoData::TermInfo { expr, type_ } => {
                write!(f, "term: {:?} : {:?}", expr, type_)
            }
            InfoData::FieldInfo {
                struct_name,
                field_name,
                ..
            } => {
                write!(f, "field: {}.{}", struct_name, field_name)
            }
            InfoData::TacticInfo { .. } => write!(f, "tactic"),
            InfoData::MacroExpansion { .. } => write!(f, "macro expansion"),
            InfoData::CommandInfo { name, .. } => write!(f, "command: {}", name),
            InfoData::CompletionInfo { prefix, .. } => {
                write!(f, "completion: {}", prefix)
            }
            InfoData::LevelInfo { level } => write!(f, "level: {:?}", level),
            InfoData::BinderInfo { name, .. } => write!(f, "binder: {}", name),
            InfoData::RefInfo { name, .. } => write!(f, "ref: {}", name),
        }
    }
}
