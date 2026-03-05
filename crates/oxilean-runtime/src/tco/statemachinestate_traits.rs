//! # StateMachineState - Trait Implementations
//!
//! This module contains trait implementations for `StateMachineState`.
//!
//! ## Implemented Traits
//!
//! - `TapId`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::TapId;
use super::types::StateMachineState;

impl TapId for StateMachineState {
    fn tap_id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }
}
