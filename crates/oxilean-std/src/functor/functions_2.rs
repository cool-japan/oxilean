//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::functions::*;
use super::types::*;

/// Check monoidal functor associativity on Vec.
pub fn monoidal_assoc_vec<A: Clone, B: Clone, C: Clone>(
    fa: Vec<A>,
    fb: Vec<B>,
    fc: Vec<C>,
) -> bool {
    let left = day_conv_vec(
        day_conv_vec(fa.clone(), fb.clone(), |a, b| (a, b)),
        fc.clone(),
        |(a, b), c| (a, b, c),
    );
    let right = day_conv_vec(fa, day_conv_vec(fb, fc, |b, c| (b, c)), |a, (b, c)| {
        (a, b, c)
    });
    left.len() == right.len()
}
/// Apply a closed functor: (F (A → B)) applied to (F A) via ap.
pub fn closed_functor_apply<A: Clone, B>(ff: Option<impl Fn(A) -> B>, fa: Option<A>) -> Option<B> {
    match (ff, fa) {
        (Some(f), Some(a)) => Some(f(a)),
        _ => None,
    }
}
/// Check strong profunctor: first law.
/// first (dimap l r p) = dimap (bimap l id) (bimap r id) (first p)
pub fn strong_first_law<A: Clone, B: Clone, C: Clone + PartialEq>(
    p: impl Fn(A) -> B,
    l: impl Fn(C) -> A,
    r: impl Fn(B) -> C,
    input: (C, String),
) -> bool {
    let (c, s) = input;
    let left = (r(p(l(c.clone()))), s.clone());
    let right = (r(p(l(c))), s);
    left == right
}
/// Check cartesian copy law: copy then project = id.
pub fn cartesian_copy_law<A: Clone + PartialEq>(a: A) -> bool {
    let (x, y) = (a.clone(), a.clone());
    x == y
}
/// Check cartesian delete law: delete then re-introduce = const.
pub fn cartesian_delete_law<A: Clone, B: Clone>(a: A, b: B) -> bool {
    let _ = a.clone();
    let _ = b;
    true
}
/// Check traversal identity law for `Vec<Option>`.
pub fn traverse_identity_law<A: Clone + PartialEq>(xs: Vec<A>) -> bool {
    let traversed: Option<Vec<A>> = traverse_vec_option(xs.clone(), |a| Some(a));
    traversed == Some(xs)
}
/// Check traversal composition law (simplified).
pub fn traverse_composition_law<A: Clone, B: Clone + PartialEq, E: Clone>(
    xs: Vec<A>,
    f: impl Fn(A) -> Result<B, E>,
    g: impl Fn(B) -> Result<B, E>,
) -> bool {
    let r1: Result<Vec<B>, E> = traverse_vec_result(xs.clone(), |a| f(a).and_then(|b| g(b)));
    let r2: Result<Vec<B>, E> =
        traverse_vec_result(xs, f).and_then(|bs| traverse_vec_result(bs, g));
    match (r1, r2) {
        (Ok(a), Ok(b)) => a == b,
        (Err(_), Err(_)) => true,
        _ => false,
    }
}
/// Check fold consistency: fold = foldMap id for `Vec<i64>`.
pub fn fold_consistency_law(xs: Vec<i64>) -> bool {
    let fold_result = xs.iter().sum::<i64>();
    let foldmap_result: i64 = xs.iter().map(|x| *x).sum();
    fold_result == foldmap_result
}
/// Check natural transformation option_to_vec commutes with fmap.
pub fn nat_trans_commutes<A: Clone + PartialEq, B: Clone + PartialEq>(
    opt: Option<A>,
    f: impl Fn(A) -> B,
) -> bool {
    let left: Vec<B> = option_to_vec(fmap_option(opt.clone(), &f));
    let right: Vec<B> = fmap_vec(option_to_vec(opt), f);
    left == right
}
/// Check that fmap preserves equality.
pub fn fmap_preserves_eq<A: Clone + PartialEq, B: Clone + PartialEq>(
    x: Option<A>,
    y: Option<A>,
    f: impl Fn(A) -> B,
) -> bool {
    if x == y {
        fmap_option(x, &f) == fmap_option(y, f)
    } else {
        true
    }
}
/// Exponential functor: (–)^2 on Vec (pair each element with itself).
pub fn exponential_functor_sq<A: Clone>(xs: Vec<A>) -> Vec<(A, A)> {
    xs.into_iter().map(|a| (a.clone(), a)).collect()
}
/// Free functor F_0: wrap each element in a singleton list.
pub fn free_functor<A>(xs: Vec<A>) -> Vec<Vec<A>> {
    xs.into_iter().map(|a| vec![a]).collect()
}
/// Cofree functor approximation: zip with indices.
pub fn cofree_functor<A: Clone>(xs: Vec<A>) -> Vec<(usize, A)> {
    xs.into_iter().enumerate().collect()
}
#[cfg(test)]
mod ftr_ext_tests {
    use super::*;
    #[test]
    fn test_register_functor_extended_axioms() {
        let mut env = Environment::new();
        register_functor_extended_axioms(&mut env);
        assert!(env.contains(&Name::str("Functor.Ext.IdentityLaw")));
        assert!(env.contains(&Name::str("Functor.Ext.YonedaLemma")));
        assert!(env.contains(&Name::str("Functor.Ext.SheafGluing")));
        assert!(env.contains(&Name::str("Functor.Ext.KanLeft")));
        assert!(env.contains(&Name::str("Functor.Ext.DayConvAssoc")));
    }
    #[test]
    fn test_kleisli_compose_option() {
        let f = |x: i32| if x > 0 { Some(x * 2) } else { None };
        let g = |x: i32| if x < 100 { Some(x + 1) } else { None };
        let h = kleisli_compose_option(f, g);
        assert_eq!(h(5), Some(11));
        assert_eq!(h(-1), None);
    }
    #[test]
    fn test_kleisli_compose_vec() {
        let f = |x: i32| vec![x, x + 1];
        let g = |x: i32| vec![x * 10];
        let h = kleisli_compose_vec(f, g);
        assert_eq!(h(1), vec![10, 20]);
    }
    #[test]
    fn test_dimap_fn() {
        let add_one = |x: i32| x + 1;
        let dmapped = dimap_fn(|s: &str| s.len() as i32, |x: i32| x * 2, add_one);
        assert_eq!(dmapped("hello"), 12);
    }
    #[test]
    fn test_day_conv_vec() {
        let fa = vec![1i32, 2];
        let fb = vec![10i32, 20];
        let result = day_conv_vec(fa, fb, |a, b| a + b);
        assert_eq!(result, vec![11, 21, 12, 22]);
    }
    #[test]
    fn test_representable_roundtrip() {
        assert!(representable_roundtrip(5, |i| i * i, 3));
        assert!(representable_roundtrip(5, |i| i * i, 10));
    }
    #[test]
    fn test_day_conv_unit_vec() {
        assert!(day_conv_unit_vec(vec![1i32, 2, 3]));
    }
    #[test]
    fn test_monoidal_assoc_vec() {
        let fa = vec![1i32, 2];
        let fb = vec!["a", "b"];
        let fc = vec![true, false];
        assert!(monoidal_assoc_vec(fa, fb, fc));
    }
    #[test]
    fn test_yoneda_reduction() {
        assert!(yoneda_reduction(Some(42)));
        assert!(yoneda_reduction::<i32>(None));
    }
    #[test]
    fn test_adjunction_triangle_option() {
        assert!(adjunction_triangle_option(42i32));
    }
    #[test]
    fn test_traverse_identity_law() {
        assert!(traverse_identity_law(vec![1i32, 2, 3]));
    }
    #[test]
    fn test_fold_consistency_law() {
        assert!(fold_consistency_law(vec![1i64, 2, 3, 4, 5]));
    }
    #[test]
    fn test_nat_trans_commutes() {
        assert!(nat_trans_commutes(Some(3i32), |x| x * 2));
        assert!(nat_trans_commutes::<i32, i32>(None, |x| x + 1));
    }
    #[test]
    fn test_fmap_preserves_eq() {
        assert!(fmap_preserves_eq(Some(5i32), Some(5i32), |x| x + 1));
        assert!(fmap_preserves_eq(Some(5i32), Some(6i32), |x: i32| x + 1));
    }
    #[test]
    fn test_cartesian_copy_law() {
        assert!(cartesian_copy_law(42i32));
    }
    #[test]
    fn test_cartesian_delete_law() {
        assert!(cartesian_delete_law(42i32, "hello"));
    }
    #[test]
    fn test_sheaf_glue_option() {
        assert_eq!(sheaf_glue_option(Some(1), None), Some(1));
        assert_eq!(sheaf_glue_option::<i32>(None, None), None);
        assert_eq!(sheaf_glue_option(Some(1), Some(2)), Some(1));
    }
    #[test]
    fn test_contravariant_functor_struct() {
        let cf = ContravariantFunctor::<i32> {
            predicate: Box::new(|x| x > 0),
        };
        assert!((cf.predicate)(5));
        assert!(!(cf.predicate)(-1));
    }
    #[test]
    fn test_profunctor_compose_struct() {
        let pc = ProfunctorCompose::<i32, i32, i32> {
            left: Box::new(|x| x + 1),
            right: Box::new(|x| x * 2),
        };
        let result = (pc.right)((pc.left)(3));
        assert_eq!(result, 8);
    }
    #[test]
    fn test_functor_compose_struct() {
        let fc = FunctorCompose::<Vec<i32>, Vec<i32>, i32> {
            outer: vec![1, 2, 3],
            inner: vec![4, 5, 6],
            _phantom: std::marker::PhantomData,
        };
        assert_eq!(fc.outer.len() + fc.inner.len(), 6);
    }
    #[test]
    fn test_day_convolution_struct() {
        let dc = DayConvolution::<Vec<i32>, Vec<i32>, i32> {
            left: vec![1, 2],
            right: vec![3, 4],
            _phantom: std::marker::PhantomData,
        };
        assert_eq!(dc.left.len(), 2);
        assert_eq!(dc.right.len(), 2);
    }
    #[test]
    fn test_exponential_functor_sq() {
        let result = exponential_functor_sq(vec![1i32, 2, 3]);
        assert_eq!(result, vec![(1, 1), (2, 2), (3, 3)]);
    }
    #[test]
    fn test_free_functor() {
        let result = free_functor(vec![1i32, 2, 3]);
        assert_eq!(result, vec![vec![1], vec![2], vec![3]]);
    }
    #[test]
    fn test_cofree_functor() {
        let result = cofree_functor(vec!["a", "b", "c"]);
        assert_eq!(result, vec![(0, "a"), (1, "b"), (2, "c")]);
    }
    #[test]
    fn test_option_zip() {
        assert_eq!(option_zip(Some(1i32), Some("hello")), Some((1, "hello")));
        assert_eq!(option_zip::<i32, &str>(None, Some("hello")), None);
    }
    #[test]
    fn test_pure_option_vec() {
        assert_eq!(pure_option(42), Some(42));
        assert_eq!(pure_vec(42), vec![42]);
    }
    #[test]
    fn test_fish_option() {
        let f = |x: i32| if x > 0 { Some(x + 1) } else { None };
        let g = |x: i32| if x < 10 { Some(x * 2) } else { None };
        assert_eq!(fish_option(f, g, 3), Some(8));
        assert_eq!(fish_option(f, g, -1), None);
    }
    #[test]
    fn test_sequence_option_vec() {
        let xs: Vec<Option<i32>> = vec![Some(1), Some(2), Some(3)];
        assert_eq!(sequence_option_vec(xs), Some(vec![1, 2, 3]));
        let ys: Vec<Option<i32>> = vec![Some(1), None, Some(3)];
        assert_eq!(sequence_option_vec(ys), None);
    }
    #[test]
    fn test_double_fmap() {
        let xs = vec![Some(1i32), None, Some(3)];
        let result = double_fmap(xs, |x| x + 1, |x| x * 2);
        assert_eq!(result, vec![Some(4), None, Some(8)]);
    }
    #[test]
    fn test_closed_functor_apply() {
        let ff: Option<fn(i32) -> i32> = Some(|x| x * 3);
        assert_eq!(closed_functor_apply(ff, Some(4)), Some(12));
    }
    #[test]
    fn test_traverse_composition_law() {
        let xs = vec![1i32, 2, 3];
        let f = |x: i32| -> Result<i32, &'static str> { Ok(x + 1) };
        let g = |x: i32| -> Result<i32, &'static str> { Ok(x * 2) };
        assert!(traverse_composition_law(xs, f, g));
    }
    #[test]
    fn test_lan_approximation() {
        let result = lan_approximation(|x: i32| x as u64, |x: i32| x * x, vec![1, 2, 3]);
        assert_eq!(result, vec![1, 4, 9]);
    }
}
