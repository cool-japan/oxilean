//! # CoqTacticExt - Trait Implementations
//!
//! This module contains trait implementations for `CoqTacticExt`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoqTacticExt;
use std::fmt;

impl std::fmt::Display for CoqTacticExt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoqTacticExt::Intro(hs) => write!(f, "intros {}", hs.join(" ")),
            CoqTacticExt::Apply(l) => write!(f, "apply {}", l),
            CoqTacticExt::Exact(e) => write!(f, "exact ({})", e),
            CoqTacticExt::Rewrite(bwd, l) => {
                if *bwd {
                    write!(f, "rewrite <- {}", l)
                } else {
                    write!(f, "rewrite {}", l)
                }
            }
            CoqTacticExt::Simpl => write!(f, "simpl"),
            CoqTacticExt::Ring => write!(f, "ring"),
            CoqTacticExt::Omega => write!(f, "omega"),
            CoqTacticExt::Lia => write!(f, "lia"),
            CoqTacticExt::Lra => write!(f, "lra"),
            CoqTacticExt::Auto => write!(f, "auto"),
            CoqTacticExt::EAuto => write!(f, "eauto"),
            CoqTacticExt::Tauto => write!(f, "tauto"),
            CoqTacticExt::Constructor => write!(f, "constructor"),
            CoqTacticExt::Split => write!(f, "split"),
            CoqTacticExt::Left => write!(f, "left"),
            CoqTacticExt::Right => write!(f, "right"),
            CoqTacticExt::Exists(w) => write!(f, "exists {}", w),
            CoqTacticExt::Induction(h) => write!(f, "induction {}", h),
            CoqTacticExt::Destruct(h) => write!(f, "destruct {}", h),
            CoqTacticExt::Inversion(h) => write!(f, "inversion {}", h),
            CoqTacticExt::Reflexivity => write!(f, "reflexivity"),
            CoqTacticExt::Symmetry => write!(f, "symmetry"),
            CoqTacticExt::Transitivity(t) => write!(f, "transitivity ({})", t),
            CoqTacticExt::Unfold(ls) => write!(f, "unfold {}", ls.join(", ")),
            CoqTacticExt::Fold(ls) => write!(f, "fold {}", ls.join(", ")),
            CoqTacticExt::Assumption => write!(f, "assumption"),
            CoqTacticExt::Contradiction => write!(f, "contradiction"),
            CoqTacticExt::Exfalso => write!(f, "exfalso"),
            CoqTacticExt::Clear(hs) => write!(f, "clear {}", hs.join(" ")),
            CoqTacticExt::Rename(a, b) => write!(f, "rename {} into {}", a, b),
            CoqTacticExt::Trivial => write!(f, "trivial"),
            CoqTacticExt::Discriminate => write!(f, "discriminate"),
            CoqTacticExt::Injection(h) => write!(f, "injection {}", h),
            CoqTacticExt::FApply(l) => write!(f, "eapply {}", l),
            CoqTacticExt::Subst(h) => {
                if let Some(h) = h {
                    write!(f, "subst {}", h)
                } else {
                    write!(f, "subst")
                }
            }
            CoqTacticExt::Custom(s) => write!(f, "{}", s),
        }
    }
}
