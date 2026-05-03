//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

pub fn arrow(a: Expr, b: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::Anonymous,
        Box::new(a),
        Box::new(b),
    )
}
pub(super) fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub(super) fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn decidable_const() -> Expr {
    Expr::Const(Name::str("Decidable"), vec![])
}
pub fn bool_const() -> Expr {
    Expr::Const(Name::str("Bool"), vec![])
}
/// Create `Decidable p`.
#[allow(dead_code)]
pub fn mk_decidable(p: Expr) -> Expr {
    Expr::App(Box::new(decidable_const()), Box::new(p))
}
/// Create `DecidableEq ty`.
#[allow(dead_code)]
pub fn mk_decidable_eq(ty: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::Const(Name::str("DecidableEq"), vec![])),
        Box::new(ty),
    )
}
/// Create `@ite cond then_branch else_branch`.
///
/// The full signature is `ite : {a : Sort u} -> (c : Prop) -> \[Decidable c\] -> a -> a -> a`
/// but we build the simplified application `ite cond then_branch else_branch`.
#[allow(dead_code)]
pub fn mk_ite(cond: Expr, then_branch: Expr, else_branch: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("ite"), vec![])),
                Box::new(cond),
            )),
            Box::new(then_branch),
        )),
        Box::new(else_branch),
    )
}
/// Create `Decidable.decide p` (instance resolution).
#[allow(dead_code)]
pub fn decide(p: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::Const(Name::str("Decidable.decide"), vec![])),
        Box::new(p),
    )
}
/// Create `Decidable.toBool dec`.
#[allow(dead_code)]
pub fn to_bool(dec: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::Const(Name::str("Decidable.toBool"), vec![])),
        Box::new(dec),
    )
}
/// Build propositional logic declarations in the environment.
pub fn build_prop_env(env: &mut Environment) -> Result<(), String> {
    add_prereqs_if_missing(env)?;
    env.add(Declaration::Axiom {
        name: Name::str("Decidable.decide"),
        univ_params: vec![],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("p"),
            Box::new(prop()),
            Box::new(Expr::Pi(
                BinderInfo::InstImplicit,
                Name::str("inst"),
                Box::new(mk_decidable(Expr::BVar(0))),
                Box::new(mk_decidable(Expr::BVar(1))),
            )),
        ),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Decidable.toBool"),
        univ_params: vec![],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("p"),
            Box::new(prop()),
            Box::new(arrow(mk_decidable(Expr::BVar(0)), bool_const())),
        ),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Decidable.byDecide"),
        univ_params: vec![],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("p"),
            Box::new(prop()),
            Box::new(Expr::Pi(
                BinderInfo::InstImplicit,
                Name::str("inst"),
                Box::new(mk_decidable(Expr::BVar(0))),
                Box::new(arrow(Expr::BVar(1), bool_const())),
            )),
        ),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("instDecidableTrue"),
        univ_params: vec![],
        ty: mk_decidable(Expr::Const(Name::str("True"), vec![])),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("instDecidableFalse"),
        univ_params: vec![],
        ty: mk_decidable(Expr::Const(Name::str("False"), vec![])),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("instDecidableAnd"),
        univ_params: vec![],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("p"),
            Box::new(prop()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("q"),
                Box::new(prop()),
                Box::new(Expr::Pi(
                    BinderInfo::InstImplicit,
                    Name::str("dp"),
                    Box::new(mk_decidable(Expr::BVar(1))),
                    Box::new(Expr::Pi(
                        BinderInfo::InstImplicit,
                        Name::str("dq"),
                        Box::new(mk_decidable(Expr::BVar(1))),
                        Box::new(mk_decidable(Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("And"), vec![])),
                                Box::new(Expr::BVar(3)),
                            )),
                            Box::new(Expr::BVar(2)),
                        ))),
                    )),
                )),
            )),
        ),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("instDecidableOr"),
        univ_params: vec![],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("p"),
            Box::new(prop()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("q"),
                Box::new(prop()),
                Box::new(Expr::Pi(
                    BinderInfo::InstImplicit,
                    Name::str("dp"),
                    Box::new(mk_decidable(Expr::BVar(1))),
                    Box::new(Expr::Pi(
                        BinderInfo::InstImplicit,
                        Name::str("dq"),
                        Box::new(mk_decidable(Expr::BVar(1))),
                        Box::new(mk_decidable(Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("Or"), vec![])),
                                Box::new(Expr::BVar(3)),
                            )),
                            Box::new(Expr::BVar(2)),
                        ))),
                    )),
                )),
            )),
        ),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("instDecidableNot"),
        univ_params: vec![],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("p"),
            Box::new(prop()),
            Box::new(Expr::Pi(
                BinderInfo::InstImplicit,
                Name::str("dp"),
                Box::new(mk_decidable(Expr::BVar(0))),
                Box::new(mk_decidable(Expr::App(
                    Box::new(Expr::Const(Name::str("Not"), vec![])),
                    Box::new(Expr::BVar(1)),
                ))),
            )),
        ),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("instDecidableIff"),
        univ_params: vec![],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("p"),
            Box::new(prop()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("q"),
                Box::new(prop()),
                Box::new(Expr::Pi(
                    BinderInfo::InstImplicit,
                    Name::str("dp"),
                    Box::new(mk_decidable(Expr::BVar(1))),
                    Box::new(Expr::Pi(
                        BinderInfo::InstImplicit,
                        Name::str("dq"),
                        Box::new(mk_decidable(Expr::BVar(1))),
                        Box::new(mk_decidable(Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("Iff"), vec![])),
                                Box::new(Expr::BVar(3)),
                            )),
                            Box::new(Expr::BVar(2)),
                        ))),
                    )),
                )),
            )),
        ),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("decidable_of_iff"),
        univ_params: vec![],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("p"),
            Box::new(prop()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("q"),
                Box::new(prop()),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("h"),
                    Box::new(Expr::App(
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("Iff"), vec![])),
                            Box::new(Expr::BVar(1)),
                        )),
                        Box::new(Expr::BVar(0)),
                    )),
                    Box::new(arrow(
                        mk_decidable(Expr::BVar(2)),
                        mk_decidable(Expr::BVar(1)),
                    )),
                )),
            )),
        ),
    })
    .map_err(|e| e.to_string())?;
    let u_name = Name::str("u");
    let sort_u = Expr::Sort(Level::Param(u_name.clone()));
    env.add(Declaration::Axiom {
        name: Name::str("ite"),
        univ_params: vec![u_name.clone()],
        ty: Expr::Pi(
            BinderInfo::Implicit,
            Name::str("a"),
            Box::new(sort_u),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("c"),
                Box::new(prop()),
                Box::new(Expr::Pi(
                    BinderInfo::InstImplicit,
                    Name::str("dec"),
                    Box::new(mk_decidable(Expr::BVar(0))),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("t"),
                        Box::new(Expr::BVar(2)),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("e"),
                            Box::new(Expr::BVar(3)),
                            Box::new(Expr::BVar(4)),
                        )),
                    )),
                )),
            )),
        ),
    })
    .map_err(|e| e.to_string())?;
    add_sorry_if_missing(env)?;
    let if_true_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("a"),
        Box::new(Expr::Sort(Level::Param(u_name.clone()))),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("t"),
            Box::new(Expr::BVar(0)),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("e"),
                Box::new(Expr::BVar(1)),
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Eq"), vec![])),
                        Box::new(Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::App(
                                    Box::new(Expr::Const(Name::str("ite"), vec![])),
                                    Box::new(Expr::Const(Name::str("True"), vec![])),
                                )),
                                Box::new(Expr::BVar(1)),
                            )),
                            Box::new(Expr::BVar(0)),
                        )),
                    )),
                    Box::new(Expr::BVar(1)),
                )),
            )),
        )),
    );
    env.add(Declaration::Theorem {
        name: Name::str("if_true"),
        univ_params: vec![u_name.clone()],
        ty: if_true_ty,
        val: Expr::Const(Name::str("sorry"), vec![]),
    })
    .map_err(|e| e.to_string())?;
    let if_false_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("a"),
        Box::new(Expr::Sort(Level::Param(u_name.clone()))),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("t"),
            Box::new(Expr::BVar(0)),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("e"),
                Box::new(Expr::BVar(1)),
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Eq"), vec![])),
                        Box::new(Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::App(
                                    Box::new(Expr::Const(Name::str("ite"), vec![])),
                                    Box::new(Expr::Const(Name::str("False"), vec![])),
                                )),
                                Box::new(Expr::BVar(1)),
                            )),
                            Box::new(Expr::BVar(0)),
                        )),
                    )),
                    Box::new(Expr::BVar(0)),
                )),
            )),
        )),
    );
    env.add(Declaration::Theorem {
        name: Name::str("if_false"),
        univ_params: vec![u_name],
        ty: if_false_ty,
        val: Expr::Const(Name::str("sorry"), vec![]),
    })
    .map_err(|e| e.to_string())?;
    Ok(())
}
pub(super) fn add_prereqs_if_missing(env: &mut Environment) -> Result<(), String> {
    if !env.contains(&Name::str("Decidable")) {
        env.add(Declaration::Axiom {
            name: Name::str("Decidable"),
            univ_params: vec![],
            ty: arrow(prop(), type1()),
        })
        .map_err(|e| e.to_string())?;
    }
    if !env.contains(&Name::str("Bool")) {
        env.add(Declaration::Axiom {
            name: Name::str("Bool"),
            univ_params: vec![],
            ty: type1(),
        })
        .map_err(|e| e.to_string())?;
    }
    if !env.contains(&Name::str("DecidableEq")) {
        env.add(Declaration::Axiom {
            name: Name::str("DecidableEq"),
            univ_params: vec![],
            ty: arrow(type1(), type1()),
        })
        .map_err(|e| e.to_string())?;
    }
    if !env.contains(&Name::str("True")) {
        env.add(Declaration::Axiom {
            name: Name::str("True"),
            univ_params: vec![],
            ty: prop(),
        })
        .map_err(|e| e.to_string())?;
    }
    if !env.contains(&Name::str("False")) {
        env.add(Declaration::Axiom {
            name: Name::str("False"),
            univ_params: vec![],
            ty: prop(),
        })
        .map_err(|e| e.to_string())?;
    }
    if !env.contains(&Name::str("And")) {
        env.add(Declaration::Axiom {
            name: Name::str("And"),
            univ_params: vec![],
            ty: arrow(prop(), arrow(prop(), prop())),
        })
        .map_err(|e| e.to_string())?;
    }
    if !env.contains(&Name::str("Or")) {
        env.add(Declaration::Axiom {
            name: Name::str("Or"),
            univ_params: vec![],
            ty: arrow(prop(), arrow(prop(), prop())),
        })
        .map_err(|e| e.to_string())?;
    }
    if !env.contains(&Name::str("Not")) {
        env.add(Declaration::Axiom {
            name: Name::str("Not"),
            univ_params: vec![],
            ty: arrow(prop(), prop()),
        })
        .map_err(|e| e.to_string())?;
    }
    if !env.contains(&Name::str("Iff")) {
        env.add(Declaration::Axiom {
            name: Name::str("Iff"),
            univ_params: vec![],
            ty: arrow(prop(), arrow(prop(), prop())),
        })
        .map_err(|e| e.to_string())?;
    }
    if !env.contains(&Name::str("Eq")) {
        let eq_ty = Expr::Pi(
            BinderInfo::Implicit,
            Name::str("a"),
            Box::new(type1()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("x"),
                Box::new(Expr::BVar(0)),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("y"),
                    Box::new(Expr::BVar(1)),
                    Box::new(prop()),
                )),
            )),
        );
        env.add(Declaration::Axiom {
            name: Name::str("Eq"),
            univ_params: vec![],
            ty: eq_ty,
        })
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
pub fn add_sorry_if_missing(env: &mut Environment) -> Result<(), String> {
    if !env.contains(&Name::str("sorry")) {
        env.add(Declaration::Axiom {
            name: Name::str("sorry"),
            univ_params: vec![],
            ty: prop(),
        })
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn setup_env() -> Environment {
        Environment::new()
    }
    #[test]
    fn test_build_prop_env() {
        let mut env = setup_env();
        assert!(build_prop_env(&mut env).is_ok());
        for name in &[
            "Decidable.decide",
            "Decidable.toBool",
            "Decidable.byDecide",
            "instDecidableTrue",
            "instDecidableFalse",
            "instDecidableAnd",
            "instDecidableOr",
            "instDecidableNot",
            "instDecidableIff",
            "decidable_of_iff",
            "ite",
            "if_true",
            "if_false",
        ] {
            assert!(env.contains(&Name::str(*name)), "missing: {}", name);
        }
    }
    #[test]
    fn test_mk_decidable() {
        let p = Expr::Const(Name::str("myProp"), vec![]);
        let d = mk_decidable(p);
        match d {
            Expr::App(ref f, _) => {
                assert_eq!(**f, Expr::Const(Name::str("Decidable"), vec![]));
            }
            _ => panic!("Expected App"),
        }
    }
    #[test]
    fn test_mk_decidable_eq() {
        let ty = Expr::Const(Name::str("Nat"), vec![]);
        let d = mk_decidable_eq(ty);
        match d {
            Expr::App(ref f, _) => {
                assert_eq!(**f, Expr::Const(Name::str("DecidableEq"), vec![]));
            }
            _ => panic!("Expected App"),
        }
    }
    #[test]
    fn test_mk_ite() {
        let cond = Expr::Const(Name::str("c"), vec![]);
        let then_b = Expr::Const(Name::str("t"), vec![]);
        let else_b = Expr::Const(Name::str("e"), vec![]);
        let e = mk_ite(cond, then_b, else_b);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_ite_depth() {
        let cond = Expr::Const(Name::str("c"), vec![]);
        let then_b = Expr::Const(Name::str("t"), vec![]);
        let else_b = Expr::Const(Name::str("e"), vec![]);
        let e = mk_ite(cond, then_b, else_b);
        let mut depth = 0;
        let mut cur = &e;
        while let Expr::App(f, _) = cur {
            depth += 1;
            cur = f;
        }
        assert_eq!(depth, 3);
    }
    #[test]
    fn test_decide() {
        let p = Expr::Const(Name::str("myProp"), vec![]);
        let d = decide(p);
        match d {
            Expr::App(ref f, _) => {
                assert_eq!(**f, Expr::Const(Name::str("Decidable.decide"), vec![]));
            }
            _ => panic!("Expected App"),
        }
    }
    #[test]
    fn test_to_bool() {
        let dec = Expr::Const(Name::str("myDec"), vec![]);
        let b = to_bool(dec);
        match b {
            Expr::App(ref f, _) => {
                assert_eq!(**f, Expr::Const(Name::str("Decidable.toBool"), vec![]));
            }
            _ => panic!("Expected App"),
        }
    }
    #[test]
    fn test_mk_decidable_structure() {
        let p = Expr::Const(Name::str("P"), vec![]);
        let d = mk_decidable(p.clone());
        assert_eq!(
            d,
            Expr::App(
                Box::new(Expr::Const(Name::str("Decidable"), vec![])),
                Box::new(p),
            )
        );
    }
    #[test]
    fn test_mk_decidable_eq_structure() {
        let ty = Expr::Const(Name::str("Bool"), vec![]);
        let d = mk_decidable_eq(ty.clone());
        assert_eq!(
            d,
            Expr::App(
                Box::new(Expr::Const(Name::str("DecidableEq"), vec![])),
                Box::new(ty),
            )
        );
    }
    #[test]
    fn test_prereqs_added() {
        let mut env = setup_env();
        build_prop_env(&mut env).expect("build_prop_env should succeed");
        assert!(env.contains(&Name::str("Decidable")));
        assert!(env.contains(&Name::str("Bool")));
        assert!(env.contains(&Name::str("True")));
        assert!(env.contains(&Name::str("False")));
        assert!(env.contains(&Name::str("And")));
        assert!(env.contains(&Name::str("Or")));
        assert!(env.contains(&Name::str("Not")));
        assert!(env.contains(&Name::str("Iff")));
        assert!(env.contains(&Name::str("Eq")));
    }
    #[test]
    fn test_helpers_smoke() {
        let p = Expr::Const(Name::str("P"), vec![]);
        let _ = mk_decidable(p.clone());
        let _ = mk_decidable_eq(p.clone());
        let _ = mk_ite(p.clone(), bool_const(), bool_const());
        let _ = decide(p.clone());
        let _ = to_bool(p);
    }
    #[test]
    fn test_build_prop_env_idempotent_prereqs() {
        let mut env = setup_env();
        assert!(build_prop_env(&mut env).is_ok());
    }
    #[test]
    fn test_ite_with_bool_cond() {
        let cond = Expr::Const(Name::str("True"), vec![]);
        let t = Expr::Const(Name::str("x"), vec![]);
        let e = Expr::Const(Name::str("y"), vec![]);
        let ite = mk_ite(cond, t, e);
        assert!(matches!(ite, Expr::App(_, _)));
    }
    #[test]
    fn test_decide_and_to_bool_compose() {
        let p = Expr::Const(Name::str("P"), vec![]);
        let dec = decide(p);
        let b = to_bool(dec);
        match b {
            Expr::App(_, ref arg) => {
                assert!(matches!(**arg, Expr::App(_, _)));
            }
            _ => panic!("Expected App"),
        }
    }
    #[test]
    fn test_prop_and_type1_distinct() {
        assert_ne!(prop(), type1());
    }
}
/// Create `And p q` (conjunction).
#[allow(dead_code)]
pub fn mk_and(p: Expr, q: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("And"), vec![])),
            Box::new(p),
        )),
        Box::new(q),
    )
}
/// Create `Or p q` (disjunction).
#[allow(dead_code)]
pub fn mk_or(p: Expr, q: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Or"), vec![])),
            Box::new(p),
        )),
        Box::new(q),
    )
}
/// Create `Not p` (negation).
#[allow(dead_code)]
pub fn mk_not(p: Expr) -> Expr {
    Expr::App(Box::new(Expr::Const(Name::str("Not"), vec![])), Box::new(p))
}
/// Create `Iff p q` (biconditional).
#[allow(dead_code)]
pub fn mk_iff(p: Expr, q: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Iff"), vec![])),
            Box::new(p),
        )),
        Box::new(q),
    )
}
/// Create `Eq x y` (propositional equality, simplified form).
#[allow(dead_code)]
pub fn mk_eq(x: Expr, y: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Eq"), vec![])),
            Box::new(x),
        )),
        Box::new(y),
    )
}
/// Create `True` (logical true).
#[allow(dead_code)]
pub fn mk_true() -> Expr {
    Expr::Const(Name::str("True"), vec![])
}
/// Create `False` (logical false).
#[allow(dead_code)]
pub fn mk_false() -> Expr {
    Expr::Const(Name::str("False"), vec![])
}
#[cfg(test)]
mod extra_prop_tests {
    use super::*;
    #[test]
    fn test_mk_and_structure() {
        let p = Expr::Const(Name::str("P"), vec![]);
        let q = Expr::Const(Name::str("Q"), vec![]);
        let and = mk_and(p.clone(), q.clone());
        match &and {
            Expr::App(f, arg_q) => {
                assert_eq!(**arg_q, q);
                match f.as_ref() {
                    Expr::App(g, arg_p) => {
                        assert_eq!(**arg_p, p);
                        assert_eq!(**g, Expr::Const(Name::str("And"), vec![]));
                    }
                    _ => panic!("Expected nested App"),
                }
            }
            _ => panic!("Expected App"),
        }
    }
    #[test]
    fn test_mk_or_structure() {
        let p = Expr::Const(Name::str("P"), vec![]);
        let q = Expr::Const(Name::str("Q"), vec![]);
        let or = mk_or(p, q);
        assert!(matches!(or, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_not_structure() {
        let p = Expr::Const(Name::str("P"), vec![]);
        let not_p = mk_not(p.clone());
        match not_p {
            Expr::App(f, arg) => {
                assert_eq!(*f, Expr::Const(Name::str("Not"), vec![]));
                assert_eq!(*arg, p);
            }
            _ => panic!("Expected App"),
        }
    }
    #[test]
    fn test_mk_iff_structure() {
        let p = Expr::Const(Name::str("P"), vec![]);
        let q = Expr::Const(Name::str("Q"), vec![]);
        let iff = mk_iff(p, q);
        assert!(matches!(iff, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_eq_structure() {
        let x = Expr::Const(Name::str("a"), vec![]);
        let y = Expr::Const(Name::str("b"), vec![]);
        let eq = mk_eq(x, y);
        assert!(matches!(eq, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_true() {
        assert_eq!(mk_true(), Expr::Const(Name::str("True"), vec![]));
    }
    #[test]
    fn test_mk_false() {
        assert_eq!(mk_false(), Expr::Const(Name::str("False"), vec![]));
    }
    #[test]
    fn test_mk_and_of_true_false() {
        let t = mk_true();
        let f = mk_false();
        let and = mk_and(t, f);
        assert!(matches!(and, Expr::App(_, _)));
    }
    #[test]
    fn test_double_negation_structure() {
        let p = Expr::Const(Name::str("P"), vec![]);
        let nn_p = mk_not(mk_not(p));
        assert!(matches!(nn_p, Expr::App(_, _)));
    }
    #[test]
    fn test_prop_and_type1() {
        assert_eq!(prop(), Expr::Sort(Level::zero()));
        assert_ne!(prop(), Expr::Sort(Level::succ(Level::zero())));
    }
}
/// Simplify a propositional expression with trivial reductions.
///
/// Applies:
/// - `And True p → p`
/// - `And p True → p`
/// - `Or False p → p`
/// - `Or p False → p`
/// - `Not True → False`
/// - `Not False → True`
#[allow(dead_code)]
pub fn prop_simplify(expr: &Expr) -> Expr {
    match expr {
        Expr::App(f, arg) => {
            let f_s = prop_simplify(f);
            let a_s = prop_simplify(arg);
            if let Expr::App(inner_f, inner_a) = &f_s {
                if let Expr::Const(n, _) = inner_f.as_ref() {
                    if n == &Name::str("And") {
                        if *inner_a.as_ref() == mk_true() {
                            return a_s;
                        }
                        if a_s == mk_true() {
                            return *inner_a.clone();
                        }
                    }
                    if n == &Name::str("Or") {
                        if *inner_a.as_ref() == mk_false() {
                            return a_s;
                        }
                        if a_s == mk_false() {
                            return *inner_a.clone();
                        }
                    }
                }
            }
            if let Expr::Const(n, _) = &f_s {
                if n == &Name::str("Not") {
                    if a_s == mk_true() {
                        return mk_false();
                    }
                    if a_s == mk_false() {
                        return mk_true();
                    }
                }
            }
            Expr::App(Box::new(f_s), Box::new(a_s))
        }
        other => other.clone(),
    }
}
/// Check if an expression is a propositional tautology (True).
#[allow(dead_code)]
pub fn is_tautology(expr: &Expr) -> bool {
    *expr == mk_true()
}
/// Check if an expression is a propositional contradiction (False).
#[allow(dead_code)]
pub fn is_contradiction(expr: &Expr) -> bool {
    *expr == mk_false()
}
/// Count the logical connectives in a propositional expression.
#[allow(dead_code)]
pub fn count_connectives(expr: &Expr) -> usize {
    match expr {
        Expr::App(f, a) => {
            let mut count = 0;
            if let Expr::App(inner_f, _) = f.as_ref() {
                if let Expr::Const(n, _) = inner_f.as_ref() {
                    if n == &Name::str("And") || n == &Name::str("Or") || n == &Name::str("Iff") {
                        count += 1;
                    }
                }
            }
            if let Expr::Const(n, _) = f.as_ref() {
                if n == &Name::str("Not") {
                    count += 1;
                }
            }
            count + count_connectives(f) + count_connectives(a)
        }
        _ => 0,
    }
}
#[cfg(test)]
mod prop_simplify_tests {
    use super::*;
    #[test]
    fn test_simplify_and_true_left() {
        let p = Expr::Const(Name::str("P"), vec![]);
        let expr = mk_and(mk_true(), p.clone());
        let simplified = prop_simplify(&expr);
        assert_eq!(simplified, p);
    }
    #[test]
    fn test_simplify_and_true_right() {
        let p = Expr::Const(Name::str("P"), vec![]);
        let expr = mk_and(p.clone(), mk_true());
        let simplified = prop_simplify(&expr);
        assert_eq!(simplified, p);
    }
    #[test]
    fn test_simplify_or_false_left() {
        let p = Expr::Const(Name::str("P"), vec![]);
        let expr = mk_or(mk_false(), p.clone());
        let simplified = prop_simplify(&expr);
        assert_eq!(simplified, p);
    }
    #[test]
    fn test_simplify_or_false_right() {
        let p = Expr::Const(Name::str("P"), vec![]);
        let expr = mk_or(p.clone(), mk_false());
        let simplified = prop_simplify(&expr);
        assert_eq!(simplified, p);
    }
    #[test]
    fn test_simplify_not_true() {
        let expr = mk_not(mk_true());
        let simplified = prop_simplify(&expr);
        assert_eq!(simplified, mk_false());
    }
    #[test]
    fn test_simplify_not_false() {
        let expr = mk_not(mk_false());
        let simplified = prop_simplify(&expr);
        assert_eq!(simplified, mk_true());
    }
    #[test]
    fn test_is_tautology() {
        assert!(is_tautology(&mk_true()));
        assert!(!is_tautology(&mk_false()));
    }
    #[test]
    fn test_is_contradiction() {
        assert!(is_contradiction(&mk_false()));
        assert!(!is_contradiction(&mk_true()));
    }
    #[test]
    fn test_count_connectives_atom() {
        let p = Expr::Const(Name::str("P"), vec![]);
        assert_eq!(count_connectives(&p), 0);
    }
    #[test]
    fn test_count_connectives_and() {
        let p = Expr::Const(Name::str("P"), vec![]);
        let q = Expr::Const(Name::str("Q"), vec![]);
        let expr = mk_and(p, q);
        assert_eq!(count_connectives(&expr), 1);
    }
}
/// Helper: arrow type with anonymous binder.
pub(super) fn prp_ext_arrow(a: Expr, b: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::Anonymous,
        Box::new(a),
        Box::new(b),
    )
}
/// Type of `ClassicalLogic.excludedMiddle`: for all p : Prop, p ∨ ¬p.
/// `ClassicalLogic.excludedMiddle : ∀ p : Prop, Or p (Not p)`
#[allow(dead_code)]
pub fn axiom_excluded_middle_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("p"),
        Box::new(prop()),
        Box::new(Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Or"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Not"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
        )),
    )
}
/// Type of `ClassicalLogic.doubleNegationElim`: ¬¬p → p.
/// `ClassicalLogic.doubleNegationElim : ∀ p : Prop, Not (Not p) → p`
#[allow(dead_code)]
pub fn axiom_double_negation_elim_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("p"),
        Box::new(prop()),
        Box::new(prp_ext_arrow(
            Expr::App(
                Box::new(Expr::Const(Name::str("Not"), vec![])),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Not"), vec![])),
                    Box::new(Expr::BVar(0)),
                )),
            ),
            Expr::BVar(0),
        )),
    )
}
/// Type of `ClassicalLogic.peirce`: ((p → q) → p) → p (Peirce's law).
/// `ClassicalLogic.peirce : ∀ p q : Prop, ((p → q) → p) → p`
#[allow(dead_code)]
pub fn axiom_peirce_law_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("p"),
        Box::new(prop()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("q"),
            Box::new(prop()),
            Box::new(prp_ext_arrow(
                prp_ext_arrow(prp_ext_arrow(Expr::BVar(1), Expr::BVar(0)), Expr::BVar(1)),
                Expr::BVar(1),
            )),
        )),
    )
}
/// Type of `Completeness.propLogic`: propositional logic is complete with
/// respect to truth table semantics.
/// `Completeness.propLogic : Prop`
#[allow(dead_code)]
pub fn axiom_prop_logic_completeness_ty() -> Expr {
    Expr::Sort(Level::zero())
}
/// Type of `Lindstrom.theorem`: any logic extending first-order logic that
/// is compact and satisfies Lowenheim-Skolem is first-order logic.
/// `Lindstrom.theorem : Prop`
#[allow(dead_code)]
pub fn axiom_lindstrom_theorem_ty() -> Expr {
    Expr::Sort(Level::zero())
}
/// Type of `Godel.completeness`: every consistent set of first-order sentences
/// has a model.
/// `Godel.completeness : Prop`
#[allow(dead_code)]
pub fn axiom_godel_completeness_ty() -> Expr {
    Expr::Sort(Level::zero())
}
/// Type of `Craig.interpolation`: if p → q is a tautology, there exists an
/// interpolant r (using only common predicates of p, q) such that p → r and r → q.
/// `Craig.interpolation : Prop`
#[allow(dead_code)]
pub fn axiom_craig_interpolation_ty() -> Expr {
    Expr::Sort(Level::zero())
}
/// Type of `Beth.definability`: implicit definability implies explicit definability.
/// `Beth.definability : Prop`
#[allow(dead_code)]
pub fn axiom_beth_definability_ty() -> Expr {
    Expr::Sort(Level::zero())
}
/// Type of `KripkeFrame`: a Kripke frame for modal logic.
/// `KripkeFrame : Type 1`
#[allow(dead_code)]
pub fn axiom_kripke_frame_ty() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
}
/// Type of `KripkeFrame.worlds`: the set of worlds in a Kripke frame.
/// `KripkeFrame.worlds : KripkeFrame → Type`
#[allow(dead_code)]
pub fn axiom_kripke_frame_worlds_ty() -> Expr {
    prp_ext_arrow(
        Expr::Const(Name::str("KripkeFrame"), vec![]),
        Expr::Sort(Level::succ(Level::zero())),
    )
}
/// Type of `KripkeFrame.accessibility`: the accessibility relation between worlds.
/// `KripkeFrame.accessibility : ∀ (F : KripkeFrame), KripkeFrame.worlds F → KripkeFrame.worlds F → Prop`
#[allow(dead_code)]
pub fn axiom_kripke_accessibility_ty() -> Expr {
    let kf = Expr::Const(Name::str("KripkeFrame"), vec![]);
    let worlds_f = Expr::App(
        Box::new(Expr::Const(Name::str("KripkeFrame.worlds"), vec![])),
        Box::new(Expr::BVar(0)),
    );
    Expr::Pi(
        BinderInfo::Default,
        Name::str("F"),
        Box::new(kf),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("w1"),
            Box::new(worlds_f.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("w2"),
                Box::new(worlds_f),
                Box::new(Expr::Sort(Level::zero())),
            )),
        )),
    )
}
/// Type of `ModalLogic.necessitation`: if p is a theorem, then □p is a theorem.
/// `ModalLogic.necessitation : ∀ p : Prop, p → Box p`
#[allow(dead_code)]
pub fn axiom_modal_necessitation_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("p"),
        Box::new(prop()),
        Box::new(prp_ext_arrow(
            Expr::BVar(0),
            Expr::App(
                Box::new(Expr::Const(Name::str("Box"), vec![])),
                Box::new(Expr::BVar(0)),
            ),
        )),
    )
}
/// Type of `ModalLogic.K`: the K axiom: □(p → q) → □p → □q.
/// `ModalLogic.K : ∀ p q : Prop, Box (p → q) → Box p → Box q`
#[allow(dead_code)]
pub fn axiom_modal_k_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("p"),
        Box::new(prop()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("q"),
            Box::new(prop()),
            Box::new(prp_ext_arrow(
                Expr::App(
                    Box::new(Expr::Const(Name::str("Box"), vec![])),
                    Box::new(prp_ext_arrow(Expr::BVar(1), Expr::BVar(0))),
                ),
                prp_ext_arrow(
                    Expr::App(
                        Box::new(Expr::Const(Name::str("Box"), vec![])),
                        Box::new(Expr::BVar(1)),
                    ),
                    Expr::App(
                        Box::new(Expr::Const(Name::str("Box"), vec![])),
                        Box::new(Expr::BVar(0)),
                    ),
                ),
            )),
        )),
    )
}
/// Type of `ModalLogic.T`: the T axiom: □p → p (reflexivity).
/// `ModalLogic.T : ∀ p : Prop, Box p → p`
#[allow(dead_code)]
pub fn axiom_modal_t_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("p"),
        Box::new(prop()),
        Box::new(prp_ext_arrow(
            Expr::App(
                Box::new(Expr::Const(Name::str("Box"), vec![])),
                Box::new(Expr::BVar(0)),
            ),
            Expr::BVar(0),
        )),
    )
}
/// Type of `ModalLogic.S4`: the S4 axiom: □p → □□p (transitivity).
/// `ModalLogic.S4 : ∀ p : Prop, Box p → Box (Box p)`
#[allow(dead_code)]
pub fn axiom_modal_s4_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("p"),
        Box::new(prop()),
        Box::new(prp_ext_arrow(
            Expr::App(
                Box::new(Expr::Const(Name::str("Box"), vec![])),
                Box::new(Expr::BVar(0)),
            ),
            Expr::App(
                Box::new(Expr::Const(Name::str("Box"), vec![])),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Box"), vec![])),
                    Box::new(Expr::BVar(0)),
                )),
            ),
        )),
    )
}
/// Type of `ModalLogic.S5`: the S5 axiom: ◇p → □◇p (Euclidean).
/// `ModalLogic.S5 : ∀ p : Prop, Diamond p → Box (Diamond p)`
#[allow(dead_code)]
pub fn axiom_modal_s5_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("p"),
        Box::new(prop()),
        Box::new(prp_ext_arrow(
            Expr::App(
                Box::new(Expr::Const(Name::str("Diamond"), vec![])),
                Box::new(Expr::BVar(0)),
            ),
            Expr::App(
                Box::new(Expr::Const(Name::str("Box"), vec![])),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Diamond"), vec![])),
                    Box::new(Expr::BVar(0)),
                )),
            ),
        )),
    )
}
/// Type of `LTL.next`: the temporal next operator X p.
/// `LTL.next : Prop → Prop`
#[allow(dead_code)]
pub fn axiom_ltl_next_ty() -> Expr {
    prp_ext_arrow(prop(), prop())
}
/// Type of `LTL.until`: the temporal until operator p U q.
/// `LTL.until : Prop → Prop → Prop`
#[allow(dead_code)]
pub fn axiom_ltl_until_ty() -> Expr {
    prp_ext_arrow(prop(), prp_ext_arrow(prop(), prop()))
}
/// Type of `LTL.globally`: the temporal globally operator G p (□ in LTL).
/// `LTL.globally : Prop → Prop`
#[allow(dead_code)]
pub fn axiom_ltl_globally_ty() -> Expr {
    prp_ext_arrow(prop(), prop())
}
/// Type of `LTL.eventually`: the temporal eventually operator F p (◇ in LTL).
/// `LTL.eventually : Prop → Prop`
#[allow(dead_code)]
pub fn axiom_ltl_eventually_ty() -> Expr {
    prp_ext_arrow(prop(), prop())
}
/// Type of `LTL.unfolding`: F p ↔ p ∨ X(F p) — the unfolding axiom for until.
/// `LTL.unfolding : ∀ p : Prop, Iff (LTL.eventually p) (Or p (LTL.next (LTL.eventually p)))`
#[allow(dead_code)]
pub fn axiom_ltl_unfolding_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("p"),
        Box::new(prop()),
        Box::new(Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Iff"), vec![])),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("LTL.eventually"), vec![])),
                    Box::new(Expr::BVar(0)),
                )),
            )),
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Or"), vec![])),
                    Box::new(Expr::BVar(0)),
                )),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("LTL.next"), vec![])),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("LTL.eventually"), vec![])),
                        Box::new(Expr::BVar(0)),
                    )),
                )),
            )),
        )),
    )
}
/// Type of `IntuitionisticLogic.exFalso`: False → p (ex falso quodlibet).
/// `IntuitionisticLogic.exFalso : ∀ p : Prop, False → p`
#[allow(dead_code)]
pub fn axiom_ex_falso_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("p"),
        Box::new(prop()),
        Box::new(prp_ext_arrow(
            Expr::Const(Name::str("False"), vec![]),
            Expr::BVar(0),
        )),
    )
}
/// Type of `IntuitionisticLogic.noExcludedMiddle`: excluded middle is not
/// provable in intuitionistic logic.
/// `IntuitionisticLogic.noExcludedMiddle : Prop`
#[allow(dead_code)]
pub fn axiom_no_excluded_middle_ty() -> Expr {
    Expr::Sort(Level::zero())
}
/// Type of `BHK.proof`: the BHK interpretation assigns proof objects to propositions.
/// `BHK.proof : Prop → Type`
#[allow(dead_code)]
pub fn axiom_bhk_proof_ty() -> Expr {
    prp_ext_arrow(prop(), type1())
}
/// Type of `BHK.andIntro`: introduction rule for conjunction in BHK.
/// `BHK.andIntro : ∀ p q : Prop, BHK.proof p → BHK.proof q → BHK.proof (And p q)`
#[allow(dead_code)]
pub fn axiom_bhk_and_intro_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("p"),
        Box::new(prop()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("q"),
            Box::new(prop()),
            Box::new(prp_ext_arrow(
                Expr::App(
                    Box::new(Expr::Const(Name::str("BHK.proof"), vec![])),
                    Box::new(Expr::BVar(1)),
                ),
                prp_ext_arrow(
                    Expr::App(
                        Box::new(Expr::Const(Name::str("BHK.proof"), vec![])),
                        Box::new(Expr::BVar(1)),
                    ),
                    Expr::App(
                        Box::new(Expr::Const(Name::str("BHK.proof"), vec![])),
                        Box::new(Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("And"), vec![])),
                                Box::new(Expr::BVar(3)),
                            )),
                            Box::new(Expr::BVar(2)),
                        )),
                    ),
                ),
            )),
        )),
    )
}
/// Type of `HeytingAlgebra`: a Heyting algebra structure on a type.
/// `HeytingAlgebra : Type → Type 1`
#[allow(dead_code)]
pub fn axiom_heyting_algebra_ty() -> Expr {
    prp_ext_arrow(type1(), Expr::Sort(Level::succ(Level::succ(Level::zero()))))
}
/// Type of `HeytingAlgebra.implication`: the relative pseudo-complement (implication) in a Heyting algebra.
/// `HeytingAlgebra.implication : ∀ {H : Type} \[HeytingAlgebra H\], H → H → H`
#[allow(dead_code)]
pub fn axiom_heyting_implication_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("H"),
        Box::new(type1()),
        Box::new(prp_ext_arrow(
            Expr::App(
                Box::new(Expr::Const(Name::str("HeytingAlgebra"), vec![])),
                Box::new(Expr::BVar(0)),
            ),
            prp_ext_arrow(Expr::BVar(1), prp_ext_arrow(Expr::BVar(1), Expr::BVar(1))),
        )),
    )
}
/// Type of `DeMorgan.classical1`: ¬(p ∧ q) ↔ ¬p ∨ ¬q (classical De Morgan 1).
/// `DeMorgan.classical1 : ∀ p q : Prop, Iff (Not (And p q)) (Or (Not p) (Not q))`
#[allow(dead_code)]
pub fn axiom_de_morgan_classical1_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("p"),
        Box::new(prop()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("q"),
            Box::new(prop()),
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Iff"), vec![])),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Not"), vec![])),
                        Box::new(Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("And"), vec![])),
                                Box::new(Expr::BVar(1)),
                            )),
                            Box::new(Expr::BVar(0)),
                        )),
                    )),
                )),
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Or"), vec![])),
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("Not"), vec![])),
                            Box::new(Expr::BVar(1)),
                        )),
                    )),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Not"), vec![])),
                        Box::new(Expr::BVar(0)),
                    )),
                )),
            )),
        )),
    )
}
/// Type of `DeMorgan.classical2`: ¬(p ∨ q) ↔ ¬p ∧ ¬q (classical De Morgan 2).
/// `DeMorgan.classical2 : ∀ p q : Prop, Iff (Not (Or p q)) (And (Not p) (Not q))`
#[allow(dead_code)]
pub fn axiom_de_morgan_classical2_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("p"),
        Box::new(prop()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("q"),
            Box::new(prop()),
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Iff"), vec![])),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Not"), vec![])),
                        Box::new(Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("Or"), vec![])),
                                Box::new(Expr::BVar(1)),
                            )),
                            Box::new(Expr::BVar(0)),
                        )),
                    )),
                )),
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("And"), vec![])),
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("Not"), vec![])),
                            Box::new(Expr::BVar(1)),
                        )),
                    )),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Not"), vec![])),
                        Box::new(Expr::BVar(0)),
                    )),
                )),
            )),
        )),
    )
}
/// Type of `Godel.gentzenTranslation`: translates classical proofs to intuitionistic proofs
/// via double negation.
/// `Godel.gentzenTranslation : Prop → Prop`
#[allow(dead_code)]
pub fn axiom_godel_gentzen_translation_ty() -> Expr {
    prp_ext_arrow(prop(), prop())
}
/// Type of `Godel.gentzenCorrectness`: the Godel-Gentzen translation is valid.
/// If p is classically provable, then ¬¬p is intuitionistically provable.
/// `Godel.gentzenCorrectness : Prop`
#[allow(dead_code)]
pub fn axiom_godel_gentzen_correctness_ty() -> Expr {
    Expr::Sort(Level::zero())
}
/// Type of `PropLogic.soundness`: truth table semantics is sound for propositional logic.
/// `PropLogic.soundness : Prop`
#[allow(dead_code)]
pub fn axiom_prop_soundness_ty() -> Expr {
    Expr::Sort(Level::zero())
}
/// Type of `PropLogic.consistencyLem`: classical propositional logic is consistent
/// (False is not provable).
/// `PropLogic.consistencyLem : Not False`
#[allow(dead_code)]
pub fn axiom_prop_consistency_ty() -> Expr {
    Expr::App(
        Box::new(Expr::Const(Name::str("Not"), vec![])),
        Box::new(Expr::Const(Name::str("False"), vec![])),
    )
}
/// Type of `IPC.andElimLeft`: p ∧ q → p (intuitionistic and-elimination left).
/// `IPC.andElimLeft : ∀ p q : Prop, And p q → p`
#[allow(dead_code)]
pub fn axiom_ipc_and_elim_left_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("p"),
        Box::new(prop()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("q"),
            Box::new(prop()),
            Box::new(prp_ext_arrow(
                Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("And"), vec![])),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::BVar(0)),
                ),
                Expr::BVar(1),
            )),
        )),
    )
}
/// Type of `IPC.andElimRight`: p ∧ q → q.
/// `IPC.andElimRight : ∀ p q : Prop, And p q → q`
#[allow(dead_code)]
pub fn axiom_ipc_and_elim_right_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("p"),
        Box::new(prop()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("q"),
            Box::new(prop()),
            Box::new(prp_ext_arrow(
                Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("And"), vec![])),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::BVar(0)),
                ),
                Expr::BVar(0),
            )),
        )),
    )
}
/// Type of `IPC.orIntroLeft`: p → p ∨ q.
/// `IPC.orIntroLeft : ∀ p q : Prop, p → Or p q`
#[allow(dead_code)]
pub fn axiom_ipc_or_intro_left_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("p"),
        Box::new(prop()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("q"),
            Box::new(prop()),
            Box::new(prp_ext_arrow(
                Expr::BVar(1),
                Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Or"), vec![])),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::BVar(0)),
                ),
            )),
        )),
    )
}
/// Type of `IPC.orIntroRight`: q → p ∨ q.
/// `IPC.orIntroRight : ∀ p q : Prop, q → Or p q`
#[allow(dead_code)]
pub fn axiom_ipc_or_intro_right_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("p"),
        Box::new(prop()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("q"),
            Box::new(prop()),
            Box::new(prp_ext_arrow(
                Expr::BVar(0),
                Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Or"), vec![])),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::BVar(0)),
                ),
            )),
        )),
    )
}
/// Type of `IPC.modusPonens`: p → (p → q) → q.
/// `IPC.modusPonens : ∀ p q : Prop, p → (p → q) → q`
#[allow(dead_code)]
pub fn axiom_modus_ponens_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("p"),
        Box::new(prop()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("q"),
            Box::new(prop()),
            Box::new(prp_ext_arrow(
                Expr::BVar(1),
                prp_ext_arrow(prp_ext_arrow(Expr::BVar(1), Expr::BVar(0)), Expr::BVar(0)),
            )),
        )),
    )
}
