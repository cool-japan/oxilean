//! # MetaContext - predicates Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::Expr;

use super::functions::MVAR_FVAR_OFFSET;
use super::types::MVarId;

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Check if an expression is a metavariable placeholder.
    pub fn is_mvar_expr(e: &Expr) -> Option<MVarId> {
        if let Expr::FVar(fid) = e {
            if fid.0 >= MVAR_FVAR_OFFSET {
                return Some(MVarId(fid.0 - MVAR_FVAR_OFFSET));
            }
        }
        None
    }
    /// Check if an expression contains any unassigned metavariables.
    pub fn has_unassigned_mvars(&self, expr: &Expr) -> bool {
        if let Some(id) = Self::is_mvar_expr(expr) {
            return !self.mvar_assignments.contains_key(&id);
        }
        match expr {
            Expr::Sort(_) | Expr::BVar(_) | Expr::Const(_, _) | Expr::Lit(_) | Expr::FVar(_) => {
                false
            }
            Expr::App(f, a) => self.has_unassigned_mvars(f) || self.has_unassigned_mvars(a),
            Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
                self.has_unassigned_mvars(ty) || self.has_unassigned_mvars(body)
            }
            Expr::Let(_, ty, val, body) => {
                self.has_unassigned_mvars(ty)
                    || self.has_unassigned_mvars(val)
                    || self.has_unassigned_mvars(body)
            }
            Expr::Proj(_, _, e) => self.has_unassigned_mvars(e),
        }
    }
}
