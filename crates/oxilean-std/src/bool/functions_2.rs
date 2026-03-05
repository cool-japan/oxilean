//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, InductiveEnv, Level, Name};

use super::functions::*;
use super::types::*;

/// Check that (Bool, XOR, AND) satisfies ring axioms by exhaustive verification.
#[allow(dead_code)]
pub fn verify_gf2_ring() -> bool {
    let vals = [false, true];
    for &a in &vals {
        for &b in &vals {
            for &c in &vals {
                if gf2_add(gf2_add(a, b), c) != gf2_add(a, gf2_add(b, c)) {
                    return false;
                }
                if gf2_mul(a, gf2_add(b, c)) != gf2_add(gf2_mul(a, b), gf2_mul(a, c)) {
                    return false;
                }
            }
        }
    }
    true
}
/// Verify De Morgan laws by exhaustive truth table check.
#[allow(dead_code)]
pub fn verify_demorgan() -> bool {
    let vals = [false, true];
    for &a in &vals {
        for &b in &vals {
            if !(a && b) != (!a || !b) {
                return false;
            }
            if !(a || b) != (!a && !b) {
                return false;
            }
        }
    }
    true
}
/// Verify lattice absorption laws by exhaustive check.
#[allow(dead_code, clippy::overly_complex_bool_expr)]
pub fn verify_lattice_absorption() -> bool {
    let vals = [false, true];
    for &a in &vals {
        for &b in &vals {
            if (a && (a || b)) != a {
                return false;
            }
            if (a || (a && b)) != a {
                return false;
            }
        }
    }
    true
}
/// Verify all basic Boolean algebra laws exhaustively.
#[allow(dead_code)]
pub fn verify_boolean_algebra() -> bool {
    verify_demorgan() && verify_gf2_ring() && verify_lattice_absorption()
}
/// Verify Kleene three-value logic satisfies De Morgan-like laws.
#[allow(dead_code)]
pub fn verify_kleene_demorgan() -> bool {
    let vals = [Kleene3Val::False, Kleene3Val::Unknown, Kleene3Val::True];
    for &a in &vals {
        for &b in &vals {
            if a.and(b).not() != a.not().or(b.not()) {
                return false;
            }
            if a.or(b).not() != a.not().and(b.not()) {
                return false;
            }
        }
    }
    true
}
/// A decision procedure for Bool predicates that checks all values.
#[allow(dead_code)]
pub fn decide_bool_pred(pred: impl Fn(bool) -> bool) -> Option<bool> {
    if pred(true) {
        Some(true)
    } else if pred(false) {
        Some(false)
    } else {
        None
    }
}
/// Verify XOR monoid laws: (Bool, XOR, false) is a commutative group.
#[allow(dead_code)]
pub fn verify_xor_monoid() -> bool {
    let vals = [false, true];
    for &a in &vals {
        if a ^ false != a {
            return false;
        }
        #[allow(clippy::eq_op)]
        if a ^ a {
            return false;
        }
        for &b in &vals {
            #[allow(clippy::eq_op)]
            if (a ^ b) != (b ^ a) {
                return false;
            }
            for &c in &vals {
                if ((a ^ b) ^ c) != (a ^ (b ^ c)) {
                    return false;
                }
            }
        }
    }
    true
}
/// Check NAND functional completeness by expressing NOT via NAND.
/// NOT a = NAND a a
#[allow(dead_code, clippy::eq_op)]
pub fn nand_not(a: bool) -> bool {
    !(a && a)
}
/// Check NOR functional completeness by expressing NOT via NOR.
/// NOT a = NOR a a
#[allow(dead_code, clippy::eq_op)]
pub fn nor_not(a: bool) -> bool {
    !(a || a)
}
/// Verify that NAND and NOR implement NOT correctly.
#[allow(dead_code)]
pub fn verify_functional_completeness() -> bool {
    !nand_not(true) && nand_not(false) && !nor_not(true) && nor_not(false)
}
#[cfg(test)]
mod extended_bool_tests {
    use super::*;
    fn setup_extended_env() -> (Environment, InductiveEnv) {
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
    fn test_register_bool_extended_axioms() {
        let (mut env, mut ind_env) = setup_extended_env();
        build_bool_env(&mut env, &mut ind_env).expect("build_bool_env should succeed");
        register_bool_extended_axioms(&mut env);
        assert!(env.contains(&Name::str("Bool.demorgan_and")));
        assert!(env.contains(&Name::str("Bool.demorgan_or")));
        assert!(env.contains(&Name::str("Bool.and_complement")));
        assert!(env.contains(&Name::str("Bool.or_complement")));
        assert!(env.contains(&Name::str("Bool.and_idempotent")));
        assert!(env.contains(&Name::str("Bool.or_idempotent")));
        assert!(env.contains(&Name::str("Bool.and_absorption")));
        assert!(env.contains(&Name::str("Bool.or_absorption")));
    }
    #[test]
    fn test_bool_extended_xor_axioms() {
        let (mut env, mut ind_env) = setup_extended_env();
        build_bool_env(&mut env, &mut ind_env).expect("build_bool_env should succeed");
        register_bool_extended_axioms(&mut env);
        assert!(env.contains(&Name::str("Bool.xor_self")));
        assert!(env.contains(&Name::str("Bool.xor_assoc")));
        assert!(env.contains(&Name::str("Bool.and_distrib_xor")));
        assert!(env.contains(&Name::str("Bool.xor_false")));
        assert!(env.contains(&Name::str("Bool.xor_true")));
        assert!(env.contains(&Name::str("Bool.xor_not_beq")));
    }
    #[test]
    fn test_bool_extended_heyting() {
        let (mut env, mut ind_env) = setup_extended_env();
        build_bool_env(&mut env, &mut ind_env).expect("build_bool_env should succeed");
        register_bool_extended_axioms(&mut env);
        assert!(env.contains(&Name::str("BoolImply")));
        assert!(env.contains(&Name::str("Bool.imply_def")));
        assert!(env.contains(&Name::str("Bool.imply_refl")));
    }
    #[test]
    fn test_bool_extended_sat_instance() {
        let mut sat = SATInstance::new(2);
        sat.add_clause(vec![1, 2]);
        sat.add_clause(vec![-1, 2]);
        let solution = sat.solve();
        assert!(solution.is_some());
        let sol = solution.expect("solution should be valid");
        assert!(sat.evaluate(&sol));
    }
    #[test]
    fn test_sat_unsat() {
        let mut sat = SATInstance::new(1);
        sat.add_clause(vec![1]);
        sat.add_clause(vec![-1]);
        assert!(sat.solve().is_none());
    }
    #[test]
    fn test_gf2_ring_verify() {
        assert!(verify_gf2_ring());
    }
    #[test]
    fn test_demorgan_verify() {
        assert!(verify_demorgan());
    }
    #[test]
    fn test_lattice_absorption_verify() {
        assert!(verify_lattice_absorption());
    }
    #[test]
    fn test_boolean_algebra_full_verify() {
        assert!(verify_boolean_algebra());
    }
    #[test]
    fn test_xor_monoid_verify() {
        assert!(verify_xor_monoid());
    }
    #[test]
    fn test_functional_completeness() {
        assert!(verify_functional_completeness());
    }
    #[test]
    fn test_kleene_three_value_not() {
        assert_eq!(Kleene3Val::True.not(), Kleene3Val::False);
        assert_eq!(Kleene3Val::False.not(), Kleene3Val::True);
        assert_eq!(Kleene3Val::Unknown.not(), Kleene3Val::Unknown);
    }
    #[test]
    fn test_kleene_three_value_and() {
        assert_eq!(Kleene3Val::True.and(Kleene3Val::True), Kleene3Val::True);
        assert_eq!(Kleene3Val::True.and(Kleene3Val::False), Kleene3Val::False);
        assert_eq!(
            Kleene3Val::True.and(Kleene3Val::Unknown),
            Kleene3Val::Unknown
        );
        assert_eq!(
            Kleene3Val::False.and(Kleene3Val::Unknown),
            Kleene3Val::False
        );
    }
    #[test]
    fn test_kleene_three_value_or() {
        assert_eq!(Kleene3Val::False.or(Kleene3Val::False), Kleene3Val::False);
        assert_eq!(Kleene3Val::False.or(Kleene3Val::True), Kleene3Val::True);
        assert_eq!(
            Kleene3Val::False.or(Kleene3Val::Unknown),
            Kleene3Val::Unknown
        );
        assert_eq!(Kleene3Val::True.or(Kleene3Val::Unknown), Kleene3Val::True);
    }
    #[test]
    fn test_kleene_demorgan() {
        assert!(verify_kleene_demorgan());
    }
    #[test]
    fn test_decide_bool_pred_some() {
        let result = decide_bool_pred(|b| b);
        assert_eq!(result, Some(true));
    }
    #[test]
    fn test_decide_bool_pred_none() {
        let result = decide_bool_pred(|_| false);
        assert_eq!(result, None);
    }
    #[test]
    fn test_nand_not() {
        assert!(!nand_not(true));
        assert!(nand_not(false));
    }
    #[test]
    fn test_nor_not() {
        assert!(!nor_not(true));
        assert!(nor_not(false));
    }
    #[test]
    fn test_bool_algebra_struct() {
        let ba = BoolAlgebra {
            carrier_size: 2,
            involutive: true,
            de_morgan: true,
        };
        assert_eq!(ba.carrier_size, 2);
        assert!(ba.involutive);
        assert!(ba.de_morgan);
    }
    #[test]
    fn test_xor_monoid_struct() {
        let xm = XorMonoid {
            identity: false,
            associative: true,
            self_inverse: true,
        };
        assert!(!xm.identity);
        assert!(xm.self_inverse);
    }
    #[test]
    fn test_bool_lattice_struct() {
        let bl = BoolLattice {
            bottom: false,
            top: true,
            distributive: true,
            complemented: true,
        };
        assert!(!bl.bottom);
        assert!(bl.top);
        assert!(bl.distributive);
        assert!(bl.complemented);
    }
    #[test]
    fn test_decidable_pred_struct() {
        let dp: DecidablePred<i32> = DecidablePred {
            decide: Box::new(|x| *x > 0),
            name: "positive".to_string(),
        };
        assert!((dp.decide)(&5));
        assert!(!(dp.decide)(&-1));
    }
    #[test]
    fn test_kleene3_from_bool() {
        assert_eq!(Kleene3Val::from_bool(true), Kleene3Val::True);
        assert_eq!(Kleene3Val::from_bool(false), Kleene3Val::False);
    }
    #[test]
    fn test_bool_extended_monoid_axioms() {
        let (mut env, mut ind_env) = setup_extended_env();
        build_bool_env(&mut env, &mut ind_env).expect("build_bool_env should succeed");
        register_bool_extended_axioms(&mut env);
        assert!(env.contains(&Name::str("Bool.and_monoid_left_id")));
        assert!(env.contains(&Name::str("Bool.and_monoid_right_id")));
        assert!(env.contains(&Name::str("Bool.or_monoid_left_id")));
        assert!(env.contains(&Name::str("Bool.or_monoid_right_id")));
    }
    #[test]
    fn test_bool_extended_fold_axioms() {
        let (mut env, mut ind_env) = setup_extended_env();
        build_bool_env(&mut env, &mut ind_env).expect("build_bool_env should succeed");
        register_bool_extended_axioms(&mut env);
        assert!(env.contains(&Name::str("BoolAll")));
        assert!(env.contains(&Name::str("BoolAny")));
    }
    #[test]
    fn test_bool_extended_ite_axioms() {
        let (mut env, mut ind_env) = setup_extended_env();
        build_bool_env(&mut env, &mut ind_env).expect("build_bool_env should succeed");
        register_bool_extended_axioms(&mut env);
        assert!(env.contains(&Name::str("BoolIte")));
        assert!(env.contains(&Name::str("BoolIte.true_branch")));
        assert!(env.contains(&Name::str("BoolIte.false_branch")));
    }
    #[test]
    fn test_bool_extended_nand_nor() {
        let (mut env, mut ind_env) = setup_extended_env();
        build_bool_env(&mut env, &mut ind_env).expect("build_bool_env should succeed");
        register_bool_extended_axioms(&mut env);
        assert!(env.contains(&Name::str("Bool.nand")));
        assert!(env.contains(&Name::str("Bool.nand_def")));
        assert!(env.contains(&Name::str("Bool.nor")));
        assert!(env.contains(&Name::str("Bool.nor_def")));
    }
    #[test]
    fn test_bool_extended_kleene3_registered() {
        let (mut env, mut ind_env) = setup_extended_env();
        build_bool_env(&mut env, &mut ind_env).expect("build_bool_env should succeed");
        register_bool_extended_axioms(&mut env);
        assert!(env.contains(&Name::str("Kleene3")));
        assert!(env.contains(&Name::str("Kleene3.unknown")));
        assert!(env.contains(&Name::str("Kleene3.and")));
    }
    #[test]
    fn test_bool_extended_beq_axioms() {
        let (mut env, mut ind_env) = setup_extended_env();
        build_bool_env(&mut env, &mut ind_env).expect("build_bool_env should succeed");
        register_bool_extended_axioms(&mut env);
        assert!(env.contains(&Name::str("Bool.beq_true")));
        assert!(env.contains(&Name::str("Bool.beq_false")));
        assert!(env.contains(&Name::str("Bool.beq_iff_eq_aux")));
        assert!(env.contains(&Name::str("BEqBoolInst")));
    }
    #[test]
    fn test_sat_instance_empty() {
        let sat = SATInstance::new(3);
        let solution = sat.solve();
        assert!(solution.is_some());
    }
    #[test]
    fn test_sat_instance_tautological_clause() {
        let mut sat = SATInstance::new(1);
        sat.add_clause(vec![1, -1]);
        assert!(sat.solve().is_some());
    }
    #[test]
    fn test_gf2_add_mul() {
        assert!(!gf2_add(false, false));
        assert!(gf2_add(true, false));
        assert!(gf2_add(false, true));
        assert!(!gf2_add(true, true));
        assert!(!gf2_mul(false, false));
        assert!(!gf2_mul(true, false));
        assert!(!gf2_mul(false, true));
        assert!(gf2_mul(true, true));
    }
    #[test]
    fn test_bool_extended_demorgan_duality() {
        let (mut env, mut ind_env) = setup_extended_env();
        build_bool_env(&mut env, &mut ind_env).expect("build_bool_env should succeed");
        register_bool_extended_axioms(&mut env);
        assert!(env.contains(&Name::str("DeMorganDuality")));
        assert!(env.contains(&Name::str("SATDecidable")));
        assert!(env.contains(&Name::str("TautologyCheck")));
    }
}
