//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{
    BinderInfo, Declaration, Environment, Expr, InductiveEnv, InductiveType, IntroRule, Level, Name,
};

use super::types::{Fin, FinFun, FinIter};

/// Build Fin type in the environment.
///
/// Fin : Nat → Type
pub fn build_fin_env(env: &mut Environment, ind_env: &mut InductiveEnv) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let fin_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("n"),
        Box::new(Expr::Const(Name::str("Nat"), vec![])),
        Box::new(type1.clone()),
    );
    let zero_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("n"),
        Box::new(Expr::Const(Name::str("Nat"), vec![])),
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Fin"), vec![])),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Nat.succ"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
        )),
    );
    let succ_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("n"),
        Box::new(Expr::Const(Name::str("Nat"), vec![])),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("i"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Fin"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Fin"), vec![])),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Nat.succ"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
            )),
        )),
    );
    let fin_ind = InductiveType::new(
        Name::str("Fin"),
        vec![],
        0,
        1,
        fin_ty.clone(),
        vec![
            IntroRule {
                name: Name::str("Fin.zero"),
                ty: zero_ty.clone(),
            },
            IntroRule {
                name: Name::str("Fin.succ"),
                ty: succ_ty.clone(),
            },
        ],
    );
    ind_env.add(fin_ind).map_err(|e| format!("{}", e))?;
    env.add(Declaration::Axiom {
        name: Name::str("Fin"),
        univ_params: vec![],
        ty: fin_ty,
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Fin.zero"),
        univ_params: vec![],
        ty: zero_ty,
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Fin.succ"),
        univ_params: vec![],
        ty: succ_ty,
    })
    .map_err(|e| e.to_string())?;
    build_fin_operations(env)?;
    build_fin_arithmetic(env)?;
    Ok(())
}
/// Build arithmetic operations on `Fin n` (add, mul, mod, etc.)
pub fn build_fin_operations(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let val_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("n"),
        Box::new(Expr::Const(Name::str("Nat"), vec![])),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("i"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Fin"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Fin.val"),
        univ_params: vec![],
        ty: val_ty,
    })
    .map_err(|e| e.to_string())?;
    let last_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("n"),
        Box::new(Expr::Const(Name::str("Nat"), vec![])),
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Fin"), vec![])),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Nat.succ"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Fin.last"),
        univ_params: vec![],
        ty: last_ty,
    })
    .map_err(|e| e.to_string())?;
    let cast_succ_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("n"),
        Box::new(Expr::Const(Name::str("Nat"), vec![])),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("i"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Fin"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Fin"), vec![])),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Nat.succ"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Fin.castSucc"),
        univ_params: vec![],
        ty: cast_succ_ty,
    })
    .map_err(|e| e.to_string())?;
    let fin_n_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("n"),
        Box::new(Expr::Const(Name::str("Nat"), vec![])),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("i"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Fin"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Fin"), vec![])),
                Box::new(Expr::BVar(1)),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Fin.rev"),
        univ_params: vec![],
        ty: fin_n_ty,
    })
    .map_err(|e| e.to_string())?;
    let of_nat_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("n"),
        Box::new(Expr::Const(Name::str("Nat"), vec![])),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("k"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Fin"), vec![])),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Nat.succ"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Fin.ofNat"),
        univ_params: vec![],
        ty: of_nat_ty,
    })
    .map_err(|e| e.to_string())?;
    let prop = Expr::Sort(Level::zero());
    let is_lt_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("n"),
        Box::new(Expr::Const(Name::str("Nat"), vec![])),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("i"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Fin"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(prop),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Fin.isLt"),
        univ_params: vec![],
        ty: is_lt_ty,
    })
    .map_err(|e| e.to_string())?;
    let elim0_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("i"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Fin"), vec![])),
                Box::new(Expr::Lit(oxilean_kernel::Literal::Nat(0u64))),
            )),
            Box::new(Expr::BVar(1)),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Fin.elim0"),
        univ_params: vec![],
        ty: elim0_ty,
    })
    .map_err(|e| e.to_string())?;
    Ok(())
}
/// Build arithmetic operations: Fin.add, Fin.mul, Fin.sub, Fin.mod
pub fn build_fin_arithmetic(env: &mut Environment) -> Result<(), String> {
    let fin_bin_op = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("n"),
        Box::new(Expr::Const(Name::str("Nat"), vec![])),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Fin"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("b"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Fin"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Fin"), vec![])),
                    Box::new(Expr::BVar(2)),
                )),
            )),
        )),
    );
    for op_name in &["Fin.add", "Fin.mul", "Fin.sub", "Fin.mod"] {
        env.add(Declaration::Axiom {
            name: Name::str(*op_name),
            univ_params: vec![],
            ty: fin_bin_op.clone(),
        })
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
/// Ordinal arithmetic: compute the ordinal sum `m + n`.
pub fn ordinal_add(m: usize, n: usize) -> usize {
    m + n
}
/// Ordinal arithmetic: compute the ordinal product `m * n`.
pub fn ordinal_mul(m: usize, n: usize) -> usize {
    m * n
}
/// Inject `Fin m` into `Fin (m + n)` (left injection).
pub fn fin_inject_left(i: Fin, right: usize) -> Fin {
    Fin {
        val: i.val,
        bound: i.bound + right,
    }
}
/// Inject `Fin n` into `Fin (m + n)` (right injection).
pub fn fin_inject_right(i: Fin, left: usize) -> Fin {
    Fin {
        val: left + i.val,
        bound: left + i.bound,
    }
}
/// Split a `Fin (m + n)` into either `Fin m` (left) or `Fin n` (right).
pub fn fin_split(i: Fin, left_bound: usize) -> Result<Fin, Fin> {
    if i.val < left_bound {
        Ok(Fin {
            val: i.val,
            bound: left_bound,
        })
    } else {
        let right_bound = i.bound - left_bound;
        Err(Fin {
            val: i.val - left_bound,
            bound: right_bound,
        })
    }
}
/// Pair of `Fin m` and `Fin n` forms an element of `Fin (m * n)`.
pub fn fin_pair_to_index(i: Fin, j: Fin) -> Option<Fin> {
    if i.bound == 0 || j.bound == 0 {
        return None;
    }
    let bound = i.bound * j.bound;
    let val = i.val * j.bound + j.val;
    Some(Fin { val, bound })
}
/// Decompose a `Fin (m * n)` into `(Fin m, Fin n)`.
pub fn fin_index_to_pair(k: Fin, col_bound: usize) -> Option<(Fin, Fin)> {
    if col_bound == 0 || k.bound % col_bound != 0 {
        return None;
    }
    let row_bound = k.bound / col_bound;
    let row = k.val / col_bound;
    let col = k.val % col_bound;
    if row >= row_bound {
        return None;
    }
    Some((
        Fin {
            val: row,
            bound: row_bound,
        },
        Fin {
            val: col,
            bound: col_bound,
        },
    ))
}
/// Build an `Expr` representing `Fin n` applied to a nat literal.
pub fn make_fin_type(n: u64) -> Expr {
    Expr::App(
        Box::new(Expr::Const(Name::str("Fin"), vec![])),
        Box::new(Expr::Lit(oxilean_kernel::Literal::Nat(n))),
    )
}
/// Build an `Expr` for `Fin.zero {n}`.
pub fn make_fin_zero(n_expr: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::Const(Name::str("Fin.zero"), vec![])),
        Box::new(n_expr),
    )
}
/// Build an `Expr` for `Fin.succ {n} i`.
pub fn make_fin_succ(n_expr: Expr, i_expr: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Fin.succ"), vec![])),
            Box::new(n_expr),
        )),
        Box::new(i_expr),
    )
}
/// Return all registered `Fin`-related names present in the environment.
pub fn registered_fin_names(env: &Environment) -> Vec<String> {
    let candidates = [
        "Fin",
        "Fin.zero",
        "Fin.succ",
        "Fin.val",
        "Fin.last",
        "Fin.castSucc",
        "Fin.rev",
        "Fin.ofNat",
        "Fin.isLt",
        "Fin.elim0",
        "Fin.add",
        "Fin.mul",
        "Fin.sub",
        "Fin.mod",
    ];
    candidates
        .iter()
        .filter(|n| env.get(&Name::str(**n)).is_some())
        .map(|s| s.to_string())
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    fn setup_base_env() -> (Environment, InductiveEnv) {
        let mut env = Environment::new();
        let ind_env = InductiveEnv::new();
        let type1 = Expr::Sort(Level::succ(Level::zero()));
        env.add(Declaration::Axiom {
            name: Name::str("Nat"),
            univ_params: vec![],
            ty: type1.clone(),
        })
        .expect("operation should succeed");
        env.add(Declaration::Axiom {
            name: Name::str("Nat.succ"),
            univ_params: vec![],
            ty: Expr::Pi(
                BinderInfo::Default,
                Name::str("n"),
                Box::new(Expr::Const(Name::str("Nat"), vec![])),
                Box::new(Expr::Const(Name::str("Nat"), vec![])),
            ),
        })
        .expect("operation should succeed");
        (env, ind_env)
    }
    #[test]
    fn test_build_fin_env() {
        let (mut env, mut ind_env) = setup_base_env();
        assert!(build_fin_env(&mut env, &mut ind_env).is_ok());
        assert!(env.get(&Name::str("Fin")).is_some());
        assert!(env.get(&Name::str("Fin.zero")).is_some());
        assert!(env.get(&Name::str("Fin.succ")).is_some());
    }
    #[test]
    fn test_fin_operations_registered() {
        let (mut env, mut ind_env) = setup_base_env();
        build_fin_env(&mut env, &mut ind_env).expect("build_fin_env should succeed");
        assert!(env.get(&Name::str("Fin.val")).is_some());
        assert!(env.get(&Name::str("Fin.last")).is_some());
        assert!(env.get(&Name::str("Fin.castSucc")).is_some());
        assert!(env.get(&Name::str("Fin.rev")).is_some());
        assert!(env.get(&Name::str("Fin.ofNat")).is_some());
    }
    #[test]
    fn test_fin_arithmetic_registered() {
        let (mut env, mut ind_env) = setup_base_env();
        build_fin_env(&mut env, &mut ind_env).expect("build_fin_env should succeed");
        assert!(env.get(&Name::str("Fin.add")).is_some());
        assert!(env.get(&Name::str("Fin.mul")).is_some());
        assert!(env.get(&Name::str("Fin.sub")).is_some());
        assert!(env.get(&Name::str("Fin.mod")).is_some());
    }
    #[test]
    fn test_fin_new() {
        assert_eq!(Fin::new(3, 5), Some(Fin { val: 3, bound: 5 }));
        assert_eq!(Fin::new(5, 5), None);
        assert_eq!(Fin::new(0, 1), Some(Fin { val: 0, bound: 1 }));
    }
    #[test]
    fn test_fin_zero() {
        let z = Fin::zero(5).expect("Fin::zero(5) should be valid");
        assert_eq!(z.val, 0);
        assert!(z.is_zero());
        assert!(Fin::zero(0).is_none());
    }
    #[test]
    fn test_fin_last() {
        let l = Fin::last(5).expect("Fin::last(5) should be valid");
        assert_eq!(l.val, 4);
        assert!(l.is_last());
        assert!(Fin::last(0).is_none());
    }
    #[test]
    fn test_fin_succ_wrap() {
        let f = Fin::new(4, 5).expect("Fin::new(4, 5) should be valid");
        let s = f.succ_wrap();
        assert_eq!(s.val, 0);
        assert_eq!(s.bound, 5);
        let f2 = Fin::new(2, 5).expect("Fin::new(2, 5) should be valid");
        assert_eq!(f2.succ_wrap().val, 3);
    }
    #[test]
    fn test_fin_pred_wrap() {
        let f = Fin::new(0, 5).expect("Fin::new(0, 5) should be valid");
        let p = f.pred_wrap();
        assert_eq!(p.val, 4);
        assert_eq!(p.bound, 5);
        let f2 = Fin::new(3, 5).expect("Fin::new(3, 5) should be valid");
        assert_eq!(f2.pred_wrap().val, 2);
    }
    #[test]
    fn test_fin_complement() {
        let f = Fin::new(2, 5).expect("Fin::new(2, 5) should be valid");
        let c = f.complement();
        assert_eq!(c.val, 2);
        assert_eq!(c.bound, 5);
    }
    #[test]
    fn test_fin_add() {
        let a = Fin::new(3, 5).expect("Fin::new(3, 5) should be valid");
        let b = Fin::new(4, 5).expect("Fin::new(4, 5) should be valid");
        let sum = a.add(b).expect("add operation should succeed");
        assert_eq!(sum.val, 2);
        assert_eq!(sum.bound, 5);
    }
    #[test]
    fn test_fin_mul() {
        let a = Fin::new(3, 5).expect("Fin::new(3, 5) should be valid");
        let b = Fin::new(4, 5).expect("Fin::new(4, 5) should be valid");
        let prod = a.mul(b).expect("mul operation should succeed");
        assert_eq!(prod.val, 2);
    }
    #[test]
    fn test_fin_sub() {
        let a = Fin::new(2, 5).expect("Fin::new(2, 5) should be valid");
        let b = Fin::new(4, 5).expect("Fin::new(4, 5) should be valid");
        let diff = a.sub(b).expect("sub operation should succeed");
        assert_eq!(diff.val, 3);
    }
    #[test]
    fn test_fin_add_different_bounds() {
        let a = Fin::new(2, 5).expect("Fin::new(2, 5) should be valid");
        let b = Fin::new(2, 7).expect("Fin::new(2, 7) should be valid");
        assert!(a.add(b).is_none());
    }
    #[test]
    fn test_fin_cast() {
        let f = Fin::new(3, 5).expect("Fin::new(3, 5) should be valid");
        assert!(f.cast(10).is_some());
        assert_eq!(f.cast(10).expect("cast to 10 should succeed").bound, 10);
        assert!(f.cast(3).is_none());
        assert!(f.cast(4).is_some());
    }
    #[test]
    fn test_fin_all() {
        let all = Fin::all(5);
        assert_eq!(all.len(), 5);
        assert_eq!(all[0].val, 0);
        assert_eq!(all[4].val, 4);
    }
    #[test]
    fn test_fin_iter() {
        let iter = FinIter::new(4);
        let collected: Vec<Fin> = iter.collect();
        assert_eq!(collected.len(), 4);
        assert_eq!(collected[0].val, 0);
        assert_eq!(collected[3].val, 3);
    }
    #[test]
    fn test_fin_iter_exact_size() {
        let iter = FinIter::new(7);
        assert_eq!(iter.len(), 7);
    }
    #[test]
    fn test_fin_fun_constant() {
        let ff: FinFun<i32> = FinFun::constant(5, 42);
        for i in Fin::all(5) {
            assert_eq!(ff.apply(i), Some(&42));
        }
    }
    #[test]
    fn test_fin_fun_from_fn() {
        let ff = FinFun::from_fn(5, |f| f.val * f.val);
        let f3 = Fin::new(3, 5).expect("Fin::new(3, 5) should be valid");
        assert_eq!(ff.apply(f3), Some(&9));
    }
    #[test]
    fn test_fin_fun_iter() {
        let ff = FinFun::from_fn(3, |f| f.val + 10);
        let pairs: Vec<(Fin, &usize)> = ff.iter().collect();
        assert_eq!(pairs.len(), 3);
        assert_eq!(*pairs[0].1, 10);
        assert_eq!(*pairs[2].1, 12);
    }
    #[test]
    fn test_fin_inject_left() {
        let f = Fin::new(2, 5).expect("Fin::new(2, 5) should be valid");
        let inj = fin_inject_left(f, 3);
        assert_eq!(inj.val, 2);
        assert_eq!(inj.bound, 8);
    }
    #[test]
    fn test_fin_inject_right() {
        let f = Fin::new(1, 3).expect("Fin::new(1, 3) should be valid");
        let inj = fin_inject_right(f, 5);
        assert_eq!(inj.val, 6);
        assert_eq!(inj.bound, 8);
    }
    #[test]
    fn test_fin_split() {
        let f = Fin::new(3, 7).expect("Fin::new(3, 7) should be valid");
        assert!(fin_split(f, 4).is_ok());
        let f2 = Fin::new(5, 7).expect("Fin::new(5, 7) should be valid");
        let right = fin_split(f2, 4).unwrap_err();
        assert_eq!(right.val, 1);
        assert_eq!(right.bound, 3);
    }
    #[test]
    fn test_fin_pair_to_index() {
        let i = Fin::new(1, 3).expect("Fin::new(1, 3) should be valid");
        let j = Fin::new(2, 4).expect("Fin::new(2, 4) should be valid");
        let k = fin_pair_to_index(i, j).expect("fin_pair_to_index should succeed");
        assert_eq!(k.val, 6);
        assert_eq!(k.bound, 12);
    }
    #[test]
    fn test_fin_index_to_pair() {
        let k = Fin::new(6, 12).expect("Fin::new(6, 12) should be valid");
        let (i, j) = fin_index_to_pair(k, 4).expect("fin_index_to_pair should succeed");
        assert_eq!(i.val, 1);
        assert_eq!(j.val, 2);
    }
    #[test]
    fn test_make_fin_type() {
        let expr = make_fin_type(5);
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_ordinal_add() {
        assert_eq!(ordinal_add(3, 4), 7);
        assert_eq!(ordinal_add(0, 5), 5);
    }
    #[test]
    fn test_ordinal_mul() {
        assert_eq!(ordinal_mul(3, 4), 12);
        assert_eq!(ordinal_mul(0, 5), 0);
    }
    #[test]
    fn test_registered_fin_names() {
        let (mut env, mut ind_env) = setup_base_env();
        build_fin_env(&mut env, &mut ind_env).expect("build_fin_env should succeed");
        let names = registered_fin_names(&env);
        assert!(names.contains(&"Fin".to_string()));
        assert!(names.contains(&"Fin.zero".to_string()));
        assert!(names.len() >= 5);
    }
}
/// Check if two `Fin` values have the same bound.
pub fn same_bound(a: Fin, b: Fin) -> bool {
    a.bound == b.bound
}
/// Return the number of elements strictly between `a` and `b` (same bound).
pub fn fin_gap(a: Fin, b: Fin) -> Option<usize> {
    if a.bound != b.bound {
        return None;
    }
    if b.val > a.val {
        Some(b.val - a.val - 1)
    } else {
        None
    }
}
/// Return the maximum of two `Fin` values with the same bound.
pub fn fin_max(a: Fin, b: Fin) -> Option<Fin> {
    if a.bound != b.bound {
        return None;
    }
    Some(if a.val >= b.val { a } else { b })
}
/// Return the minimum of two `Fin` values with the same bound.
pub fn fin_min(a: Fin, b: Fin) -> Option<Fin> {
    if a.bound != b.bound {
        return None;
    }
    Some(if a.val <= b.val { a } else { b })
}
/// Count how many elements of `Fin n` satisfy predicate `p`.
pub fn fin_count<P: Fn(Fin) -> bool>(bound: usize, p: P) -> usize {
    (0..bound).filter(|&i| p(Fin { val: i, bound })).count()
}
/// Compute the sum of `f` over all elements of `Fin n`.
pub fn fin_sum<F: Fn(Fin) -> u64>(bound: usize, f: F) -> u64 {
    (0..bound).map(|i| f(Fin { val: i, bound })).sum()
}
/// Build an `Expr` for `Fin.last {n}`.
pub fn make_fin_last(n_expr: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::Const(Name::str("Fin.last"), vec![])),
        Box::new(n_expr),
    )
}
/// Build an `Expr` for `Fin.castSucc {n} i`.
pub fn make_fin_cast_succ(n_expr: Expr, i_expr: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Fin.castSucc"), vec![])),
            Box::new(n_expr),
        )),
        Box::new(i_expr),
    )
}
/// Whether a `Fin n` value can be viewed as an element of `Fin m` (m >= n).
pub fn fin_embeds_into(i: Fin, m: usize) -> bool {
    m >= i.bound
}
#[cfg(test)]
mod fin_extra_tests {
    use super::*;
    #[test]
    fn test_same_bound() {
        let a = Fin::new(1, 5).expect("Fin::new(1, 5) should be valid");
        let b = Fin::new(3, 5).expect("Fin::new(3, 5) should be valid");
        assert!(same_bound(a, b));
        let c = Fin::new(1, 7).expect("Fin::new(1, 7) should be valid");
        assert!(!same_bound(a, c));
    }
    #[test]
    fn test_fin_gap() {
        let a = Fin::new(1, 5).expect("Fin::new(1, 5) should be valid");
        let b = Fin::new(4, 5).expect("Fin::new(4, 5) should be valid");
        assert_eq!(fin_gap(a, b), Some(2));
        let c = Fin::new(2, 5).expect("Fin::new(2, 5) should be valid");
        let d = Fin::new(3, 5).expect("Fin::new(3, 5) should be valid");
        assert_eq!(fin_gap(c, d), Some(0));
    }
    #[test]
    fn test_fin_max_min() {
        let a = Fin::new(2, 5).expect("Fin::new(2, 5) should be valid");
        let b = Fin::new(4, 5).expect("Fin::new(4, 5) should be valid");
        assert_eq!(fin_max(a, b).expect("fin_max should succeed").val, 4);
        assert_eq!(fin_min(a, b).expect("fin_min should succeed").val, 2);
    }
    #[test]
    fn test_fin_count() {
        let count = fin_count(10, |f| f.val % 2 == 0);
        assert_eq!(count, 5);
    }
    #[test]
    fn test_fin_sum() {
        let s = fin_sum(5, |f| f.val as u64);
        assert_eq!(s, 10);
    }
    #[test]
    fn test_make_fin_last() {
        let n_expr = Expr::Lit(oxilean_kernel::Literal::Nat(4));
        let e = make_fin_last(n_expr);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_fin_embeds_into() {
        let f = Fin::new(3, 5).expect("Fin::new(3, 5) should be valid");
        assert!(fin_embeds_into(f, 5));
        assert!(fin_embeds_into(f, 10));
        assert!(!fin_embeds_into(f, 4));
    }
}
pub fn fin_ext_cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub fn fin_ext_app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn fin_ext_app2(f: Expr, a: Expr, b: Expr) -> Expr {
    fin_ext_app(fin_ext_app(f, a), b)
}
pub fn fin_ext_app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    fin_ext_app(fin_ext_app2(f, a, b), c)
}
pub fn fin_ext_prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn fin_ext_type1() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn fin_ext_nat() -> Expr {
    fin_ext_cst("Nat")
}
pub fn fin_ext_fin_n() -> Expr {
    fin_ext_app(fin_ext_cst("Fin"), Expr::BVar(0))
}
pub fn fin_ext_pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(body))
}
pub fn fin_ext_forall_nat(body: Expr) -> Expr {
    fin_ext_pi(BinderInfo::Default, "n", fin_ext_nat(), body)
}
pub fn fin_ext_nat_eq(a: Expr, b: Expr) -> Expr {
    fin_ext_app3(
        Expr::Const(Name::str("Eq"), vec![Level::succ(Level::zero())]),
        fin_ext_nat(),
        a,
        b,
    )
}
/// Fin.card : ∀ n, Eq (Fintype.card (Fin n)) n
/// (the cardinality of Fin n is n)
#[allow(dead_code)]
pub fn axiom_fin_card_ty() -> Expr {
    fin_ext_forall_nat(fin_ext_nat_eq(
        fin_ext_app(fin_ext_cst("Fintype.card"), fin_ext_fin_n()),
        Expr::BVar(0),
    ))
}
/// Fin.val_fin_lt : ∀ {n} (i : Fin n), Nat.lt (Fin.val i) n
#[allow(dead_code)]
pub fn axiom_fin_val_fin_lt_ty() -> Expr {
    fin_ext_pi(
        BinderInfo::Implicit,
        "n",
        fin_ext_nat(),
        fin_ext_pi(
            BinderInfo::Default,
            "i",
            fin_ext_fin_n(),
            fin_ext_app2(
                fin_ext_cst("Nat.lt"),
                fin_ext_app(fin_ext_cst("Fin.val"), Expr::BVar(0)),
                Expr::BVar(1),
            ),
        ),
    )
}
/// Fin.zero_val : ∀ {n}, Eq (Fin.val Fin.zero) 0
#[allow(dead_code)]
pub fn axiom_fin_zero_val_ty() -> Expr {
    fin_ext_pi(
        BinderInfo::Implicit,
        "n",
        fin_ext_nat(),
        fin_ext_nat_eq(
            fin_ext_app(
                fin_ext_cst("Fin.val"),
                fin_ext_app(fin_ext_cst("Fin.zero"), Expr::BVar(0)),
            ),
            Expr::Lit(oxilean_kernel::Literal::Nat(0)),
        ),
    )
}
/// Fin.succ_val : ∀ {n} (i : Fin n), Eq (Fin.val (Fin.succ i)) (Nat.succ (Fin.val i))
#[allow(dead_code)]
pub fn axiom_fin_succ_val_ty() -> Expr {
    fin_ext_pi(
        BinderInfo::Implicit,
        "n",
        fin_ext_nat(),
        fin_ext_pi(
            BinderInfo::Default,
            "i",
            fin_ext_fin_n(),
            fin_ext_nat_eq(
                fin_ext_app(
                    fin_ext_cst("Fin.val"),
                    fin_ext_app2(fin_ext_cst("Fin.succ"), Expr::BVar(1), Expr::BVar(0)),
                ),
                fin_ext_app(
                    fin_ext_cst("Nat.succ"),
                    fin_ext_app(fin_ext_cst("Fin.val"), Expr::BVar(0)),
                ),
            ),
        ),
    )
}
/// Fin.last_val : ∀ n, Eq (Fin.val (Fin.last n)) n
#[allow(dead_code)]
pub fn axiom_fin_last_val_ty() -> Expr {
    fin_ext_forall_nat(fin_ext_nat_eq(
        fin_ext_app(
            fin_ext_cst("Fin.val"),
            fin_ext_app(fin_ext_cst("Fin.last"), Expr::BVar(0)),
        ),
        Expr::BVar(0),
    ))
}
/// Fin.castSucc_val : ∀ {n} (i : Fin n), Eq (Fin.val (Fin.castSucc i)) (Fin.val i)
#[allow(dead_code)]
pub fn axiom_fin_cast_succ_val_ty() -> Expr {
    fin_ext_pi(
        BinderInfo::Implicit,
        "n",
        fin_ext_nat(),
        fin_ext_pi(
            BinderInfo::Default,
            "i",
            fin_ext_fin_n(),
            fin_ext_nat_eq(
                fin_ext_app(
                    fin_ext_cst("Fin.val"),
                    fin_ext_app2(fin_ext_cst("Fin.castSucc"), Expr::BVar(1), Expr::BVar(0)),
                ),
                fin_ext_app(fin_ext_cst("Fin.val"), Expr::BVar(0)),
            ),
        ),
    )
}
/// Fin.add_val : ∀ {n} (i j : Fin n), Eq (Fin.val (Fin.add i j)) (Nat.mod (Nat.add (Fin.val i) (Fin.val j)) n)
#[allow(dead_code)]
pub fn axiom_fin_add_val_ty() -> Expr {
    fin_ext_pi(
        BinderInfo::Implicit,
        "n",
        fin_ext_nat(),
        fin_ext_pi(
            BinderInfo::Default,
            "i",
            fin_ext_fin_n(),
            fin_ext_pi(
                BinderInfo::Default,
                "j",
                fin_ext_app(fin_ext_cst("Fin"), Expr::BVar(1)),
                fin_ext_nat_eq(
                    fin_ext_app(
                        fin_ext_cst("Fin.val"),
                        fin_ext_app3(
                            fin_ext_cst("Fin.add"),
                            Expr::BVar(2),
                            Expr::BVar(1),
                            Expr::BVar(0),
                        ),
                    ),
                    fin_ext_app2(
                        fin_ext_cst("Nat.mod"),
                        fin_ext_app2(
                            fin_ext_cst("Nat.add"),
                            fin_ext_app(fin_ext_cst("Fin.val"), Expr::BVar(1)),
                            fin_ext_app(fin_ext_cst("Fin.val"), Expr::BVar(0)),
                        ),
                        Expr::BVar(2),
                    ),
                ),
            ),
        ),
    )
}
/// Fin.rev_val : ∀ {n} (i : Fin n), Eq (Fin.val (Fin.rev i)) (Nat.sub (Nat.sub n 1) (Fin.val i))
#[allow(dead_code)]
pub fn axiom_fin_rev_val_ty() -> Expr {
    fin_ext_pi(
        BinderInfo::Implicit,
        "n",
        fin_ext_nat(),
        fin_ext_pi(
            BinderInfo::Default,
            "i",
            fin_ext_fin_n(),
            fin_ext_nat_eq(
                fin_ext_app(
                    fin_ext_cst("Fin.val"),
                    fin_ext_app2(fin_ext_cst("Fin.rev"), Expr::BVar(1), Expr::BVar(0)),
                ),
                fin_ext_app2(
                    fin_ext_cst("Nat.sub"),
                    fin_ext_app2(
                        fin_ext_cst("Nat.sub"),
                        Expr::BVar(1),
                        Expr::Lit(oxilean_kernel::Literal::Nat(1)),
                    ),
                    fin_ext_app(fin_ext_cst("Fin.val"), Expr::BVar(0)),
                ),
            ),
        ),
    )
}
/// Fin.eq_iff_val_eq : ∀ {n} (i j : Fin n), Iff (Eq i j) (Eq (Fin.val i) (Fin.val j))
#[allow(dead_code)]
pub fn axiom_fin_eq_iff_val_eq_ty() -> Expr {
    fin_ext_pi(
        BinderInfo::Implicit,
        "n",
        fin_ext_nat(),
        fin_ext_pi(
            BinderInfo::Default,
            "i",
            fin_ext_fin_n(),
            fin_ext_pi(
                BinderInfo::Default,
                "j",
                fin_ext_app(fin_ext_cst("Fin"), Expr::BVar(1)),
                fin_ext_app2(
                    fin_ext_cst("Iff"),
                    fin_ext_app3(
                        Expr::Const(Name::str("Eq"), vec![Level::succ(Level::zero())]),
                        fin_ext_app(fin_ext_cst("Fin"), Expr::BVar(2)),
                        Expr::BVar(1),
                        Expr::BVar(0),
                    ),
                    fin_ext_nat_eq(
                        fin_ext_app(fin_ext_cst("Fin.val"), Expr::BVar(1)),
                        fin_ext_app(fin_ext_cst("Fin.val"), Expr::BVar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// Fin.ofNat_val : ∀ n k, Eq (Fin.val (Fin.ofNat n k)) (Nat.mod k (Nat.succ n))
#[allow(dead_code)]
pub fn axiom_fin_of_nat_val_ty() -> Expr {
    fin_ext_pi(
        BinderInfo::Default,
        "n",
        fin_ext_nat(),
        fin_ext_pi(
            BinderInfo::Default,
            "k",
            fin_ext_nat(),
            fin_ext_nat_eq(
                fin_ext_app(
                    fin_ext_cst("Fin.val"),
                    fin_ext_app2(fin_ext_cst("Fin.ofNat"), Expr::BVar(1), Expr::BVar(0)),
                ),
                fin_ext_app2(
                    fin_ext_cst("Nat.mod"),
                    Expr::BVar(0),
                    fin_ext_app(fin_ext_cst("Nat.succ"), Expr::BVar(1)),
                ),
            ),
        ),
    )
}
/// Fin.Equiv.finZeroElim : Fin 0 → α   (vacuous equivalence)
#[allow(dead_code)]
pub fn axiom_fin_zero_elim_ty() -> Expr {
    fin_ext_pi(
        BinderInfo::Implicit,
        "α",
        fin_ext_type1(),
        fin_ext_pi(
            BinderInfo::Default,
            "i",
            fin_ext_app(
                fin_ext_cst("Fin"),
                Expr::Lit(oxilean_kernel::Literal::Nat(0)),
            ),
            Expr::BVar(1),
        ),
    )
}
/// Fin.Perm : Nat → Type  (type of permutations of Fin n)
#[allow(dead_code)]
pub fn axiom_fin_perm_ty() -> Expr {
    fin_ext_pi(BinderInfo::Default, "n", fin_ext_nat(), fin_ext_type1())
}
/// Fin.Perm.id : ∀ n, Fin.Perm n  (identity permutation)
#[allow(dead_code)]
pub fn axiom_fin_perm_id_ty() -> Expr {
    fin_ext_forall_nat(fin_ext_app(fin_ext_cst("Fin.Perm"), Expr::BVar(0)))
}
/// Fin.Perm.comp : ∀ n, Fin.Perm n → Fin.Perm n → Fin.Perm n
#[allow(dead_code)]
pub fn axiom_fin_perm_comp_ty() -> Expr {
    fin_ext_forall_nat(fin_ext_pi(
        BinderInfo::Default,
        "σ",
        fin_ext_app(fin_ext_cst("Fin.Perm"), Expr::BVar(0)),
        fin_ext_pi(
            BinderInfo::Default,
            "τ",
            fin_ext_app(fin_ext_cst("Fin.Perm"), Expr::BVar(1)),
            fin_ext_app(fin_ext_cst("Fin.Perm"), Expr::BVar(2)),
        ),
    ))
}
/// Fin.Perm.inv : ∀ n, Fin.Perm n → Fin.Perm n
#[allow(dead_code)]
pub fn axiom_fin_perm_inv_ty() -> Expr {
    fin_ext_forall_nat(fin_ext_pi(
        BinderInfo::Default,
        "σ",
        fin_ext_app(fin_ext_cst("Fin.Perm"), Expr::BVar(0)),
        fin_ext_app(fin_ext_cst("Fin.Perm"), Expr::BVar(1)),
    ))
}
/// Fin.Perm.count : ∀ n, Eq (Fintype.card (Fin.Perm n)) (Nat.factorial n)
/// (the number of permutations of Fin n equals n!)
#[allow(dead_code)]
pub fn axiom_fin_perm_count_ty() -> Expr {
    fin_ext_forall_nat(fin_ext_nat_eq(
        fin_ext_app(
            fin_ext_cst("Fintype.card"),
            fin_ext_app(fin_ext_cst("Fin.Perm"), Expr::BVar(0)),
        ),
        fin_ext_app(fin_ext_cst("Nat.factorial"), Expr::BVar(0)),
    ))
}
/// Nat.factorial : Nat → Nat
#[allow(dead_code)]
pub fn axiom_nat_factorial_ty() -> Expr {
    fin_ext_pi(BinderInfo::Default, "n", fin_ext_nat(), fin_ext_nat())
}
/// Nat.factorial_zero : Eq (Nat.factorial 0) 1
#[allow(dead_code)]
pub fn axiom_nat_factorial_zero_ty() -> Expr {
    fin_ext_nat_eq(
        fin_ext_app(
            fin_ext_cst("Nat.factorial"),
            Expr::Lit(oxilean_kernel::Literal::Nat(0)),
        ),
        Expr::Lit(oxilean_kernel::Literal::Nat(1)),
    )
}
/// Nat.factorial_succ : ∀ n, Eq (Nat.factorial (Nat.succ n)) (Nat.mul (Nat.succ n) (Nat.factorial n))
#[allow(dead_code)]
pub fn axiom_nat_factorial_succ_ty() -> Expr {
    fin_ext_forall_nat(fin_ext_nat_eq(
        fin_ext_app(
            fin_ext_cst("Nat.factorial"),
            fin_ext_app(fin_ext_cst("Nat.succ"), Expr::BVar(0)),
        ),
        fin_ext_app2(
            fin_ext_cst("Nat.mul"),
            fin_ext_app(fin_ext_cst("Nat.succ"), Expr::BVar(0)),
            fin_ext_app(fin_ext_cst("Nat.factorial"), Expr::BVar(0)),
        ),
    ))
}
/// Fin.derangement_count : ∀ n, Eq (Fin.derangementCount n) (...)
/// (number of derangements of Fin n; simplified as an axiom)
#[allow(dead_code)]
pub fn axiom_fin_derangement_count_ty() -> Expr {
    fin_ext_pi(BinderInfo::Default, "n", fin_ext_nat(), fin_ext_nat())
}
/// Fin.stirling_first : Nat → Nat → Nat
/// (unsigned Stirling numbers of the first kind)
#[allow(dead_code)]
pub fn axiom_fin_stirling_first_ty() -> Expr {
    fin_ext_pi(
        BinderInfo::Default,
        "n",
        fin_ext_nat(),
        fin_ext_pi(BinderInfo::Default, "k", fin_ext_nat(), fin_ext_nat()),
    )
}
/// Fin.stirling_second : Nat → Nat → Nat
/// (Stirling numbers of the second kind)
#[allow(dead_code)]
pub fn axiom_fin_stirling_second_ty() -> Expr {
    fin_ext_pi(
        BinderInfo::Default,
        "n",
        fin_ext_nat(),
        fin_ext_pi(BinderInfo::Default, "k", fin_ext_nat(), fin_ext_nat()),
    )
}
/// Fin.binomial : Nat → Nat → Nat  (binomial coefficient)
#[allow(dead_code)]
pub fn axiom_fin_binomial_ty() -> Expr {
    fin_ext_pi(
        BinderInfo::Default,
        "n",
        fin_ext_nat(),
        fin_ext_pi(BinderInfo::Default, "k", fin_ext_nat(), fin_ext_nat()),
    )
}
/// Fin.binomial_zero : ∀ n, Eq (Fin.binomial n 0) 1
#[allow(dead_code)]
pub fn axiom_fin_binomial_zero_ty() -> Expr {
    fin_ext_forall_nat(fin_ext_nat_eq(
        fin_ext_app2(
            fin_ext_cst("Fin.binomial"),
            Expr::BVar(0),
            Expr::Lit(oxilean_kernel::Literal::Nat(0)),
        ),
        Expr::Lit(oxilean_kernel::Literal::Nat(1)),
    ))
}
/// Fin.binomial_self : ∀ n, Eq (Fin.binomial n n) 1
#[allow(dead_code)]
pub fn axiom_fin_binomial_self_ty() -> Expr {
    fin_ext_forall_nat(fin_ext_nat_eq(
        fin_ext_app2(fin_ext_cst("Fin.binomial"), Expr::BVar(0), Expr::BVar(0)),
        Expr::Lit(oxilean_kernel::Literal::Nat(1)),
    ))
}
/// Fin.binomial_symm : ∀ n k, Eq (Fin.binomial n k) (Fin.binomial n (Nat.sub n k))
#[allow(dead_code)]
pub fn axiom_fin_binomial_symm_ty() -> Expr {
    fin_ext_pi(
        BinderInfo::Default,
        "n",
        fin_ext_nat(),
        fin_ext_pi(
            BinderInfo::Default,
            "k",
            fin_ext_nat(),
            fin_ext_nat_eq(
                fin_ext_app2(fin_ext_cst("Fin.binomial"), Expr::BVar(1), Expr::BVar(0)),
                fin_ext_app2(
                    fin_ext_cst("Fin.binomial"),
                    Expr::BVar(1),
                    fin_ext_app2(fin_ext_cst("Nat.sub"), Expr::BVar(1), Expr::BVar(0)),
                ),
            ),
        ),
    )
}
/// Fin.pascal : ∀ n k, Eq (Fin.binomial (Nat.succ n) (Nat.succ k))
///   (Nat.add (Fin.binomial n k) (Fin.binomial n (Nat.succ k)))
/// (Pascal's identity)
#[allow(dead_code)]
pub fn axiom_fin_pascal_ty() -> Expr {
    fin_ext_pi(
        BinderInfo::Default,
        "n",
        fin_ext_nat(),
        fin_ext_pi(
            BinderInfo::Default,
            "k",
            fin_ext_nat(),
            fin_ext_nat_eq(
                fin_ext_app2(
                    fin_ext_cst("Fin.binomial"),
                    fin_ext_app(fin_ext_cst("Nat.succ"), Expr::BVar(1)),
                    fin_ext_app(fin_ext_cst("Nat.succ"), Expr::BVar(0)),
                ),
                fin_ext_app2(
                    fin_ext_cst("Nat.add"),
                    fin_ext_app2(fin_ext_cst("Fin.binomial"), Expr::BVar(1), Expr::BVar(0)),
                    fin_ext_app2(
                        fin_ext_cst("Fin.binomial"),
                        Expr::BVar(1),
                        fin_ext_app(fin_ext_cst("Nat.succ"), Expr::BVar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// Fin.eulerPhi : Nat → Nat  (Euler's totient function)
#[allow(dead_code)]
pub fn axiom_fin_euler_phi_ty() -> Expr {
    fin_ext_pi(BinderInfo::Default, "n", fin_ext_nat(), fin_ext_nat())
}
/// Fin.eulerPhi_prime : ∀ p, Nat.Prime p → Eq (Fin.eulerPhi p) (Nat.sub p 1)
#[allow(dead_code)]
pub fn axiom_fin_euler_phi_prime_ty() -> Expr {
    fin_ext_forall_nat(fin_ext_pi(
        BinderInfo::Default,
        "hp",
        fin_ext_app(fin_ext_cst("Nat.Prime"), Expr::BVar(0)),
        fin_ext_nat_eq(
            fin_ext_app(fin_ext_cst("Fin.eulerPhi"), Expr::BVar(1)),
            fin_ext_app2(
                fin_ext_cst("Nat.sub"),
                Expr::BVar(1),
                Expr::Lit(oxilean_kernel::Literal::Nat(1)),
            ),
        ),
    ))
}
/// Fin.eulerPhi_mul : ∀ m n, Nat.Coprime m n
///   → Eq (Fin.eulerPhi (Nat.mul m n)) (Nat.mul (Fin.eulerPhi m) (Fin.eulerPhi n))
#[allow(dead_code)]
pub fn axiom_fin_euler_phi_mul_ty() -> Expr {
    fin_ext_pi(
        BinderInfo::Default,
        "m",
        fin_ext_nat(),
        fin_ext_pi(
            BinderInfo::Default,
            "n",
            fin_ext_nat(),
            fin_ext_pi(
                BinderInfo::Default,
                "hcop",
                fin_ext_app2(fin_ext_cst("Nat.Coprime"), Expr::BVar(1), Expr::BVar(0)),
                fin_ext_nat_eq(
                    fin_ext_app(
                        fin_ext_cst("Fin.eulerPhi"),
                        fin_ext_app2(fin_ext_cst("Nat.mul"), Expr::BVar(2), Expr::BVar(1)),
                    ),
                    fin_ext_app2(
                        fin_ext_cst("Nat.mul"),
                        fin_ext_app(fin_ext_cst("Fin.eulerPhi"), Expr::BVar(2)),
                        fin_ext_app(fin_ext_cst("Fin.eulerPhi"), Expr::BVar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// Fin.cayley_formula : ∀ n, Eq (Fin.labeledTreeCount n) (Nat.pow n (Nat.sub n 2))
/// (Cayley's formula: number of labeled trees on n vertices is n^(n-2))
#[allow(dead_code)]
pub fn axiom_fin_cayley_formula_ty() -> Expr {
    fin_ext_forall_nat(fin_ext_nat_eq(
        fin_ext_app(fin_ext_cst("Fin.labeledTreeCount"), Expr::BVar(0)),
        fin_ext_app2(
            fin_ext_cst("Nat.pow"),
            Expr::BVar(0),
            fin_ext_app2(
                fin_ext_cst("Nat.sub"),
                Expr::BVar(0),
                Expr::Lit(oxilean_kernel::Literal::Nat(2)),
            ),
        ),
    ))
}
/// Fin.ramanujan_sum : Nat → Nat → Int
/// (Ramanujan sum c_q(n))
#[allow(dead_code)]
pub fn axiom_fin_ramanujan_sum_ty() -> Expr {
    fin_ext_pi(
        BinderInfo::Default,
        "q",
        fin_ext_nat(),
        fin_ext_pi(BinderInfo::Default, "n", fin_ext_nat(), fin_ext_cst("Int")),
    )
}
/// Fin.Perm.sign : ∀ n, Fin.Perm n → Int
/// (sign/signature of a permutation: +1 or -1)
#[allow(dead_code)]
pub fn axiom_fin_perm_sign_ty() -> Expr {
    fin_ext_forall_nat(fin_ext_pi(
        BinderInfo::Default,
        "σ",
        fin_ext_app(fin_ext_cst("Fin.Perm"), Expr::BVar(0)),
        fin_ext_cst("Int"),
    ))
}
/// Fin.Perm.sign_comp : ∀ n (σ τ : Fin.Perm n),
///   Eq (Fin.Perm.sign (Fin.Perm.comp σ τ)) (Int.mul (Fin.Perm.sign σ) (Fin.Perm.sign τ))
#[allow(dead_code)]
pub fn axiom_fin_perm_sign_comp_ty() -> Expr {
    fin_ext_forall_nat(fin_ext_pi(
        BinderInfo::Default,
        "σ",
        fin_ext_app(fin_ext_cst("Fin.Perm"), Expr::BVar(0)),
        fin_ext_pi(
            BinderInfo::Default,
            "τ",
            fin_ext_app(fin_ext_cst("Fin.Perm"), Expr::BVar(1)),
            fin_ext_app3(
                Expr::Const(Name::str("Eq"), vec![Level::succ(Level::zero())]),
                fin_ext_cst("Int"),
                fin_ext_app(
                    fin_ext_cst("Fin.Perm.sign"),
                    fin_ext_app3(
                        fin_ext_cst("Fin.Perm.comp"),
                        Expr::BVar(2),
                        Expr::BVar(1),
                        Expr::BVar(0),
                    ),
                ),
                fin_ext_app2(
                    fin_ext_cst("Int.mul"),
                    fin_ext_app2(fin_ext_cst("Fin.Perm.sign"), Expr::BVar(2), Expr::BVar(1)),
                    fin_ext_app2(fin_ext_cst("Fin.Perm.sign"), Expr::BVar(2), Expr::BVar(0)),
                ),
            ),
        ),
    ))
}
/// Fin.Perm.sign_id : ∀ n, Eq (Fin.Perm.sign (Fin.Perm.id n)) (Int.ofNat 1)
#[allow(dead_code)]
pub fn axiom_fin_perm_sign_id_ty() -> Expr {
    fin_ext_forall_nat(fin_ext_app3(
        Expr::Const(Name::str("Eq"), vec![Level::succ(Level::zero())]),
        fin_ext_cst("Int"),
        fin_ext_app2(
            fin_ext_cst("Fin.Perm.sign"),
            Expr::BVar(0),
            fin_ext_app(fin_ext_cst("Fin.Perm.id"), Expr::BVar(0)),
        ),
        fin_ext_app(
            fin_ext_cst("Int.ofNat"),
            Expr::Lit(oxilean_kernel::Literal::Nat(1)),
        ),
    ))
}
/// Fin.burnside_count : ∀ n (G : Fin.Perm n → Prop) (X : Fin n → Prop),
///   (orbit counting lemma placeholder)
#[allow(dead_code)]
pub fn axiom_fin_burnside_ty() -> Expr {
    fin_ext_prop()
}
/// Fin.fubini : ∀ m n (f : Fin (m * n) → Nat),
///   Eq (Finset.sum (Fin (m*n)) f)
///      (Finset.sum (Fin m) (fun i => Finset.sum (Fin n) (fun j => f (fin_pair_to_index i j))))
#[allow(dead_code)]
pub fn axiom_fin_fubini_ty() -> Expr {
    fin_ext_pi(
        BinderInfo::Default,
        "m",
        fin_ext_nat(),
        fin_ext_pi(BinderInfo::Default, "n", fin_ext_nat(), fin_ext_prop()),
    )
}
/// Register all extended Fin axioms into the environment.
#[allow(clippy::too_many_lines)]
pub fn register_fin_extended(env: &mut Environment) -> Result<(), String> {
    macro_rules! reg {
        ($name:expr, $ty:expr) => {
            env.add(Declaration::Axiom {
                name: Name::str($name),
                univ_params: vec![],
                ty: $ty,
            })
            .map_err(|e| e.to_string())?;
        };
    }
    reg!(
        "Fintype.card",
        fin_ext_pi(BinderInfo::Default, "α", fin_ext_type1(), fin_ext_nat())
    );
    reg!(
        "Nat.lt",
        fin_ext_pi(
            BinderInfo::Default,
            "a",
            fin_ext_nat(),
            fin_ext_pi(BinderInfo::Default, "b", fin_ext_nat(), fin_ext_prop())
        )
    );
    reg!(
        "Nat.mod",
        fin_ext_pi(
            BinderInfo::Default,
            "a",
            fin_ext_nat(),
            fin_ext_pi(BinderInfo::Default, "b", fin_ext_nat(), fin_ext_nat())
        )
    );
    reg!(
        "Nat.add",
        fin_ext_pi(
            BinderInfo::Default,
            "a",
            fin_ext_nat(),
            fin_ext_pi(BinderInfo::Default, "b", fin_ext_nat(), fin_ext_nat())
        )
    );
    reg!(
        "Nat.sub",
        fin_ext_pi(
            BinderInfo::Default,
            "a",
            fin_ext_nat(),
            fin_ext_pi(BinderInfo::Default, "b", fin_ext_nat(), fin_ext_nat())
        )
    );
    reg!(
        "Nat.mul",
        fin_ext_pi(
            BinderInfo::Default,
            "a",
            fin_ext_nat(),
            fin_ext_pi(BinderInfo::Default, "b", fin_ext_nat(), fin_ext_nat())
        )
    );
    reg!(
        "Nat.pow",
        fin_ext_pi(
            BinderInfo::Default,
            "a",
            fin_ext_nat(),
            fin_ext_pi(BinderInfo::Default, "b", fin_ext_nat(), fin_ext_nat())
        )
    );
    reg!("Nat.factorial", axiom_nat_factorial_ty());
    reg!("Nat.factorial_zero", axiom_nat_factorial_zero_ty());
    reg!("Nat.factorial_succ", axiom_nat_factorial_succ_ty());
    reg!(
        "Nat.Prime",
        fin_ext_pi(BinderInfo::Default, "n", fin_ext_nat(), fin_ext_prop())
    );
    reg!(
        "Nat.Coprime",
        fin_ext_pi(
            BinderInfo::Default,
            "a",
            fin_ext_nat(),
            fin_ext_pi(BinderInfo::Default, "b", fin_ext_nat(), fin_ext_prop())
        )
    );
    reg!(
        "Iff",
        fin_ext_pi(
            BinderInfo::Default,
            "_",
            fin_ext_prop(),
            fin_ext_pi(BinderInfo::Default, "_", fin_ext_prop(), fin_ext_prop())
        )
    );
    reg!("Int", fin_ext_type1());
    reg!(
        "Int.ofNat",
        fin_ext_pi(BinderInfo::Default, "n", fin_ext_nat(), fin_ext_cst("Int"))
    );
    reg!(
        "Int.mul",
        fin_ext_pi(
            BinderInfo::Default,
            "a",
            fin_ext_cst("Int"),
            fin_ext_pi(
                BinderInfo::Default,
                "b",
                fin_ext_cst("Int"),
                fin_ext_cst("Int")
            )
        )
    );
    reg!("Fin.Perm", axiom_fin_perm_ty());
    reg!("Fin.Perm.id", axiom_fin_perm_id_ty());
    reg!("Fin.Perm.comp", axiom_fin_perm_comp_ty());
    reg!("Fin.Perm.inv", axiom_fin_perm_inv_ty());
    reg!("Fin.Perm.sign", axiom_fin_perm_sign_ty());
    reg!("Fin.Perm.sign_comp", axiom_fin_perm_sign_comp_ty());
    reg!("Fin.Perm.sign_id", axiom_fin_perm_sign_id_ty());
    reg!("Fin.Perm.count", axiom_fin_perm_count_ty());
    reg!("Fin.card", axiom_fin_card_ty());
    reg!("Fin.val_fin_lt", axiom_fin_val_fin_lt_ty());
    reg!("Fin.zero_val", axiom_fin_zero_val_ty());
    reg!("Fin.succ_val", axiom_fin_succ_val_ty());
    reg!("Fin.last_val", axiom_fin_last_val_ty());
    reg!("Fin.castSucc_val", axiom_fin_cast_succ_val_ty());
    reg!("Fin.add_val", axiom_fin_add_val_ty());
    reg!("Fin.rev_val", axiom_fin_rev_val_ty());
    reg!("Fin.eq_iff_val_eq", axiom_fin_eq_iff_val_eq_ty());
    reg!("Fin.ofNat_val", axiom_fin_of_nat_val_ty());
    reg!("Fin.zeroElim", axiom_fin_zero_elim_ty());
    reg!("Fin.derangementCount", axiom_fin_derangement_count_ty());
    reg!("Fin.stirlingFirst", axiom_fin_stirling_first_ty());
    reg!("Fin.stirlingSecond", axiom_fin_stirling_second_ty());
    reg!("Fin.binomial", axiom_fin_binomial_ty());
    reg!("Fin.binomial_zero", axiom_fin_binomial_zero_ty());
    reg!("Fin.binomial_self", axiom_fin_binomial_self_ty());
    reg!("Fin.binomial_symm", axiom_fin_binomial_symm_ty());
    reg!("Fin.pascal", axiom_fin_pascal_ty());
    reg!("Fin.eulerPhi", axiom_fin_euler_phi_ty());
    reg!("Fin.eulerPhi_prime", axiom_fin_euler_phi_prime_ty());
    reg!("Fin.eulerPhi_mul", axiom_fin_euler_phi_mul_ty());
    reg!(
        "Fin.labeledTreeCount",
        fin_ext_pi(BinderInfo::Default, "n", fin_ext_nat(), fin_ext_nat())
    );
    reg!("Fin.cayley_formula", axiom_fin_cayley_formula_ty());
    reg!("Fin.ramanujan_sum", axiom_fin_ramanujan_sum_ty());
    reg!("Fin.burnside", axiom_fin_burnside_ty());
    reg!("Fin.fubini", axiom_fin_fubini_ty());
    Ok(())
}
pub fn gcd_usize(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd_usize(b, a % b)
    }
}
pub fn lcm_usize(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        0
    } else {
        a / gcd_usize(a, b) * b
    }
}
/// Compute the number of derangements of n elements using the subfactorial recurrence.
#[allow(dead_code)]
pub fn fin_ext_derangements(n: usize) -> u64 {
    if n == 0 {
        return 1;
    }
    if n == 1 {
        return 0;
    }
    let mut d_prev = 1u64;
    let mut d_curr = 0u64;
    for k in 2..=n {
        let d_next = ((k as u64) - 1) * (d_curr + d_prev);
        d_prev = d_curr;
        d_curr = d_next;
    }
    d_curr
}
/// Binomial coefficient C(n, k) using multiplicative formula.
#[allow(dead_code)]
pub fn fin_ext_binomial(n: usize, k: usize) -> u64 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut result = 1u64;
    for i in 0..k {
        result = result * (n - i) as u64 / (i + 1) as u64;
    }
    result
}
/// Stirling numbers of the second kind S(n, k) via inclusion-exclusion.
#[allow(dead_code)]
pub fn fin_ext_stirling_second(n: usize, k: usize) -> u64 {
    if k == 0 {
        return if n == 0 { 1 } else { 0 };
    }
    if k > n {
        return 0;
    }
    let mut sum: i64 = 0;
    for j in 0..=k {
        let sign: i64 = if (k - j) % 2 == 0 { 1 } else { -1 };
        let binom = fin_ext_binomial(k, j) as i64;
        let power = (j as i64).pow(n as u32);
        sum += sign * binom * power;
    }
    let factorial_k: u64 = (1..=k as u64).product();
    (sum.unsigned_abs()) / factorial_k
}
/// Euler's totient function φ(n): count of integers in \[1,n\] coprime to n.
#[allow(dead_code)]
pub fn fin_ext_euler_phi(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    let mut result = n;
    let mut p = 2usize;
    let mut m = n;
    while p * p <= m {
        if m % p == 0 {
            while m % p == 0 {
                m /= p;
            }
            result -= result / p;
        }
        p += 1;
    }
    if m > 1 {
        result -= result / m;
    }
    result
}
