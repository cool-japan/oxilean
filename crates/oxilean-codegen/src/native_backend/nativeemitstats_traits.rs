//! # NativeEmitStats - Trait Implementations
//!
//! This module contains trait implementations for `NativeEmitStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::NativeEmitStats;
use std::fmt;

impl fmt::Display for NativeEmitStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "NativeEmitStats {{ fns={}, blocks={}, insts={}, vregs={}, stacks={}, spills={} }}",
            self.functions_compiled,
            self.blocks_generated,
            self.instructions_generated,
            self.virtual_regs_used,
            self.stack_slots_allocated,
            self.spills,
        )
    }
}
