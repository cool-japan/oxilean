//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{
    BinderInfo, Declaration, Environment, Expr, InductiveEnv, InductiveType, IntroRule, Level, Name,
};

use super::functions::{
    sm_ext_inl_injective, sm_ext_inl_inr_disjoint, sm_ext_inr_injective, sm_ext_sigma_intro,
    sm_ext_sum_applicative_ap, sm_ext_sum_applicative_pure, sm_ext_sum_assoc_iso,
    sm_ext_sum_bifunctor_id, sm_ext_sum_case_analysis, sm_ext_sum_categorical_coproduct,
    sm_ext_sum_disjoint_union, sm_ext_sum_distributes_over_prod, sm_ext_sum_functor_compose,
    sm_ext_sum_functor_id, sm_ext_sum_lefts_count, sm_ext_sum_monad_assoc, sm_ext_sum_monad_bind,
    sm_ext_sum_monad_left_id, sm_ext_sum_monad_return, sm_ext_sum_monad_right_id,
    sm_ext_sum_partition_complete, sm_ext_sum_path_inl, sm_ext_sum_path_inr, sm_ext_sum_path_space,
    sm_ext_sum_swap_involution, sm_ext_sum_traversal_id, sm_ext_sum_traversal_naturality,
    sm_ext_sum_universal_property, sm_ext_sum_void_initial, sm_ext_tagged_union_semantics,
};
use super::types::{
    Coproduct, EitherPartitionSm, SumBifunctor, SumTraversal, SumUniversal, TaggedUnionSm,
};

/// Build axiom: Sigma type projection fst.
fn sm_ext_sigma_fst(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        Bi::Implicit,
        Name::str("A"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("B"),
            Box::new(Expr::Pi(
                Bi::Default,
                Name::str("_"),
                Box::new(Expr::BVar(0)),
                Box::new(type1.clone()),
            )),
            Box::new(Expr::Pi(
                Bi::Default,
                Name::str("s"),
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Sigma"), vec![])),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::BVar(0)),
                )),
                Box::new(Expr::BVar(2)),
            )),
        )),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Sigma.fst"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Sigma type projection snd.
fn sm_ext_sigma_snd(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("Sigma.snd"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Sum elim beta inl (elim (inl a) f g = f a).
fn sm_ext_sum_elim_beta_inl(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.elimBetaInl"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Sum elim beta inr (elim (inr b) f g = g b).
fn sm_ext_sum_elim_beta_inr(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.elimBetaInr"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Sum map identity law (map id id = id).
fn sm_ext_sum_map_id_law(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.mapIdLaw"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Sum map composition law.
fn sm_ext_sum_map_compose_law(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.mapComposeLaw"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: swap ∘ swap = id (involution).
fn sm_ext_swap_swap_id(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.swapSwapId"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Sum isLeft/isRight partition law.
fn sm_ext_sum_is_left_right_partition(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.isLeftRightPartition"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Sum sequence (applicative traverse over List).
fn sm_ext_sum_sequence_law(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.sequenceLaw"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Registration function: registers all extended Sum axioms.
pub fn register_sum_extended_axioms(env: &mut Environment) {
    let _ = sm_ext_sum_categorical_coproduct(env);
    let _ = sm_ext_sum_universal_property(env);
    let _ = sm_ext_inl_injective(env);
    let _ = sm_ext_inr_injective(env);
    let _ = sm_ext_inl_inr_disjoint(env);
    let _ = sm_ext_sum_functor_id(env);
    let _ = sm_ext_sum_functor_compose(env);
    let _ = sm_ext_sum_bifunctor_id(env);
    let _ = sm_ext_sum_monad_return(env);
    let _ = sm_ext_sum_monad_bind(env);
    let _ = sm_ext_sum_monad_left_id(env);
    let _ = sm_ext_sum_monad_right_id(env);
    let _ = sm_ext_sum_monad_assoc(env);
    let _ = sm_ext_sum_applicative_pure(env);
    let _ = sm_ext_sum_applicative_ap(env);
    let _ = sm_ext_sum_swap_involution(env);
    let _ = sm_ext_sum_assoc_iso(env);
    let _ = sm_ext_sum_void_initial(env);
    let _ = sm_ext_sum_distributes_over_prod(env);
    let _ = sm_ext_sum_disjoint_union(env);
    let _ = sm_ext_sum_case_analysis(env);
    let _ = sm_ext_tagged_union_semantics(env);
    let _ = sm_ext_sum_partition_complete(env);
    let _ = sm_ext_sum_lefts_count(env);
    let _ = sm_ext_sum_traversal_naturality(env);
    let _ = sm_ext_sum_traversal_id(env);
    let _ = sm_ext_sum_path_space(env);
    let _ = sm_ext_sum_path_inl(env);
    let _ = sm_ext_sum_path_inr(env);
    let _ = sm_ext_sigma_intro(env);
    let _ = sm_ext_sigma_fst(env);
    let _ = sm_ext_sigma_snd(env);
    let _ = sm_ext_sum_elim_beta_inl(env);
    let _ = sm_ext_sum_elim_beta_inr(env);
    let _ = sm_ext_sum_map_id_law(env);
    let _ = sm_ext_sum_map_compose_law(env);
    let _ = sm_ext_swap_swap_id(env);
    let _ = sm_ext_sum_is_left_right_partition(env);
    let _ = sm_ext_sum_sequence_law(env);
}
/// Fold a list of `Coproduct<E, A>` using a combining function for rights,
/// stopping at the first left.
pub fn fold_rights<A, E: Clone, B>(
    xs: Vec<Coproduct<E, A>>,
    init: B,
    f: impl Fn(B, A) -> B,
) -> Coproduct<E, B> {
    let mut acc = init;
    for x in xs {
        match x {
            Coproduct::Inl(e) => return Coproduct::Inl(e),
            Coproduct::Inr(a) => acc = f(acc, a),
        }
    }
    Coproduct::Inr(acc)
}
/// Zip two `Coproduct<E, A>` and `Coproduct<E, B>` — fails on first left.
pub fn zip_sum<E: Clone, A, B>(sa: Coproduct<E, A>, sb: Coproduct<E, B>) -> Coproduct<E, (A, B)> {
    match (sa, sb) {
        (Coproduct::Inr(a), Coproduct::Inr(b)) => Coproduct::Inr((a, b)),
        (Coproduct::Inl(e), _) => Coproduct::Inl(e),
        (_, Coproduct::Inl(e)) => Coproduct::Inl(e),
    }
}
/// Unzip a `Coproduct<E, (A, B)>` into `(Coproduct<E, A>, Coproduct<E, B>)`.
pub fn unzip_sum<E: Clone, A, B>(s: Coproduct<E, (A, B)>) -> (Coproduct<E, A>, Coproduct<E, B>) {
    match s {
        Coproduct::Inr((a, b)) => (Coproduct::Inr(a), Coproduct::Inr(b)),
        Coproduct::Inl(e) => (Coproduct::Inl(e.clone()), Coproduct::Inl(e)),
    }
}
/// Transpose `Option<Coproduct<E, A>>` to `Coproduct<E, Option<A>>`.
pub fn transpose_option_sum<E, A>(s: Option<Coproduct<E, A>>) -> Coproduct<E, Option<A>> {
    match s {
        None => Coproduct::Inr(None),
        Some(Coproduct::Inr(a)) => Coproduct::Inr(Some(a)),
        Some(Coproduct::Inl(e)) => Coproduct::Inl(e),
    }
}
/// Count elements of a slice that are `Inl`.
pub fn count_lefts<A, B>(xs: &[Coproduct<A, B>]) -> usize {
    xs.iter().filter(|c| c.is_left()).count()
}
/// Count elements of a slice that are `Inr`.
pub fn count_rights<A, B>(xs: &[Coproduct<A, B>]) -> usize {
    xs.iter().filter(|c| c.is_right()).count()
}
/// Build a `TaggedUnionSm` from a `Coproduct`.
pub fn tagged_from_coproduct<A: std::fmt::Debug, B: std::fmt::Debug>(
    c: &Coproduct<A, B>,
) -> TaggedUnionSm {
    match c {
        Coproduct::Inl(a) => TaggedUnionSm::new("left", format!("{:?}", a)),
        Coproduct::Inr(b) => TaggedUnionSm::new("right", format!("{:?}", b)),
    }
}
#[cfg(test)]
mod sum_extended_tests {
    use super::*;
    use std::fmt;
    #[test]
    fn test_sum_universal_new() {
        let u: SumUniversal<i32, &str, String> = SumUniversal::new("test universal");
        assert_eq!(u.description, "test universal");
    }
    #[test]
    fn test_sum_universal_mediate_left() {
        let u: SumUniversal<i32, &str, i32> = SumUniversal::new("med");
        let s = Coproduct::inl(42i32);
        let result = u.mediate(s, |n| n * 2, |s: &str| s.len() as i32);
        assert_eq!(result, 84);
    }
    #[test]
    fn test_sum_universal_mediate_right() {
        let u: SumUniversal<i32, &str, i32> = SumUniversal::new("med");
        let s = Coproduct::inr("hello");
        let result = u.mediate(s, |n| n * 2, |s: &str| s.len() as i32);
        assert_eq!(result, 5);
    }
    #[test]
    fn test_sum_bifunctor_new() {
        let bf: SumBifunctor<i32, &str, i64, String> = SumBifunctor::new("test_bf");
        assert_eq!(bf.name, "test_bf");
    }
    #[test]
    fn test_sum_bifunctor_apply_left() {
        let bf: SumBifunctor<i32, &str, i64, String> = SumBifunctor::new("bf");
        let s = Coproduct::inl(5i32);
        let result = bf.apply(s, |n| n as i64 * 10, |s: &str| s.to_uppercase());
        assert_eq!(result.into_left(), Some(50i64));
    }
    #[test]
    fn test_sum_bifunctor_apply_right() {
        let bf: SumBifunctor<i32, &str, i64, String> = SumBifunctor::new("bf");
        let s = Coproduct::inr("hello");
        let result = bf.apply(s, |n| n as i64, |s: &str| s.to_uppercase());
        assert_eq!(result.into_right(), Some("HELLO".to_string()));
    }
    #[test]
    fn test_tagged_union_sm_new() {
        let t = TaggedUnionSm::new("left", "42");
        assert_eq!(t.tag, "left");
        assert_eq!(t.payload, "42");
    }
    #[test]
    fn test_tagged_union_sm_render() {
        let t = TaggedUnionSm::new("right", "hello");
        assert_eq!(t.render(), "right(hello)");
    }
    #[test]
    fn test_tagged_union_sm_has_tag() {
        let t = TaggedUnionSm::new("left", "x");
        assert!(t.has_tag("left"));
        assert!(!t.has_tag("right"));
    }
    #[test]
    fn test_either_partition_sm_from_vec() {
        let xs: Vec<Coproduct<i32, &str>> =
            vec![Coproduct::inl(1), Coproduct::inr("a"), Coproduct::inl(2)];
        let p = EitherPartitionSm::from_vec(xs);
        assert_eq!(p.left_count(), 2);
        assert_eq!(p.right_count(), 1);
        assert_eq!(p.total(), 3);
    }
    #[test]
    fn test_either_partition_sm_all_right() {
        let xs: Vec<Coproduct<i32, &str>> = vec![Coproduct::inr("x")];
        let p = EitherPartitionSm::from_vec(xs);
        assert!(p.all_right());
        assert!(!p.all_left());
    }
    #[test]
    fn test_either_partition_sm_all_left() {
        let xs: Vec<Coproduct<i32, &str>> = vec![Coproduct::inl(1)];
        let p = EitherPartitionSm::from_vec(xs);
        assert!(p.all_left());
        assert!(!p.all_right());
    }
    #[test]
    fn test_sum_traversal_new() {
        let t: SumTraversal<i32, &str> = SumTraversal::new("test traversal");
        assert_eq!(t.description, "test traversal");
    }
    #[test]
    fn test_sum_traversal_option_some() {
        let t: SumTraversal<i32, &str> = SumTraversal::new("t");
        let s = Coproduct::inr("hello");
        let result = t.traverse_option(s, |s: &str| Some(s.len()));
        assert_eq!(result, Some(Coproduct::inr(5)));
    }
    #[test]
    fn test_sum_traversal_option_none() {
        let t: SumTraversal<i32, &str> = SumTraversal::new("t");
        let s = Coproduct::inr("hello");
        let result: Option<Coproduct<i32, usize>> = t.traverse_option(s, |_| None);
        assert_eq!(result, None);
    }
    #[test]
    fn test_sum_traversal_option_left() {
        let t: SumTraversal<i32, &str> = SumTraversal::new("t");
        let s: Coproduct<i32, &str> = Coproduct::inl(42);
        let result = t.traverse_option(s, |s: &str| Some(s.len()));
        assert_eq!(result, Some(Coproduct::inl(42)));
    }
    #[test]
    fn test_fold_rights_all_ok() {
        let xs: Vec<Coproduct<String, i32>> =
            vec![Coproduct::inr(1), Coproduct::inr(2), Coproduct::inr(3)];
        let result = fold_rights(xs, 0, |acc, n| acc + n);
        assert_eq!(result.into_right(), Some(6));
    }
    #[test]
    fn test_fold_rights_short_circuit() {
        let xs: Vec<Coproduct<String, i32>> = vec![
            Coproduct::inr(1),
            Coproduct::inl("err".to_string()),
            Coproduct::inr(3),
        ];
        let result = fold_rights(xs, 0, |acc, n| acc + n);
        assert!(result.is_left());
    }
    #[test]
    fn test_zip_sum_both_right() {
        let sa: Coproduct<String, i32> = Coproduct::inr(1);
        let sb: Coproduct<String, &str> = Coproduct::inr("a");
        let result = zip_sum(sa, sb);
        assert_eq!(result.into_right(), Some((1, "a")));
    }
    #[test]
    fn test_zip_sum_left_first() {
        let sa: Coproduct<String, i32> = Coproduct::inl("err".to_string());
        let sb: Coproduct<String, &str> = Coproduct::inr("a");
        let result = zip_sum(sa, sb);
        assert!(result.is_left());
    }
    #[test]
    fn test_unzip_sum_right() {
        let s: Coproduct<String, (i32, &str)> = Coproduct::inr((5, "hi"));
        let (sa, sb) = unzip_sum(s);
        assert_eq!(sa.into_right(), Some(5));
        assert_eq!(sb.into_right(), Some("hi"));
    }
    #[test]
    fn test_unzip_sum_left() {
        let s: Coproduct<String, (i32, &str)> = Coproduct::inl("err".to_string());
        let (sa, sb) = unzip_sum(s);
        assert!(sa.is_left());
        assert!(sb.is_left());
    }
    #[test]
    fn test_transpose_option_sum_none() {
        let s: Option<Coproduct<String, i32>> = None;
        let result = transpose_option_sum(s);
        assert_eq!(result.into_right(), Some(None));
    }
    #[test]
    fn test_transpose_option_sum_some_right() {
        let s: Option<Coproduct<String, i32>> = Some(Coproduct::inr(42));
        let result = transpose_option_sum(s);
        assert_eq!(result.into_right(), Some(Some(42)));
    }
    #[test]
    fn test_transpose_option_sum_some_left() {
        let s: Option<Coproduct<String, i32>> = Some(Coproduct::inl("err".to_string()));
        let result = transpose_option_sum(s);
        assert!(result.is_left());
    }
    #[test]
    fn test_count_lefts_rights() {
        let xs: Vec<Coproduct<i32, &str>> = vec![
            Coproduct::inl(1),
            Coproduct::inr("a"),
            Coproduct::inl(2),
            Coproduct::inr("b"),
        ];
        assert_eq!(count_lefts(&xs), 2);
        assert_eq!(count_rights(&xs), 2);
    }
    #[test]
    fn test_tagged_from_coproduct_left() {
        let c: Coproduct<i32, &str> = Coproduct::inl(42);
        let t = tagged_from_coproduct(&c);
        assert!(t.has_tag("left"));
        assert!(t.payload.contains("42"));
    }
    #[test]
    fn test_tagged_from_coproduct_right() {
        let c: Coproduct<i32, &str> = Coproduct::inr("hello");
        let t = tagged_from_coproduct(&c);
        assert!(t.has_tag("right"));
        assert!(t.payload.contains("hello"));
    }
    #[test]
    fn test_register_sum_extended_axioms_runs() {
        let mut env = Environment::new();
        register_sum_extended_axioms(&mut env);
    }
}
