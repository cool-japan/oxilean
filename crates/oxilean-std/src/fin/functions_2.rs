//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{
    BinderInfo, Declaration, Environment, Expr, InductiveEnv, InductiveType, IntroRule, Level, Name,
};

use super::functions::*;
use super::types::*;

#[cfg(test)]
mod fin_extended_tests {
    use super::*;
    fn setup_env_for_extended() -> (Environment, InductiveEnv) {
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
    fn test_axiom_fin_card_ty() {
        let ty = axiom_fin_card_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_axiom_fin_perm_ty() {
        let ty = axiom_fin_perm_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_axiom_fin_binomial_ty() {
        let ty = axiom_fin_binomial_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_axiom_fin_euler_phi_ty() {
        let ty = axiom_fin_euler_phi_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_register_fin_extended() {
        let (mut env, mut ind_env) = setup_env_for_extended();
        build_fin_env(&mut env, &mut ind_env).expect("build_fin_env should succeed");
        let result = register_fin_extended(&mut env);
        assert!(result.is_ok(), "register_fin_extended failed: {:?}", result);
        assert!(env.get(&Name::str("Fin.Perm")).is_some());
        assert!(env.get(&Name::str("Nat.factorial")).is_some());
        assert!(env.get(&Name::str("Fin.binomial")).is_some());
        assert!(env.get(&Name::str("Fin.eulerPhi")).is_some());
        assert!(env.get(&Name::str("Fin.cayley_formula")).is_some());
        assert!(env.get(&Name::str("Fin.ramanujan_sum")).is_some());
    }
    #[test]
    fn test_fin_ext_perm_identity() {
        let p = FinExtPerm::identity(5);
        assert_eq!(p.sign(), 1);
        assert_eq!(p.cycle_count(), 5);
        assert!(!p.is_derangement());
    }
    #[test]
    fn test_fin_ext_perm_from_vec() {
        let p =
            FinExtPerm::from_vec(vec![1, 2, 0, 4, 3]).expect("FinExtPerm::from_vec should succeed");
        assert_eq!(p.len(), 5);
        assert!(p.is_derangement());
    }
    #[test]
    fn test_fin_ext_perm_invalid() {
        assert!(FinExtPerm::from_vec(vec![1, 1, 0]).is_none());
        assert!(FinExtPerm::from_vec(vec![3, 0, 1]).is_none());
    }
    #[test]
    fn test_fin_ext_perm_sign() {
        let p = FinExtPerm::from_vec(vec![1, 2, 0]).expect("FinExtPerm::from_vec should succeed");
        assert_eq!(p.sign(), 1);
        let q = FinExtPerm::from_vec(vec![1, 0, 2]).expect("FinExtPerm::from_vec should succeed");
        assert_eq!(q.sign(), -1);
    }
    #[test]
    fn test_fin_ext_perm_compose() {
        let p = FinExtPerm::from_vec(vec![1, 0, 2]).expect("FinExtPerm::from_vec should succeed");
        let q = FinExtPerm::from_vec(vec![0, 2, 1]).expect("FinExtPerm::from_vec should succeed");
        let r = p.compose(&q).expect("compose should succeed");
        assert_eq!(r.perm, vec![1, 2, 0]);
    }
    #[test]
    fn test_fin_ext_perm_inverse() {
        let p = FinExtPerm::from_vec(vec![2, 0, 1]).expect("FinExtPerm::from_vec should succeed");
        let inv = p.inverse();
        let id = p.compose(&inv).expect("compose should succeed");
        assert_eq!(id, FinExtPerm::identity(3));
    }
    #[test]
    fn test_fin_ext_perm_order() {
        let p = FinExtPerm::from_vec(vec![1, 2, 0]).expect("FinExtPerm::from_vec should succeed");
        assert_eq!(p.order(), 3);
        let id = FinExtPerm::identity(4);
        assert_eq!(id.order(), 1);
    }
    #[test]
    fn test_fin_ext_derangements() {
        assert_eq!(fin_ext_derangements(0), 1);
        assert_eq!(fin_ext_derangements(1), 0);
        assert_eq!(fin_ext_derangements(2), 1);
        assert_eq!(fin_ext_derangements(3), 2);
        assert_eq!(fin_ext_derangements(4), 9);
    }
    #[test]
    fn test_fin_ext_binomial_basic() {
        assert_eq!(fin_ext_binomial(5, 2), 10);
        assert_eq!(fin_ext_binomial(5, 0), 1);
        assert_eq!(fin_ext_binomial(5, 5), 1);
        assert_eq!(fin_ext_binomial(10, 3), 120);
    }
    #[test]
    fn test_fin_ext_binomial_symmetry() {
        for n in 0..8 {
            for k in 0..=n {
                assert_eq!(fin_ext_binomial(n, k), fin_ext_binomial(n, n - k));
            }
        }
    }
    #[test]
    fn test_fin_ext_stirling_second_basic() {
        assert_eq!(fin_ext_stirling_second(0, 0), 1);
        assert_eq!(fin_ext_stirling_second(4, 2), 7);
        assert_eq!(fin_ext_stirling_second(5, 3), 25);
    }
    #[test]
    fn test_fin_ext_product_encode_decode() {
        let p = FinExtProduct::new(3, 4);
        assert_eq!(p.size(), 12);
        assert_eq!(p.encode(1, 2), Some(6));
        assert_eq!(p.decode(6), Some((1, 2)));
    }
    #[test]
    fn test_fin_ext_product_sum() {
        let p = FinExtProduct::new(2, 3);
        let s = p.sum_over(|i, j| (i * 3 + j) as u64);
        assert_eq!(s, 0 + 1 + 2 + 3 + 4 + 5);
    }
    #[test]
    fn test_fin_ext_euler_phi() {
        assert_eq!(fin_ext_euler_phi(1), 1);
        assert_eq!(fin_ext_euler_phi(2), 1);
        assert_eq!(fin_ext_euler_phi(6), 2);
        assert_eq!(fin_ext_euler_phi(7), 6);
        assert_eq!(fin_ext_euler_phi(12), 4);
    }
    #[test]
    fn test_fin_ext_young_shape_basic() {
        let shape =
            FinExtYoungShape::new(vec![4, 3, 1]).expect("FinExtYoungShape::new should succeed");
        assert_eq!(shape.size(), 8);
        assert_eq!(shape.rows(), 3);
        assert_eq!(shape.row_len(0), 4);
        assert_eq!(shape.row_len(1), 3);
        assert_eq!(shape.row_len(2), 1);
    }
    #[test]
    fn test_fin_ext_young_shape_invalid() {
        assert!(FinExtYoungShape::new(vec![2, 3]).is_none());
    }
    #[test]
    fn test_fin_ext_young_conjugate() {
        let shape =
            FinExtYoungShape::new(vec![3, 2, 1]).expect("FinExtYoungShape::new should succeed");
        let conj = shape.conjugate();
        assert_eq!(conj.parts, vec![3, 2, 1]);
        let shape2 =
            FinExtYoungShape::new(vec![3, 1]).expect("FinExtYoungShape::new should succeed");
        let conj2 = shape2.conjugate();
        assert_eq!(conj2.parts, vec![2, 1, 1]);
    }
    #[test]
    fn test_fin_ext_young_hook_length() {
        let shape =
            FinExtYoungShape::new(vec![2, 1]).expect("FinExtYoungShape::new should succeed");
        assert_eq!(shape.hook_length_count(), 2);
        let row = FinExtYoungShape::new(vec![3]).expect("FinExtYoungShape::new should succeed");
        assert_eq!(row.hook_length_count(), 1);
        let sq = FinExtYoungShape::new(vec![2, 2]).expect("FinExtYoungShape::new should succeed");
        assert_eq!(sq.hook_length_count(), 2);
    }
}
