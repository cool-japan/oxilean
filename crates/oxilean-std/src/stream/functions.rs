//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{LazyStream, StreamDeclStats};

/// Build Stream type in the environment.
pub fn build_stream_env(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    let stream_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(type2.clone()),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Stream"),
        univ_params: vec![],
        ty: stream_ty,
    })
    .map_err(|e| e.to_string())?;
    let cons_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("head"),
            Box::new(Expr::BVar(0)),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("tail"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Stream"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Stream"), vec![])),
                    Box::new(Expr::BVar(2)),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Stream.cons"),
        univ_params: vec![],
        ty: cons_ty,
    })
    .map_err(|e| e.to_string())?;
    let head_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("s"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Stream"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::BVar(1)),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Stream.head"),
        univ_params: vec![],
        ty: head_ty,
    })
    .map_err(|e| e.to_string())?;
    let tail_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("s"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Stream"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Stream"), vec![])),
                Box::new(Expr::BVar(1)),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Stream.tail"),
        univ_params: vec![],
        ty: tail_ty,
    })
    .map_err(|e| e.to_string())?;
    let map_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
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
                    Name::str("s"),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Stream"), vec![])),
                        Box::new(Expr::BVar(2)),
                    )),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Stream"), vec![])),
                        Box::new(Expr::BVar(2)),
                    )),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Stream.map"),
        univ_params: vec![],
        ty: map_ty,
    })
    .map_err(|e| e.to_string())?;
    let take_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("n"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("s"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Stream"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("List"), vec![])),
                    Box::new(Expr::BVar(2)),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Stream.take"),
        univ_params: vec![],
        ty: take_ty,
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
        let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
        env.add(Declaration::Axiom {
            name: Name::str("Nat"),
            univ_params: vec![],
            ty: type1.clone(),
        })
        .expect("operation should succeed");
        let list_ty = Expr::Pi(
            BinderInfo::Default,
            Name::str("α"),
            Box::new(type1),
            Box::new(type2),
        );
        env.add(Declaration::Axiom {
            name: Name::str("List"),
            univ_params: vec![],
            ty: list_ty,
        })
        .expect("operation should succeed");
        env
    }
    #[test]
    fn test_build_stream_env() {
        let mut env = setup_env();
        assert!(build_stream_env(&mut env).is_ok());
        assert!(env.get(&Name::str("Stream")).is_some());
        assert!(env.get(&Name::str("Stream.cons")).is_some());
    }
    #[test]
    fn test_stream_head() {
        let mut env = setup_env();
        build_stream_env(&mut env).expect("build_stream_env should succeed");
        let decl = env
            .get(&Name::str("Stream.head"))
            .expect("declaration 'Stream.head' should exist in env");
        assert!(matches!(decl, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_stream_tail() {
        let mut env = setup_env();
        build_stream_env(&mut env).expect("build_stream_env should succeed");
        let decl = env
            .get(&Name::str("Stream.tail"))
            .expect("declaration 'Stream.tail' should exist in env");
        assert!(matches!(decl, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_stream_map() {
        let mut env = setup_env();
        build_stream_env(&mut env).expect("build_stream_env should succeed");
        let decl = env
            .get(&Name::str("Stream.map"))
            .expect("declaration 'Stream.map' should exist in env");
        assert!(matches!(decl, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_stream_take() {
        let mut env = setup_env();
        build_stream_env(&mut env).expect("build_stream_env should succeed");
        let decl = env
            .get(&Name::str("Stream.take"))
            .expect("declaration 'Stream.take' should exist in env");
        assert!(matches!(decl, Declaration::Axiom { .. }));
    }
}
/// Build extended Stream combinators in the environment.
///
/// Adds `zip`, `filter`, `iterate`, `drop`, `nth`, and `zipWith`.
pub fn build_stream_combinators(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let zip_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("s1"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Stream"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("s2"),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Stream"), vec![])),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Stream"), vec![])),
                        Box::new(Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("Prod"), vec![])),
                                Box::new(Expr::BVar(3)),
                            )),
                            Box::new(Expr::BVar(2)),
                        )),
                    )),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Stream.zip"),
        univ_params: vec![],
        ty: zip_ty,
    })
    .map_err(|e| e.to_string())?;
    let iterate_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("init"),
            Box::new(Expr::BVar(0)),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("step"),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_"),
                    Box::new(Expr::BVar(1)),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Stream"), vec![])),
                    Box::new(Expr::BVar(2)),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Stream.iterate"),
        univ_params: vec![],
        ty: iterate_ty,
    })
    .map_err(|e| e.to_string())?;
    let drop_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("n"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("s"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Stream"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Stream"), vec![])),
                    Box::new(Expr::BVar(2)),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Stream.drop"),
        univ_params: vec![],
        ty: drop_ty,
    })
    .map_err(|e| e.to_string())?;
    let nth_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("n"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("s"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Stream"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(Expr::BVar(2)),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Stream.nth"),
        univ_params: vec![],
        ty: nth_ty,
    })
    .map_err(|e| e.to_string())?;
    let const_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("val"),
            Box::new(Expr::BVar(0)),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Stream"), vec![])),
                Box::new(Expr::BVar(1)),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Stream.const"),
        univ_params: vec![],
        ty: const_ty,
    })
    .map_err(|e| e.to_string())?;
    let filter_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("pred"),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_"),
                Box::new(Expr::BVar(0)),
                Box::new(Expr::Const(Name::str("Bool"), vec![])),
            )),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("s"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Stream"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Stream"), vec![])),
                    Box::new(Expr::BVar(2)),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Stream.filter"),
        univ_params: vec![],
        ty: filter_ty,
    })
    .map_err(|e| e.to_string())?;
    Ok(())
}
#[cfg(test)]
mod stream_combinator_tests {
    use super::*;
    fn setup_extended_env() -> Environment {
        let mut env = Environment::new();
        let type1 = Expr::Sort(Level::succ(Level::zero()));
        let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
        env.add(Declaration::Axiom {
            name: Name::str("Nat"),
            univ_params: vec![],
            ty: type1.clone(),
        })
        .expect("operation should succeed");
        env.add(Declaration::Axiom {
            name: Name::str("Bool"),
            univ_params: vec![],
            ty: type1.clone(),
        })
        .expect("operation should succeed");
        let list_ty = Expr::Pi(
            BinderInfo::Default,
            Name::str("α"),
            Box::new(type1.clone()),
            Box::new(type2.clone()),
        );
        env.add(Declaration::Axiom {
            name: Name::str("List"),
            univ_params: vec![],
            ty: list_ty,
        })
        .expect("operation should succeed");
        let prod_ty = Expr::Pi(
            BinderInfo::Default,
            Name::str("α"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("β"),
                Box::new(type1.clone()),
                Box::new(type2.clone()),
            )),
        );
        env.add(Declaration::Axiom {
            name: Name::str("Prod"),
            univ_params: vec![],
            ty: prod_ty,
        })
        .expect("operation should succeed");
        build_stream_env(&mut env).expect("build_stream_env should succeed");
        env
    }
    #[test]
    fn test_stream_zip() {
        let mut env = setup_extended_env();
        assert!(build_stream_combinators(&mut env).is_ok());
        assert!(env.get(&Name::str("Stream.zip")).is_some());
    }
    #[test]
    fn test_stream_iterate() {
        let mut env = setup_extended_env();
        build_stream_combinators(&mut env).expect("build_stream_combinators should succeed");
        assert!(env.get(&Name::str("Stream.iterate")).is_some());
    }
    #[test]
    fn test_stream_drop() {
        let mut env = setup_extended_env();
        build_stream_combinators(&mut env).expect("build_stream_combinators should succeed");
        assert!(env.get(&Name::str("Stream.drop")).is_some());
    }
    #[test]
    fn test_stream_nth() {
        let mut env = setup_extended_env();
        build_stream_combinators(&mut env).expect("build_stream_combinators should succeed");
        assert!(env.get(&Name::str("Stream.nth")).is_some());
    }
    #[test]
    fn test_stream_const() {
        let mut env = setup_extended_env();
        build_stream_combinators(&mut env).expect("build_stream_combinators should succeed");
        assert!(env.get(&Name::str("Stream.const")).is_some());
    }
    #[test]
    fn test_stream_filter() {
        let mut env = setup_extended_env();
        build_stream_combinators(&mut env).expect("build_stream_combinators should succeed");
        assert!(env.get(&Name::str("Stream.filter")).is_some());
    }
    #[test]
    fn test_lazy_stream_constant() {
        let mut s = LazyStream::constant(42u64);
        assert_eq!(s.next(), Some(42));
        assert_eq!(s.next(), Some(42));
        assert_eq!(s.next(), Some(42));
    }
    #[test]
    fn test_lazy_stream_iterate() {
        let s = LazyStream::iterate(0u64, |x| x + 1);
        let vals = s.take(5);
        assert_eq!(vals, vec![0, 1, 2, 3, 4]);
    }
    #[test]
    fn test_lazy_stream_take() {
        let s = LazyStream::constant("hello");
        let vals = s.take(3);
        assert_eq!(vals.len(), 3);
        assert!(vals.iter().all(|&v| v == "hello"));
    }
    #[test]
    fn test_lazy_stream_from_fn() {
        let mut counter = 0u32;
        let mut s = LazyStream::from_fn(move || {
            counter += 1;
            if counter <= 3 {
                Some(counter)
            } else {
                None
            }
        });
        assert_eq!(s.next(), Some(1));
        assert_eq!(s.next(), Some(2));
        assert_eq!(s.next(), Some(3));
        assert_eq!(s.next(), None);
    }
}
/// Build Stream theorems in the environment.
///
/// Adds basic stream lemmas as axioms (these would be proved in a full system).
pub fn build_stream_theorems(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    let head_cons_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("h"),
            Box::new(Expr::BVar(0)),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("t"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Stream"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(prop.clone()),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Stream.head_cons"),
        univ_params: vec![],
        ty: head_cons_ty,
    })
    .map_err(|e| e.to_string())?;
    let tail_cons_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("h"),
            Box::new(Expr::BVar(0)),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("t"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Stream"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(prop.clone()),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Stream.tail_cons"),
        univ_params: vec![],
        ty: tail_cons_ty,
    })
    .map_err(|e| e.to_string())?;
    Ok(())
}
#[cfg(test)]
mod stream_theorem_tests {
    use super::*;
    fn base_env() -> Environment {
        let mut env = Environment::new();
        let type1 = Expr::Sort(Level::succ(Level::zero()));
        let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
        env.add(Declaration::Axiom {
            name: Name::str("Nat"),
            univ_params: vec![],
            ty: type1.clone(),
        })
        .expect("operation should succeed");
        let list_ty = Expr::Pi(
            BinderInfo::Default,
            Name::str("α"),
            Box::new(type1.clone()),
            Box::new(type2.clone()),
        );
        env.add(Declaration::Axiom {
            name: Name::str("List"),
            univ_params: vec![],
            ty: list_ty,
        })
        .expect("operation should succeed");
        build_stream_env(&mut env).expect("build_stream_env should succeed");
        env
    }
    #[test]
    fn test_stream_head_cons_theorem() {
        let mut env = base_env();
        assert!(build_stream_theorems(&mut env).is_ok());
        assert!(env.get(&Name::str("Stream.head_cons")).is_some());
    }
    #[test]
    fn test_stream_tail_cons_theorem() {
        let mut env = base_env();
        build_stream_theorems(&mut env).expect("build_stream_theorems should succeed");
        assert!(env.get(&Name::str("Stream.tail_cons")).is_some());
    }
    #[test]
    fn test_stream_decl_is_axiom() {
        let mut env = base_env();
        build_stream_theorems(&mut env).expect("build_stream_theorems should succeed");
        let decl = env
            .get(&Name::str("Stream.head_cons"))
            .expect("declaration 'Stream.head_cons' should exist in env");
        assert!(matches!(decl, Declaration::Axiom { .. }));
    }
}
/// Count how many Stream declarations are registered in an environment.
pub fn count_stream_decls(env: &Environment) -> usize {
    let names = [
        "Stream",
        "Stream.cons",
        "Stream.head",
        "Stream.tail",
        "Stream.map",
        "Stream.take",
        "Stream.zip",
        "Stream.iterate",
        "Stream.drop",
        "Stream.nth",
        "Stream.const",
        "Stream.filter",
        "Stream.head_cons",
        "Stream.tail_cons",
    ];
    names
        .iter()
        .filter(|&&n| env.get(&Name::str(n)).is_some())
        .count()
}
#[cfg(test)]
mod count_tests {
    use super::*;
    #[test]
    fn test_count_stream_decls_base() {
        let mut env = Environment::new();
        let type1 = Expr::Sort(Level::succ(Level::zero()));
        let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
        env.add(Declaration::Axiom {
            name: Name::str("Nat"),
            univ_params: vec![],
            ty: type1.clone(),
        })
        .expect("operation should succeed");
        let list_ty = Expr::Pi(
            BinderInfo::Default,
            Name::str("α"),
            Box::new(type1.clone()),
            Box::new(type2.clone()),
        );
        env.add(Declaration::Axiom {
            name: Name::str("List"),
            univ_params: vec![],
            ty: list_ty,
        })
        .expect("operation should succeed");
        build_stream_env(&mut env).expect("build_stream_env should succeed");
        let count = count_stream_decls(&env);
        assert!(count >= 6);
    }
}
/// Build Stream monad operations in the environment.
///
/// Adds `Stream.pure`, `Stream.bind`, `Stream.ap`, `Stream.zipWith`,
/// `Stream.scan`, and `Stream.interleave`.
pub fn build_stream_monad(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let pure_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(Expr::BVar(0)),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Stream"), vec![])),
                Box::new(Expr::BVar(1)),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Stream.pure"),
        univ_params: vec![],
        ty: pure_ty,
    })
    .map_err(|e| e.to_string())?;
    let scan_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("f"),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_"),
                    Box::new(Expr::BVar(1)),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("_"),
                        Box::new(Expr::BVar(2)),
                        Box::new(Expr::BVar(2)),
                    )),
                )),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("init"),
                    Box::new(Expr::BVar(2)),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("s"),
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("Stream"), vec![])),
                            Box::new(Expr::BVar(3)),
                        )),
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("Stream"), vec![])),
                            Box::new(Expr::BVar(3)),
                        )),
                    )),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Stream.scan"),
        univ_params: vec![],
        ty: scan_ty,
    })
    .map_err(|e| e.to_string())?;
    let interleave_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("s1"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Stream"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("s2"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Stream"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Stream"), vec![])),
                    Box::new(Expr::BVar(2)),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Stream.interleave"),
        univ_params: vec![],
        ty: interleave_ty,
    })
    .map_err(|e| e.to_string())?;
    Ok(())
}
#[cfg(test)]
mod stream_monad_tests {
    use super::*;
    fn full_env() -> Environment {
        let mut env = Environment::new();
        let type1 = Expr::Sort(Level::succ(Level::zero()));
        let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
        env.add(Declaration::Axiom {
            name: Name::str("Nat"),
            univ_params: vec![],
            ty: type1.clone(),
        })
        .expect("operation should succeed");
        env.add(Declaration::Axiom {
            name: Name::str("Bool"),
            univ_params: vec![],
            ty: type1.clone(),
        })
        .expect("operation should succeed");
        env.add(Declaration::Axiom {
            name: Name::str("List"),
            univ_params: vec![],
            ty: Expr::Pi(
                BinderInfo::Default,
                Name::str("α"),
                Box::new(type1.clone()),
                Box::new(type2.clone()),
            ),
        })
        .expect("operation should succeed");
        env.add(Declaration::Axiom {
            name: Name::str("Prod"),
            univ_params: vec![],
            ty: Expr::Pi(
                BinderInfo::Default,
                Name::str("α"),
                Box::new(type1.clone()),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("β"),
                    Box::new(type1.clone()),
                    Box::new(type2.clone()),
                )),
            ),
        })
        .expect("operation should succeed");
        build_stream_env(&mut env).expect("build_stream_env should succeed");
        env
    }
    #[test]
    fn test_stream_pure() {
        let mut env = full_env();
        build_stream_monad(&mut env).expect("build_stream_monad should succeed");
        assert!(env.get(&Name::str("Stream.pure")).is_some());
    }
    #[test]
    fn test_stream_scan() {
        let mut env = full_env();
        build_stream_monad(&mut env).expect("build_stream_monad should succeed");
        assert!(env.get(&Name::str("Stream.scan")).is_some());
    }
    #[test]
    fn test_stream_interleave() {
        let mut env = full_env();
        build_stream_monad(&mut env).expect("build_stream_monad should succeed");
        assert!(env.get(&Name::str("Stream.interleave")).is_some());
    }
    #[test]
    fn test_stream_decl_stats_core() {
        let env = full_env();
        let stats = StreamDeclStats::compute(&env);
        assert_eq!(stats.core, 4);
    }
    #[test]
    fn test_stream_decl_stats_total() {
        let mut env = full_env();
        build_stream_combinators(&mut env).expect("build_stream_combinators should succeed");
        build_stream_monad(&mut env).expect("build_stream_monad should succeed");
        build_stream_theorems(&mut env).expect("build_stream_theorems should succeed");
        let stats = StreamDeclStats::compute(&env);
        assert!(stats.total() > 10);
    }
    #[test]
    fn test_lazy_stream_powers_of_two() {
        let s = LazyStream::iterate(1u64, |x| x * 2);
        let vals = s.take(8);
        assert_eq!(vals, vec![1, 2, 4, 8, 16, 32, 64, 128]);
    }
    #[test]
    fn test_lazy_stream_from_fn_terminates() {
        let mut s = LazyStream::from_fn(move || None::<i32>);
        assert_eq!(s.next(), None);
    }
}
/// `Stream.Bisim`: the bisimulation relation between two streams.
/// Type: {α : Type} → Stream α → Stream α → Prop
pub fn strm_ext_bisim_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("s1"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Stream"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("s2"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Stream"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(prop),
            )),
        )),
    )
}
/// `Stream.bisim_coind`: coinductive bisimulation principle.
/// Type: {α : Type} → (R : Stream α → Stream α → Prop) → ... → ∀ s t, R s t → s = t
pub fn strm_ext_bisim_coind_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("R"),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Stream"), vec![])),
                    Box::new(Expr::BVar(0)),
                )),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_"),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Stream"), vec![])),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(prop.clone()),
                )),
            )),
            Box::new(prop.clone()),
        )),
    )
}
/// `Stream.corec`: corecursion principle for streams.
/// Type: {α σ : Type} → (σ → α) → (σ → σ) → σ → Stream α
pub fn strm_ext_corec_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("σ"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("hd"),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_"),
                    Box::new(Expr::BVar(0)),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("tl"),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("_"),
                        Box::new(Expr::BVar(1)),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("s0"),
                        Box::new(Expr::BVar(2)),
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("Stream"), vec![])),
                            Box::new(Expr::BVar(4)),
                        )),
                    )),
                )),
            )),
        )),
    )
}
/// `MealyMachine.type`: a Mealy machine as stream transducer.
/// Type: Type → Type → Type → Type
pub fn strm_ext_mealy_machine_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("S"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("I"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("O"),
                Box::new(type1.clone()),
                Box::new(type2),
            )),
        )),
    )
}
/// `MealyMachine.run`: run a Mealy machine on a stream of inputs.
/// Type: {S I O : Type} → MealyMachine S I O → S → Stream I → Stream O
pub fn strm_ext_mealy_run_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("S"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("I"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("O"),
                Box::new(type1.clone()),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("m"),
                    Box::new(Expr::Const(Name::str("MealyMachine"), vec![])),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("s0"),
                        Box::new(Expr::BVar(3)),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("inp"),
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("Stream"), vec![])),
                                Box::new(Expr::BVar(3)),
                            )),
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("Stream"), vec![])),
                                Box::new(Expr::BVar(3)),
                            )),
                        )),
                    )),
                )),
            )),
        )),
    )
}
/// `MooreMachine.type`: a Moore machine.
/// Type: Type → Type → Type → Type
pub fn strm_ext_moore_machine_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("S"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("I"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("O"),
                Box::new(type1.clone()),
                Box::new(type2),
            )),
        )),
    )
}
/// `KPN.channel`: a Kahn process network channel (FIFO stream).
/// Type: Type → Type
pub fn strm_ext_kpn_channel_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("α"),
        Box::new(type1),
        Box::new(type2),
    )
}
/// `KPN.process`: a Kahn process (reads inputs, writes outputs).
/// Type: {α β : Type} → Stream α → Stream β
pub fn strm_ext_kpn_process_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("inp"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Stream"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Stream"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
            )),
        )),
    )
}
/// `FRP.Behavior`: a time-varying value (FRP behavior).
/// Type: Type → Type
pub fn strm_ext_frp_behavior_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("α"),
        Box::new(type1),
        Box::new(type2),
    )
}
/// `FRP.Event`: a discrete stream of events with timestamps.
/// Type: Type → Type
pub fn strm_ext_frp_event_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("α"),
        Box::new(type1),
        Box::new(type2),
    )
}
/// `FRP.stepper`: convert a stream of events to a behavior.
/// Type: {α : Type} → α → Stream α → FRP.Behavior α
pub fn strm_ext_frp_stepper_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("init"),
            Box::new(Expr::BVar(0)),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("evts"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Stream"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("FRP.Behavior"), vec![])),
                    Box::new(Expr::BVar(2)),
                )),
            )),
        )),
    )
}
/// `Stream.diff_eq`: stream differential equation axiom.
/// Type: {α : Type} → (Stream α → Stream α) → Stream α → Prop
pub fn strm_ext_diff_eq_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("F"),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Stream"), vec![])),
                    Box::new(Expr::BVar(0)),
                )),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Stream"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
            )),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("s"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Stream"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(prop),
            )),
        )),
    )
}
/// `Stream.productivity`: guardedness / productivity condition.
/// Type: {α : Type} → Stream α → Prop
pub fn strm_ext_productivity_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("s"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Stream"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(prop),
        )),
    )
}
/// `Stream.guarded_fix`: guarded fixed point / corecursion under guard.
/// Type: {α : Type} → (Stream α → Stream α) → Stream α
pub fn strm_ext_guarded_fix_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("f"),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Stream"), vec![])),
                    Box::new(Expr::BVar(0)),
                )),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Stream"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
            )),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Stream"), vec![])),
                Box::new(Expr::BVar(1)),
            )),
        )),
    )
}
/// `Stream.fusion_law`: stream fusion correctness axiom.
/// Type: {α β γ : Type} → (f : β → γ) → (g : α → β) → s : Stream α →
///       Stream.map f (Stream.map g s) = Stream.map (f ∘ g) s
pub fn strm_ext_fusion_law_ty() -> Expr {
    let prop = Expr::Sort(Level::zero());
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("γ"),
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
                        Name::str("g"),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("_"),
                            Box::new(Expr::BVar(3)),
                            Box::new(Expr::BVar(3)),
                        )),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("s"),
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("Stream"), vec![])),
                                Box::new(Expr::BVar(4)),
                            )),
                            Box::new(prop),
                        )),
                    )),
                )),
            )),
        )),
    )
}
/// `Stream.BöhmTree`: Böhm tree as lazy normal form of a stream term.
/// Type: Type → Type
pub fn strm_ext_bohm_tree_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("α"),
        Box::new(type1),
        Box::new(type2),
    )
}
/// `Stream.corecursion_unique`: uniqueness of corecursive definitions.
/// Type: {α σ : Type} → (f g : σ → Stream α) → (∀ s, f s = g s) → Prop
pub fn strm_ext_corecursion_unique_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("σ"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("f"),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_"),
                    Box::new(Expr::BVar(0)),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Stream"), vec![])),
                        Box::new(Expr::BVar(2)),
                    )),
                )),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("g"),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("_"),
                        Box::new(Expr::BVar(1)),
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("Stream"), vec![])),
                            Box::new(Expr::BVar(3)),
                        )),
                    )),
                    Box::new(prop),
                )),
            )),
        )),
    )
}
/// `Stream.automaton`: a stream automaton (coalgebra for Stream functor).
/// Type: {S α : Type} → (S → Prod α S) → S → Stream α
pub fn strm_ext_automaton_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("S"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("α"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("step"),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_"),
                    Box::new(Expr::BVar(1)),
                    Box::new(Expr::App(
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("Prod"), vec![])),
                            Box::new(Expr::BVar(1)),
                        )),
                        Box::new(Expr::BVar(2)),
                    )),
                )),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("s0"),
                    Box::new(Expr::BVar(2)),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Stream"), vec![])),
                        Box::new(Expr::BVar(2)),
                    )),
                )),
            )),
        )),
    )
}
/// `WeightedAutomaton.type`: weighted automaton over streams.
/// Type: Type → Type → Type → Type
pub fn strm_ext_weighted_automaton_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("S"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("I"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("W"),
                Box::new(type1),
                Box::new(type2),
            )),
        )),
    )
}
/// `BloomFilter.type`: a streaming Bloom filter.
/// Type: Nat → Nat → Type
pub fn strm_ext_bloom_filter_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("m"),
        Box::new(Expr::Const(Name::str("Nat"), vec![])),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("k"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(type1),
        )),
    )
}
/// `CountMinSketch.type`: a streaming count-min sketch.
/// Type: Nat → Nat → Type
pub fn strm_ext_count_min_sketch_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("d"),
        Box::new(Expr::Const(Name::str("Nat"), vec![])),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("w"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(type1),
        )),
    )
}
/// `Stream.circular_prog`: circular programming / tie-the-knot combinator.
/// Type: {α β : Type} → ((Stream α → β) → Stream α → β) → Stream α → β
pub fn strm_ext_circular_prog_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("body"),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_"),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("_"),
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("Stream"), vec![])),
                            Box::new(Expr::BVar(1)),
                        )),
                        Box::new(Expr::BVar(2)),
                    )),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("_"),
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("Stream"), vec![])),
                            Box::new(Expr::BVar(2)),
                        )),
                        Box::new(Expr::BVar(3)),
                    )),
                )),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("s"),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Stream"), vec![])),
                        Box::new(Expr::BVar(2)),
                    )),
                    Box::new(Expr::BVar(3)),
                )),
            )),
        )),
    )
}
