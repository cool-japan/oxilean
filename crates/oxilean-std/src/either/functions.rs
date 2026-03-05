//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{EitherLeftIter, EitherRightIter, LeftIter, OxiEither, RightIter, TripleSum};

pub fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn type2() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
}
pub fn either_of(alpha: Expr, beta: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Either"), vec![])),
            Box::new(alpha),
        )),
        Box::new(beta),
    )
}
pub fn option_of(alpha: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::Const(Name::str("Option"), vec![])),
        Box::new(alpha),
    )
}
pub fn bool_ty() -> Expr {
    Expr::Const(Name::str("Bool"), vec![])
}
pub fn list_of(alpha: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::Const(Name::str("List"), vec![])),
        Box::new(alpha),
    )
}
pub fn prod_of(alpha: Expr, beta: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Prod"), vec![])),
            Box::new(alpha),
        )),
        Box::new(beta),
    )
}
pub fn axiom(env: &mut Environment, name: &str, ty: Expr) -> Result<(), String> {
    env.add(Declaration::Axiom {
        name: Name::str(name),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
pub fn ab_implicit(inner: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1()),
            Box::new(inner),
        )),
    )
}
/// Build Either type in the environment.
pub fn build_either_env(env: &mut Environment) -> Result<(), String> {
    let either_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("β"),
            Box::new(type1()),
            Box::new(type2()),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Either"),
        univ_params: vec![],
        ty: either_ty,
    })
    .map_err(|e| e.to_string())?;
    add_left(env)?;
    add_right(env)?;
    add_is_left(env)?;
    add_is_right(env)?;
    add_get_left(env)?;
    add_get_right(env)?;
    add_cases(env)?;
    add_map(env)?;
    add_map_left(env)?;
    add_bimap(env)?;
    add_swap(env)?;
    add_fold(env)?;
    add_to_option(env)?;
    add_from_option(env)?;
    add_sequence(env)?;
    add_partition_eithers(env)?;
    Ok(())
}
pub fn add_left(env: &mut Environment) -> Result<(), String> {
    let ty = ab_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("a"),
        Box::new(Expr::BVar(1)),
        Box::new(either_of(Expr::BVar(2), Expr::BVar(1))),
    ));
    axiom(env, "Either.left", ty)
}
pub fn add_right(env: &mut Environment) -> Result<(), String> {
    let ty = ab_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("b"),
        Box::new(Expr::BVar(0)),
        Box::new(either_of(Expr::BVar(2), Expr::BVar(1))),
    ));
    axiom(env, "Either.right", ty)
}
pub fn add_is_left(env: &mut Environment) -> Result<(), String> {
    let ty = ab_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("e"),
        Box::new(either_of(Expr::BVar(1), Expr::BVar(0))),
        Box::new(bool_ty()),
    ));
    axiom(env, "Either.isLeft", ty)
}
pub fn add_is_right(env: &mut Environment) -> Result<(), String> {
    let ty = ab_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("e"),
        Box::new(either_of(Expr::BVar(1), Expr::BVar(0))),
        Box::new(bool_ty()),
    ));
    axiom(env, "Either.isRight", ty)
}
pub fn add_get_left(env: &mut Environment) -> Result<(), String> {
    let ty = ab_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("e"),
        Box::new(either_of(Expr::BVar(1), Expr::BVar(0))),
        Box::new(option_of(Expr::BVar(2))),
    ));
    axiom(env, "Either.getLeft", ty)
}
pub fn add_get_right(env: &mut Environment) -> Result<(), String> {
    let ty = ab_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("e"),
        Box::new(either_of(Expr::BVar(1), Expr::BVar(0))),
        Box::new(option_of(Expr::BVar(1))),
    ));
    axiom(env, "Either.getRight", ty)
}
pub fn add_cases(env: &mut Environment) -> Result<(), String> {
    let fl = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(Expr::BVar(2)),
        Box::new(Expr::BVar(2)),
    );
    let fr = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(Expr::BVar(2)),
        Box::new(Expr::BVar(2)),
    );
    let ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("γ"),
                Box::new(type1()),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("e"),
                    Box::new(either_of(Expr::BVar(2), Expr::BVar(1))),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("l"),
                        Box::new(fl),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("r"),
                            Box::new(fr),
                            Box::new(Expr::BVar(3)),
                        )),
                    )),
                )),
            )),
        )),
    );
    axiom(env, "Either.cases", ty)
}
pub fn add_map(env: &mut Environment) -> Result<(), String> {
    let fn_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(Expr::BVar(1)),
        Box::new(Expr::BVar(1)),
    );
    let ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("γ"),
                Box::new(type1()),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("f"),
                    Box::new(fn_ty),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("e"),
                        Box::new(either_of(Expr::BVar(3), Expr::BVar(2))),
                        Box::new(either_of(Expr::BVar(4), Expr::BVar(2))),
                    )),
                )),
            )),
        )),
    );
    axiom(env, "Either.map", ty)
}
pub fn add_map_left(env: &mut Environment) -> Result<(), String> {
    let fn_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(Expr::BVar(2)),
        Box::new(Expr::BVar(2)),
    );
    let ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("γ"),
                Box::new(type1()),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("f"),
                    Box::new(fn_ty),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("e"),
                        Box::new(either_of(Expr::BVar(3), Expr::BVar(2))),
                        Box::new(either_of(Expr::BVar(2), Expr::BVar(3))),
                    )),
                )),
            )),
        )),
    );
    axiom(env, "Either.mapLeft", ty)
}
pub fn add_bimap(env: &mut Environment) -> Result<(), String> {
    let fl = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(Expr::BVar(3)),
        Box::new(Expr::BVar(3)),
    );
    let fr = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(Expr::BVar(3)),
        Box::new(Expr::BVar(3)),
    );
    let ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("γ"),
                Box::new(type1()),
                Box::new(Expr::Pi(
                    BinderInfo::Implicit,
                    Name::str("δ"),
                    Box::new(type1()),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("fl"),
                        Box::new(fl),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("fr"),
                            Box::new(fr),
                            Box::new(Expr::Pi(
                                BinderInfo::Default,
                                Name::str("e"),
                                Box::new(either_of(Expr::BVar(5), Expr::BVar(4))),
                                Box::new(either_of(Expr::BVar(4), Expr::BVar(4))),
                            )),
                        )),
                    )),
                )),
            )),
        )),
    );
    axiom(env, "Either.bimap", ty)
}
pub fn add_swap(env: &mut Environment) -> Result<(), String> {
    let ty = ab_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("e"),
        Box::new(either_of(Expr::BVar(1), Expr::BVar(0))),
        Box::new(either_of(Expr::BVar(1), Expr::BVar(2))),
    ));
    axiom(env, "Either.swap", ty)
}
pub fn add_fold(env: &mut Environment) -> Result<(), String> {
    let fl = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(Expr::BVar(2)),
        Box::new(Expr::BVar(2)),
    );
    let fr = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(Expr::BVar(2)),
        Box::new(Expr::BVar(2)),
    );
    let ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("γ"),
                Box::new(type1()),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("fl"),
                    Box::new(fl),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("fr"),
                        Box::new(fr),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("e"),
                            Box::new(either_of(Expr::BVar(4), Expr::BVar(3))),
                            Box::new(Expr::BVar(3)),
                        )),
                    )),
                )),
            )),
        )),
    );
    axiom(env, "Either.fold", ty)
}
pub fn add_to_option(env: &mut Environment) -> Result<(), String> {
    let ty = ab_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("e"),
        Box::new(either_of(Expr::BVar(1), Expr::BVar(0))),
        Box::new(option_of(Expr::BVar(1))),
    ));
    axiom(env, "Either.toOption", ty)
}
pub fn add_from_option(env: &mut Environment) -> Result<(), String> {
    let ty = ab_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("err"),
        Box::new(Expr::BVar(1)),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("opt"),
            Box::new(option_of(Expr::BVar(1))),
            Box::new(either_of(Expr::BVar(3), Expr::BVar(2))),
        )),
    ));
    axiom(env, "Either.fromOption", ty)
}
pub fn add_sequence(env: &mut Environment) -> Result<(), String> {
    let ty = ab_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("xs"),
        Box::new(list_of(either_of(Expr::BVar(1), Expr::BVar(0)))),
        Box::new(either_of(Expr::BVar(2), list_of(Expr::BVar(1)))),
    ));
    axiom(env, "Either.sequence", ty)
}
pub fn add_partition_eithers(env: &mut Environment) -> Result<(), String> {
    let ty = ab_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("xs"),
        Box::new(list_of(either_of(Expr::BVar(1), Expr::BVar(0)))),
        Box::new(prod_of(list_of(Expr::BVar(2)), list_of(Expr::BVar(1)))),
    ));
    axiom(env, "Either.partitionEithers", ty)
}
pub fn setup_base_env() -> Environment {
    let mut env = Environment::new();
    let t1 = type1();
    for name in ["Bool", "Option", "List", "Prod"] {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty: t1.clone(),
        })
        .unwrap_or(());
    }
    env
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_either_env() {
        let mut env = setup_base_env();
        assert!(build_either_env(&mut env).is_ok());
        assert!(env.get(&Name::str("Either")).is_some());
        assert!(env.get(&Name::str("Either.left")).is_some());
        assert!(env.get(&Name::str("Either.right")).is_some());
    }
    #[test]
    fn test_either_is_left() {
        let mut env = setup_base_env();
        build_either_env(&mut env).expect("build_either_env should succeed");
        assert!(matches!(
            env.get(&Name::str("Either.isLeft"))
                .expect("declaration 'Either.isLeft' should exist in env"),
            Declaration::Axiom { .. }
        ));
    }
    #[test]
    fn test_either_is_right() {
        let mut env = setup_base_env();
        build_either_env(&mut env).expect("build_either_env should succeed");
        assert!(matches!(
            env.get(&Name::str("Either.isRight"))
                .expect("declaration 'Either.isRight' should exist in env"),
            Declaration::Axiom { .. }
        ));
    }
    #[test]
    fn test_either_get_left() {
        let mut env = setup_base_env();
        build_either_env(&mut env).expect("build_either_env should succeed");
        assert!(env.get(&Name::str("Either.getLeft")).is_some());
    }
    #[test]
    fn test_either_get_right() {
        let mut env = setup_base_env();
        build_either_env(&mut env).expect("build_either_env should succeed");
        assert!(env.get(&Name::str("Either.getRight")).is_some());
    }
    #[test]
    fn test_either_cases() {
        let mut env = setup_base_env();
        build_either_env(&mut env).expect("build_either_env should succeed");
        assert!(env.get(&Name::str("Either.cases")).is_some());
    }
    #[test]
    fn test_either_map() {
        let mut env = setup_base_env();
        build_either_env(&mut env).expect("build_either_env should succeed");
        assert!(env.get(&Name::str("Either.map")).is_some());
    }
    #[test]
    fn test_either_map_left() {
        let mut env = setup_base_env();
        build_either_env(&mut env).expect("build_either_env should succeed");
        assert!(env.get(&Name::str("Either.mapLeft")).is_some());
    }
    #[test]
    fn test_either_bimap() {
        let mut env = setup_base_env();
        build_either_env(&mut env).expect("build_either_env should succeed");
        assert!(env.get(&Name::str("Either.bimap")).is_some());
    }
    #[test]
    fn test_either_swap() {
        let mut env = setup_base_env();
        build_either_env(&mut env).expect("build_either_env should succeed");
        assert!(env.get(&Name::str("Either.swap")).is_some());
    }
    #[test]
    fn test_either_fold() {
        let mut env = setup_base_env();
        build_either_env(&mut env).expect("build_either_env should succeed");
        assert!(env.get(&Name::str("Either.fold")).is_some());
    }
    #[test]
    fn test_either_to_option() {
        let mut env = setup_base_env();
        build_either_env(&mut env).expect("build_either_env should succeed");
        assert!(env.get(&Name::str("Either.toOption")).is_some());
    }
    #[test]
    fn test_either_from_option() {
        let mut env = setup_base_env();
        build_either_env(&mut env).expect("build_either_env should succeed");
        assert!(env.get(&Name::str("Either.fromOption")).is_some());
    }
    #[test]
    fn test_either_sequence() {
        let mut env = setup_base_env();
        build_either_env(&mut env).expect("build_either_env should succeed");
        assert!(env.get(&Name::str("Either.sequence")).is_some());
    }
    #[test]
    fn test_either_partition_eithers() {
        let mut env = setup_base_env();
        build_either_env(&mut env).expect("build_either_env should succeed");
        assert!(env.get(&Name::str("Either.partitionEithers")).is_some());
    }
}
/// Extension methods for iterators of `OxiEither`.
pub trait EitherIterExt<A, B>: Iterator<Item = OxiEither<A, B>> + Sized {
    /// Yield only the `Left` values.
    fn lefts(self) -> LeftIter<A, B, Self> {
        LeftIter { inner: self }
    }
    /// Yield only the `Right` values.
    fn rights(self) -> RightIter<A, B, Self> {
        RightIter { inner: self }
    }
    /// Partition into (lefts, rights).
    fn partition_either(self) -> (Vec<A>, Vec<B>) {
        let mut ls = Vec::new();
        let mut rs = Vec::new();
        for e in self {
            match e {
                OxiEither::Left(a) => ls.push(a),
                OxiEither::Right(b) => rs.push(b),
            }
        }
        (ls, rs)
    }
}
impl<A, B, I: Iterator<Item = OxiEither<A, B>>> EitherIterExt<A, B> for I {}
/// Validate a list of items, collecting either all successes or the first error.
///
/// Returns `Right(errors)` if any item validates to an `OxiEither::Right(error)`;
/// otherwise returns `Left(successes)`.
pub fn validate_all<A: Clone, E: Clone, I: IntoIterator<Item = OxiEither<A, E>>>(
    items: I,
) -> OxiEither<Vec<A>, Vec<E>> {
    let mut successes = Vec::new();
    let mut errors = Vec::new();
    for item in items {
        match item {
            OxiEither::Left(a) => successes.push(a),
            OxiEither::Right(e) => errors.push(e),
        }
    }
    if errors.is_empty() {
        OxiEither::Left(successes)
    } else {
        OxiEither::Right(errors)
    }
}
/// Sequence a list of `OxiEither` values, stopping at the first `Right`.
pub fn sequence_either<A: Clone, E: Clone, I: IntoIterator<Item = OxiEither<A, E>>>(
    items: I,
) -> OxiEither<Vec<A>, E> {
    let mut results = Vec::new();
    for item in items {
        match item {
            OxiEither::Left(a) => results.push(a),
            OxiEither::Right(e) => return OxiEither::Right(e),
        }
    }
    OxiEither::Left(results)
}
/// Apply a function to each element, collecting all results.
pub fn traverse_either<A, B, E, F, I>(items: I, f: F) -> OxiEither<Vec<B>, E>
where
    I: IntoIterator<Item = A>,
    F: Fn(A) -> OxiEither<B, E>,
{
    let mut results = Vec::new();
    for item in items {
        match f(item) {
            OxiEither::Left(b) => results.push(b),
            OxiEither::Right(e) => return OxiEither::Right(e),
        }
    }
    OxiEither::Left(results)
}
/// Zip two `OxiEither` values, pairing their contents.
pub fn zip_either<A, B, C>(ea: OxiEither<A, C>, eb: OxiEither<B, C>) -> OxiEither<(A, B), C> {
    match (ea, eb) {
        (OxiEither::Left(a), OxiEither::Left(b)) => OxiEither::Left((a, b)),
        (OxiEither::Right(e), _) | (_, OxiEither::Right(e)) => OxiEither::Right(e),
    }
}
/// Merge two `OxiEither` values with a combining function.
pub fn merge_either<A, B, C, F>(ea: OxiEither<A, B>, eb: OxiEither<A, B>, f: F) -> OxiEither<A, B>
where
    F: Fn(A, A) -> A,
    A: Clone,
    B: Clone,
{
    match (ea, eb) {
        (OxiEither::Left(a1), OxiEither::Left(a2)) => OxiEither::Left(f(a1, a2)),
        (OxiEither::Right(e), _) | (_, OxiEither::Right(e)) => OxiEither::Right(e),
    }
}
/// Convert `Option<A>` to `OxiEither<A, ()>`.
pub fn option_to_either<A>(opt: Option<A>) -> OxiEither<A, ()> {
    match opt {
        Some(a) => OxiEither::Left(a),
        None => OxiEither::Right(()),
    }
}
/// Convert `OxiEither<A, ()>` back to `Option<A>`.
pub fn either_to_option<A>(e: OxiEither<A, ()>) -> Option<A> {
    match e {
        OxiEither::Left(a) => Some(a),
        OxiEither::Right(()) => None,
    }
}
/// Convert `Result<A, E>` to `OxiEither<A, E>`.
pub fn result_to_either<A, E>(r: Result<A, E>) -> OxiEither<A, E> {
    match r {
        Ok(a) => OxiEither::Left(a),
        Err(e) => OxiEither::Right(e),
    }
}
/// Convert `OxiEither<A, E>` to `Result<A, E>`.
pub fn either_to_result<A, E>(e: OxiEither<A, E>) -> Result<A, E> {
    match e {
        OxiEither::Left(a) => Ok(a),
        OxiEither::Right(e) => Err(e),
    }
}
/// Count `Left` and `Right` values in a collection.
pub fn count_lefts_rights<A, B, I: IntoIterator<Item = OxiEither<A, B>>>(
    items: I,
) -> (usize, usize) {
    items.into_iter().fold((0, 0), |(ls, rs), e| match e {
        OxiEither::Left(_) => (ls + 1, rs),
        OxiEither::Right(_) => (ls, rs + 1),
    })
}
/// Build an `OxiEither` from a predicate.
pub fn either_from_pred<A: Clone, B: Clone, F: Fn(&A) -> Option<B>>(a: A, f: F) -> OxiEither<A, B> {
    match f(&a) {
        Some(b) => OxiEither::Right(b),
        None => OxiEither::Left(a),
    }
}
#[cfg(test)]
mod either_extended_tests {
    use super::*;
    #[test]
    fn test_lefts_iter() {
        let items: Vec<OxiEither<i32, &str>> = vec![
            OxiEither::Left(1),
            OxiEither::Right("no"),
            OxiEither::Left(2),
        ];
        let lefts: Vec<i32> = items.into_iter().lefts().collect();
        assert_eq!(lefts, vec![1, 2]);
    }
    #[test]
    fn test_rights_iter() {
        let items: Vec<OxiEither<i32, &str>> = vec![
            OxiEither::Left(1),
            OxiEither::Right("yes"),
            OxiEither::Right("also"),
        ];
        let rights: Vec<&str> = items.into_iter().rights().collect();
        assert_eq!(rights, vec!["yes", "also"]);
    }
    #[test]
    fn test_partition_either_iter() {
        let items: Vec<OxiEither<i32, &str>> = vec![
            OxiEither::Left(1),
            OxiEither::Right("e"),
            OxiEither::Left(2),
        ];
        let (ls, rs) = items.into_iter().partition_either();
        assert_eq!(ls, vec![1, 2]);
        assert_eq!(rs, vec!["e"]);
    }
    #[test]
    fn test_validate_all_ok() {
        let items: Vec<OxiEither<i32, &str>> = vec![OxiEither::Left(1), OxiEither::Left(2)];
        let result = validate_all(items);
        assert_eq!(result, OxiEither::Left(vec![1, 2]));
    }
    #[test]
    fn test_validate_all_errors() {
        let items: Vec<OxiEither<i32, &str>> = vec![OxiEither::Left(1), OxiEither::Right("bad")];
        let result = validate_all(items);
        assert_eq!(result, OxiEither::Right(vec!["bad"]));
    }
    #[test]
    fn test_sequence_either_ok() {
        let items: Vec<OxiEither<i32, &str>> = vec![OxiEither::Left(1), OxiEither::Left(2)];
        assert_eq!(sequence_either(items), OxiEither::Left(vec![1, 2]));
    }
    #[test]
    fn test_sequence_either_fail() {
        let items: Vec<OxiEither<i32, &str>> = vec![OxiEither::Left(1), OxiEither::Right("err")];
        assert_eq!(sequence_either(items), OxiEither::Right("err"));
    }
    #[test]
    fn test_traverse_either_ok() {
        let result = traverse_either(vec![1i32, 2, 3], |x| OxiEither::Left(x * 2));
        assert_eq!(result, OxiEither::<Vec<i32>, &str>::Left(vec![2, 4, 6]));
    }
    #[test]
    fn test_option_to_either() {
        let e = option_to_either(Some(42i32));
        assert_eq!(e, OxiEither::Left(42));
        let e2: OxiEither<i32, ()> = option_to_either(None);
        assert_eq!(e2, OxiEither::Right(()));
    }
    #[test]
    fn test_result_to_either() {
        let e: OxiEither<i32, &str> = result_to_either(Ok(1));
        assert_eq!(e, OxiEither::Left(1));
        let e2: OxiEither<i32, &str> = result_to_either(Err("oops"));
        assert_eq!(e2, OxiEither::Right("oops"));
    }
    #[test]
    fn test_count_lefts_rights() {
        let items: Vec<OxiEither<i32, &str>> = vec![
            OxiEither::Left(1),
            OxiEither::Right("a"),
            OxiEither::Left(2),
        ];
        assert_eq!(count_lefts_rights(items), (2, 1));
    }
    #[test]
    fn test_triple_sum_to_nested() {
        let t: TripleSum<i32, &str, f64> = TripleSum::First(1);
        let nested = t.to_nested();
        assert!(nested.is_left());
        let t2: TripleSum<i32, &str, f64> = TripleSum::Second("hi");
        let nested2 = t2.to_nested();
        assert!(nested2.is_right());
    }
    #[test]
    fn test_triple_sum_variants() {
        let a: TripleSum<i32, i32, i32> = TripleSum::First(1);
        let b: TripleSum<i32, i32, i32> = TripleSum::Second(2);
        let c: TripleSum<i32, i32, i32> = TripleSum::Third(3);
        assert!(a.is_first() && !a.is_second() && !a.is_third());
        assert!(b.is_second());
        assert!(c.is_third());
    }
    #[test]
    fn test_zip_either_both_left() {
        let ea: OxiEither<i32, &str> = OxiEither::Left(1);
        let eb: OxiEither<i32, &str> = OxiEither::Left(2);
        let result = zip_either(ea, eb);
        assert_eq!(result, OxiEither::Left((1, 2)));
    }
    #[test]
    fn test_zip_either_one_right() {
        let ea: OxiEither<i32, &str> = OxiEither::Left(1);
        let eb: OxiEither<i32, &str> = OxiEither::Right("err");
        let result = zip_either(ea, eb);
        assert!(result.is_right());
    }
}
/// Unwrap a `Left` or compute a default from a `Right`.
pub fn left_or_else<A, B, F: FnOnce(B) -> A>(e: OxiEither<A, B>, f: F) -> A {
    match e {
        OxiEither::Left(a) => a,
        OxiEither::Right(b) => f(b),
    }
}
/// Unwrap a `Right` or compute a default from a `Left`.
pub fn right_or_else<A, B, F: FnOnce(A) -> B>(e: OxiEither<A, B>, f: F) -> B {
    match e {
        OxiEither::Right(b) => b,
        OxiEither::Left(a) => f(a),
    }
}
/// Filter a collection, returning `Left(v)` if predicate holds, `Right(v)` otherwise.
pub fn filter_with_either<T, F: Fn(&T) -> bool>(
    items: impl IntoIterator<Item = T>,
    pred: F,
) -> Vec<OxiEither<T, T>> {
    items
        .into_iter()
        .map(|item| {
            if pred(&item) {
                OxiEither::Left(item)
            } else {
                OxiEither::Right(item)
            }
        })
        .collect()
}
/// Flatten a nested `OxiEither<OxiEither<A, B>, B>` to `OxiEither<A, B>`.
pub fn flatten_either<A, B>(e: OxiEither<OxiEither<A, B>, B>) -> OxiEither<A, B> {
    match e {
        OxiEither::Left(inner) => inner,
        OxiEither::Right(b) => OxiEither::Right(b),
    }
}
/// Transpose `OxiEither<Option<A>, B>` to `Option<OxiEither<A, B>>`.
pub fn transpose_option_either<A, B>(e: OxiEither<Option<A>, B>) -> Option<OxiEither<A, B>> {
    match e {
        OxiEither::Left(Some(a)) => Some(OxiEither::Left(a)),
        OxiEither::Left(None) => None,
        OxiEither::Right(b) => Some(OxiEither::Right(b)),
    }
}
/// Collect `OxiEither` values from an iterator, accumulating errors.
pub fn collect_errors<A, E, I: IntoIterator<Item = OxiEither<A, E>>>(items: I) -> (Vec<A>, Vec<E>) {
    items
        .into_iter()
        .fold((Vec::new(), Vec::new()), |(mut ls, mut rs), e| {
            match e {
                OxiEither::Left(a) => ls.push(a),
                OxiEither::Right(e) => rs.push(e),
            }
            (ls, rs)
        })
}
/// Apply a mapping that may fail, short-circuiting on the first error.
pub fn try_map<A, B, E, F, I>(items: I, f: F) -> OxiEither<Vec<B>, E>
where
    I: IntoIterator<Item = A>,
    F: Fn(A) -> OxiEither<B, E>,
{
    traverse_either(items, f)
}
/// An `OxiEither` where both sides have the same type (a homogeneous sum).
pub type Homo<T> = OxiEither<T, T>;
#[cfg(test)]
mod either_further_tests {
    use super::*;
    #[test]
    fn test_left_or_else() {
        let e: OxiEither<i32, &str> = OxiEither::Left(5);
        assert_eq!(left_or_else(e, |_| 99), 5);
        let e2: OxiEither<i32, &str> = OxiEither::Right("err");
        assert_eq!(left_or_else(e2, |_| 99), 99);
    }
    #[test]
    fn test_right_or_else() {
        let e: OxiEither<i32, &str> = OxiEither::Right("ok");
        assert_eq!(right_or_else(e, |_| "default"), "ok");
        let e2: OxiEither<i32, &str> = OxiEither::Left(1);
        assert_eq!(right_or_else(e2, |_| "default"), "default");
    }
    #[test]
    fn test_filter_with_either() {
        let items = vec![1i32, 2, 3, 4];
        let result = filter_with_either(items, |x| x % 2 == 0);
        let (evens, odds): (Vec<_>, Vec<_>) = result.into_iter().partition(|e| e.is_left());
        assert_eq!(evens.len(), 2);
        assert_eq!(odds.len(), 2);
    }
    #[test]
    fn test_flatten_either() {
        let e: OxiEither<OxiEither<i32, &str>, &str> = OxiEither::Left(OxiEither::Left(1));
        assert_eq!(flatten_either(e), OxiEither::Left(1));
        let e2: OxiEither<OxiEither<i32, &str>, &str> = OxiEither::Right("err");
        assert_eq!(flatten_either(e2), OxiEither::Right("err"));
    }
    #[test]
    fn test_transpose_option_either_some() {
        let e: OxiEither<Option<i32>, &str> = OxiEither::Left(Some(42));
        assert_eq!(transpose_option_either(e), Some(OxiEither::Left(42)));
    }
    #[test]
    fn test_transpose_option_either_none() {
        let e: OxiEither<Option<i32>, &str> = OxiEither::Left(None);
        assert_eq!(transpose_option_either(e), None);
    }
    #[test]
    fn test_collect_errors() {
        let items: Vec<OxiEither<i32, &str>> = vec![
            OxiEither::Left(1),
            OxiEither::Right("e1"),
            OxiEither::Left(2),
            OxiEither::Right("e2"),
        ];
        let (ls, rs) = collect_errors(items);
        assert_eq!(ls, vec![1, 2]);
        assert_eq!(rs, vec!["e1", "e2"]);
    }
    #[test]
    fn test_homo_into_inner() {
        let e: OxiEither<i32, i32> = OxiEither::Left(5);
        assert_eq!(e.into_inner(), 5);
        let e2: OxiEither<i32, i32> = OxiEither::Right(7);
        assert_eq!(e2.into_inner(), 7);
    }
    #[test]
    fn test_try_map_ok() {
        let result = try_map(vec![1i32, 2, 3], |x| {
            if x > 0 {
                OxiEither::Left(x * 2)
            } else {
                OxiEither::Right("neg")
            }
        });
        assert_eq!(result, OxiEither::Left(vec![2, 4, 6]));
    }
    #[test]
    fn test_try_map_fail() {
        let result: OxiEither<Vec<i32>, &str> = try_map(vec![1i32, -1, 2], |x| {
            if x > 0 {
                OxiEither::Left(x)
            } else {
                OxiEither::Right("neg")
            }
        });
        assert_eq!(result, OxiEither::Right("neg"));
    }
}
/// Separate a slice of `OxiEither`s into lefts and rights.
pub fn separate<A: Clone, B: Clone>(items: &[OxiEither<A, B>]) -> (Vec<A>, Vec<B>) {
    let mut lefts = Vec::new();
    let mut rights = Vec::new();
    for item in items {
        match item {
            OxiEither::Left(a) => lefts.push(a.clone()),
            OxiEither::Right(b) => rights.push(b.clone()),
        }
    }
    (lefts, rights)
}
/// Count the number of `Left` values in a slice.
pub fn count_lefts<A, B>(items: &[OxiEither<A, B>]) -> usize {
    items.iter().filter(|e| e.is_left()).count()
}
/// Count the number of `Right` values in a slice.
pub fn count_rights<A, B>(items: &[OxiEither<A, B>]) -> usize {
    items.iter().filter(|e| e.is_right()).count()
}
/// Map a function over all `Right` values in a slice, leaving `Left`s unchanged.
pub fn map_rights<A: Clone, B: Clone, C>(
    items: &[OxiEither<A, B>],
    f: impl Fn(B) -> C,
) -> Vec<OxiEither<A, C>> {
    items.iter().map(|e| e.clone().map_right(&f)).collect()
}
/// Map a function over all `Left` values in a slice, leaving `Right`s unchanged.
pub fn map_lefts<A: Clone, B: Clone, C>(
    items: &[OxiEither<A, B>],
    f: impl Fn(A) -> C,
) -> Vec<OxiEither<C, B>> {
    items.iter().map(|e| e.clone().map_left(&f)).collect()
}
/// Collect only the `Right` values from a slice.
pub fn collect_rights<A, B: Clone>(items: &[OxiEither<A, B>]) -> Vec<B> {
    items.iter().filter_map(|e| e.as_right().cloned()).collect()
}
/// Collect only the `Left` values from a slice.
pub fn collect_lefts<A: Clone, B>(items: &[OxiEither<A, B>]) -> Vec<A> {
    items.iter().filter_map(|e| e.as_left().cloned()).collect()
}
#[cfg(test)]
mod extra_either_tests {
    use super::*;
    #[test]
    fn test_either_right_iter_some() {
        let e: OxiEither<i32, &str> = OxiEither::Right("hello");
        let mut it = EitherRightIter::new(e);
        assert_eq!(it.next(), Some("hello"));
        assert_eq!(it.next(), None);
    }
    #[test]
    fn test_either_right_iter_none() {
        let e: OxiEither<i32, &str> = OxiEither::Left(42);
        let mut it = EitherRightIter::new(e);
        assert_eq!(it.next(), None);
    }
    #[test]
    fn test_either_left_iter_some() {
        let e: OxiEither<i32, &str> = OxiEither::Left(99);
        let mut it = EitherLeftIter::new(e);
        assert_eq!(it.next(), Some(99));
        assert_eq!(it.next(), None);
    }
    #[test]
    fn test_either_left_iter_none() {
        let e: OxiEither<i32, &str> = OxiEither::Right("r");
        let mut it = EitherLeftIter::new(e);
        assert_eq!(it.next(), None);
    }
    #[test]
    fn test_separate() {
        let items: Vec<OxiEither<i32, &str>> = vec![
            OxiEither::Left(1),
            OxiEither::Right("a"),
            OxiEither::Left(2),
        ];
        let (ls, rs) = separate(&items);
        assert_eq!(ls, vec![1, 2]);
        assert_eq!(rs, vec!["a"]);
    }
    #[test]
    fn test_count_lefts_rights() {
        let items: Vec<OxiEither<i32, &str>> = vec![
            OxiEither::Left(1),
            OxiEither::Right("x"),
            OxiEither::Left(2),
            OxiEither::Right("y"),
        ];
        assert_eq!(count_lefts(&items), 2);
        assert_eq!(count_rights(&items), 2);
    }
    #[test]
    fn test_map_rights() {
        let items: Vec<OxiEither<i32, i32>> = vec![OxiEither::Left(1), OxiEither::Right(2)];
        let mapped = map_rights(&items, |x| x * 10);
        assert_eq!(mapped[0], OxiEither::Left(1));
        assert_eq!(mapped[1], OxiEither::Right(20));
    }
    #[test]
    fn test_map_lefts() {
        let items: Vec<OxiEither<i32, i32>> = vec![OxiEither::Left(3), OxiEither::Right(4)];
        let mapped = map_lefts(&items, |x| x + 100);
        assert_eq!(mapped[0], OxiEither::Left(103));
        assert_eq!(mapped[1], OxiEither::Right(4));
    }
    #[test]
    fn test_collect_rights() {
        let items: Vec<OxiEither<i32, &str>> = vec![
            OxiEither::Left(1),
            OxiEither::Right("a"),
            OxiEither::Right("b"),
        ];
        assert_eq!(collect_rights(&items), vec!["a", "b"]);
    }
    #[test]
    fn test_collect_lefts() {
        let items: Vec<OxiEither<i32, &str>> = vec![
            OxiEither::Left(10),
            OxiEither::Right("x"),
            OxiEither::Left(20),
        ];
        assert_eq!(collect_lefts(&items), vec![10, 20]);
    }
}
pub fn ei_ext_app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn ei_ext_app2(f: Expr, a: Expr, b: Expr) -> Expr {
    ei_ext_app(ei_ext_app(f, a), b)
}
pub fn ei_ext_cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub fn ei_ext_prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn ei_ext_type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn ei_ext_bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn ei_ext_nat_ty() -> Expr {
    ei_ext_cst("Nat")
}
pub fn ei_ext_arrow(dom: Expr, cod: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::Anonymous,
        Box::new(dom),
        Box::new(cod),
    )
}
pub fn ei_ext_pi(binfo: BinderInfo, nm: &str, dom: Expr, cod: Expr) -> Expr {
    Expr::Pi(binfo, Name::str(nm), Box::new(dom), Box::new(cod))
}
pub fn ei_ext_ipi(nm: &str, dom: Expr, cod: Expr) -> Expr {
    ei_ext_pi(BinderInfo::Implicit, nm, dom, cod)
}
pub fn ei_ext_dpi(nm: &str, dom: Expr, cod: Expr) -> Expr {
    ei_ext_pi(BinderInfo::Default, nm, dom, cod)
}
pub fn ei_ext_either(a: Expr, b: Expr) -> Expr {
    ei_ext_app2(ei_ext_cst("Either"), a, b)
}
pub fn ei_ext_prod(a: Expr, b: Expr) -> Expr {
    ei_ext_app2(ei_ext_cst("Prod"), a, b)
}
pub fn ei_ext_list(a: Expr) -> Expr {
    ei_ext_app(ei_ext_cst("List"), a)
}
pub fn ei_ext_option(a: Expr) -> Expr {
    ei_ext_app(ei_ext_cst("Option"), a)
}
pub fn ei_ext_result(a: Expr, e: Expr) -> Expr {
    ei_ext_app2(ei_ext_cst("Result"), a, e)
}
pub fn ei_ext_sum(a: Expr, b: Expr) -> Expr {
    ei_ext_app2(ei_ext_cst("Sum"), a, b)
}
pub fn ei_ext_eq(a: Expr, x: Expr, y: Expr) -> Expr {
    ei_ext_app2(ei_ext_app(ei_ext_cst("Eq"), a), x, y)
}
pub fn ei_ext_axiom(env: &mut Environment, name: &str, ty: Expr) -> Result<(), String> {
    env.add(Declaration::Axiom {
        name: Name::str(name),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
pub fn ei_ext_forall_ab(body: Expr) -> Expr {
    ei_ext_ipi("α", ei_ext_type0(), ei_ext_ipi("β", ei_ext_type0(), body))
}
pub fn ei_ext_forall_abg(body: Expr) -> Expr {
    ei_ext_ipi(
        "α",
        ei_ext_type0(),
        ei_ext_ipi("β", ei_ext_type0(), ei_ext_ipi("γ", ei_ext_type0(), body)),
    )
}
pub fn ei_ext_forall_abgd(body: Expr) -> Expr {
    ei_ext_ipi(
        "α",
        ei_ext_type0(),
        ei_ext_ipi(
            "β",
            ei_ext_type0(),
            ei_ext_ipi("γ", ei_ext_type0(), ei_ext_ipi("δ", ei_ext_type0(), body)),
        ),
    )
}
/// Either as coproduct: left injection inl : α → Either α β
pub fn ei_coproduct_inl(env: &mut Environment) -> Result<(), String> {
    let ty = ei_ext_forall_ab(ei_ext_arrow(
        ei_ext_bvar(1),
        ei_ext_either(ei_ext_bvar(2), ei_ext_bvar(1)),
    ));
    ei_ext_axiom(env, "Either.inl", ty)
}
/// Either as coproduct: right injection inr : β → Either α β
pub fn ei_coproduct_inr(env: &mut Environment) -> Result<(), String> {
    let ty = ei_ext_forall_ab(ei_ext_arrow(
        ei_ext_bvar(0),
        ei_ext_either(ei_ext_bvar(2), ei_ext_bvar(1)),
    ));
    ei_ext_axiom(env, "Either.inr", ty)
}
/// Coproduct universal property: unique mediating morphism
/// Either.coprod : {α β γ : Type} → (α → γ) → (β → γ) → Either α β → γ
pub fn ei_coproduct_universal(env: &mut Environment) -> Result<(), String> {
    let ty = ei_ext_forall_abg(ei_ext_dpi(
        "fl",
        ei_ext_arrow(ei_ext_bvar(2), ei_ext_bvar(0)),
        ei_ext_dpi(
            "fr",
            ei_ext_arrow(ei_ext_bvar(2), ei_ext_bvar(1)),
            ei_ext_dpi(
                "e",
                ei_ext_either(ei_ext_bvar(4), ei_ext_bvar(3)),
                ei_ext_bvar(3),
            ),
        ),
    ));
    ei_ext_axiom(env, "Either.coprod", ty)
}
/// Functor law: bimap identity
/// Either.bimap_id : {α β : Type} → ∀ e : Either α β, bimap id id e = e
pub fn ei_bimap_id(env: &mut Environment) -> Result<(), String> {
    let ty = ei_ext_forall_ab(ei_ext_dpi(
        "e",
        ei_ext_either(ei_ext_bvar(1), ei_ext_bvar(0)),
        ei_ext_eq(
            ei_ext_either(ei_ext_bvar(3), ei_ext_bvar(2)),
            ei_ext_bvar(0),
            ei_ext_bvar(0),
        ),
    ));
    ei_ext_axiom(env, "Either.bimap_id", ty)
}
/// Functor law: bimap composition (propositional)
pub fn ei_bimap_comp(env: &mut Environment) -> Result<(), String> {
    ei_ext_axiom(env, "Either.bimap_comp", ei_ext_prop())
}
/// Either as bifunctor: left map preserves composition
pub fn ei_bifunctor_left_comp(env: &mut Environment) -> Result<(), String> {
    ei_ext_axiom(env, "Either.mapLeft_comp", ei_ext_prop())
}
/// Either as bifunctor: right map preserves composition
pub fn ei_bifunctor_right_comp(env: &mut Environment) -> Result<(), String> {
    ei_ext_axiom(env, "Either.map_comp", ei_ext_prop())
}
/// Monad return (pure): right injection
/// Either.pure : {ε β : Type} → β → Either ε β
pub fn ei_monad_pure(env: &mut Environment) -> Result<(), String> {
    let ty = ei_ext_forall_ab(ei_ext_arrow(
        ei_ext_bvar(0),
        ei_ext_either(ei_ext_bvar(2), ei_ext_bvar(1)),
    ));
    ei_ext_axiom(env, "Either.pure", ty)
}
/// Monad bind: sequence with short-circuit on Left
/// Either.bind : {ε α β : Type} → Either ε α → (α → Either ε β) → Either ε β
pub fn ei_monad_bind(env: &mut Environment) -> Result<(), String> {
    let ty = ei_ext_forall_abg(ei_ext_dpi(
        "e",
        ei_ext_either(ei_ext_bvar(2), ei_ext_bvar(1)),
        ei_ext_dpi(
            "f",
            ei_ext_arrow(
                ei_ext_bvar(1),
                ei_ext_either(ei_ext_bvar(4), ei_ext_bvar(2)),
            ),
            ei_ext_either(ei_ext_bvar(4), ei_ext_bvar(2)),
        ),
    ));
    ei_ext_axiom(env, "Either.bind", ty)
}
/// Monad left identity: bind (pure a) f = f a
pub fn ei_monad_left_id(env: &mut Environment) -> Result<(), String> {
    ei_ext_axiom(env, "Either.bind_pure_left", ei_ext_prop())
}
/// Monad right identity: bind m pure = m
pub fn ei_monad_right_id(env: &mut Environment) -> Result<(), String> {
    ei_ext_axiom(env, "Either.bind_pure_right", ei_ext_prop())
}
/// Monad associativity
pub fn ei_monad_assoc(env: &mut Environment) -> Result<(), String> {
    ei_ext_axiom(env, "Either.bind_assoc", ei_ext_prop())
}
/// Applicative ap: Either.ap : {ε α β : Type} → Either ε (α → β) → Either ε α → Either ε β
pub fn ei_applicative_ap(env: &mut Environment) -> Result<(), String> {
    let ty = ei_ext_forall_abg(ei_ext_dpi(
        "ef",
        ei_ext_either(ei_ext_bvar(2), ei_ext_arrow(ei_ext_bvar(1), ei_ext_bvar(0))),
        ei_ext_dpi(
            "ea",
            ei_ext_either(ei_ext_bvar(3), ei_ext_bvar(2)),
            ei_ext_either(ei_ext_bvar(4), ei_ext_bvar(2)),
        ),
    ));
    ei_ext_axiom(env, "Either.ap", ty)
}
/// Applicative homomorphism law
pub fn ei_applicative_hom(env: &mut Environment) -> Result<(), String> {
    ei_ext_axiom(env, "Either.ap_hom", ei_ext_prop())
}
/// Applicative interchange law
pub fn ei_applicative_interchange(env: &mut Environment) -> Result<(), String> {
    ei_ext_axiom(env, "Either.ap_interchange", ei_ext_prop())
}
/// Alternative: first Right wins combinator
/// Either.alt : {α β : Type} → Either α β → Either α β → Either α β
pub fn ei_alternative_alt(env: &mut Environment) -> Result<(), String> {
    let ty = ei_ext_forall_ab(ei_ext_dpi(
        "e1",
        ei_ext_either(ei_ext_bvar(1), ei_ext_bvar(0)),
        ei_ext_dpi(
            "e2",
            ei_ext_either(ei_ext_bvar(2), ei_ext_bvar(1)),
            ei_ext_either(ei_ext_bvar(3), ei_ext_bvar(2)),
        ),
    ));
    ei_ext_axiom(env, "Either.alt", ty)
}
/// Isomorphism with Result: Either.toResult : {α β : Type} → Either α β → Result β α
pub fn ei_iso_result_to(env: &mut Environment) -> Result<(), String> {
    let ty = ei_ext_forall_ab(ei_ext_arrow(
        ei_ext_either(ei_ext_bvar(1), ei_ext_bvar(0)),
        ei_ext_result(ei_ext_bvar(1), ei_ext_bvar(2)),
    ));
    ei_ext_axiom(env, "Either.toResult", ty)
}
/// Isomorphism with Result: Either.fromResult : {α β : Type} → Result β α → Either α β
pub fn ei_iso_result_from(env: &mut Environment) -> Result<(), String> {
    let ty = ei_ext_forall_ab(ei_ext_arrow(
        ei_ext_result(ei_ext_bvar(0), ei_ext_bvar(1)),
        ei_ext_either(ei_ext_bvar(2), ei_ext_bvar(1)),
    ));
    ei_ext_axiom(env, "Either.fromResult", ty)
}
/// Isomorphism with Sum: Either.toSum : {α β : Type} → Either α β → Sum α β
pub fn ei_iso_sum_to(env: &mut Environment) -> Result<(), String> {
    let ty = ei_ext_forall_ab(ei_ext_arrow(
        ei_ext_either(ei_ext_bvar(1), ei_ext_bvar(0)),
        ei_ext_sum(ei_ext_bvar(2), ei_ext_bvar(1)),
    ));
    ei_ext_axiom(env, "Either.toSum", ty)
}
/// Isomorphism with Sum: Either.fromSum : {α β : Type} → Sum α β → Either α β
pub fn ei_iso_sum_from(env: &mut Environment) -> Result<(), String> {
    let ty = ei_ext_forall_ab(ei_ext_arrow(
        ei_ext_sum(ei_ext_bvar(1), ei_ext_bvar(0)),
        ei_ext_either(ei_ext_bvar(2), ei_ext_bvar(1)),
    ));
    ei_ext_axiom(env, "Either.fromSum", ty)
}
/// Traversable: traverse over the Right value (propositional)
pub fn ei_traversable_traverse(env: &mut Environment) -> Result<(), String> {
    ei_ext_axiom(env, "Either.traverse", ei_ext_prop())
}
/// Traversable law: traverse (pure ∘ f) = pure ∘ map f
pub fn ei_traversable_law_pure(env: &mut Environment) -> Result<(), String> {
    ei_ext_axiom(env, "Either.traverse_pure", ei_ext_prop())
}
/// Traversable law: naturality
pub fn ei_traversable_law_naturality(env: &mut Environment) -> Result<(), String> {
    ei_ext_axiom(env, "Either.traverse_naturality", ei_ext_prop())
}
/// Foldable: foldl over Either (only folds Right values)
/// Either.foldl : {α β γ : Type} → (γ → β → γ) → γ → Either α β → γ
pub fn ei_foldable_foldl(env: &mut Environment) -> Result<(), String> {
    let ty = ei_ext_forall_abg(ei_ext_dpi(
        "f",
        ei_ext_arrow(ei_ext_bvar(0), ei_ext_arrow(ei_ext_bvar(1), ei_ext_bvar(1))),
        ei_ext_dpi(
            "z",
            ei_ext_bvar(1),
            ei_ext_dpi(
                "e",
                ei_ext_either(ei_ext_bvar(4), ei_ext_bvar(3)),
                ei_ext_bvar(2),
            ),
        ),
    ));
    ei_ext_axiom(env, "Either.foldl", ty)
}
/// Foldable: foldr over Either
/// Either.foldr : {α β γ : Type} → (β → γ → γ) → γ → Either α β → γ
pub fn ei_foldable_foldr(env: &mut Environment) -> Result<(), String> {
    let ty = ei_ext_forall_abg(ei_ext_dpi(
        "f",
        ei_ext_arrow(ei_ext_bvar(1), ei_ext_arrow(ei_ext_bvar(1), ei_ext_bvar(1))),
        ei_ext_dpi(
            "z",
            ei_ext_bvar(1),
            ei_ext_dpi(
                "e",
                ei_ext_either(ei_ext_bvar(4), ei_ext_bvar(3)),
                ei_ext_bvar(2),
            ),
        ),
    ));
    ei_ext_axiom(env, "Either.foldr", ty)
}
/// Profunctor dimap: dimap a function on the left and right sides
/// Either.dimap : {α β γ δ : Type} → (γ → α) → (β → δ) → Either α β → Either γ δ
pub fn ei_profunctor_dimap(env: &mut Environment) -> Result<(), String> {
    let ty = ei_ext_forall_abgd(ei_ext_dpi(
        "fl",
        ei_ext_arrow(ei_ext_bvar(1), ei_ext_bvar(3)),
        ei_ext_dpi(
            "fr",
            ei_ext_arrow(ei_ext_bvar(3), ei_ext_bvar(1)),
            ei_ext_dpi(
                "e",
                ei_ext_either(ei_ext_bvar(5), ei_ext_bvar(4)),
                ei_ext_either(ei_ext_bvar(4), ei_ext_bvar(4)),
            ),
        ),
    ));
    ei_ext_axiom(env, "Either.dimap", ty)
}
/// Partitioning: lefts extracts all left values from a list
/// Either.lefts : {α β : Type} → List (Either α β) → List α
pub fn ei_partition_lefts(env: &mut Environment) -> Result<(), String> {
    let ty = ei_ext_forall_ab(ei_ext_arrow(
        ei_ext_list(ei_ext_either(ei_ext_bvar(1), ei_ext_bvar(0))),
        ei_ext_list(ei_ext_bvar(2)),
    ));
    ei_ext_axiom(env, "Either.lefts", ty)
}
/// Partitioning: rights extracts all right values from a list
/// Either.rights : {α β : Type} → List (Either α β) → List β
pub fn ei_partition_rights(env: &mut Environment) -> Result<(), String> {
    let ty = ei_ext_forall_ab(ei_ext_arrow(
        ei_ext_list(ei_ext_either(ei_ext_bvar(1), ei_ext_bvar(0))),
        ei_ext_list(ei_ext_bvar(1)),
    ));
    ei_ext_axiom(env, "Either.rights", ty)
}
/// Either fold (elimination)
/// Either.elim : {α β γ : Type} → (α → γ) → (β → γ) → Either α β → γ
pub fn ei_elim(env: &mut Environment) -> Result<(), String> {
    let ty = ei_ext_forall_abg(ei_ext_dpi(
        "fl",
        ei_ext_arrow(ei_ext_bvar(2), ei_ext_bvar(0)),
        ei_ext_dpi(
            "fr",
            ei_ext_arrow(ei_ext_bvar(2), ei_ext_bvar(1)),
            ei_ext_dpi(
                "e",
                ei_ext_either(ei_ext_bvar(4), ei_ext_bvar(3)),
                ei_ext_bvar(3),
            ),
        ),
    ));
    ei_ext_axiom(env, "Either.elim", ty)
}
/// Swap involution: swap (swap e) = e
pub fn ei_swap_involution(env: &mut Environment) -> Result<(), String> {
    let ty = ei_ext_forall_ab(ei_ext_dpi(
        "e",
        ei_ext_either(ei_ext_bvar(1), ei_ext_bvar(0)),
        ei_ext_eq(
            ei_ext_either(ei_ext_bvar(3), ei_ext_bvar(2)),
            ei_ext_bvar(0),
            ei_ext_bvar(0),
        ),
    ));
    ei_ext_axiom(env, "Either.swap_involution", ty)
}
/// Either as tagged union: tag accessor
/// Either.tag : {α β : Type} → Either α β → Bool
pub fn ei_tagged_union_tag(env: &mut Environment) -> Result<(), String> {
    let ty = ei_ext_forall_ab(ei_ext_arrow(
        ei_ext_either(ei_ext_bvar(1), ei_ext_bvar(0)),
        ei_ext_cst("Bool"),
    ));
    ei_ext_axiom(env, "Either.tag", ty)
}
/// Error handling: catchLeft maps Left to a new Either
/// Either.catchLeft : {ε α δ : Type} → Either ε α → (ε → Either δ α) → Either δ α
pub fn ei_error_catch_left(env: &mut Environment) -> Result<(), String> {
    let ty = ei_ext_forall_abg(ei_ext_dpi(
        "e",
        ei_ext_either(ei_ext_bvar(2), ei_ext_bvar(1)),
        ei_ext_dpi(
            "handler",
            ei_ext_arrow(
                ei_ext_bvar(3),
                ei_ext_either(ei_ext_bvar(3), ei_ext_bvar(3)),
            ),
            ei_ext_either(ei_ext_bvar(3), ei_ext_bvar(4)),
        ),
    ));
    ei_ext_axiom(env, "Either.catchLeft", ty)
}
/// Commutativity iso: Either α β ≃ Either β α (via swap)
pub fn ei_commutativity_iso(env: &mut Environment) -> Result<(), String> {
    ei_ext_axiom(env, "Either.comm_iso", ei_ext_prop())
}
/// Associativity iso: Either (Either α β) γ ≃ Either α (Either β γ)
pub fn ei_associativity_iso(env: &mut Environment) -> Result<(), String> {
    ei_ext_axiom(env, "Either.assoc_iso", ei_ext_prop())
}
/// Either.assocLeft : {α β γ : Type} → Either (Either α β) γ → Either α (Either β γ)
pub fn ei_assoc_left(env: &mut Environment) -> Result<(), String> {
    let either_ab = ei_ext_either(ei_ext_bvar(2), ei_ext_bvar(1));
    let ty = ei_ext_forall_abg(ei_ext_dpi(
        "e",
        ei_ext_either(either_ab, ei_ext_bvar(0)),
        ei_ext_either(
            ei_ext_bvar(3),
            ei_ext_either(ei_ext_bvar(3), ei_ext_bvar(2)),
        ),
    ));
    ei_ext_axiom(env, "Either.assocLeft", ty)
}
/// Either.assocRight : {α β γ : Type} → Either α (Either β γ) → Either (Either α β) γ
pub fn ei_assoc_right(env: &mut Environment) -> Result<(), String> {
    let either_bg = ei_ext_either(ei_ext_bvar(1), ei_ext_bvar(0));
    let ty = ei_ext_forall_abg(ei_ext_dpi(
        "e",
        ei_ext_either(ei_ext_bvar(2), either_bg),
        ei_ext_either(
            ei_ext_either(ei_ext_bvar(4), ei_ext_bvar(3)),
            ei_ext_bvar(2),
        ),
    ));
    ei_ext_axiom(env, "Either.assocRight", ty)
}
/// Either with Void: Either Void β ≃ β
/// Either.elimVoidLeft : {β : Type} → Either Void β → β
pub fn ei_void_elim_left(env: &mut Environment) -> Result<(), String> {
    let ty = ei_ext_ipi(
        "β",
        ei_ext_type0(),
        ei_ext_arrow(
            ei_ext_either(ei_ext_cst("Void"), ei_ext_bvar(0)),
            ei_ext_bvar(1),
        ),
    );
    ei_ext_axiom(env, "Either.elimVoidLeft", ty)
}
/// Either.introVoidLeft : {β : Type} → β → Either Void β
pub fn ei_void_intro_left(env: &mut Environment) -> Result<(), String> {
    let ty = ei_ext_ipi(
        "β",
        ei_ext_type0(),
        ei_ext_arrow(
            ei_ext_bvar(0),
            ei_ext_either(ei_ext_cst("Void"), ei_ext_bvar(1)),
        ),
    );
    ei_ext_axiom(env, "Either.introVoidLeft", ty)
}
/// Distributivity over product (propositional)
pub fn ei_distrib_over_prod(env: &mut Environment) -> Result<(), String> {
    ei_ext_axiom(env, "Either.distrib_prod", ei_ext_prop())
}
/// Either.distribLeft : {α β γ : Type} → Either α (β × γ) → (Either α β) × (Either α γ)
pub fn ei_distrib_left(env: &mut Environment) -> Result<(), String> {
    let prod_bg = ei_ext_prod(ei_ext_bvar(1), ei_ext_bvar(0));
    let ty = ei_ext_forall_abg(ei_ext_dpi(
        "e",
        ei_ext_either(ei_ext_bvar(2), prod_bg),
        ei_ext_prod(
            ei_ext_either(ei_ext_bvar(4), ei_ext_bvar(3)),
            ei_ext_either(ei_ext_bvar(4), ei_ext_bvar(3)),
        ),
    ));
    ei_ext_axiom(env, "Either.distribLeft", ty)
}
/// do-notation bind alias: >>= operator type
/// Either.seqBind : {ε α β : Type} → Either ε α → (α → Either ε β) → Either ε β
pub fn ei_do_seq_bind(env: &mut Environment) -> Result<(), String> {
    let ty = ei_ext_forall_abg(ei_ext_dpi(
        "m",
        ei_ext_either(ei_ext_bvar(2), ei_ext_bvar(1)),
        ei_ext_dpi(
            "f",
            ei_ext_arrow(
                ei_ext_bvar(1),
                ei_ext_either(ei_ext_bvar(4), ei_ext_bvar(2)),
            ),
            ei_ext_either(ei_ext_bvar(4), ei_ext_bvar(2)),
        ),
    ));
    ei_ext_axiom(env, "Either.seqBind", ty)
}
/// Kleisli composition: (>=>) for Either monad
/// Either.kleisliComp : {ε α β γ : Type} →
///   (α → Either ε β) → (β → Either ε γ) → α → Either ε γ
pub fn ei_kleisli_comp(env: &mut Environment) -> Result<(), String> {
    let ty = ei_ext_forall_abgd(ei_ext_dpi(
        "f",
        ei_ext_arrow(
            ei_ext_bvar(3),
            ei_ext_either(ei_ext_bvar(3), ei_ext_bvar(2)),
        ),
        ei_ext_dpi(
            "g",
            ei_ext_arrow(
                ei_ext_bvar(3),
                ei_ext_either(ei_ext_bvar(4), ei_ext_bvar(2)),
            ),
            ei_ext_arrow(
                ei_ext_bvar(5),
                ei_ext_either(ei_ext_bvar(5), ei_ext_bvar(3)),
            ),
        ),
    ));
    ei_ext_axiom(env, "Either.kleisliComp", ty)
}
/// Kleisli identity law
pub fn ei_kleisli_id(env: &mut Environment) -> Result<(), String> {
    ei_ext_axiom(env, "Either.kleisli_id_law", ei_ext_prop())
}
/// EitherT monad transformer: run function type (propositional)
pub fn ei_eithert_run(env: &mut Environment) -> Result<(), String> {
    ei_ext_axiom(env, "EitherT.run", ei_ext_prop())
}
/// EitherT.lift (propositional)
pub fn ei_eithert_lift(env: &mut Environment) -> Result<(), String> {
    ei_ext_axiom(env, "EitherT.lift", ei_ext_prop())
}
/// EitherT.bind (propositional)
pub fn ei_eithert_bind(env: &mut Environment) -> Result<(), String> {
    ei_ext_axiom(env, "EitherT.bind", ei_ext_prop())
}
/// Either.sequenceList : {α β : Type} → List (Either α β) → Either α (List β)
pub fn ei_sequence_list(env: &mut Environment) -> Result<(), String> {
    let ty = ei_ext_forall_ab(ei_ext_arrow(
        ei_ext_list(ei_ext_either(ei_ext_bvar(1), ei_ext_bvar(0))),
        ei_ext_either(ei_ext_bvar(2), ei_ext_list(ei_ext_bvar(1))),
    ));
    ei_ext_axiom(env, "Either.sequenceList", ty)
}
/// Either.traverseList : {ε α β : Type} → (α → Either ε β) → List α → Either ε (List β)
pub fn ei_traverse_list(env: &mut Environment) -> Result<(), String> {
    let ty = ei_ext_forall_abg(ei_ext_dpi(
        "f",
        ei_ext_arrow(
            ei_ext_bvar(1),
            ei_ext_either(ei_ext_bvar(2), ei_ext_bvar(1)),
        ),
        ei_ext_dpi(
            "xs",
            ei_ext_list(ei_ext_bvar(2)),
            ei_ext_either(ei_ext_bvar(4), ei_ext_list(ei_ext_bvar(2))),
        ),
    ));
    ei_ext_axiom(env, "Either.traverseList", ty)
}
/// Either select combinator (Selective)
/// Either.select : {ε α β : Type} → Either ε α → Either ε (α → β) → Either ε β
pub fn ei_select_combinator(env: &mut Environment) -> Result<(), String> {
    let ty = ei_ext_forall_abg(ei_ext_dpi(
        "e",
        ei_ext_either(ei_ext_bvar(2), ei_ext_bvar(1)),
        ei_ext_dpi(
            "f",
            ei_ext_either(ei_ext_bvar(3), ei_ext_arrow(ei_ext_bvar(2), ei_ext_bvar(1))),
            ei_ext_either(ei_ext_bvar(4), ei_ext_bvar(2)),
        ),
    ));
    ei_ext_axiom(env, "Either.select", ty)
}
/// Select law: select (Right x) f = map ($ x) f
pub fn ei_select_law_right(env: &mut Environment) -> Result<(), String> {
    ei_ext_axiom(env, "Either.select_right_law", ei_ext_prop())
}
/// Select law: select (Left e) (Left h) = Left e
pub fn ei_select_law_left(env: &mut Environment) -> Result<(), String> {
    ei_ext_axiom(env, "Either.select_left_law", ei_ext_prop())
}
/// Nat-indexed sum type via iterated Either
/// Either.natSum : Nat → Type → Type → Type
pub fn ei_nat_sum(env: &mut Environment) -> Result<(), String> {
    let ty = ei_ext_arrow(
        ei_ext_nat_ty(),
        ei_ext_arrow(ei_ext_type0(), ei_ext_arrow(ei_ext_type0(), ei_ext_type0())),
    );
    ei_ext_axiom(env, "Either.natSum", ty)
}
/// Either.mapBoth : {α β γ : Type} → (α → γ) → (β → γ) → Either α β → γ
pub fn ei_map_both(env: &mut Environment) -> Result<(), String> {
    let ty = ei_ext_forall_abg(ei_ext_dpi(
        "fl",
        ei_ext_arrow(ei_ext_bvar(2), ei_ext_bvar(0)),
        ei_ext_dpi(
            "fr",
            ei_ext_arrow(ei_ext_bvar(2), ei_ext_bvar(1)),
            ei_ext_dpi(
                "e",
                ei_ext_either(ei_ext_bvar(4), ei_ext_bvar(3)),
                ei_ext_bvar(3),
            ),
        ),
    ));
    ei_ext_axiom(env, "Either.mapBoth", ty)
}
/// Either.joinWith : {α β : Type} → (α → β) → Either α β → β
pub fn ei_join_with(env: &mut Environment) -> Result<(), String> {
    let ty = ei_ext_forall_ab(ei_ext_dpi(
        "f",
        ei_ext_arrow(ei_ext_bvar(1), ei_ext_bvar(0)),
        ei_ext_dpi(
            "e",
            ei_ext_either(ei_ext_bvar(2), ei_ext_bvar(1)),
            ei_ext_bvar(1),
        ),
    ));
    ei_ext_axiom(env, "Either.joinWith", ty)
}
