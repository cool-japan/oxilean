//! # MetaVarContext - new_group Methods
//!
//! This module contains method implementations for `MetaVarContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::Expr;
use std::collections::{HashMap, HashSet};

use super::functions::*;
use super::metavarcontext_type::MetaVarContext;
use super::types::MetaVar;

impl MetaVarContext {
    /// Create a new context.
    pub fn new() -> Self {
        Self {
            metas: HashMap::new(),
            next_id: 0,
            constraints: Vec::new(),
            depth: 0,
            frozen: HashSet::new(),
            current_scope: Vec::new(),
        }
    }
    /// Create a fresh natural metavariable (captures current scope).
    pub fn fresh(&mut self, ty: Expr) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let scope = self.current_scope.clone();
        self.metas.insert(
            id,
            MetaVar::new(id, ty)
                .with_depth(self.depth)
                .with_scope(scope),
        );
        id
    }
    /// Create a named metavariable (captures current scope).
    pub fn fresh_named(&mut self, ty: Expr, name: impl Into<String>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let scope = self.current_scope.clone();
        self.metas.insert(
            id,
            MetaVar::new(id, ty)
                .with_name(name)
                .with_depth(self.depth)
                .with_scope(scope),
        );
        id
    }
    /// Instantiate an expression by replacing solved metavariables.
    pub fn instantiate(&self, expr: &Expr) -> Expr {
        match expr {
            Expr::App(f, a) => {
                Expr::App(Box::new(self.instantiate(f)), Box::new(self.instantiate(a)))
            }
            Expr::Lam(info, name, ty, body) => Expr::Lam(
                *info,
                name.clone(),
                Box::new(self.instantiate(ty)),
                Box::new(self.instantiate(body)),
            ),
            Expr::Pi(info, name, ty, body) => Expr::Pi(
                *info,
                name.clone(),
                Box::new(self.instantiate(ty)),
                Box::new(self.instantiate(body)),
            ),
            Expr::Let(name, ty, val, body) => Expr::Let(
                name.clone(),
                Box::new(self.instantiate(ty)),
                Box::new(self.instantiate(val)),
                Box::new(self.instantiate(body)),
            ),
            _ => expr.clone(),
        }
    }
    /// Zonk an expression: replace all solved metavariables with their assignments.
    ///
    /// Unlike `instantiate`, this also handles `Proj` and `Let` sub-expressions,
    /// and recursively zonks the assignment value (to handle chains of assignments).
    /// Since `MetaVarContext` does not embed metavariable IDs into expressions as
    /// `FVar` nodes, this method simply recursively traverses the expression and
    /// rebuilds compound nodes, identical in structure to `instantiate`.  Callers
    /// that use the `ElabContext` FVar-encoding for metavariables should use
    /// `ElabContext::zonk` instead.
    pub fn zonk(&self, expr: &Expr) -> Expr {
        match expr {
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
