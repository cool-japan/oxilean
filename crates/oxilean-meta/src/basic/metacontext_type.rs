//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Environment, Expr, FVarId, Level};
use std::collections::HashMap;

use super::types::{LocalDecl, MVarId, MetaConfig, MetavarDecl, PostponedConstraint};

/// The core meta context.
///
/// This is the central state for all meta-level operations:
/// metavariable management, local context, unification state.
pub struct MetaContext {
    /// Metavariable declarations (type, context, kind).
    pub(super) mvar_decls: HashMap<MVarId, MetavarDecl>,
    /// Metavariable assignments.
    pub(super) mvar_assignments: HashMap<MVarId, Expr>,
    /// Next metavariable ID.
    pub(super) next_mvar_id: u64,
    /// Level metavariable assignments.
    pub(super) level_assignments: HashMap<u64, Level>,
    /// Next level mvar ID.
    pub(super) next_level_id: u64,
    /// Local variable declarations ordered by index.
    pub(super) local_decls: Vec<LocalDecl>,
    /// Map from FVarId to index in local_decls.
    pub(super) fvar_map: HashMap<FVarId, usize>,
    /// Next free variable ID.
    pub(super) next_fvar_id: u64,
    /// Constraints that couldn't be solved immediately.
    pub(super) postponed: Vec<PostponedConstraint>,
    /// Meta configuration.
    pub(super) config: MetaConfig,
    /// Current depth (for scoping metavariables).
    pub(super) depth: u32,
    /// The kernel environment (immutable during meta operations).
    pub(super) env: Environment,
}
