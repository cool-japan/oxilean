//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::functions::*;
use super::types::*;

#[cfg(test)]
mod lazy_extended_tests {
    use super::*;
    fn full_lazy_ext_env() -> Environment {
        let mut env = Environment::new();
        let type1 = Expr::Sort(Level::succ(Level::zero()));
        let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
        for name in &["Unit", "Nat", "Bool", "Int"] {
            env.add(Declaration::Axiom {
                name: Name::str(*name),
                univ_params: vec![],
                ty: type1.clone(),
            })
            .expect("operation should succeed");
        }
        for nm in &["Option", "Stream"] {
            let ty = Expr::Pi(
                BinderInfo::Default,
                Name::str("α"),
                Box::new(type1.clone()),
                Box::new(type2.clone()),
            );
            env.add(Declaration::Axiom {
                name: Name::str(*nm),
                univ_params: vec![],
                ty,
            })
            .expect("operation should succeed");
        }
        env.add(Declaration::Axiom {
            name: Name::str("Prod"),
            univ_params: vec![],
            ty: Expr::Pi(
                BinderInfo::Default,
                Name::str("α"),
                Box::new(type1.clone()),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("β"),
                    Box::new(type1.clone()),
                    Box::new(type2.clone()),
                )),
            ),
        })
        .expect("operation should succeed");
        {
            let nm = "Fin";
            let ty = Expr::Pi(
                BinderInfo::Default,
                Name::str("n"),
                Box::new(Expr::Const(Name::str("Nat"), vec![])),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_"),
                    Box::new(type1.clone()),
                    Box::new(type2.clone()),
                )),
            );
            env.add(Declaration::Axiom {
                name: Name::str(nm),
                univ_params: vec![],
                ty,
            })
            .expect("operation should succeed");
        }
        build_lazy_env(&mut env).expect("build_lazy_env should succeed");
        env
    }
    #[test]
    fn test_register_lazy_extended_ok() {
        let mut env = full_lazy_ext_env();
        assert!(register_lazy_extended(&mut env).is_ok());
    }
    #[test]
    fn test_conat_registered() {
        let mut env = full_lazy_ext_env();
        register_lazy_extended(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("CoNat")).is_some());
        assert!(env.get(&Name::str("CoNat.zero")).is_some());
        assert!(env.get(&Name::str("CoNat.succ")).is_some());
        assert!(env.get(&Name::str("CoNat.infinity")).is_some());
    }
    #[test]
    fn test_nakano_registered() {
        let mut env = full_lazy_ext_env();
        register_lazy_extended(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Nakano.later")).is_some());
        assert!(env.get(&Name::str("Nakano.next")).is_some());
    }
    #[test]
    fn test_lob_axiom_registered() {
        let mut env = full_lazy_ext_env();
        register_lazy_extended(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Löb.axiom")).is_some());
    }
    #[test]
    fn test_delay_monad_registered() {
        let mut env = full_lazy_ext_env();
        register_lazy_extended(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Delay")).is_some());
        assert!(env.get(&Name::str("Delay.now")).is_some());
        assert!(env.get(&Name::str("Delay.later")).is_some());
        assert!(env.get(&Name::str("Delay.bind")).is_some());
    }
    #[test]
    fn test_capretta_setoid_registered() {
        let mut env = full_lazy_ext_env();
        register_lazy_extended(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Capretta.setoid")).is_some());
    }
    #[test]
    fn test_monad_laws_registered() {
        let mut env = full_lazy_ext_env();
        register_lazy_extended(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Lazy.monad_left_identity")).is_some());
        assert!(env.get(&Name::str("Lazy.monad_right_identity")).is_some());
        assert!(env.get(&Name::str("Lazy.monad_assoc")).is_some());
    }
    #[test]
    fn test_productive_corecursion_registered() {
        let mut env = full_lazy_ext_env();
        register_lazy_extended(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Productive.corecursion")).is_some());
    }
    #[test]
    fn test_conat_rs_zero_succ() {
        let n = CoNatRs::zero();
        assert!(n.is_zero());
        let n1 = CoNatRs::Finite(0).succ();
        assert_eq!(n1, CoNatRs::Finite(1));
    }
    #[test]
    fn test_conat_rs_infinity_succ() {
        let inf = CoNatRs::Infinity;
        assert!(inf.is_infinity());
        let inf2 = CoNatRs::Infinity.succ();
        assert!(inf2.is_infinity());
    }
    #[test]
    fn test_conat_rs_add() {
        let a = CoNatRs::Finite(3);
        let b = CoNatRs::Finite(4);
        assert_eq!(a.add(b), CoNatRs::Finite(7));
        let c = CoNatRs::Finite(5);
        let inf = CoNatRs::Infinity;
        assert_eq!(c.add(inf), CoNatRs::Infinity);
    }
    #[test]
    fn test_conat_rs_min() {
        let a = CoNatRs::Finite(3);
        let b = CoNatRs::Finite(7);
        assert_eq!(a.clone().min(b), CoNatRs::Finite(3));
        assert_eq!(a.min(CoNatRs::Infinity), CoNatRs::Finite(3));
    }
    #[test]
    fn test_delay_rs_now() {
        let d: DelayRs<i32> = DelayRs::now(42);
        assert_eq!(d.run(0), Some(42));
    }
    #[test]
    fn test_delay_rs_later() {
        let d: DelayRs<i32> = DelayRs::later(|| DelayRs::later(|| DelayRs::now(7)));
        assert_eq!(d.run(0), None);
        let d2: DelayRs<i32> = DelayRs::later(|| DelayRs::later(|| DelayRs::now(7)));
        assert_eq!(d2.run(5), Some(7));
    }
    #[test]
    fn test_delay_rs_map() {
        let d = DelayRs::now(5i32).map(|x| x * 2);
        assert_eq!(d.run(1), Some(10));
    }
    #[test]
    fn test_delay_rs_bind() {
        let d = DelayRs::now(3i32).bind(|x| DelayRs::now(x + 1));
        assert_eq!(d.run(1), Some(4));
    }
    #[test]
    fn test_lazy_window_push_and_force() {
        let mut w: LazyWindowRs<i32> = LazyWindowRs::new(3);
        w.push(Deferred::pure(1));
        w.push(Deferred::pure(2));
        w.push(Deferred::pure(3));
        assert_eq!(w.len(), 3);
        let vals = w.force_all();
        assert_eq!(vals, vec![1, 2, 3]);
    }
    #[test]
    fn test_lazy_window_sliding() {
        let mut w: LazyWindowRs<i32> = LazyWindowRs::new(2);
        w.push(Deferred::pure(1));
        w.push(Deferred::pure(2));
        w.push(Deferred::pure(3));
        assert_eq!(w.len(), 2);
        let vals = w.force_all();
        assert_eq!(vals, vec![2, 3]);
    }
    #[test]
    fn test_lazy_window_capacity() {
        let w: LazyWindowRs<i32> = LazyWindowRs::new(5);
        assert_eq!(w.capacity(), 5);
        assert!(w.is_empty());
    }
    #[test]
    fn test_lzy_ext_conat_ty_is_sort() {
        let ty = lzy_ext_conat_ty();
        assert!(matches!(ty, Expr::Sort(_)));
    }
    #[test]
    fn test_lzy_ext_lob_axiom_ty_is_pi() {
        let ty = lzy_ext_lob_axiom_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_lzy_ext_delay_bind_ty_is_pi() {
        let ty = lzy_ext_delay_bind_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_all_extended_lazy_axioms_count() {
        let mut env = full_lazy_ext_env();
        register_lazy_extended(&mut env).expect("operation should succeed");
        let extended_names = [
            "CoNat",
            "CoNat.zero",
            "CoNat.succ",
            "CoNat.infinity",
            "Nakano.later",
            "Nakano.next",
            "Löb.axiom",
            "Delay",
            "Delay.now",
            "Delay.later",
            "Delay.bind",
            "Capretta.setoid",
            "Lazy.monad_left_identity",
            "Lazy.monad_right_identity",
            "Lazy.monad_assoc",
            "Productive.corecursion",
            "Lazy.force_pure",
            "Lazy.eta",
        ];
        for name in &extended_names {
            assert!(
                env.get(&Name::str(*name)).is_some(),
                "{} should be registered",
                name
            );
        }
    }
}
