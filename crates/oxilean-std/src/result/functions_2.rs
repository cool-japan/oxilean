//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::functions::*;

/// `Result.from_option_none : fromOption None e = err e`
fn res_ext2_build_from_option_none(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.from_option_none", env)
}
/// `Result.to_option_ok : toOption (ok v) = Some v`
fn res_ext2_build_to_option_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall_type_axiom("Result.to_option_ok", env)
}
/// `Result.to_option_err : toOption (err e) = None`
fn res_ext2_build_to_option_err(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall_type_axiom("Result.to_option_err", env)
}
/// `Result.contains_ok : contains (ok v) x = (v == x)`
fn res_ext2_build_contains_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.contains_ok", env)
}
/// `Result.contains_err : contains (err e) x = false`
fn res_ext2_build_contains_err(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.contains_err", env)
}
/// `Result.all_ok : all p (ok v) = p v`
fn res_ext2_build_all_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.all_ok", env)
}
/// `Result.all_err : all p (err e) = true` (vacuously true)
fn res_ext2_build_all_err(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.all_err", env)
}
/// `Result.any_ok : any p (ok v) = p v`
fn res_ext2_build_any_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.any_ok", env)
}
/// `Result.any_err : any p (err e) = false`
fn res_ext2_build_any_err(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.any_err", env)
}
/// `Result.iter_ok : iter (ok v) = \[v\]`
fn res_ext2_build_iter_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall_type_axiom("Result.iter_ok", env)
}
/// `Result.iter_err : iter (err e) = []`
fn res_ext2_build_iter_err(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall_type_axiom("Result.iter_err", env)
}
/// `Result.expect_ok : expect (ok v) msg = v`
fn res_ext2_build_expect_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall_type_axiom("Result.expect_ok", env)
}
/// `Result.filter_ok_true : filterOk (ok v) p = if p v then ok v else err _`
fn res_ext2_build_filter_ok_true(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.filter_ok_true", env)
}
/// `Result.partition_results_split : partitionResults \[ok a, err b, ok c\] = (\[a,c\], \[b\])`
fn res_ext2_build_partition_results_split(
    env: &mut Environment,
) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.partition_results_split", env)
}
/// `Result.collect_all_ok : collectAll \[ok a, ok b, ok c\] = ok \[a, b, c\]`
fn res_ext2_build_collect_all_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.collect_all_ok", env)
}
/// `Result.collect_first_err : collectAll \[ok a, err b, ok c\] = err b`
fn res_ext2_build_collect_first_err(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.collect_first_err", env)
}
/// `Result.inspect_ok : inspect (ok v) f = (f v; ok v)`
fn res_ext2_build_inspect_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.inspect_ok", env)
}
/// `Result.inspect_err_ok : inspectErr (ok v) f = ok v` (f not called)
fn res_ext2_build_inspect_err_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.inspect_err_ok", env)
}
/// `Result.and_ok_ok : and (ok v1) (ok v2) = ok v2`
fn res_ext2_build_and_ok_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.and_ok_ok", env)
}
/// `Result.and_err_left : and (err e) r = err e`
fn res_ext2_build_and_err_left(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.and_err_left", env)
}
/// `Result.or_ok_left : or (ok v) r = ok v`
fn res_ext2_build_or_ok_left(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.or_ok_left", env)
}
/// `Result.or_err_right : or (err e1) (err e2) = err e2`
fn res_ext2_build_or_err_right(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.or_err_right", env)
}
/// `Result.flatten_ok_identity : flatten (map ok r) = r`
fn res_ext2_build_flatten_ok_identity(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.flatten_ok_identity", env)
}
/// `Result.map_id_is_id : map id = id` (functor identity at the function level)
fn res_ext2_build_map_id_is_id(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall_type_axiom("Result.map_id_is_id", env)
}
/// `Result.bimap_map_map_err : bimap f g r = mapErr g (map f r)`
fn res_ext2_build_bimap_via_map_map_err(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.bimap_via_map_map_err", env)
}
/// `Result.getOrElse_ok : getOrElse (ok v) default = v`
fn res_ext2_build_get_or_else_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.getOrElse_ok_axiom2", env)
}
/// `Result.getOrElse_err : getOrElse (err e) default = default`
fn res_ext2_build_get_or_else_err(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.getOrElse_err_axiom2", env)
}
/// `Result.unwrap_ok : unwrap (ok v) = v`  (partial — total only when Ok)
fn res_ext2_build_unwrap_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall_type_axiom("Result.unwrap_ok", env)
}
/// `Result.coerce_ok_err : coerce (ok v : Result T E1) : Result T E2 = ok v`
fn res_ext2_build_coerce_ok_err(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.coerce_ok_err", env)
}
/// `Result.traverse_id : traverse id r = r` (identity applicative)
fn res_ext2_build_traverse_id(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.traverse_id", env)
}
/// `Result.traverse_comp : traverse (f >=> g) = traverse g ∘ traverse f` (composition law)
fn res_ext2_build_traverse_comp(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.traverse_comp", env)
}
/// `Result.sequence_length : sequence xs where all ok → ok (map unwrap xs)`
fn res_ext2_build_sequence_length(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.sequence_length", env)
}
/// `Result.mapM_pure : mapM ok xs = ok xs`  (mapping with pure)
fn res_ext2_build_mapm_pure(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall_type_axiom("Result.mapM_pure", env)
}
/// `Result.kleisli_comp_assoc : (f >=> g) >=> h = f >=> (g >=> h)` (Kleisli category)
fn res_ext2_build_kleisli_comp_assoc(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.kleisli_comp_assoc", env)
}
/// `Result.kleisli_left_id : ok >=> f = f`
fn res_ext2_build_kleisli_left_id(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.kleisli_left_id", env)
}
/// `Result.kleisli_right_id : f >=> ok = f`
fn res_ext2_build_kleisli_right_id(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.kleisli_right_id", env)
}
/// `Result.cata_ana_id : cata (ana seed) = id` (hylomorphism identity)
fn res_ext2_build_cata_ana_id(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.cata_ana_id", env)
}
/// `Result.para_extends_cata : para is a generalization of cata`
fn res_ext2_build_para_extends_cata(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.para_extends_cata", env)
}
/// `Result.ap_map_equiv : ap (map (,) r1) r2 = andThen r1 (fun a => map (a,) r2)`
fn res_ext2_build_ap_map_equiv(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.ap_map_equiv", env)
}
/// `Result.selective_branch : branch (ok (Left a)) fl fr = map fl (ok a)`
fn res_ext2_build_selective_branch_left(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.selective_branch_left", env)
}
/// `Result.selective_branch_right : branch (ok (Right b)) fl fr = map fr (ok b)`
fn res_ext2_build_selective_branch_right(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.selective_branch_right", env)
}
/// `Result.writer_lift_ok : liftW (ok v) = ok (v, mempty)`
fn res_ext2_build_writer_lift_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.writer_lift_ok", env)
}
/// `Result.state_lift_ok : liftS (ok v) s = (ok v, s)` (state unchanged)
fn res_ext2_build_state_lift_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.state_lift_ok", env)
}
/// `Result.reader_lift_ok : liftR (ok v) env = ok v`
fn res_ext2_build_reader_lift_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.reader_lift_ok", env)
}
/// `Result.free_monad_embed : embed (ok v) = Pure v` in Free monad sense
fn res_ext2_build_free_monad_embed(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall_type_axiom("Result.free_monad_embed", env)
}
/// `Result.codensity_transform : Codensity (Result _ E) T ≅ Result T E`
fn res_ext2_build_codensity_iso(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.codensity_iso", env)
}
/// `Result.continuation_pure : (pure v >>= k) = k v` (CPS translation)
fn res_ext2_build_continuation_pure(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.continuation_pure", env)
}
/// `Result.church_encoding_ok : church(ok v) f g = f v`
fn res_ext2_build_church_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.church_ok", env)
}
/// `Result.church_encoding_err : church(err e) f g = g e`
fn res_ext2_build_church_err(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.church_err", env)
}
/// `Result.scott_encoding_ok : scott(ok v) f g = f v`
fn res_ext2_build_scott_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.scott_ok", env)
}
/// `Result.scott_encoding_err : scott(err e) f g = g e`
fn res_ext2_build_scott_err(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.scott_err", env)
}
/// Register the second batch of extended Result axioms.
///
/// Covers: optics (lens/prism), natural transformations, monad morphisms,
/// strength, distributive laws, pointed, swap, Option conversions, iterators,
/// boolean predicates, collection utilities, Kleisli category, hylomorphisms,
/// applicative variants, selective functors, monad transformer lifting,
/// free monad embedding, CPS/continuation, and Church/Scott encodings.
pub fn register_result_extended_axioms_part2(env: &mut Environment) {
    let builders: &[fn(&mut Environment) -> std::result::Result<(), String>] = &[
        res_ext2_build_strength_ok,
        res_ext2_build_strength_err,
        res_ext2_build_natural_transform_ok,
        res_ext2_build_natural_transform_err,
        res_ext2_build_monad_morphism_unit,
        res_ext2_build_monad_morphism_bind,
        res_ext2_build_distribute_list_result,
        res_ext2_build_pointed_pure,
        res_ext2_build_lens_get_ok,
        res_ext2_build_lens_set_ok,
        res_ext2_build_prism_review,
        res_ext2_build_prism_preview_ok,
        res_ext2_build_prism_preview_err,
        res_ext2_build_swap_ok,
        res_ext2_build_swap_err,
        res_ext2_build_swap_involution,
        res_ext2_build_unwrap_or_default_ok,
        res_ext2_build_ok_or_else,
        res_ext2_build_from_option_none,
        res_ext2_build_to_option_ok,
        res_ext2_build_to_option_err,
        res_ext2_build_contains_ok,
        res_ext2_build_contains_err,
        res_ext2_build_all_ok,
        res_ext2_build_all_err,
        res_ext2_build_any_ok,
        res_ext2_build_any_err,
        res_ext2_build_iter_ok,
        res_ext2_build_iter_err,
        res_ext2_build_expect_ok,
        res_ext2_build_filter_ok_true,
        res_ext2_build_partition_results_split,
        res_ext2_build_collect_all_ok,
        res_ext2_build_collect_first_err,
        res_ext2_build_inspect_ok,
        res_ext2_build_inspect_err_ok,
        res_ext2_build_and_ok_ok,
        res_ext2_build_and_err_left,
        res_ext2_build_or_ok_left,
        res_ext2_build_or_err_right,
        res_ext2_build_flatten_ok_identity,
        res_ext2_build_map_id_is_id,
        res_ext2_build_bimap_via_map_map_err,
        res_ext2_build_get_or_else_ok,
        res_ext2_build_get_or_else_err,
        res_ext2_build_unwrap_ok,
        res_ext2_build_coerce_ok_err,
        res_ext2_build_traverse_id,
        res_ext2_build_traverse_comp,
        res_ext2_build_sequence_length,
        res_ext2_build_mapm_pure,
        res_ext2_build_kleisli_comp_assoc,
        res_ext2_build_kleisli_left_id,
        res_ext2_build_kleisli_right_id,
        res_ext2_build_cata_ana_id,
        res_ext2_build_para_extends_cata,
        res_ext2_build_ap_map_equiv,
        res_ext2_build_selective_branch_left,
        res_ext2_build_selective_branch_right,
        res_ext2_build_writer_lift_ok,
        res_ext2_build_state_lift_ok,
        res_ext2_build_reader_lift_ok,
        res_ext2_build_free_monad_embed,
        res_ext2_build_codensity_iso,
        res_ext2_build_continuation_pure,
        res_ext2_build_church_ok,
        res_ext2_build_church_err,
        res_ext2_build_scott_ok,
        res_ext2_build_scott_err,
    ];
    for builder in builders {
        let _ = builder(env);
    }
}
#[cfg(test)]
mod result_extended_axiom_tests_part2 {
    use super::*;
    fn make_env() -> Environment {
        let mut env = Environment::new();
        let type1 = Expr::Sort(Level::succ(Level::zero()));
        env.add(Declaration::Axiom {
            name: Name::str("Bool"),
            univ_params: vec![],
            ty: type1,
        })
        .expect("operation should succeed");
        build_result_env(&mut env).expect("build_result_env should succeed");
        env
    }
    #[test]
    fn test_part2_register_runs() {
        let mut env = make_env();
        register_result_extended_axioms_part2(&mut env);
        assert!(env.get(&Name::str("Result.strength_ok")).is_some());
        assert!(env.get(&Name::str("Result.swap_involution")).is_some());
    }
    #[test]
    fn test_optics_laws_present() {
        let mut env = make_env();
        register_result_extended_axioms_part2(&mut env);
        assert!(env.get(&Name::str("Result.lens_get_ok")).is_some());
        assert!(env.get(&Name::str("Result.lens_set_ok")).is_some());
        assert!(env.get(&Name::str("Result.prism_review")).is_some());
        assert!(env.get(&Name::str("Result.prism_preview_ok")).is_some());
        assert!(env.get(&Name::str("Result.prism_preview_err")).is_some());
    }
    #[test]
    fn test_swap_laws_present() {
        let mut env = make_env();
        register_result_extended_axioms_part2(&mut env);
        assert!(env.get(&Name::str("Result.swap_ok")).is_some());
        assert!(env.get(&Name::str("Result.swap_err")).is_some());
        assert!(env.get(&Name::str("Result.swap_involution")).is_some());
    }
    #[test]
    fn test_option_conv_present() {
        let mut env = make_env();
        register_result_extended_axioms_part2(&mut env);
        assert!(env.get(&Name::str("Result.to_option_ok")).is_some());
        assert!(env.get(&Name::str("Result.to_option_err")).is_some());
        assert!(env.get(&Name::str("Result.from_option_none")).is_some());
        assert!(env.get(&Name::str("Result.ok_or_else")).is_some());
    }
    #[test]
    fn test_predicate_laws_present() {
        let mut env = make_env();
        register_result_extended_axioms_part2(&mut env);
        assert!(env.get(&Name::str("Result.all_ok")).is_some());
        assert!(env.get(&Name::str("Result.all_err")).is_some());
        assert!(env.get(&Name::str("Result.any_ok")).is_some());
        assert!(env.get(&Name::str("Result.any_err")).is_some());
        assert!(env.get(&Name::str("Result.contains_ok")).is_some());
        assert!(env.get(&Name::str("Result.contains_err")).is_some());
    }
    #[test]
    fn test_iterator_laws_present() {
        let mut env = make_env();
        register_result_extended_axioms_part2(&mut env);
        assert!(env.get(&Name::str("Result.iter_ok")).is_some());
        assert!(env.get(&Name::str("Result.iter_err")).is_some());
    }
    #[test]
    fn test_collection_laws_present() {
        let mut env = make_env();
        register_result_extended_axioms_part2(&mut env);
        assert!(env.get(&Name::str("Result.collect_all_ok")).is_some());
        assert!(env.get(&Name::str("Result.collect_first_err")).is_some());
        assert!(env
            .get(&Name::str("Result.partition_results_split"))
            .is_some());
    }
    #[test]
    fn test_kleisli_laws_present() {
        let mut env = make_env();
        register_result_extended_axioms_part2(&mut env);
        assert!(env.get(&Name::str("Result.kleisli_comp_assoc")).is_some());
        assert!(env.get(&Name::str("Result.kleisli_left_id")).is_some());
        assert!(env.get(&Name::str("Result.kleisli_right_id")).is_some());
    }
    #[test]
    fn test_traverse_laws_part2_present() {
        let mut env = make_env();
        register_result_extended_axioms_part2(&mut env);
        assert!(env.get(&Name::str("Result.traverse_id")).is_some());
        assert!(env.get(&Name::str("Result.traverse_comp")).is_some());
        assert!(env.get(&Name::str("Result.sequence_length")).is_some());
        assert!(env.get(&Name::str("Result.mapM_pure")).is_some());
    }
    #[test]
    fn test_church_scott_encoding_present() {
        let mut env = make_env();
        register_result_extended_axioms_part2(&mut env);
        assert!(env.get(&Name::str("Result.church_ok")).is_some());
        assert!(env.get(&Name::str("Result.church_err")).is_some());
        assert!(env.get(&Name::str("Result.scott_ok")).is_some());
        assert!(env.get(&Name::str("Result.scott_err")).is_some());
    }
    #[test]
    fn test_transformer_lifting_present() {
        let mut env = make_env();
        register_result_extended_axioms_part2(&mut env);
        assert!(env.get(&Name::str("Result.writer_lift_ok")).is_some());
        assert!(env.get(&Name::str("Result.state_lift_ok")).is_some());
        assert!(env.get(&Name::str("Result.reader_lift_ok")).is_some());
    }
    #[test]
    fn test_free_monad_and_cps_present() {
        let mut env = make_env();
        register_result_extended_axioms_part2(&mut env);
        assert!(env.get(&Name::str("Result.free_monad_embed")).is_some());
        assert!(env.get(&Name::str("Result.codensity_iso")).is_some());
        assert!(env.get(&Name::str("Result.continuation_pure")).is_some());
    }
    #[test]
    fn test_selective_functor_present() {
        let mut env = make_env();
        register_result_extended_axioms_part2(&mut env);
        assert!(env
            .get(&Name::str("Result.selective_branch_left"))
            .is_some());
        assert!(env
            .get(&Name::str("Result.selective_branch_right"))
            .is_some());
    }
    #[test]
    fn test_natural_transform_present() {
        let mut env = make_env();
        register_result_extended_axioms_part2(&mut env);
        assert!(env.get(&Name::str("Result.natural_transform_ok")).is_some());
        assert!(env
            .get(&Name::str("Result.natural_transform_err"))
            .is_some());
        assert!(env.get(&Name::str("Result.monad_morphism_unit")).is_some());
        assert!(env.get(&Name::str("Result.monad_morphism_bind")).is_some());
    }
    #[test]
    fn test_strength_laws_present() {
        let mut env = make_env();
        register_result_extended_axioms_part2(&mut env);
        assert!(env.get(&Name::str("Result.strength_ok")).is_some());
        assert!(env.get(&Name::str("Result.strength_err")).is_some());
    }
    #[test]
    fn test_hylomorphism_present() {
        let mut env = make_env();
        register_result_extended_axioms_part2(&mut env);
        assert!(env.get(&Name::str("Result.cata_ana_id")).is_some());
        assert!(env.get(&Name::str("Result.para_extends_cata")).is_some());
    }
    #[test]
    fn test_and_or_combinators_present() {
        let mut env = make_env();
        register_result_extended_axioms_part2(&mut env);
        assert!(env.get(&Name::str("Result.and_ok_ok")).is_some());
        assert!(env.get(&Name::str("Result.and_err_left")).is_some());
        assert!(env.get(&Name::str("Result.or_ok_left")).is_some());
        assert!(env.get(&Name::str("Result.or_err_right")).is_some());
    }
    #[test]
    fn test_combined_part1_and_part2() {
        let mut env = make_env();
        register_result_extended_axioms(&mut env);
        register_result_extended_axioms_part2(&mut env);
        assert!(env.get(&Name::str("Result.monad_left_identity")).is_some());
        assert!(env.get(&Name::str("Result.kleisli_comp_assoc")).is_some());
    }
}
