//! # ReplEvent - Trait Implementations
//!
//! This module contains trait implementations for `ReplEvent`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ReplEvent;

impl std::fmt::Display for ReplEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReplEvent::Parsed(s) => write!(f, "Parsed: {}", s),
            ReplEvent::Error(s) => write!(f, "Error: {}", s),
            ReplEvent::Reset => write!(f, "Reset"),
            ReplEvent::OptionChanged(k, v) => write!(f, "Option {} = {}", k, v),
            ReplEvent::Exit => write!(f, "Exit"),
        }
    }
}
