//! # CommandStage - Trait Implementations
//!
//! This module contains trait implementations for `CommandStage`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CommandStage;
use std::fmt;

impl fmt::Display for CommandStage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CommandStage::Parse => "Parse",
            CommandStage::Resolve => "Resolve",
            CommandStage::Elaborate => "Elaborate",
            CommandStage::TypeCheck => "TypeCheck",
            CommandStage::AddToEnv => "AddToEnv",
            CommandStage::PostProcess => "PostProcess",
            CommandStage::Done => "Done",
        };
        f.write_str(s)
    }
}
