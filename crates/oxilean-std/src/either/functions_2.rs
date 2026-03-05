//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::functions::*;
use super::types::*;

/// Register all extended Either axioms into an environment.
///
/// This is a separate registration function from `build_either_env` and
/// should be called after that function has set up the base Either type.
pub fn register_either_extended_axioms(env: &mut Environment) -> Result<(), String> {
    let t1 = ei_ext_type0();
    for name in [
        "Bool", "Option", "List", "Prod", "Result", "Sum", "Void", "Nat", "Eq",
    ] {
        let _ = env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty: t1.clone(),
        });
    }
    ei_coproduct_inl(env)?;
    ei_coproduct_inr(env)?;
    ei_coproduct_universal(env)?;
    ei_bimap_id(env)?;
    ei_bimap_comp(env)?;
    ei_bifunctor_left_comp(env)?;
    ei_bifunctor_right_comp(env)?;
    ei_monad_pure(env)?;
    ei_monad_bind(env)?;
    ei_monad_left_id(env)?;
    ei_monad_right_id(env)?;
    ei_monad_assoc(env)?;
    ei_applicative_ap(env)?;
    ei_applicative_hom(env)?;
    ei_applicative_interchange(env)?;
    ei_alternative_alt(env)?;
    ei_iso_result_to(env)?;
    ei_iso_result_from(env)?;
    ei_iso_sum_to(env)?;
    ei_iso_sum_from(env)?;
    ei_traversable_traverse(env)?;
    ei_traversable_law_pure(env)?;
    ei_traversable_law_naturality(env)?;
    ei_foldable_foldl(env)?;
    ei_foldable_foldr(env)?;
    ei_profunctor_dimap(env)?;
    ei_partition_lefts(env)?;
    ei_partition_rights(env)?;
    ei_elim(env)?;
    ei_swap_involution(env)?;
    ei_tagged_union_tag(env)?;
    ei_error_catch_left(env)?;
    ei_commutativity_iso(env)?;
    ei_associativity_iso(env)?;
    ei_assoc_left(env)?;
    ei_assoc_right(env)?;
    ei_void_elim_left(env)?;
    ei_void_intro_left(env)?;
    ei_distrib_over_prod(env)?;
    ei_distrib_left(env)?;
    ei_do_seq_bind(env)?;
    ei_kleisli_comp(env)?;
    ei_kleisli_id(env)?;
    ei_eithert_run(env)?;
    ei_eithert_lift(env)?;
    ei_eithert_bind(env)?;
    ei_sequence_list(env)?;
    ei_traverse_list(env)?;
    ei_select_combinator(env)?;
    ei_select_law_right(env)?;
    ei_select_law_left(env)?;
    ei_nat_sum(env)?;
    ei_map_both(env)?;
    ei_join_with(env)?;
    Ok(())
}
#[cfg(test)]
mod either_extended_axiom_tests {
    use super::*;
    fn setup() -> Environment {
        let mut env = Environment::new();
        build_either_env(&mut env).expect("build_either_env should succeed");
        env
    }
    #[test]
    fn test_register_extended_axioms_ok() {
        let mut env = setup();
        assert!(register_either_extended_axioms(&mut env).is_ok());
    }
    #[test]
    fn test_coproduct_injections_present() {
        let mut env = setup();
        register_either_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Either.inl")).is_some());
        assert!(env.get(&Name::str("Either.inr")).is_some());
        assert!(env.get(&Name::str("Either.coprod")).is_some());
    }
    #[test]
    fn test_monad_axioms_present() {
        let mut env = setup();
        register_either_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Either.pure")).is_some());
        assert!(env.get(&Name::str("Either.bind")).is_some());
        assert!(env.get(&Name::str("Either.bind_pure_left")).is_some());
        assert!(env.get(&Name::str("Either.bind_pure_right")).is_some());
        assert!(env.get(&Name::str("Either.bind_assoc")).is_some());
    }
    #[test]
    fn test_applicative_axioms_present() {
        let mut env = setup();
        register_either_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Either.ap")).is_some());
        assert!(env.get(&Name::str("Either.ap_hom")).is_some());
        assert!(env.get(&Name::str("Either.ap_interchange")).is_some());
    }
    #[test]
    fn test_iso_axioms_present() {
        let mut env = setup();
        register_either_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Either.toResult")).is_some());
        assert!(env.get(&Name::str("Either.fromResult")).is_some());
        assert!(env.get(&Name::str("Either.toSum")).is_some());
        assert!(env.get(&Name::str("Either.fromSum")).is_some());
    }
    #[test]
    fn test_kleisli_axioms_present() {
        let mut env = setup();
        register_either_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Either.kleisliComp")).is_some());
        assert!(env.get(&Name::str("Either.kleisli_id_law")).is_some());
    }
    #[test]
    fn test_eithert_axioms_present() {
        let mut env = setup();
        register_either_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("EitherT.run")).is_some());
        assert!(env.get(&Name::str("EitherT.lift")).is_some());
        assert!(env.get(&Name::str("EitherT.bind")).is_some());
    }
    #[test]
    fn test_select_axioms_present() {
        let mut env = setup();
        register_either_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Either.select")).is_some());
        assert!(env.get(&Name::str("Either.select_right_law")).is_some());
        assert!(env.get(&Name::str("Either.select_left_law")).is_some());
    }
    #[test]
    fn test_swap_involution_axiom_present() {
        let mut env = setup();
        register_either_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Either.swap_involution")).is_some());
    }
    #[test]
    fn test_void_axioms_present() {
        let mut env = setup();
        register_either_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Either.elimVoidLeft")).is_some());
        assert!(env.get(&Name::str("Either.introVoidLeft")).is_some());
    }
    #[test]
    fn test_either_partition_struct() {
        let items: Vec<OxiEither<i32, &str>> = vec![
            OxiEither::Left(1),
            OxiEither::Right("err"),
            OxiEither::Left(2),
        ];
        let p = EitherPartition::from_iter(items);
        assert_eq!(p.lefts, vec![1, 2]);
        assert_eq!(p.rights, vec!["err"]);
        assert_eq!(p.total(), 3);
        assert!(!p.no_lefts());
        assert!(!p.no_rights());
    }
    #[test]
    fn test_either_partition_left_ratio() {
        let items: Vec<OxiEither<i32, i32>> = vec![
            OxiEither::Left(1),
            OxiEither::Left(2),
            OxiEither::Right(3),
            OxiEither::Right(4),
        ];
        let p = EitherPartition::from_iter(items);
        assert!((p.left_ratio() - 0.5).abs() < 1e-10);
    }
    #[test]
    fn test_either_traversal_ok() {
        let mut t: EitherTraversal<&str, i32> = EitherTraversal::new();
        t.step(OxiEither::Left(1));
        t.step(OxiEither::Left(2));
        assert!(!t.has_error());
        assert_eq!(t.success_count(), 2);
        let result = t.finish();
        assert_eq!(result, OxiEither::Left(vec![1, 2]));
    }
    #[test]
    fn test_either_traversal_error() {
        let mut t: EitherTraversal<&str, i32> = EitherTraversal::new();
        t.step(OxiEither::Left(1));
        t.step(OxiEither::Right("oops"));
        t.step(OxiEither::Left(3));
        assert!(t.has_error());
        let result = t.finish();
        assert_eq!(result, OxiEither::Right("oops"));
    }
    #[test]
    fn test_either_t_monad_pure() {
        struct Dummy;
        let m: EitherTMonad<Dummy, &str, i32> = EitherTMonad::pure(42);
        let result = m.run();
        assert_eq!(result, Some(OxiEither::Left(42)));
    }
    #[test]
    fn test_either_t_monad_throw() {
        struct Dummy;
        let m: EitherTMonad<Dummy, &str, i32> = EitherTMonad::throw("error");
        let result = m.run();
        assert_eq!(result, Some(OxiEither::Right("error")));
    }
    #[test]
    fn test_either_t_monad_bind_ok() {
        struct Dummy;
        let m: EitherTMonad<Dummy, &str, i32> = EitherTMonad::pure(5);
        let result = m.bind(|x| EitherTMonad::pure(x * 2)).run();
        assert_eq!(result, Some(OxiEither::Left(10)));
    }
    #[test]
    fn test_either_t_monad_bind_error() {
        struct Dummy;
        let m: EitherTMonad<Dummy, &str, i32> = EitherTMonad::throw("fail");
        let result = m
            .bind(|x: i32| EitherTMonad::<Dummy, &str, i32>::pure(x * 2))
            .run();
        assert_eq!(result, Some(OxiEither::Right("fail")));
    }
    #[test]
    fn test_either_t_monad_map() {
        struct Dummy;
        let m: EitherTMonad<Dummy, &str, i32> = EitherTMonad::pure(3);
        let result = m.map(|x| x + 1).run();
        assert_eq!(result, Some(OxiEither::Left(4)));
    }
    #[test]
    fn test_either_kleisli_apply() {
        let k: EitherKleisli<&str, i32, i32> = EitherKleisli::new(|x| {
            if x > 0 {
                OxiEither::Left(x * 2)
            } else {
                OxiEither::Right("neg")
            }
        });
        assert_eq!(k.apply(5), OxiEither::Left(10));
        assert_eq!(k.apply(-1), OxiEither::Right("neg"));
    }
    #[test]
    fn test_either_kleisli_compose() {
        let f: EitherKleisli<&str, i32, i32> = EitherKleisli::new(|x| OxiEither::Left(x + 1));
        let g: EitherKleisli<&str, i32, i32> = EitherKleisli::new(|x| OxiEither::Left(x * 2));
        let h = f.compose(g);
        assert_eq!(h.apply(3), OxiEither::Left(8));
    }
    #[test]
    fn test_either_kleisli_compose_error() {
        let f: EitherKleisli<&str, i32, i32> = EitherKleisli::new(|_| OxiEither::Right("err"));
        let g: EitherKleisli<&str, i32, i32> = EitherKleisli::new(|x| OxiEither::Left(x * 2));
        let h = f.compose(g);
        assert_eq!(h.apply(5), OxiEither::Right("err"));
    }
    #[test]
    fn test_either_kleisli_lift_fn() {
        let k: EitherKleisli<&str, i32, i32> = EitherKleisli::lift_fn(|x| x + 100);
        assert_eq!(k.apply(5), OxiEither::Left(105));
    }
    #[test]
    fn test_select_combinator_left_left() {
        let lhs: OxiEither<i32, &str> = OxiEither::Left(42);
        let f: Box<dyn Fn(i32) -> i32> = Box::new(|x| x + 1);
        let rhs: OxiEither<Box<dyn Fn(i32) -> i32>, &str> = OxiEither::Left(f);
        let sel: SelectCombinator<&str, i32, i32> = SelectCombinator::new(lhs, rhs);
        assert_eq!(sel.select(), OxiEither::Left(43));
    }
    #[test]
    fn test_select_combinator_left_right() {
        let lhs: OxiEither<i32, &str> = OxiEither::Left(42);
        let rhs: OxiEither<Box<dyn Fn(i32) -> i32>, &str> = OxiEither::Right("err");
        let sel: SelectCombinator<&str, i32, i32> = SelectCombinator::new(lhs, rhs);
        assert_eq!(sel.select(), OxiEither::Right("err"));
    }
    #[test]
    fn test_select_combinator_right() {
        let lhs: OxiEither<i32, &str> = OxiEither::Right("e");
        let f: Box<dyn Fn(i32) -> i32> = Box::new(|x| x);
        let rhs: OxiEither<Box<dyn Fn(i32) -> i32>, &str> = OxiEither::Left(f);
        let sel: SelectCombinator<&str, i32, i32> = SelectCombinator::new(lhs, rhs);
        assert_eq!(sel.select(), OxiEither::Right("e"));
    }
    #[test]
    fn test_select_right_law_short_circuits() {
        let result: OxiEither<i32, &str> =
            SelectCombinator::<&str, i32, i32>::select_right_law("e");
        assert_eq!(result, OxiEither::Right("e"));
    }
    #[test]
    fn test_bimap_id_axiom_present() {
        let mut env = setup();
        register_either_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Either.bimap_id")).is_some());
    }
    #[test]
    fn test_profunctor_dimap_axiom_present() {
        let mut env = setup();
        register_either_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Either.dimap")).is_some());
    }
    #[test]
    fn test_partition_lefts_rights_axioms_present() {
        let mut env = setup();
        register_either_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Either.lefts")).is_some());
        assert!(env.get(&Name::str("Either.rights")).is_some());
    }
    #[test]
    fn test_assoc_axioms_present() {
        let mut env = setup();
        register_either_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Either.assocLeft")).is_some());
        assert!(env.get(&Name::str("Either.assocRight")).is_some());
    }
    #[test]
    fn test_traverse_list_axiom_present() {
        let mut env = setup();
        register_either_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Either.traverseList")).is_some());
    }
    #[test]
    fn test_join_with_axiom_present() {
        let mut env = setup();
        register_either_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Either.joinWith")).is_some());
    }
    #[test]
    fn test_nat_sum_axiom_present() {
        let mut env = setup();
        register_either_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Either.natSum")).is_some());
    }
    #[test]
    fn test_either_partition_empty() {
        let items: Vec<OxiEither<i32, &str>> = vec![];
        let p = EitherPartition::from_iter(items);
        assert!(p.no_lefts());
        assert!(p.no_rights());
        assert_eq!(p.total(), 0);
        assert!((p.left_ratio() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_either_traversal_default() {
        let t: EitherTraversal<&str, i32> = EitherTraversal::default();
        assert!(!t.has_error());
        assert_eq!(t.success_count(), 0);
    }
}
