//! Congruence before unfolding: `f a ≡ f b` from `a ≡ b`.

use super::types::DefEqChecker;
use crate::expr_util::{get_app_args, get_app_fn};
use crate::{level, Expr};

impl DefEqChecker<'_> {
    /// Whether `t` and `s` are applications of one and the same unfoldable
    /// definition, at equivalent universe levels, to the same number of
    /// arguments, and the arguments are pairwise definitionally equal.
    ///
    /// `true` is conclusive — `f a₁ … aₙ ≡ f b₁ … bₙ` follows from
    /// `aᵢ ≡ bᵢ` by congruence, with no reduction of `f` at all. `false`
    /// decides nothing: the caller goes on to reduce both sides.
    ///
    /// Why it matters. Reducing first, as the def-eq core otherwise
    /// does, unfolds `f` on both sides and compares what comes out. When `f`
    /// recurses on an argument that is stuck — `Nat.mod x USize.size`, whose
    /// divisor is `2 ^ System.Platform.numBits` — what comes out is a stuck
    /// eliminator whose branches hold the next recursive call, and comparing
    /// two of those walks the recursion one unfolding at a time, as far as
    /// the budget lasts (`(2^64 - 1) % USize.size` against
    /// `(2^64 - 1) % 2 ^ numBits` is 2^64 steps). Lean's kernel never gets
    /// there: its lazy delta reduction compares the arguments of two
    /// applications of the same definition before it unfolds either.
    ///
    /// Only an unfoldable definition is tried here. For a head the reducer
    /// leaves alone (a constructor, an axiom, an opaque constant, a stuck
    /// recursor) the structural comparison after reduction compares the
    /// arguments anyway.
    pub(super) fn same_definition_applied_to_equal_args(&mut self, t: &Expr, s: &Expr) -> bool {
        if !matches!((t, s), (Expr::App(_, _), Expr::App(_, _))) {
            return false;
        }
        let (Expr::Const(t_name, t_levels), Expr::Const(s_name, s_levels)) =
            (get_app_fn(t), get_app_fn(s))
        else {
            return false;
        };
        if t_name != s_name || self.get_hint(get_app_fn(t)).is_none() {
            return false;
        }
        if t_levels.len() != s_levels.len()
            || !t_levels
                .iter()
                .zip(s_levels.iter())
                .all(|(l1, l2)| level::is_equivalent(l1, l2))
        {
            return false;
        }
        let t_args = get_app_args(t);
        let s_args = get_app_args(s);
        t_args.len() == s_args.len()
            && t_args
                .iter()
                .zip(s_args.iter())
                .all(|(a, b)| self.is_def_eq(a, b))
    }
}
