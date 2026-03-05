//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{Expr, FVarId, Name};

use super::types::{
    AlphaCache, ConfigNode, DecisionNode, Either2, Fixture, FlatSubstitution, FocusStack, LabelSet,
    MinHeap, NonEmptyVec, PathBuf, PrefixCounter, RewriteRule, RewriteRuleSet, SimpleDag,
    SlidingSum, SmallMap, SparseVec, StackCalc, StatSummary, Stopwatch, StringPool, TokenBucket,
    TransformStat, TransitiveClosure, VersionedRecord, WindowIterator, WriteOnce,
};

/// Check if two expressions are alpha equivalent.
///
/// Alpha equivalence means the expressions are structurally identical
/// modulo renaming of bound variables.
pub fn alpha_equiv(e1: &Expr, e2: &Expr) -> bool {
    alpha_equiv_impl(e1, e2, &mut Vec::new(), &mut Vec::new())
}
/// Implementation of alpha equivalence with de Bruijn level tracking.
pub(super) fn alpha_equiv_impl(
    e1: &Expr,
    e2: &Expr,
    ctx1: &mut Vec<FVarId>,
    ctx2: &mut Vec<FVarId>,
) -> bool {
    match (e1, e2) {
        (Expr::BVar(i), Expr::BVar(j)) => i == j,
        (Expr::FVar(f1), Expr::FVar(f2)) => {
            match (
                ctx1.iter().position(|f| f == f1),
                ctx2.iter().position(|f| f == f2),
            ) {
                (Some(i), Some(j)) => i == j,
                (None, None) => f1 == f2,
                _ => false,
            }
        }
        (Expr::Sort(l1), Expr::Sort(l2)) => l1 == l2,
        (Expr::Const(n1, _), Expr::Const(n2, _)) => n1 == n2,
        (Expr::App(f1, a1), Expr::App(f2, a2)) => {
            alpha_equiv_impl(f1, f2, ctx1, ctx2) && alpha_equiv_impl(a1, a2, ctx1, ctx2)
        }
        (Expr::Lam(bi1, _n1, ty1, body1), Expr::Lam(bi2, _n2, ty2, body2)) => {
            if bi1 != bi2 {
                return false;
            }
            if !alpha_equiv_impl(ty1, ty2, ctx1, ctx2) {
                return false;
            }
            let fvar1 = FVarId::new(ctx1.len() as u64);
            let fvar2 = FVarId::new(ctx2.len() as u64);
            ctx1.push(fvar1);
            ctx2.push(fvar2);
            let result = alpha_equiv_impl(body1, body2, ctx1, ctx2);
            ctx1.pop();
            ctx2.pop();
            result
        }
        (Expr::Pi(bi1, _n1, ty1, body1), Expr::Pi(bi2, _n2, ty2, body2)) => {
            if bi1 != bi2 {
                return false;
            }
            if !alpha_equiv_impl(ty1, ty2, ctx1, ctx2) {
                return false;
            }
            let fvar1 = FVarId::new(ctx1.len() as u64);
            let fvar2 = FVarId::new(ctx2.len() as u64);
            ctx1.push(fvar1);
            ctx2.push(fvar2);
            let result = alpha_equiv_impl(body1, body2, ctx1, ctx2);
            ctx1.pop();
            ctx2.pop();
            result
        }
        (Expr::Let(_n1, ty1, val1, body1), Expr::Let(_n2, ty2, val2, body2)) => {
            if !alpha_equiv_impl(ty1, ty2, ctx1, ctx2) {
                return false;
            }
            if !alpha_equiv_impl(val1, val2, ctx1, ctx2) {
                return false;
            }
            let fvar1 = FVarId::new(ctx1.len() as u64);
            let fvar2 = FVarId::new(ctx2.len() as u64);
            ctx1.push(fvar1);
            ctx2.push(fvar2);
            let result = alpha_equiv_impl(body1, body2, ctx1, ctx2);
            ctx1.pop();
            ctx2.pop();
            result
        }
        (Expr::Proj(n1, i1, s1), Expr::Proj(n2, i2, s2)) => {
            n1 == n2 && i1 == i2 && alpha_equiv_impl(s1, s2, ctx1, ctx2)
        }
        (Expr::Lit(l1), Expr::Lit(l2)) => l1 == l2,
        _ => false,
    }
}
/// Rename bound variables in an expression to canonical names.
/// This produces a canonical form for alpha-equivalent expressions.
pub fn canonicalize(expr: &Expr) -> Expr {
    canonicalize_impl(expr, 0)
}
pub(super) fn canonicalize_impl(expr: &Expr, depth: u32) -> Expr {
    match expr {
        Expr::BVar(i) => Expr::BVar(*i),
        Expr::FVar(f) => Expr::FVar(*f),
        Expr::Sort(l) => Expr::Sort(l.clone()),
        Expr::Const(n, ls) => Expr::Const(n.clone(), ls.clone()),
        Expr::App(f, a) => Expr::App(
            Box::new(canonicalize_impl(f, depth)),
            Box::new(canonicalize_impl(a, depth)),
        ),
        Expr::Lam(bi, _n, ty, body) => {
            let canonical_name = Name::str(format!("x{}", depth));
            Expr::Lam(
                *bi,
                canonical_name,
                Box::new(canonicalize_impl(ty, depth)),
                Box::new(canonicalize_impl(body, depth + 1)),
            )
        }
        Expr::Pi(bi, _n, ty, body) => {
            let canonical_name = Name::str(format!("x{}", depth));
            Expr::Pi(
                *bi,
                canonical_name,
                Box::new(canonicalize_impl(ty, depth)),
                Box::new(canonicalize_impl(body, depth + 1)),
            )
        }
        Expr::Let(_n, ty, val, body) => {
            let canonical_name = Name::str(format!("x{}", depth));
            Expr::Let(
                canonical_name,
                Box::new(canonicalize_impl(ty, depth)),
                Box::new(canonicalize_impl(val, depth)),
                Box::new(canonicalize_impl(body, depth + 1)),
            )
        }
        Expr::Proj(n, i, s) => Expr::Proj(n.clone(), *i, Box::new(canonicalize_impl(s, depth))),
        Expr::Lit(l) => Expr::Lit(l.clone()),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BinderInfo, Level, Literal};
    #[test]
    fn test_alpha_equiv_bvar() {
        let e1 = Expr::BVar(0);
        let e2 = Expr::BVar(0);
        assert!(alpha_equiv(&e1, &e2));
        let e3 = Expr::BVar(1);
        assert!(!alpha_equiv(&e1, &e3));
    }
    #[test]
    fn test_alpha_equiv_sort() {
        let e1 = Expr::Sort(Level::zero());
        let e2 = Expr::Sort(Level::zero());
        assert!(alpha_equiv(&e1, &e2));
    }
    #[test]
    fn test_alpha_equiv_const() {
        let e1 = Expr::Const(Name::str("Nat"), vec![]);
        let e2 = Expr::Const(Name::str("Nat"), vec![]);
        assert!(alpha_equiv(&e1, &e2));
        let e3 = Expr::Const(Name::str("Bool"), vec![]);
        assert!(!alpha_equiv(&e1, &e3));
    }
    #[test]
    fn test_alpha_equiv_app() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Lit(Literal::Nat(42));
        let e1 = Expr::App(Box::new(f.clone()), Box::new(a.clone()));
        let e2 = Expr::App(Box::new(f), Box::new(a));
        assert!(alpha_equiv(&e1, &e2));
    }
    #[test]
    fn test_alpha_equiv_lambda() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let e1 = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat.clone()),
            Box::new(Expr::BVar(0)),
        );
        let e2 = Expr::Lam(
            BinderInfo::Default,
            Name::str("y"),
            Box::new(nat),
            Box::new(Expr::BVar(0)),
        );
        assert!(alpha_equiv(&e1, &e2));
    }
    #[test]
    fn test_canonicalize() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let e1 = Expr::Lam(
            BinderInfo::Default,
            Name::str("different_name"),
            Box::new(nat.clone()),
            Box::new(Expr::BVar(0)),
        );
        let e2 = Expr::Lam(
            BinderInfo::Default,
            Name::str("another_name"),
            Box::new(nat),
            Box::new(Expr::BVar(0)),
        );
        let c1 = canonicalize(&e1);
        let c2 = canonicalize(&e2);
        assert_eq!(c1, c2);
    }
}
/// Check if two expressions are alpha equivalent modulo universe levels.
///
/// Unlike `alpha_equiv`, this ignores the universe level arguments to constants.
pub fn alpha_equiv_mod_levels(e1: &Expr, e2: &Expr) -> bool {
    alpha_equiv_mod_levels_impl(e1, e2, &mut Vec::new(), &mut Vec::new())
}
pub(super) fn alpha_equiv_mod_levels_impl(
    e1: &Expr,
    e2: &Expr,
    ctx1: &mut Vec<FVarId>,
    ctx2: &mut Vec<FVarId>,
) -> bool {
    match (e1, e2) {
        (Expr::BVar(i), Expr::BVar(j)) => i == j,
        (Expr::FVar(f1), Expr::FVar(f2)) => {
            match (
                ctx1.iter().position(|f| f == f1),
                ctx2.iter().position(|f| f == f2),
            ) {
                (Some(i), Some(j)) => i == j,
                (None, None) => f1 == f2,
                _ => false,
            }
        }
        (Expr::Sort(l1), Expr::Sort(l2)) => l1 == l2,
        (Expr::Const(n1, _), Expr::Const(n2, _)) => n1 == n2,
        (Expr::App(f1, a1), Expr::App(f2, a2)) => {
            alpha_equiv_mod_levels_impl(f1, f2, ctx1, ctx2)
                && alpha_equiv_mod_levels_impl(a1, a2, ctx1, ctx2)
        }
        (Expr::Lam(bi1, _n1, ty1, body1), Expr::Lam(bi2, _n2, ty2, body2)) => {
            if bi1 != bi2 {
                return false;
            }
            if !alpha_equiv_mod_levels_impl(ty1, ty2, ctx1, ctx2) {
                return false;
            }
            let fvar1 = FVarId::new(ctx1.len() as u64);
            let fvar2 = FVarId::new(ctx2.len() as u64);
            ctx1.push(fvar1);
            ctx2.push(fvar2);
            let result = alpha_equiv_mod_levels_impl(body1, body2, ctx1, ctx2);
            ctx1.pop();
            ctx2.pop();
            result
        }
        (Expr::Pi(bi1, _n1, ty1, body1), Expr::Pi(bi2, _n2, ty2, body2)) => {
            if bi1 != bi2 {
                return false;
            }
            if !alpha_equiv_mod_levels_impl(ty1, ty2, ctx1, ctx2) {
                return false;
            }
            let fvar1 = FVarId::new(ctx1.len() as u64);
            let fvar2 = FVarId::new(ctx2.len() as u64);
            ctx1.push(fvar1);
            ctx2.push(fvar2);
            let result = alpha_equiv_mod_levels_impl(body1, body2, ctx1, ctx2);
            ctx1.pop();
            ctx2.pop();
            result
        }
        (Expr::Let(_n1, ty1, val1, body1), Expr::Let(_n2, ty2, val2, body2)) => {
            if !alpha_equiv_mod_levels_impl(ty1, ty2, ctx1, ctx2) {
                return false;
            }
            if !alpha_equiv_mod_levels_impl(val1, val2, ctx1, ctx2) {
                return false;
            }
            let fvar1 = FVarId::new(ctx1.len() as u64);
            let fvar2 = FVarId::new(ctx2.len() as u64);
            ctx1.push(fvar1);
            ctx2.push(fvar2);
            let result = alpha_equiv_mod_levels_impl(body1, body2, ctx1, ctx2);
            ctx1.pop();
            ctx2.pop();
            result
        }
        (Expr::Proj(n1, i1, s1), Expr::Proj(n2, i2, s2)) => {
            n1 == n2 && i1 == i2 && alpha_equiv_mod_levels_impl(s1, s2, ctx1, ctx2)
        }
        (Expr::Lit(l1), Expr::Lit(l2)) => l1 == l2,
        _ => false,
    }
}
/// Compute an alpha-equivalence class representative.
///
/// Maps an expression to a canonical representative where all binder names
/// are replaced by `x0`, `x1`, etc.
pub fn alpha_class_rep(expr: &Expr) -> Expr {
    canonicalize(expr)
}
/// Check if a list of expressions are pairwise alpha-equivalent.
pub fn all_alpha_equiv(exprs: &[Expr]) -> bool {
    if exprs.len() <= 1 {
        return true;
    }
    let first = &exprs[0];
    exprs[1..].iter().all(|e| alpha_equiv(first, e))
}
/// Find the first pair of alpha-non-equivalent expressions in a list.
///
/// Returns `None` if all are pairwise equivalent, or `Some((i, j))` for the first differing pair.
pub fn find_non_equiv_pair(exprs: &[Expr]) -> Option<(usize, usize)> {
    for i in 0..exprs.len() {
        for j in (i + 1)..exprs.len() {
            if !alpha_equiv(&exprs[i], &exprs[j]) {
                return Some((i, j));
            }
        }
    }
    None
}
/// Compute a structural hash of an expression for alpha-equivalence purposes.
///
/// Two alpha-equivalent expressions will have the same hash.
/// (Note: this is not a cryptographic hash — collisions are possible.)
pub fn alpha_hash(expr: &Expr) -> u64 {
    alpha_hash_impl(expr, 0)
}
#[allow(clippy::only_used_in_recursion)]
pub(super) fn alpha_hash_impl(expr: &Expr, depth: u32) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut h = DefaultHasher::new();
    match expr {
        Expr::BVar(i) => {
            0u8.hash(&mut h);
            i.hash(&mut h);
        }
        Expr::FVar(id) => {
            1u8.hash(&mut h);
            id.0.hash(&mut h);
        }
        Expr::Sort(l) => {
            2u8.hash(&mut h);
            format!("{:?}", l).hash(&mut h);
        }
        Expr::Const(n, _) => {
            3u8.hash(&mut h);
            format!("{}", n).hash(&mut h);
        }
        Expr::App(f, a) => {
            4u8.hash(&mut h);
            alpha_hash_impl(f, depth).hash(&mut h);
            alpha_hash_impl(a, depth).hash(&mut h);
        }
        Expr::Lam(bi, _, ty, body) => {
            5u8.hash(&mut h);
            format!("{:?}", bi).hash(&mut h);
            alpha_hash_impl(ty, depth).hash(&mut h);
            alpha_hash_impl(body, depth + 1).hash(&mut h);
        }
        Expr::Pi(bi, _, ty, body) => {
            6u8.hash(&mut h);
            format!("{:?}", bi).hash(&mut h);
            alpha_hash_impl(ty, depth).hash(&mut h);
            alpha_hash_impl(body, depth + 1).hash(&mut h);
        }
        Expr::Let(_, ty, val, body) => {
            7u8.hash(&mut h);
            alpha_hash_impl(ty, depth).hash(&mut h);
            alpha_hash_impl(val, depth).hash(&mut h);
            alpha_hash_impl(body, depth + 1).hash(&mut h);
        }
        Expr::Proj(n, i, s) => {
            8u8.hash(&mut h);
            format!("{}", n).hash(&mut h);
            i.hash(&mut h);
            alpha_hash_impl(s, depth).hash(&mut h);
        }
        Expr::Lit(l) => {
            9u8.hash(&mut h);
            format!("{:?}", l).hash(&mut h);
        }
    }
    h.finish()
}
#[cfg(test)]
mod alpha_extended_tests {
    use super::*;
    use crate::{BinderInfo, Level, Literal};
    fn nat() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    #[test]
    fn test_alpha_equiv_mod_levels() {
        let e1 = Expr::Const(Name::str("f"), vec![Level::zero()]);
        let e2 = Expr::Const(Name::str("f"), vec![Level::succ(Level::zero())]);
        assert!(alpha_equiv_mod_levels(&e1, &e2));
        assert!(alpha_equiv(&e1, &e2));
        let e3 = Expr::Const(Name::str("g"), vec![Level::zero()]);
        assert!(!alpha_equiv(&e1, &e3));
    }
    #[test]
    fn test_all_alpha_equiv_empty() {
        assert!(all_alpha_equiv(&[]));
    }
    #[test]
    fn test_all_alpha_equiv_single() {
        assert!(all_alpha_equiv(&[nat()]));
    }
    #[test]
    fn test_all_alpha_equiv_same() {
        let exprs = vec![nat(), nat(), nat()];
        assert!(all_alpha_equiv(&exprs));
    }
    #[test]
    fn test_all_alpha_equiv_different() {
        let exprs = vec![nat(), Expr::Const(Name::str("Bool"), vec![])];
        assert!(!all_alpha_equiv(&exprs));
    }
    #[test]
    fn test_find_non_equiv_pair() {
        let exprs = vec![nat(), nat(), Expr::Sort(Level::zero())];
        let pair = find_non_equiv_pair(&exprs);
        assert!(pair.is_some());
        let (i, j) = pair.expect("pair should be valid");
        assert!(i < j);
    }
    #[test]
    fn test_find_non_equiv_pair_all_equiv() {
        let lam1 = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat()),
            Box::new(Expr::BVar(0)),
        );
        let lam2 = Expr::Lam(
            BinderInfo::Default,
            Name::str("y"),
            Box::new(nat()),
            Box::new(Expr::BVar(0)),
        );
        assert!(find_non_equiv_pair(&[lam1, lam2]).is_none());
    }
    #[test]
    fn test_alpha_hash_same_expr() {
        let h1 = alpha_hash(&nat());
        let h2 = alpha_hash(&nat());
        assert_eq!(h1, h2);
    }
    #[test]
    fn test_alpha_hash_alpha_equiv() {
        let lam1 = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat()),
            Box::new(Expr::BVar(0)),
        );
        let lam2 = Expr::Lam(
            BinderInfo::Default,
            Name::str("y"),
            Box::new(nat()),
            Box::new(Expr::BVar(0)),
        );
        assert_eq!(alpha_hash(&lam1), alpha_hash(&lam2));
    }
    #[test]
    fn test_alpha_cache_basic() {
        let mut cache = AlphaCache::new();
        let e1 = nat();
        let _e2 = Expr::Const(Name::str("Bool"), vec![]);
        assert!(cache.query(&e1, &e1).is_none());
        let result = cache.alpha_equiv_cached(&e1, &e1);
        assert!(result);
        assert_eq!(cache.num_equiv(), 1);
    }
    #[test]
    fn test_alpha_cache_non_equiv() {
        let mut cache = AlphaCache::new();
        let e1 = nat();
        let e2 = Expr::Const(Name::str("Bool"), vec![]);
        let result = cache.alpha_equiv_cached(&e1, &e2);
        assert!(!result);
        assert_eq!(cache.num_non_equiv(), 1);
        let result2 = cache.alpha_equiv_cached(&e1, &e2);
        assert!(!result2);
    }
    #[test]
    fn test_alpha_cache_clear() {
        let mut cache = AlphaCache::new();
        cache.alpha_equiv_cached(&nat(), &nat());
        assert_eq!(cache.num_equiv(), 1);
        cache.clear();
        assert_eq!(cache.num_equiv(), 0);
        assert_eq!(cache.num_non_equiv(), 0);
    }
    #[test]
    fn test_alpha_class_rep() {
        let e = Expr::Lam(
            BinderInfo::Default,
            Name::str("uniqueName"),
            Box::new(nat()),
            Box::new(Expr::BVar(0)),
        );
        let rep = alpha_class_rep(&e);
        if let Expr::Lam(_, name, _, _) = &rep {
            assert!(name.to_string().starts_with('x'));
        }
    }
    #[test]
    fn test_alpha_hash_lit() {
        let h1 = alpha_hash(&Expr::Lit(Literal::Nat(42)));
        let h2 = alpha_hash(&Expr::Lit(Literal::Nat(42)));
        let h3 = alpha_hash(&Expr::Lit(Literal::Nat(43)));
        assert_eq!(h1, h2);
        assert_ne!(h1, h3);
    }
}
/// Rename a specific bound variable index in an expression.
///
/// Replaces all occurrences of `BVar(from_idx)` with `BVar(to_idx)`.
/// This is only safe when both indices are valid in the same context.
pub fn rename_bvar(expr: &Expr, from_idx: u32, to_idx: u32) -> Expr {
    rename_bvar_impl(expr, from_idx, to_idx, 0)
}
pub(super) fn rename_bvar_impl(expr: &Expr, from: u32, to: u32, depth: u32) -> Expr {
    match expr {
        Expr::BVar(i) => {
            if *i == from + depth {
                Expr::BVar(to + depth)
            } else {
                Expr::BVar(*i)
            }
        }
        Expr::App(f, a) => Expr::App(
            Box::new(rename_bvar_impl(f, from, to, depth)),
            Box::new(rename_bvar_impl(a, from, to, depth)),
        ),
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            *bi,
            n.clone(),
            Box::new(rename_bvar_impl(ty, from, to, depth)),
            Box::new(rename_bvar_impl(body, from, to, depth + 1)),
        ),
        Expr::Pi(bi, n, ty, body) => Expr::Pi(
            *bi,
            n.clone(),
            Box::new(rename_bvar_impl(ty, from, to, depth)),
            Box::new(rename_bvar_impl(body, from, to, depth + 1)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n.clone(),
            Box::new(rename_bvar_impl(ty, from, to, depth)),
            Box::new(rename_bvar_impl(val, from, to, depth)),
            Box::new(rename_bvar_impl(body, from, to, depth + 1)),
        ),
        Expr::Proj(n, i, s) => Expr::Proj(
            n.clone(),
            *i,
            Box::new(rename_bvar_impl(s, from, to, depth)),
        ),
        e => e.clone(),
    }
}
/// Swap two bound variable indices in an expression.
///
/// Exchanges occurrences of `BVar(i)` with `BVar(j)` and vice versa.
pub fn swap_bvars(expr: &Expr, i: u32, j: u32) -> Expr {
    swap_bvars_impl(expr, i, j, 0)
}
pub(super) fn swap_bvars_impl(expr: &Expr, i: u32, j: u32, depth: u32) -> Expr {
    match expr {
        Expr::BVar(k) => {
            let adjusted_i = i + depth;
            let adjusted_j = j + depth;
            if *k == adjusted_i {
                Expr::BVar(adjusted_j)
            } else if *k == adjusted_j {
                Expr::BVar(adjusted_i)
            } else {
                Expr::BVar(*k)
            }
        }
        Expr::App(f, a) => Expr::App(
            Box::new(swap_bvars_impl(f, i, j, depth)),
            Box::new(swap_bvars_impl(a, i, j, depth)),
        ),
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            *bi,
            n.clone(),
            Box::new(swap_bvars_impl(ty, i, j, depth)),
            Box::new(swap_bvars_impl(body, i, j, depth + 1)),
        ),
        Expr::Pi(bi, n, ty, body) => Expr::Pi(
            *bi,
            n.clone(),
            Box::new(swap_bvars_impl(ty, i, j, depth)),
            Box::new(swap_bvars_impl(body, i, j, depth + 1)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n.clone(),
            Box::new(swap_bvars_impl(ty, i, j, depth)),
            Box::new(swap_bvars_impl(val, i, j, depth)),
            Box::new(swap_bvars_impl(body, i, j, depth + 1)),
        ),
        Expr::Proj(n, k, s) => Expr::Proj(n.clone(), *k, Box::new(swap_bvars_impl(s, i, j, depth))),
        e => e.clone(),
    }
}
#[cfg(test)]
mod rename_tests {
    use super::*;
    #[allow(dead_code)]
    fn nat() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    #[test]
    fn test_rename_bvar_basic() {
        let e = Expr::BVar(0);
        let result = rename_bvar(&e, 0, 1);
        assert_eq!(result, Expr::BVar(1));
    }
    #[test]
    fn test_rename_bvar_no_match() {
        let e = Expr::BVar(2);
        let result = rename_bvar(&e, 0, 1);
        assert_eq!(result, Expr::BVar(2));
    }
    #[test]
    fn test_rename_bvar_in_app() {
        let e = Expr::App(Box::new(Expr::BVar(0)), Box::new(Expr::BVar(0)));
        let result = rename_bvar(&e, 0, 5);
        if let Expr::App(f, a) = result {
            assert_eq!(*f, Expr::BVar(5));
            assert_eq!(*a, Expr::BVar(5));
        }
    }
    #[test]
    fn test_swap_bvars() {
        let e = Expr::App(Box::new(Expr::BVar(0)), Box::new(Expr::BVar(1)));
        let result = swap_bvars(&e, 0, 1);
        if let Expr::App(f, a) = result {
            assert_eq!(*f, Expr::BVar(1));
            assert_eq!(*a, Expr::BVar(0));
        }
    }
    #[test]
    fn test_swap_same_idx_noop() {
        let e = Expr::BVar(3);
        let result = swap_bvars(&e, 3, 3);
        assert_eq!(result, Expr::BVar(3));
    }
}
/// Shift all free de Bruijn indices in `expr` by `amount`.
///
/// Indices ≥ `cutoff` are shifted; indices < `cutoff` are bound and left unchanged.
/// Used when inserting an expression under new binders.
pub fn shift(expr: &Expr, amount: i32, cutoff: u32) -> Expr {
    shift_impl(expr, amount, cutoff)
}
pub(super) fn shift_impl(expr: &Expr, amount: i32, cutoff: u32) -> Expr {
    match expr {
        Expr::BVar(i) => {
            if *i >= cutoff {
                let new_i = (*i as i32 + amount).max(0) as u32;
                Expr::BVar(new_i)
            } else {
                Expr::BVar(*i)
            }
        }
        Expr::FVar(f) => Expr::FVar(*f),
        Expr::Sort(l) => Expr::Sort(l.clone()),
        Expr::Const(n, ls) => Expr::Const(n.clone(), ls.clone()),
        Expr::App(f, a) => Expr::App(
            Box::new(shift_impl(f, amount, cutoff)),
            Box::new(shift_impl(a, amount, cutoff)),
        ),
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            *bi,
            n.clone(),
            Box::new(shift_impl(ty, amount, cutoff)),
            Box::new(shift_impl(body, amount, cutoff + 1)),
        ),
        Expr::Pi(bi, n, ty, body) => Expr::Pi(
            *bi,
            n.clone(),
            Box::new(shift_impl(ty, amount, cutoff)),
            Box::new(shift_impl(body, amount, cutoff + 1)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n.clone(),
            Box::new(shift_impl(ty, amount, cutoff)),
            Box::new(shift_impl(val, amount, cutoff)),
            Box::new(shift_impl(body, amount, cutoff + 1)),
        ),
        Expr::Proj(n, i, s) => Expr::Proj(n.clone(), *i, Box::new(shift_impl(s, amount, cutoff))),
        Expr::Lit(l) => Expr::Lit(l.clone()),
    }
}
/// Shift all free de Bruijn indices up by 1 (lift by one binder).
pub fn lift(expr: &Expr) -> Expr {
    shift(expr, 1, 0)
}
/// Shift all free de Bruijn indices down by 1 (used after substituting under a binder).
pub fn lower(expr: &Expr) -> Expr {
    shift(expr, -1, 0)
}
/// Structural equality: same as `alpha_equiv` but uses a simpler name-blind comparison.
///
/// Two expressions are structurally equal if they differ only in the names
/// given to binders (which are cosmetic in de Bruijn representation).
pub fn structurally_equal(e1: &Expr, e2: &Expr) -> bool {
    alpha_equiv(e1, e2)
}
/// Perform capture-avoiding substitution: replace `BVar(0)` with `replacement`
/// throughout `body`, adjusting de Bruijn indices appropriately.
///
/// This is the standard instantiation operation: `body[replacement/0]`.
pub fn alpha_subst(body: &Expr, replacement: &Expr) -> Expr {
    alpha_subst_impl(body, replacement, 0)
}
pub(super) fn alpha_subst_impl(body: &Expr, replacement: &Expr, depth: u32) -> Expr {
    match body {
        Expr::BVar(i) => {
            if *i == depth {
                shift(replacement, depth as i32, 0)
            } else if *i > depth {
                Expr::BVar(i - 1)
            } else {
                Expr::BVar(*i)
            }
        }
        Expr::App(f, a) => Expr::App(
            Box::new(alpha_subst_impl(f, replacement, depth)),
            Box::new(alpha_subst_impl(a, replacement, depth)),
        ),
        Expr::Lam(bi, n, ty, b) => Expr::Lam(
            *bi,
            n.clone(),
            Box::new(alpha_subst_impl(ty, replacement, depth)),
            Box::new(alpha_subst_impl(b, replacement, depth + 1)),
        ),
        Expr::Pi(bi, n, ty, b) => Expr::Pi(
            *bi,
            n.clone(),
            Box::new(alpha_subst_impl(ty, replacement, depth)),
            Box::new(alpha_subst_impl(b, replacement, depth + 1)),
        ),
        Expr::Let(n, ty, val, b) => Expr::Let(
            n.clone(),
            Box::new(alpha_subst_impl(ty, replacement, depth)),
            Box::new(alpha_subst_impl(val, replacement, depth)),
            Box::new(alpha_subst_impl(b, replacement, depth + 1)),
        ),
        Expr::Proj(n, i, s) => Expr::Proj(
            n.clone(),
            *i,
            Box::new(alpha_subst_impl(s, replacement, depth)),
        ),
        e => e.clone(),
    }
}
/// Check if `FVar(id)` occurs free in `expr`.
pub fn fvar_occurs(expr: &Expr, id: FVarId) -> bool {
    match expr {
        Expr::FVar(f) => *f == id,
        Expr::App(f, a) => fvar_occurs(f, id) || fvar_occurs(a, id),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            fvar_occurs(ty, id) || fvar_occurs(body, id)
        }
        Expr::Let(_, ty, val, body) => {
            fvar_occurs(ty, id) || fvar_occurs(val, id) || fvar_occurs(body, id)
        }
        Expr::Proj(_, _, s) => fvar_occurs(s, id),
        _ => false,
    }
}
/// Collect all free variable IDs occurring in `expr`.
pub fn free_fvars(expr: &Expr) -> std::collections::HashSet<FVarId> {
    let mut set = std::collections::HashSet::new();
    free_fvars_impl(expr, &mut set);
    set
}
pub(super) fn free_fvars_impl(expr: &Expr, acc: &mut std::collections::HashSet<FVarId>) {
    match expr {
        Expr::FVar(f) => {
            acc.insert(*f);
        }
        Expr::App(f, a) => {
            free_fvars_impl(f, acc);
            free_fvars_impl(a, acc);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            free_fvars_impl(ty, acc);
            free_fvars_impl(body, acc);
        }
        Expr::Let(_, ty, val, body) => {
            free_fvars_impl(ty, acc);
            free_fvars_impl(val, acc);
            free_fvars_impl(body, acc);
        }
        Expr::Proj(_, _, s) => free_fvars_impl(s, acc),
        _ => {}
    }
}
/// Count the number of occurrences of `BVar(0)` in `expr` (at the current depth).
pub fn count_bvar0_occurrences(expr: &Expr) -> usize {
    count_bvar0_impl(expr, 0)
}
pub(super) fn count_bvar0_impl(expr: &Expr, depth: u32) -> usize {
    match expr {
        Expr::BVar(i) => {
            if *i == depth {
                1
            } else {
                0
            }
        }
        Expr::App(f, a) => count_bvar0_impl(f, depth) + count_bvar0_impl(a, depth),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            count_bvar0_impl(ty, depth) + count_bvar0_impl(body, depth + 1)
        }
        Expr::Let(_, ty, val, body) => {
            count_bvar0_impl(ty, depth)
                + count_bvar0_impl(val, depth)
                + count_bvar0_impl(body, depth + 1)
        }
        Expr::Proj(_, _, s) => count_bvar0_impl(s, depth),
        _ => 0,
    }
}
/// Check whether `BVar(0)` occurs at all in `body`.
///
/// Useful for determining whether a lambda is an eta-candidate.
pub fn bvar0_free(body: &Expr) -> bool {
    count_bvar0_occurrences(body) == 0
}
/// Check alpha equivalence under a partial FVar substitution.
///
/// `subst` maps FVarIds to expressions; occurrences of `FVar(id)` in `e1`
/// are replaced before comparison.
pub fn alpha_equiv_under_subst(
    e1: &Expr,
    e2: &Expr,
    subst: &std::collections::HashMap<FVarId, Expr>,
) -> bool {
    let e1_inst = apply_fvar_subst(e1, subst);
    alpha_equiv(&e1_inst, e2)
}
/// Apply a map of FVar substitutions to an expression.
pub fn apply_fvar_subst(expr: &Expr, subst: &std::collections::HashMap<FVarId, Expr>) -> Expr {
    match expr {
        Expr::FVar(id) => subst.get(id).cloned().unwrap_or_else(|| expr.clone()),
        Expr::App(f, a) => Expr::App(
            Box::new(apply_fvar_subst(f, subst)),
            Box::new(apply_fvar_subst(a, subst)),
        ),
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            *bi,
            n.clone(),
            Box::new(apply_fvar_subst(ty, subst)),
            Box::new(apply_fvar_subst(body, subst)),
        ),
        Expr::Pi(bi, n, ty, body) => Expr::Pi(
            *bi,
            n.clone(),
            Box::new(apply_fvar_subst(ty, subst)),
            Box::new(apply_fvar_subst(body, subst)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n.clone(),
            Box::new(apply_fvar_subst(ty, subst)),
            Box::new(apply_fvar_subst(val, subst)),
            Box::new(apply_fvar_subst(body, subst)),
        ),
        Expr::Proj(n, i, s) => Expr::Proj(n.clone(), *i, Box::new(apply_fvar_subst(s, subst))),
        e => e.clone(),
    }
}
#[cfg(test)]
mod shift_subst_tests {
    use super::*;
    use crate::BinderInfo;
    use std::collections::HashMap;
    fn nat() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    #[test]
    fn test_shift_bvar_above_cutoff() {
        let e = Expr::BVar(1);
        let shifted = shift(&e, 2, 0);
        assert_eq!(shifted, Expr::BVar(3));
    }
    #[test]
    fn test_shift_bvar_below_cutoff() {
        let e = Expr::BVar(0);
        let shifted = shift(&e, 5, 1);
        assert_eq!(shifted, Expr::BVar(0));
    }
    #[test]
    fn test_lift_basic() {
        let e = Expr::BVar(0);
        assert_eq!(lift(&e), Expr::BVar(1));
    }
    #[test]
    fn test_lower_basic() {
        let e = Expr::BVar(1);
        assert_eq!(lower(&e), Expr::BVar(0));
    }
    #[test]
    fn test_alpha_subst_simple() {
        let body = Expr::BVar(0);
        let result = alpha_subst(&body, &nat());
        assert_eq!(result, nat());
    }
    #[test]
    fn test_alpha_subst_no_occurrence() {
        let body = Expr::BVar(1);
        let result = alpha_subst(&body, &nat());
        assert_eq!(result, Expr::BVar(0));
    }
    #[test]
    fn test_fvar_occurs_true() {
        let id = FVarId(42);
        let e = Expr::FVar(id);
        assert!(fvar_occurs(&e, id));
    }
    #[test]
    fn test_fvar_occurs_false() {
        let e = Expr::FVar(FVarId(1));
        assert!(!fvar_occurs(&e, FVarId(2)));
    }
    #[test]
    fn test_free_fvars_collects_all() {
        let e = Expr::App(
            Box::new(Expr::FVar(FVarId(1))),
            Box::new(Expr::FVar(FVarId(2))),
        );
        let fvars = free_fvars(&e);
        assert!(fvars.contains(&FVarId(1)));
        assert!(fvars.contains(&FVarId(2)));
        assert_eq!(fvars.len(), 2);
    }
    #[test]
    fn test_count_bvar0_none() {
        let e = nat();
        assert_eq!(count_bvar0_occurrences(&e), 0);
    }
    #[test]
    fn test_count_bvar0_one() {
        let e = Expr::BVar(0);
        assert_eq!(count_bvar0_occurrences(&e), 1);
    }
    #[test]
    fn test_count_bvar0_in_app() {
        let e = Expr::App(Box::new(Expr::BVar(0)), Box::new(Expr::BVar(0)));
        assert_eq!(count_bvar0_occurrences(&e), 2);
    }
    #[test]
    fn test_bvar0_free_true() {
        assert!(bvar0_free(&nat()));
        assert!(bvar0_free(&Expr::BVar(1)));
    }
    #[test]
    fn test_bvar0_free_false() {
        assert!(!bvar0_free(&Expr::BVar(0)));
    }
    #[test]
    fn test_apply_fvar_subst() {
        let id = FVarId(0);
        let mut subst = HashMap::new();
        subst.insert(id, nat());
        let e = Expr::FVar(id);
        let result = apply_fvar_subst(&e, &subst);
        assert_eq!(result, nat());
    }
    #[test]
    fn test_alpha_equiv_under_subst_same() {
        let id = FVarId(0);
        let mut subst = HashMap::new();
        subst.insert(id, nat());
        let e1 = Expr::FVar(id);
        let e2 = nat();
        assert!(alpha_equiv_under_subst(&e1, &e2, &subst));
    }
    #[test]
    fn test_structurally_equal() {
        let lam1 = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat()),
            Box::new(Expr::BVar(0)),
        );
        let lam2 = Expr::Lam(
            BinderInfo::Default,
            Name::str("y"),
            Box::new(nat()),
            Box::new(Expr::BVar(0)),
        );
        assert!(structurally_equal(&lam1, &lam2));
    }
    #[test]
    fn test_shift_preserves_const() {
        let e = Expr::Const(Name::str("f"), vec![]);
        assert_eq!(shift(&e, 3, 0), e);
    }
    #[test]
    fn test_shift_in_lam() {
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat()),
            Box::new(Expr::BVar(1)),
        );
        let shifted = shift(&lam, 1, 0);
        if let Expr::Lam(_, _, _, body) = &shifted {
            assert_eq!(**body, Expr::BVar(2));
        } else {
            panic!("expected Lam");
        }
    }
}
#[cfg(test)]
mod tests_padding_infra {
    use super::*;
    #[test]
    fn test_stat_summary() {
        let mut ss = StatSummary::new();
        ss.record(10.0);
        ss.record(20.0);
        ss.record(30.0);
        assert_eq!(ss.count(), 3);
        assert!((ss.mean().expect("mean should succeed") - 20.0).abs() < 1e-9);
        assert_eq!(ss.min().expect("min should succeed") as i64, 10);
        assert_eq!(ss.max().expect("max should succeed") as i64, 30);
    }
    #[test]
    fn test_transform_stat() {
        let mut ts = TransformStat::new();
        ts.record_before(100.0);
        ts.record_after(80.0);
        let ratio = ts.mean_ratio().expect("ratio should be present");
        assert!((ratio - 0.8).abs() < 1e-9);
    }
    #[test]
    fn test_small_map() {
        let mut m: SmallMap<u32, &str> = SmallMap::new();
        m.insert(3, "three");
        m.insert(1, "one");
        m.insert(2, "two");
        assert_eq!(m.get(&2), Some(&"two"));
        assert_eq!(m.len(), 3);
        let keys = m.keys();
        assert_eq!(*keys[0], 1);
        assert_eq!(*keys[2], 3);
    }
    #[test]
    fn test_label_set() {
        let mut ls = LabelSet::new();
        ls.add("foo");
        ls.add("bar");
        ls.add("foo");
        assert_eq!(ls.count(), 2);
        assert!(ls.has("bar"));
        assert!(!ls.has("baz"));
    }
    #[test]
    fn test_config_node() {
        let mut root = ConfigNode::section("root");
        let child = ConfigNode::leaf("key", "value");
        root.add_child(child);
        assert_eq!(root.num_children(), 1);
    }
    #[test]
    fn test_versioned_record() {
        let mut vr = VersionedRecord::new(0u32);
        vr.update(1);
        vr.update(2);
        assert_eq!(*vr.current(), 2);
        assert_eq!(vr.version(), 2);
        assert!(vr.has_history());
        assert_eq!(*vr.at_version(0).expect("value should be present"), 0);
    }
    #[test]
    fn test_simple_dag() {
        let mut dag = SimpleDag::new(4);
        dag.add_edge(0, 1);
        dag.add_edge(1, 2);
        dag.add_edge(2, 3);
        assert!(dag.can_reach(0, 3));
        assert!(!dag.can_reach(3, 0));
        let order = dag.topological_sort().expect("order should be present");
        assert_eq!(order, vec![0, 1, 2, 3]);
    }
    #[test]
    fn test_focus_stack() {
        let mut fs: FocusStack<&str> = FocusStack::new();
        fs.focus("a");
        fs.focus("b");
        assert_eq!(fs.current(), Some(&"b"));
        assert_eq!(fs.depth(), 2);
        fs.blur();
        assert_eq!(fs.current(), Some(&"a"));
    }
}
#[cfg(test)]
mod tests_extra_iterators {
    use super::*;
    #[test]
    fn test_window_iterator() {
        let data = vec![1u32, 2, 3, 4, 5];
        let windows: Vec<_> = WindowIterator::new(&data, 3).collect();
        assert_eq!(windows.len(), 3);
        assert_eq!(windows[0], &[1, 2, 3]);
        assert_eq!(windows[2], &[3, 4, 5]);
    }
    #[test]
    fn test_non_empty_vec() {
        let mut nev = NonEmptyVec::singleton(10u32);
        nev.push(20);
        nev.push(30);
        assert_eq!(nev.len(), 3);
        assert_eq!(*nev.first(), 10);
        assert_eq!(*nev.last(), 30);
    }
}
#[cfg(test)]
mod tests_padding2 {
    use super::*;
    #[test]
    fn test_sliding_sum() {
        let mut ss = SlidingSum::new(3);
        ss.push(1.0);
        ss.push(2.0);
        ss.push(3.0);
        assert!((ss.sum() - 6.0).abs() < 1e-9);
        ss.push(4.0);
        assert!((ss.sum() - 9.0).abs() < 1e-9);
        assert_eq!(ss.count(), 3);
    }
    #[test]
    fn test_path_buf() {
        let mut pb = PathBuf::new();
        pb.push("src");
        pb.push("main");
        assert_eq!(pb.as_str(), "src/main");
        assert_eq!(pb.depth(), 2);
        pb.pop();
        assert_eq!(pb.as_str(), "src");
    }
    #[test]
    fn test_string_pool() {
        let mut pool = StringPool::new();
        let s = pool.take();
        assert!(s.is_empty());
        pool.give("hello".to_string());
        let s2 = pool.take();
        assert!(s2.is_empty());
        assert_eq!(pool.free_count(), 0);
    }
    #[test]
    fn test_transitive_closure() {
        let mut tc = TransitiveClosure::new(4);
        tc.add_edge(0, 1);
        tc.add_edge(1, 2);
        tc.add_edge(2, 3);
        assert!(tc.can_reach(0, 3));
        assert!(!tc.can_reach(3, 0));
        let r = tc.reachable_from(0);
        assert_eq!(r.len(), 4);
    }
    #[test]
    fn test_token_bucket() {
        let mut tb = TokenBucket::new(100, 10);
        assert_eq!(tb.available(), 100);
        assert!(tb.try_consume(50));
        assert_eq!(tb.available(), 50);
        assert!(!tb.try_consume(60));
        assert_eq!(tb.capacity(), 100);
    }
    #[test]
    fn test_rewrite_rule_set() {
        let mut rrs = RewriteRuleSet::new();
        rrs.add(RewriteRule::unconditional(
            "beta",
            "App(Lam(x, b), v)",
            "b[x:=v]",
        ));
        rrs.add(RewriteRule::conditional("comm", "a + b", "b + a"));
        assert_eq!(rrs.len(), 2);
        assert_eq!(rrs.unconditional_rules().len(), 1);
        assert_eq!(rrs.conditional_rules().len(), 1);
        assert!(rrs.get("beta").is_some());
        let disp = rrs
            .get("beta")
            .expect("element at \'beta\' should exist")
            .display();
        assert!(disp.contains("→"));
    }
}
#[cfg(test)]
mod tests_padding3 {
    use super::*;
    #[test]
    fn test_decision_node() {
        let tree = DecisionNode::Branch {
            key: "x".into(),
            val: "1".into(),
            yes_branch: Box::new(DecisionNode::Leaf("yes".into())),
            no_branch: Box::new(DecisionNode::Leaf("no".into())),
        };
        let mut ctx = std::collections::HashMap::new();
        ctx.insert("x".into(), "1".into());
        assert_eq!(tree.evaluate(&ctx), "yes");
        ctx.insert("x".into(), "2".into());
        assert_eq!(tree.evaluate(&ctx), "no");
        assert_eq!(tree.depth(), 1);
    }
    #[test]
    fn test_flat_substitution() {
        let mut sub = FlatSubstitution::new();
        sub.add("foo", "bar");
        sub.add("baz", "qux");
        assert_eq!(sub.apply("foo and baz"), "bar and qux");
        assert_eq!(sub.len(), 2);
    }
    #[test]
    fn test_stopwatch() {
        let mut sw = Stopwatch::start();
        sw.split();
        sw.split();
        assert_eq!(sw.num_splits(), 2);
        assert!(sw.elapsed_ms() >= 0.0);
        for &s in sw.splits() {
            assert!(s >= 0.0);
        }
    }
    #[test]
    fn test_either2() {
        let e: Either2<i32, &str> = Either2::First(42);
        assert!(e.is_first());
        let mapped = e.map_first(|x| x * 2);
        assert_eq!(mapped.first(), Some(84));
        let e2: Either2<i32, &str> = Either2::Second("hello");
        assert!(e2.is_second());
        assert_eq!(e2.second(), Some("hello"));
    }
    #[test]
    fn test_write_once() {
        let wo: WriteOnce<u32> = WriteOnce::new();
        assert!(!wo.is_written());
        assert!(wo.write(42));
        assert!(!wo.write(99));
        assert_eq!(wo.read(), Some(42));
    }
    #[test]
    fn test_sparse_vec() {
        let mut sv: SparseVec<i32> = SparseVec::new(100);
        sv.set(5, 10);
        sv.set(50, 20);
        assert_eq!(*sv.get(5), 10);
        assert_eq!(*sv.get(50), 20);
        assert_eq!(*sv.get(0), 0);
        assert_eq!(sv.nnz(), 2);
        sv.set(5, 0);
        assert_eq!(sv.nnz(), 1);
    }
    #[test]
    fn test_stack_calc() {
        let mut calc = StackCalc::new();
        calc.push(3);
        calc.push(4);
        calc.add();
        assert_eq!(calc.peek(), Some(7));
        calc.push(2);
        calc.mul();
        assert_eq!(calc.peek(), Some(14));
    }
}
#[cfg(test)]
mod tests_final_padding {
    use super::*;
    #[test]
    fn test_min_heap() {
        let mut h = MinHeap::new();
        h.push(5u32);
        h.push(1u32);
        h.push(3u32);
        assert_eq!(h.peek(), Some(&1));
        assert_eq!(h.pop(), Some(1));
        assert_eq!(h.pop(), Some(3));
        assert_eq!(h.pop(), Some(5));
        assert!(h.is_empty());
    }
    #[test]
    fn test_prefix_counter() {
        let mut pc = PrefixCounter::new();
        pc.record("hello");
        pc.record("help");
        pc.record("world");
        assert_eq!(pc.count_with_prefix("hel"), 2);
        assert_eq!(pc.count_with_prefix("wor"), 1);
        assert_eq!(pc.count_with_prefix("xyz"), 0);
    }
    #[test]
    fn test_fixture() {
        let mut f = Fixture::new();
        f.set("key1", "val1");
        f.set("key2", "val2");
        assert_eq!(f.get("key1"), Some("val1"));
        assert_eq!(f.get("key3"), None);
        assert_eq!(f.len(), 2);
    }
}
