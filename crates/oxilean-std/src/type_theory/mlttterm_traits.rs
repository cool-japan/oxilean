//! # MlttTerm - Trait Implementations
//!
//! This module contains trait implementations for `MlttTerm`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MlttTerm;
use std::fmt;

impl std::fmt::Display for MlttTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MlttTerm::Var(n) => write!(f, "{n}"),
            MlttTerm::Type(l) => write!(f, "Type_{}", l.0),
            MlttTerm::Pi {
                binder,
                domain,
                codomain,
            } => {
                write!(f, "Π ({binder} : {domain}), {codomain}")
            }
            MlttTerm::Lam { binder, body } => write!(f, "λ {binder}. {body}"),
            MlttTerm::App(fun, arg) => write!(f, "({fun} {arg})"),
            MlttTerm::Sigma { binder, fst, snd } => {
                write!(f, "Σ ({binder} : {fst}), {snd}")
            }
            MlttTerm::Pair(a, b) => write!(f, "({a}, {b})"),
            MlttTerm::Fst(t) => write!(f, "fst {t}"),
            MlttTerm::Snd(t) => write!(f, "snd {t}"),
            MlttTerm::Id { ty, lhs, rhs } => write!(f, "({lhs} =_{ty} {rhs})"),
            MlttTerm::Refl(t) => write!(f, "refl {t}"),
            MlttTerm::J { motive, base, path } => {
                write!(f, "J({motive}, {base}, {path})")
            }
            MlttTerm::Nat => write!(f, "Nat"),
            MlttTerm::Zero => write!(f, "0"),
            MlttTerm::Succ(t) => write!(f, "S({t})"),
            MlttTerm::NatRec {
                motive,
                base,
                step,
                n,
            } => {
                write!(f, "NatRec({motive}, {base}, {step}, {n})")
            }
            MlttTerm::Unit => write!(f, "Unit"),
            MlttTerm::Star => write!(f, "⋆"),
            MlttTerm::Empty => write!(f, "Empty"),
            MlttTerm::Abort(t) => write!(f, "abort({t})"),
        }
    }
}
