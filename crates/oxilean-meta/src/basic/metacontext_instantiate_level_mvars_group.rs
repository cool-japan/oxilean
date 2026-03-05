//! # MetaContext - instantiate_level_mvars_group Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::Level;

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Instantiate level metavariables in a level.
    pub fn instantiate_level_mvars(&self, level: &Level) -> Level {
        match level {
            Level::MVar(oxilean_kernel::LevelMVarId(id)) => {
                if let Some(assigned) = self.level_assignments.get(id) {
                    self.instantiate_level_mvars(assigned)
                } else {
                    level.clone()
                }
            }
            Level::Succ(inner) => Level::succ(self.instantiate_level_mvars(inner)),
            Level::Max(l, r) => Level::max(
                self.instantiate_level_mvars(l),
                self.instantiate_level_mvars(r),
            ),
            Level::IMax(l, r) => Level::imax(
                self.instantiate_level_mvars(l),
                self.instantiate_level_mvars(r),
            ),
            _ => level.clone(),
        }
    }
}
