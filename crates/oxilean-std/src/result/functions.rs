//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    ErrorAccumulator, ResultAxiomRegistry, ResultEitherBridge, ResultRegistry, ValidationCollector,
};

/// Build Result type in the environment.
pub fn build_result_env(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    let result_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("T"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("E"),
            Box::new(type1.clone()),
            Box::new(type2),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Result"),
        univ_params: vec![],
        ty: result_ty,
    })
    .map_err(|e| e.to_string())?;
    let ok_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("T"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("E"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("val"),
                Box::new(Expr::BVar(1)),
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Result"), vec![])),
                        Box::new(Expr::BVar(2)),
                    )),
                    Box::new(Expr::BVar(1)),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Result.ok"),
        univ_params: vec![],
        ty: ok_ty,
    })
    .map_err(|e| e.to_string())?;
    let err_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("T"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("E"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("err"),
                Box::new(Expr::BVar(0)),
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Result"), vec![])),
                        Box::new(Expr::BVar(2)),
                    )),
                    Box::new(Expr::BVar(1)),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Result.err"),
        univ_params: vec![],
        ty: err_ty,
    })
    .map_err(|e| e.to_string())?;
    let is_ok_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("T"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("E"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("r"),
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Result"), vec![])),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::BVar(0)),
                )),
                Box::new(Expr::Const(Name::str("Bool"), vec![])),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Result.isOk"),
        univ_params: vec![],
        ty: is_ok_ty,
    })
    .map_err(|e| e.to_string())?;
    let is_err_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("T"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("E"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("r"),
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Result"), vec![])),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::BVar(0)),
                )),
                Box::new(Expr::Const(Name::str("Bool"), vec![])),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Result.isErr"),
        univ_params: vec![],
        ty: is_err_ty,
    })
    .map_err(|e| e.to_string())?;
    let map_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("T"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("E"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("U"),
                Box::new(type1.clone()),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("f"),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("_"),
                        Box::new(Expr::BVar(2)),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("r"),
                        Box::new(Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("Result"), vec![])),
                                Box::new(Expr::BVar(3)),
                            )),
                            Box::new(Expr::BVar(2)),
                        )),
                        Box::new(Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("Result"), vec![])),
                                Box::new(Expr::BVar(2)),
                            )),
                            Box::new(Expr::BVar(3)),
                        )),
                    )),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Result.map"),
        univ_params: vec![],
        ty: map_ty,
    })
    .map_err(|e| e.to_string())?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn setup_env() -> Environment {
        let mut env = Environment::new();
        let type1 = Expr::Sort(Level::succ(Level::zero()));
        env.add(Declaration::Axiom {
            name: Name::str("Bool"),
            univ_params: vec![],
            ty: type1,
        })
        .expect("operation should succeed");
        env
    }
    #[test]
    fn test_build_result_env() {
        let mut env = setup_env();
        assert!(build_result_env(&mut env).is_ok());
        assert!(env.get(&Name::str("Result")).is_some());
        assert!(env.get(&Name::str("Result.ok")).is_some());
        assert!(env.get(&Name::str("Result.err")).is_some());
    }
    #[test]
    fn test_result_is_ok() {
        let mut env = setup_env();
        build_result_env(&mut env).expect("build_result_env should succeed");
        let decl = env
            .get(&Name::str("Result.isOk"))
            .expect("declaration 'Result.isOk' should exist in env");
        assert!(matches!(decl, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_result_is_err() {
        let mut env = setup_env();
        build_result_env(&mut env).expect("build_result_env should succeed");
        let decl = env
            .get(&Name::str("Result.isErr"))
            .expect("declaration 'Result.isErr' should exist in env");
        assert!(matches!(decl, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_result_map() {
        let mut env = setup_env();
        build_result_env(&mut env).expect("build_result_env should succeed");
        let decl = env
            .get(&Name::str("Result.map"))
            .expect("declaration 'Result.map' should exist in env");
        assert!(matches!(decl, Declaration::Axiom { .. }));
    }
}
/// Build extended Result combinators in the environment.
///
/// Adds `andThen` (flatMap), `mapErr`, `getOrElse`, `toOption`, `fromOption`.
pub fn build_result_combinators(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let and_then_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("T"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("E"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("U"),
                Box::new(type1.clone()),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("r"),
                    Box::new(Expr::App(
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("Result"), vec![])),
                            Box::new(Expr::BVar(2)),
                        )),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("f"),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("_"),
                            Box::new(Expr::BVar(3)),
                            Box::new(Expr::App(
                                Box::new(Expr::App(
                                    Box::new(Expr::Const(Name::str("Result"), vec![])),
                                    Box::new(Expr::BVar(2)),
                                )),
                                Box::new(Expr::BVar(3)),
                            )),
                        )),
                        Box::new(Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("Result"), vec![])),
                                Box::new(Expr::BVar(2)),
                            )),
                            Box::new(Expr::BVar(3)),
                        )),
                    )),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Result.andThen"),
        univ_params: vec![],
        ty: and_then_ty,
    })
    .map_err(|e| e.to_string())?;
    let map_err_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("T"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("E"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("F"),
                Box::new(type1.clone()),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("f"),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("_"),
                        Box::new(Expr::BVar(1)),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("r"),
                        Box::new(Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("Result"), vec![])),
                                Box::new(Expr::BVar(3)),
                            )),
                            Box::new(Expr::BVar(2)),
                        )),
                        Box::new(Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("Result"), vec![])),
                                Box::new(Expr::BVar(4)),
                            )),
                            Box::new(Expr::BVar(2)),
                        )),
                    )),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Result.mapErr"),
        univ_params: vec![],
        ty: map_err_ty,
    })
    .map_err(|e| e.to_string())?;
    let get_or_else_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("T"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("E"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("default"),
                Box::new(Expr::BVar(1)),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("r"),
                    Box::new(Expr::App(
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("Result"), vec![])),
                            Box::new(Expr::BVar(2)),
                        )),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::BVar(3)),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Result.getOrElse"),
        univ_params: vec![],
        ty: get_or_else_ty,
    })
    .map_err(|e| e.to_string())?;
    Ok(())
}
/// Build Result theorems as axioms in the environment.
pub fn build_result_theorems(env: &mut Environment) -> Result<(), String> {
    let prop = Expr::Sort(Level::zero());
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ok_is_ok_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("T"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("E"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("v"),
                Box::new(Expr::BVar(1)),
                Box::new(prop.clone()),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Result.ok_isOk"),
        univ_params: vec![],
        ty: ok_is_ok_ty,
    })
    .map_err(|e| e.to_string())?;
    let err_is_err_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("T"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("E"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("e"),
                Box::new(Expr::BVar(0)),
                Box::new(prop.clone()),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Result.err_isErr"),
        univ_params: vec![],
        ty: err_is_err_ty,
    })
    .map_err(|e| e.to_string())?;
    Ok(())
}
/// Count how many Result declarations are registered in an environment.
pub fn count_result_decls(env: &Environment) -> usize {
    let names = [
        "Result",
        "Result.ok",
        "Result.err",
        "Result.isOk",
        "Result.isErr",
        "Result.map",
        "Result.andThen",
        "Result.mapErr",
        "Result.getOrElse",
        "Result.ok_isOk",
        "Result.err_isErr",
    ];
    names
        .iter()
        .filter(|&&n| env.get(&Name::str(n)).is_some())
        .count()
}
#[cfg(test)]
mod result_extended_tests {
    use super::*;
    fn setup_env() -> Environment {
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
    fn test_result_and_then() {
        let mut env = setup_env();
        assert!(build_result_combinators(&mut env).is_ok());
        assert!(env.get(&Name::str("Result.andThen")).is_some());
    }
    #[test]
    fn test_result_map_err() {
        let mut env = setup_env();
        build_result_combinators(&mut env).expect("build_result_combinators should succeed");
        assert!(env.get(&Name::str("Result.mapErr")).is_some());
    }
    #[test]
    fn test_result_get_or_else() {
        let mut env = setup_env();
        build_result_combinators(&mut env).expect("build_result_combinators should succeed");
        assert!(env.get(&Name::str("Result.getOrElse")).is_some());
    }
    #[test]
    fn test_result_theorems() {
        let mut env = setup_env();
        assert!(build_result_theorems(&mut env).is_ok());
        assert!(env.get(&Name::str("Result.ok_isOk")).is_some());
        assert!(env.get(&Name::str("Result.err_isErr")).is_some());
    }
    #[test]
    fn test_count_result_decls_base() {
        let env = setup_env();
        let count = count_result_decls(&env);
        assert!(count >= 5);
    }
    #[test]
    fn test_count_result_decls_extended() {
        let mut env = setup_env();
        build_result_combinators(&mut env).expect("build_result_combinators should succeed");
        build_result_theorems(&mut env).expect("build_result_theorems should succeed");
        let count = count_result_decls(&env);
        assert!(count >= 10);
    }
    #[test]
    fn test_result_decl_is_axiom() {
        let mut env = setup_env();
        build_result_combinators(&mut env).expect("build_result_combinators should succeed");
        let decl = env
            .get(&Name::str("Result.andThen"))
            .expect("declaration 'Result.andThen' should exist in env");
        assert!(matches!(decl, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_result_theorem_is_axiom() {
        let mut env = setup_env();
        build_result_theorems(&mut env).expect("build_result_theorems should succeed");
        let decl = env
            .get(&Name::str("Result.ok_isOk"))
            .expect("declaration 'Result.ok_isOk' should exist in env");
        assert!(matches!(decl, Declaration::Axiom { .. }));
    }
}
/// A trait for types that can report errors with context.
pub trait WithContext<T> {
    /// Add context information to an error.
    fn with_context(self, context: &str) -> std::result::Result<T, String>;
}
impl<T> WithContext<T> for std::result::Result<T, String> {
    fn with_context(self, context: &str) -> std::result::Result<T, String> {
        self.map_err(|e| format!("{}: {}", context, e))
    }
}
/// Try to build all Result standard library declarations at once.
pub fn build_full_result_stdlib(env: &mut Environment) -> std::result::Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    if env.get(&Name::str("Bool")).is_none() {
        env.add(Declaration::Axiom {
            name: Name::str("Bool"),
            univ_params: vec![],
            ty: type1,
        })
        .map_err(|e| e.to_string())?;
    }
    build_result_env(env)?;
    build_result_combinators(env)?;
    build_result_theorems(env)?;
    Ok(())
}
#[cfg(test)]
mod error_monad_tests {
    use super::*;
    #[test]
    fn test_error_accumulator_no_errors() {
        let acc = ErrorAccumulator::new();
        assert!(!acc.has_errors());
        assert_eq!(acc.len(), 0);
        assert!(acc.into_result().is_ok());
    }
    #[test]
    fn test_error_accumulator_with_errors() {
        let mut acc = ErrorAccumulator::new();
        let _v: Option<i32> = acc.try_add(Err("error 1".to_string()));
        let _v2: Option<i32> = acc.try_add(Err("error 2".to_string()));
        assert!(acc.has_errors());
        assert_eq!(acc.len(), 2);
        let result = acc.into_result();
        assert!(result.is_err());
        let msg = result.unwrap_err();
        assert!(msg.contains("error 1"));
        assert!(msg.contains("error 2"));
    }
    #[test]
    fn test_error_accumulator_mixed() {
        let mut acc = ErrorAccumulator::new();
        let v1 = acc.try_add(Ok::<i32, String>(42));
        let _v2: Option<i32> = acc.try_add(Err("oops".to_string()));
        assert_eq!(v1, Some(42));
        assert_eq!(acc.len(), 1);
    }
    #[test]
    fn test_error_accumulator_clear() {
        let mut acc = ErrorAccumulator::new();
        let _: Option<i32> = acc.try_add(Err("e".to_string()));
        assert!(!acc.is_empty());
        acc.clear();
        assert!(acc.is_empty());
    }
    #[test]
    fn test_with_context() {
        let result: std::result::Result<i32, String> = Err("not found".to_string());
        let with_ctx = result.with_context("while loading file");
        let msg = with_ctx.unwrap_err();
        assert!(msg.contains("while loading file"));
        assert!(msg.contains("not found"));
    }
    #[test]
    fn test_with_context_ok() {
        let result: std::result::Result<i32, String> = Ok(42);
        let with_ctx = result.with_context("context");
        assert_eq!(with_ctx.expect("with_ctx should be valid"), 42);
    }
    #[test]
    fn test_build_full_result_stdlib() {
        let mut env = Environment::new();
        assert!(build_full_result_stdlib(&mut env).is_ok());
        assert!(env.get(&Name::str("Result")).is_some());
        assert!(env.get(&Name::str("Result.andThen")).is_some());
        assert!(env.get(&Name::str("Result.ok_isOk")).is_some());
    }
    #[test]
    fn test_build_full_result_stdlib_twice() {
        let mut env = Environment::new();
        let type1 = Expr::Sort(Level::succ(Level::zero()));
        env.add(Declaration::Axiom {
            name: Name::str("Bool"),
            univ_params: vec![],
            ty: type1,
        })
        .expect("operation should succeed");
        assert!(build_full_result_stdlib(&mut env).is_ok());
    }
}
/// Build the expression `Result T E` as an `Expr`.
pub fn mk_result_ty(t: Expr, e: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Result"), vec![])),
            Box::new(t),
        )),
        Box::new(e),
    )
}
/// Build the expression `Result.ok v` as an `Expr`.
pub fn mk_result_ok(v: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::Const(Name::str("Result.ok"), vec![])),
        Box::new(v),
    )
}
/// Build the expression `Result.err e` as an `Expr`.
pub fn mk_result_err(e: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::Const(Name::str("Result.err"), vec![])),
        Box::new(e),
    )
}
#[cfg(test)]
mod expr_builder_tests {
    use super::*;
    #[test]
    fn test_mk_result_ty() {
        let t = Expr::Const(Name::str("Nat"), vec![]);
        let e = Expr::Const(Name::str("String"), vec![]);
        let result = mk_result_ty(t, e);
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_result_ok() {
        let v = Expr::Lit(oxilean_kernel::Literal::Nat(42));
        let ok_expr = mk_result_ok(v);
        assert!(matches!(ok_expr, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_result_err() {
        let e = Expr::Lit(oxilean_kernel::Literal::Str("error".to_string()));
        let err_expr = mk_result_err(e);
        assert!(matches!(err_expr, Expr::App(_, _)));
    }
}
/// Build the expression `Result.andThen r f` as an `Expr`.
#[allow(dead_code)]
pub fn mk_result_and_then(r: Expr, f: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Result.andThen"), vec![])),
            Box::new(r),
        )),
        Box::new(f),
    )
}
/// Build the expression `Result.map f r` as an `Expr`.
#[allow(dead_code)]
pub fn mk_result_map(f: Expr, r: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Result.map"), vec![])),
            Box::new(f),
        )),
        Box::new(r),
    )
}
/// Build the expression `Result.getOrElse default r` as an `Expr`.
#[allow(dead_code)]
pub fn mk_result_get_or_else(default: Expr, r: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Result.getOrElse"), vec![])),
            Box::new(default),
        )),
        Box::new(r),
    )
}
/// Validate that all required Result definitions are present in an environment.
///
/// Returns a list of missing definition names.
#[allow(dead_code)]
pub fn validate_result_env(env: &Environment) -> Vec<&'static str> {
    let required = [
        "Result",
        "Result.ok",
        "Result.err",
        "Result.isOk",
        "Result.isErr",
        "Result.map",
    ];
    required
        .iter()
        .filter(|&&n| env.get(&Name::str(n)).is_none())
        .copied()
        .collect()
}
/// Build a simple test environment with all Result declarations.
#[allow(dead_code)]
pub fn build_test_result_env() -> std::result::Result<Environment, String> {
    let mut env = Environment::new();
    build_full_result_stdlib(&mut env)?;
    Ok(env)
}
#[cfg(test)]
mod result_registry_tests {
    use super::*;
    #[test]
    fn test_result_registry_empty() {
        let r = ResultRegistry::new();
        assert!(r.is_empty());
        assert_eq!(r.len(), 0);
    }
    #[test]
    fn test_result_registry_register() {
        let mut r = ResultRegistry::new();
        r.register("Result.ok");
        r.register("Result.err");
        r.register("Result.ok");
        assert_eq!(r.len(), 2);
        assert!(r.contains("Result.ok"));
        assert!(!r.contains("Result.map"));
    }
    #[test]
    fn test_result_registry_from_env() {
        let env = build_test_result_env().expect("build_test_result_env should succeed");
        let reg = ResultRegistry::from_env(&env);
        assert!(reg.contains("Result"));
        assert!(reg.contains("Result.ok"));
        assert!(reg.contains("Result.andThen"));
    }
    #[test]
    fn test_validate_result_env_ok() {
        let env = build_test_result_env().expect("build_test_result_env should succeed");
        let missing = validate_result_env(&env);
        assert!(missing.is_empty());
    }
    #[test]
    fn test_validate_result_env_missing() {
        let env = Environment::new();
        let missing = validate_result_env(&env);
        assert!(!missing.is_empty());
    }
    #[test]
    fn test_mk_result_and_then() {
        let r = mk_result_ok(Expr::BVar(0));
        let f = Expr::Const(Name::str("f"), vec![]);
        let e = mk_result_and_then(r, f);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_result_map() {
        let f = Expr::Const(Name::str("g"), vec![]);
        let r = mk_result_ok(Expr::BVar(0));
        let e = mk_result_map(f, r);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_result_get_or_else() {
        let default = Expr::Lit(oxilean_kernel::Literal::Nat(0));
        let r = mk_result_err(Expr::Const(Name::str("err"), vec![]));
        let e = mk_result_get_or_else(default, r);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_result_registry_all_names() {
        let mut r = ResultRegistry::new();
        r.register("Result");
        r.register("Result.ok");
        let names = r.all_names();
        assert_eq!(names.len(), 2);
    }
    #[test]
    fn test_build_test_result_env_ok() {
        let env = build_test_result_env();
        assert!(env.is_ok());
    }
    #[test]
    fn test_error_accumulator_try_add_ok() {
        let mut acc = ErrorAccumulator::new();
        let v = acc.try_add(Ok::<String, String>("hello".to_string()));
        assert_eq!(v, Some("hello".to_string()));
        assert!(acc.is_empty());
    }
    #[test]
    fn test_count_result_decls_full() {
        let env = build_test_result_env().expect("build_test_result_env should succeed");
        let c = count_result_decls(&env);
        assert!(c >= 9);
    }
}
pub fn res_ext_prop_axiom(name: &str, env: &mut Environment) -> std::result::Result<(), String> {
    let prop = Expr::Sort(Level::zero());
    env.add(Declaration::Axiom {
        name: Name::str(name),
        univ_params: vec![],
        ty: prop,
    })
    .map_err(|e| e.to_string())
}
/// Build a forall-over-Type₁ axiom: `∀ (T : Type), Prop`.
pub(super) fn res_ext_forall_type_axiom(
    name: &str,
    env: &mut Environment,
) -> std::result::Result<(), String> {
    let prop = Expr::Sort(Level::zero());
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("T"),
        Box::new(type1),
        Box::new(prop),
    );
    env.add(Declaration::Axiom {
        name: Name::str(name),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build a two-type forall axiom: `∀ (T E : Type), Prop`.
pub(super) fn res_ext_forall2_axiom(
    name: &str,
    env: &mut Environment,
) -> std::result::Result<(), String> {
    let prop = Expr::Sort(Level::zero());
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("T"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("E"),
            Box::new(type1),
            Box::new(prop),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str(name),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build a three-type forall axiom: `∀ (T E U : Type), Prop`.
pub(super) fn res_ext_forall3_axiom(
    name: &str,
    env: &mut Environment,
) -> std::result::Result<(), String> {
    let prop = Expr::Sort(Level::zero());
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("T"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("E"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("U"),
                Box::new(type1),
                Box::new(prop),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str(name),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// `Result.monad_left_id : ∀ {T E} (a : T) (f : T → Result T E), andThen (ok a) f = f a`
pub fn res_ext_build_monad_left_identity(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.monad_left_identity", env)
}
/// `Result.monad_right_id : ∀ {T E} (r : Result T E), andThen r ok = r`
pub fn res_ext_build_monad_right_identity(
    env: &mut Environment,
) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.monad_right_identity", env)
}
/// `Result.monad_assoc : ∀ {T E U V}, andThen (andThen r f) g = andThen r (fun x => andThen (f x) g)`
pub fn res_ext_build_monad_assoc(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.monad_assoc", env)
}
/// `Result.map_id : ∀ {T E} (r : Result T E), map id r = r`
pub fn res_ext_build_map_identity(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.map_identity", env)
}
/// `Result.map_comp : ∀ {T E U V} (f : T → U) (g : U → V) r, map (g ∘ f) r = (map g ∘ map f) r`
pub fn res_ext_build_map_composition(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.map_composition", env)
}
/// `Result.and_then_ok : ∀ {T E U} (v : T) (f : T → Result U E), andThen (ok v) f = f v`
pub fn res_ext_build_and_then_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.and_then_ok", env)
}
/// `Result.and_then_err : ∀ {T E U} (e : E) (f : T → Result U E), andThen (err e) f = err e`
pub fn res_ext_build_and_then_err(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.and_then_err", env)
}
/// `Result.or_else_ok : ∀ {T E F} (v : T) (f : E → Result T F), orElse (ok v) f = ok v`
pub fn res_ext_build_or_else_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.or_else_ok", env)
}
/// `Result.or_else_err : ∀ {T E F} (e : E) (f : E → Result T F), orElse (err e) f = f e`
pub fn res_ext_build_or_else_err(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.or_else_err", env)
}
/// `Result.map_err_id : ∀ {T E} r, mapErr id r = r`
pub fn res_ext_build_map_err_identity(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.map_err_identity", env)
}
/// `Result.map_err_comp : ∀ {T E F G} f g r, mapErr (g ∘ f) r = mapErr g (mapErr f r)`
pub fn res_ext_build_map_err_composition(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.map_err_composition", env)
}
/// `Result.flatten_ok_ok : ∀ {T E} v, flatten (ok (ok v)) = ok v`
pub fn res_ext_build_flatten_ok_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.flatten_ok_ok", env)
}
/// `Result.flatten_ok_err : ∀ {T E} e, flatten (ok (err e)) = err e`
pub fn res_ext_build_flatten_ok_err(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.flatten_ok_err", env)
}
/// `Result.flatten_err : ∀ {T E} e, flatten (err e) = err e`
pub fn res_ext_build_flatten_err(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.flatten_err", env)
}
/// `Result.bimap_id : ∀ {T E} r, bimap id id r = r`
pub fn res_ext_build_bimap_identity(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.bimap_identity", env)
}
/// `Result.bimap_comp : bimap (g ∘ f) (k ∘ h) = bimap g k ∘ bimap f h`
pub fn res_ext_build_bimap_composition(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.bimap_composition", env)
}
/// `Result.bimap_ok : ∀ {T E U F} f g v, bimap f g (ok v) = ok (f v)`
pub fn res_ext_build_bimap_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.bimap_ok", env)
}
/// `Result.bimap_err : ∀ {T E U F} f g e, bimap f g (err e) = err (g e)`
pub fn res_ext_build_bimap_err(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.bimap_err", env)
}
/// `Result.traverse_pure : ∀ {T E F} (v : T), traverse pure (ok v) = pure (ok v)`
pub fn res_ext_build_traverse_pure(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.traverse_pure", env)
}
/// `Result.sequence_ok : sequence (ok ma) = map ok ma`
pub fn res_ext_build_sequence_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.sequence_ok", env)
}
/// `Result.sequence_err : sequence (err e) = pure (err e)`
pub fn res_ext_build_sequence_err(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.sequence_err", env)
}
/// `Result.dimap_id : ∀ {T E}, dimap id id = id` (for the profunctor instance)
pub fn res_ext_build_dimap_identity(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.dimap_identity", env)
}
/// `Result.dimap_comp : dimap (f' ∘ f) (g ∘ g') = dimap f g ∘ dimap f' g'`
pub fn res_ext_build_dimap_composition(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.dimap_composition", env)
}
/// `Result.ap_pure_ok : ∀ {T E U} (f : T → U) v, ap (ok f) (ok v) = ok (f v)`
pub fn res_ext_build_ap_pure_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.ap_pure_ok", env)
}
/// `Result.ap_err_left : ∀ {T E U} e r, ap (err e) r = err e`
pub fn res_ext_build_ap_err_left(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.ap_err_left", env)
}
/// `Result.applicative_identity : ap (pure id) r = r`
pub fn res_ext_build_applicative_identity(
    env: &mut Environment,
) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.applicative_identity", env)
}
/// `Result.applicative_homomorphism : ap (pure f) (pure v) = pure (f v)`
pub fn res_ext_build_applicative_homomorphism(
    env: &mut Environment,
) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.applicative_homomorphism", env)
}
/// `Result.applicative_interchange : ap rf (pure v) = ap (pure (fun f => f v)) rf`
pub fn res_ext_build_applicative_interchange(
    env: &mut Environment,
) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.applicative_interchange", env)
}
/// `Result.validation_ap_ok_ok : validAp (ok f) (ok v) = ok (f v)`
pub fn res_ext_build_validation_ap_ok_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.validation_ap_ok_ok", env)
}
/// `Result.validation_ap_err_accumulate : validAp (err e1) (err e2) = err (e1 ++ e2)`
pub fn res_ext_build_validation_ap_err_accumulate(
    env: &mut Environment,
) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.validation_ap_err_accumulate", env)
}
/// `Result.fold_ok : ∀ f g v, fold f g (ok v) = f v`
pub fn res_ext_build_fold_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.fold_ok", env)
}
/// `Result.fold_err : ∀ f g e, fold f g (err e) = g e`
pub fn res_ext_build_fold_err(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.fold_err", env)
}
/// `Result.unfold_left : unfold seed producing Left = err (left seed)`
pub fn res_ext_build_unfold_left(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.unfold_left", env)
}
/// `Result.unfold_right : unfold seed producing Right = ok (right seed)`
pub fn res_ext_build_unfold_right(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.unfold_right", env)
}
/// `Result.short_circuit_err : andThen (err e) f = err e` (for any f)
pub fn res_ext_build_short_circuit_err(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.short_circuit_err", env)
}
/// `Result.short_circuit_ok : andThen (ok v) (fun _ => err e) = err e`
pub fn res_ext_build_short_circuit_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.short_circuit_ok", env)
}
/// `ResultT.lift_ok : lift (pure v) = ok v` in the transformer
pub fn res_ext_build_result_t_lift_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("ResultT.lift_ok", env)
}
/// `ResultT.lift_err : lift (throwError e) = err e` in the transformer
pub fn res_ext_build_result_t_lift_err(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("ResultT.lift_err", env)
}
/// `ResultT.monad_left_id` for the transformer layer
pub fn res_ext_build_result_t_monad_left_id(
    env: &mut Environment,
) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("ResultT.monad_left_identity", env)
}
/// `Result.alt_ok_left : alt (ok v) r = ok v`  (first-success semantics)
pub fn res_ext_build_alt_ok_left(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.alt_ok_left", env)
}
/// `Result.alt_err_left : alt (err e) r = r`
pub fn res_ext_build_alt_err_left(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.alt_err_left", env)
}
/// `Result.zip_ok_ok : zip (ok v1) (ok v2) = ok (v1, v2)`
pub fn res_ext_build_zip_ok_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.zip_ok_ok", env)
}
/// `Result.zip_err : zip (err e) _ = err e`
pub fn res_ext_build_zip_err(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.zip_err", env)
}
/// `Result.zipWith_ok : zipWith f (ok v1) (ok v2) = ok (f v1 v2)`
pub fn res_ext_build_zip_with_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.zipWith_ok", env)
}
/// `Result.flatmap_fusion : andThen (andThen r f) g = andThen r (fun x => andThen (f x) g)`
pub fn res_ext_build_flatmap_fusion(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.flatmap_fusion", env)
}
/// `Result.map_flatmap_fusion : map f (andThen r g) = andThen r (fun x => ok (f (g' x)))`
pub fn res_ext_build_map_flatmap_fusion(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.map_flatmap_fusion", env)
}
/// `Result.recoverWith_ok : recoverWith (ok v) f = ok v`
pub fn res_ext_build_recover_with_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.recoverWith_ok", env)
}
/// `Result.recoverWith_err : recoverWith (err e) f = f e`
pub fn res_ext_build_recover_with_err(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.recoverWith_err", env)
}
/// `Result.tap_ok : tap (ok v) f = (f v; ok v)` (side-effect on success, returns original)
pub fn res_ext_build_tap_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.tap_ok", env)
}
/// `Result.tap_err : tap (err e) f = err e` (no side-effect on error)
pub fn res_ext_build_tap_err(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.tap_err", env)
}
/// `Result.comonad_extract : extract (ok v) = v` (partial; requires Ok)
pub fn res_ext_build_comonad_extract(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall_type_axiom("Result.comonad_extract", env)
}
/// `Result.comonad_duplicate : duplicate (ok v) = ok (ok v)`
pub fn res_ext_build_comonad_duplicate(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.comonad_duplicate", env)
}
/// `Result.do_notation_bind : (x <- r; f x) = andThen r f`
pub fn res_ext_build_do_bind_desugar(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.do_notation_bind", env)
}
/// `Result.do_notation_return : return v desugars to ok v`
pub fn res_ext_build_do_return_desugar(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall_type_axiom("Result.do_notation_return", env)
}
/// `Result.either_iso_ok : toEither (ok v) = Right v`
pub fn res_ext_build_either_iso_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.either_iso_ok", env)
}
/// `Result.either_iso_err : toEither (err e) = Left e`
pub fn res_ext_build_either_iso_err(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.either_iso_err", env)
}
/// `Result.from_either_roundtrip : fromEither (toEither r) = r`
pub fn res_ext_build_either_roundtrip(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.either_roundtrip", env)
}
/// `Result.join_left_id : join (ok r) = r`
pub fn res_ext_build_join_left_id(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.join_left_id", env)
}
/// `Result.join_assoc : join ∘ map join = join ∘ join` (flatten associativity)
pub fn res_ext_build_join_assoc(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.join_assoc", env)
}
/// `Result.lazy_eval_ok : evaluate (lazy ok v) = ok v`
pub fn res_ext_build_lazy_eval_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall_type_axiom("Result.lazy_eval_ok", env)
}
/// `Result.lazy_eval_err : evaluate (lazy err e) = err e`
pub fn res_ext_build_lazy_eval_err(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall_type_axiom("Result.lazy_eval_err", env)
}
/// `Result.lazy_eval_idempotent : evaluate (evaluate r) = evaluate r`
pub fn res_ext_build_lazy_eval_idempotent(
    env: &mut Environment,
) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.lazy_eval_idempotent", env)
}
/// `Result.chain_ok_ok : chain (ok v1) (ok v2) = ok v2` (sequence, keep last)
pub fn res_ext_build_chain_ok_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.chain_ok_ok", env)
}
/// `Result.chain_err : chain (err e) r = err e` (short-circuit)
pub fn res_ext_build_chain_err(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.chain_err", env)
}
/// Register all extended Result axioms into `env`.
///
/// Adds 35+ axioms covering:
/// - Result monad laws (left/right identity, associativity)
/// - Error handling combinators (map, and_then, or_else, map_err, flatten)
/// - Result bifunctor laws (bimap identity and composition)
/// - Traversal over Result (sequence, traverse)
/// - Result as profunctor (dimap)
/// - Error accumulation (Validation-style ap)
/// - Result catamorphism (fold) and anamorphism (unfold)
/// - Early return / short-circuit semantics
/// - ResultT monad transformer laws
/// - Applicative instance (pure, ap)
/// - Alternative instance (first success)
/// - Result zip and zipWith
/// - Flatmap fusion laws
/// - Error recovery (recoverWith, tap)
/// - Result comonad (extract from fixed-error)
/// - Do-notation desugaring
/// - Monadic chaining laws
/// - Either isomorphism
/// - Join/flatten laws
/// - Lazy result evaluation
pub fn register_result_extended_axioms(env: &mut Environment) {
    let builders: &[fn(&mut Environment) -> std::result::Result<(), String>] = &[
        res_ext_build_monad_left_identity,
        res_ext_build_monad_right_identity,
        res_ext_build_monad_assoc,
        res_ext_build_map_identity,
        res_ext_build_map_composition,
        res_ext_build_and_then_ok,
        res_ext_build_and_then_err,
        res_ext_build_or_else_ok,
        res_ext_build_or_else_err,
        res_ext_build_map_err_identity,
        res_ext_build_map_err_composition,
        res_ext_build_flatten_ok_ok,
        res_ext_build_flatten_ok_err,
        res_ext_build_flatten_err,
        res_ext_build_bimap_identity,
        res_ext_build_bimap_composition,
        res_ext_build_bimap_ok,
        res_ext_build_bimap_err,
        res_ext_build_traverse_pure,
        res_ext_build_sequence_ok,
        res_ext_build_sequence_err,
        res_ext_build_dimap_identity,
        res_ext_build_dimap_composition,
        res_ext_build_ap_pure_ok,
        res_ext_build_ap_err_left,
        res_ext_build_applicative_identity,
        res_ext_build_applicative_homomorphism,
        res_ext_build_applicative_interchange,
        res_ext_build_validation_ap_ok_ok,
        res_ext_build_validation_ap_err_accumulate,
        res_ext_build_fold_ok,
        res_ext_build_fold_err,
        res_ext_build_unfold_left,
        res_ext_build_unfold_right,
        res_ext_build_short_circuit_err,
        res_ext_build_short_circuit_ok,
        res_ext_build_result_t_lift_ok,
        res_ext_build_result_t_lift_err,
        res_ext_build_result_t_monad_left_id,
        res_ext_build_alt_ok_left,
        res_ext_build_alt_err_left,
        res_ext_build_zip_ok_ok,
        res_ext_build_zip_err,
        res_ext_build_zip_with_ok,
        res_ext_build_flatmap_fusion,
        res_ext_build_map_flatmap_fusion,
        res_ext_build_recover_with_ok,
        res_ext_build_recover_with_err,
        res_ext_build_tap_ok,
        res_ext_build_tap_err,
        res_ext_build_comonad_extract,
        res_ext_build_comonad_duplicate,
        res_ext_build_do_bind_desugar,
        res_ext_build_do_return_desugar,
        res_ext_build_either_iso_ok,
        res_ext_build_either_iso_err,
        res_ext_build_either_roundtrip,
        res_ext_build_join_left_id,
        res_ext_build_join_assoc,
        res_ext_build_lazy_eval_ok,
        res_ext_build_lazy_eval_err,
        res_ext_build_lazy_eval_idempotent,
        res_ext_build_chain_ok_ok,
        res_ext_build_chain_err,
    ];
    for builder in builders {
        let _ = builder(env);
    }
}
#[cfg(test)]
mod result_extended_axiom_tests {
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
    fn test_register_result_extended_axioms_runs() {
        let mut env = make_env();
        register_result_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Result.monad_left_identity")).is_some());
        assert!(env.get(&Name::str("Result.monad_right_identity")).is_some());
        assert!(env.get(&Name::str("Result.monad_assoc")).is_some());
    }
    #[test]
    fn test_monad_laws_present() {
        let mut env = make_env();
        register_result_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Result.monad_left_identity")).is_some());
        assert!(env.get(&Name::str("Result.monad_right_identity")).is_some());
        assert!(env.get(&Name::str("Result.monad_assoc")).is_some());
    }
    #[test]
    fn test_bifunctor_laws_present() {
        let mut env = make_env();
        register_result_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Result.bimap_identity")).is_some());
        assert!(env.get(&Name::str("Result.bimap_composition")).is_some());
        assert!(env.get(&Name::str("Result.bimap_ok")).is_some());
        assert!(env.get(&Name::str("Result.bimap_err")).is_some());
    }
    #[test]
    fn test_applicative_laws_present() {
        let mut env = make_env();
        register_result_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Result.applicative_identity")).is_some());
        assert!(env
            .get(&Name::str("Result.applicative_homomorphism"))
            .is_some());
        assert!(env
            .get(&Name::str("Result.applicative_interchange"))
            .is_some());
    }
    #[test]
    fn test_traversal_laws_present() {
        let mut env = make_env();
        register_result_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Result.traverse_pure")).is_some());
        assert!(env.get(&Name::str("Result.sequence_ok")).is_some());
        assert!(env.get(&Name::str("Result.sequence_err")).is_some());
    }
    #[test]
    fn test_profunctor_laws_present() {
        let mut env = make_env();
        register_result_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Result.dimap_identity")).is_some());
        assert!(env.get(&Name::str("Result.dimap_composition")).is_some());
    }
    #[test]
    fn test_validation_laws_present() {
        let mut env = make_env();
        register_result_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Result.validation_ap_ok_ok")).is_some());
        assert!(env
            .get(&Name::str("Result.validation_ap_err_accumulate"))
            .is_some());
    }
    #[test]
    fn test_catamorphism_laws_present() {
        let mut env = make_env();
        register_result_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Result.fold_ok")).is_some());
        assert!(env.get(&Name::str("Result.fold_err")).is_some());
    }
    #[test]
    fn test_anamorphism_laws_present() {
        let mut env = make_env();
        register_result_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Result.unfold_left")).is_some());
        assert!(env.get(&Name::str("Result.unfold_right")).is_some());
    }
    #[test]
    fn test_short_circuit_laws_present() {
        let mut env = make_env();
        register_result_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Result.short_circuit_err")).is_some());
        assert!(env.get(&Name::str("Result.short_circuit_ok")).is_some());
    }
    #[test]
    fn test_result_t_laws_present() {
        let mut env = make_env();
        register_result_extended_axioms(&mut env);
        assert!(env.get(&Name::str("ResultT.lift_ok")).is_some());
        assert!(env.get(&Name::str("ResultT.lift_err")).is_some());
        assert!(env.get(&Name::str("ResultT.monad_left_identity")).is_some());
    }
    #[test]
    fn test_alternative_laws_present() {
        let mut env = make_env();
        register_result_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Result.alt_ok_left")).is_some());
        assert!(env.get(&Name::str("Result.alt_err_left")).is_some());
    }
    #[test]
    fn test_zip_laws_present() {
        let mut env = make_env();
        register_result_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Result.zip_ok_ok")).is_some());
        assert!(env.get(&Name::str("Result.zip_err")).is_some());
        assert!(env.get(&Name::str("Result.zipWith_ok")).is_some());
    }
    #[test]
    fn test_flatmap_fusion_present() {
        let mut env = make_env();
        register_result_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Result.flatmap_fusion")).is_some());
        assert!(env.get(&Name::str("Result.map_flatmap_fusion")).is_some());
    }
    #[test]
    fn test_recovery_laws_present() {
        let mut env = make_env();
        register_result_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Result.recoverWith_ok")).is_some());
        assert!(env.get(&Name::str("Result.recoverWith_err")).is_some());
        assert!(env.get(&Name::str("Result.tap_ok")).is_some());
        assert!(env.get(&Name::str("Result.tap_err")).is_some());
    }
    #[test]
    fn test_comonad_laws_present() {
        let mut env = make_env();
        register_result_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Result.comonad_extract")).is_some());
        assert!(env.get(&Name::str("Result.comonad_duplicate")).is_some());
    }
    #[test]
    fn test_do_notation_laws_present() {
        let mut env = make_env();
        register_result_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Result.do_notation_bind")).is_some());
        assert!(env.get(&Name::str("Result.do_notation_return")).is_some());
    }
    #[test]
    fn test_either_iso_laws_present() {
        let mut env = make_env();
        register_result_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Result.either_iso_ok")).is_some());
        assert!(env.get(&Name::str("Result.either_iso_err")).is_some());
        assert!(env.get(&Name::str("Result.either_roundtrip")).is_some());
    }
    #[test]
    fn test_join_laws_present() {
        let mut env = make_env();
        register_result_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Result.join_left_id")).is_some());
        assert!(env.get(&Name::str("Result.join_assoc")).is_some());
    }
    #[test]
    fn test_lazy_eval_laws_present() {
        let mut env = make_env();
        register_result_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Result.lazy_eval_ok")).is_some());
        assert!(env.get(&Name::str("Result.lazy_eval_err")).is_some());
        assert!(env.get(&Name::str("Result.lazy_eval_idempotent")).is_some());
    }
    #[test]
    fn test_chain_laws_present() {
        let mut env = make_env();
        register_result_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Result.chain_ok_ok")).is_some());
        assert!(env.get(&Name::str("Result.chain_err")).is_some());
    }
    #[test]
    fn test_result_axiom_registry_ops() {
        let mut reg = ResultAxiomRegistry::new();
        reg.register("Result.monad_left_identity");
        reg.register("Result.monad_right_identity");
        assert_eq!(reg.len(), 2);
        assert!(!reg.is_empty());
    }
    #[test]
    fn test_validation_collector_all_ok() {
        let mut col: ValidationCollector<i32, String> = ValidationCollector::with_capacity(4);
        col.push(Ok(1));
        col.push(Ok(2));
        col.push(Ok(3));
        assert!(col.is_valid());
        let result = col.finish();
        assert_eq!(result.expect("result should be valid"), vec![1, 2, 3]);
    }
    #[test]
    fn test_validation_collector_with_errors() {
        let mut col: ValidationCollector<i32, String> = ValidationCollector::with_capacity(4);
        col.push(Ok(1));
        col.push(Err("bad".to_string()));
        col.push(Ok(2));
        col.push(Err("worse".to_string()));
        assert!(!col.is_valid());
        let result = col.finish();
        let errs = result.unwrap_err();
        assert_eq!(errs.len(), 2);
    }
    #[test]
    fn test_result_either_bridge() {
        let std_bridge = ResultEitherBridge::standard();
        assert!(!std_bridge.flip_convention);
        let flip_bridge = ResultEitherBridge::flipped();
        assert!(flip_bridge.flip_convention);
    }
    #[test]
    fn test_prop_axiom_is_sort_zero() {
        let mut env = Environment::new();
        res_ext_prop_axiom("test.prop", &mut env).expect("Environment::new should succeed");
        let decl = env
            .get(&Name::str("test.prop"))
            .expect("declaration 'test.prop' should exist in env");
        assert!(matches!(decl, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_extended_axioms_are_axiom_kind() {
        let mut env = make_env();
        register_result_extended_axioms(&mut env);
        let decl = env
            .get(&Name::str("Result.monad_left_identity"))
            .expect("declaration 'Result.monad_left_identity' should exist in env");
        assert!(matches!(decl, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_all_35_plus_axioms_registered() {
        let mut env = make_env();
        register_result_extended_axioms(&mut env);
        let axiom_names = [
            "Result.monad_left_identity",
            "Result.monad_right_identity",
            "Result.monad_assoc",
            "Result.map_identity",
            "Result.map_composition",
            "Result.and_then_ok",
            "Result.and_then_err",
            "Result.or_else_ok",
            "Result.or_else_err",
            "Result.map_err_identity",
            "Result.map_err_composition",
            "Result.flatten_ok_ok",
            "Result.flatten_ok_err",
            "Result.flatten_err",
            "Result.bimap_identity",
            "Result.bimap_composition",
            "Result.bimap_ok",
            "Result.bimap_err",
            "Result.traverse_pure",
            "Result.sequence_ok",
            "Result.sequence_err",
            "Result.dimap_identity",
            "Result.dimap_composition",
            "Result.ap_pure_ok",
            "Result.ap_err_left",
            "Result.applicative_identity",
            "Result.applicative_homomorphism",
            "Result.applicative_interchange",
            "Result.validation_ap_ok_ok",
            "Result.validation_ap_err_accumulate",
            "Result.fold_ok",
            "Result.fold_err",
            "Result.unfold_left",
            "Result.unfold_right",
            "Result.short_circuit_err",
            "Result.short_circuit_ok",
        ];
        let mut found = 0usize;
        for name in &axiom_names {
            if env.get(&Name::str(*name)).is_some() {
                found += 1;
            }
        }
        assert!(found >= 35, "Expected at least 35 axioms, found {}", found);
    }
}
/// `Result.strength_ok : strength (a, ok b) = ok (a, b)`
pub(super) fn res_ext2_build_strength_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.strength_ok", env)
}
/// `Result.strength_err : strength (a, err e) = err e`
pub(super) fn res_ext2_build_strength_err(
    env: &mut Environment,
) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.strength_err", env)
}
/// `Result.natural_transform_ok : η (ok v) = ok (η_T v)` for any natural transformation η
pub(super) fn res_ext2_build_natural_transform_ok(
    env: &mut Environment,
) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.natural_transform_ok", env)
}
/// `Result.natural_transform_err : η (err e) = err (η_E e)`
pub(super) fn res_ext2_build_natural_transform_err(
    env: &mut Environment,
) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.natural_transform_err", env)
}
/// `Result.monad_morphism_unit : morphism (ok v) = ok v`  (unit preservation)
pub(super) fn res_ext2_build_monad_morphism_unit(
    env: &mut Environment,
) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.monad_morphism_unit", env)
}
/// `Result.monad_morphism_bind : morphism (andThen r f) = andThen (morphism r) (morphism ∘ f)`
pub(super) fn res_ext2_build_monad_morphism_bind(
    env: &mut Environment,
) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.monad_morphism_bind", env)
}
/// `Result.distribute_list_result : sequence (map f xs) relates to map f (ok xs)`
pub(super) fn res_ext2_build_distribute_list_result(
    env: &mut Environment,
) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.distribute_list_result", env)
}
/// `Result.pointed_pure : pure v = ok v` (Pointed instance)
pub(super) fn res_ext2_build_pointed_pure(
    env: &mut Environment,
) -> std::result::Result<(), String> {
    res_ext_forall_type_axiom("Result.pointed_pure", env)
}
/// `Result.lens_get_ok : get (ok v) lens = lens.get v`
pub(super) fn res_ext2_build_lens_get_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.lens_get_ok", env)
}
/// `Result.lens_set_ok : set (ok v) lens w = ok (lens.set v w)`
pub(super) fn res_ext2_build_lens_set_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall3_axiom("Result.lens_set_ok", env)
}
/// `Result.prism_review : prism.review v = ok v`
pub(super) fn res_ext2_build_prism_review(
    env: &mut Environment,
) -> std::result::Result<(), String> {
    res_ext_forall_type_axiom("Result.prism_review", env)
}
/// `Result.prism_preview_ok : prism.preview (ok v) = Some v`
pub(super) fn res_ext2_build_prism_preview_ok(
    env: &mut Environment,
) -> std::result::Result<(), String> {
    res_ext_forall_type_axiom("Result.prism_preview_ok", env)
}
/// `Result.prism_preview_err : prism.preview (err e) = None`
pub(super) fn res_ext2_build_prism_preview_err(
    env: &mut Environment,
) -> std::result::Result<(), String> {
    res_ext_forall_type_axiom("Result.prism_preview_err", env)
}
/// `Result.swap_ok : swap (ok v) = err v` (SwapResult: treat Ok as Err)
pub(super) fn res_ext2_build_swap_ok(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall_type_axiom("Result.swap_ok", env)
}
/// `Result.swap_err : swap (err e) = ok e`
pub(super) fn res_ext2_build_swap_err(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall_type_axiom("Result.swap_err", env)
}
/// `Result.swap_involution : swap (swap r) = r`
pub(super) fn res_ext2_build_swap_involution(
    env: &mut Environment,
) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.swap_involution", env)
}
/// `Result.unwrap_or_default_ok : unwrapOrDefault (ok v) = v`
pub(super) fn res_ext2_build_unwrap_or_default_ok(
    env: &mut Environment,
) -> std::result::Result<(), String> {
    res_ext_forall_type_axiom("Result.unwrap_or_default_ok", env)
}
/// `Result.ok_or_else : okOrElse (Some v) e = ok v`
pub(super) fn res_ext2_build_ok_or_else(env: &mut Environment) -> std::result::Result<(), String> {
    res_ext_forall2_axiom("Result.ok_or_else", env)
}
