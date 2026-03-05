//! # MetaContext - Trait Implementations
//!
//! This module contains trait implementations for `MetaContext`.
//!
//! ## Implemented Traits
//!
//! - `MetaContextExt`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::MetaContextExt;
use super::metacontext_type::MetaContext;
use super::types::MVarId;

impl MetaContextExt for MetaContext {
    fn mvar_count(&self) -> usize {
        self.num_mvars()
    }
    fn unassigned_mvars_ext(&self) -> Vec<MVarId> {
        self.unassigned_mvars()
    }
}
