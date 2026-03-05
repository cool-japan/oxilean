//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::context::ElabContext;
use oxilean_kernel::{BinderInfo, Expr, Level, Name};
use oxilean_parse::{Lexer, Located, Parser, SortKind, StringPart, SurfaceExpr};

use super::functions::*;
use super::types::{ElabError, ElabRunMetrics};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elaborate::*;
    use oxilean_kernel::Environment;
    use oxilean_parse::{Binder, BinderKind, Span};
    fn mk_span() -> Span {
        Span::new(0, 1, 1, 1)
    }
    fn mk_located<T>(value: T) -> Located<T> {
        Located::new(value, mk_span())
    }
    fn mk_var(name: &str) -> Located<SurfaceExpr> {
        mk_located(SurfaceExpr::Var(name.to_string()))
    }
    fn mk_nat(n: u64) -> Located<SurfaceExpr> {
        mk_located(SurfaceExpr::Lit(oxilean_parse::Literal::Nat(n)))
    }
    fn mk_str(s: &str) -> Located<SurfaceExpr> {
        mk_located(SurfaceExpr::Lit(oxilean_parse::Literal::String(
            s.to_string(),
        )))
    }
    fn mk_binder(name: &str, ty: Option<Located<SurfaceExpr>>, kind: BinderKind) -> Binder {
        Binder {
            name: name.to_string(),
            ty: ty.map(Box::new),
            info: kind,
        }
    }
    #[test]
    fn test_elaborate_sort_type() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let sort = mk_located(SurfaceExpr::Sort(SortKind::Type));
        let result = elaborate_expr(&mut ctx, &sort).expect("elaboration should succeed");
        assert!(matches!(result, Expr::Sort(_)));
    }
    #[test]
    fn test_elaborate_sort_prop() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let sort = mk_located(SurfaceExpr::Sort(SortKind::Prop));
        let result = elaborate_expr(&mut ctx, &sort).expect("elaboration should succeed");
        assert!(matches!(result, Expr::Sort(l) if l == Level::zero()));
    }
    #[test]
    fn test_elaborate_sort_type_u() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let sort = mk_located(SurfaceExpr::Sort(SortKind::TypeU("u".to_string())));
        let result = elaborate_expr(&mut ctx, &sort).expect("elaboration should succeed");
        assert!(matches!(result, Expr::Sort(Level::Param(_))));
    }
    #[test]
    fn test_elaborate_var_not_found() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let var = mk_var("x");
        let result = elaborate_expr(&mut ctx, &var);
        assert!(result.is_err());
    }
    #[test]
    fn test_elaborate_var_local() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let fvar = ctx.push_local(Name::str("x"), Expr::Const(Name::str("Nat"), vec![]), None);
        let var = mk_var("x");
        let result = elaborate_expr(&mut ctx, &var).expect("elaboration should succeed");
        assert_eq!(result, Expr::FVar(fvar));
    }
    #[test]
    fn test_elaborate_lit_nat() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let lit = mk_nat(42);
        let result = elaborate_expr(&mut ctx, &lit).expect("elaboration should succeed");
        assert_eq!(result, Expr::Lit(oxilean_kernel::Literal::Nat(42)));
    }
    #[test]
    fn test_elaborate_lit_string() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let lit = mk_str("hello");
        let result = elaborate_expr(&mut ctx, &lit).expect("elaboration should succeed");
        assert_eq!(
            result,
            Expr::Lit(oxilean_kernel::Literal::Str("hello".to_string()))
        );
    }
    #[test]
    fn test_elaborate_lit_char() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let lit = mk_located(SurfaceExpr::Lit(oxilean_parse::Literal::Char('a')));
        let result = elaborate_expr(&mut ctx, &lit).expect("elaboration should succeed");
        assert_eq!(
            result,
            Expr::Lit(oxilean_kernel::Literal::Str("a".to_string()))
        );
    }
    #[test]
    fn test_elaborate_lit_float() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        #[allow(clippy::approx_constant)]
        let lit = mk_located(SurfaceExpr::Lit(oxilean_parse::Literal::Float(3.14)));
        let result = elaborate_expr(&mut ctx, &lit).expect("elaboration should succeed");
        assert_eq!(result, Expr::Lit(oxilean_kernel::Literal::Nat(0)));
    }
    #[test]
    fn test_elaborate_hole() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let hole = mk_located(SurfaceExpr::Hole);
        let result = elaborate_expr(&mut ctx, &hole).expect("elaboration should succeed");
        assert!(matches!(result, Expr::FVar(_)));
    }
    #[test]
    fn test_elaborate_app_basic() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        ctx.push_local(Name::str("f"), Expr::Sort(Level::succ(Level::zero())), None);
        ctx.push_local(Name::str("x"), Expr::Const(Name::str("Nat"), vec![]), None);
        let app = mk_located(SurfaceExpr::App(
            Box::new(mk_var("f")),
            Box::new(mk_var("x")),
        ));
        let result = elaborate_expr(&mut ctx, &app).expect("elaboration should succeed");
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_elaborate_lambda_typed() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let lam = mk_located(SurfaceExpr::Lam(
            vec![mk_binder(
                "x",
                Some(mk_located(SurfaceExpr::Sort(SortKind::Type))),
                BinderKind::Default,
            )],
            Box::new(mk_var("x")),
        ));
        let result = elaborate_expr(&mut ctx, &lam).expect("elaboration should succeed");
        assert!(matches!(result, Expr::Lam(_, _, _, _)));
    }
    #[test]
    fn test_elaborate_lambda_untyped() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let lam = mk_located(SurfaceExpr::Lam(
            vec![mk_binder("x", None, BinderKind::Default)],
            Box::new(mk_var("x")),
        ));
        let result = elaborate_expr(&mut ctx, &lam).expect("elaboration should succeed");
        assert!(matches!(result, Expr::Lam(_, _, _, _)));
    }
    #[test]
    fn test_elaborate_lambda_with_expected_pi() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let expected = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
        );
        let lam_expr = mk_located(SurfaceExpr::Lam(
            vec![mk_binder("x", None, BinderKind::Default)],
            Box::new(mk_var("x")),
        ));
        let result = elaborate_with_expected_type(&mut ctx, &lam_expr, &expected)
            .expect("elaboration should succeed");
        if let Expr::Lam(_, _, ty, _) = &result {
            assert_eq!(**ty, Expr::Const(Name::str("Nat"), vec![]));
        } else {
            panic!("Expected Lam, got {:?}", result);
        }
    }
    #[test]
    fn test_elaborate_lambda_multiple_binders() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let lam = mk_located(SurfaceExpr::Lam(
            vec![
                mk_binder(
                    "x",
                    Some(mk_located(SurfaceExpr::Sort(SortKind::Type))),
                    BinderKind::Default,
                ),
                mk_binder(
                    "y",
                    Some(mk_located(SurfaceExpr::Sort(SortKind::Prop))),
                    BinderKind::Default,
                ),
            ],
            Box::new(mk_var("x")),
        ));
        let result = elaborate_expr(&mut ctx, &lam).expect("elaboration should succeed");
        if let Expr::Lam(_, name, _, inner) = &result {
            assert_eq!(name, &Name::str("x"));
            assert!(matches!(inner.as_ref(), Expr::Lam(_, _, _, _)));
        } else {
            panic!("Expected nested Lam");
        }
    }
    #[test]
    fn test_elaborate_pi_basic() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let pi = mk_located(SurfaceExpr::Pi(
            vec![mk_binder(
                "x",
                Some(mk_located(SurfaceExpr::Sort(SortKind::Type))),
                BinderKind::Default,
            )],
            Box::new(mk_located(SurfaceExpr::Sort(SortKind::Type))),
        ));
        let result = elaborate_expr(&mut ctx, &pi).expect("elaboration should succeed");
        assert!(matches!(result, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_elaborate_pi_no_type_fails() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let pi = mk_located(SurfaceExpr::Pi(
            vec![mk_binder("x", None, BinderKind::Default)],
            Box::new(mk_located(SurfaceExpr::Sort(SortKind::Type))),
        ));
        let result = elaborate_expr(&mut ctx, &pi);
        assert!(result.is_err());
    }
    #[test]
    fn test_elaborate_let_typed() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let let_expr = mk_located(SurfaceExpr::Let(
            "x".to_string(),
            Some(Box::new(mk_located(SurfaceExpr::Sort(SortKind::Type)))),
            Box::new(mk_located(SurfaceExpr::Sort(SortKind::Prop))),
            Box::new(mk_var("x")),
        ));
        let result = elaborate_expr(&mut ctx, &let_expr).expect("elaboration should succeed");
        assert!(matches!(result, Expr::Let(_, _, _, _)));
    }
    #[test]
    fn test_elaborate_let_untyped() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let let_expr = mk_located(SurfaceExpr::Let(
            "x".to_string(),
            None,
            Box::new(mk_nat(42)),
            Box::new(mk_var("x")),
        ));
        let result = elaborate_expr(&mut ctx, &let_expr).expect("elaboration should succeed");
        assert!(matches!(result, Expr::Let(_, _, _, _)));
    }
    #[test]
    fn test_elaborate_annotation() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let ann = mk_located(SurfaceExpr::Ann(
            Box::new(mk_nat(42)),
            Box::new(mk_located(SurfaceExpr::Sort(SortKind::Type))),
        ));
        let result = elaborate_expr(&mut ctx, &ann).expect("elaboration should succeed");
        assert_eq!(result, Expr::Lit(oxilean_kernel::Literal::Nat(42)));
    }
    #[test]
    fn test_elaborate_proj() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        ctx.push_local(Name::str("p"), Expr::Const(Name::str("Prod"), vec![]), None);
        let proj = mk_located(SurfaceExpr::Proj(Box::new(mk_var("p")), "fst".to_string()));
        let result = elaborate_expr(&mut ctx, &proj).expect("elaboration should succeed");
        assert!(matches!(result, Expr::Proj(_, _, _)));
    }
    #[test]
    fn test_elaborate_if() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        ctx.push_local(Name::str("b"), Expr::Const(Name::str("Bool"), vec![]), None);
        let if_expr = mk_located(SurfaceExpr::If(
            Box::new(mk_var("b")),
            Box::new(mk_nat(1)),
            Box::new(mk_nat(0)),
        ));
        let result = elaborate_expr(&mut ctx, &if_expr).expect("elaboration should succeed");
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_elaborate_have() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let have_expr = mk_located(SurfaceExpr::Have(
            "h".to_string(),
            Box::new(mk_located(SurfaceExpr::Sort(SortKind::Prop))),
            Box::new(mk_located(SurfaceExpr::Hole)),
            Box::new(mk_var("h")),
        ));
        let result = elaborate_expr(&mut ctx, &have_expr).expect("elaboration should succeed");
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_elaborate_suffices() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let suffices_expr = mk_located(SurfaceExpr::Suffices(
            "h".to_string(),
            Box::new(mk_located(SurfaceExpr::Sort(SortKind::Prop))),
            Box::new(mk_located(SurfaceExpr::Hole)),
        ));
        let result = elaborate_expr(&mut ctx, &suffices_expr).expect("elaboration should succeed");
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_elaborate_show() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let show_expr = mk_located(SurfaceExpr::Show(
            Box::new(mk_located(SurfaceExpr::Sort(SortKind::Prop))),
            Box::new(mk_nat(42)),
        ));
        let result = elaborate_expr(&mut ctx, &show_expr).expect("elaboration should succeed");
        assert_eq!(result, Expr::Lit(oxilean_kernel::Literal::Nat(42)));
    }
    #[test]
    fn test_elaborate_list_empty() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let list = mk_located(SurfaceExpr::ListLit(vec![]));
        let result = elaborate_expr(&mut ctx, &list).expect("elaboration should succeed");
        assert_eq!(result, Expr::Const(Name::str("List.nil"), vec![]));
    }
    #[test]
    fn test_elaborate_list_singleton() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let list = mk_located(SurfaceExpr::ListLit(vec![mk_nat(1)]));
        let result = elaborate_expr(&mut ctx, &list).expect("elaboration should succeed");
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_elaborate_list_multiple() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let list = mk_located(SurfaceExpr::ListLit(vec![mk_nat(1), mk_nat(2), mk_nat(3)]));
        let result = elaborate_expr(&mut ctx, &list).expect("elaboration should succeed");
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_elaborate_tuple_empty() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let tuple = mk_located(SurfaceExpr::Tuple(vec![]));
        let result = elaborate_expr(&mut ctx, &tuple).expect("elaboration should succeed");
        assert_eq!(result, Expr::Const(Name::str("Unit.unit"), vec![]));
    }
    #[test]
    fn test_elaborate_tuple_single() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let tuple = mk_located(SurfaceExpr::Tuple(vec![mk_nat(42)]));
        let result = elaborate_expr(&mut ctx, &tuple).expect("elaboration should succeed");
        assert_eq!(result, Expr::Lit(oxilean_kernel::Literal::Nat(42)));
    }
    #[test]
    fn test_elaborate_tuple_pair() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let tuple = mk_located(SurfaceExpr::Tuple(vec![mk_nat(1), mk_nat(2)]));
        let result = elaborate_expr(&mut ctx, &tuple).expect("elaboration should succeed");
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_elaborate_tuple_triple() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let tuple = mk_located(SurfaceExpr::Tuple(vec![mk_nat(1), mk_nat(2), mk_nat(3)]));
        let result = elaborate_expr(&mut ctx, &tuple).expect("elaboration should succeed");
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_elaborate_anonymous_ctor() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let ctor = mk_located(SurfaceExpr::AnonymousCtor(vec![mk_nat(1), mk_nat(2)]));
        let result = elaborate_expr(&mut ctx, &ctor).expect("elaboration should succeed");
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_elaborate_return() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let ret = mk_located(SurfaceExpr::Return(Box::new(mk_nat(42))));
        let result = elaborate_expr(&mut ctx, &ret).expect("elaboration should succeed");
        if let Expr::App(f, a) = &result {
            assert_eq!(**f, Expr::Const(Name::str("Pure.pure"), vec![]));
            assert_eq!(**a, Expr::Lit(oxilean_kernel::Literal::Nat(42)));
        } else {
            panic!("Expected App, got {:?}", result);
        }
    }
    #[test]
    fn test_elaborate_string_interp_literal_only() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let interp = mk_located(SurfaceExpr::StringInterp(vec![StringPart::Literal(
            "hello".to_string(),
        )]));
        let result = elaborate_expr(&mut ctx, &interp).expect("elaboration should succeed");
        assert_eq!(
            result,
            Expr::Lit(oxilean_kernel::Literal::Str("hello".to_string()))
        );
    }
    #[test]
    fn test_elaborate_string_interp_empty() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let interp = mk_located(SurfaceExpr::StringInterp(vec![]));
        let result = elaborate_expr(&mut ctx, &interp).expect("elaboration should succeed");
        assert_eq!(
            result,
            Expr::Lit(oxilean_kernel::Literal::Str(String::new()))
        );
    }
    #[test]
    fn test_elaborate_string_interp_with_hole() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let interp = mk_located(SurfaceExpr::StringInterp(vec![
            StringPart::Literal("x = ".to_string()),
            StringPart::Interpolation(vec![]),
        ]));
        let result = elaborate_expr(&mut ctx, &interp).expect("elaboration should succeed");
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_elaborate_range_full() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let range = mk_located(SurfaceExpr::Range(
            Some(Box::new(mk_nat(0))),
            Some(Box::new(mk_nat(10))),
        ));
        let result = elaborate_expr(&mut ctx, &range).expect("elaboration should succeed");
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_elaborate_range_from_zero() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let range = mk_located(SurfaceExpr::Range(None, Some(Box::new(mk_nat(10)))));
        let result = elaborate_expr(&mut ctx, &range).expect("elaboration should succeed");
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_elaborate_by_tactic() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let by = mk_located(SurfaceExpr::ByTactic(vec![mk_located(
            "exact rfl".to_string(),
        )]));
        let result = elaborate_expr(&mut ctx, &by).expect("elaboration should succeed");
        assert!(matches!(result, Expr::FVar(_)));
    }
    #[test]
    fn test_elaborate_calc_single_step() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        ctx.push_local(Name::str("proof1"), Expr::Sort(Level::zero()), None);
        let calc = mk_located(SurfaceExpr::Calc(vec![oxilean_parse::AstCalcStep {
            lhs: mk_nat(1),
            rel: "=".to_string(),
            rhs: mk_nat(1),
            proof: mk_var("proof1"),
        }]));
        let result = elaborate_expr(&mut ctx, &calc).expect("elaboration should succeed");
        assert!(matches!(result, Expr::FVar(_)));
    }
    #[test]
    fn test_elaborate_calc_empty_fails() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let calc = mk_located(SurfaceExpr::Calc(vec![]));
        let result = elaborate_expr(&mut ctx, &calc);
        assert!(result.is_err());
    }
    #[test]
    fn test_elaborate_do_single_expr() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let do_expr = mk_located(SurfaceExpr::Do(vec![oxilean_parse::DoAction::Expr(
            mk_nat(42),
        )]));
        let result = elaborate_expr(&mut ctx, &do_expr).expect("elaboration should succeed");
        assert_eq!(result, Expr::Lit(oxilean_kernel::Literal::Nat(42)));
    }
    #[test]
    fn test_elaborate_do_return() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let do_expr = mk_located(SurfaceExpr::Do(vec![oxilean_parse::DoAction::Return(
            mk_nat(42),
        )]));
        let result = elaborate_expr(&mut ctx, &do_expr).expect("elaboration should succeed");
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_elaborate_do_let_then_expr() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let do_expr = mk_located(SurfaceExpr::Do(vec![
            oxilean_parse::DoAction::Let("x".to_string(), mk_nat(42)),
            oxilean_parse::DoAction::Expr(mk_var("x")),
        ]));
        let result = elaborate_expr(&mut ctx, &do_expr).expect("elaboration should succeed");
        assert!(matches!(result, Expr::Let(_, _, _, _)));
    }
    #[test]
    fn test_elaborate_do_bind() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        ctx.push_local(
            Name::str("getLine"),
            Expr::Sort(Level::succ(Level::zero())),
            None,
        );
        let do_expr = mk_located(SurfaceExpr::Do(vec![
            oxilean_parse::DoAction::Bind("x".to_string(), mk_var("getLine")),
            oxilean_parse::DoAction::Expr(mk_var("x")),
        ]));
        let result = elaborate_expr(&mut ctx, &do_expr).expect("elaboration should succeed");
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_elaborate_do_empty_fails() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let do_expr = mk_located(SurfaceExpr::Do(vec![]));
        let result = elaborate_expr(&mut ctx, &do_expr);
        assert!(result.is_err());
    }
    #[test]
    fn test_elaborate_match_basic() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        ctx.push_local(Name::str("n"), Expr::Const(Name::str("Nat"), vec![]), None);
        let match_expr = mk_located(SurfaceExpr::Match(
            Box::new(mk_var("n")),
            vec![oxilean_parse::MatchArm {
                pattern: mk_located(oxilean_parse::Pattern::Wild),
                guard: None,
                rhs: mk_nat(0),
            }],
        ));
        let result = elaborate_expr(&mut ctx, &match_expr).expect("elaboration should succeed");
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_elaborate_match_no_arms_fails() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        ctx.push_local(Name::str("n"), Expr::Const(Name::str("Nat"), vec![]), None);
        let match_expr = mk_located(SurfaceExpr::Match(Box::new(mk_var("n")), vec![]));
        let result = elaborate_expr(&mut ctx, &match_expr);
        assert!(result.is_err());
    }
    #[test]
    fn test_elaborate_named_arg() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        ctx.push_local(Name::str("f"), Expr::Sort(Level::succ(Level::zero())), None);
        let named = mk_located(SurfaceExpr::NamedArg(
            Box::new(mk_var("f")),
            "x".to_string(),
            Box::new(mk_nat(42)),
        ));
        let result = elaborate_expr(&mut ctx, &named).expect("elaboration should succeed");
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_elab_error_display_name_not_found() {
        let err = ElabError::NameNotFound("foo".to_string());
        let s = format!("{}", err);
        assert!(s.contains("foo"));
    }
    #[test]
    fn test_elab_error_display_implicit_arg_failed() {
        let err = ElabError::ImplicitArgFailed("cannot resolve".to_string());
        let s = format!("{}", err);
        assert!(s.contains("implicit"));
    }
    #[test]
    fn test_elab_error_display_overload() {
        let err = ElabError::OverloadAmbiguity("multiple candidates".to_string());
        let s = format!("{}", err);
        assert!(s.contains("overload"));
    }
    #[test]
    fn test_elab_error_display_coercion() {
        let err = ElabError::CoercionFailed("no coercion".to_string());
        let s = format!("{}", err);
        assert!(s.contains("coercion"));
    }
    #[test]
    fn test_elab_error_display_tactic() {
        let err = ElabError::TacticFailed("tactic failed".to_string());
        let s = format!("{}", err);
        assert!(s.contains("tactic"));
    }
    #[test]
    fn test_resolve_overload_empty() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let result = resolve_overload(&mut ctx, &[], &[]);
        assert!(result.is_err());
    }
    #[test]
    fn test_resolve_overload_single() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let candidates = vec![Name::str("Nat.add")];
        let result = resolve_overload(&mut ctx, &candidates, &[]);
        assert!(result.is_ok());
        assert_eq!(
            result.expect("test operation should succeed"),
            Expr::Const(Name::str("Nat.add"), vec![])
        );
    }
    #[test]
    fn test_expected_type_hole() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let expected = Expr::Const(Name::str("Nat"), vec![]);
        let hole = mk_located(SurfaceExpr::Hole);
        let result = elaborate_with_expected_type(&mut ctx, &hole, &expected)
            .expect("elaboration should succeed");
        assert!(matches!(result, Expr::FVar(_)));
    }
    #[test]
    fn test_expected_type_let_propagation() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let expected = Expr::Const(Name::str("Nat"), vec![]);
        let let_expr = mk_located(SurfaceExpr::Let(
            "x".to_string(),
            None,
            Box::new(mk_nat(1)),
            Box::new(mk_var("x")),
        ));
        let result = elaborate_with_expected_type(&mut ctx, &let_expr, &expected)
            .expect("elaboration should succeed");
        assert!(matches!(result, Expr::Let(_, _, _, _)));
    }
}
#[cfg(test)]
mod elaborate_ext_tests {
    use super::*;
    use crate::elaborate::*;
    #[test]
    fn test_elab_run_metrics_record_decl() {
        let mut m = ElabRunMetrics::new();
        m.record_decl(true);
        m.record_decl(false);
        assert_eq!(m.decls_elaborated, 2);
        assert_eq!(m.decls_failed, 1);
        assert!((m.decl_success_rate() - 0.5).abs() < 1e-10);
    }
    #[test]
    fn test_elab_run_metrics_merge() {
        let mut a = ElabRunMetrics::new();
        a.record_decl(true);
        let mut b = ElabRunMetrics::new();
        b.record_decl(false);
        a.merge(&b);
        assert_eq!(a.decls_elaborated, 2);
        assert_eq!(a.decls_failed, 1);
    }
    #[test]
    fn test_elab_run_metrics_metavar_rate() {
        let mut m = ElabRunMetrics::new();
        m.metavars_created = 10;
        m.metavars_solved = 8;
        assert!((m.metavar_solve_rate() - 0.8).abs() < 1e-10);
    }
    #[test]
    fn test_elab_run_metrics_summary() {
        let mut m = ElabRunMetrics::new();
        m.record_decl(true);
        let s = m.summary();
        assert!(s.contains("decls=1"));
    }
    #[test]
    fn test_elab_run_metrics_empty_rate() {
        let m = ElabRunMetrics::new();
        assert!((m.decl_success_rate() - 1.0).abs() < 1e-10);
        assert!((m.metavar_solve_rate() - 1.0).abs() < 1e-10);
    }
}
