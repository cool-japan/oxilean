//! # ExprKey - Trait Implementations
//!
//! This module contains trait implementations for `ExprKey`.
//!
//! ## Implemented Traits
//!
//! - `Hash`
//! - `PartialEq`
//! - `Eq`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{BinderInfo, Expr, Level, Literal, Name};
use std::hash::{Hash, Hasher};

use super::functions::{hash_binder_info, hash_level, hash_literal, hash_name, hash_tag};
use super::types::ExprKey;

impl Hash for ExprKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self.0 {
            Expr::Sort(lvl) => {
                hash_tag(state, 0);
                hash_level(lvl, state);
            }
            Expr::BVar(idx) => {
                hash_tag(state, 1);
                idx.hash(state);
            }
            Expr::FVar(fvar_id) => {
                hash_tag(state, 2);
                fvar_id.hash(state);
            }
            Expr::Const(name, levels) => {
                hash_tag(state, 3);
                hash_name(name, state);
                for lvl in levels {
                    hash_level(lvl, state);
                }
            }
            Expr::App(f, a) => {
                hash_tag(state, 4);
                ExprKey(*f.clone()).hash(state);
                ExprKey(*a.clone()).hash(state);
            }
            Expr::Lam(bi, name, ty, body) => {
                hash_tag(state, 5);
                hash_binder_info(bi, state);
                hash_name(name, state);
                ExprKey(*ty.clone()).hash(state);
                ExprKey(*body.clone()).hash(state);
            }
            Expr::Pi(bi, name, ty, body) => {
                hash_tag(state, 6);
                hash_binder_info(bi, state);
                hash_name(name, state);
                ExprKey(*ty.clone()).hash(state);
                ExprKey(*body.clone()).hash(state);
            }
            Expr::Let(name, ty, val, body) => {
                hash_tag(state, 7);
                hash_name(name, state);
                ExprKey(*ty.clone()).hash(state);
                ExprKey(*val.clone()).hash(state);
                ExprKey(*body.clone()).hash(state);
            }
            Expr::Lit(lit) => {
                hash_tag(state, 8);
                hash_literal(lit, state);
            }
            Expr::Proj(name, idx, expr) => {
                hash_tag(state, 9);
                hash_name(name, state);
                idx.hash(state);
                ExprKey(*expr.clone()).hash(state);
            }
        }
    }
}

impl PartialEq for ExprKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for ExprKey {}
