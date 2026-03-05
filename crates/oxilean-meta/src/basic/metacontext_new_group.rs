//! # MetaContext - new_group Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Environment, Expr, FVarId, Name};
use std::collections::HashMap;

use super::functions::{abstract_fvar_in_expr, MVAR_FVAR_OFFSET};
use super::types::{LocalDecl, MVarId, MetaConfig, MetavarDecl, MetavarKind};

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Create a new meta context with the given environment.
    pub fn new(env: Environment) -> Self {
        Self {
            mvar_decls: HashMap::new(),
            mvar_assignments: HashMap::new(),
            next_mvar_id: 0,
            level_assignments: HashMap::new(),
            next_level_id: 0,
            local_decls: Vec::new(),
            fvar_map: HashMap::new(),
            next_fvar_id: 0,
            postponed: Vec::new(),
            config: MetaConfig::default(),
            depth: 0,
            env,
        }
    }
    /// Create a meta context with custom configuration.
    pub fn with_config(env: Environment, config: MetaConfig) -> Self {
        let mut ctx = Self::new(env);
        ctx.config = config;
        ctx
    }
    /// Create a fresh expression metavariable with a user-facing name.
    pub fn mk_fresh_expr_mvar_with_name(
        &mut self,
        ty: Expr,
        kind: MetavarKind,
        user_name: Name,
    ) -> (MVarId, Expr) {
        let id = MVarId(self.next_mvar_id);
        self.next_mvar_id += 1;
        let lctx_snapshot: Vec<FVarId> = self.local_decls.iter().map(|d| d.fvar_id).collect();
        let decl = MetavarDecl {
            ty: ty.clone(),
            lctx_snapshot,
            kind,
            user_name,
            num_scope_args: 0,
            depth: self.depth,
        };
        self.mvar_decls.insert(id, decl);
        let placeholder = Expr::FVar(FVarId::new(id.0 + MVAR_FVAR_OFFSET));
        (id, placeholder)
    }
    /// Implementation of mvar instantiation with depth tracking.
    pub(super) fn instantiate_mvars_impl(&self, expr: &Expr, depth: u32) -> Expr {
        if depth > self.config.max_recursion_depth {
            return expr.clone();
        }
        if let Some(id) = Self::is_mvar_expr(expr) {
            if let Some(val) = self.mvar_assignments.get(&id) {
                return self.instantiate_mvars_impl(val, depth + 1);
            }
            return expr.clone();
        }
        match expr {
            Expr::Sort(_) | Expr::BVar(_) | Expr::Const(_, _) | Expr::Lit(_) => expr.clone(),
            Expr::FVar(_) => expr.clone(),
            Expr::App(f, a) => {
                let f2 = self.instantiate_mvars_impl(f, depth + 1);
                let a2 = self.instantiate_mvars_impl(a, depth + 1);
                Expr::App(Box::new(f2), Box::new(a2))
            }
            Expr::Lam(info, name, ty, body) => {
                let ty2 = self.instantiate_mvars_impl(ty, depth + 1);
                let body2 = self.instantiate_mvars_impl(body, depth + 1);
                Expr::Lam(*info, name.clone(), Box::new(ty2), Box::new(body2))
            }
            Expr::Pi(info, name, ty, body) => {
                let ty2 = self.instantiate_mvars_impl(ty, depth + 1);
                let body2 = self.instantiate_mvars_impl(body, depth + 1);
                Expr::Pi(*info, name.clone(), Box::new(ty2), Box::new(body2))
            }
            Expr::Let(name, ty, val, body) => {
                let ty2 = self.instantiate_mvars_impl(ty, depth + 1);
                let val2 = self.instantiate_mvars_impl(val, depth + 1);
                let body2 = self.instantiate_mvars_impl(body, depth + 1);
                Expr::Let(name.clone(), Box::new(ty2), Box::new(val2), Box::new(body2))
            }
            Expr::Proj(name, idx, e) => {
                let e2 = self.instantiate_mvars_impl(e, depth + 1);
                Expr::Proj(name.clone(), *idx, Box::new(e2))
            }
        }
    }
    /// Add a local declaration (regular variable).
    pub fn mk_local_decl(&mut self, user_name: Name, ty: Expr, binder_info: BinderInfo) -> FVarId {
        let fvar_id = FVarId::new(self.next_fvar_id);
        self.next_fvar_id += 1;
        let index = self.local_decls.len() as u32;
        let decl = LocalDecl {
            fvar_id,
            user_name,
            ty,
            binder_info,
            value: None,
            index,
        };
        self.fvar_map.insert(fvar_id, self.local_decls.len());
        self.local_decls.push(decl);
        fvar_id
    }
    /// Add a let-binding to the local context.
    pub fn mk_let_decl(&mut self, user_name: Name, ty: Expr, value: Expr) -> FVarId {
        let fvar_id = FVarId::new(self.next_fvar_id);
        self.next_fvar_id += 1;
        let index = self.local_decls.len() as u32;
        let decl = LocalDecl {
            fvar_id,
            user_name,
            ty,
            binder_info: BinderInfo::Default,
            value: Some(value),
            index,
        };
        self.fvar_map.insert(fvar_id, self.local_decls.len());
        self.local_decls.push(decl);
        fvar_id
    }
    /// Build a lambda from free variables.
    ///
    /// Given `fvars = [x₁, ..., xₙ]` and `body`, builds:
    /// `λ (x₁ : τ₁) ... (xₙ : τₙ), body[xᵢ ↦ #(n-i)]`
    pub fn mk_lambda(&self, fvars: &[FVarId], body: Expr) -> Expr {
        let mut result = body;
        for (i, fvar_id) in fvars.iter().rev().enumerate() {
            if let Some(decl) = self.find_local_decl(*fvar_id) {
                result = abstract_fvar_in_expr(&result, *fvar_id, i as u32);
                result = Expr::Lam(
                    decl.binder_info,
                    decl.user_name.clone(),
                    Box::new(decl.ty.clone()),
                    Box::new(result),
                );
            }
        }
        result
    }
    /// Build a pi type from free variables.
    ///
    /// Given `fvars = [x₁, ..., xₙ]` and `body`, builds:
    /// `Π (x₁ : τ₁) ... (xₙ : τₙ), body[xᵢ ↦ #(n-i)]`
    pub fn mk_pi(&self, fvars: &[FVarId], body: Expr) -> Expr {
        let mut result = body;
        for (i, fvar_id) in fvars.iter().rev().enumerate() {
            if let Some(decl) = self.find_local_decl(*fvar_id) {
                result = abstract_fvar_in_expr(&result, *fvar_id, i as u32);
                result = Expr::Pi(
                    decl.binder_info,
                    decl.user_name.clone(),
                    Box::new(decl.ty.clone()),
                    Box::new(result),
                );
            }
        }
        result
    }
}
