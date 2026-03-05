//! # LuaType - Trait Implementations
//!
//! This module contains trait implementations for `LuaType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::LuaType;
use std::fmt;

impl fmt::Display for LuaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LuaType::Nil => write!(f, "nil"),
            LuaType::Boolean => write!(f, "boolean"),
            LuaType::Number(true) => write!(f, "integer"),
            LuaType::Number(false) => write!(f, "float"),
            LuaType::String => write!(f, "string"),
            LuaType::Table => write!(f, "table"),
            LuaType::Function => write!(f, "function"),
            LuaType::Userdata => write!(f, "userdata"),
            LuaType::Thread => write!(f, "thread"),
            LuaType::Custom(name) => write!(f, "{}", name),
        }
    }
}
