//! # ElabContext - new_group Methods
//!
//! This module contains method implementations for `ElabContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Environment, Expr, FVarId, Name};
use std::collections::HashMap;

use super::elabcontext_type::ElabContext;
use super::functions::*;
use super::types::ElabOptions;

impl<'env> ElabContext<'env> {
    /// Create a new elaboration context with default options.
    pub fn new(env: &'env Environment) -> Self {
        Self {
            env,
            locals: Vec::new(),
            metas: HashMap::new(),
            next_meta: 0,
            next_fvar: 0,
            univ_params: Vec::new(),
            options: ElabOptions::default(),
            depth: 0,
            pending_goals: Vec::new(),
            expected_type_stack: Vec::new(),
        }
    }
    /// Create with custom options.
    pub fn with_options(env: &'env Environment, options: ElabOptions) -> Self {
        Self {
            env,
            locals: Vec::new(),
            metas: HashMap::new(),
            next_meta: 0,
            next_fvar: 0,
            univ_params: Vec::new(),
            options,
            depth: 0,
            pending_goals: Vec::new(),
            expected_type_stack: Vec::new(),
        }
    }
    /// Zonk an expression: replace all FVar-encoded solved metavariables with
    /// their assignments, recursively.
    ///
    /// Metavariables in `ElabContext` are encoded as `Expr::FVar(FVarId(1_000_000 + id))`.
    /// This method walks the expression, and whenever it encounters such an FVar that
    /// has been assigned (via `assign_meta`), it replaces it with the assignment and
    /// recursively zonks the result (to handle chains of assignments).
    pub fn zonk(&self, expr: &Expr) -> Expr {
        const MVAR_OFFSET: u64 = 1_000_000;
        match expr {
            Expr::FVar(fvar) if fvar.0 >= MVAR_OFFSET => {
                let meta_id = fvar.0 - MVAR_OFFSET;
                if let Some(assigned) = self.get_meta(meta_id) {
                    let assigned = assigned.clone();
                    self.zonk(&assigned)
                } else {
                    expr.clone()
                }
            }
            Expr::App(f, a) => Expr::App(Box::new(self.zonk(f)), Box::new(self.zonk(a))),
            Expr::Lam(info, name, ty, body) => Expr::Lam(
                *info,
                name.clone(),
                Box::new(self.zonk(ty)),
                Box::new(self.zonk(body)),
            ),
            Expr::Pi(info, name, ty, body) => Expr::Pi(
                *info,
                name.clone(),
                Box::new(self.zonk(ty)),
                Box::new(self.zonk(body)),
            ),
            Expr::Let(name, ty, val, body) => Expr::Let(
                name.clone(),
                Box::new(self.zonk(ty)),
                Box::new(self.zonk(val)),
                Box::new(self.zonk(body)),
            ),
            Expr::Proj(name, idx, inner) => {
                Expr::Proj(name.clone(), *idx, Box::new(self.zonk(inner)))
            }
            _ => expr.clone(),
        }
    }
}
