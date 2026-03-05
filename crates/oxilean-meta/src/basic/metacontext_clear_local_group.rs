//! # MetaContext - clear_local_group Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::Name;

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Remove a local declaration by user name.
    /// Returns true if the declaration was found and removed.
    pub fn clear_local(&mut self, name: &Name) -> bool {
        if let Some(idx) = self.local_decls.iter().position(|d| &d.user_name == name) {
            let decl = self.local_decls.remove(idx);
            self.fvar_map.remove(&decl.fvar_id);
            for (i, d) in self.local_decls.iter().enumerate() {
                self.fvar_map.insert(d.fvar_id, i);
            }
            true
        } else {
            false
        }
    }
}
