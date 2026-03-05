//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::context::ElabContext;
use oxilean_kernel::{BinderInfo, Expr, FVarId, Level, Name};
use oxilean_parse::{Binder, BinderKind, Located, SurfaceExpr};

use super::functions::*;
use super::types::{
    AutoBoundImplicitInfo, BinderDep, BinderElabResult, BinderScope, BinderTypeInference,
    BinderTypeResult, BinderUniverse, BinderValidationError, BinderWithDefault, ImplicitStrictness,
    Telescope,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binder::*;
    use oxilean_kernel::Environment;
    use oxilean_parse::Span;
    fn mk_span() -> Span {
        Span::new(0, 1, 1, 1)
    }
    fn mk_located<T>(value: T) -> Located<T> {
        Located::new(value, mk_span())
    }
    #[test]
    fn test_elaborate_binders_empty() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let result = elaborate_binders(&mut ctx, &[]).expect("elaboration should succeed");
        assert!(result.is_empty());
    }
    #[test]
    fn test_elaborate_binders_single_typed() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let binder = make_binder(
            "x",
            Some(mk_located(SurfaceExpr::Sort(oxilean_parse::SortKind::Type))),
            BinderKind::Default,
        );
        let result = elaborate_binders(&mut ctx, &[binder]).expect("elaboration should succeed");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, Name::str("x"));
        assert_eq!(result[0].info, BinderInfo::Default);
    }
    #[test]
    fn test_elaborate_binders_single_untyped() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let binder = make_binder("x", None, BinderKind::Default);
        let result = elaborate_binders(&mut ctx, &[binder]).expect("elaboration should succeed");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, Name::str("x"));
        assert!(matches!(result[0].ty, Expr::FVar(_)));
    }
    #[test]
    fn test_elaborate_binders_multiple() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let binders = vec![
            make_binder(
                "a",
                Some(mk_located(SurfaceExpr::Sort(oxilean_parse::SortKind::Type))),
                BinderKind::Implicit,
            ),
            make_binder(
                "x",
                Some(mk_located(SurfaceExpr::Sort(oxilean_parse::SortKind::Prop))),
                BinderKind::Default,
            ),
        ];
        let result = elaborate_binders(&mut ctx, &binders).expect("elaboration should succeed");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name, Name::str("a"));
        assert_eq!(result[0].info, BinderInfo::Implicit);
        assert_eq!(result[1].name, Name::str("x"));
        assert_eq!(result[1].info, BinderInfo::Default);
    }
    #[test]
    fn test_elaborate_binders_instance() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let binder = make_binder(
            "inst",
            Some(mk_located(SurfaceExpr::Sort(oxilean_parse::SortKind::Type))),
            BinderKind::Instance,
        );
        let result = elaborate_binders(&mut ctx, &[binder]).expect("elaboration should succeed");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].info, BinderInfo::InstImplicit);
    }
    #[test]
    fn test_elaborate_binders_strict_implicit() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let binder = make_binder(
            "x",
            Some(mk_located(SurfaceExpr::Sort(oxilean_parse::SortKind::Type))),
            BinderKind::StrictImplicit,
        );
        let result = elaborate_binders(&mut ctx, &[binder]).expect("elaboration should succeed");
        assert_eq!(result[0].info, BinderInfo::StrictImplicit);
    }
    #[test]
    fn test_elaborate_binder_types_basic() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let binder = make_binder(
            "x",
            Some(mk_located(SurfaceExpr::Sort(oxilean_parse::SortKind::Prop))),
            BinderKind::Default,
        );
        let result =
            elaborate_binder_types(&mut ctx, &[binder]).expect("elaboration should succeed");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, Name::str("x"));
        assert!(matches!(result[0].1, Expr::Sort(_)));
        assert_eq!(result[0].2, BinderInfo::Default);
    }
    #[test]
    fn test_elaborate_binder_types_no_context_push() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let binder = make_binder(
            "x",
            Some(mk_located(SurfaceExpr::Sort(oxilean_parse::SortKind::Type))),
            BinderKind::Default,
        );
        let _result =
            elaborate_binder_types(&mut ctx, &[binder]).expect("elaboration should succeed");
        assert!(ctx.lookup_local(&Name::str("x")).is_none());
    }
    #[test]
    fn test_abstract_binders_empty() {
        let body = Expr::Lit(oxilean_kernel::Literal::Nat(42));
        let result = abstract_binders(&[], body.clone());
        assert_eq!(result, body);
    }
    #[test]
    fn test_abstract_binders_single() {
        let body = Expr::BVar(0);
        let binder = BinderElabResult {
            name: Name::str("x"),
            ty: Expr::Const(Name::str("Nat"), vec![]),
            info: BinderInfo::Default,
            fvar: FVarId(0),
        };
        let result = abstract_binders(&[binder], body);
        assert!(matches!(result, Expr::Lam(BinderInfo::Default, _, _, _)));
    }
    #[test]
    fn test_abstract_binders_nested() {
        let body = Expr::BVar(0);
        let binders = vec![
            BinderElabResult {
                name: Name::str("x"),
                ty: Expr::Const(Name::str("Nat"), vec![]),
                info: BinderInfo::Default,
                fvar: FVarId(0),
            },
            BinderElabResult {
                name: Name::str("y"),
                ty: Expr::Const(Name::str("Bool"), vec![]),
                info: BinderInfo::Implicit,
                fvar: FVarId(1),
            },
        ];
        let result = abstract_binders(&binders, body);
        if let Expr::Lam(BinderInfo::Default, name, _, inner) = &result {
            assert_eq!(name, &Name::str("x"));
            assert!(matches!(
                inner.as_ref(),
                Expr::Lam(BinderInfo::Implicit, _, _, _)
            ));
        } else {
            panic!("Expected nested Lam, got {:?}", result);
        }
    }
    #[test]
    fn test_pi_binders_single() {
        let body = Expr::Sort(Level::zero());
        let binder = BinderElabResult {
            name: Name::str("x"),
            ty: Expr::Const(Name::str("Nat"), vec![]),
            info: BinderInfo::Default,
            fvar: FVarId(0),
        };
        let result = pi_binders(&[binder], body);
        assert!(matches!(result, Expr::Pi(BinderInfo::Default, _, _, _)));
    }
    #[test]
    fn test_pi_binders_nested() {
        let body = Expr::Sort(Level::zero());
        let binders = vec![
            BinderElabResult {
                name: Name::str("α"),
                ty: Expr::Sort(Level::succ(Level::zero())),
                info: BinderInfo::Implicit,
                fvar: FVarId(0),
            },
            BinderElabResult {
                name: Name::str("x"),
                ty: Expr::FVar(FVarId(0)),
                info: BinderInfo::Default,
                fvar: FVarId(1),
            },
        ];
        let result = pi_binders(&binders, body);
        if let Expr::Pi(BinderInfo::Implicit, name, _, _) = &result {
            assert_eq!(name, &Name::str("α"));
        } else {
            panic!("Expected Pi with implicit, got {:?}", result);
        }
    }
    #[test]
    fn test_let_binders_single() {
        let body = Expr::BVar(0);
        let binder = BinderElabResult {
            name: Name::str("x"),
            ty: Expr::Const(Name::str("Nat"), vec![]),
            info: BinderInfo::Default,
            fvar: FVarId(0),
        };
        let val = Expr::Lit(oxilean_kernel::Literal::Nat(42));
        let result = let_binders(&[binder], &[val], body);
        assert!(matches!(result, Expr::Let(_, _, _, _)));
    }
    #[test]
    fn test_collect_binder_fvars_empty() {
        let expr = Expr::Lit(oxilean_kernel::Literal::Nat(0));
        let fvars = collect_binder_fvars(&expr);
        assert!(fvars.is_empty());
    }
    #[test]
    fn test_collect_binder_fvars_single() {
        let expr = Expr::FVar(FVarId(42));
        let fvars = collect_binder_fvars(&expr);
        assert_eq!(fvars.len(), 1);
        assert_eq!(fvars[0], FVarId(42));
    }
    #[test]
    fn test_collect_binder_fvars_nested() {
        let expr = Expr::App(
            Box::new(Expr::FVar(FVarId(1))),
            Box::new(Expr::FVar(FVarId(2))),
        );
        let fvars = collect_binder_fvars(&expr);
        assert_eq!(fvars.len(), 2);
        assert!(fvars.contains(&FVarId(1)));
        assert!(fvars.contains(&FVarId(2)));
    }
    #[test]
    fn test_collect_binder_fvars_dedup() {
        let expr = Expr::App(
            Box::new(Expr::FVar(FVarId(1))),
            Box::new(Expr::FVar(FVarId(1))),
        );
        let fvars = collect_binder_fvars(&expr);
        assert_eq!(fvars.len(), 1);
    }
    #[test]
    fn test_collect_unbound_vars_empty() {
        let env = Environment::new();
        let ctx = ElabContext::new(&env);
        let expr = mk_located(SurfaceExpr::Lit(oxilean_parse::Literal::Nat(42)));
        let unbound = collect_unbound_vars(&ctx, &expr);
        assert!(unbound.is_empty());
    }
    #[test]
    fn test_collect_unbound_vars_found() {
        let env = Environment::new();
        let ctx = ElabContext::new(&env);
        let expr = mk_located(SurfaceExpr::Var("alpha".to_string()));
        let unbound = collect_unbound_vars(&ctx, &expr);
        assert_eq!(unbound.len(), 1);
        assert_eq!(unbound[0], "alpha");
    }
    #[test]
    fn test_collect_unbound_vars_dedup() {
        let env = Environment::new();
        let ctx = ElabContext::new(&env);
        let expr = mk_located(SurfaceExpr::App(
            Box::new(mk_located(SurfaceExpr::Var("f".to_string()))),
            Box::new(mk_located(SurfaceExpr::Var("f".to_string()))),
        ));
        let unbound = collect_unbound_vars(&ctx, &expr);
        assert_eq!(unbound.len(), 1);
    }
    #[test]
    fn test_auto_bind_implicits_no_free() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let expr = Expr::BVar(0);
        let ty = Expr::Sort(Level::zero());
        let (new_expr, new_ty) = auto_bind_implicits(&mut ctx, expr.clone(), ty.clone());
        assert_eq!(new_expr, expr);
        assert_eq!(new_ty, ty);
    }
    #[test]
    fn test_auto_bind_implicits_with_free() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let fvar = FVarId(9999);
        let expr = Expr::FVar(fvar);
        let ty = Expr::FVar(fvar);
        let (new_expr, new_ty) = auto_bind_implicits(&mut ctx, expr, ty);
        assert!(matches!(new_expr, Expr::Lam(BinderInfo::Implicit, _, _, _)));
        assert!(matches!(new_ty, Expr::Pi(BinderInfo::Implicit, _, _, _)));
    }
    #[test]
    fn test_convert_binder_kind_default() {
        assert_eq!(
            convert_binder_kind(&BinderKind::Default),
            BinderInfo::Default
        );
    }
    #[test]
    fn test_convert_binder_kind_implicit() {
        assert_eq!(
            convert_binder_kind(&BinderKind::Implicit),
            BinderInfo::Implicit
        );
    }
    #[test]
    fn test_convert_binder_kind_instance() {
        assert_eq!(
            convert_binder_kind(&BinderKind::Instance),
            BinderInfo::InstImplicit
        );
    }
    #[test]
    fn test_convert_binder_kind_strict() {
        assert_eq!(
            convert_binder_kind(&BinderKind::StrictImplicit),
            BinderInfo::StrictImplicit
        );
    }
    #[test]
    fn test_is_implicit_binder() {
        assert!(!is_implicit_binder(&BinderKind::Default));
        assert!(is_implicit_binder(&BinderKind::Implicit));
        assert!(is_implicit_binder(&BinderKind::Instance));
        assert!(is_implicit_binder(&BinderKind::StrictImplicit));
    }
    #[test]
    fn test_pop_binders() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let ty = Expr::Sort(Level::zero());
        ctx.push_local(Name::str("a"), ty.clone(), None);
        ctx.push_local(Name::str("b"), ty.clone(), None);
        ctx.push_local(Name::str("c"), ty, None);
        pop_binders(&mut ctx, 2);
        assert!(ctx.lookup_local(&Name::str("a")).is_some());
        assert!(ctx.lookup_local(&Name::str("b")).is_none());
        assert!(ctx.lookup_local(&Name::str("c")).is_none());
    }
    #[test]
    fn test_insert_instance_implicits_no_implicits() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let ty = Expr::Sort(Level::zero());
        let result = insert_instance_implicits(&mut ctx, &ty);
        assert!(result.is_empty());
    }
    #[test]
    fn test_insert_instance_implicits_one_instance() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let inst_ty = Expr::App(
            Box::new(Expr::Const(Name::str("Add"), vec![])),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
        );
        let result_ty = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
        );
        let fun_ty = Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("inst"),
            Box::new(inst_ty),
            Box::new(result_ty),
        );
        let result = insert_instance_implicits(&mut ctx, &fun_ty);
        assert_eq!(result.len(), 1);
    }
    #[test]
    fn test_abstract_binders_tuple_basic() {
        let binders = vec![(
            Name::str("x"),
            Expr::Const(Name::str("Nat"), vec![]),
            BinderInfo::Default,
        )];
        let body = Expr::BVar(0);
        let result = abstract_binders_tuple(&binders, body);
        assert!(matches!(result, Expr::Lam(_, _, _, _)));
    }
    #[test]
    fn test_pi_binders_tuple_basic() {
        let binders = vec![(
            Name::str("x"),
            Expr::Const(Name::str("Nat"), vec![]),
            BinderInfo::Default,
        )];
        let body = Expr::Sort(Level::zero());
        let result = pi_binders_tuple(&binders, body);
        assert!(matches!(result, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_make_binder_no_type() {
        let b = make_binder("x", None, BinderKind::Default);
        assert_eq!(b.name, "x");
        assert!(b.ty.is_none());
        assert_eq!(b.info, BinderKind::Default);
    }
    #[test]
    fn test_make_binder_with_type() {
        let ty = mk_located(SurfaceExpr::Sort(oxilean_parse::SortKind::Type));
        let b = make_binder("y", Some(ty), BinderKind::Implicit);
        assert_eq!(b.name, "y");
        assert!(b.ty.is_some());
        assert_eq!(b.info, BinderKind::Implicit);
    }
    #[test]
    fn test_binder_scope_basic() {
        let mut scope = BinderScope::new();
        assert_eq!(scope.depth(), 0);
        scope.push(FVarId(1), Name::str("x"));
        assert_eq!(scope.depth(), 1);
        assert!(scope.contains_name(&Name::str("x")));
        assert!(scope.contains_fvar(FVarId(1)));
        scope.pop();
        assert_eq!(scope.depth(), 0);
    }
    #[test]
    fn test_binder_scope_child() {
        let mut scope = BinderScope::new();
        scope.push(FVarId(0), Name::str("a"));
        let child = scope.child();
        assert_eq!(child.depth(), 1);
        assert!(child.contains_name(&Name::str("a")));
    }
    #[test]
    fn test_binder_scope_from_binders() {
        let binders = vec![
            BinderElabResult {
                name: Name::str("x"),
                ty: Expr::Sort(Level::zero()),
                info: BinderInfo::Default,
                fvar: FVarId(10),
            },
            BinderElabResult {
                name: Name::str("y"),
                ty: Expr::Sort(Level::zero()),
                info: BinderInfo::Default,
                fvar: FVarId(11),
            },
        ];
        let scope = BinderScope::from_binders(&binders);
        assert_eq!(scope.depth(), 2);
        assert!(scope.contains_name(&Name::str("x")));
        assert!(scope.contains_fvar(FVarId(11)));
    }
    #[test]
    fn test_telescope_basic() {
        let mut tel = Telescope::new();
        assert!(tel.is_empty());
        tel.push(BinderElabResult {
            name: Name::str("x"),
            ty: Expr::Sort(Level::zero()),
            info: BinderInfo::Default,
            fvar: FVarId(0),
        });
        assert_eq!(tel.len(), 1);
        assert!(!tel.is_empty());
    }
    #[test]
    fn test_telescope_to_pi() {
        let tel = Telescope {
            binders: vec![BinderElabResult {
                name: Name::str("x"),
                ty: Expr::Const(Name::str("Nat"), vec![]),
                info: BinderInfo::Default,
                fvar: FVarId(0),
            }],
        };
        let pi = tel.to_pi(Expr::Sort(Level::zero()));
        assert!(matches!(pi, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_telescope_to_lam() {
        let tel = Telescope {
            binders: vec![BinderElabResult {
                name: Name::str("x"),
                ty: Expr::Const(Name::str("Nat"), vec![]),
                info: BinderInfo::Default,
                fvar: FVarId(0),
            }],
        };
        let lam = tel.to_lam(Expr::BVar(0));
        assert!(matches!(lam, Expr::Lam(_, _, _, _)));
    }
    #[test]
    fn test_telescope_split_implicit() {
        let tel = Telescope {
            binders: vec![
                BinderElabResult {
                    name: Name::str("α"),
                    ty: Expr::Sort(Level::succ(Level::zero())),
                    info: BinderInfo::Implicit,
                    fvar: FVarId(0),
                },
                BinderElabResult {
                    name: Name::str("x"),
                    ty: Expr::FVar(FVarId(0)),
                    info: BinderInfo::Default,
                    fvar: FVarId(1),
                },
            ],
        };
        let (implicit, explicit) = tel.split_implicit();
        assert_eq!(implicit.len(), 1);
        assert_eq!(explicit.len(), 1);
    }
    #[test]
    fn test_is_dependent_binder_true() {
        let binders = vec![
            BinderElabResult {
                name: Name::str("α"),
                ty: Expr::Sort(Level::succ(Level::zero())),
                info: BinderInfo::Implicit,
                fvar: FVarId(0),
            },
            BinderElabResult {
                name: Name::str("x"),
                ty: Expr::FVar(FVarId(0)),
                info: BinderInfo::Default,
                fvar: FVarId(1),
            },
        ];
        assert!(is_dependent_binder(&binders, 0));
        assert!(!is_dependent_binder(&binders, 1));
    }
    #[test]
    fn test_has_dependent_binders_true() {
        let binders = vec![
            BinderElabResult {
                name: Name::str("α"),
                ty: Expr::Sort(Level::succ(Level::zero())),
                info: BinderInfo::Implicit,
                fvar: FVarId(0),
            },
            BinderElabResult {
                name: Name::str("x"),
                ty: Expr::FVar(FVarId(0)),
                info: BinderInfo::Default,
                fvar: FVarId(1),
            },
        ];
        assert!(has_dependent_binders(&binders));
    }
    #[test]
    fn test_dependent_binder_indices() {
        let binders = vec![
            BinderElabResult {
                name: Name::str("α"),
                ty: Expr::Sort(Level::succ(Level::zero())),
                info: BinderInfo::Implicit,
                fvar: FVarId(0),
            },
            BinderElabResult {
                name: Name::str("x"),
                ty: Expr::FVar(FVarId(0)),
                info: BinderInfo::Default,
                fvar: FVarId(1),
            },
        ];
        let indices = dependent_binder_indices(&binders);
        assert_eq!(indices, vec![0]);
    }
    #[test]
    fn test_is_anonymous_binder() {
        let b = make_anonymous_binder(Located::new(
            SurfaceExpr::Sort(oxilean_parse::SortKind::Type),
            mk_span(),
        ));
        assert!(is_anonymous_binder(&b));
    }
    #[test]
    fn test_make_anonymous_result() {
        let r = make_anonymous_result(Expr::Sort(Level::zero()), FVarId(99));
        assert!(is_anonymous_result(&r));
    }
    #[test]
    fn test_resolve_named_binder_found() {
        let binders = vec![
            BinderElabResult {
                name: Name::str("x"),
                ty: Expr::Sort(Level::zero()),
                info: BinderInfo::Default,
                fvar: FVarId(0),
            },
            BinderElabResult {
                name: Name::str("y"),
                ty: Expr::Sort(Level::zero()),
                info: BinderInfo::Default,
                fvar: FVarId(1),
            },
        ];
        let result = resolve_named_binder(&binders, &Name::str("y"));
        assert!(result.is_some());
        let (idx, b) = result.expect("test operation should succeed");
        assert_eq!(idx, 1);
        assert_eq!(b.name, Name::str("y"));
    }
    #[test]
    fn test_resolve_named_binder_not_found() {
        let binders = vec![BinderElabResult {
            name: Name::str("x"),
            ty: Expr::Sort(Level::zero()),
            info: BinderInfo::Default,
            fvar: FVarId(0),
        }];
        assert!(resolve_named_binder(&binders, &Name::str("z")).is_none());
    }
    #[test]
    fn test_reorder_named_args() {
        let binders = vec![
            BinderElabResult {
                name: Name::str("x"),
                ty: Expr::Sort(Level::zero()),
                info: BinderInfo::Default,
                fvar: FVarId(0),
            },
            BinderElabResult {
                name: Name::str("y"),
                ty: Expr::Sort(Level::zero()),
                info: BinderInfo::Default,
                fvar: FVarId(1),
            },
        ];
        let named = vec![
            (Name::str("y"), Expr::Lit(oxilean_kernel::Literal::Nat(2))),
            (Name::str("x"), Expr::Lit(oxilean_kernel::Literal::Nat(1))),
        ];
        let ordered = reorder_named_args(&binders, &named);
        assert_eq!(ordered.len(), 2);
        assert!(ordered[0].is_some());
        assert!(ordered[1].is_some());
    }
    #[test]
    fn test_binder_with_default_no_default() {
        let result = BinderElabResult {
            name: Name::str("x"),
            ty: Expr::Sort(Level::zero()),
            info: BinderInfo::Default,
            fvar: FVarId(0),
        };
        let b = BinderWithDefault::no_default(result);
        assert!(!b.has_default());
    }
    #[test]
    fn test_binder_with_default_has_default() {
        let result = BinderElabResult {
            name: Name::str("x"),
            ty: Expr::Sort(Level::zero()),
            info: BinderInfo::Default,
            fvar: FVarId(0),
        };
        let val = Expr::Lit(oxilean_kernel::Literal::Nat(42));
        let b = BinderWithDefault::with_default(result, val);
        assert!(b.has_default());
    }
    #[test]
    fn test_validate_binder_ok() {
        let b = Binder {
            name: "x".to_string(),
            ty: None,
            info: BinderKind::Default,
        };
        assert!(validate_binder(&b).is_ok());
    }
    #[test]
    fn test_validate_binder_empty_name() {
        let b = Binder {
            name: "".to_string(),
            ty: None,
            info: BinderKind::Default,
        };
        assert!(matches!(
            validate_binder(&b),
            Err(BinderValidationError::EmptyName)
        ));
    }
    #[test]
    fn test_validate_binder_reserved_name() {
        let b = Binder {
            name: "def".to_string(),
            ty: None,
            info: BinderKind::Default,
        };
        assert!(matches!(
            validate_binder(&b),
            Err(BinderValidationError::ReservedName(_))
        ));
    }
    #[test]
    fn test_validate_binder_instance_without_type() {
        let b = Binder {
            name: "inst".to_string(),
            ty: None,
            info: BinderKind::Instance,
        };
        assert!(matches!(
            validate_binder(&b),
            Err(BinderValidationError::InstanceBinderWithoutType)
        ));
    }
    #[test]
    fn test_count_binder_kinds() {
        let binders = vec![
            Binder {
                name: "α".to_string(),
                ty: None,
                info: BinderKind::Implicit,
            },
            Binder {
                name: "x".to_string(),
                ty: None,
                info: BinderKind::Default,
            },
            Binder {
                name: "inst".to_string(),
                ty: None,
                info: BinderKind::Instance,
            },
            Binder {
                name: "y".to_string(),
                ty: None,
                info: BinderKind::Default,
            },
        ];
        let counts = count_binder_kinds(&binders);
        assert_eq!(counts.explicit, 2);
        assert_eq!(counts.implicit, 1);
        assert_eq!(counts.instance, 1);
        assert_eq!(counts.strict, 0);
        assert_eq!(counts.total(), 4);
        assert_eq!(counts.any_implicit(), 2);
    }
    #[test]
    fn test_count_elab_binder_kinds() {
        let binders = vec![
            BinderElabResult {
                name: Name::str("α"),
                ty: Expr::Sort(Level::succ(Level::zero())),
                info: BinderInfo::Implicit,
                fvar: FVarId(0),
            },
            BinderElabResult {
                name: Name::str("x"),
                ty: Expr::Sort(Level::zero()),
                info: BinderInfo::Default,
                fvar: FVarId(1),
            },
        ];
        let counts = count_elab_binder_kinds(&binders);
        assert_eq!(counts.explicit, 1);
        assert_eq!(counts.implicit, 1);
    }
    #[test]
    fn test_classify_binder_universe_prop() {
        assert_eq!(
            classify_binder_universe(&Expr::Sort(Level::zero())),
            BinderUniverse::Prop
        );
    }
    #[test]
    fn test_classify_binder_universe_type() {
        assert_eq!(
            classify_binder_universe(&Expr::Sort(Level::succ(Level::zero()))),
            BinderUniverse::Type
        );
    }
    #[test]
    fn test_classify_binder_universe_unknown() {
        assert_eq!(
            classify_binder_universe(&Expr::Const(Name::str("Nat"), vec![])),
            BinderUniverse::Unknown
        );
    }
    #[test]
    fn test_implicit_strictness_from_kind() {
        assert_eq!(
            ImplicitStrictness::from_kind(&BinderKind::Implicit),
            Some(ImplicitStrictness::Regular)
        );
        assert_eq!(
            ImplicitStrictness::from_kind(&BinderKind::StrictImplicit),
            Some(ImplicitStrictness::Strict)
        );
        assert_eq!(ImplicitStrictness::from_kind(&BinderKind::Default), None);
    }
    #[test]
    fn test_implicit_strictness_is_eager() {
        assert!(ImplicitStrictness::Regular.is_eager());
        assert!(!ImplicitStrictness::Strict.is_eager());
    }
    #[test]
    fn test_auto_bound_info_greek() {
        let info = AutoBoundImplicitInfo::for_name("α");
        assert!(info.looks_like_type_var);
    }
    #[test]
    fn test_auto_bound_info_term_var() {
        let info = AutoBoundImplicitInfo::for_name("x");
        assert!(info.looks_like_term_var);
        assert!(!info.looks_like_type_var);
    }
    #[test]
    fn test_expr_contains_fvar_true() {
        let expr = Expr::App(
            Box::new(Expr::FVar(FVarId(5))),
            Box::new(Expr::Lit(oxilean_kernel::Literal::Nat(0))),
        );
        assert!(expr_contains_fvar(&expr, FVarId(5)));
    }
    #[test]
    fn test_expr_contains_fvar_false() {
        let expr = Expr::Const(Name::str("Nat"), vec![]);
        assert!(!expr_contains_fvar(&expr, FVarId(0)));
    }
    #[test]
    fn test_infer_binder_type_from_context_no_expected() {
        let env = Environment::new();
        let ctx = ElabContext::new(&env);
        let (ty, strategy) = infer_binder_type_from_context(&ctx, None, 0);
        assert!(ty.is_none());
        assert_eq!(strategy, BinderTypeInference::Fresh);
    }
    #[test]
    fn test_infer_binder_type_from_expected() {
        let env = Environment::new();
        let ctx = ElabContext::new(&env);
        let expected = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(Expr::Sort(Level::zero())),
        );
        let (ty, strategy) = infer_binder_type_from_context(&ctx, Some(&expected), 0);
        assert!(ty.is_some());
        assert_eq!(strategy, BinderTypeInference::FromExpected);
    }
    #[test]
    fn test_abstract_over_telescope_empty() {
        let body = Expr::Lit(oxilean_kernel::Literal::Nat(42));
        let result = abstract_over_telescope(&[], body.clone());
        assert_eq!(result, body);
    }
    #[test]
    fn test_abstract_over_telescope_single() {
        let binders = vec![BinderElabResult {
            name: Name::str("x"),
            ty: Expr::Const(Name::str("Nat"), vec![]),
            info: BinderInfo::Default,
            fvar: FVarId(100),
        }];
        let body = Expr::FVar(FVarId(100));
        let result = abstract_over_telescope(&binders, body);
        assert!(matches!(result, Expr::Lam(_, _, _, _)));
    }
    #[test]
    fn test_build_dependency_graph_no_deps() {
        let binders = vec![
            BinderElabResult {
                name: Name::str("x"),
                ty: Expr::Const(Name::str("Nat"), vec![]),
                info: BinderInfo::Default,
                fvar: FVarId(0),
            },
            BinderElabResult {
                name: Name::str("y"),
                ty: Expr::Const(Name::str("Bool"), vec![]),
                info: BinderInfo::Default,
                fvar: FVarId(1),
            },
        ];
        let graph = build_dependency_graph(&binders);
        assert!(graph.is_empty());
    }
    #[test]
    fn test_build_dependency_graph_with_dep() {
        let binders = vec![
            BinderElabResult {
                name: Name::str("α"),
                ty: Expr::Sort(Level::succ(Level::zero())),
                info: BinderInfo::Implicit,
                fvar: FVarId(10),
            },
            BinderElabResult {
                name: Name::str("x"),
                ty: Expr::FVar(FVarId(10)),
                info: BinderInfo::Default,
                fvar: FVarId(11),
            },
        ];
        let graph = build_dependency_graph(&binders);
        assert_eq!(graph.len(), 1);
        assert_eq!(graph[0].from, 1);
        assert_eq!(graph[0].to, 0);
    }
    #[test]
    fn test_topological_binder_order_no_deps() {
        let order = topological_binder_order(3, &[]);
        assert!(order.is_some());
        assert_eq!(order.expect("test operation should succeed").len(), 3);
    }
    #[test]
    fn test_topological_binder_order_linear() {
        let deps = vec![BinderDep { from: 1, to: 0 }, BinderDep { from: 2, to: 1 }];
        let order = topological_binder_order(3, &deps).expect("test operation should succeed");
        let pos: std::collections::HashMap<usize, usize> =
            order.iter().enumerate().map(|(i, &v)| (v, i)).collect();
        assert!(pos[&0] < pos[&1]);
        assert!(pos[&1] < pos[&2]);
    }
    #[test]
    fn test_normalise_binder_names() {
        let mut binders = vec![
            Binder {
                name: "  x  ".to_string(),
                ty: None,
                info: BinderKind::Default,
            },
            Binder {
                name: "".to_string(),
                ty: None,
                info: BinderKind::Default,
            },
        ];
        normalise_binder_names(&mut binders);
        assert_eq!(binders[0].name, "x");
        assert_eq!(binders[1].name, "_");
    }
    #[test]
    fn test_binders_have_distinct_names_ok() {
        let binders = vec![
            Binder {
                name: "x".to_string(),
                ty: None,
                info: BinderKind::Default,
            },
            Binder {
                name: "y".to_string(),
                ty: None,
                info: BinderKind::Default,
            },
        ];
        assert!(binders_have_distinct_names(&binders));
    }
    #[test]
    fn test_binders_have_distinct_names_dup() {
        let binders = vec![
            Binder {
                name: "x".to_string(),
                ty: None,
                info: BinderKind::Default,
            },
            Binder {
                name: "x".to_string(),
                ty: None,
                info: BinderKind::Default,
            },
        ];
        assert!(!binders_have_distinct_names(&binders));
    }
    #[test]
    fn test_duplicate_name_indices() {
        let binders = vec![
            Binder {
                name: "x".to_string(),
                ty: None,
                info: BinderKind::Default,
            },
            Binder {
                name: "y".to_string(),
                ty: None,
                info: BinderKind::Default,
            },
            Binder {
                name: "x".to_string(),
                ty: None,
                info: BinderKind::Default,
            },
        ];
        let dupes = duplicate_name_indices(&binders);
        assert_eq!(dupes, vec![2]);
    }
    #[test]
    fn test_validate_binders_ok() {
        let binders = vec![
            Binder {
                name: "x".to_string(),
                ty: None,
                info: BinderKind::Default,
            },
            Binder {
                name: "y".to_string(),
                ty: None,
                info: BinderKind::Default,
            },
        ];
        assert!(validate_binders(&binders).is_ok());
    }
    #[test]
    fn test_binder_validation_error_display() {
        assert!(BinderValidationError::EmptyName
            .to_string()
            .contains("empty"));
        assert!(BinderValidationError::ReservedName("def".into())
            .to_string()
            .contains("def"));
        assert!(BinderValidationError::InstanceBinderWithoutType
            .to_string()
            .contains("instance"));
    }
    #[test]
    fn test_BinderTypeResult_fields() {
        let r = BinderTypeResult {
            name: Name::str("x"),
            ty: Expr::Sort(Level::zero()),
            info: BinderInfo::Default,
        };
        assert_eq!(r.name, Name::str("x"));
        assert_eq!(r.info, BinderInfo::Default);
    }
}
