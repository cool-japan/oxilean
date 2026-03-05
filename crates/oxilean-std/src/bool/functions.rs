//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, InductiveEnv, Level, Name};

use super::types::{BoolExpr, TruthTable};

/// The Bool type expression.
#[allow(dead_code)]
pub fn bool_ty() -> Expr {
    Expr::Const(Name::str("Bool"), vec![])
}
/// Create a `true` expression.
pub fn bool_true() -> Expr {
    Expr::Const(Name::str("true"), vec![])
}
/// Create a `false` expression.
pub fn bool_false() -> Expr {
    Expr::Const(Name::str("false"), vec![])
}
/// Create `Bool.not b`.
#[allow(dead_code)]
pub fn bool_not(b: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::Const(Name::str("Bool.not"), vec![])),
        Box::new(b),
    )
}
/// Create `Bool.and a b`.
#[allow(dead_code)]
pub fn bool_and(a: Expr, b: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Bool.and"), vec![])),
            Box::new(a),
        )),
        Box::new(b),
    )
}
/// Create `Bool.or a b`.
#[allow(dead_code)]
pub fn bool_or(a: Expr, b: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Bool.or"), vec![])),
            Box::new(a),
        )),
        Box::new(b),
    )
}
/// Create `Bool.xor a b`.
#[allow(dead_code)]
pub fn bool_xor(a: Expr, b: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Bool.xor"), vec![])),
            Box::new(a),
        )),
        Box::new(b),
    )
}
/// Create `Bool.beq a b`.
#[allow(dead_code)]
pub fn bool_beq(a: Expr, b: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Bool.beq"), vec![])),
            Box::new(a),
        )),
        Box::new(b),
    )
}
/// Create `Bool.rec motive false_case true_case b`.
#[allow(dead_code)]
pub fn bool_rec(motive: Expr, false_case: Expr, true_case: Expr, b: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Bool.rec"), vec![])),
                    Box::new(motive),
                )),
                Box::new(false_case),
            )),
            Box::new(true_case),
        )),
        Box::new(b),
    )
}
/// Create `Eq a b` where both are Bool.
#[allow(dead_code)]
pub fn mk_bool_eq(a: Expr, b: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Eq"), vec![])),
                Box::new(bool_ty()),
            )),
            Box::new(a),
        )),
        Box::new(b),
    )
}
pub fn arrow(a: Expr, b: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::Anonymous,
        Box::new(a),
        Box::new(b),
    )
}
pub fn forall_bool(name: &str, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str(name),
        Box::new(bool_ty()),
        Box::new(body),
    )
}
pub fn eq_bool(lhs: Expr, rhs: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Eq"), vec![])),
                Box::new(bool_ty()),
            )),
            Box::new(lhs),
        )),
        Box::new(rhs),
    )
}
/// Build the Bool type and all associated declarations.
pub fn build_bool_env(env: &mut Environment, _ind_env: &mut InductiveEnv) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let bool_c = bool_ty();
    env.add(Declaration::Axiom {
        name: Name::str("Bool"),
        univ_params: vec![],
        ty: type1.clone(),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("true"),
        univ_params: vec![],
        ty: bool_c.clone(),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("false"),
        univ_params: vec![],
        ty: bool_c.clone(),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Bool.not"),
        univ_params: vec![],
        ty: arrow(bool_c.clone(), bool_c.clone()),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Bool.and"),
        univ_params: vec![],
        ty: arrow(bool_c.clone(), arrow(bool_c.clone(), bool_c.clone())),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Bool.or"),
        univ_params: vec![],
        ty: arrow(bool_c.clone(), arrow(bool_c.clone(), bool_c.clone())),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Bool.xor"),
        univ_params: vec![],
        ty: arrow(bool_c.clone(), arrow(bool_c.clone(), bool_c.clone())),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Bool.beq"),
        univ_params: vec![],
        ty: arrow(bool_c.clone(), arrow(bool_c.clone(), bool_c.clone())),
    })
    .map_err(|e| e.to_string())?;
    let u = Name::str("u");
    let sort_u = Expr::Sort(Level::Param(u.clone()));
    let motive_ty = arrow(bool_c.clone(), sort_u);
    let rec_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("C"),
        Box::new(motive_ty),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("hf"),
            Box::new(Expr::App(Box::new(Expr::BVar(0)), Box::new(bool_false()))),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("ht"),
                Box::new(Expr::App(Box::new(Expr::BVar(1)), Box::new(bool_true()))),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("b"),
                    Box::new(bool_c.clone()),
                    Box::new(Expr::App(Box::new(Expr::BVar(3)), Box::new(Expr::BVar(0)))),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Bool.rec"),
        univ_params: vec![u.clone()],
        ty: rec_ty,
    })
    .map_err(|e| e.to_string())?;
    let cases_motive_ty = arrow(bool_c.clone(), Expr::Sort(Level::Param(u.clone())));
    let cases_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("C"),
        Box::new(cases_motive_ty),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("b"),
            Box::new(bool_c.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("hf"),
                Box::new(Expr::App(Box::new(Expr::BVar(1)), Box::new(bool_false()))),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("ht"),
                    Box::new(Expr::App(Box::new(Expr::BVar(2)), Box::new(bool_true()))),
                    Box::new(Expr::App(Box::new(Expr::BVar(3)), Box::new(Expr::BVar(2)))),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Bool.casesOn"),
        univ_params: vec![u],
        ty: cases_ty,
    })
    .map_err(|e| e.to_string())?;
    let dec_eq_bool = Expr::App(
        Box::new(Expr::Const(Name::str("DecidableEq"), vec![])),
        Box::new(bool_c.clone()),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Bool.decEq"),
        univ_params: vec![],
        ty: dec_eq_bool,
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("BEq"),
        univ_params: vec![],
        ty: arrow(
            Expr::Sort(Level::succ(Level::zero())),
            Expr::Sort(Level::succ(Level::zero())),
        ),
    })
    .map_err(|e| e.to_string())?;
    let beq_beq_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("a"),
        Box::new(type1),
        Box::new(Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("inst"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("BEq"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("x"),
                Box::new(Expr::BVar(1)),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("y"),
                    Box::new(Expr::BVar(2)),
                    Box::new(bool_c),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("BEq.beq"),
        univ_params: vec![],
        ty: beq_beq_ty,
    })
    .map_err(|e| e.to_string())?;
    add_eq_if_missing(env)?;
    add_bool_theorem(
        env,
        "Bool.not_not",
        forall_bool(
            "b",
            eq_bool(bool_not(bool_not(Expr::BVar(0))), Expr::BVar(0)),
        ),
    )?;
    add_bool_theorem(
        env,
        "Bool.and_true",
        forall_bool(
            "b",
            eq_bool(bool_and(Expr::BVar(0), bool_true()), Expr::BVar(0)),
        ),
    )?;
    add_bool_theorem(
        env,
        "Bool.true_and",
        forall_bool(
            "b",
            eq_bool(bool_and(bool_true(), Expr::BVar(0)), Expr::BVar(0)),
        ),
    )?;
    add_bool_theorem(
        env,
        "Bool.and_false",
        forall_bool(
            "b",
            eq_bool(bool_and(Expr::BVar(0), bool_false()), bool_false()),
        ),
    )?;
    add_bool_theorem(
        env,
        "Bool.false_and",
        forall_bool(
            "b",
            eq_bool(bool_and(bool_false(), Expr::BVar(0)), bool_false()),
        ),
    )?;
    add_bool_theorem(
        env,
        "Bool.or_true",
        forall_bool(
            "b",
            eq_bool(bool_or(Expr::BVar(0), bool_true()), bool_true()),
        ),
    )?;
    add_bool_theorem(
        env,
        "Bool.true_or",
        forall_bool(
            "b",
            eq_bool(bool_or(bool_true(), Expr::BVar(0)), bool_true()),
        ),
    )?;
    add_bool_theorem(
        env,
        "Bool.or_false",
        forall_bool(
            "b",
            eq_bool(bool_or(Expr::BVar(0), bool_false()), Expr::BVar(0)),
        ),
    )?;
    add_bool_theorem(
        env,
        "Bool.false_or",
        forall_bool(
            "b",
            eq_bool(bool_or(bool_false(), Expr::BVar(0)), Expr::BVar(0)),
        ),
    )?;
    add_bool_theorem(
        env,
        "Bool.and_comm",
        forall_bool(
            "a",
            forall_bool(
                "b",
                eq_bool(
                    bool_and(Expr::BVar(1), Expr::BVar(0)),
                    bool_and(Expr::BVar(0), Expr::BVar(1)),
                ),
            ),
        ),
    )?;
    add_bool_theorem(
        env,
        "Bool.or_comm",
        forall_bool(
            "a",
            forall_bool(
                "b",
                eq_bool(
                    bool_or(Expr::BVar(1), Expr::BVar(0)),
                    bool_or(Expr::BVar(0), Expr::BVar(1)),
                ),
            ),
        ),
    )?;
    add_bool_theorem(
        env,
        "Bool.and_assoc",
        forall_bool(
            "a",
            forall_bool(
                "b",
                forall_bool(
                    "c",
                    eq_bool(
                        bool_and(bool_and(Expr::BVar(2), Expr::BVar(1)), Expr::BVar(0)),
                        bool_and(Expr::BVar(2), bool_and(Expr::BVar(1), Expr::BVar(0))),
                    ),
                ),
            ),
        ),
    )?;
    add_bool_theorem(
        env,
        "Bool.or_assoc",
        forall_bool(
            "a",
            forall_bool(
                "b",
                forall_bool(
                    "c",
                    eq_bool(
                        bool_or(bool_or(Expr::BVar(2), Expr::BVar(1)), Expr::BVar(0)),
                        bool_or(Expr::BVar(2), bool_or(Expr::BVar(1), Expr::BVar(0))),
                    ),
                ),
            ),
        ),
    )?;
    add_bool_theorem(
        env,
        "Bool.and_or_distrib",
        forall_bool(
            "a",
            forall_bool(
                "b",
                forall_bool(
                    "c",
                    eq_bool(
                        bool_and(Expr::BVar(2), bool_or(Expr::BVar(1), Expr::BVar(0))),
                        bool_or(
                            bool_and(Expr::BVar(2), Expr::BVar(1)),
                            bool_and(Expr::BVar(2), Expr::BVar(0)),
                        ),
                    ),
                ),
            ),
        ),
    )?;
    add_bool_theorem(
        env,
        "Bool.not_true",
        eq_bool(bool_not(bool_true()), bool_false()),
    )?;
    add_bool_theorem(
        env,
        "Bool.not_false",
        eq_bool(bool_not(bool_false()), bool_true()),
    )?;
    add_bool_theorem(
        env,
        "Bool.xor_comm",
        forall_bool(
            "a",
            forall_bool(
                "b",
                eq_bool(
                    bool_xor(Expr::BVar(1), Expr::BVar(0)),
                    bool_xor(Expr::BVar(0), Expr::BVar(1)),
                ),
            ),
        ),
    )?;
    add_bool_theorem(
        env,
        "Bool.beq_refl",
        forall_bool(
            "b",
            eq_bool(bool_beq(Expr::BVar(0), Expr::BVar(0)), bool_true()),
        ),
    )?;
    add_bool_theorem(
        env,
        "Bool.eq_of_beq",
        forall_bool(
            "a",
            forall_bool(
                "b",
                Expr::Pi(
                    BinderInfo::Default,
                    Name::str("h"),
                    Box::new(eq_bool(bool_beq(Expr::BVar(1), Expr::BVar(0)), bool_true())),
                    Box::new(eq_bool(Expr::BVar(2), Expr::BVar(1))),
                ),
            ),
        ),
    )?;
    Ok(())
}
/// Add Eq to environment if not already present.
pub fn add_eq_if_missing(env: &mut Environment) -> Result<(), String> {
    if env.contains(&Name::str("Eq")) {
        return Ok(());
    }
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let eq_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("a"),
        Box::new(type1),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::BVar(0)),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("y"),
                Box::new(Expr::BVar(1)),
                Box::new(Expr::Sort(Level::zero())),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Eq"),
        univ_params: vec![],
        ty: eq_ty,
    })
    .map_err(|e| e.to_string())
}
/// Add a theorem declaration with a sorry placeholder proof.
pub fn add_bool_theorem(env: &mut Environment, name: &str, ty: Expr) -> Result<(), String> {
    let val = Expr::Const(Name::str("sorry"), vec![]);
    env.add(Declaration::Theorem {
        name: Name::str(name),
        univ_params: vec![],
        ty,
        val,
    })
    .map_err(|e| e.to_string())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn setup_env() -> (Environment, InductiveEnv) {
        let mut env = Environment::new();
        let ind_env = InductiveEnv::new();
        env.add(Declaration::Axiom {
            name: Name::str("DecidableEq"),
            univ_params: vec![],
            ty: Expr::Pi(
                BinderInfo::Default,
                Name::str("a"),
                Box::new(Expr::Sort(Level::succ(Level::zero()))),
                Box::new(Expr::Sort(Level::succ(Level::zero()))),
            ),
        })
        .expect("operation should succeed");
        env.add(Declaration::Axiom {
            name: Name::str("sorry"),
            univ_params: vec![],
            ty: Expr::Sort(Level::zero()),
        })
        .expect("operation should succeed");
        (env, ind_env)
    }
    #[test]
    fn test_build_bool_env_full() {
        let (mut env, mut ind_env) = setup_env();
        assert!(build_bool_env(&mut env, &mut ind_env).is_ok());
        assert!(env.contains(&Name::str("Bool")));
        assert!(env.contains(&Name::str("true")));
        assert!(env.contains(&Name::str("false")));
        assert!(env.contains(&Name::str("Bool.not")));
        assert!(env.contains(&Name::str("Bool.and")));
        assert!(env.contains(&Name::str("Bool.or")));
        assert!(env.contains(&Name::str("Bool.xor")));
        assert!(env.contains(&Name::str("Bool.beq")));
        assert!(env.contains(&Name::str("Bool.rec")));
        assert!(env.contains(&Name::str("Bool.casesOn")));
        assert!(env.contains(&Name::str("Bool.decEq")));
        assert!(env.contains(&Name::str("BEq")));
        assert!(env.contains(&Name::str("BEq.beq")));
        for name in &[
            "Bool.not_not",
            "Bool.and_true",
            "Bool.true_and",
            "Bool.and_false",
            "Bool.false_and",
            "Bool.or_true",
            "Bool.true_or",
            "Bool.or_false",
            "Bool.false_or",
            "Bool.and_comm",
            "Bool.or_comm",
            "Bool.and_assoc",
            "Bool.or_assoc",
            "Bool.and_or_distrib",
            "Bool.not_true",
            "Bool.not_false",
            "Bool.xor_comm",
            "Bool.beq_refl",
            "Bool.eq_of_beq",
        ] {
            assert!(env.contains(&Name::str(*name)), "missing: {}", name);
        }
    }
    #[test]
    fn test_bool_ty() {
        let ty = bool_ty();
        assert!(matches!(ty, Expr::Const(_, _)));
    }
    #[test]
    fn test_bool_not_expr() {
        let not_b = bool_not(bool_true());
        assert!(matches!(not_b, Expr::App(_, _)));
    }
    #[test]
    fn test_bool_and_expr() {
        let e = bool_and(bool_true(), bool_false());
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_bool_or_expr() {
        let e = bool_or(bool_true(), bool_false());
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_bool_xor_expr() {
        let e = bool_xor(bool_true(), bool_false());
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_bool_beq_expr() {
        let e = bool_beq(bool_true(), bool_true());
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_bool_rec_expr() {
        let motive = Expr::Lam(
            BinderInfo::Default,
            Name::str("b"),
            Box::new(bool_ty()),
            Box::new(Expr::Sort(Level::succ(Level::zero()))),
        );
        let fc = Expr::Const(Name::str("Nat"), vec![]);
        let tc = Expr::Const(Name::str("Bool"), vec![]);
        let b = Expr::BVar(0);
        let rec = bool_rec(motive, fc, tc, b);
        assert!(matches!(rec, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_bool_eq_expr() {
        let e = mk_bool_eq(bool_true(), bool_false());
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_bool_true_false_distinct() {
        assert_ne!(bool_true(), bool_false());
    }
    #[test]
    fn test_bool_not_double_structure() {
        let double = bool_not(bool_not(bool_true()));
        match double {
            Expr::App(ref func, _) => {
                assert!(matches!(**func, Expr::Const(_, _)));
            }
            _ => panic!("Expected App"),
        }
    }
    #[test]
    fn test_bool_and_not_commutative_syntactically() {
        let a = bool_true();
        let b = bool_false();
        let ab = bool_and(a.clone(), b.clone());
        let ba = bool_and(b, a);
        assert_ne!(ab, ba);
    }
    #[test]
    fn test_bool_or_not_commutative_syntactically() {
        let a = bool_true();
        let b = bool_false();
        let ab = bool_or(a.clone(), b.clone());
        let ba = bool_or(b, a);
        assert_ne!(ab, ba);
    }
    #[test]
    fn test_bool_xor_binary() {
        let e = bool_xor(bool_true(), bool_false());
        match e {
            Expr::App(ref f, _) => assert!(matches!(**f, Expr::App(_, _))),
            _ => panic!("Expected nested App"),
        }
    }
    #[test]
    fn test_bool_beq_self() {
        let b = bool_true();
        let e = bool_beq(b.clone(), b);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_bool_rec_depth() {
        let motive = Expr::Const(Name::str("M"), vec![]);
        let fc = Expr::Const(Name::str("FC"), vec![]);
        let tc = Expr::Const(Name::str("TC"), vec![]);
        let b = Expr::Const(Name::str("bval"), vec![]);
        let rec = bool_rec(motive, fc, tc, b);
        let mut depth = 0;
        let mut cur = &rec;
        while let Expr::App(f, _) = cur {
            depth += 1;
            cur = f;
        }
        assert_eq!(depth, 4);
    }
    #[test]
    fn test_mk_bool_eq_structure() {
        let e = mk_bool_eq(bool_true(), bool_false());
        match e {
            Expr::App(ref inner, ref rhs) => {
                assert_eq!(**rhs, bool_false());
                match **inner {
                    Expr::App(ref inner2, ref lhs) => {
                        assert_eq!(**lhs, bool_true());
                        match **inner2 {
                            Expr::App(ref eq_c, ref ty) => {
                                assert_eq!(**eq_c, Expr::Const(Name::str("Eq"), vec![]));
                                assert_eq!(**ty, bool_ty());
                            }
                            _ => panic!("Expected App(Eq, Bool)"),
                        }
                    }
                    _ => panic!("Expected App"),
                }
            }
            _ => panic!("Expected App"),
        }
    }
    #[test]
    fn test_arrow_helper() {
        let arr = arrow(bool_ty(), bool_ty());
        match arr {
            Expr::Pi(info, name, _, _) => {
                assert_eq!(info, BinderInfo::Default);
                assert_eq!(name, Name::Anonymous);
            }
            _ => panic!("Expected Pi"),
        }
    }
    #[test]
    fn test_forall_bool_helper() {
        let fa = forall_bool("x", Expr::Sort(Level::zero()));
        match fa {
            Expr::Pi(info, name, ref dom, _) => {
                assert_eq!(info, BinderInfo::Default);
                assert_eq!(name, Name::str("x"));
                assert_eq!(**dom, bool_ty());
            }
            _ => panic!("Expected Pi"),
        }
    }
    #[test]
    fn test_eq_bool_helper() {
        let e = eq_bool(bool_true(), bool_false());
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_build_bool_env_old_compat() {
        let (mut env, mut ind_env) = setup_env();
        assert!(build_bool_env(&mut env, &mut ind_env).is_ok());
        assert!(env.contains(&Name::str("Bool")));
        assert!(env.contains(&Name::str("true")));
        assert!(env.contains(&Name::str("false")));
    }
    #[test]
    fn test_bool_helpers_smoke() {
        let _ = bool_ty();
        let _ = bool_true();
        let _ = bool_false();
        let _ = bool_not(bool_true());
        let _ = bool_and(bool_true(), bool_false());
        let _ = bool_or(bool_true(), bool_false());
        let _ = bool_xor(bool_true(), bool_false());
        let _ = bool_beq(bool_true(), bool_true());
        let _ = mk_bool_eq(bool_true(), bool_false());
    }
    #[test]
    fn test_bool_rec_concrete_motive() {
        let motive = Expr::Lam(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(bool_ty()),
            Box::new(Expr::Sort(Level::zero())),
        );
        let fc = Expr::Const(Name::str("proof_false"), vec![]);
        let tc = Expr::Const(Name::str("proof_true"), vec![]);
        let result = bool_rec(motive, fc, tc, bool_true());
        let mut depth = 0;
        let mut cur = &result;
        while let Expr::App(f, _) = cur {
            depth += 1;
            cur = f;
        }
        assert_eq!(depth, 4);
    }
}
/// Standard boolean truth tables.
/// Truth table for logical AND.
#[allow(dead_code)]
pub fn and_table() -> TruthTable {
    TruthTable::compute("and", |a, b| a && b)
}
/// Truth table for logical OR.
#[allow(dead_code)]
pub fn or_table() -> TruthTable {
    TruthTable::compute("or", |a, b| a || b)
}
/// Truth table for logical XOR.
#[allow(dead_code)]
pub fn xor_table() -> TruthTable {
    TruthTable::compute("xor", |a, b| a ^ b)
}
/// Truth table for logical NAND.
#[allow(dead_code)]
pub fn nand_table() -> TruthTable {
    TruthTable::compute("nand", |a, b| !(a && b))
}
/// Truth table for logical NOR.
#[allow(dead_code)]
pub fn nor_table() -> TruthTable {
    TruthTable::compute("nor", |a, b| !(a || b))
}
/// Truth table for logical IFF (biconditional).
#[allow(dead_code)]
pub fn iff_table() -> TruthTable {
    TruthTable::compute("iff", |a, b| a == b)
}
/// Truth table for logical implication.
#[allow(dead_code)]
pub fn imply_table() -> TruthTable {
    TruthTable::compute("imply", |a, b| !a || b)
}
#[cfg(test)]
mod extra_bool_tests {
    use super::*;
    #[test]
    fn test_truth_table_and() {
        let t = and_table();
        assert!(!t.lookup(false, false));
        assert!(!t.lookup(false, true));
        assert!(!t.lookup(true, false));
        assert!(t.lookup(true, true));
        assert!(t.is_commutative());
        assert!(!t.is_tautology());
        assert!(!t.is_contradiction());
        assert_eq!(t.true_count(), 1);
    }
    #[test]
    fn test_truth_table_or() {
        let t = or_table();
        assert!(t.is_commutative());
        assert_eq!(t.true_count(), 3);
    }
    #[test]
    fn test_truth_table_xor() {
        let t = xor_table();
        assert!(t.is_commutative());
        assert_eq!(t.true_count(), 2);
    }
    #[test]
    fn test_truth_table_iff_is_tautology_when_a_eq_b() {
        let t = iff_table();
        assert!(t.lookup(false, false));
        assert!(!t.lookup(false, true));
        assert_eq!(t.true_count(), 2);
    }
    #[test]
    fn test_truth_table_nand() {
        let t = nand_table();
        assert!(t.lookup(false, false));
        assert!(!t.lookup(true, true));
        assert_eq!(t.true_count(), 3);
    }
    #[test]
    fn test_truth_table_nor_is_contradiction() {
        let t = nor_table();
        assert!(!t.is_tautology());
        assert_eq!(t.true_count(), 1);
    }
    #[test]
    fn test_bool_expr_const() {
        let env = std::collections::HashMap::new();
        assert_eq!(BoolExpr::Const(true).eval(&env), Some(true));
        assert_eq!(BoolExpr::Const(false).eval(&env), Some(false));
    }
    #[test]
    fn test_bool_expr_var() {
        let mut env = std::collections::HashMap::new();
        env.insert("x", true);
        assert_eq!(BoolExpr::Var("x".to_string()).eval(&env), Some(true));
        assert_eq!(BoolExpr::Var("y".to_string()).eval(&env), None);
    }
    #[test]
    fn test_bool_expr_not() {
        let mut env = std::collections::HashMap::new();
        env.insert("x", false);
        let expr = BoolExpr::Not(Box::new(BoolExpr::Var("x".to_string())));
        assert_eq!(expr.eval(&env), Some(true));
    }
    #[test]
    fn test_bool_expr_and() {
        let mut env = std::collections::HashMap::new();
        env.insert("a", true);
        env.insert("b", false);
        let expr = BoolExpr::And(
            Box::new(BoolExpr::Var("a".to_string())),
            Box::new(BoolExpr::Var("b".to_string())),
        );
        assert_eq!(expr.eval(&env), Some(false));
    }
    #[test]
    fn test_bool_expr_or() {
        let mut env = std::collections::HashMap::new();
        env.insert("a", true);
        env.insert("b", false);
        let expr = BoolExpr::Or(
            Box::new(BoolExpr::Var("a".to_string())),
            Box::new(BoolExpr::Var("b".to_string())),
        );
        assert_eq!(expr.eval(&env), Some(true));
    }
    #[test]
    fn test_bool_expr_implies() {
        let mut env = std::collections::HashMap::new();
        env.insert("a", true);
        env.insert("b", false);
        let expr = BoolExpr::Implies(
            Box::new(BoolExpr::Var("a".to_string())),
            Box::new(BoolExpr::Var("b".to_string())),
        );
        assert_eq!(expr.eval(&env), Some(false));
    }
    #[test]
    fn test_bool_expr_is_tautology_true() {
        let a = BoolExpr::Var("a".to_string());
        let not_a = BoolExpr::Not(Box::new(BoolExpr::Var("a".to_string())));
        let taut = BoolExpr::Or(Box::new(a), Box::new(not_a));
        assert!(taut.is_tautology());
    }
    #[test]
    fn test_bool_expr_is_tautology_false() {
        let expr = BoolExpr::Var("x".to_string());
        assert!(!expr.is_tautology());
    }
    #[test]
    fn test_bool_expr_variables() {
        let expr = BoolExpr::And(
            Box::new(BoolExpr::Var("x".to_string())),
            Box::new(BoolExpr::Or(
                Box::new(BoolExpr::Var("y".to_string())),
                Box::new(BoolExpr::Var("x".to_string())),
            )),
        );
        let vars = expr.variables();
        assert_eq!(vars.len(), 2);
        assert!(vars.contains(&"x".to_string()));
        assert!(vars.contains(&"y".to_string()));
    }
    #[test]
    fn test_imply_table_commutative() {
        let t = imply_table();
        assert!(!t.is_commutative());
    }
}
/// Build the De Morgan AND-NOT law: ¬(a ∧ b) = ¬a ∨ ¬b
pub fn bl_ext_demorgan_and(env: &mut Environment) -> Result<(), String> {
    let ty = forall_bool(
        "a",
        forall_bool(
            "b",
            eq_bool(
                bool_not(bool_and(Expr::BVar(1), Expr::BVar(0))),
                bool_or(bool_not(Expr::BVar(1)), bool_not(Expr::BVar(0))),
            ),
        ),
    );
    add_bool_theorem(env, "Bool.demorgan_and", ty)
}
/// Build the De Morgan OR-NOT law: ¬(a ∨ b) = ¬a ∧ ¬b
pub fn bl_ext_demorgan_or(env: &mut Environment) -> Result<(), String> {
    let ty = forall_bool(
        "a",
        forall_bool(
            "b",
            eq_bool(
                bool_not(bool_or(Expr::BVar(1), Expr::BVar(0))),
                bool_and(bool_not(Expr::BVar(1)), bool_not(Expr::BVar(0))),
            ),
        ),
    );
    add_bool_theorem(env, "Bool.demorgan_or", ty)
}
/// Build the AND complementation law: a ∧ ¬a = false
pub fn bl_ext_and_complement(env: &mut Environment) -> Result<(), String> {
    let ty = forall_bool(
        "a",
        eq_bool(
            bool_and(Expr::BVar(0), bool_not(Expr::BVar(0))),
            bool_false(),
        ),
    );
    add_bool_theorem(env, "Bool.and_complement", ty)
}
/// Build the OR complementation law: a ∨ ¬a = true
pub fn bl_ext_or_complement(env: &mut Environment) -> Result<(), String> {
    let ty = forall_bool(
        "a",
        eq_bool(bool_or(Expr::BVar(0), bool_not(Expr::BVar(0))), bool_true()),
    );
    add_bool_theorem(env, "Bool.or_complement", ty)
}
/// Build the AND idempotency law: a ∧ a = a
pub fn bl_ext_and_idempotent(env: &mut Environment) -> Result<(), String> {
    let ty = forall_bool(
        "a",
        eq_bool(bool_and(Expr::BVar(0), Expr::BVar(0)), Expr::BVar(0)),
    );
    add_bool_theorem(env, "Bool.and_idempotent", ty)
}
/// Build the OR idempotency law: a ∨ a = a
pub fn bl_ext_or_idempotent(env: &mut Environment) -> Result<(), String> {
    let ty = forall_bool(
        "a",
        eq_bool(bool_or(Expr::BVar(0), Expr::BVar(0)), Expr::BVar(0)),
    );
    add_bool_theorem(env, "Bool.or_idempotent", ty)
}
/// Build the AND-OR absorption law: a ∧ (a ∨ b) = a
pub fn bl_ext_and_absorption(env: &mut Environment) -> Result<(), String> {
    let ty = forall_bool(
        "a",
        forall_bool(
            "b",
            eq_bool(
                bool_and(Expr::BVar(1), bool_or(Expr::BVar(1), Expr::BVar(0))),
                Expr::BVar(1),
            ),
        ),
    );
    add_bool_theorem(env, "Bool.and_absorption", ty)
}
/// Build the OR-AND absorption law: a ∨ (a ∧ b) = a
pub fn bl_ext_or_absorption(env: &mut Environment) -> Result<(), String> {
    let ty = forall_bool(
        "a",
        forall_bool(
            "b",
            eq_bool(
                bool_or(Expr::BVar(1), bool_and(Expr::BVar(1), Expr::BVar(0))),
                Expr::BVar(1),
            ),
        ),
    );
    add_bool_theorem(env, "Bool.or_absorption", ty)
}
/// Build the Bool-as-ring XOR ring axiom: (Bool, XOR, AND) forms a field GF(2).
pub fn bl_ext_xor_ring(env: &mut Environment) -> Result<(), String> {
    let ty = forall_bool(
        "a",
        eq_bool(bool_xor(Expr::BVar(0), Expr::BVar(0)), bool_false()),
    );
    add_bool_theorem(env, "Bool.xor_self", ty)
}
/// Build the XOR associativity law.
pub fn bl_ext_xor_assoc(env: &mut Environment) -> Result<(), String> {
    let ty = forall_bool(
        "a",
        forall_bool(
            "b",
            forall_bool(
                "c",
                eq_bool(
                    bool_xor(bool_xor(Expr::BVar(2), Expr::BVar(1)), Expr::BVar(0)),
                    bool_xor(Expr::BVar(2), bool_xor(Expr::BVar(1), Expr::BVar(0))),
                ),
            ),
        ),
    );
    add_bool_theorem(env, "Bool.xor_assoc", ty)
}
/// Build the AND distributivity over XOR law: a ∧ (b XOR c) = (a ∧ b) XOR (a ∧ c).
pub fn bl_ext_and_distrib_xor(env: &mut Environment) -> Result<(), String> {
    let ty = forall_bool(
        "a",
        forall_bool(
            "b",
            forall_bool(
                "c",
                eq_bool(
                    bool_and(Expr::BVar(2), bool_xor(Expr::BVar(1), Expr::BVar(0))),
                    bool_xor(
                        bool_and(Expr::BVar(2), Expr::BVar(1)),
                        bool_and(Expr::BVar(2), Expr::BVar(0)),
                    ),
                ),
            ),
        ),
    );
    add_bool_theorem(env, "Bool.and_distrib_xor", ty)
}
/// Build the lattice join-commutativity axiom (OR is commutative join).
pub fn bl_ext_lattice_join_comm(env: &mut Environment) -> Result<(), String> {
    let ty = forall_bool(
        "a",
        forall_bool(
            "b",
            eq_bool(
                bool_or(Expr::BVar(1), Expr::BVar(0)),
                bool_or(Expr::BVar(0), Expr::BVar(1)),
            ),
        ),
    );
    add_bool_theorem(env, "Bool.lattice_join_comm", ty)
}
/// Build the lattice meet-commutativity axiom (AND is commutative meet).
pub fn bl_ext_lattice_meet_comm(env: &mut Environment) -> Result<(), String> {
    let ty = forall_bool(
        "a",
        forall_bool(
            "b",
            eq_bool(
                bool_and(Expr::BVar(1), Expr::BVar(0)),
                bool_and(Expr::BVar(0), Expr::BVar(1)),
            ),
        ),
    );
    add_bool_theorem(env, "Bool.lattice_meet_comm", ty)
}
/// Build the Heyting algebra implication axiom.
/// In Bool, a => b = ¬a ∨ b (material implication)
pub fn bl_ext_heyting_implication(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    env.add(Declaration::Axiom {
        name: Name::str("BoolImply"),
        univ_params: vec![],
        ty: Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(bool_ty()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_"),
                Box::new(bool_ty()),
                Box::new(bool_ty()),
            )),
        ),
    })
    .map_err(|e| e.to_string())?;
    let _ = type1;
    let ty = forall_bool(
        "a",
        forall_bool(
            "b",
            eq_bool(
                Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("BoolImply"), vec![])),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::BVar(0)),
                ),
                bool_or(bool_not(Expr::BVar(1)), Expr::BVar(0)),
            ),
        ),
    );
    add_bool_theorem(env, "Bool.imply_def", ty)
}
/// Build the Heyting reflexivity law: a => a = true
pub fn bl_ext_heyting_refl(env: &mut Environment) -> Result<(), String> {
    let ty = forall_bool(
        "a",
        eq_bool(bool_or(bool_not(Expr::BVar(0)), Expr::BVar(0)), bool_true()),
    );
    add_bool_theorem(env, "Bool.imply_refl", ty)
}
/// Build the B2 two-element Boolean algebra axiom.
/// B2 = {false, true} is the unique Boolean algebra of size 2.
pub fn bl_ext_b2_algebra(env: &mut Environment) -> Result<(), String> {
    env.add(Declaration::Axiom {
        name: Name::str("B2Card"),
        univ_params: vec![],
        ty: Expr::Sort(Level::zero()),
    })
    .map_err(|e| e.to_string())
}
/// Build the Bool-Prop decidability bridge axiom.
/// decide : Decidable P -> Bool  (extract boolean from decidable prop)
pub fn bl_ext_decidable_to_bool(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    env.add(Declaration::Axiom {
        name: Name::str("DecidableToBool"),
        univ_params: vec![],
        ty: Expr::Pi(
            BinderInfo::Default,
            Name::str("α"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("DecidableEq"), vec![])),
                    Box::new(Expr::BVar(0)),
                )),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_"),
                    Box::new(Expr::BVar(1)),
                    Box::new(bool_ty()),
                )),
            )),
        ),
    })
    .map_err(|e| e.to_string())
}
/// Build the Bool reflection axiom: beq_iff_eq.
/// a = b <-> beq a b = true
pub fn bl_ext_bool_reflection(env: &mut Environment) -> Result<(), String> {
    let ty = forall_bool(
        "a",
        forall_bool(
            "b",
            eq_bool(bool_beq(Expr::BVar(1), Expr::BVar(0)), bool_true()),
        ),
    );
    add_bool_theorem(env, "Bool.beq_iff_eq_aux", ty)
}
/// Build the BEq class instance for Bool.
/// instance : BEq Bool via Bool.beq
pub fn bl_ext_beq_bool_instance(env: &mut Environment) -> Result<(), String> {
    env.add(Declaration::Axiom {
        name: Name::str("BEqBoolInst"),
        univ_params: vec![],
        ty: Expr::App(
            Box::new(Expr::Const(Name::str("BEq"), vec![])),
            Box::new(bool_ty()),
        ),
    })
    .map_err(|e| e.to_string())
}
/// Build the Bool-valued predicate functor axiom.
/// A Bool predicate P: α -> Bool can be lifted to Prop via (P x = true).
pub fn bl_ext_bool_pred_prop(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    env.add(Declaration::Axiom {
        name: Name::str("BoolPredToProp"),
        univ_params: vec![],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("α"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("p"),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_"),
                    Box::new(Expr::BVar(0)),
                    Box::new(bool_ty()),
                )),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("x"),
                    Box::new(Expr::BVar(1)),
                    Box::new(Expr::Sort(Level::zero())),
                )),
            )),
        ),
    })
    .map_err(|e| e.to_string())
}
/// Build the short-circuit AND law: false && f = false (f not evaluated).
pub fn bl_ext_short_circuit_and(env: &mut Environment) -> Result<(), String> {
    let ty = forall_bool(
        "b",
        eq_bool(bool_and(bool_false(), Expr::BVar(0)), bool_false()),
    );
    add_bool_theorem(env, "Bool.short_circuit_and", ty)
}
/// Build the short-circuit OR law: true || f = true (f not evaluated).
pub fn bl_ext_short_circuit_or(env: &mut Environment) -> Result<(), String> {
    let ty = forall_bool(
        "b",
        eq_bool(bool_or(bool_true(), Expr::BVar(0)), bool_true()),
    );
    add_bool_theorem(env, "Bool.short_circuit_or", ty)
}
/// Build the XOR false identity: a XOR false = a.
pub fn bl_ext_xor_false(env: &mut Environment) -> Result<(), String> {
    let ty = forall_bool(
        "a",
        eq_bool(bool_xor(Expr::BVar(0), bool_false()), Expr::BVar(0)),
    );
    add_bool_theorem(env, "Bool.xor_false", ty)
}
/// Build the XOR true law: a XOR true = ¬a.
pub fn bl_ext_xor_true(env: &mut Environment) -> Result<(), String> {
    let ty = forall_bool(
        "a",
        eq_bool(
            bool_xor(Expr::BVar(0), bool_true()),
            bool_not(Expr::BVar(0)),
        ),
    );
    add_bool_theorem(env, "Bool.xor_true", ty)
}
/// Build the NAND functional completeness axiom.
/// Every boolean function is expressible via NAND alone.
pub fn bl_ext_nand_complete(env: &mut Environment) -> Result<(), String> {
    env.add(Declaration::Axiom {
        name: Name::str("Bool.nand"),
        univ_params: vec![],
        ty: Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(bool_ty()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_"),
                Box::new(bool_ty()),
                Box::new(bool_ty()),
            )),
        ),
    })
    .map_err(|e| e.to_string())?;
    let ty = forall_bool(
        "a",
        forall_bool(
            "b",
            eq_bool(
                Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Bool.nand"), vec![])),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::BVar(0)),
                ),
                bool_not(bool_and(Expr::BVar(1), Expr::BVar(0))),
            ),
        ),
    );
    add_bool_theorem(env, "Bool.nand_def", ty)
}
/// Build the NOR functional completeness axiom.
/// Every boolean function is expressible via NOR alone.
pub fn bl_ext_nor_complete(env: &mut Environment) -> Result<(), String> {
    env.add(Declaration::Axiom {
        name: Name::str("Bool.nor"),
        univ_params: vec![],
        ty: Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(bool_ty()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_"),
                Box::new(bool_ty()),
                Box::new(bool_ty()),
            )),
        ),
    })
    .map_err(|e| e.to_string())?;
    let ty = forall_bool(
        "a",
        forall_bool(
            "b",
            eq_bool(
                Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Bool.nor"), vec![])),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::BVar(0)),
                ),
                bool_not(bool_or(Expr::BVar(1), Expr::BVar(0))),
            ),
        ),
    );
    add_bool_theorem(env, "Bool.nor_def", ty)
}
/// Build the AND monoid identity axiom.
/// (Bool, AND, true) is a monoid.
pub fn bl_ext_and_monoid_id(env: &mut Environment) -> Result<(), String> {
    let ty_l = forall_bool(
        "b",
        eq_bool(bool_and(bool_true(), Expr::BVar(0)), Expr::BVar(0)),
    );
    add_bool_theorem(env, "Bool.and_monoid_left_id", ty_l)?;
    let ty_r = forall_bool(
        "b",
        eq_bool(bool_and(Expr::BVar(0), bool_true()), Expr::BVar(0)),
    );
    add_bool_theorem(env, "Bool.and_monoid_right_id", ty_r)
}
/// Build the OR monoid identity axiom.
/// (Bool, OR, false) is a monoid.
pub fn bl_ext_or_monoid_id(env: &mut Environment) -> Result<(), String> {
    let ty_l = forall_bool(
        "b",
        eq_bool(bool_or(bool_false(), Expr::BVar(0)), Expr::BVar(0)),
    );
    add_bool_theorem(env, "Bool.or_monoid_left_id", ty_l)?;
    let ty_r = forall_bool(
        "b",
        eq_bool(bool_or(Expr::BVar(0), bool_false()), Expr::BVar(0)),
    );
    add_bool_theorem(env, "Bool.or_monoid_right_id", ty_r)
}
/// Build the boolean fold-all axiom.
/// all p xs = foldr (∧) true (map p xs)
pub fn bl_ext_bool_fold_all(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    env.add(Declaration::Axiom {
        name: Name::str("BoolAll"),
        univ_params: vec![],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("α"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("p"),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_"),
                    Box::new(Expr::BVar(0)),
                    Box::new(bool_ty()),
                )),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_xs"),
                    Box::new(type1.clone()),
                    Box::new(bool_ty()),
                )),
            )),
        ),
    })
    .map_err(|e| e.to_string())
}
/// Build the boolean fold-any axiom.
/// any p xs = foldr (∨) false (map p xs)
pub fn bl_ext_bool_fold_any(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    env.add(Declaration::Axiom {
        name: Name::str("BoolAny"),
        univ_params: vec![],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("α"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("p"),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_"),
                    Box::new(Expr::BVar(0)),
                    Box::new(bool_ty()),
                )),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_xs"),
                    Box::new(type1.clone()),
                    Box::new(bool_ty()),
                )),
            )),
        ),
    })
    .map_err(|e| e.to_string())
}
/// Build the if-then-else semantics axiom.
/// ite true a b = a, ite false a b = b
pub fn bl_ext_ite_semantics(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    env.add(Declaration::Axiom {
        name: Name::str("BoolIte"),
        univ_params: vec![],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("α"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("cond"),
                Box::new(bool_ty()),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("t"),
                    Box::new(Expr::BVar(1)),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("f"),
                        Box::new(Expr::BVar(2)),
                        Box::new(Expr::BVar(3)),
                    )),
                )),
            )),
        ),
    })
    .map_err(|e| e.to_string())
}
/// Build the ITE true branch law: ite true a b = a
pub fn bl_ext_ite_true(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    env.add(Declaration::Axiom {
        name: Name::str("BoolIte.true_branch"),
        univ_params: vec![],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("α"),
            Box::new(type1.clone()),
            Box::new(Expr::Sort(Level::zero())),
        ),
    })
    .map_err(|e| e.to_string())
}
/// Build the ITE false branch law: ite false a b = b
pub fn bl_ext_ite_false(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    env.add(Declaration::Axiom {
        name: Name::str("BoolIte.false_branch"),
        univ_params: vec![],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("α"),
            Box::new(type1.clone()),
            Box::new(Expr::Sort(Level::zero())),
        ),
    })
    .map_err(|e| e.to_string())
}
/// Build Kleene three-value extension axiom.
/// Kleene3 = {false, unknown, true} extends Bool with undecidability.
pub fn bl_ext_kleene_three_value(env: &mut Environment) -> Result<(), String> {
    env.add(Declaration::Axiom {
        name: Name::str("Kleene3"),
        univ_params: vec![],
        ty: Expr::Sort(Level::succ(Level::zero())),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Kleene3.unknown"),
        univ_params: vec![],
        ty: Expr::Const(Name::str("Kleene3"), vec![]),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Kleene3.and"),
        univ_params: vec![],
        ty: Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(Expr::Const(Name::str("Kleene3"), vec![])),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_"),
                Box::new(Expr::Const(Name::str("Kleene3"), vec![])),
                Box::new(Expr::Const(Name::str("Kleene3"), vec![])),
            )),
        ),
    })
    .map_err(|e| e.to_string())
}
/// Build De Morgan duality as a meta-theorem axiom.
/// The De Morgan laws establish duality between ∧ and ∨.
pub fn bl_ext_demorgan_duality(env: &mut Environment) -> Result<(), String> {
    env.add(Declaration::Axiom {
        name: Name::str("DeMorganDuality"),
        univ_params: vec![],
        ty: Expr::Sort(Level::zero()),
    })
    .map_err(|e| e.to_string())
}
/// Build the SAT decidability axiom.
/// Boolean satisfiability is decidable by exhaustive enumeration.
pub fn bl_ext_sat_decidable(env: &mut Environment) -> Result<(), String> {
    env.add(Declaration::Axiom {
        name: Name::str("SATDecidable"),
        univ_params: vec![],
        ty: Expr::Sort(Level::zero()),
    })
    .map_err(|e| e.to_string())
}
/// Build the tautology-check axiom.
/// A formula is a tautology iff it evaluates to true for all assignments.
pub fn bl_ext_tautology_check(env: &mut Environment) -> Result<(), String> {
    env.add(Declaration::Axiom {
        name: Name::str("TautologyCheck"),
        univ_params: vec![],
        ty: Expr::Sort(Level::zero()),
    })
    .map_err(|e| e.to_string())
}
/// Build the OR-distrib-AND law: a ∨ (b ∧ c) = (a ∨ b) ∧ (a ∨ c).
pub fn bl_ext_or_distrib_and(env: &mut Environment) -> Result<(), String> {
    let ty = forall_bool(
        "a",
        forall_bool(
            "b",
            forall_bool(
                "c",
                eq_bool(
                    bool_or(Expr::BVar(2), bool_and(Expr::BVar(1), Expr::BVar(0))),
                    bool_and(
                        bool_or(Expr::BVar(2), Expr::BVar(1)),
                        bool_or(Expr::BVar(2), Expr::BVar(0)),
                    ),
                ),
            ),
        ),
    );
    add_bool_theorem(env, "Bool.or_distrib_and", ty)
}
/// Build the NOT-and-OR duality axiom.
/// ¬a ∨ ¬b = ¬(a ∧ b)  (alternate De Morgan form)
pub fn bl_ext_not_or_demorgan(env: &mut Environment) -> Result<(), String> {
    let ty = forall_bool(
        "a",
        forall_bool(
            "b",
            eq_bool(
                bool_not(bool_and(Expr::BVar(1), Expr::BVar(0))),
                bool_or(bool_not(Expr::BVar(1)), bool_not(Expr::BVar(0))),
            ),
        ),
    );
    add_bool_theorem(env, "Bool.not_and_demorgan", ty)
}
/// Build the XOR-not-beq relationship: a XOR b = ¬(beq a b).
pub fn bl_ext_xor_not_beq(env: &mut Environment) -> Result<(), String> {
    let ty = forall_bool(
        "a",
        forall_bool(
            "b",
            eq_bool(
                bool_xor(Expr::BVar(1), Expr::BVar(0)),
                bool_not(bool_beq(Expr::BVar(1), Expr::BVar(0))),
            ),
        ),
    );
    add_bool_theorem(env, "Bool.xor_not_beq", ty)
}
/// Build the beq-true-iff law: beq a true = a.
pub fn bl_ext_beq_true(env: &mut Environment) -> Result<(), String> {
    let ty = forall_bool(
        "a",
        eq_bool(bool_beq(Expr::BVar(0), bool_true()), Expr::BVar(0)),
    );
    add_bool_theorem(env, "Bool.beq_true", ty)
}
/// Build the beq-false-iff law: beq a false = ¬a.
pub fn bl_ext_beq_false(env: &mut Environment) -> Result<(), String> {
    let ty = forall_bool(
        "a",
        eq_bool(
            bool_beq(Expr::BVar(0), bool_false()),
            bool_not(Expr::BVar(0)),
        ),
    );
    add_bool_theorem(env, "Bool.beq_false", ty)
}
/// Build the AND-false-annihilation axiom: a ∧ false = false.
pub fn bl_ext_and_false_annihilate(env: &mut Environment) -> Result<(), String> {
    let ty = forall_bool(
        "a",
        eq_bool(bool_and(Expr::BVar(0), bool_false()), bool_false()),
    );
    add_bool_theorem(env, "Bool.and_false_annihilate", ty)
}
/// Build the OR-true-annihilation axiom: a ∨ true = true.
pub fn bl_ext_or_true_annihilate(env: &mut Environment) -> Result<(), String> {
    let ty = forall_bool(
        "a",
        eq_bool(bool_or(Expr::BVar(0), bool_true()), bool_true()),
    );
    add_bool_theorem(env, "Bool.or_true_annihilate", ty)
}
/// Build the AND-self law: a ∧ a = a (idempotency alias).
pub fn bl_ext_and_self(env: &mut Environment) -> Result<(), String> {
    let ty = forall_bool(
        "a",
        eq_bool(bool_and(Expr::BVar(0), Expr::BVar(0)), Expr::BVar(0)),
    );
    add_bool_theorem(env, "Bool.and_self", ty)
}
/// Build the OR-self law: a ∨ a = a (idempotency alias).
pub fn bl_ext_or_self(env: &mut Environment) -> Result<(), String> {
    let ty = forall_bool(
        "a",
        eq_bool(bool_or(Expr::BVar(0), Expr::BVar(0)), Expr::BVar(0)),
    );
    add_bool_theorem(env, "Bool.or_self", ty)
}
/// Register all extended Bool axioms into the environment.
pub fn register_bool_extended_axioms(env: &mut Environment) {
    let builders: &[fn(&mut Environment) -> Result<(), String>] = &[
        bl_ext_demorgan_and,
        bl_ext_demorgan_or,
        bl_ext_and_complement,
        bl_ext_or_complement,
        bl_ext_and_idempotent,
        bl_ext_or_idempotent,
        bl_ext_and_absorption,
        bl_ext_or_absorption,
        bl_ext_xor_ring,
        bl_ext_xor_assoc,
        bl_ext_and_distrib_xor,
        bl_ext_lattice_join_comm,
        bl_ext_lattice_meet_comm,
        bl_ext_heyting_implication,
        bl_ext_heyting_refl,
        bl_ext_b2_algebra,
        bl_ext_decidable_to_bool,
        bl_ext_bool_reflection,
        bl_ext_beq_bool_instance,
        bl_ext_bool_pred_prop,
        bl_ext_short_circuit_and,
        bl_ext_short_circuit_or,
        bl_ext_xor_false,
        bl_ext_xor_true,
        bl_ext_nand_complete,
        bl_ext_nor_complete,
        bl_ext_and_monoid_id,
        bl_ext_or_monoid_id,
        bl_ext_bool_fold_all,
        bl_ext_bool_fold_any,
        bl_ext_ite_semantics,
        bl_ext_ite_true,
        bl_ext_ite_false,
        bl_ext_kleene_three_value,
        bl_ext_demorgan_duality,
        bl_ext_sat_decidable,
        bl_ext_tautology_check,
        bl_ext_or_distrib_and,
        bl_ext_not_or_demorgan,
        bl_ext_xor_not_beq,
        bl_ext_beq_true,
        bl_ext_beq_false,
        bl_ext_and_false_annihilate,
        bl_ext_or_true_annihilate,
        bl_ext_and_self,
        bl_ext_or_self,
    ];
    for builder in builders {
        let _ = builder(env);
    }
}
/// Evaluate boolean XOR as ring addition in GF(2).
#[allow(dead_code)]
pub fn gf2_add(a: bool, b: bool) -> bool {
    a ^ b
}
/// Evaluate boolean AND as ring multiplication in GF(2).
#[allow(dead_code)]
pub fn gf2_mul(a: bool, b: bool) -> bool {
    a && b
}
