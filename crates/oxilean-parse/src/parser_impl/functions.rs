//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast_impl::{
        BinderKind, Decl, DoAction, Literal, Located, Pattern, SortKind, SurfaceExpr,
    };
    use crate::error_impl::ParseError;
    use crate::lexer::Lexer;
    use crate::parser_impl::*;
    fn parse_expr_from_str(s: &str) -> Result<Located<SurfaceExpr>, ParseError> {
        let mut lexer = Lexer::new(s);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        parser.parse_expr()
    }
    fn parse_decl_from_str(s: &str) -> Result<Located<Decl>, ParseError> {
        let mut lexer = Lexer::new(s);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        parser.parse_decl()
    }
    #[test]
    fn test_parse_var() {
        let expr = parse_expr_from_str("x").expect("parse_expr should succeed");
        assert!(matches!(expr.value, SurfaceExpr::Var(_)));
    }
    #[test]
    fn test_parse_nat() {
        let expr = parse_expr_from_str("42").expect("parse_expr should succeed");
        assert_eq!(expr.value, SurfaceExpr::Lit(Literal::Nat(42)));
    }
    #[test]
    fn test_parse_type() {
        let expr = parse_expr_from_str("Type").expect("parse_expr should succeed");
        assert_eq!(expr.value, SurfaceExpr::Sort(SortKind::Type));
    }
    #[test]
    fn test_parse_app() {
        let expr = parse_expr_from_str("f x").expect("parse_expr should succeed");
        assert!(matches!(expr.value, SurfaceExpr::App(_, _)));
    }
    #[test]
    fn test_parse_arrow() {
        let expr = parse_expr_from_str("A -> B").expect("parse_expr should succeed");
        assert!(matches!(expr.value, SurfaceExpr::Pi(_, _)));
    }
    #[test]
    fn test_parse_lambda() {
        let expr = parse_expr_from_str("fun (x : Nat) -> x").expect("parse_expr should succeed");
        assert!(matches!(expr.value, SurfaceExpr::Lam(_, _)));
    }
    #[test]
    fn test_parse_paren() {
        let expr = parse_expr_from_str("(x)").expect("parse_expr should succeed");
        assert!(matches!(expr.value, SurfaceExpr::Var(_)));
    }
    #[test]
    fn test_parse_hole() {
        let expr = parse_expr_from_str("_").expect("parse_expr should succeed");
        assert_eq!(expr.value, SurfaceExpr::Hole);
    }
    #[test]
    fn test_parse_if_then_else() {
        let expr = parse_expr_from_str("if x then y else z").expect("parse_expr should succeed");
        assert!(matches!(expr.value, SurfaceExpr::If(_, _, _)));
    }
    #[test]
    fn test_parse_if_nested() {
        let expr = parse_expr_from_str("if a then if b then c else d else e")
            .expect("parse_expr should succeed");
        assert!(matches!(expr.value, SurfaceExpr::If(_, _, _)));
    }
    #[test]
    fn test_parse_match() {
        let expr = parse_expr_from_str("match x with | y -> z").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Match(_, arms) => {
                assert_eq!(arms.len(), 1);
            }
            other => panic!("Expected Match, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_match_multi_arm() {
        let expr = parse_expr_from_str("match x with | 0 -> a | n -> b")
            .expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Match(_, arms) => {
                assert_eq!(arms.len(), 2);
            }
            other => panic!("Expected Match, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_match_wildcard() {
        let expr = parse_expr_from_str("match x with | _ -> y").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Match(_, arms) => {
                assert!(matches!(arms[0].pattern.value, Pattern::Wild));
            }
            other => panic!("Expected Match, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_match_ctor_pattern() {
        let expr =
            parse_expr_from_str("match x with | Cons h t -> h").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Match(_, arms) => {
                assert!(matches!(arms[0].pattern.value, Pattern::Ctor(_, _)));
            }
            other => panic!("Expected Match, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_match_with_guard() {
        let expr = parse_expr_from_str("match x with | n if cond -> body")
            .expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Match(_, arms) => {
                assert_eq!(arms.len(), 1);
                assert!(arms[0].guard.is_some(), "guard should be Some");
                if let Some(guard) = &arms[0].guard {
                    assert!(matches!(guard.value, SurfaceExpr::Var(_)));
                }
            }
            other => panic!("Expected Match, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_match_no_guard() {
        let expr =
            parse_expr_from_str("match x with | n -> body").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Match(_, arms) => {
                assert_eq!(arms.len(), 1);
                assert!(arms[0].guard.is_none(), "guard should be None");
            }
            other => panic!("Expected Match, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_do() {
        let expr = parse_expr_from_str("do { x }").expect("parse_expr should succeed");
        assert!(matches!(expr.value, SurfaceExpr::Do(_)));
    }
    #[test]
    fn test_parse_do_multi_action() {
        let expr = parse_expr_from_str("do { let x := 1; x }").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Do(actions) => {
                assert_eq!(actions.len(), 2);
                assert!(matches!(actions[0], DoAction::Let(_, _)));
                assert!(matches!(actions[1], DoAction::Expr(_)));
            }
            other => panic!("Expected Do, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_do_bind() {
        let expr =
            parse_expr_from_str("do { x <- getLine; x }").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Do(actions) => {
                assert_eq!(actions.len(), 2);
                assert!(matches!(actions[0], DoAction::Bind(_, _)));
            }
            other => panic!("Expected Do, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_have() {
        let expr = parse_expr_from_str("have h : Nat := 42; h").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Have(name, _, _, _) => {
                assert_eq!(name, "h");
            }
            other => panic!("Expected Have, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_suffices() {
        let expr =
            parse_expr_from_str("suffices h : Nat by auto").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Suffices(name, _, _) => {
                assert_eq!(name, "h");
            }
            other => panic!("Expected Suffices, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_show() {
        let expr = parse_expr_from_str("show Nat from 42").expect("parse_expr should succeed");
        assert!(matches!(expr.value, SurfaceExpr::Show(_, _)));
    }
    #[test]
    fn test_parse_tuple() {
        let expr = parse_expr_from_str("(1, 2)").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Tuple(elems) => {
                assert_eq!(elems.len(), 2);
            }
            other => panic!("Expected Tuple, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_tuple_triple() {
        let expr = parse_expr_from_str("(1, 2, 3)").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Tuple(elems) => {
                assert_eq!(elems.len(), 3);
            }
            other => panic!("Expected Tuple, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_list_empty() {
        let expr = parse_expr_from_str("[]").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::ListLit(elems) => {
                assert!(elems.is_empty());
            }
            other => panic!("Expected ListLit, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_list() {
        let expr = parse_expr_from_str("[1, 2, 3]").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::ListLit(elems) => {
                assert_eq!(elems.len(), 3);
            }
            other => panic!("Expected ListLit, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_type_annotation() {
        let expr = parse_expr_from_str("(x : Nat)").expect("parse_expr should succeed");
        assert!(matches!(expr.value, SurfaceExpr::Ann(_, _)));
    }
    #[test]
    fn test_parse_proj() {
        let expr = parse_expr_from_str("x.foo").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Proj(_, field) => {
                assert_eq!(field, "foo");
            }
            other => panic!("Expected Proj, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_proj_chain() {
        let expr = parse_expr_from_str("x.foo.bar").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Proj(inner, field) => {
                assert_eq!(field, "bar");
                assert!(matches!(inner.value, SurfaceExpr::Proj(_, _)));
            }
            other => panic!("Expected Proj, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_plus() {
        let expr = parse_expr_from_str("a + b").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::App(lhs, rhs) => {
                assert!(matches!(rhs.value, SurfaceExpr::Var(_)));
                match &lhs.value {
                    SurfaceExpr::App(op, _lhs_inner) => {
                        assert!(matches!(& op.value, SurfaceExpr::Var(n) if n == "+"));
                    }
                    other => panic!("Expected App, got {:?}", other),
                }
            }
            other => panic!("Expected App, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_binop_precedence() {
        let expr = parse_expr_from_str("a + b * c").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::App(lhs, _rhs) => match &lhs.value {
                SurfaceExpr::App(op, _) => {
                    assert!(matches!(& op.value, SurfaceExpr::Var(n) if n == "+"));
                }
                other => panic!("Expected App(Var(+), ..), got {:?}", other),
            },
            other => panic!("Expected App, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_comparison() {
        let expr = parse_expr_from_str("a < b").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::App(lhs, _) => match &lhs.value {
                SurfaceExpr::App(op, _) => {
                    assert!(matches!(& op.value, SurfaceExpr::Var(n) if n == "Lt"));
                }
                other => panic!("Expected App, got {:?}", other),
            },
            other => panic!("Expected App, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_and_or() {
        let expr = parse_expr_from_str("a && b").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::App(lhs, _) => match &lhs.value {
                SurfaceExpr::App(op, _) => {
                    assert!(matches!(& op.value, SurfaceExpr::Var(n) if n == "&&"));
                }
                other => panic!("Expected App, got {:?}", other),
            },
            other => panic!("Expected App, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_not() {
        let expr = parse_expr_from_str("!x").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::App(func, _) => {
                assert!(matches!(& func.value, SurfaceExpr::Var(n) if n == "Not"));
            }
            other => panic!("Expected App(Not, x), got {:?}", other),
        }
    }
    #[test]
    fn test_parse_string_lit() {
        let expr = parse_expr_from_str(r#""hello""#).expect("parse_expr should succeed");
        assert_eq!(
            expr.value,
            SurfaceExpr::Lit(Literal::String("hello".to_string()))
        );
    }
    #[test]
    fn test_parse_question_mark_hole() {
        let expr = parse_expr_from_str("?").expect("parse_expr should succeed");
        assert_eq!(expr.value, SurfaceExpr::Hole);
    }
    #[test]
    fn test_parse_prop() {
        let expr = parse_expr_from_str("Prop").expect("parse_expr should succeed");
        assert_eq!(expr.value, SurfaceExpr::Sort(SortKind::Prop));
    }
    #[test]
    fn test_parse_forall() {
        let expr = parse_expr_from_str("forall (x : Nat), x").expect("parse_expr should succeed");
        assert!(matches!(expr.value, SurfaceExpr::Pi(_, _)));
    }
    #[test]
    fn test_parse_let() {
        let expr = parse_expr_from_str("let x := 1 in x").expect("parse_expr should succeed");
        assert!(matches!(expr.value, SurfaceExpr::Let(_, _, _, _)));
    }
    #[test]
    fn test_parse_let_typed() {
        let expr = parse_expr_from_str("let x : Nat := 1 in x").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Let(name, ty, _, _) => {
                assert_eq!(name, "x");
                assert!(ty.is_some());
            }
            other => panic!("Expected Let, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_implicit_binder() {
        let expr = parse_expr_from_str("fun {x : Nat} -> x").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Lam(binders, _) => {
                assert_eq!(binders.len(), 1);
                assert_eq!(binders[0].info, BinderKind::Implicit);
            }
            other => panic!("Expected Lam, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_instance_binder() {
        let expr = parse_expr_from_str("fun [x : Monad] -> x").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Lam(binders, _) => {
                assert_eq!(binders.len(), 1);
                assert_eq!(binders[0].info, BinderKind::Instance);
            }
            other => panic!("Expected Lam, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_strict_implicit_binder() {
        let expr = parse_expr_from_str("fun {{x : Nat}} -> x").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Lam(binders, _) => {
                assert_eq!(binders.len(), 1);
                assert_eq!(binders[0].info, BinderKind::StrictImplicit);
            }
            other => panic!("Expected Lam, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_mixed_binders() {
        let expr = parse_expr_from_str("fun (a : Nat) {b : Nat} [c : Monad] -> a")
            .expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Lam(binders, _) => {
                assert_eq!(binders.len(), 3);
                assert_eq!(binders[0].info, BinderKind::Default);
                assert_eq!(binders[1].info, BinderKind::Implicit);
                assert_eq!(binders[2].info, BinderKind::Instance);
            }
            other => panic!("Expected Lam, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_axiom_decl() {
        let decl = parse_decl_from_str("axiom A : Prop").expect("parse_decl should succeed");
        match &decl.value {
            Decl::Axiom { name, .. } => assert_eq!(name, "A"),
            other => panic!("Expected Axiom, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_def_decl() {
        let decl = parse_decl_from_str("def x := 42").expect("parse_decl should succeed");
        match &decl.value {
            Decl::Definition { name, ty, .. } => {
                assert_eq!(name, "x");
                assert!(ty.is_none());
            }
            other => panic!("Expected Definition, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_def_typed() {
        let decl = parse_decl_from_str("def x : Nat := 42").expect("parse_decl should succeed");
        match &decl.value {
            Decl::Definition { name, ty, .. } => {
                assert_eq!(name, "x");
                assert!(ty.is_some());
            }
            other => panic!("Expected Definition, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_theorem_decl() {
        let decl = parse_decl_from_str("theorem t : Prop := x").expect("parse_decl should succeed");
        match &decl.value {
            Decl::Theorem { name, .. } => assert_eq!(name, "t"),
            other => panic!("Expected Theorem, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_lemma_decl() {
        let decl = parse_decl_from_str("lemma l : Prop := x").expect("parse_decl should succeed");
        match &decl.value {
            Decl::Theorem { name, .. } => assert_eq!(name, "l"),
            other => panic!("Expected Theorem, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_import_decl() {
        let decl = parse_decl_from_str("import Foo.Bar").expect("parse_decl should succeed");
        match &decl.value {
            Decl::Import { path } => {
                assert_eq!(path, &["Foo", "Bar"]);
            }
            other => panic!("Expected Import, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_structure_decl() {
        let decl = parse_decl_from_str("structure Point where x : Nat y : Nat")
            .expect("parse_decl should succeed");
        match &decl.value {
            Decl::Structure { name, fields, .. } => {
                assert_eq!(name, "Point");
                assert_eq!(fields.len(), 2);
                assert_eq!(fields[0].name, "x");
                assert_eq!(fields[1].name, "y");
            }
            other => panic!("Expected Structure, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_class_decl() {
        let decl =
            parse_decl_from_str("class Monoid where op : Nat").expect("parse_decl should succeed");
        match &decl.value {
            Decl::ClassDecl { name, fields, .. } => {
                assert_eq!(name, "Monoid");
                assert_eq!(fields.len(), 1);
            }
            other => panic!("Expected ClassDecl, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_instance_decl() {
        let decl = parse_decl_from_str("instance : Monoid Nat where op := 42")
            .expect("parse_decl should succeed");
        match &decl.value {
            Decl::InstanceDecl {
                name,
                class_name,
                defs,
                ..
            } => {
                assert!(name.is_none());
                assert_eq!(class_name, "Monoid");
                assert_eq!(defs.len(), 1);
                assert_eq!(defs[0].0, "op");
            }
            other => panic!("Expected InstanceDecl, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_instance_named() {
        let decl = parse_decl_from_str("instance myInst : Monoid Nat where op := 42")
            .expect("parse_decl should succeed");
        match &decl.value {
            Decl::InstanceDecl { name, .. } => {
                assert_eq!(name.as_deref(), Some("myInst"));
            }
            other => panic!("Expected InstanceDecl, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_section_decl() {
        let decl = parse_decl_from_str("section Foo axiom a : Prop end Foo")
            .expect("parse_decl should succeed");
        match &decl.value {
            Decl::SectionDecl { name, decls } => {
                assert_eq!(name, "Foo");
                assert_eq!(decls.len(), 1);
            }
            other => panic!("Expected SectionDecl, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_variable_decl() {
        let decl = parse_decl_from_str("variable (n : Nat)").expect("parse_decl should succeed");
        match &decl.value {
            Decl::Variable { binders } => {
                assert_eq!(binders.len(), 1);
                assert_eq!(binders[0].name, "n");
            }
            other => panic!("Expected Variable, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_variable_implicit() {
        let decl = parse_decl_from_str("variable {a : Type}").expect("parse_decl should succeed");
        match &decl.value {
            Decl::Variable { binders } => {
                assert_eq!(binders.len(), 1);
                assert_eq!(binders[0].info, BinderKind::Implicit);
            }
            other => panic!("Expected Variable, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_open_decl() {
        let decl = parse_decl_from_str("open Foo.Bar").expect("parse_decl should succeed");
        match &decl.value {
            Decl::Open { path, names } => {
                assert_eq!(path, &["Foo", "Bar"]);
                assert!(names.is_empty());
            }
            other => panic!("Expected Open, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_open_with_names() {
        let decl = parse_decl_from_str("open Foo (bar baz)").expect("parse_decl should succeed");
        match &decl.value {
            Decl::Open { path, names } => {
                assert_eq!(path, &["Foo"]);
                assert_eq!(names, &["bar", "baz"]);
            }
            other => panic!("Expected Open, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_attribute_decl() {
        let decl =
            parse_decl_from_str("@[simp] axiom a : Prop").expect("parse_decl should succeed");
        match &decl.value {
            Decl::Attribute { attrs, decl } => {
                assert_eq!(attrs, &["simp"]);
                assert!(matches!(decl.value, Decl::Axiom { .. }));
            }
            other => panic!("Expected Attribute, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_attribute_multi() {
        let decl = parse_decl_from_str("@[simp, ext] theorem t : Prop := x")
            .expect("parse_decl should succeed");
        match &decl.value {
            Decl::Attribute { attrs, .. } => {
                assert_eq!(attrs, &["simp", "ext"]);
            }
            other => panic!("Expected Attribute, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_hash_check() {
        let decl = parse_decl_from_str("#check Nat").expect("parse_decl should succeed");
        match &decl.value {
            Decl::HashCmd { cmd, .. } => {
                assert_eq!(cmd, "check");
            }
            other => panic!("Expected HashCmd, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_hash_eval() {
        let decl = parse_decl_from_str("#eval 42").expect("parse_decl should succeed");
        match &decl.value {
            Decl::HashCmd { cmd, arg } => {
                assert_eq!(cmd, "eval");
                assert_eq!(arg.value, SurfaceExpr::Lit(Literal::Nat(42)));
            }
            other => panic!("Expected HashCmd, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_hash_print() {
        let decl = parse_decl_from_str("#print Nat").expect("parse_decl should succeed");
        match &decl.value {
            Decl::HashCmd { cmd, .. } => {
                assert_eq!(cmd, "print");
            }
            other => panic!("Expected HashCmd, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_namespace_decl() {
        let decl = parse_decl_from_str("namespace Foo axiom a : Prop end Foo")
            .expect("parse_decl should succeed");
        match &decl.value {
            Decl::Namespace { name, decls } => {
                assert_eq!(name, "Foo");
                assert_eq!(decls.len(), 1);
            }
            other => panic!("Expected Namespace, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_inductive_decl() {
        let decl = parse_decl_from_str("inductive Bool : Type | true : Bool | false : Bool")
            .expect("parse_decl should succeed");
        match &decl.value {
            Decl::Inductive { name, ctors, .. } => {
                assert_eq!(name, "Bool");
                assert_eq!(ctors.len(), 2);
            }
            other => panic!("Expected Inductive, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_arrow_chain() {
        let expr = parse_expr_from_str("A -> B -> C").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Pi(_, body) => {
                assert!(matches!(body.value, SurfaceExpr::Pi(_, _)));
            }
            other => panic!("Expected Pi, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_app_chain() {
        let expr = parse_expr_from_str("f x y").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::App(lhs, _) => {
                assert!(matches!(lhs.value, SurfaceExpr::App(_, _)));
            }
            other => panic!("Expected App, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_minus_prefix() {
        let expr = parse_expr_from_str("(- x)").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::App(func, _) => {
                assert!(matches!(& func.value, SurfaceExpr::Var(n) if n == "Neg"));
            }
            other => panic!("Expected App(Neg, x), got {:?}", other),
        }
    }
    #[test]
    fn test_parse_sort_with_universe() {
        let expr = parse_expr_from_str("Type u").expect("parse_expr should succeed");
        assert_eq!(
            expr.value,
            SurfaceExpr::Sort(SortKind::TypeU("u".to_string()))
        );
    }
    #[test]
    fn test_parse_empty_tuple() {
        let expr = parse_expr_from_str("()").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Tuple(elems) => assert!(elems.is_empty()),
            other => panic!("Expected empty Tuple, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_do_let_typed() {
        let expr =
            parse_expr_from_str("do { let x : Nat := 1; x }").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Do(actions) => {
                assert_eq!(actions.len(), 2);
                assert!(matches!(actions[0], DoAction::LetTyped(_, _, _)));
            }
            other => panic!("Expected Do, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_caret_right_assoc() {
        let expr = parse_expr_from_str("a ^ b ^ c").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::App(lhs, _rhs) => match &lhs.value {
                SurfaceExpr::App(op, _) => {
                    assert!(matches!(& op.value, SurfaceExpr::Var(n) if n == "^"));
                }
                other => panic!("Expected App(^, ...), got {:?}", other),
            },
            other => panic!("Expected App, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_multi_binder_group() {
        let expr =
            parse_expr_from_str("fun (a : Nat) (b : Nat) -> a").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Lam(binders, _) => {
                assert_eq!(binders.len(), 2);
                assert_eq!(binders[0].name, "a");
                assert_eq!(binders[1].name, "b");
            }
            other => panic!("Expected Lam, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_structure_extends() {
        let decl = parse_decl_from_str("structure ColorPoint extends Point where color : Nat")
            .expect("parse_decl should succeed");
        match &decl.value {
            Decl::Structure { name, extends, .. } => {
                assert_eq!(name, "ColorPoint");
                assert_eq!(extends, &["Point"]);
            }
            other => panic!("Expected Structure, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_univ_params() {
        let decl = parse_decl_from_str("axiom A {u, v} : Prop").expect("parse_decl should succeed");
        match &decl.value {
            Decl::Axiom { univ_params, .. } => {
                assert_eq!(univ_params, &["u", "v"]);
            }
            other => panic!("Expected Axiom, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_eq_binop() {
        let expr = parse_expr_from_str("a = b").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::App(lhs, _) => match &lhs.value {
                SurfaceExpr::App(op, _) => {
                    assert!(matches!(& op.value, SurfaceExpr::Var(n) if n == "Eq"));
                }
                other => panic!("Expected App(Eq, ..), got {:?}", other),
            },
            other => panic!("Expected App, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_percent_binop() {
        let expr = parse_expr_from_str("a % b").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::App(lhs, _) => match &lhs.value {
                SurfaceExpr::App(op, _) => {
                    assert!(matches!(& op.value, SurfaceExpr::Var(n) if n == "%"));
                }
                other => panic!("Expected App, got {:?}", other),
            },
            other => panic!("Expected App, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_list_single() {
        let expr = parse_expr_from_str("[42]").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::ListLit(elems) => {
                assert_eq!(elems.len(), 1);
                assert_eq!(elems[0].value, SurfaceExpr::Lit(Literal::Nat(42)));
            }
            other => panic!("Expected ListLit, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_or_binop() {
        let expr = parse_expr_from_str("a || b").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::App(lhs, _) => match &lhs.value {
                SurfaceExpr::App(op, _) => {
                    assert!(matches!(& op.value, SurfaceExpr::Var(n) if n == "||"));
                }
                other => panic!("Expected App, got {:?}", other),
            },
            other => panic!("Expected App, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_variable_instance_binder() {
        let decl = parse_decl_from_str("variable [m : Monad]").expect("parse_decl should succeed");
        match &decl.value {
            Decl::Variable { binders } => {
                assert_eq!(binders.len(), 1);
                assert_eq!(binders[0].info, BinderKind::Instance);
                assert_eq!(binders[0].name, "m");
            }
            other => panic!("Expected Variable, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_do_without_braces() {
        let expr = parse_expr_from_str("do x <- getLine; x").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Do(actions) => {
                assert_eq!(actions.len(), 2);
                assert!(matches!(actions[0], DoAction::Bind(_, _)));
            }
            other => panic!("Expected Do, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_structure_with_default() {
        let decl = parse_decl_from_str("structure Config where verbose : Nat := 0")
            .expect("parse_decl should succeed");
        match &decl.value {
            Decl::Structure { name, fields, .. } => {
                assert_eq!(name, "Config");
                assert_eq!(fields.len(), 1);
                assert!(fields[0].default.is_some());
            }
            other => panic!("Expected Structure, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_anonymous_ctor_single() {
        let expr = parse_expr_from_str("⟨42⟩").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::AnonymousCtor(elems) => {
                assert_eq!(elems.len(), 1);
            }
            other => panic!("Expected AnonymousCtor, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_anonymous_ctor_pair() {
        let expr = parse_expr_from_str("⟨1, 2⟩").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::AnonymousCtor(elems) => {
                assert_eq!(elems.len(), 2);
            }
            other => panic!("Expected AnonymousCtor, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_anonymous_ctor_triple() {
        let expr = parse_expr_from_str("⟨1, 2, 3⟩").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::AnonymousCtor(elems) => {
                assert_eq!(elems.len(), 3);
            }
            other => panic!("Expected AnonymousCtor, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_anonymous_ctor_nested() {
        let expr = parse_expr_from_str("⟨⟨1, 2⟩, 3⟩").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::AnonymousCtor(elems) => {
                assert_eq!(elems.len(), 2);
                assert!(matches!(elems[0].value, SurfaceExpr::AnonymousCtor(_)));
            }
            other => panic!("Expected AnonymousCtor, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_anonymous_ctor_empty() {
        let expr = parse_expr_from_str("⟨⟩").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::AnonymousCtor(elems) => {
                assert!(elems.is_empty());
            }
            other => panic!("Expected empty AnonymousCtor, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_named_arg_single() {
        let expr = parse_expr_from_str("f (x := 1)").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::App(_, arg) => {
                assert!(matches!(arg.value, SurfaceExpr::NamedArg(_, _, _)));
            }
            other => panic!("Expected App with NamedArg, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_named_arg_extraction() {
        let expr = parse_expr_from_str("f (x := 1)").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::App(_, arg) => match &arg.value {
                SurfaceExpr::NamedArg(_, name, val) => {
                    assert_eq!(name, "x");
                    assert_eq!(val.value, SurfaceExpr::Lit(Literal::Nat(1)));
                }
                other => panic!("Expected NamedArg, got {:?}", other),
            },
            other => panic!("Expected App, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_named_arg_complex_value() {
        let expr = parse_expr_from_str("f (name := x + y)").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::App(_, arg) => {
                assert!(matches!(arg.value, SurfaceExpr::NamedArg(_, _, _)));
            }
            other => panic!("Expected App with NamedArg, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_do_let_bind_mixed() {
        let expr = parse_expr_from_str("do { let x := 1; y <- f x; y }")
            .expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Do(actions) => {
                assert_eq!(actions.len(), 3);
                assert!(matches!(actions[0], DoAction::Let(_, _)));
                assert!(matches!(actions[1], DoAction::Bind(_, _)));
                assert!(matches!(actions[2], DoAction::Expr(_)));
            }
            other => panic!("Expected Do, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_do_action_expr() {
        let expr = parse_expr_from_str("do { x; y; z }").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Do(actions) => {
                assert_eq!(actions.len(), 3);
                for action in actions {
                    assert!(matches!(action, DoAction::Expr(_)));
                }
            }
            other => panic!("Expected Do, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_do_bind_simple() {
        let expr = parse_expr_from_str("do { x <- getLine; putLine x }")
            .expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Do(actions) => {
                assert_eq!(actions.len(), 2);
                match &actions[0] {
                    DoAction::Bind(name, _) => {
                        assert_eq!(name, "x");
                    }
                    other => panic!("Expected Bind, got {:?}", other),
                }
            }
            other => panic!("Expected Do, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_have_simple() {
        let expr =
            parse_expr_from_str("have h : True := trivial; h").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Have(name, ty, proof, body) => {
                assert_eq!(name, "h");
                let _ = (ty, proof, body);
            }
            other => panic!("Expected Have, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_have_body_chain() {
        let expr = parse_expr_from_str("have h1 : True := trivial; have h2 : True := h1; h2")
            .expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Have(_, _, _, body) => {
                assert!(matches!(body.value, SurfaceExpr::Have(_, _, _, _)));
            }
            other => panic!("Expected nested Have, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_suffices_with_tactic() {
        let expr =
            parse_expr_from_str("suffices h : Nat by simp").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Suffices(name, ty, tactic) => {
                assert_eq!(name, "h");
                let _ = (ty, tactic);
            }
            other => panic!("Expected Suffices, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_suffices_complex_type() {
        let expr = parse_expr_from_str("suffices h : forall x, x = x by simp")
            .expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Suffices(name, _, _) => {
                assert_eq!(name, "h");
            }
            other => panic!("Expected Suffices, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_show_simple() {
        let expr = parse_expr_from_str("show Nat from 42").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Show(ty, proof) => {
                let _ = (ty, proof);
            }
            other => panic!("Expected Show, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_show_complex_type() {
        let expr = parse_expr_from_str("show (forall x, x = x) from (fun x -> rfl)")
            .expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Show(_, _) => {}
            other => panic!("Expected Show, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_show_nested() {
        let expr = parse_expr_from_str("show Nat from (show Nat from 0)")
            .expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Show(_, proof) => {
                assert!(matches!(proof.value, SurfaceExpr::Show(_, _)));
            }
            other => panic!("Expected nested Show, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_list_nested() {
        let expr = parse_expr_from_str("[[1, 2], [3, 4]]").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::ListLit(elems) => {
                assert_eq!(elems.len(), 2);
                for elem in elems {
                    assert!(matches!(elem.value, SurfaceExpr::ListLit(_)));
                }
            }
            other => panic!("Expected nested ListLit, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_list_mixed_types() {
        let expr = parse_expr_from_str("[1, x, y + z]").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::ListLit(elems) => {
                assert_eq!(elems.len(), 3);
            }
            other => panic!("Expected ListLit, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_tuple_single() {
        let expr = parse_expr_from_str("(1, 1)").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Tuple(elems) => {
                assert_eq!(elems.len(), 2);
            }
            other => panic!("Expected Tuple with 2 elements, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_tuple_nested() {
        let expr = parse_expr_from_str("((1, 2), (3, 4))").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Tuple(elems) => {
                assert_eq!(elems.len(), 2);
                for elem in elems {
                    assert!(matches!(elem.value, SurfaceExpr::Tuple(_)));
                }
            }
            other => panic!("Expected nested Tuple, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_tuple_large() {
        let expr = parse_expr_from_str("(1, 2, 3, 4, 5)").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Tuple(elems) => {
                assert_eq!(elems.len(), 5);
            }
            other => panic!("Expected Tuple with 5 elements, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_proj_multi_level() {
        let expr = parse_expr_from_str("x.a.b.c").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Proj(inner, field) => {
                assert_eq!(field, "c");
                assert!(matches!(inner.value, SurfaceExpr::Proj(_, _)));
            }
            other => panic!("Expected Proj, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_proj_on_app() {
        let expr = parse_expr_from_str("(f x).field").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Proj(inner, field) => {
                assert_eq!(field, "field");
                assert!(matches!(inner.value, SurfaceExpr::App(_, _)));
            }
            other => panic!("Expected Proj, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_proj_on_tuple() {
        let expr = parse_expr_from_str("(1, 2, 3).foo").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Proj(inner, field) => {
                assert_eq!(field, "foo");
                assert!(matches!(inner.value, SurfaceExpr::Tuple(_)));
            }
            other => panic!("Expected Proj on Tuple, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_mixed_precedence() {
        let expr = parse_expr_from_str("a + b * c - d").expect("parse_expr should succeed");
        assert!(matches!(expr.value, SurfaceExpr::App(_, _)));
    }
    #[test]
    fn test_parse_comparison_chain() {
        let expr = parse_expr_from_str("a < b").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::App(lhs, _) => {
                assert!(matches!(lhs.value, SurfaceExpr::App(_, _)));
            }
            other => panic!("Expected App, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_iff_operator() {
        let expr = parse_expr_from_str("Iff a b").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::App(lhs, _) => match &lhs.value {
                SurfaceExpr::App(_, _) => {}
                other => panic!("Expected App, got {:?}", other),
            },
            other => panic!("Expected App, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_logical_and() {
        let expr = parse_expr_from_str("a && b").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::App(lhs, _) => match &lhs.value {
                SurfaceExpr::App(op, _) => {
                    assert!(matches!(& op.value, SurfaceExpr::Var(n) if n == "&&"));
                }
                other => panic!("Expected App(&&, ...), got {:?}", other),
            },
            other => panic!("Expected App, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_ann_in_app() {
        let expr = parse_expr_from_str("f (x : Nat)").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::App(_, arg) => {
                assert!(matches!(arg.value, SurfaceExpr::Ann(_, _)));
            }
            other => panic!("Expected App with Ann, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_ann_complex_type() {
        let expr =
            parse_expr_from_str("(x : forall a, a -> a)").expect("parse_expr should succeed");
        assert!(matches!(expr.value, SurfaceExpr::Ann(_, _)));
    }
    #[test]
    fn test_parse_if_associativity() {
        let expr = parse_expr_from_str("if a then if b then c else d else e")
            .expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::If(_, then_branch, _) => {
                assert!(matches!(then_branch.value, SurfaceExpr::If(_, _, _)));
            }
            other => panic!("Expected If with nested If in then branch, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_if_complex_condition() {
        let expr =
            parse_expr_from_str("if x && y || z then a else b").expect("parse_expr should succeed");
        assert!(matches!(expr.value, SurfaceExpr::If(_, _, _)));
    }
    #[test]
    fn test_parse_match_multiple_arms() {
        let expr = parse_expr_from_str("match x with | 0 -> a | 1 -> b | _ -> c")
            .expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Match(_, arms) => {
                assert_eq!(arms.len(), 3);
            }
            other => panic!("Expected Match with 3 arms, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_match_constructor_patterns() {
        let expr = parse_expr_from_str("match x with | nil -> a | cons h t -> h")
            .expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Match(_, arms) => {
                assert_eq!(arms.len(), 2);
                assert!(matches!(arms[0].pattern.value, Pattern::Var(_)));
                assert!(matches!(arms[1].pattern.value, Pattern::Ctor(_, _)));
            }
            other => panic!("Expected Match, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_match_with_literals() {
        let expr = parse_expr_from_str("match x with | \"hello\" -> a | \"world\" -> b")
            .expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Match(_, arms) => {
                assert_eq!(arms.len(), 2);
            }
            other => panic!("Expected Match, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_let_chain() {
        let expr = parse_expr_from_str("let x := 1 in let y := 2 in x + y")
            .expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Let(_, _, _, body) => {
                assert!(matches!(body.value, SurfaceExpr::Let(_, _, _, _)));
            }
            other => panic!("Expected chained Let, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_let_no_type() {
        let expr = parse_expr_from_str("let x := 42 in x").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Let(_, ty, _, _) => {
                assert!(ty.is_none());
            }
            other => panic!("Expected Let, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_let_with_app() {
        let expr = parse_expr_from_str("let x := f 1 2 in x").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Let(_, _, val, _) => {
                assert!(matches!(val.value, SurfaceExpr::App(_, _)));
            }
            other => panic!("Expected Let, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_lambda_multi_arg() {
        let expr = parse_expr_from_str("fun (x : Nat) (y : Nat) -> x + y")
            .expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Lam(binders, _) => {
                assert_eq!(binders.len(), 2);
            }
            other => panic!("Expected Lam with 2 binders, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_lambda_no_type() {
        let expr = parse_expr_from_str("fun x -> x").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Lam(binders, _) => {
                assert_eq!(binders.len(), 1);
                assert!(binders[0].ty.is_none());
            }
            other => panic!("Expected Lam, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_lambda_mixed_binders() {
        let expr = parse_expr_from_str("fun (x : Nat) {y} [z : Monad] -> x")
            .expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Lam(binders, _) => {
                assert_eq!(binders.len(), 3);
                assert_eq!(binders[0].info, BinderKind::Default);
                assert_eq!(binders[1].info, BinderKind::Implicit);
                assert_eq!(binders[2].info, BinderKind::Instance);
            }
            other => panic!("Expected Lam with mixed binders, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_pi_multi_arg() {
        let expr = parse_expr_from_str("forall (x : Nat) (y : Nat), x = y")
            .expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Pi(binders, _) => {
                assert_eq!(binders.len(), 2);
            }
            other => panic!("Expected Pi with 2 binders, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_pi_implicit() {
        let expr =
            parse_expr_from_str("forall {x : Nat}, x = x").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Pi(binders, _) => {
                assert_eq!(binders[0].info, BinderKind::Implicit);
            }
            other => panic!("Expected Pi with implicit, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_def_with_univ_params() {
        let decl =
            parse_decl_from_str("def id {u} : Type := Type").expect("parse_decl should succeed");
        match &decl.value {
            Decl::Definition {
                name, univ_params, ..
            } => {
                assert_eq!(name, "id");
                assert_eq!(univ_params, &["u"]);
            }
            other => panic!("Expected Definition, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_theorem_with_type() {
        let decl =
            parse_decl_from_str("theorem identity : Nat := 42").expect("parse_decl should succeed");
        match &decl.value {
            Decl::Theorem { name, ty, .. } => {
                assert_eq!(name, "identity");
                let _ = ty;
            }
            other => panic!("Expected Theorem, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_inductive_multi_ctor() {
        let decl =
            parse_decl_from_str("inductive List : Type | nil : Type | cons : Nat → List → List")
                .expect("test operation should succeed");
        match &decl.value {
            Decl::Inductive { name, ctors, .. } => {
                assert_eq!(name, "List");
                assert_eq!(ctors.len(), 2);
            }
            other => panic!("Expected Inductive, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_class_with_extends() {
        let decl = parse_decl_from_str("class Functor extends Inhabited where map : Nat")
            .expect("parse_decl should succeed");
        match &decl.value {
            Decl::ClassDecl { name, extends, .. } => {
                assert_eq!(name, "Functor");
                assert!(extends.contains(&"Inhabited".to_string()));
            }
            other => panic!("Expected ClassDecl, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_instance_with_methods() {
        let decl = parse_decl_from_str("instance : Monoid Nat where op := Nat")
            .expect("parse_decl should succeed");
        match &decl.value {
            Decl::InstanceDecl { defs, .. } => {
                assert_eq!(defs.len(), 1);
                assert_eq!(defs[0].0, "op");
            }
            other => panic!("Expected InstanceDecl, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_deeply_nested_expr() {
        let expr = parse_expr_from_str("(((((x)))))").expect("parse_expr should succeed");
        assert!(matches!(expr.value, SurfaceExpr::Var(_)));
    }
    #[test]
    fn test_parse_deeply_nested_app() {
        let expr = parse_expr_from_str("f (g (h (i (j x))))").expect("parse_expr should succeed");
        assert!(matches!(expr.value, SurfaceExpr::App(_, _)));
    }
    #[test]
    fn test_parse_complex_mixed_expr() {
        let expr =
            parse_expr_from_str("let x := 1 in match x with | 0 -> if y then z else w | n -> n")
                .expect("test operation should succeed");
        assert!(matches!(expr.value, SurfaceExpr::Let(_, _, _, _)));
    }
    #[test]
    fn test_parse_annotation_in_lambda() {
        let expr =
            parse_expr_from_str("fun (x : Nat) -> (x : Nat)").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Lam(_, body) => {
                assert!(matches!(body.value, SurfaceExpr::Ann(_, _)));
            }
            other => panic!("Expected Lam, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_app_with_annotation() {
        let expr =
            parse_expr_from_str("f (x : Nat) (y : String)").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::App(_, _) => {}
            other => panic!("Expected App, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_single_char_ident() {
        let expr = parse_expr_from_str("x").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Var(name) => {
                assert_eq!(name, "x");
            }
            other => panic!("Expected Var, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_long_ident() {
        let expr = parse_expr_from_str("veryLongIdentifierNameWithManyWords")
            .expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Var(name) => {
                assert_eq!(name, "veryLongIdentifierNameWithManyWords");
            }
            other => panic!("Expected Var, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_large_nat() {
        let expr = parse_expr_from_str("999999999999").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Lit(Literal::Nat(n)) => {
                assert_eq!(*n, 999999999999);
            }
            other => panic!("Expected Nat literal, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_binder_underscore() {
        let expr = parse_expr_from_str("fun (_ : Nat) -> 42").expect("parse_expr should succeed");
        match &expr.value {
            SurfaceExpr::Lam(binders, _) => {
                assert_eq!(binders[0].name, "_");
            }
            other => panic!("Expected Lam, got {:?}", other),
        }
    }
    #[test]
    fn test_parse_multiple_projections() {
        let expr = parse_expr_from_str("x.a.b.c.d.e").expect("parse_expr should succeed");
        let mut depth = 0;
        let mut current = &expr.value;
        while let SurfaceExpr::Proj(inner, _) = current {
            depth += 1;
            current = &inner.value;
        }
        assert_eq!(depth, 5);
    }
}
