//! # MetaContext - instantiate_level_mvars_group Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Level, LevelView};

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Instantiate level metavariables in a level.
    pub fn instantiate_level_mvars(&self, level: &Level) -> Level {
        match level.view() {
            LevelView::MVar(oxilean_kernel::LevelMVarId(id)) => {
                if let Some(assigned) = self.level_assignments.get(&id) {
                    self.instantiate_level_mvars(assigned)
                } else {
                    level.clone()
                }
            }
            LevelView::Succ(inner) => Level::succ(self.instantiate_level_mvars(inner)),
            LevelView::Max(l, r) => Level::max(
                self.instantiate_level_mvars(l),
                self.instantiate_level_mvars(r),
            ),
            LevelView::IMax(l, r) => Level::imax(
                self.instantiate_level_mvars(l),
                self.instantiate_level_mvars(r),
            ),
            _ => level.clone(),
        }
    }
}
