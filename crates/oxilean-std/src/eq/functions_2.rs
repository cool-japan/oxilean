//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::env_builder::*;
use oxilean_kernel::{Declaration, Environment, Expr, Level, Name};

use super::functions::*;
use super::types::*;

/// Build the type of `Eq.recOn` (motive-first recursor).
pub fn mk_eq_rec_on_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_implicit(
            "a",
            bvar(0),
            pi_implicit(
                "b",
                bvar(1),
                pi_named(
                    "h",
                    app(app(app(var("Eq"), bvar(2)), bvar(1)), bvar(0)),
                    pi_named(
                        "P",
                        pi(bvar(3), eq_ext_prop()),
                        pi(app(bvar(0), bvar(3)), app(bvar(1), bvar(3))),
                    ),
                ),
            ),
        ),
    )
}
/// Build the type of the definitional-to-propositional bridge.
pub fn mk_defn_to_prop_eq_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_named(
            "a",
            bvar(0),
            app(app(app(var("Eq"), bvar(1)), bvar(0)), bvar(0)),
        ),
    )
}
/// Build the type of the Nat.succ injectivity axiom.
pub fn mk_nat_succ_inj_ty() -> Expr {
    pi_named(
        "m",
        eq_ext_nat_ty(),
        pi_named(
            "n",
            eq_ext_nat_ty(),
            pi(
                app(
                    app(
                        app(var("Eq"), eq_ext_nat_ty()),
                        app(var("Nat.succ"), bvar(1)),
                    ),
                    app(var("Nat.succ"), bvar(0)),
                ),
                app(app(app(var("Eq"), eq_ext_nat_ty()), bvar(2)), bvar(1)),
            ),
        ),
    )
}
/// Build the type of the Nat zero-vs-succ distinctness axiom.
pub fn mk_nat_zero_ne_succ_ty() -> Expr {
    pi_named(
        "n",
        eq_ext_nat_ty(),
        pi(
            app(
                app(app(var("Eq"), eq_ext_nat_ty()), var("Nat.zero")),
                app(var("Nat.succ"), bvar(0)),
            ),
            var("False"),
        ),
    )
}
/// Build the type of the HEq symmetry axiom.
pub fn mk_heq_symm_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_implicit(
            "β",
            sort(1),
            pi_implicit(
                "a",
                bvar(1),
                pi_implicit(
                    "b",
                    bvar(1),
                    pi(
                        app(
                            app(app(app(var("HEq"), bvar(3)), bvar(1)), bvar(2)),
                            bvar(0),
                        ),
                        app(
                            app(app(app(var("HEq"), bvar(3)), bvar(1)), bvar(4)),
                            bvar(2),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Build the type of the HEq transitivity axiom.
pub fn mk_heq_trans_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_implicit(
            "β",
            sort(1),
            pi_implicit(
                "γ",
                sort(1),
                pi_implicit(
                    "a",
                    bvar(2),
                    pi_implicit(
                        "b",
                        bvar(2),
                        pi_implicit(
                            "c",
                            bvar(2),
                            pi(
                                app(
                                    app(app(app(var("HEq"), bvar(5)), bvar(2)), bvar(4)),
                                    bvar(1),
                                ),
                                pi(
                                    app(
                                        app(app(app(var("HEq"), bvar(5)), bvar(2)), bvar(4)),
                                        bvar(1),
                                    ),
                                    app(
                                        app(app(app(var("HEq"), bvar(7)), bvar(4)), bvar(6)),
                                        bvar(3),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Register all extended equality axioms into a kernel `Environment`.
///
/// This function is idempotent: if an axiom is already present it is silently
/// skipped.  The axioms cover Eq class laws, primitive/compound type equality,
/// Leibniz equality, K/UIP/J axioms, HEq, subst, congruence, funext/propext,
/// quotient equality, bisimulation, observational equality, setoid morphisms,
/// groupoid structure, decidable equality, and homotopy equivalence.
pub fn register_eq_extended_axioms(env: &mut Environment) {
    let axioms: &[(&str, fn() -> Expr)] = &[
        ("Eq.class.refl", mk_eq_class_refl_ty),
        ("Eq.class.symm", mk_eq_class_symm_ty),
        ("Eq.class.trans", mk_eq_class_trans_ty),
        ("Eq.beq.consistency", mk_beq_eq_consistency_ty),
        ("Nat.decEq", mk_nat_decidable_eq_ty),
        ("Bool.decEq", mk_bool_decidable_eq_ty),
        ("Char.decEq", mk_char_decidable_eq_ty),
        ("Float.decEq", mk_float_eq_decidable_ty),
        ("Int.decEq", mk_int_decidable_eq_ty),
        ("List.decEq", mk_list_decidable_eq_ty),
        ("Option.decEq", mk_option_decidable_eq_ty),
        ("Pair.decEq", mk_pair_decidable_eq_ty),
        ("Eq.leibniz", mk_leibniz_eq_ty),
        ("Eq.leibniz.subst", mk_leibniz_subst_ty),
        ("Eq.reflection", mk_eq_reflection_ty),
        ("Eq.K", mk_k_axiom_ty),
        ("Eq.UIP", mk_uip_ty),
        ("Eq.J", mk_j_axiom_ty),
        ("HEq.intro", mk_heq_intro_ty),
        ("HEq.typeEq", mk_heq_type_eq_ty),
        ("Eq.subst.general", mk_subst_axiom_ty),
        ("Eq.cong.general", mk_cong_axiom_ty),
        ("funext", mk_funext_ty),
        ("propext", mk_propext_ty),
        ("Quotient.sound", mk_quotient_sound_ty),
        ("Bisim.eq", mk_bisim_eq_ty),
        ("Eq.obs", mk_obs_eq_ty),
        ("Setoid.mk", mk_setoid_ax_ty),
        ("Setoid.morphism", mk_setoid_morphism_ax_ty),
        ("Path.concat", mk_path_concat_ty),
        ("Path.inv", mk_path_inv_ty),
        ("Eq.defn.mltt", mk_def_eq_mltt_ty),
        ("Eq.decEq", mk_decidable_eq_instance_ty),
        ("HomotopyEquiv.toEquiv", mk_homotopy_equiv_ty),
        ("Eq.cong.closure", mk_cong_closure_ty),
        ("Subsingleton.eq", mk_subsingleton_eq_ty),
        ("Sigma.eq", mk_sigma_eq_ty),
        ("Subtype.eq", mk_subtype_eq_ty),
        ("Eq.fun.pointwise", mk_fun_eq_pointwise_ty),
        ("Either.decEq", mk_either_decidable_eq_ty),
        ("Result.decEq", mk_result_decidable_eq_ty),
        ("Eq.ndrec", mk_eq_ndrec_ty),
        ("Eq.recOn", mk_eq_rec_on_ty),
        ("Eq.defn.to.prop", mk_defn_to_prop_eq_ty),
        ("Nat.succ.inj", mk_nat_succ_inj_ty),
        ("Nat.zero.ne.succ", mk_nat_zero_ne_succ_ty),
        ("HEq.symm", mk_heq_symm_ty),
        ("HEq.trans", mk_heq_trans_ty),
    ];
    for (name, builder) in axioms {
        let ty = builder();
        let _ = env.add(Declaration::Axiom {
            name: Name::str(name.to_string()),
            univ_params: vec![],
            ty,
        });
    }
}
/// Alias for `mk_eq_class_symm_ty`.
pub fn mk_eq_symm_simple_ty() -> Expr {
    mk_eq_class_symm_ty()
}
/// Alias for `mk_nat_decidable_eq_ty`.
pub fn mk_nat_eq_simple_ty() -> Expr {
    mk_nat_decidable_eq_ty()
}
#[cfg(test)]
mod eq_kernel_ext_tests {
    use super::*;
    #[test]
    fn test_mk_eq_class_refl_ty_shape() {
        let ty = mk_eq_class_refl_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_eq_class_symm_ty_shape() {
        let ty = mk_eq_class_symm_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_eq_class_trans_ty_shape() {
        let ty = mk_eq_class_trans_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_leibniz_eq_ty_shape() {
        let ty = mk_leibniz_eq_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_k_axiom_ty_shape() {
        let ty = mk_k_axiom_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_uip_ty_shape() {
        let ty = mk_uip_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_j_axiom_ty_shape() {
        let ty = mk_j_axiom_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_funext_ty_shape() {
        let ty = mk_funext_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_propext_ty_shape() {
        let ty = mk_propext_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_nat_decidable_eq_ty_shape() {
        let ty = mk_nat_decidable_eq_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_bool_decidable_eq_ty_shape() {
        let ty = mk_bool_decidable_eq_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_heq_intro_ty_shape() {
        let ty = mk_heq_intro_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_heq_symm_ty_shape() {
        let ty = mk_heq_symm_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_heq_trans_ty_shape() {
        let ty = mk_heq_trans_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_nat_succ_inj_ty_shape() {
        let ty = mk_nat_succ_inj_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_nat_zero_ne_succ_ty_shape() {
        let ty = mk_nat_zero_ne_succ_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_cong_axiom_ty_shape() {
        let ty = mk_cong_axiom_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_subst_axiom_ty_shape() {
        let ty = mk_subst_axiom_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_decidable_eq_instance_ty_shape() {
        let ty = mk_decidable_eq_instance_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_path_concat_ty_shape() {
        let ty = mk_path_concat_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_obs_eq_ty_shape() {
        let ty = mk_obs_eq_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_homotopy_equiv_ty_shape() {
        let ty = mk_homotopy_equiv_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_subsingleton_eq_ty_shape() {
        let ty = mk_subsingleton_eq_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_leibniz_subst_ty_shape() {
        let ty = mk_leibniz_subst_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_beq_consistency_ty_shape() {
        let ty = mk_beq_eq_consistency_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_eq_ext_prop_is_sort() {
        let p = eq_ext_prop();
        assert!(matches!(p, Expr::Sort(_)));
    }
    #[test]
    fn test_eq_ext_type0_is_sort() {
        let t = eq_ext_type0();
        assert!(matches!(t, Expr::Sort(_)));
    }
    #[test]
    fn test_eq_ext_bvar_index() {
        let b = eq_ext_bvar(3);
        assert!(matches!(b, Expr::BVar(3)));
    }
    #[test]
    fn test_eq_ext_cst_is_const() {
        let c = eq_ext_cst("Nat");
        assert!(matches!(c, Expr::Const(_, _)));
    }
    #[test]
    fn test_eq_ext_arrow_is_pi() {
        let arr = eq_ext_arrow(eq_ext_nat_ty(), eq_ext_prop());
        assert!(matches!(arr, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_eq_ext_app2_shape() {
        let f = var("f");
        let a = var("a");
        let b = var("b");
        let result = eq_ext_app2(f, a, b);
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_setoid_instance_refl() {
        let s: SetoidInstance<u32> = SetoidInstance::new(vec![1, 2, 3], |a, b| a == b);
        assert!(s.check_refl());
    }
    #[test]
    fn test_setoid_instance_symm() {
        let s: SetoidInstance<u32> = SetoidInstance::new(vec![1, 2, 3], |a, b| a == b);
        assert!(s.check_symm());
    }
    #[test]
    fn test_setoid_instance_trans() {
        let s: SetoidInstance<u32> = SetoidInstance::new(vec![1, 2, 3], |a, b| a == b);
        assert!(s.check_trans());
    }
    #[test]
    fn test_setoid_instance_are_equiv() {
        let s: SetoidInstance<u32> = SetoidInstance::new(vec![1, 2], |a, b| a == b);
        assert!(s.are_equiv(&1, &1));
        assert!(!s.are_equiv(&1, &2));
    }
    #[test]
    fn test_leibniz_eq_refl_apply() {
        let leq: LeibnizEq<u32, u32> = LeibnizEq::refl();
        assert_eq!(leq.apply(42), 42);
    }
    #[test]
    fn test_heterogeneous_eq_homogeneous_true() {
        let heq: HeterogeneousEq<u32, u32> = HeterogeneousEq::new(5, 5, "refl");
        assert!(heq.is_homogeneous_eq());
    }
    #[test]
    fn test_heterogeneous_eq_homogeneous_false() {
        let heq: HeterogeneousEq<u32, u32> = HeterogeneousEq::new(5, 6, "different");
        assert!(!heq.is_homogeneous_eq());
    }
    #[test]
    fn test_heterogeneous_eq_to_homogeneous_some() {
        let heq: HeterogeneousEq<u32, u32> = HeterogeneousEq::new(7, 7, "refl");
        assert!(heq.to_homogeneous().is_some());
    }
    #[test]
    fn test_heterogeneous_eq_to_homogeneous_none() {
        let heq: HeterogeneousEq<u32, u32> = HeterogeneousEq::new(7, 8, "distinct");
        assert!(heq.to_homogeneous().is_none());
    }
    #[test]
    fn test_setoid_morphism_apply() {
        let m: SetoidMorphism<u32, u32> = SetoidMorphism::new(|x| x + 1, |_, _, e| e);
        assert_eq!(m.apply(10), 11);
    }
    #[test]
    fn test_setoid_morphism_verify_respects() {
        let m: SetoidMorphism<u32, u32> = SetoidMorphism::new(|x| x * 2, |_, _, e| e);
        assert!(m.verify_respects(&1, &1, true));
        assert!(!m.verify_respects(&1, &2, false));
    }
    #[test]
    fn test_decidable_eq_instance_u32() {
        let inst: DecidableEqInstance<u32> = DecidableEqInstance::for_type("Nat");
        assert!(inst.is_eq(&42, &42));
        assert!(!inst.is_eq(&42, &43));
    }
    #[test]
    fn test_decidable_eq_instance_str() {
        let inst: DecidableEqInstance<&str> = DecidableEqInstance::for_type("String");
        assert!(inst.is_eq(&"hello", &"hello"));
        assert!(!inst.is_eq(&"hello", &"world"));
    }
    #[test]
    fn test_register_eq_extended_axioms_smoke() {
        let mut env = Environment::default();
        register_eq_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Eq.class.refl")).is_some());
        assert!(env.get(&Name::str("Eq.K")).is_some());
        assert!(env.get(&Name::str("Eq.UIP")).is_some());
        assert!(env.get(&Name::str("Eq.J")).is_some());
        assert!(env.get(&Name::str("funext")).is_some());
        assert!(env.get(&Name::str("propext")).is_some());
        assert!(env.get(&Name::str("HEq.symm")).is_some());
        assert!(env.get(&Name::str("HEq.trans")).is_some());
        assert!(env.get(&Name::str("Nat.succ.inj")).is_some());
        assert!(env.get(&Name::str("Nat.zero.ne.succ")).is_some());
    }
    #[test]
    fn test_register_eq_extended_axioms_idempotent() {
        let mut env = Environment::default();
        register_eq_extended_axioms(&mut env);
        let count_before = env.len();
        register_eq_extended_axioms(&mut env);
        let count_after = env.len();
        assert_eq!(count_before, count_after);
    }
    #[test]
    fn test_mk_eq_ndrec_ty_shape() {
        let ty = mk_eq_ndrec_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_eq_rec_on_ty_shape() {
        let ty = mk_eq_rec_on_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_sigma_eq_ty_shape() {
        let ty = mk_sigma_eq_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_subtype_eq_ty_shape() {
        let ty = mk_subtype_eq_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_either_decidable_eq_ty_shape() {
        let ty = mk_either_decidable_eq_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_result_decidable_eq_ty_shape() {
        let ty = mk_result_decidable_eq_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_fun_eq_pointwise_ty_shape() {
        let ty = mk_fun_eq_pointwise_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_cong_closure_ty_shape() {
        let ty = mk_cong_closure_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_bisim_eq_ty_shape() {
        let ty = mk_bisim_eq_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_quotient_sound_ty_shape() {
        let ty = mk_quotient_sound_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_setoid_ax_ty_shape() {
        let ty = mk_setoid_ax_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_setoid_morphism_ax_ty_shape() {
        let ty = mk_setoid_morphism_ax_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_heq_type_eq_ty_shape() {
        let ty = mk_heq_type_eq_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_defn_to_prop_eq_ty_shape() {
        let ty = mk_defn_to_prop_eq_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
}
