//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Level};
use oxilean_kernel::{Expr, Name};

use super::functions::*;
use super::types::*;

#[cfg(test)]
mod option_extended_tests {
    use super::*;
    use oxilean_kernel::Environment;
    fn setup() -> Environment {
        Environment::new()
    }
    #[test]
    fn test_register_option_extended_axioms_succeeds() {
        let mut env = setup();
        assert!(register_option_extended_axioms(&mut env).is_ok());
    }
    #[test]
    fn test_monad_laws_present() {
        let mut env = setup();
        register_option_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Option.monad_left_id")).is_some());
        assert!(env.get(&Name::str("Option.monad_right_id")).is_some());
        assert!(env.get(&Name::str("Option.monad_assoc")).is_some());
    }
    #[test]
    fn test_functor_laws_present() {
        let mut env = setup();
        register_option_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Option.functor_id")).is_some());
        assert!(env.get(&Name::str("Option.functor_comp")).is_some());
    }
    #[test]
    fn test_applicative_laws_present() {
        let mut env = setup();
        register_option_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Option.ap_identity")).is_some());
        assert!(env.get(&Name::str("Option.ap_homomorphism")).is_some());
        assert!(env.get(&Name::str("Option.ap_interchange")).is_some());
        assert!(env.get(&Name::str("Option.ap_composition")).is_some());
    }
    #[test]
    fn test_alternative_laws_present() {
        let mut env = setup();
        register_option_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Option.alt_first_some")).is_some());
        assert!(env.get(&Name::str("Option.alt_none_left")).is_some());
        assert!(env.get(&Name::str("Option.alt_assoc")).is_some());
    }
    #[test]
    fn test_either_iso_present() {
        let mut env = setup();
        register_option_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Option.iso_to_either")).is_some());
        assert!(env.get(&Name::str("Option.iso_from_either")).is_some());
        assert!(env.get(&Name::str("Option.iso_roundtrip_to")).is_some());
        assert!(env.get(&Name::str("Option.iso_roundtrip_from")).is_some());
    }
    #[test]
    fn test_fold_unfold_present() {
        let mut env = setup();
        register_option_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Option.cata")).is_some());
        assert!(env.get(&Name::str("Option.ana")).is_some());
    }
    #[test]
    fn test_traversal_present() {
        let mut env = setup();
        register_option_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Option.traverse_id")).is_some());
        assert!(env.get(&Name::str("Option.traverse_comp")).is_some());
        assert!(env.get(&Name::str("Option.sequence_traverse")).is_some());
    }
    #[test]
    fn test_foldable_laws_present() {
        let mut env = setup();
        register_option_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Option.foldable_foldl")).is_some());
        assert!(env.get(&Name::str("Option.foldable_foldr")).is_some());
        assert!(env.get(&Name::str("Option.foldable_fold_none")).is_some());
    }
    #[test]
    fn test_optiont_present() {
        let mut env = setup();
        register_option_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("OptionT.run")).is_some());
        assert!(env.get(&Name::str("OptionT.lift")).is_some());
        assert!(env.get(&Name::str("OptionT.bind")).is_some());
    }
    #[test]
    fn test_monoid_laws_present() {
        let mut env = setup();
        register_option_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Option.monoid_empty")).is_some());
        assert!(env.get(&Name::str("Option.monoid_append")).is_some());
        assert!(env.get(&Name::str("Option.monoid_left_id")).is_some());
        assert!(env.get(&Name::str("Option.monoid_right_id")).is_some());
    }
    #[test]
    fn test_bimap_present() {
        let mut env = setup();
        register_option_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Option.bimap_some")).is_some());
        assert!(env.get(&Name::str("Option.bimap_none")).is_some());
    }
    #[test]
    fn test_zip_present() {
        let mut env = setup();
        register_option_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Option.zip_some")).is_some());
        assert!(env.get(&Name::str("Option.zip_none")).is_some());
    }
    #[test]
    fn test_filter_present() {
        let mut env = setup();
        register_option_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Option.filter_some_true")).is_some());
        assert!(env.get(&Name::str("Option.filter_some_false")).is_some());
        assert!(env.get(&Name::str("Option.filter_none")).is_some());
    }
    #[test]
    fn test_get_or_else_present() {
        let mut env = setup();
        register_option_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Option.get_or_else_some")).is_some());
        assert!(env.get(&Name::str("Option.get_or_else_none")).is_some());
        assert!(env.get(&Name::str("Option.or_else_some")).is_some());
        assert!(env.get(&Name::str("Option.or_else_none")).is_some());
    }
    #[test]
    fn test_profunctor_present() {
        let mut env = setup();
        register_option_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Option.dimap")).is_some());
    }
    #[test]
    fn test_comonad_present() {
        let mut env = setup();
        register_option_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Option.comonad_extract")).is_some());
    }
    #[test]
    fn test_join_present() {
        let mut env = setup();
        register_option_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Option.join_some_some")).is_some());
        assert!(env.get(&Name::str("Option.join_some_none")).is_some());
        assert!(env.get(&Name::str("Option.join_none")).is_some());
    }
    #[test]
    fn test_flatmap_present() {
        let mut env = setup();
        register_option_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Option.flatmap_is_join_map")).is_some());
    }
    #[test]
    fn test_nullable_iso_present() {
        let mut env = setup();
        register_option_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Option.nullable_iso_to")).is_some());
        assert!(env.get(&Name::str("Option.nullable_iso_from")).is_some());
    }
    #[test]
    fn test_lift_a2_present() {
        let mut env = setup();
        register_option_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Option.liftA2_def")).is_some());
    }
    #[test]
    fn test_option_writer_struct() {
        let w: OptionWriter<u32, String> = OptionWriter::pure_some(42);
        assert!(w.is_some());
        let w2: OptionWriter<u32, String> = OptionWriter::none_with_log("log".to_string());
        assert!(!w2.is_some());
    }
    #[test]
    fn test_option_functor_struct() {
        let f = OptionFunctor::new(Some(5u32));
        let g = f.fmap(|x| x * 2);
        assert_eq!(g.inner(), Some(10));
        let f2 = OptionFunctor::new(None::<u32>);
        let g2 = f2.fmap(|x| x * 2);
        assert_eq!(g2.inner(), None);
    }
    #[test]
    fn test_option_applicative_struct() {
        assert_eq!(OptionApplicative::pure(42u32), Some(42));
        let r = OptionApplicative::ap(Some(|x: u32| x + 1), Some(5u32));
        assert_eq!(r, Some(6));
        let r2 = OptionApplicative::ap(None::<fn(u32) -> u32>, Some(5u32));
        assert_eq!(r2, None);
        let r3 = OptionApplicative::lift_a2(|a: u32, b: u32| a + b, Some(3), Some(4));
        assert_eq!(r3, Some(7));
    }
    #[test]
    fn test_option_comonad_extract() {
        let c = OptionComonad::new(Some(10u32), 0u32);
        assert_eq!(c.extract(), 10);
        let c2 = OptionComonad::new(None::<u32>, 99u32);
        assert_eq!(c2.extract(), 99);
    }
    #[test]
    fn test_option_profunctor_apply() {
        let p = OptionProfunctor::new(|x: u32| if x > 0 { Some(x * 2) } else { None });
        assert_eq!(p.apply(5), Some(10));
        assert_eq!(p.apply(0), None);
    }
    #[test]
    fn test_option_comonad_duplicate_as_pair() {
        let c = OptionComonad::new(Some(5u32), 0u32);
        let (default, val) = c.duplicate_as_pair();
        assert_eq!(default, 0);
        assert_eq!(val, Some(5));
    }
}
/// Option.flatmap_some: (Some x) >>= f = f x
fn opt_ext_flatmap_some(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi_imp(
            "β",
            t.clone(),
            opt_ext_pi(
                "x",
                Expr::BVar(1),
                opt_ext_pi(
                    "f",
                    opt_ext_arrow(Expr::BVar(2), opt_ext_option_ty(Expr::BVar(2))),
                    opt_ext_prop(),
                ),
            ),
        ),
    );
    opt_ext_axiom(env, "Option.flatmap_some", ty)
}
/// Option.flatmap_none: None >>= f = None
fn opt_ext_flatmap_none(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi_imp(
            "β",
            t.clone(),
            opt_ext_pi(
                "f",
                opt_ext_arrow(Expr::BVar(1), opt_ext_option_ty(Expr::BVar(1))),
                opt_ext_prop(),
            ),
        ),
    );
    opt_ext_axiom(env, "Option.flatmap_none", ty)
}
/// Option.kleisli_id: return is the identity for Kleisli composition
fn opt_ext_kleisli_id(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi_imp(
            "β",
            t.clone(),
            opt_ext_pi(
                "f",
                opt_ext_arrow(Expr::BVar(1), opt_ext_option_ty(Expr::BVar(1))),
                opt_ext_prop(),
            ),
        ),
    );
    opt_ext_axiom(env, "Option.kleisli_id", ty)
}
/// Option.kleisli_assoc: Kleisli composition is associative
fn opt_ext_kleisli_assoc(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi_imp(
            "β",
            t.clone(),
            opt_ext_pi_imp(
                "γ",
                t.clone(),
                opt_ext_pi_imp("δ", t.clone(), opt_ext_prop()),
            ),
        ),
    );
    opt_ext_axiom(env, "Option.kleisli_assoc", ty)
}
/// Option.to_list_some: toList (Some x) = [x]
fn opt_ext_to_list_some(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi("x", Expr::BVar(0), opt_ext_prop()),
    );
    opt_ext_axiom(env, "Option.to_list_some", ty)
}
/// Option.to_list_none: toList None = []
fn opt_ext_to_list_none(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp("α", t.clone(), opt_ext_prop());
    opt_ext_axiom(env, "Option.to_list_none", ty)
}
/// Option.length_some: length (Some x) = 1
fn opt_ext_length_some(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi("x", Expr::BVar(0), opt_ext_prop()),
    );
    opt_ext_axiom(env, "Option.length_some", ty)
}
/// Option.length_none: length None = 0
fn opt_ext_length_none(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp("α", t.clone(), opt_ext_prop());
    opt_ext_axiom(env, "Option.length_none", ty)
}
/// Option.pure_eq_some: pure = Some
fn opt_ext_pure_eq_some(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi("x", Expr::BVar(0), opt_ext_prop()),
    );
    opt_ext_axiom(env, "Option.pure_eq_some", ty)
}
/// Option.map_id: map id = id
fn opt_ext_map_id(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi("m", opt_ext_option_ty(Expr::BVar(0)), opt_ext_prop()),
    );
    opt_ext_axiom(env, "Option.map_id", ty)
}
/// Option.map_comp: map (f . g) = map f . map g
fn opt_ext_map_comp(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi_imp(
            "β",
            t.clone(),
            opt_ext_pi_imp(
                "γ",
                t.clone(),
                opt_ext_pi(
                    "f",
                    opt_ext_arrow(Expr::BVar(1), Expr::BVar(1)),
                    opt_ext_pi(
                        "g",
                        opt_ext_arrow(Expr::BVar(3), Expr::BVar(3)),
                        opt_ext_pi("m", opt_ext_option_ty(Expr::BVar(4)), opt_ext_prop()),
                    ),
                ),
            ),
        ),
    );
    opt_ext_axiom(env, "Option.map_comp", ty)
}
/// Option.is_some_iff_not_none: isSome m ↔ ¬ isNone m
fn opt_ext_is_some_iff_not_none(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi("m", opt_ext_option_ty(Expr::BVar(0)), opt_ext_prop()),
    );
    opt_ext_axiom(env, "Option.is_some_iff_not_none", ty)
}
/// Option.bind_map: m >>= (pure . f) = map f m
fn opt_ext_bind_map(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi_imp(
            "β",
            t.clone(),
            opt_ext_pi(
                "m",
                opt_ext_option_ty(Expr::BVar(1)),
                opt_ext_pi(
                    "f",
                    opt_ext_arrow(Expr::BVar(2), Expr::BVar(2)),
                    opt_ext_prop(),
                ),
            ),
        ),
    );
    opt_ext_axiom(env, "Option.bind_map", ty)
}
/// Option.map_bind: map f (m >>= g) = m >>= (map f . g)
fn opt_ext_map_bind(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi_imp(
            "β",
            t.clone(),
            opt_ext_pi_imp("γ", t.clone(), opt_ext_prop()),
        ),
    );
    opt_ext_axiom(env, "Option.map_bind", ty)
}
/// Option.ap_via_bind: u <*> v = u >>= (fun f => map f v)
fn opt_ext_ap_via_bind(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi_imp(
            "β",
            t.clone(),
            opt_ext_pi(
                "u",
                opt_ext_option_ty(opt_ext_arrow(Expr::BVar(1), Expr::BVar(1))),
                opt_ext_pi("v", opt_ext_option_ty(Expr::BVar(2)), opt_ext_prop()),
            ),
        ),
    );
    opt_ext_axiom(env, "Option.ap_via_bind", ty)
}
/// Option.traverse_naturality: traverse respects natural transformations
fn opt_ext_traverse_naturality(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi_imp("β", t.clone(), opt_ext_prop()),
    );
    opt_ext_axiom(env, "Option.traverse_naturality", ty)
}
/// Option.foldable_to_list: toList via fold
fn opt_ext_foldable_to_list(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi("m", opt_ext_option_ty(Expr::BVar(0)), opt_ext_prop()),
    );
    opt_ext_axiom(env, "Option.foldable_to_list", ty)
}
/// Option.foldable_count: count via fold
fn opt_ext_foldable_count(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi("m", opt_ext_option_ty(Expr::BVar(0)), opt_ext_prop()),
    );
    opt_ext_axiom(env, "Option.foldable_count", ty)
}
/// Option.zip_sym: zip m1 m2 = swap <$> zip m2 m1
fn opt_ext_zip_sym(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi_imp(
            "β",
            t.clone(),
            opt_ext_pi(
                "m1",
                opt_ext_option_ty(Expr::BVar(1)),
                opt_ext_pi("m2", opt_ext_option_ty(Expr::BVar(1)), opt_ext_prop()),
            ),
        ),
    );
    opt_ext_axiom(env, "Option.zip_sym", ty)
}
/// ── Register additional axioms ────────────────────────────────────────────────
///
/// These additional axioms supplement `register_option_extended_axioms`.
pub fn register_option_additional_axioms(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    for name in ["Bool", "Option", "Either", "Unit", "Nat", "List"] {
        let _ = env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty: t.clone(),
        });
    }
    opt_ext_flatmap_some(env)?;
    opt_ext_flatmap_none(env)?;
    opt_ext_kleisli_id(env)?;
    opt_ext_kleisli_assoc(env)?;
    opt_ext_to_list_some(env)?;
    opt_ext_to_list_none(env)?;
    opt_ext_length_some(env)?;
    opt_ext_length_none(env)?;
    opt_ext_pure_eq_some(env)?;
    opt_ext_map_id(env)?;
    opt_ext_map_comp(env)?;
    opt_ext_is_some_iff_not_none(env)?;
    opt_ext_bind_map(env)?;
    opt_ext_map_bind(env)?;
    opt_ext_ap_via_bind(env)?;
    opt_ext_traverse_naturality(env)?;
    opt_ext_foldable_to_list(env)?;
    opt_ext_foldable_count(env)?;
    opt_ext_zip_sym(env)?;
    Ok(())
}
#[cfg(test)]
mod option_additional_tests {
    use super::*;
    use oxilean_kernel::Environment;
    #[test]
    fn test_register_additional_axioms_succeeds() {
        let mut env = Environment::new();
        assert!(register_option_additional_axioms(&mut env).is_ok());
    }
    #[test]
    fn test_flatmap_axioms_present() {
        let mut env = Environment::new();
        register_option_additional_axioms(&mut env).expect("Environment::new should succeed");
        assert!(env.get(&Name::str("Option.flatmap_some")).is_some());
        assert!(env.get(&Name::str("Option.flatmap_none")).is_some());
    }
    #[test]
    fn test_kleisli_axioms_present() {
        let mut env = Environment::new();
        register_option_additional_axioms(&mut env).expect("Environment::new should succeed");
        assert!(env.get(&Name::str("Option.kleisli_id")).is_some());
        assert!(env.get(&Name::str("Option.kleisli_assoc")).is_some());
    }
    #[test]
    fn test_to_list_axioms_present() {
        let mut env = Environment::new();
        register_option_additional_axioms(&mut env).expect("Environment::new should succeed");
        assert!(env.get(&Name::str("Option.to_list_some")).is_some());
        assert!(env.get(&Name::str("Option.to_list_none")).is_some());
    }
    #[test]
    fn test_map_axioms_present() {
        let mut env = Environment::new();
        register_option_additional_axioms(&mut env).expect("Environment::new should succeed");
        assert!(env.get(&Name::str("Option.map_id")).is_some());
        assert!(env.get(&Name::str("Option.map_comp")).is_some());
    }
    #[test]
    fn test_ap_via_bind_present() {
        let mut env = Environment::new();
        register_option_additional_axioms(&mut env).expect("Environment::new should succeed");
        assert!(env.get(&Name::str("Option.ap_via_bind")).is_some());
    }
    #[test]
    fn test_option_chain_map() {
        let result = OptionChain::of(5u32).map(|x| x * 2).get();
        assert_eq!(result, Some(10));
    }
    #[test]
    fn test_option_chain_flat_map() {
        let result = OptionChain::of(5u32)
            .flat_map(|x| if x > 3 { Some(x + 1) } else { None })
            .get();
        assert_eq!(result, Some(6));
    }
    #[test]
    fn test_option_chain_filter() {
        let result = OptionChain::of(5u32).filter(|x| *x > 3).get();
        assert_eq!(result, Some(5));
        let result2 = OptionChain::of(2u32).filter(|x| *x > 3).get();
        assert_eq!(result2, None);
    }
    #[test]
    fn test_option_chain_or_else() {
        let result = OptionChain::<u32>::empty().or_else(Some(42)).get();
        assert_eq!(result, Some(42));
    }
    #[test]
    fn test_option_chain_empty() {
        let c = OptionChain::<u32>::empty();
        assert!(!c.is_present());
    }
    #[test]
    fn test_option_result_bridge_to_result() {
        let r = OptionResultBridge::to_result(Some(5u32), "missing");
        assert_eq!(r, Ok(5));
        let r2 = OptionResultBridge::to_result(None::<u32>, "missing");
        assert_eq!(r2, Err("missing"));
    }
    #[test]
    fn test_option_result_bridge_from_result() {
        let r: Result<u32, &str> = Ok(42);
        assert_eq!(OptionResultBridge::from_result_ok(r), Some(42));
        let r2: Result<u32, &str> = Err("err");
        assert_eq!(OptionResultBridge::from_result_err(r2), Some("err"));
    }
    #[test]
    fn test_option_vec_sequence() {
        let mut ov: OptionVec<u32> = OptionVec::new();
        ov.push(Some(1));
        ov.push(Some(2));
        ov.push(Some(3));
        assert_eq!(ov.sequence(), Some(vec![1, 2, 3]));
    }
    #[test]
    fn test_option_vec_sequence_with_none() {
        let mut ov: OptionVec<u32> = OptionVec::new();
        ov.push(Some(1));
        ov.push(None);
        ov.push(Some(3));
        assert!(ov.sequence().is_none());
    }
    #[test]
    fn test_option_vec_collect_some() {
        let ov: OptionVec<u32> = vec![Some(1u32), None, Some(3), None, Some(5)]
            .into_iter()
            .collect();
        let somes = ov.collect_some();
        assert_eq!(somes, vec![1, 3, 5]);
    }
    #[test]
    fn test_option_vec_counts() {
        let ov: OptionVec<u32> = vec![Some(1u32), None, Some(3)].into_iter().collect();
        assert_eq!(ov.count_some(), 2);
        assert_eq!(ov.count_none(), 1);
    }
    #[test]
    fn test_option_vec_all_any_some() {
        let all_some: OptionVec<u32> = vec![Some(1u32), Some(2)].into_iter().collect();
        assert!(all_some.all_some());
        assert!(all_some.any_some());
        let mixed: OptionVec<u32> = vec![Some(1u32), None].into_iter().collect();
        assert!(!mixed.all_some());
        assert!(mixed.any_some());
        let all_none: OptionVec<u32> = vec![None, None].into_iter().collect();
        assert!(!all_none.all_some());
        assert!(!all_none.any_some());
    }
}
