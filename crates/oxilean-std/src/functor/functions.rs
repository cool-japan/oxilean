//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    Either, OptionFunctor, Pred, Reader, RepresentableFunctorExt, ResultFunctor, VecFunctor, Writer,
};

/// Build Functor type class in the environment.
pub fn build_functor_env(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    let functor_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("f"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(type2.clone()),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Functor"),
        univ_params: vec![],
        ty: functor_ty,
    })
    .map_err(|e| e.to_string())?;
    let map_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("f"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Functor"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("a"),
                Box::new(type1.clone()),
                Box::new(Expr::Pi(
                    BinderInfo::Implicit,
                    Name::str("b"),
                    Box::new(type1.clone()),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("fn"),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("_"),
                            Box::new(Expr::BVar(1)),
                            Box::new(Expr::BVar(1)),
                        )),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("fa"),
                            Box::new(Expr::App(Box::new(Expr::BVar(4)), Box::new(Expr::BVar(2)))),
                            Box::new(Expr::App(Box::new(Expr::BVar(5)), Box::new(Expr::BVar(2)))),
                        )),
                    )),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Functor.map"),
        univ_params: vec![],
        ty: map_ty,
    })
    .map_err(|e| e.to_string())?;
    Ok(())
}
/// Build mapConst in the environment.
pub fn build_functor_map_const(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(type1.clone()),
        Box::new(type1),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Functor.mapConst"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Register all Functor declarations.
pub fn build_all_functor_decls(env: &mut Environment) -> Result<(), String> {
    build_functor_env(env)?;
    build_functor_map_const(env)?;
    Ok(())
}
/// Rust-level Functor trait.
pub trait Functor<A> {
    /// The mapped-over container type.
    type Mapped<B>;
    /// Apply f to every element.
    fn fmap<B, F: Fn(A) -> B>(self, f: F) -> Self::Mapped<B>;
}
/// Lift f over Option<A>.
pub fn fmap_option<A, B, F: Fn(A) -> B>(opt: Option<A>, f: F) -> Option<B> {
    opt.map(f)
}
/// Lift f over Vec<A>.
pub fn fmap_vec<A, B, F: Fn(A) -> B>(v: Vec<A>, f: F) -> Vec<B> {
    v.into_iter().map(f).collect()
}
/// Lift f over Result<A, E>.
pub fn fmap_result<A, B, E, F: Fn(A) -> B>(r: Result<A, E>, f: F) -> Result<B, E> {
    r.map(f)
}
/// Build a fmap application expression.
pub fn make_fmap_expr(f_ty: Expr, map_fn: Expr, fa: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Functor.map"), vec![])),
                Box::new(f_ty),
            )),
            Box::new(map_fn),
        )),
        Box::new(fa),
    )
}
/// Build a mapConst application expression.
pub fn make_map_const_expr(f_ty: Expr, a: Expr, fb: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Functor.mapConst"), vec![])),
                Box::new(f_ty),
            )),
            Box::new(a),
        )),
        Box::new(fb),
    )
}
/// Check the identity law for Option.
pub fn option_identity_law<A: Clone + PartialEq>(x: Option<A>) -> bool {
    fmap_option(x.clone(), |a| a) == x
}
/// Check the composition law for Option.
pub fn option_composition_law<A: Clone, B: Clone + PartialEq, C: PartialEq>(
    x: Option<A>,
    f: impl Fn(A) -> B + Clone,
    g: impl Fn(B) -> C + Clone,
) -> bool {
    let f2 = f.clone();
    let g2 = g.clone();
    fmap_option(x.clone(), move |a| g(f(a))) == fmap_option(fmap_option(x, f2), g2)
}
/// Check the identity law for Vec.
pub fn vec_identity_law<A: Clone + PartialEq>(xs: Vec<A>) -> bool {
    fmap_vec(xs.clone(), |a| a) == xs
}
/// Map f over Option<Vec<A>>.
pub fn fmap_option_vec<A, B, F: Fn(A) -> B>(opt_vec: Option<Vec<A>>, f: F) -> Option<Vec<B>> {
    fmap_option(opt_vec, |v| fmap_vec(v, &f))
}
/// Map f over Vec<Option<A>>.
pub fn fmap_vec_option<A, B, F: Fn(A) -> B>(v: Vec<Option<A>>, f: F) -> Vec<Option<B>> {
    fmap_vec(v, |opt| fmap_option(opt, &f))
}
/// Map f over Vec<Result<A, E>>.
pub fn fmap_vec_result<A, B, E, F: Fn(A) -> B>(v: Vec<Result<A, E>>, f: F) -> Vec<Result<B, E>> {
    fmap_vec(v, |r| fmap_result(r, &f))
}
/// Option to Vec.
pub fn option_to_vec<A>(opt: Option<A>) -> Vec<A> {
    match opt {
        Some(a) => vec![a],
        None => vec![],
    }
}
/// Vec to Option (first element).
pub fn vec_to_option<A>(v: Vec<A>) -> Option<A> {
    v.into_iter().next()
}
/// Flatten Option<Option<A>>.
pub fn join_option<A>(opt: Option<Option<A>>) -> Option<A> {
    opt.flatten()
}
/// Flatten Vec<Vec<A>>.
pub fn join_vec<A>(vv: Vec<Vec<A>>) -> Vec<A> {
    vv.into_iter().flatten().collect()
}
/// Apply Option<fn> to Option<A>.
pub fn ap_option<A, B, F: Fn(A) -> B>(f: Option<F>, a: Option<A>) -> Option<B> {
    match (f, a) {
        (Some(g), Some(x)) => Some(g(x)),
        _ => None,
    }
}
/// Apply Vec<fn> to Vec<A> (cartesian product).
#[allow(clippy::redundant_closure)]
pub fn ap_vec<A: Clone, B, F: Fn(A) -> B>(fs: Vec<F>, xs: Vec<A>) -> Vec<B> {
    fs.into_iter()
        .flat_map(|f| xs.iter().cloned().map(move |x| f(x)))
        .collect()
}
/// Monadic bind for Option.
pub fn bind_option<A, B, F: Fn(A) -> Option<B>>(opt: Option<A>, f: F) -> Option<B> {
    opt.and_then(f)
}
/// Monadic bind for Vec (list monad).
pub fn bind_vec<A, B, F: Fn(A) -> Vec<B>>(v: Vec<A>, f: F) -> Vec<B> {
    v.into_iter().flat_map(f).collect()
}
/// Traverse Vec<A> with Option effect.
pub fn traverse_vec_option<A, B, F: Fn(A) -> Option<B>>(v: Vec<A>, f: F) -> Option<Vec<B>> {
    v.into_iter().map(f).collect()
}
/// Traverse Vec<A> with Result effect.
pub fn traverse_vec_result<A, B, E, F: Fn(A) -> Result<B, E>>(
    v: Vec<A>,
    f: F,
) -> Result<Vec<B>, E> {
    v.into_iter().map(f).collect()
}
/// Fold Vec from left.
pub fn fold_left_vec<A, B, F: Fn(B, &A) -> B>(v: &[A], init: B, f: F) -> B {
    v.iter().fold(init, f)
}
/// Fold Vec from right.
pub fn fold_right_vec<A, B, F: Fn(&A, B) -> B>(v: &[A], init: B, f: F) -> B {
    v.iter().rfold(init, |b, a| f(a, b))
}
/// Count elements.
pub fn count_elements<A>(v: &[A]) -> usize {
    v.len()
}
/// Sum u64 elements.
pub fn sum_u64_slice(v: &[u64]) -> u64 {
    v.iter().sum()
}
/// Lift a binary function into Option applicative.
pub fn option_lift2<A, B, C, F: FnOnce(A, B) -> C>(f: F, a: Option<A>, b: Option<B>) -> Option<C> {
    match (a, b) {
        (Some(x), Some(y)) => Some(f(x, y)),
        _ => None,
    }
}
/// Lift a ternary function into Option applicative.
pub fn option_lift3<A, B, C, D, F: FnOnce(A, B, C) -> D>(
    f: F,
    a: Option<A>,
    b: Option<B>,
    c: Option<C>,
) -> Option<D> {
    match (a, b, c) {
        (Some(x), Some(y), Some(z)) => Some(f(x, y, z)),
        _ => None,
    }
}
/// Collect Some values from a slice.
pub fn collect_somes<A: Clone>(opts: &[Option<A>]) -> Vec<A> {
    opts.iter().filter_map(|o| o.clone()).collect()
}
/// Count Some values.
pub fn count_somes<A>(opts: &[Option<A>]) -> usize {
    opts.iter().filter(|o| o.is_some()).count()
}
/// Zip two Vecs into pairs.
pub fn zip_vecs<A: Clone, B: Clone>(a: &[A], b: &[B]) -> Vec<(A, B)> {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x.clone(), y.clone()))
        .collect()
}
/// Unzip a Vec of pairs.
pub fn unzip_pairs<A, B>(pairs: Vec<(A, B)>) -> (Vec<A>, Vec<B>) {
    pairs.into_iter().unzip()
}
/// Find first element satisfying predicate.
pub fn find_first<A, P: Fn(&A) -> bool>(v: &[A], pred: P) -> Option<&A> {
    v.iter().find(|a| pred(a))
}
/// Partition by predicate.
pub fn partition_by<A, P: Fn(&A) -> bool>(v: Vec<A>, pred: P) -> (Vec<A>, Vec<A>) {
    v.into_iter().partition(|a| pred(a))
}
/// Scan left (prefix sums generalized).
pub fn scan_left<A: Clone, B: Clone, F: Fn(B, &A) -> B>(v: &[A], init: B, f: F) -> Vec<B> {
    let mut acc = init;
    let mut result = vec![acc.clone()];
    for a in v {
        acc = f(acc, a);
        result.push(acc.clone());
    }
    result
}
/// Group consecutive equal elements.
pub fn group_by_eq<A: PartialEq + Clone>(v: Vec<A>) -> Vec<Vec<A>> {
    let mut result: Vec<Vec<A>> = Vec::new();
    for item in v {
        if let Some(last) = result.last_mut() {
            if last.first() == Some(&item) {
                last.push(item);
                continue;
            }
        }
        result.push(vec![item]);
    }
    result
}
/// Interleave two Vecs.
pub fn interleave<A>(a: Vec<A>, b: Vec<A>) -> Vec<A> {
    let mut result = Vec::new();
    let mut ai = a.into_iter();
    let mut bi = b.into_iter();
    loop {
        match (ai.next(), bi.next()) {
            (Some(x), Some(y)) => {
                result.push(x);
                result.push(y);
            }
            (Some(x), None) => {
                result.push(x);
                result.extend(ai);
                break;
            }
            (None, Some(y)) => {
                result.push(y);
                result.extend(bi);
                break;
            }
            (None, None) => break,
        }
    }
    result
}
/// Rotate a Vec left by n positions.
pub fn rotate_left<A: Clone>(v: Vec<A>, n: usize) -> Vec<A> {
    if v.is_empty() {
        return v;
    }
    let n = n % v.len();
    let mut result = v[n..].to_vec();
    result.extend_from_slice(&v[..n]);
    result
}
/// Rotate a Vec right by n positions.
pub fn rotate_right<A: Clone>(v: Vec<A>, n: usize) -> Vec<A> {
    if v.is_empty() {
        return v;
    }
    let len = v.len();
    let n = n % len;
    rotate_left(v, len.wrapping_sub(n) % len)
}
/// Deduplicate a Vec (keeps first occurrence).
pub fn dedup_stable<A: PartialEq + Clone>(v: Vec<A>) -> Vec<A> {
    let mut result = Vec::new();
    for item in v {
        if !result.contains(&item) {
            result.push(item);
        }
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_functor_env() {
        let mut env = Environment::new();
        assert!(build_functor_env(&mut env).is_ok());
        assert!(env.get(&Name::str("Functor")).is_some());
        assert!(env.get(&Name::str("Functor.map")).is_some());
    }
    #[test]
    fn test_build_all_functor_decls() {
        let mut env = Environment::new();
        assert!(build_all_functor_decls(&mut env).is_ok());
        assert!(env.contains(&Name::str("Functor")));
        assert!(env.contains(&Name::str("Functor.map")));
        assert!(env.contains(&Name::str("Functor.mapConst")));
    }
    #[test]
    fn test_fmap_option_some() {
        assert_eq!(fmap_option(Some(3), |x| x * 2), Some(6));
    }
    #[test]
    fn test_fmap_option_none() {
        assert_eq!(fmap_option(None::<i32>, |x| x * 2), None);
    }
    #[test]
    fn test_fmap_vec() {
        assert_eq!(fmap_vec(vec![1i32, 2, 3], |x| x + 10), vec![11, 12, 13]);
    }
    #[test]
    fn test_fmap_result_ok() {
        assert_eq!(fmap_result(Ok::<i32, &str>(5), |x| x + 1), Ok(6));
    }
    #[test]
    fn test_fmap_result_err() {
        assert!(fmap_result(Err::<i32, &str>("bad"), |x| x + 1).is_err());
    }
    #[test]
    fn test_option_identity_law() {
        assert!(option_identity_law(Some(42)));
        assert!(option_identity_law::<i32>(None));
    }
    #[test]
    fn test_option_composition_law() {
        assert!(option_composition_law(Some(3i32), |a| a + 1, |b| b * 2));
    }
    #[test]
    fn test_vec_identity_law() {
        assert!(vec_identity_law(vec![1i32, 2, 3]));
        assert!(vec_identity_law::<i32>(vec![]));
    }
    #[test]
    fn test_either_left_right() {
        let l: Either<i32, &str> = Either::Left(1);
        let r: Either<i32, &str> = Either::Right("hi");
        assert!(l.is_left());
        assert!(r.is_right());
    }
    #[test]
    fn test_either_bimap() {
        let e: Either<i32, &str> = Either::Left(2);
        let m = e.bimap(|x| x + 1, |s: &str| s.len());
        assert_eq!(m.unwrap_left(), 3);
    }
    #[test]
    fn test_pred_contramap() {
        let is_pos = Pred::new(|x: i32| x > 0);
        let is_len_pos = is_pos.contramap(|s: &str| s.len() as i32);
        assert!(is_len_pos.test("hello"));
        assert!(!is_len_pos.test(""));
    }
    #[test]
    fn test_fmap_option_vec() {
        let v: Option<Vec<i32>> = Some(vec![1, 2, 3]);
        assert_eq!(fmap_option_vec(v, |x| x * 2), Some(vec![2, 4, 6]));
    }
    #[test]
    fn test_natural_transformations() {
        assert_eq!(option_to_vec(Some(1)), vec![1]);
        assert_eq!(option_to_vec::<i32>(None), vec![]);
        assert_eq!(vec_to_option(vec![1, 2, 3]), Some(1));
    }
    #[test]
    fn test_join_option() {
        assert_eq!(join_option(Some(Some(42))), Some(42));
        assert_eq!(join_option::<i32>(None), None);
    }
    #[test]
    fn test_join_vec() {
        assert_eq!(join_vec(vec![vec![1, 2], vec![3]]), vec![1, 2, 3]);
    }
    #[test]
    fn test_ap_option() {
        let f: Option<fn(i32) -> i32> = Some(|x| x + 1);
        assert_eq!(ap_option(f, Some(5)), Some(6));
    }
    #[test]
    fn test_bind_option() {
        assert_eq!(
            bind_option(Some(5), |x| if x > 3 { Some(x * 2) } else { None }),
            Some(10)
        );
    }
    #[test]
    fn test_bind_vec() {
        assert_eq!(
            bind_vec(vec![1i32, 2], |x| vec![x, x * 10]),
            vec![1, 10, 2, 20]
        );
    }
    #[test]
    fn test_traverse_vec_option() {
        assert_eq!(
            traverse_vec_option(vec![1i32, 2, 3], |x| if x > 0 { Some(x * 2) } else { None }),
            Some(vec![2, 4, 6])
        );
        assert_eq!(
            traverse_vec_option(vec![1i32, -1], |x| if x > 0 { Some(x) } else { None }),
            None
        );
    }
    #[test]
    fn test_traverse_vec_result() {
        let r: Result<Vec<i32>, _> = traverse_vec_result(vec!["1", "2"], |s| s.parse::<i32>());
        assert_eq!(r, Ok(vec![1, 2]));
    }
    #[test]
    fn test_fold_left_vec() {
        assert_eq!(fold_left_vec(&[1u64, 2, 3], 0, |acc, x| acc + x), 6);
    }
    #[test]
    fn test_option_lift2() {
        assert_eq!(option_lift2(|a, b| a + b, Some(3i32), Some(4)), Some(7));
        assert_eq!(option_lift2(|a: i32, b: i32| a + b, None, Some(4)), None);
    }
    #[test]
    fn test_option_lift3() {
        assert_eq!(
            option_lift3(|a, b, c| a + b + c, Some(1i32), Some(2), Some(3)),
            Some(6)
        );
    }
    #[test]
    fn test_collect_somes() {
        let opts = vec![Some(1), None, Some(3), None];
        assert_eq!(collect_somes(&opts), vec![1, 3]);
    }
    #[test]
    fn test_zip_unzip() {
        let pairs = zip_vecs(&[1i32, 2], &["a", "b"]);
        assert_eq!(pairs, vec![(1, "a"), (2, "b")]);
        let (a, b): (Vec<i32>, Vec<&str>) = unzip_pairs(pairs);
        assert_eq!(a, vec![1, 2]);
        assert_eq!(b, vec!["a", "b"]);
    }
    #[test]
    fn test_find_first() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(find_first(&v, |x| *x > 3), Some(&4));
    }
    #[test]
    fn test_partition_by() {
        let v = vec![1i32, 2, 3, 4, 5];
        let (evens, odds) = partition_by(v, |x| x % 2 == 0);
        assert_eq!(evens, vec![2, 4]);
        assert_eq!(odds, vec![1, 3, 5]);
    }
    #[test]
    fn test_scan_left() {
        let v = vec![1u64, 2, 3, 4];
        let scanned = scan_left(&v, 0u64, |acc, x| acc + x);
        assert_eq!(scanned, vec![0, 1, 3, 6, 10]);
    }
    #[test]
    fn test_rotate_left() {
        assert_eq!(rotate_left(vec![1, 2, 3, 4, 5], 2), vec![3, 4, 5, 1, 2]);
    }
    #[test]
    fn test_dedup_stable() {
        assert_eq!(dedup_stable(vec![1, 2, 1, 3, 2]), vec![1, 2, 3]);
    }
    #[test]
    fn test_interleave() {
        assert_eq!(
            interleave(vec![1, 3, 5], vec![2, 4, 6]),
            vec![1, 2, 3, 4, 5, 6]
        );
    }
    #[test]
    fn test_make_fmap_expr() {
        let result = make_fmap_expr(
            Expr::Sort(Level::zero()),
            Expr::Sort(Level::zero()),
            Expr::Sort(Level::zero()),
        );
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_option_functor_wrapper() {
        let of = OptionFunctor(Some(5i32));
        let mapped = of.fmap(|x| x.to_string());
        assert_eq!(mapped.0, Some("5".to_string()));
    }
    #[test]
    fn test_vec_functor_wrapper() {
        let vf = VecFunctor(vec![1i32, 2, 3]);
        let mapped = vf.fmap(|x| x * x);
        assert_eq!(mapped.0, vec![1, 4, 9]);
    }
    #[test]
    fn test_result_functor_wrapper() {
        let rf: ResultFunctor<i32, &str> = ResultFunctor(Ok(7));
        let mapped = rf.fmap(|x| x + 1);
        assert_eq!(mapped.0, Ok(8));
    }
}
/// Apply a function to a value, returning the value unchanged (tap/inspect).
pub fn tap<A, F: FnMut(&A)>(v: A, mut f: F) -> A {
    f(&v);
    v
}
/// Apply a function to a value in Option, returning the value unchanged.
pub fn tap_option<A, F: Fn(&A)>(opt: Option<A>, f: F) -> Option<A> {
    if let Some(ref v) = opt {
        f(v);
    }
    opt
}
/// Apply a function to each element of a Vec, returning the Vec unchanged.
pub fn tap_vec<A, F: Fn(&A)>(v: Vec<A>, f: F) -> Vec<A> {
    for x in &v {
        f(x);
    }
    v
}
/// Repeatedly apply f to x until predicate p returns false.
pub fn iterate_while<A: Clone, F: Fn(A) -> A, P: Fn(&A) -> bool>(mut x: A, f: F, p: P) -> A {
    while p(&x) {
        x = f(x);
    }
    x
}
/// Generate a Vec by iterating f from seed n times.
pub fn generate_vec<A: Clone, F: Fn(A) -> A>(seed: A, f: F, n: usize) -> Vec<A> {
    let mut result = vec![seed.clone()];
    let mut cur = seed;
    for _ in 0..n {
        cur = f(cur);
        result.push(cur.clone());
    }
    result
}
/// Apply a function n times.
pub fn iterate_n<A, F: Fn(A) -> A>(mut x: A, f: F, n: usize) -> A {
    for _ in 0..n {
        x = f(x);
    }
    x
}
/// Memoize a function using a Vec cache (for usize -> A).
pub fn memoize_vec<A: Clone, F: Fn(usize) -> A>(f: F, n: usize) -> Vec<A> {
    (0..n).map(f).collect()
}
/// Compose two functions.
pub fn compose<A, B, C, F: Fn(A) -> B, G: Fn(B) -> C>(f: F, g: G) -> impl Fn(A) -> C {
    move |a| g(f(a))
}
/// Compose three functions.
pub fn compose3<A, B, C, D, F: Fn(A) -> B, G: Fn(B) -> C, H: Fn(C) -> D>(
    f: F,
    g: G,
    h: H,
) -> impl Fn(A) -> D {
    move |a| h(g(f(a)))
}
/// Identity function.
pub fn identity<A>(a: A) -> A {
    a
}
/// Constant function.
pub fn constant<A: Clone, B>(a: A) -> impl Fn(B) -> A {
    move |_| a.clone()
}
/// Flip the arguments of a binary function.
pub fn flip2<A, B, C, F: Fn(A, B) -> C>(f: F) -> impl Fn(B, A) -> C {
    move |b, a| f(a, b)
}
/// Apply f to a if cond is true, otherwise return a.
pub fn apply_if<A, F: Fn(A) -> A>(cond: bool, a: A, f: F) -> A {
    if cond {
        f(a)
    } else {
        a
    }
}
/// Apply f to elements satisfying pred, keeping others unchanged.
pub fn map_where<A, F: Fn(A) -> A, P: Fn(&A) -> bool>(v: Vec<A>, pred: P, f: F) -> Vec<A> {
    v.into_iter()
        .map(|x| if pred(&x) { f(x) } else { x })
        .collect()
}
/// Split a Vec at the first element satisfying pred.
pub fn split_at_pred<A, P: Fn(&A) -> bool>(v: Vec<A>, pred: P) -> (Vec<A>, Vec<A>) {
    let pos = v.iter().position(pred).unwrap_or(v.len());
    let mut right = v;
    let left = right.drain(..pos).collect();
    (left, right)
}
/// Flatten an Option<Vec<A>> to Vec<A>.
pub fn flatten_opt_vec<A>(opt: Option<Vec<A>>) -> Vec<A> {
    opt.unwrap_or_default()
}
/// Flatten a Vec<Option<A>> to Vec<A> (keep Some values).
pub fn flatten_vec_opt<A>(v: Vec<Option<A>>) -> Vec<A> {
    v.into_iter().flatten().collect()
}
/// Chunk a Vec into sub-Vecs of size n.
pub fn chunk<A: Clone>(v: Vec<A>, n: usize) -> Vec<Vec<A>> {
    if n == 0 {
        return vec![];
    }
    v.chunks(n).map(|c| c.to_vec()).collect()
}
/// Sliding window of size n.
pub fn windows<A: Clone>(v: &[A], n: usize) -> Vec<Vec<A>> {
    v.windows(n).map(|w| w.to_vec()).collect()
}
/// Transpose a Vec<Vec<A>> (assuming all inner vecs have the same length).
pub fn transpose<A: Clone>(vv: Vec<Vec<A>>) -> Vec<Vec<A>> {
    if vv.is_empty() {
        return vec![];
    }
    let n = vv[0].len();
    (0..n)
        .map(|i| vv.iter().filter_map(|row| row.get(i)).cloned().collect())
        .collect()
}
/// Map with index.
pub fn map_indexed<A, B, F: Fn(usize, A) -> B>(v: Vec<A>, f: F) -> Vec<B> {
    v.into_iter().enumerate().map(|(i, a)| f(i, a)).collect()
}
/// Flat map with index.
pub fn flat_map_indexed<A, B, F: Fn(usize, A) -> Vec<B>>(v: Vec<A>, f: F) -> Vec<B> {
    v.into_iter()
        .enumerate()
        .flat_map(|(i, a)| f(i, a))
        .collect()
}
/// Filter with index.
pub fn filter_indexed<A, P: Fn(usize, &A) -> bool>(v: Vec<A>, pred: P) -> Vec<A> {
    v.into_iter()
        .enumerate()
        .filter(|(i, a)| pred(*i, a))
        .map(|(_, a)| a)
        .collect()
}
/// Repeat a value n times into a Vec.
pub fn replicate<A: Clone>(n: usize, a: A) -> Vec<A> {
    vec![a; n]
}
/// Take while predicate holds.
pub fn take_while<A, P: Fn(&A) -> bool>(v: Vec<A>, pred: P) -> Vec<A> {
    v.into_iter().take_while(|a| pred(a)).collect()
}
/// Drop while predicate holds.
pub fn drop_while<A, P: Fn(&A) -> bool>(v: Vec<A>, pred: P) -> Vec<A> {
    v.into_iter().skip_while(|a| pred(a)).collect()
}
/// Span: split into (take_while, drop_while).
pub fn span<A, P: Fn(&A) -> bool>(v: Vec<A>, pred: P) -> (Vec<A>, Vec<A>) {
    let pos = v.iter().position(|a| !pred(a)).unwrap_or(v.len());
    let mut right = v;
    let left = right.drain(..pos).collect();
    (left, right)
}
#[cfg(test)]
mod extra_tests {
    use super::*;
    #[test]
    fn test_tap() {
        let mut side = 0;
        let v = tap(42, |x| {
            let _ = x;
            side = 1;
        });
        assert_eq!(v, 42);
        assert_eq!(side, 1);
    }
    #[test]
    fn test_iterate_while() {
        let result = iterate_while(1u32, |x| x * 2, |x| *x < 100);
        assert!(result >= 100);
    }
    #[test]
    fn test_generate_vec() {
        let v = generate_vec(1u64, |x| x * 2, 4);
        assert_eq!(v, vec![1, 2, 4, 8, 16]);
    }
    #[test]
    fn test_iterate_n() {
        let result = iterate_n(1u64, |x| x * 2, 10);
        assert_eq!(result, 1024);
    }
    #[test]
    fn test_compose() {
        let f = compose(|x: i32| x + 1, |x: i32| x * 2);
        assert_eq!(f(3), 8);
    }
    #[test]
    fn test_compose3() {
        let f = compose3(|x: i32| x + 1, |x: i32| x * 2, |x: i32| x - 1);
        assert_eq!(f(3), 7);
    }
    #[test]
    fn test_identity() {
        assert_eq!(identity(42), 42);
        assert_eq!(identity("hello"), "hello");
    }
    #[test]
    fn test_constant() {
        let always5 = constant(5i32);
        assert_eq!(always5("anything"), 5);
        assert_eq!(always5("any"), 5);
    }
    #[test]
    fn test_flip2() {
        let sub = |a: i32, b: i32| a - b;
        let flipped = flip2(sub);
        assert_eq!(flipped(1, 10), 9);
    }
    #[test]
    fn test_apply_if_true() {
        let result = apply_if(true, 5, |x| x * 2);
        assert_eq!(result, 10);
    }
    #[test]
    fn test_apply_if_false() {
        let result = apply_if(false, 5, |x| x * 2);
        assert_eq!(result, 5);
    }
    #[test]
    fn test_map_where() {
        let v = vec![1i32, 2, 3, 4, 5];
        let result = map_where(v, |x| *x % 2 == 0, |x| x * 10);
        assert_eq!(result, vec![1, 20, 3, 40, 5]);
    }
    #[test]
    fn test_flatten_opt_vec() {
        assert_eq!(flatten_opt_vec(Some(vec![1, 2, 3])), vec![1, 2, 3]);
        assert_eq!(flatten_opt_vec::<i32>(None), vec![]);
    }
    #[test]
    fn test_flatten_vec_opt() {
        let v = vec![Some(1), None, Some(3)];
        assert_eq!(flatten_vec_opt(v), vec![1, 3]);
    }
    #[test]
    fn test_chunk() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(chunk(v, 2), vec![vec![1, 2], vec![3, 4], vec![5]]);
    }
    #[test]
    fn test_windows() {
        let v = vec![1, 2, 3, 4];
        assert_eq!(windows(&v, 2), vec![vec![1, 2], vec![2, 3], vec![3, 4]]);
    }
    #[test]
    fn test_transpose() {
        let vv = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let t = transpose(vv);
        assert_eq!(t, vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
    }
    #[test]
    fn test_map_indexed() {
        let v = vec!["a", "b", "c"];
        let result = map_indexed(v, |i, s| format!("{}{}", i, s));
        assert_eq!(result, vec!["0a", "1b", "2c"]);
    }
    #[test]
    fn test_replicate() {
        assert_eq!(replicate(4, 0i32), vec![0, 0, 0, 0]);
    }
    #[test]
    fn test_take_while() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(take_while(v, |x| *x < 4), vec![1, 2, 3]);
    }
    #[test]
    fn test_drop_while() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(drop_while(v, |x| *x < 4), vec![4, 5]);
    }
    #[test]
    fn test_span() {
        let v = vec![1, 2, 3, 4, 5];
        let (left, right) = span(v, |x| *x < 4);
        assert_eq!(left, vec![1, 2, 3]);
        assert_eq!(right, vec![4, 5]);
    }
    #[test]
    fn test_memoize_vec() {
        let cached = memoize_vec(|i| i * i, 5);
        assert_eq!(cached, vec![0, 1, 4, 9, 16]);
    }
}
/// A trait that models the `Foldable` typeclass.
///
/// A foldable structure can be reduced to a single value by combining
/// all elements with a binary operation and an initial accumulator.
pub trait Foldable<A> {
    /// Fold from the left.
    fn fold_left<B>(self, init: B, f: impl Fn(B, A) -> B) -> B;
    /// Fold from the right.
    fn fold_right<B>(self, init: B, f: impl Fn(A, B) -> B) -> B;
    /// Compute the length of the structure.
    fn length(self) -> usize
    where
        Self: Sized,
        A: Clone,
    {
        self.fold_left(0, |acc, _| acc + 1)
    }
    /// Check whether any element satisfies the predicate.
    fn any<F: Fn(&A) -> bool>(self, pred: F) -> bool
    where
        Self: Sized,
        A: Clone,
    {
        self.fold_left(false, |acc, x| acc || pred(&x))
    }
    /// Check whether all elements satisfy the predicate.
    fn all<F: Fn(&A) -> bool>(self, pred: F) -> bool
    where
        Self: Sized,
        A: Clone,
    {
        self.fold_left(true, |acc, x| acc && pred(&x))
    }
}
impl<A: Clone> Foldable<A> for Vec<A> {
    fn fold_left<B>(self, init: B, f: impl Fn(B, A) -> B) -> B {
        self.into_iter().fold(init, f)
    }
    fn fold_right<B>(self, init: B, f: impl Fn(A, B) -> B) -> B {
        self.into_iter().rev().fold(init, |acc, x| f(x, acc))
    }
}
/// A trait that models the `Traversable` typeclass.
///
/// A traversable structure can be mapped with an effectful function
/// and the effects can be combined.
pub trait Traversable<A, B, E> {
    /// The output container type.
    type Output;
    /// Traverse the structure, applying `f` to each element.
    fn traverse(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Output, E>;
}
impl<A, B, E> Traversable<A, B, E> for Vec<A> {
    type Output = Vec<B>;
    fn traverse(self, f: impl Fn(A) -> Result<B, E>) -> Result<Vec<B>, E> {
        let mut result = Vec::with_capacity(self.len());
        for item in self {
            result.push(f(item)?);
        }
        Ok(result)
    }
}
#[cfg(test)]
mod extra_functor_tests {
    use super::*;
    #[test]
    fn test_foldable_fold_left() {
        let v = vec![1i32, 2, 3, 4, 5];
        let sum = v.fold_left(0i32, |acc, x| acc + x);
        assert_eq!(sum, 15);
    }
    #[test]
    fn test_foldable_fold_right() {
        let v = vec![1i32, 2, 3];
        let result = v.fold_right(Vec::<i32>::new(), |x, mut acc| {
            acc.insert(0, x);
            acc
        });
        assert_eq!(result, vec![1, 2, 3]);
    }
    #[test]
    fn test_foldable_any_true() {
        let v = vec![1i32, 2, 3];
        assert!(v.any(|x| *x == 2));
    }
    #[test]
    fn test_foldable_any_false() {
        let v = vec![1i32, 2, 3];
        assert!(!v.any(|x| *x == 99));
    }
    #[test]
    fn test_foldable_all_true() {
        let v = vec![2i32, 4, 6];
        assert!(v.all(|x| *x % 2 == 0));
    }
    #[test]
    fn test_foldable_all_false() {
        let v = vec![2i32, 3, 6];
        assert!(!v.all(|x| *x % 2 == 0));
    }
    #[test]
    fn test_traversable_ok() {
        let v = vec![1i32, 2, 3];
        let result: Result<Vec<i32>, &str> = v.traverse(|x| Ok(x * 2));
        assert_eq!(result, Ok(vec![2, 4, 6]));
    }
    #[test]
    fn test_traversable_err() {
        let v = vec![1i32, 0, 3];
        let result: Result<Vec<i32>, &str> =
            v.traverse(|x| if x != 0 { Ok(x * 2) } else { Err("zero") });
        assert_eq!(result, Err("zero"));
    }
    #[test]
    fn test_reader_run() {
        let r: Reader<i32, i32> = Reader::new(|env| env * 2);
        assert_eq!(r.run_reader(5), 10);
    }
    #[test]
    fn test_reader_map() {
        let r: Reader<i32, i32> = Reader::new(|env| env + 1);
        let r2 = r.map(|x| x * 3);
        assert_eq!(r2.run_reader(4), 15);
    }
    #[test]
    fn test_reader_pure() {
        let r: Reader<i32, i32> = Reader::pure(42);
        assert_eq!(r.run_reader(999), 42);
    }
    #[test]
    fn test_writer_run() {
        let w: Writer<i32, Vec<String>> = Writer::new(42, vec!["logged".to_string()]);
        let (v, log) = w.run();
        assert_eq!(v, 42);
        assert_eq!(log, vec!["logged".to_string()]);
    }
    #[test]
    fn test_writer_pure() {
        let w: Writer<i32, Vec<String>> = Writer::pure(7);
        assert_eq!(w.get_value(), 7);
    }
}
pub fn ftr_ext_identity_law_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("f_alpha"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_composition_law_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("f_alpha"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_naturality_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("nat_ty"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_list_identity_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("list_a"),
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("List"), vec![])),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("List"), vec![])),
            Box::new(type1),
        )),
    )
}
pub fn ftr_ext_list_composition_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("list_compose"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_option_identity_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("opt_id"),
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Option"), vec![])),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Option"), vec![])),
            Box::new(type1),
        )),
    )
}
pub fn ftr_ext_option_composition_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("opt_comp"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_result_identity_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("res_id"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_pair_fmap_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("pair_fmap"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_bifunctor_identity_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("bimap_id"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_bifunctor_composition_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("bimap_comp"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_contramap_identity_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("cmap_id"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_contramap_composition_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("cmap_comp"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_profunctor_identity_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("dimap_id"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_profunctor_composition_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("dimap_comp"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_ap_identity_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("ap_id"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_ap_homomorphism_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("ap_hom"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_ap_interchange_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("ap_interchange"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_ap_composition_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("ap_comp"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_monoidal_unit_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("mono_unit"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_monoidal_zip_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("mono_zip"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_strong_first_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("strong_first"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_strong_second_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("strong_second"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_cartesian_copy_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("cartesian_copy"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_cartesian_delete_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("cartesian_delete"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_closed_apply_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("closed_apply"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_closed_curry_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("closed_curry"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_traverse_identity_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("trav_id"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_traverse_composition_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("trav_comp"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_fold_consistent_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("fold_consist"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_foldmap_morphism_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("foldmap_morph"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_representable_tabulate_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("rep_tab"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_representable_index_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("rep_idx"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_kan_left_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("lan_k"),
        Box::new(type2),
        Box::new(type1),
    )
}
pub fn ftr_ext_kan_right_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("ran_k"),
        Box::new(type2),
        Box::new(type1),
    )
}
pub fn ftr_ext_day_conv_unit_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("day_unit"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_day_conv_assoc_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("day_assoc"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_compose_identity_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("fcomp_id"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_compose_assoc_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("fcomp_assoc"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_nat_trans_id_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("nat_trans_id"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_nat_trans_comp_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("nat_trans_comp"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_adjunction_unit_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("adj_unit"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_adjunction_counit_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("adj_counit"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_adjunction_triangle_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("adj_triangle"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_functor_cat_obj_ty() -> Expr {
    let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("fcat_obj"),
        Box::new(type2.clone()),
        Box::new(type2),
    )
}
pub fn ftr_ext_functor_cat_mor_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("fcat_mor"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_presheaf_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("presheaf"),
        Box::new(type2),
        Box::new(type1),
    )
}
pub fn ftr_ext_yoneda_lemma_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("yoneda"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_yoneda_embedding_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("yoneda_embed"),
        Box::new(type1),
        Box::new(type2),
    )
}
pub fn ftr_ext_sheaf_gluing_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("sheaf_glue"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_sheaf_locality_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("sheaf_local"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_fmap_preserves_eq_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("fmap_eq"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_free_functor_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("free_functor"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_cofree_functor_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("cofree_functor"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
pub fn ftr_ext_exponential_functor_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("exp_functor"),
        Box::new(type1.clone()),
        Box::new(type1),
    )
}
/// Register all extended functor axioms in the environment.
pub fn register_functor_extended_axioms(env: &mut Environment) {
    let axioms: &[(&str, fn() -> Expr)] = &[
        ("Functor.Ext.IdentityLaw", ftr_ext_identity_law_ty),
        ("Functor.Ext.CompositionLaw", ftr_ext_composition_law_ty),
        ("Functor.Ext.Naturality", ftr_ext_naturality_ty),
        ("Functor.Ext.ListIdentity", ftr_ext_list_identity_ty),
        ("Functor.Ext.ListComposition", ftr_ext_list_composition_ty),
        ("Functor.Ext.OptionIdentity", ftr_ext_option_identity_ty),
        (
            "Functor.Ext.OptionComposition",
            ftr_ext_option_composition_ty,
        ),
        ("Functor.Ext.ResultIdentity", ftr_ext_result_identity_ty),
        ("Functor.Ext.PairFmap", ftr_ext_pair_fmap_ty),
        (
            "Functor.Ext.BifunctorIdentity",
            ftr_ext_bifunctor_identity_ty,
        ),
        (
            "Functor.Ext.BifunctorComposition",
            ftr_ext_bifunctor_composition_ty,
        ),
        (
            "Functor.Ext.ContramapIdentity",
            ftr_ext_contramap_identity_ty,
        ),
        (
            "Functor.Ext.ContramapComposition",
            ftr_ext_contramap_composition_ty,
        ),
        (
            "Functor.Ext.ProfunctorIdentity",
            ftr_ext_profunctor_identity_ty,
        ),
        (
            "Functor.Ext.ProfunctorComposition",
            ftr_ext_profunctor_composition_ty,
        ),
        ("Functor.Ext.ApIdentity", ftr_ext_ap_identity_ty),
        ("Functor.Ext.ApHomomorphism", ftr_ext_ap_homomorphism_ty),
        ("Functor.Ext.ApInterchange", ftr_ext_ap_interchange_ty),
        ("Functor.Ext.ApComposition", ftr_ext_ap_composition_ty),
        ("Functor.Ext.MonoidalUnit", ftr_ext_monoidal_unit_ty),
        ("Functor.Ext.MonoidalZip", ftr_ext_monoidal_zip_ty),
        ("Functor.Ext.StrongFirst", ftr_ext_strong_first_ty),
        ("Functor.Ext.StrongSecond", ftr_ext_strong_second_ty),
        ("Functor.Ext.CartesianCopy", ftr_ext_cartesian_copy_ty),
        ("Functor.Ext.CartesianDelete", ftr_ext_cartesian_delete_ty),
        ("Functor.Ext.ClosedApply", ftr_ext_closed_apply_ty),
        ("Functor.Ext.ClosedCurry", ftr_ext_closed_curry_ty),
        ("Functor.Ext.TraverseIdentity", ftr_ext_traverse_identity_ty),
        (
            "Functor.Ext.TraverseComposition",
            ftr_ext_traverse_composition_ty,
        ),
        ("Functor.Ext.FoldConsistent", ftr_ext_fold_consistent_ty),
        ("Functor.Ext.FoldmapMorphism", ftr_ext_foldmap_morphism_ty),
        (
            "Functor.Ext.RepresentableTabulate",
            ftr_ext_representable_tabulate_ty,
        ),
        (
            "Functor.Ext.RepresentableIndex",
            ftr_ext_representable_index_ty,
        ),
        ("Functor.Ext.KanLeft", ftr_ext_kan_left_ty),
        ("Functor.Ext.KanRight", ftr_ext_kan_right_ty),
        ("Functor.Ext.DayConvUnit", ftr_ext_day_conv_unit_ty),
        ("Functor.Ext.DayConvAssoc", ftr_ext_day_conv_assoc_ty),
        ("Functor.Ext.ComposeIdentity", ftr_ext_compose_identity_ty),
        ("Functor.Ext.ComposeAssoc", ftr_ext_compose_assoc_ty),
        ("Functor.Ext.NatTransId", ftr_ext_nat_trans_id_ty),
        ("Functor.Ext.NatTransComp", ftr_ext_nat_trans_comp_ty),
        ("Functor.Ext.AdjunctionUnit", ftr_ext_adjunction_unit_ty),
        ("Functor.Ext.AdjunctionCounit", ftr_ext_adjunction_counit_ty),
        (
            "Functor.Ext.AdjunctionTriangle",
            ftr_ext_adjunction_triangle_ty,
        ),
        ("Functor.Ext.FunctorCatObj", ftr_ext_functor_cat_obj_ty),
        ("Functor.Ext.FunctorCatMor", ftr_ext_functor_cat_mor_ty),
        ("Functor.Ext.Presheaf", ftr_ext_presheaf_ty),
        ("Functor.Ext.YonedaLemma", ftr_ext_yoneda_lemma_ty),
        ("Functor.Ext.YonedaEmbedding", ftr_ext_yoneda_embedding_ty),
        ("Functor.Ext.SheafGluing", ftr_ext_sheaf_gluing_ty),
        ("Functor.Ext.SheafLocality", ftr_ext_sheaf_locality_ty),
        ("Functor.Ext.FmapPreservesEq", ftr_ext_fmap_preserves_eq_ty),
        ("Functor.Ext.FreeFunctor", ftr_ext_free_functor_ty),
        ("Functor.Ext.CofreeFunctor", ftr_ext_cofree_functor_ty),
        (
            "Functor.Ext.ExponentialFunctor",
            ftr_ext_exponential_functor_ty,
        ),
    ];
    for (name, ty_fn) in axioms {
        let _ = env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty_fn(),
        });
    }
}
/// Kleisli composition for Option monad.
pub fn kleisli_compose_option<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> Option<C>
where
    F: Fn(A) -> Option<B>,
    G: Fn(B) -> Option<C>,
{
    move |a| f(a).and_then(|b| g(b))
}
/// Kleisli composition for Vec monad (list monad).
pub fn kleisli_compose_vec<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> Vec<C>
where
    F: Fn(A) -> Vec<B>,
    G: Fn(B) -> Vec<C>,
    B: Clone,
{
    move |a| f(a).into_iter().flat_map(|b| g(b)).collect()
}
/// Lift a function into the Reader functor.
pub fn reader_fmap<E: Clone + 'static, A: 'static, B: 'static>(
    reader: Reader<E, A>,
    f: impl Fn(A) -> B + 'static,
) -> Reader<E, B> {
    reader.map(f)
}
/// Dimap for a function (profunctor instance for Fn).
pub fn dimap_fn<A, B, C, D>(
    pre: impl Fn(C) -> A + 'static,
    post: impl Fn(B) -> D + 'static,
    f: impl Fn(A) -> B + 'static,
) -> impl Fn(C) -> D {
    move |c| post(f(pre(c)))
}
/// Apply a natural transformation option_to_vec then fmap.
pub fn fmap_via_nat_trans<A: Clone, B, F: Fn(A) -> B>(v: Option<A>, f: F) -> Vec<B> {
    option_to_vec(v).into_iter().map(f).collect()
}
/// Sequence Option<Vec<A>> — swap the two functors.
pub fn sequence_option_vec<A: Clone>(xs: Vec<Option<A>>) -> Option<Vec<A>> {
    xs.into_iter().collect()
}
/// Map with two effects: Option then Vec.
pub fn fmap_option_then_vec<A, B: Clone, F: Fn(A) -> B>(opt: Option<A>, f: F) -> Vec<B> {
    match opt {
        Some(a) => vec![f(a)],
        None => vec![],
    }
}
/// Pure for the Option applicative.
pub fn pure_option<A>(a: A) -> Option<A> {
    Some(a)
}
/// Pure for the Vec applicative.
pub fn pure_vec<A>(a: A) -> Vec<A> {
    vec![a]
}
/// Left-to-right Kleisli fish operator for Option.
pub fn fish_option<A, B, C>(
    f: impl Fn(A) -> Option<B>,
    g: impl Fn(B) -> Option<C>,
    a: A,
) -> Option<C> {
    f(a).and_then(g)
}
/// Monoidal product for Options: pair them up.
pub fn option_zip<A, B>(a: Option<A>, b: Option<B>) -> Option<(A, B)> {
    match (a, b) {
        (Some(x), Some(y)) => Some((x, y)),
        _ => None,
    }
}
/// Day convolution combinator on Vec.
pub fn day_conv_vec<A: Clone, B: Clone, C>(
    fa: Vec<A>,
    fb: Vec<B>,
    combine: impl Fn(A, B) -> C,
) -> Vec<C> {
    let mut result = Vec::new();
    for a in fa {
        for b in fb.iter().cloned() {
            result.push(combine(a.clone(), b));
        }
    }
    result
}
/// Cofunctor (contravariant) composition.
pub fn contramap_compose<A: 'static, B: 'static, C: 'static>(
    pred: Pred<A>,
    f: impl Fn(B) -> A + 'static,
    g: impl Fn(C) -> B + 'static,
) -> Pred<C> {
    pred.contramap(move |c| f(g(c)))
}
/// Check the profunctor identity law for functions.
pub fn profunctor_identity_law<A: Clone + PartialEq>(
    f: impl Fn(A) -> A + Clone + 'static,
    x: A,
) -> bool {
    let f2 = f.clone();
    let dimapped = dimap_fn(|a: A| a, |a: A| a, f2);
    dimapped(x.clone()) == f(x)
}
/// Functor composition: apply outer fmap then inner fmap.
pub fn double_fmap<A, B, C>(
    xs: Vec<Option<A>>,
    f: impl Fn(A) -> B,
    g: impl Fn(B) -> C,
) -> Vec<Option<C>> {
    xs.into_iter().map(|opt| opt.map(|a| g(f(a)))).collect()
}
/// Check adjunction triangle law (L ⊣ R): (ε L) ∘ (L η) = id.
/// Approximated as: round-trip through Option is identity.
pub fn adjunction_triangle_option<A: Clone + PartialEq>(a: A) -> bool {
    let unit = |x: A| Some(x);
    let counit = |opt: Option<A>| opt.expect("unit always produces Some, so counit receives Some");
    counit(unit(a.clone())) == a
}
/// Yoneda reduction: Nat(Hom(A, –), F) ≅ F A.
/// Approximated by showing that applying fmap id to a value is identity.
pub fn yoneda_reduction<A: Clone + PartialEq>(fa: Option<A>) -> bool {
    fmap_option(fa.clone(), |a| a) == fa
}
/// Left Kan extension approximation: extend f along k using Vec.
pub fn lan_approximation<A: Clone, B: Clone, C>(
    k: impl Fn(A) -> B,
    f: impl Fn(A) -> C,
    xs: Vec<A>,
) -> Vec<C> {
    xs.into_iter()
        .map(|a| {
            let _ = k(a.clone());
            f(a)
        })
        .collect()
}
/// Check sheaf gluing: two compatible sections glue uniquely.
/// Approximated: merging two Option sections.
pub fn sheaf_glue_option<A: Clone + PartialEq>(s1: Option<A>, s2: Option<A>) -> Option<A> {
    match (s1, s2) {
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (Some(a), Some(_)) => Some(a),
        (None, None) => None,
    }
}
/// Representable functor: tabulate then index roundtrip.
pub fn representable_roundtrip<A: Clone + PartialEq>(
    n: usize,
    f: impl Fn(usize) -> A,
    i: usize,
) -> bool {
    let rep = RepresentableFunctorExt::tabulate(n, &f);
    if i < n {
        rep.index(i).cloned() == Some(f(i))
    } else {
        rep.index(i).is_none()
    }
}
/// Check Day convolution unit law: Day(F, Id) ≅ F.
pub fn day_conv_unit_vec<A: Clone + PartialEq>(fa: Vec<A>) -> bool {
    let id_unit: Vec<()> = vec![()];
    let result: Vec<A> = day_conv_vec(fa.clone(), id_unit, |a, _| a);
    result == fa
}
