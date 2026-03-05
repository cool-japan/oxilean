//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

#[cfg(test)]
mod tests {
    use super::*;
    fn nat_ty() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn setup_env() -> Environment {
        let mut env = Environment::new();
        for name in &[
            "Eq",
            "HEq",
            "Not",
            "False",
            "Sigma.Exists",
            "Sigma",
            "Subtype",
            "Sigma.mk",
            "Sigma.fst",
            "Sigma.snd",
            "Subtype.mk",
            "Subtype.val",
        ] {
            let _ = env.add(Declaration::Axiom {
                name: Name::str(*name),
                univ_params: vec![],
                ty: prop(),
            });
        }
        env
    }
    #[test]
    fn test_build_sigma_env() {
        let mut env = Environment::new();
        assert!(build_sigma_env(&mut env).is_ok());
        let expected = [
            "Sigma",
            "Sigma.mk",
            "Sigma.fst",
            "Sigma.snd",
            "PSigma",
            "PSigma.mk",
            "PSigma.fst",
            "PSigma.snd",
            "Subtype",
            "Subtype.mk",
            "Subtype.val",
            "Subtype.property",
            "Sigma.Exists",
            "Sigma.Exists.intro",
            "Sigma.Exists.elim",
            "SetOf",
            "SetOf.mem",
            "PLift",
            "PLift.up",
            "PLift.down",
            "ULift",
            "ULift.up",
            "ULift.down",
            "Sigma.eta",
            "Subtype.eq",
            "Sigma.Exists.imp",
            "Sigma.Exists.not_forall",
            "Sigma.casesOn",
            "Subtype.casesOn",
            "PLift.casesOn",
            "Sigma.ext",
        ];
        for name in &expected {
            assert!(env.contains(&Name::str(*name)), "missing: {}", name);
        }
    }
    #[test]
    fn test_mk_sigma() {
        let alpha = nat_ty();
        let beta = cst("my_beta");
        let e = mk_sigma(alpha, beta);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_sigma_mk() {
        let e = mk_sigma_mk(cst("a"), cst("b"));
        if let Expr::App(f, _) = &e {
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Sigma.mk"));
                } else {
                    panic!("expected Const");
                }
            } else {
                panic!("expected nested App");
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_sigma_fst() {
        let e = mk_sigma_fst(cst("s"));
        if let Expr::App(f, a) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Sigma.fst"));
            }
            if let Expr::Const(n, _) = a.as_ref() {
                assert_eq!(*n, Name::str("s"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_sigma_snd() {
        let e = mk_sigma_snd(cst("s"));
        if let Expr::App(f, _) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Sigma.snd"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_psigma() {
        let e = mk_psigma(nat_ty(), cst("beta"));
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_psigma_mk() {
        let e = mk_psigma_mk(cst("x"), cst("y"));
        if let Expr::App(f, _) = &e {
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("PSigma.mk"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_psigma_fst() {
        let e = mk_psigma_fst(cst("p"));
        if let Expr::App(f, _) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("PSigma.fst"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_subtype() {
        let e = mk_subtype(nat_ty(), cst("is_positive"));
        if let Expr::App(f, p) = &e {
            if let Expr::Const(n, _) = p.as_ref() {
                assert_eq!(*n, Name::str("is_positive"));
            }
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Subtype"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_subtype_mk() {
        let e = mk_subtype_mk(cst("v"), cst("prf"));
        if let Expr::App(f, _) = &e {
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Subtype.mk"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_subtype_val() {
        let e = mk_subtype_val(cst("s"));
        if let Expr::App(f, _) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Subtype.val"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_exists() {
        let e = mk_exists(nat_ty(), cst("is_even"));
        if let Expr::App(f, _) = &e {
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Sigma.Exists"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_exists_intro() {
        let e = mk_exists_intro(cst("w"), cst("pf"));
        if let Expr::App(f, _) = &e {
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Sigma.Exists.intro"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_exists_elim() {
        let e = mk_exists_elim(cst("hex"), cst("f"));
        if let Expr::App(f, _) = &e {
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Sigma.Exists.elim"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_setof() {
        let e = mk_setof(nat_ty(), cst("pred"));
        if let Expr::App(f, _) = &e {
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("SetOf"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_plift() {
        let e = mk_plift(cst("P"));
        if let Expr::App(f, a) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("PLift"));
            }
            if let Expr::Const(n, _) = a.as_ref() {
                assert_eq!(*n, Name::str("P"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_plift_up() {
        let e = mk_plift_up(cst("pf"));
        if let Expr::App(f, _) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("PLift.up"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_plift_down() {
        let e = mk_plift_down(cst("h"));
        if let Expr::App(f, _) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("PLift.down"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_ulift() {
        let e = mk_ulift(nat_ty());
        if let Expr::App(f, _) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("ULift"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_ulift_up() {
        let e = mk_ulift_up(cst("a"));
        if let Expr::App(f, _) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("ULift.up"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_ulift_down() {
        let e = mk_ulift_down(cst("a"));
        if let Expr::App(f, _) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("ULift.down"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_sigma_mk_structure() {
        let e = mk_sigma_mk(cst("x"), cst("y"));
        if let Expr::App(f, rhs) = &e {
            if let Expr::Const(n, _) = rhs.as_ref() {
                assert_eq!(*n, Name::str("y"));
            }
            if let Expr::App(g, lhs) = f.as_ref() {
                if let Expr::Const(n, _) = lhs.as_ref() {
                    assert_eq!(*n, Name::str("x"));
                }
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Sigma.mk"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_exists_intro_structure() {
        let e = mk_exists_intro(cst("w"), cst("pf"));
        if let Expr::App(f, pf) = &e {
            if let Expr::Const(n, _) = pf.as_ref() {
                assert_eq!(*n, Name::str("pf"));
            }
            if let Expr::App(g, w) = f.as_ref() {
                if let Expr::Const(n, _) = w.as_ref() {
                    assert_eq!(*n, Name::str("w"));
                }
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Sigma.Exists.intro"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_subtype_mk_structure() {
        let e = mk_subtype_mk(cst("val"), cst("proof"));
        if let Expr::App(f, prf) = &e {
            if let Expr::Const(n, _) = prf.as_ref() {
                assert_eq!(*n, Name::str("proof"));
            }
            if let Expr::App(g, v) = f.as_ref() {
                if let Expr::Const(n, _) = v.as_ref() {
                    assert_eq!(*n, Name::str("val"));
                }
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Subtype.mk"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_env_contains_all_sigma_decls() {
        let mut env = Environment::new();
        build_sigma_env(&mut env).expect("build_sigma_env should succeed");
        assert!(env.contains(&Name::str("Sigma")));
        assert!(env.contains(&Name::str("Sigma.mk")));
        assert!(env.contains(&Name::str("Sigma.fst")));
        assert!(env.contains(&Name::str("Sigma.snd")));
        assert!(env.contains(&Name::str("Sigma.eta")));
        assert!(env.contains(&Name::str("Sigma.casesOn")));
        assert!(env.contains(&Name::str("Sigma.ext")));
    }
    #[test]
    fn test_env_contains_all_subtype_decls() {
        let mut env = Environment::new();
        build_sigma_env(&mut env).expect("build_sigma_env should succeed");
        assert!(env.contains(&Name::str("Subtype")));
        assert!(env.contains(&Name::str("Subtype.mk")));
        assert!(env.contains(&Name::str("Subtype.val")));
        assert!(env.contains(&Name::str("Subtype.property")));
        assert!(env.contains(&Name::str("Subtype.eq")));
        assert!(env.contains(&Name::str("Subtype.casesOn")));
    }
    #[test]
    fn test_env_contains_all_exists_decls() {
        let mut env = Environment::new();
        build_sigma_env(&mut env).expect("build_sigma_env should succeed");
        assert!(env.contains(&Name::str("Sigma.Exists")));
        assert!(env.contains(&Name::str("Sigma.Exists.intro")));
        assert!(env.contains(&Name::str("Sigma.Exists.elim")));
        assert!(env.contains(&Name::str("Sigma.Exists.imp")));
        assert!(env.contains(&Name::str("Sigma.Exists.not_forall")));
    }
    #[test]
    fn test_env_contains_all_plift_ulift() {
        let mut env = Environment::new();
        build_sigma_env(&mut env).expect("build_sigma_env should succeed");
        assert!(env.contains(&Name::str("PLift")));
        assert!(env.contains(&Name::str("PLift.up")));
        assert!(env.contains(&Name::str("PLift.down")));
        assert!(env.contains(&Name::str("PLift.casesOn")));
        assert!(env.contains(&Name::str("ULift")));
        assert!(env.contains(&Name::str("ULift.up")));
        assert!(env.contains(&Name::str("ULift.down")));
    }
    #[test]
    fn test_sigma_type_is_axiom() {
        let mut env = Environment::new();
        build_sigma_env(&mut env).expect("build_sigma_env should succeed");
        let decl = env
            .get(&Name::str("Sigma"))
            .expect("declaration 'Sigma' should exist in env");
        assert!(matches!(decl, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_subtype_type_is_axiom() {
        let mut env = Environment::new();
        build_sigma_env(&mut env).expect("build_sigma_env should succeed");
        let decl = env
            .get(&Name::str("Subtype"))
            .expect("declaration 'Subtype' should exist in env");
        assert!(matches!(decl, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_plift_type_is_axiom() {
        let mut env = Environment::new();
        build_sigma_env(&mut env).expect("build_sigma_env should succeed");
        let decl = env
            .get(&Name::str("PLift"))
            .expect("declaration 'PLift' should exist in env");
        assert!(matches!(decl, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_setup_env_helper() {
        let env = setup_env();
        assert!(env.contains(&Name::str("Eq")));
        assert!(env.contains(&Name::str("HEq")));
    }
}
