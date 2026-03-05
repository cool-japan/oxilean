//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::functions::*;
use super::types::*;

#[cfg(test)]
mod tests {
    use super::*;
    fn setup_env() -> Environment {
        Environment::new()
    }
    #[test]
    fn test_build_wellfounded_env() {
        let mut env = setup_env();
        assert!(build_wellfounded_env(&mut env).is_ok());
        for name in &[
            "WellFounded",
            "Acc",
            "Acc.intro",
            "Acc.rec",
            "WellFounded.intro",
            "WellFounded.apply",
            "WellFounded.fix",
            "Measure",
            "InvImage",
            "Prod.Lex",
            "SizeOf",
            "sizeOf",
            "Nat.lt",
            "Nat.lt_wfRel",
            "measure_wf",
            "invImage_wf",
            "prod_lex_wf",
            "Decreasing",
            "PSigma",
            "PSigma.mk",
            "PSigma.fst",
            "PSigma.snd",
            "Acc.rec_on",
            "WellFounded.fix_eq",
            "measure_lt",
            "Nat.lt_wf_aux",
            "sizeOf_nat",
            "sizeOf_prod",
        ] {
            assert!(env.contains(&Name::str(*name)), "missing: {}", name);
        }
    }
    #[test]
    fn test_mk_wellfounded() {
        let rel = Expr::Const(Name::str("Nat.lt"), vec![]);
        let wf = mk_wellfounded(rel);
        match wf {
            Expr::App(ref f, _) => {
                assert_eq!(**f, Expr::Const(Name::str("WellFounded"), vec![]));
            }
            _ => panic!("Expected App"),
        }
    }
    #[test]
    fn test_mk_acc() {
        let rel = Expr::Const(Name::str("Nat.lt"), vec![]);
        let x = Expr::Const(Name::str("n"), vec![]);
        let acc = mk_acc(rel, x);
        match acc {
            Expr::App(ref f, _) => {
                assert!(matches!(**f, Expr::App(_, _)));
            }
            _ => panic!("Expected App"),
        }
    }
    #[test]
    fn test_mk_acc_intro() {
        let x = Expr::Const(Name::str("n"), vec![]);
        let h = Expr::Const(Name::str("proof"), vec![]);
        let intro = mk_acc_intro(x, h);
        match intro {
            Expr::App(ref f, _) => {
                assert!(matches!(**f, Expr::App(_, _)));
            }
            _ => panic!("Expected App"),
        }
    }
    #[test]
    fn test_mk_wf_fix() {
        let wf = Expr::Const(Name::str("wf"), vec![]);
        let f = Expr::Const(Name::str("F"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let fix = mk_wf_fix(wf, f, a);
        let mut depth = 0;
        let mut cur = &fix;
        while let Expr::App(inner, _) = cur {
            depth += 1;
            cur = inner;
        }
        assert_eq!(depth, 3);
    }
    #[test]
    fn test_mk_measure() {
        let f = Expr::Const(Name::str("myMeasure"), vec![]);
        let m = mk_measure(f);
        match m {
            Expr::App(ref func, _) => {
                assert_eq!(**func, Expr::Const(Name::str("Measure"), vec![]));
            }
            _ => panic!("Expected App"),
        }
    }
    #[test]
    fn test_mk_inv_image() {
        let r = Expr::Const(Name::str("Nat.lt"), vec![]);
        let f = Expr::Const(Name::str("myFunc"), vec![]);
        let inv = mk_inv_image(r, f);
        match inv {
            Expr::App(ref inner, _) => {
                assert!(matches!(**inner, Expr::App(_, _)));
            }
            _ => panic!("Expected App"),
        }
    }
    #[test]
    fn test_mk_prod_lex() {
        let ra = Expr::Const(Name::str("ra"), vec![]);
        let rb = Expr::Const(Name::str("rb"), vec![]);
        let lex = mk_prod_lex(ra, rb);
        match lex {
            Expr::App(ref inner, _) => {
                assert!(matches!(**inner, Expr::App(_, _)));
            }
            _ => panic!("Expected App"),
        }
    }
    #[test]
    fn test_mk_sizeof() {
        let ty = Expr::Const(Name::str("Nat"), vec![]);
        let a = Expr::Const(Name::str("n"), vec![]);
        let sz = mk_sizeof(ty, a);
        match sz {
            Expr::App(ref inner, _) => {
                assert!(matches!(**inner, Expr::App(_, _)));
            }
            _ => panic!("Expected App"),
        }
    }
    #[test]
    fn test_mk_psigma() {
        let alpha = Expr::Const(Name::str("A"), vec![]);
        let beta = Expr::Const(Name::str("B"), vec![]);
        let ps = mk_psigma(alpha, beta);
        match ps {
            Expr::App(ref inner, _) => {
                assert!(matches!(**inner, Expr::App(_, _)));
            }
            _ => panic!("Expected App"),
        }
    }
    #[test]
    fn test_mk_psigma_mk() {
        let fst = Expr::Const(Name::str("a"), vec![]);
        let snd = Expr::Const(Name::str("b"), vec![]);
        let mk = mk_psigma_mk(fst, snd);
        match mk {
            Expr::App(ref inner, _) => {
                assert!(matches!(**inner, Expr::App(_, _)));
            }
            _ => panic!("Expected App"),
        }
    }
    #[test]
    fn test_mk_decreasing() {
        let rel = Expr::Const(Name::str("r"), vec![]);
        let x = Expr::Const(Name::str("x"), vec![]);
        let y = Expr::Const(Name::str("y"), vec![]);
        let dec = mk_decreasing(rel, x, y);
        let mut depth = 0;
        let mut cur = &dec;
        while let Expr::App(inner, _) = cur {
            depth += 1;
            cur = inner;
        }
        assert_eq!(depth, 3);
    }
    #[test]
    fn test_mk_wellfounded_structure() {
        let rel = Expr::Const(Name::str("R"), vec![]);
        let wf = mk_wellfounded(rel.clone());
        assert_eq!(
            wf,
            Expr::App(
                Box::new(Expr::Const(Name::str("WellFounded"), vec![])),
                Box::new(rel),
            )
        );
    }
    #[test]
    fn test_mk_acc_structure() {
        let rel = Expr::Const(Name::str("R"), vec![]);
        let x = Expr::Const(Name::str("x"), vec![]);
        let acc = mk_acc(rel.clone(), x.clone());
        assert_eq!(
            acc,
            Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Acc"), vec![])),
                    Box::new(rel),
                )),
                Box::new(x),
            )
        );
    }
    #[test]
    fn test_mk_acc_intro_structure() {
        let x = Expr::Const(Name::str("x"), vec![]);
        let h = Expr::Const(Name::str("h"), vec![]);
        let intro = mk_acc_intro(x.clone(), h.clone());
        assert_eq!(
            intro,
            Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Acc.intro"), vec![])),
                    Box::new(x),
                )),
                Box::new(h),
            )
        );
    }
    #[test]
    fn test_helpers_smoke() {
        let r = Expr::Const(Name::str("R"), vec![]);
        let x = Expr::Const(Name::str("x"), vec![]);
        let y = Expr::Const(Name::str("y"), vec![]);
        let f = Expr::Const(Name::str("f"), vec![]);
        let wf = Expr::Const(Name::str("wf"), vec![]);
        let _ = mk_wellfounded(r.clone());
        let _ = mk_acc(r.clone(), x.clone());
        let _ = mk_acc_intro(x.clone(), y.clone());
        let _ = mk_wf_fix(wf, f.clone(), x.clone());
        let _ = mk_measure(f.clone());
        let _ = mk_inv_image(r.clone(), f);
        let _ = mk_prod_lex(r.clone(), r);
        let _ = mk_sizeof(x.clone(), y.clone());
        let _ = mk_psigma(x.clone(), y.clone());
        let _ = mk_psigma_mk(x.clone(), y.clone());
        let _ = mk_decreasing(Expr::Const(Name::str("rel"), vec![]), x, y);
    }
    #[test]
    fn test_prereqs_added() {
        let mut env = setup_env();
        build_wellfounded_env(&mut env).expect("build_wellfounded_env should succeed");
        assert!(env.contains(&Name::str("Nat")));
        assert!(env.contains(&Name::str("Eq")));
        assert!(env.contains(&Name::str("sorry")));
    }
    #[test]
    fn test_nat_lt_present() {
        let mut env = setup_env();
        build_wellfounded_env(&mut env).expect("build_wellfounded_env should succeed");
        assert!(env.contains(&Name::str("Nat.lt")));
    }
    #[test]
    fn test_mk_wf_fix_structure() {
        let wf = Expr::Const(Name::str("wf"), vec![]);
        let f = Expr::Const(Name::str("F"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let fix = mk_wf_fix(wf.clone(), f.clone(), a.clone());
        assert_eq!(
            fix,
            Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("WellFounded.fix"), vec![])),
                        Box::new(wf),
                    )),
                    Box::new(f),
                )),
                Box::new(a),
            )
        );
    }
    #[test]
    fn test_mk_measure_structure() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let m = mk_measure(f.clone());
        assert_eq!(
            m,
            Expr::App(
                Box::new(Expr::Const(Name::str("Measure"), vec![])),
                Box::new(f),
            )
        );
    }
    #[test]
    fn test_mk_inv_image_structure() {
        let r = Expr::Const(Name::str("r"), vec![]);
        let f = Expr::Const(Name::str("f"), vec![]);
        let inv = mk_inv_image(r.clone(), f.clone());
        assert_eq!(
            inv,
            Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("InvImage"), vec![])),
                    Box::new(r),
                )),
                Box::new(f),
            )
        );
    }
    #[test]
    fn test_mk_prod_lex_structure() {
        let ra = Expr::Const(Name::str("ra"), vec![]);
        let rb = Expr::Const(Name::str("rb"), vec![]);
        let lex = mk_prod_lex(ra.clone(), rb.clone());
        assert_eq!(
            lex,
            Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Prod.Lex"), vec![])),
                    Box::new(ra),
                )),
                Box::new(rb),
            )
        );
    }
    #[test]
    fn test_mk_sizeof_structure() {
        let ty = Expr::Const(Name::str("T"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let sz = mk_sizeof(ty.clone(), a.clone());
        assert_eq!(
            sz,
            Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("sizeOf"), vec![])),
                    Box::new(ty),
                )),
                Box::new(a),
            )
        );
    }
    #[test]
    fn test_mk_psigma_structure() {
        let alpha = Expr::Const(Name::str("A"), vec![]);
        let beta = Expr::Const(Name::str("B"), vec![]);
        let ps = mk_psigma(alpha.clone(), beta.clone());
        assert_eq!(
            ps,
            Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("PSigma"), vec![])),
                    Box::new(alpha),
                )),
                Box::new(beta),
            )
        );
    }
    #[test]
    fn test_mk_psigma_mk_structure() {
        let fst = Expr::Const(Name::str("a"), vec![]);
        let snd = Expr::Const(Name::str("b"), vec![]);
        let mk = mk_psigma_mk(fst.clone(), snd.clone());
        assert_eq!(
            mk,
            Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("PSigma.mk"), vec![])),
                    Box::new(fst),
                )),
                Box::new(snd),
            )
        );
    }
    #[test]
    fn test_mk_decreasing_structure() {
        let rel = Expr::Const(Name::str("R"), vec![]);
        let x = Expr::Const(Name::str("x"), vec![]);
        let y = Expr::Const(Name::str("y"), vec![]);
        let dec = mk_decreasing(rel.clone(), x.clone(), y.clone());
        assert_eq!(
            dec,
            Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Decreasing"), vec![])),
                        Box::new(rel),
                    )),
                    Box::new(x),
                )),
                Box::new(y),
            )
        );
    }
}
/// Register ordinal arithmetic axioms in the environment.
///
/// Covers: ordinal addition, multiplication, exponentiation,
/// the Cantor normal form representation, and limit ordinals.
#[allow(dead_code)]
pub fn add_ordinal_arithmetic(env: &mut Environment) {
    let prop = || Expr::Sort(Level::zero());
    let type1 = || Expr::Sort(Level::succ(Level::zero()));
    let ord_ty = || Expr::Const(Name::str("Ordinal"), vec![]);
    let nat_ty = || Expr::Const(Name::str("Nat"), vec![]);
    let arr = |a: Expr, b: Expr| arrow(a, b);
    let c = |s: &str| Expr::Const(Name::str(s), vec![]);
    let ord_decl = Declaration::Axiom {
        name: Name::str("Ordinal"),
        univ_params: vec![],
        ty: type1(),
    };
    let _ = env.add(ord_decl);
    let succ_decl = Declaration::Axiom {
        name: Name::str("Ordinal.succ"),
        univ_params: vec![],
        ty: arr(ord_ty(), ord_ty()),
    };
    let _ = env.add(succ_decl);
    let limit_decl = Declaration::Axiom {
        name: Name::str("Ordinal.isLimit"),
        univ_params: vec![],
        ty: arr(ord_ty(), prop()),
    };
    let _ = env.add(limit_decl);
    let zero_decl = Declaration::Axiom {
        name: Name::str("Ordinal.zero"),
        univ_params: vec![],
        ty: ord_ty(),
    };
    let _ = env.add(zero_decl);
    let omega_decl = Declaration::Axiom {
        name: Name::str("Ordinal.omega"),
        univ_params: vec![],
        ty: ord_ty(),
    };
    let _ = env.add(omega_decl);
    let add_decl = Declaration::Axiom {
        name: Name::str("Ordinal.add"),
        univ_params: vec![],
        ty: arr(ord_ty(), arr(ord_ty(), ord_ty())),
    };
    let _ = env.add(add_decl);
    let mul_decl = Declaration::Axiom {
        name: Name::str("Ordinal.mul"),
        univ_params: vec![],
        ty: arr(ord_ty(), arr(ord_ty(), ord_ty())),
    };
    let _ = env.add(mul_decl);
    let exp_decl = Declaration::Axiom {
        name: Name::str("Ordinal.pow"),
        univ_params: vec![],
        ty: arr(ord_ty(), arr(ord_ty(), ord_ty())),
    };
    let _ = env.add(exp_decl);
    let lt_decl = Declaration::Axiom {
        name: Name::str("Ordinal.lt"),
        univ_params: vec![],
        ty: arr(ord_ty(), arr(ord_ty(), prop())),
    };
    let _ = env.add(lt_decl);
    let le_decl = Declaration::Axiom {
        name: Name::str("Ordinal.le"),
        univ_params: vec![],
        ty: arr(ord_ty(), arr(ord_ty(), prop())),
    };
    let _ = env.add(le_decl);
    let embed_decl = Declaration::Axiom {
        name: Name::str("Ordinal.ofNat"),
        univ_params: vec![],
        ty: arr(nat_ty(), ord_ty()),
    };
    let _ = env.add(embed_decl);
    let cnf_decl = Declaration::Axiom {
        name: Name::str("Ordinal.cantorNF"),
        univ_params: vec![],
        ty: arr(
            ord_ty(),
            Expr::App(
                Box::new(Expr::Const(Name::str("List"), vec![])),
                Box::new(Expr::App(
                    Box::new(Expr::App(Box::new(c("Prod")), Box::new(ord_ty()))),
                    Box::new(nat_ty()),
                )),
            ),
        ),
    };
    let _ = env.add(cnf_decl);
    let tfi_decl = Declaration::Axiom {
        name: Name::str("Ordinal.transfiniteInduction"),
        univ_params: vec![Name::str("u")],
        ty: {
            let sort_u = Expr::Sort(Level::Param(Name::str("u")));
            let p_ty = arrow(ord_ty(), sort_u.clone());
            Expr::Pi(
                BinderInfo::Implicit,
                Name::str("P"),
                Box::new(p_ty.clone()),
                Box::new(arrow(
                    Expr::Pi(
                        BinderInfo::Default,
                        Name::Anonymous,
                        Box::new(ord_ty()),
                        Box::new(arrow(
                            Expr::Pi(
                                BinderInfo::Default,
                                Name::Anonymous,
                                Box::new(ord_ty()),
                                Box::new(arrow(
                                    Expr::App(
                                        Box::new(Expr::App(
                                            Box::new(Expr::Const(Name::str("Ordinal.lt"), vec![])),
                                            Box::new(Expr::BVar(0)),
                                        )),
                                        Box::new(Expr::BVar(1)),
                                    ),
                                    Expr::App(Box::new(Expr::BVar(3)), Box::new(Expr::BVar(1))),
                                )),
                            ),
                            Expr::App(Box::new(Expr::BVar(1)), Box::new(Expr::BVar(0))),
                        )),
                    ),
                    Expr::Pi(
                        BinderInfo::Default,
                        Name::Anonymous,
                        Box::new(ord_ty()),
                        Box::new(Expr::App(Box::new(Expr::BVar(1)), Box::new(Expr::BVar(0)))),
                    ),
                )),
            )
        },
    };
    let _ = env.add(tfi_decl);
    for (nm, stmt) in [
        (
            "ordinal_add_zero",
            "forall (alpha : Ordinal), Eq (Ordinal.add alpha Ordinal.zero) alpha",
        ),
        (
            "ordinal_zero_add",
            "forall (alpha : Ordinal), Eq (Ordinal.add Ordinal.zero alpha) alpha",
        ),
        (
            "ordinal_add_succ",
            "forall (alpha beta : Ordinal), Eq (Ordinal.add alpha (Ordinal.succ beta)) (Ordinal.succ (Ordinal.add alpha beta))",
        ),
        (
            "ordinal_mul_zero",
            "forall (alpha : Ordinal), Eq (Ordinal.mul alpha Ordinal.zero) Ordinal.zero",
        ),
        (
            "ordinal_mul_one",
            "forall (alpha : Ordinal), Eq (Ordinal.mul alpha (Ordinal.succ Ordinal.zero)) alpha",
        ),
        (
            "ordinal_pow_zero",
            "forall (alpha : Ordinal), Eq (Ordinal.pow alpha Ordinal.zero) (Ordinal.succ Ordinal.zero)",
        ),
        ("ordinal_well_order", "WellFounded Ordinal.lt"),
        (
            "ordinal_add_lt_mono",
            "forall (alpha beta gamma : Ordinal), Ordinal.lt beta gamma -> Ordinal.lt (Ordinal.add alpha beta) (Ordinal.add alpha gamma)",
        ),
        ("ordinal_omega_limit", "Ordinal.isLimit Ordinal.omega"),
        (
            "cantor_normal_form",
            "forall (alpha : Ordinal), Ordinal.lt Ordinal.zero alpha -> is_cnf (Ordinal.cantorNF alpha)",
        ),
    ] {
        let decl = Declaration::Axiom {
            name: Name::str(nm),
            univ_params: vec![],
            ty: Expr::Const(Name::str(stmt), vec![]),
        };
        let _ = env.add(decl);
    }
}
/// Register well-foundedness theorems and combinators in an environment.
#[allow(dead_code)]
pub fn add_wf_combinators(env: &mut Environment) {
    use oxilean_kernel::ReducibilityHint;
    let prop = || Expr::Sort(Level::zero());
    let type1 = || Expr::Sort(Level::succ(Level::zero()));
    let sorry = || Expr::Const(Name::str("sorry"), vec![]);
    for (nm, ty_str) in [
        ("InvImage", "type"),
        ("Prod.Lex", "type"),
        ("Nat.lt.wf", "WellFounded Nat.lt"),
        ("measure.wf", "forall (f : alpha -> Nat), WellFounded (InvImage Nat.lt f)"),
        (
            "invImage.wf",
            "forall (r : alpha -> alpha -> Prop) (f : beta -> alpha), WellFounded r -> WellFounded (InvImage r f)",
        ),
        (
            "prod.lex.wf",
            "forall (r : alpha -> alpha -> Prop) (s : beta -> beta -> Prop), WellFounded r -> WellFounded s -> WellFounded (Prod.Lex r s)",
        ),
        (
            "Subrel.wf",
            "forall (r s : alpha -> alpha -> Prop), Subrel r s -> WellFounded s -> WellFounded r",
        ),
        ("empty_wf", "WellFounded (fun (x : Empty) _ => False.elim x)"),
        (
            "sigma.lex.wf",
            "forall (r : alpha -> alpha -> Prop) (s : forall x : alpha, beta x -> beta x -> Prop), WellFounded r -> (forall x, WellFounded (s x)) -> WellFounded (Sigma.Lex r s)",
        ),
    ] {
        let ty = if ty_str == "type" {
            type1()
        } else {
            Expr::Const(Name::str(ty_str), vec![])
        };
        let _ = env
            .add(Declaration::Definition {
                name: Name::str(nm),
                univ_params: vec![],
                ty: ty,
                val: sorry(),
                hint: ReducibilityHint::Opaque,
            });
        let _ = prop();
    }
}
/// Register accessibility (`Acc`) induction lemmas.
#[allow(dead_code)]
pub fn add_acc_lemmas(env: &mut Environment) {
    use oxilean_kernel::ReducibilityHint;
    let sorry = || Expr::Const(Name::str("sorry"), vec![]);
    for nm in [
        "Acc.inv",
        "Acc.ndrecOn",
        "Acc.recOn",
        "Acc.motive_correct",
        "WellFounded.apply",
        "WellFounded.fixF",
        "WellFounded.fix",
        "WellFounded.induction",
        "WellFounded.recursion",
        "WellFounded.min",
    ] {
        let _ = env.add(Declaration::Definition {
            name: Name::str(nm),
            univ_params: vec![],
            ty: Expr::Const(Name::str("Prop"), vec![]),
            val: sorry(),
            hint: ReducibilityHint::Opaque,
        });
    }
}
#[cfg(test)]
mod wf_extended_tests {
    use super::*;
    #[test]
    fn test_termination_cert_structural() {
        let cert = TerminationCert::structural("fibonacci");
        assert!(cert.is_valid());
        assert!(cert.verified);
        assert!(cert.fn_name.contains("fibonacci"));
    }
    #[test]
    fn test_termination_cert_well_founded() {
        let cert = TerminationCert::well_founded("ackermann", "Nat.add m n");
        assert!(cert.is_valid());
        assert_eq!(cert.measure_fn, "Nat.add m n");
    }
    #[test]
    fn test_termination_cert_lex() {
        let cert =
            TerminationCert::lexicographic("ackermann2", vec!["m".to_owned(), "n".to_owned()]);
        assert!(cert.is_valid());
        assert!(cert.justification.contains("Lexicographic"));
    }
    #[test]
    fn test_termination_registry() {
        let mut reg = TerminationRegistry::new();
        reg.register(TerminationCert::structural("foo"));
        reg.register(TerminationCert::well_founded("bar", "size"));
        assert_eq!(reg.count(), 2);
        assert!(reg.lookup("foo").is_some());
        assert!(reg.lookup("baz").is_none());
        assert_eq!(reg.verified_certs().len(), 2);
    }
    #[test]
    fn test_lex_order_check_decrease() {
        let lex = LexOrder::new(vec!["m", "n"]);
        assert!(lex.check_decrease(&[3, 0], &[2, 999]));
        assert!(!lex.check_decrease(&[2, 5], &[2, 5]));
        assert!(!lex.check_decrease(&[2, 3], &[3, 0]));
        assert!(lex.check_decrease(&[2, 5], &[2, 4]));
    }
    #[test]
    fn test_recursion_tree_depth() {
        let leaf = RecursionTree::Base {
            label: "base".into(),
        };
        assert_eq!(leaf.depth(), 0);
        assert_eq!(leaf.recursive_calls(), 0);
        let rec = RecursionTree::Rec {
            label: "step".into(),
            decreasing_arg: "n".into(),
            child: Box::new(RecursionTree::Base {
                label: "base".into(),
            }),
        };
        assert_eq!(rec.depth(), 1);
        assert_eq!(rec.recursive_calls(), 1);
        let branch = RecursionTree::Branch {
            label: "match".into(),
            arms: vec![
                RecursionTree::Base {
                    label: "nil".into(),
                },
                RecursionTree::Rec {
                    label: "cons".into(),
                    decreasing_arg: "xs".into(),
                    child: Box::new(RecursionTree::Base {
                        label: "base".into(),
                    }),
                },
            ],
        };
        assert_eq!(branch.depth(), 1);
        assert_eq!(branch.recursive_calls(), 1);
    }
    #[test]
    fn test_size_measure() {
        let m = SizeMeasure::list_length();
        assert_eq!(m.name, "List.length");
        assert_eq!(m.domain, "List");
        assert!(m.induced_relation_name().contains("length"));
    }
    #[test]
    fn test_wf_rel_builder() {
        let r = WfRelBuilder::nat_lt();
        let f = Expr::Const(Name::str("List.length"), vec![]);
        let inv = WfRelBuilder::inv_image(r.clone(), f.clone());
        assert!(matches!(inv, Expr::App(_, _)));
        let measure = WfRelBuilder::measure(f);
        assert!(matches!(measure, Expr::App(_, _)));
        let lex = WfRelBuilder::prod_lex(r.clone(), r);
        assert!(matches!(lex, Expr::App(_, _)));
    }
    #[test]
    fn test_add_ordinal_arithmetic() {
        let mut env = Environment::new();
        add_ordinal_arithmetic(&mut env);
        for name in &[
            "Ordinal",
            "Ordinal.zero",
            "Ordinal.succ",
            "Ordinal.omega",
            "Ordinal.add",
            "Ordinal.mul",
            "Ordinal.pow",
            "Ordinal.lt",
            "Ordinal.le",
            "Ordinal.ofNat",
            "Ordinal.cantorNF",
            "Ordinal.isLimit",
        ] {
            assert!(env.get(&Name::str(*name)).is_some(), "missing: {}", name);
        }
    }
    #[test]
    fn test_add_wf_combinators() {
        let mut env = Environment::new();
        add_wf_combinators(&mut env);
        assert!(env.get(&Name::str("InvImage")).is_some());
        assert!(env.get(&Name::str("Prod.Lex")).is_some());
        assert!(env.get(&Name::str("Nat.lt.wf")).is_some());
        assert!(env.get(&Name::str("measure.wf")).is_some());
    }
    #[test]
    fn test_termination_strategy_descriptions() {
        for s in TerminationStrategy::all() {
            assert!(!s.description().is_empty());
        }
    }
}
