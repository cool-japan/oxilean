//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_quotient_types_env() {
        let mut env = Environment::new();
        build_quotient_types_env(&mut env).expect("build_quotient_types_env should succeed");
        for name in ["Setoid", "Setoid.mk", "Setoid.r"] {
            assert!(
                env.contains(&Name::str(name)),
                "environment should contain {name}"
            );
        }
    }
    #[test]
    fn test_setoid_properties_registered() {
        let mut env = Environment::new();
        build_quotient_types_env(&mut env).expect("build_quotient_types_env should succeed");
        for name in ["Setoid.refl", "Setoid.symm", "Setoid.trans"] {
            assert!(
                env.contains(&Name::str(name)),
                "environment should contain {name}"
            );
        }
    }
    #[test]
    fn test_quotient_type_registered() {
        let mut env = Environment::new();
        build_quotient_types_env(&mut env).expect("build_quotient_types_env should succeed");
        for name in ["Quotient", "Quotient.mk", "Quotient.lift", "Quotient.ind"] {
            assert!(
                env.contains(&Name::str(name)),
                "environment should contain {name}"
            );
        }
    }
    #[test]
    fn test_quotient_soundness_registered() {
        let mut env = Environment::new();
        build_quotient_types_env(&mut env).expect("build_quotient_types_env should succeed");
        for name in ["Quotient.sound", "Quotient.exact"] {
            assert!(
                env.contains(&Name::str(name)),
                "environment should contain {name}"
            );
        }
    }
    #[test]
    fn test_quotient_operations_registered() {
        let mut env = Environment::new();
        build_quotient_types_env(&mut env).expect("build_quotient_types_env should succeed");
        for name in [
            "Quotient.map",
            "Quotient.map2",
            "Quotient.liftOn",
            "Quotient.rec",
        ] {
            assert!(
                env.contains(&Name::str(name)),
                "environment should contain {name}"
            );
        }
    }
    #[test]
    fn test_has_equiv_registered() {
        let mut env = Environment::new();
        build_quotient_types_env(&mut env).expect("build_quotient_types_env should succeed");
        for name in ["HasEquiv", "HasEquiv.equiv", "instHasEquivOfSetoid"] {
            assert!(
                env.contains(&Name::str(name)),
                "environment should contain {name}"
            );
        }
    }
    #[test]
    fn test_quot_registered() {
        let mut env = Environment::new();
        build_quotient_types_env(&mut env).expect("build_quotient_types_env should succeed");
        for name in ["Quot", "Quot.mk", "Quot.lift", "Quot.ind", "Quot.sound"] {
            assert!(
                env.contains(&Name::str(name)),
                "environment should contain {name}"
            );
        }
    }
    #[test]
    fn test_theorems_registered() {
        let mut env = Environment::new();
        build_quotient_types_env(&mut env).expect("build_quotient_types_env should succeed");
        for name in [
            "Quotient.lift_mk",
            "Quotient.map_mk",
            "Quotient.ind_mk",
            "Quotient.sound_iff",
            "Quot.lift_mk",
        ] {
            assert!(
                env.contains(&Name::str(name)),
                "environment should contain {name}"
            );
        }
    }
    #[test]
    fn test_all_declarations_registered() {
        let mut env = Environment::new();
        build_quotient_types_env(&mut env).expect("build_quotient_types_env should succeed");
        let all_names = [
            "Setoid",
            "Setoid.mk",
            "Setoid.r",
            "Setoid.refl",
            "Setoid.symm",
            "Setoid.trans",
            "Quotient",
            "Quotient.mk",
            "Quotient.lift",
            "Quotient.ind",
            "Quotient.sound",
            "Quotient.exact",
            "Quotient.map",
            "Quotient.map2",
            "Quotient.liftOn",
            "Quotient.rec",
            "HasEquiv",
            "HasEquiv.equiv",
            "instHasEquivOfSetoid",
            "Quot",
            "Quot.mk",
            "Quot.lift",
            "Quot.ind",
            "Quot.sound",
            "Quotient.lift_mk",
            "Quotient.map_mk",
            "Quotient.ind_mk",
            "Quotient.sound_iff",
            "Quot.lift_mk",
        ];
        for name in all_names {
            assert!(
                env.contains(&Name::str(name)),
                "environment should contain {name}"
            );
        }
    }
    #[test]
    fn test_mk_setoid_structure() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let e = mk_setoid(nat);
        if let Expr::App(f, arg) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Setoid"));
            }
            if let Expr::Const(n, _) = arg.as_ref() {
                assert_eq!(*n, Name::str("Nat"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_quotient_structure() {
        let e = mk_quotient();
        if let Expr::Const(n, _) = &e {
            assert_eq!(*n, Name::str("Quotient"));
        } else {
            panic!("expected Const");
        }
    }
    #[test]
    fn test_mk_quotient_mk_structure() {
        let val = cst("x");
        let e = mk_quotient_mk(val);
        if let Expr::App(f, arg) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Quotient.mk"));
            }
            if let Expr::Const(n, _) = arg.as_ref() {
                assert_eq!(*n, Name::str("x"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_quotient_lift_structure() {
        let f = cst("my_func");
        let h = cst("my_proof");
        let q = cst("my_quot");
        let e = mk_quotient_lift(f, h, q);
        if let Expr::App(outer, rhs) = &e {
            if let Expr::Const(n, _) = rhs.as_ref() {
                assert_eq!(*n, Name::str("my_quot"));
            }
            if let Expr::App(mid, _) = outer.as_ref() {
                if let Expr::App(inner, _) = mid.as_ref() {
                    if let Expr::Const(n, _) = inner.as_ref() {
                        assert_eq!(*n, Name::str("Quotient.lift"));
                    }
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_quotient_map_structure() {
        let f = cst("my_func");
        let h = cst("my_proof");
        let q = cst("my_quot");
        let e = mk_quotient_map(f, h, q);
        if let Expr::App(outer, _) = &e {
            if let Expr::App(mid, _) = outer.as_ref() {
                if let Expr::App(inner, _) = mid.as_ref() {
                    if let Expr::Const(n, _) = inner.as_ref() {
                        assert_eq!(*n, Name::str("Quotient.map"));
                    }
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_quotient_ind_structure() {
        let h = cst("my_proof");
        let q = cst("my_quot");
        let e = mk_quotient_ind(h, q);
        if let Expr::App(f, rhs) = &e {
            if let Expr::Const(n, _) = rhs.as_ref() {
                assert_eq!(*n, Name::str("my_quot"));
            }
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Quotient.ind"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_quotient_sound_structure() {
        let h = cst("my_proof");
        let e = mk_quotient_sound(h);
        if let Expr::App(f, arg) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Quotient.sound"));
            }
            if let Expr::Const(n, _) = arg.as_ref() {
                assert_eq!(*n, Name::str("my_proof"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_setoid_r_structure() {
        let s = cst("my_setoid");
        let a = cst("x");
        let b = cst("y");
        let e = mk_setoid_r(s, a, b);
        if let Expr::App(outer, rhs) = &e {
            if let Expr::Const(n, _) = rhs.as_ref() {
                assert_eq!(*n, Name::str("y"));
            }
            if let Expr::App(mid, lhs) = outer.as_ref() {
                if let Expr::Const(n, _) = lhs.as_ref() {
                    assert_eq!(*n, Name::str("x"));
                }
                if let Expr::App(inner, _) = mid.as_ref() {
                    if let Expr::Const(n, _) = inner.as_ref() {
                        assert_eq!(*n, Name::str("Setoid.r"));
                    }
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_quot_structure() {
        let r = cst("my_rel");
        let e = mk_quot(r);
        if let Expr::App(f, arg) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Quot"));
            }
            if let Expr::Const(n, _) = arg.as_ref() {
                assert_eq!(*n, Name::str("my_rel"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_quot_mk_structure() {
        let val = cst("x");
        let e = mk_quot_mk(val);
        if let Expr::App(f, arg) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Quot.mk"));
            }
            if let Expr::Const(n, _) = arg.as_ref() {
                assert_eq!(*n, Name::str("x"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_quot_lift_structure() {
        let f = cst("my_func");
        let h = cst("my_proof");
        let q = cst("my_quot");
        let e = mk_quot_lift(f, h, q);
        if let Expr::App(outer, rhs) = &e {
            if let Expr::Const(n, _) = rhs.as_ref() {
                assert_eq!(*n, Name::str("my_quot"));
            }
            if let Expr::App(mid, _) = outer.as_ref() {
                if let Expr::App(inner, _) = mid.as_ref() {
                    if let Expr::Const(n, _) = inner.as_ref() {
                        assert_eq!(*n, Name::str("Quot.lift"));
                    }
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_quot_sound_structure() {
        let h = cst("my_proof");
        let e = mk_quot_sound(h);
        if let Expr::App(f, arg) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Quot.sound"));
            }
            if let Expr::Const(n, _) = arg.as_ref() {
                assert_eq!(*n, Name::str("my_proof"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_has_equiv_structure() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let e = mk_has_equiv(nat);
        if let Expr::App(f, arg) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("HasEquiv"));
            }
            if let Expr::Const(n, _) = arg.as_ref() {
                assert_eq!(*n, Name::str("Nat"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_quotient_mk_then_lift() {
        let val = cst("x");
        let qmk = mk_quotient_mk(val);
        let f = cst("my_func");
        let h = cst("my_proof");
        let e = mk_quotient_lift(f, h, qmk);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_quotient_mk_then_map() {
        let val = cst("x");
        let qmk = mk_quotient_mk(val);
        let f = cst("my_func");
        let h = cst("my_proof");
        let e = mk_quotient_map(f, h, qmk);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_quot_mk_then_lift() {
        let val = cst("x");
        let qmk = mk_quot_mk(val);
        let f = cst("my_func");
        let h = cst("my_proof");
        let e = mk_quot_lift(f, h, qmk);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_setoid_r_applied() {
        let s = cst("my_setoid");
        let a = cst("a");
        let b = cst("b");
        let rel = mk_setoid_r(s, a, b);
        assert!(matches!(rel, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_eq_expr_quotients() {
        let lhs = mk_quotient_mk(cst("a"));
        let rhs = mk_quotient_mk(cst("b"));
        let e = mk_eq_expr(mk_quotient(), lhs, rhs);
        if let Expr::App(f, _) = &e {
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::App(h, _) = g.as_ref() {
                    if let Expr::Const(n, _) = h.as_ref() {
                        assert_eq!(*n, Name::str("Eq"));
                    }
                }
            }
        } else {
            panic!("expected App");
        }
    }
}
