//! # MetaContext - mk_fresh_level_mvar_group Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::Level;

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Create a fresh level metavariable.
    pub fn mk_fresh_level_mvar(&mut self) -> Level {
        let id = self.next_level_id;
        self.next_level_id += 1;
        Level::MVar(oxilean_kernel::LevelMVarId(id))
    }
}
