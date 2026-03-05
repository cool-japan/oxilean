//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::Environment;
    #[test]
    fn test_build_real_env() {
        let mut env = Environment::new();
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Eq"),
            univ_params: vec![],
            ty: pi(
                BinderInfo::Implicit,
                "_",
                type0(),
                pi(
                    BinderInfo::Default,
                    "_",
                    bvar(0),
                    pi(BinderInfo::Default, "_", bvar(1), prop()),
                ),
            ),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("And"),
            univ_params: vec![],
            ty: arrow(prop(), arrow(prop(), prop())),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Or"),
            univ_params: vec![],
            ty: arrow(prop(), arrow(prop(), prop())),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Iff"),
            univ_params: vec![],
            ty: arrow(prop(), arrow(prop(), prop())),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Not"),
            univ_params: vec![],
            ty: arrow(prop(), prop()),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Exists"),
            univ_params: vec![],
            ty: pi(
                BinderInfo::Implicit,
                "a",
                type0(),
                arrow(arrow(bvar(0), prop()), prop()),
            ),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Nat"),
            univ_params: vec![],
            ty: type0(),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Nat.zero"),
            univ_params: vec![],
            ty: nat_ty(),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Nat.succ"),
            univ_params: vec![],
            ty: arrow(nat_ty(), nat_ty()),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Nat.add"),
            univ_params: vec![],
            ty: arrow(nat_ty(), arrow(nat_ty(), nat_ty())),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Nat.mul"),
            univ_params: vec![],
            ty: arrow(nat_ty(), arrow(nat_ty(), nat_ty())),
        });
        let result = build_real_env(&mut env);
        assert!(result.is_ok(), "build_real_env failed: {:?}", result);
    }
    #[test]
    fn test_real_expression_builders() {
        let a = cst("a");
        let b = cst("b");
        let sum = real_add(a.clone(), b.clone());
        assert!(matches!(sum, Expr::App(_, _)));
        let prod = real_mul(a.clone(), b.clone());
        assert!(matches!(prod, Expr::App(_, _)));
        let neg = real_neg(a.clone());
        assert!(matches!(neg, Expr::App(_, _)));
        let inv = real_inv(a.clone());
        assert!(matches!(inv, Expr::App(_, _)));
        let _abs = real_abs(a.clone());
        assert!(matches!(_abs, Expr::App(_, _)));
        let le = real_le(a.clone(), b.clone());
        assert!(matches!(le, Expr::App(_, _)));
        let zero = real_zero();
        assert!(matches!(zero, Expr::Const(_, _)));
        let one = real_one();
        assert!(matches!(one, Expr::Const(_, _)));
        let dist = real_dist(a, b);
        assert!(matches!(dist, Expr::App(_, _)));
    }
}
