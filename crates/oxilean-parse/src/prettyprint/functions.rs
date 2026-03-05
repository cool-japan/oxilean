//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::ast_impl::*;

use super::types::{DocNode, PrettyConfig, PrettyPrinter};

/// Precedence levels for expressions.
/// Higher precedence means tighter binding.
#[allow(dead_code)]
pub mod prec {
    /// Lambda, forall (lowest precedence)
    pub const LAMBDA: u32 = 0;
    /// Let, if expressions
    pub const LET: u32 = 1;
    /// If expressions
    pub const IF: u32 = 1;
    /// Or operator
    pub const OR: u32 = 10;
    /// And operator
    pub const AND: u32 = 15;
    /// Comparison operators
    pub const CMP: u32 = 20;
    /// Arrow type
    pub const ARROW: u32 = 25;
    /// Addition, subtraction
    pub const ADD: u32 = 65;
    /// Multiplication, division
    pub const MUL: u32 = 70;
    /// Unary operators
    pub const UNARY: u32 = 100;
    /// Application
    pub const APP: u32 = 1024;
    /// Atomic expressions (highest precedence)
    pub const ATOM: u32 = u32::MAX;
}
/// Pretty print a surface expression to a string.
pub fn print_expr(expr: &SurfaceExpr) -> String {
    let mut pp = PrettyPrinter::new();
    pp.print_expr(expr)
        .expect("writing to String is infallible");
    pp.output()
}
/// Pretty print a surface expression to a string using a given configuration.
#[allow(dead_code)]
pub fn print_expr_with_config(expr: &SurfaceExpr, config: PrettyConfig) -> String {
    let mut pp = PrettyPrinter::with_config(config);
    pp.print_expr(expr)
        .expect("writing to String is infallible");
    pp.output()
}
/// Pretty print a declaration to a string.
pub fn print_decl(decl: &Decl) -> String {
    let mut pp = PrettyPrinter::new();
    pp.print_decl(decl)
        .expect("writing to String is infallible");
    pp.output()
}
/// Pretty print a declaration to a string using a given configuration.
#[allow(dead_code)]
pub fn print_decl_with_config(decl: &Decl, config: PrettyConfig) -> String {
    let mut pp = PrettyPrinter::with_config(config);
    pp.print_decl(decl)
        .expect("writing to String is infallible");
    pp.output()
}
/// Pretty print a pattern to a string.
#[allow(dead_code)]
pub fn print_pattern(pat: &Pattern) -> String {
    let mut pp = PrettyPrinter::new();
    pp.print_pattern(pat)
        .expect("writing to String is infallible");
    pp.output()
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::prettyprint::*;
    use crate::Span;
    fn mk_span() -> Span {
        Span::new(0, 1, 1, 1)
    }
    fn mk_loc<T>(value: T) -> Located<T> {
        Located::new(value, mk_span())
    }
    fn mk_loc_box<T>(value: T) -> Box<Located<T>> {
        Box::new(mk_loc(value))
    }
    #[test]
    fn test_print_var() {
        let expr = SurfaceExpr::Var("x".to_string());
        assert_eq!(print_expr(&expr), "x");
    }
    #[test]
    fn test_print_sort() {
        let expr = SurfaceExpr::Sort(SortKind::Type);
        assert_eq!(print_expr(&expr), "Type");
    }
    #[test]
    fn test_print_lit() {
        let expr = SurfaceExpr::Lit(Literal::Nat(42));
        assert_eq!(print_expr(&expr), "42");
    }
    #[test]
    fn test_print_hole() {
        let expr = SurfaceExpr::Hole;
        assert_eq!(print_expr(&expr), "_");
    }
    #[test]
    fn test_print_arrow() {
        let binder = Binder {
            name: "_".to_string(),
            ty: Some(mk_loc_box(SurfaceExpr::Var("A".to_string()))),
            info: BinderKind::Default,
        };
        let expr = SurfaceExpr::Pi(vec![binder], mk_loc_box(SurfaceExpr::Var("B".to_string())));
        let output = print_expr(&expr);
        assert!(output.contains("->"));
    }
    #[test]
    fn test_config_defaults() {
        let config = PrettyConfig::default();
        assert_eq!(config.max_width, 100);
        assert_eq!(config.indent_size, 2);
        assert!(!config.show_implicit);
        assert!(!config.show_universes);
        assert!(!config.use_unicode);
        assert!(config.use_notation);
        assert_eq!(config.parens_mode, ParensMode::Minimal);
    }
    #[test]
    fn test_config_builder() {
        let config = PrettyConfig::new()
            .with_max_width(80)
            .with_indent_size(4)
            .with_show_implicit(true)
            .with_show_universes(true)
            .with_unicode(true)
            .with_notation(false)
            .with_parens_mode(ParensMode::Full);
        assert_eq!(config.max_width, 80);
        assert_eq!(config.indent_size, 4);
        assert!(config.show_implicit);
        assert!(config.show_universes);
        assert!(config.use_unicode);
        assert!(!config.use_notation);
        assert_eq!(config.parens_mode, ParensMode::Full);
    }
    #[test]
    fn test_unicode_arrow() {
        let config = PrettyConfig::new().with_unicode(true);
        let binder = Binder {
            name: "_".to_string(),
            ty: Some(mk_loc_box(SurfaceExpr::Var("A".to_string()))),
            info: BinderKind::Default,
        };
        let expr = SurfaceExpr::Pi(vec![binder], mk_loc_box(SurfaceExpr::Var("B".to_string())));
        let output = print_expr_with_config(&expr, config);
        assert!(output.contains("\u{2192}"));
        assert!(!output.contains("->"));
    }
    #[test]
    fn test_ascii_arrow() {
        let config = PrettyConfig::new().with_unicode(false);
        let binder = Binder {
            name: "_".to_string(),
            ty: Some(mk_loc_box(SurfaceExpr::Var("A".to_string()))),
            info: BinderKind::Default,
        };
        let expr = SurfaceExpr::Pi(vec![binder], mk_loc_box(SurfaceExpr::Var("B".to_string())));
        let output = print_expr_with_config(&expr, config);
        assert!(output.contains("->"));
    }
    #[test]
    fn test_unicode_forall() {
        let config = PrettyConfig::new().with_unicode(true);
        let binder = Binder {
            name: "x".to_string(),
            ty: Some(mk_loc_box(SurfaceExpr::Var("A".to_string()))),
            info: BinderKind::Default,
        };
        let expr = SurfaceExpr::Pi(vec![binder], mk_loc_box(SurfaceExpr::Var("B".to_string())));
        let output = print_expr_with_config(&expr, config);
        assert!(output.contains("\u{2200}"));
    }
    #[test]
    fn test_unicode_lambda() {
        let config = PrettyConfig::new().with_unicode(true);
        let binder = Binder {
            name: "x".to_string(),
            ty: Some(mk_loc_box(SurfaceExpr::Var("Nat".to_string()))),
            info: BinderKind::Default,
        };
        let expr = SurfaceExpr::Lam(vec![binder], mk_loc_box(SurfaceExpr::Var("x".to_string())));
        let output = print_expr_with_config(&expr, config);
        assert!(output.contains("\u{03bb}"));
    }
    #[test]
    fn test_ascii_lambda() {
        let binder = Binder {
            name: "x".to_string(),
            ty: Some(mk_loc_box(SurfaceExpr::Var("Nat".to_string()))),
            info: BinderKind::Default,
        };
        let expr = SurfaceExpr::Lam(vec![binder], mk_loc_box(SurfaceExpr::Var("x".to_string())));
        let output = print_expr(&expr);
        assert!(output.contains("fun"));
    }
    #[test]
    fn test_hide_universes() {
        let config = PrettyConfig::new().with_show_universes(false);
        let expr = SurfaceExpr::Sort(SortKind::TypeU("u".to_string()));
        let output = print_expr_with_config(&expr, config);
        assert_eq!(output, "Type");
    }
    #[test]
    fn test_show_universes() {
        let config = PrettyConfig::new().with_show_universes(true);
        let expr = SurfaceExpr::Sort(SortKind::TypeU("u".to_string()));
        let output = print_expr_with_config(&expr, config);
        assert_eq!(output, "Type u");
    }
    #[test]
    fn test_prec_atom_no_parens() {
        let mut pp = PrettyPrinter::new();
        pp.print_expr_prec(&SurfaceExpr::Var("x".to_string()), prec::APP)
            .expect("test operation should succeed");
        assert_eq!(pp.output(), "x");
    }
    #[test]
    fn test_prec_lambda_in_app() {
        let mut pp = PrettyPrinter::new();
        let binder = Binder {
            name: "x".to_string(),
            ty: None,
            info: BinderKind::Default,
        };
        let lam = SurfaceExpr::Lam(vec![binder], mk_loc_box(SurfaceExpr::Var("x".to_string())));
        pp.print_expr_prec(&lam, prec::APP)
            .expect("test operation should succeed");
        let output = pp.output();
        assert!(output.starts_with('('));
        assert!(output.ends_with(')'));
    }
    #[test]
    fn test_prec_full_mode() {
        let config = PrettyConfig::new().with_parens_mode(ParensMode::Full);
        let mut pp = PrettyPrinter::with_config(config);
        let app = SurfaceExpr::App(
            mk_loc_box(SurfaceExpr::Var("f".to_string())),
            mk_loc_box(SurfaceExpr::Var("x".to_string())),
        );
        pp.print_expr_prec(&app, 0)
            .expect("test operation should succeed");
        let output = pp.output();
        assert!(output.starts_with('('));
    }
    #[test]
    fn test_print_pattern_wild() {
        assert_eq!(print_pattern(&Pattern::Wild), "_");
    }
    #[test]
    fn test_print_pattern_var() {
        assert_eq!(print_pattern(&Pattern::Var("x".to_string())), "x");
    }
    #[test]
    fn test_print_pattern_ctor_no_args() {
        assert_eq!(
            print_pattern(&Pattern::Ctor("None".to_string(), vec![])),
            "None"
        );
    }
    #[test]
    fn test_print_pattern_ctor_with_args() {
        let pat = Pattern::Ctor(
            "Some".to_string(),
            vec![mk_loc(Pattern::Var("x".to_string()))],
        );
        assert_eq!(print_pattern(&pat), "Some x");
    }
    #[test]
    fn test_print_pattern_nested_ctor_parenthesized() {
        let inner = Pattern::Ctor(
            "Pair".to_string(),
            vec![
                mk_loc(Pattern::Var("a".to_string())),
                mk_loc(Pattern::Var("b".to_string())),
            ],
        );
        let pat = Pattern::Ctor("Some".to_string(), vec![mk_loc(inner)]);
        assert_eq!(print_pattern(&pat), "Some (Pair a b)");
    }
    #[test]
    fn test_print_pattern_or() {
        let pat = Pattern::Or(
            Box::new(mk_loc(Pattern::Ctor("A".to_string(), vec![]))),
            Box::new(mk_loc(Pattern::Ctor("B".to_string(), vec![]))),
        );
        assert_eq!(print_pattern(&pat), "A | B");
    }
    #[test]
    fn test_print_pattern_lit() {
        assert_eq!(print_pattern(&Pattern::Lit(Literal::Nat(42))), "42");
    }
    #[test]
    fn test_print_match_expression() {
        let scrutinee = SurfaceExpr::Var("x".to_string());
        let arms = vec![
            MatchArm {
                pattern: mk_loc(Pattern::Ctor("Nat.zero".to_string(), vec![])),
                guard: None,
                rhs: mk_loc(SurfaceExpr::Lit(Literal::Nat(0))),
            },
            MatchArm {
                pattern: mk_loc(Pattern::Ctor(
                    "Nat.succ".to_string(),
                    vec![mk_loc(Pattern::Var("n".to_string()))],
                )),
                guard: None,
                rhs: mk_loc(SurfaceExpr::Var("n".to_string())),
            },
        ];
        let mut pp = PrettyPrinter::new();
        pp.print_match(&scrutinee, &arms)
            .expect("test operation should succeed");
        let output = pp.output();
        assert!(output.contains("match x with"));
        assert!(output.contains("| Nat.zero => 0"));
        assert!(output.contains("| Nat.succ n => n"));
    }
    #[test]
    fn test_print_match_with_guard() {
        let arms = vec![MatchArm {
            pattern: mk_loc(Pattern::Var("n".to_string())),
            guard: Some(mk_loc(SurfaceExpr::Var("cond".to_string()))),
            rhs: mk_loc(SurfaceExpr::Var("n".to_string())),
        }];
        let mut pp = PrettyPrinter::new();
        pp.print_match(&SurfaceExpr::Var("x".to_string()), &arms)
            .expect("test operation should succeed");
        let output = pp.output();
        assert!(output.contains("when cond"));
    }
    #[test]
    fn test_print_do_notation() {
        let actions = vec![
            DoAction::Bind(
                "x".to_string(),
                mk_loc(SurfaceExpr::Var("getLine".to_string())),
            ),
            DoAction::Let("y".to_string(), mk_loc(SurfaceExpr::Var("x".to_string()))),
            DoAction::Expr(mk_loc(SurfaceExpr::App(
                mk_loc_box(SurfaceExpr::Var("pure".to_string())),
                mk_loc_box(SurfaceExpr::Var("y".to_string())),
            ))),
        ];
        let mut pp = PrettyPrinter::new();
        pp.print_do(&actions)
            .expect("test operation should succeed");
        let output = pp.output();
        assert!(output.starts_with("do\n"));
        assert!(output.contains("let x <- getLine"));
        assert!(output.contains("let y := x"));
        assert!(output.contains("pure y"));
    }
    #[test]
    fn test_print_do_unicode_bind() {
        let config = PrettyConfig::new().with_unicode(true);
        let actions = vec![DoAction::Bind(
            "x".to_string(),
            mk_loc(SurfaceExpr::Var("getLine".to_string())),
        )];
        let mut pp = PrettyPrinter::with_config(config);
        pp.print_do(&actions)
            .expect("test operation should succeed");
        let output = pp.output();
        assert!(output.contains("\u{2190}"));
    }
    #[test]
    fn test_print_inductive_simple() {
        let ctors = vec![
            Constructor {
                name: "nil".to_string(),
                ty: mk_loc(SurfaceExpr::Var("List".to_string())),
            },
            Constructor {
                name: "cons".to_string(),
                ty: mk_loc(SurfaceExpr::Var("List".to_string())),
            },
        ];
        let mut pp = PrettyPrinter::new();
        pp.print_inductive(
            "List",
            &[],
            &[Binder {
                name: "\u{03b1}".to_string(),
                ty: Some(mk_loc_box(SurfaceExpr::Sort(SortKind::Type))),
                info: BinderKind::Default,
            }],
            &SurfaceExpr::Sort(SortKind::Type),
            &ctors,
        )
        .expect("test operation should succeed");
        let output = pp.output();
        assert!(output.contains("inductive List"));
        assert!(output.contains("where"));
        assert!(output.contains("| nil"));
        assert!(output.contains("| cons"));
    }
    #[test]
    fn test_print_structure_decl() {
        let decl = Decl::Structure {
            name: "Point".to_string(),
            univ_params: vec![],
            extends: vec![],
            fields: vec![
                FieldDecl {
                    name: "x".to_string(),
                    ty: mk_loc(SurfaceExpr::Var("Nat".to_string())),
                    default: None,
                },
                FieldDecl {
                    name: "y".to_string(),
                    ty: mk_loc(SurfaceExpr::Var("Nat".to_string())),
                    default: None,
                },
            ],
        };
        let output = print_decl(&decl);
        assert!(output.contains("structure Point where"));
        assert!(output.contains("x : Nat"));
        assert!(output.contains("y : Nat"));
    }
    #[test]
    fn test_print_class_decl() {
        let decl = Decl::ClassDecl {
            name: "Functor".to_string(),
            univ_params: vec!["u".to_string()],
            extends: vec![],
            fields: vec![FieldDecl {
                name: "map".to_string(),
                ty: mk_loc(SurfaceExpr::Var("MapTy".to_string())),
                default: None,
            }],
        };
        let output = print_decl(&decl);
        assert!(output.contains("class Functor"));
        assert!(output.contains("{u}"));
        assert!(output.contains("map : MapTy"));
    }
    #[test]
    fn test_print_structure_with_extends() {
        let decl = Decl::Structure {
            name: "ColorPoint".to_string(),
            univ_params: vec![],
            extends: vec!["Point".to_string()],
            fields: vec![FieldDecl {
                name: "color".to_string(),
                ty: mk_loc(SurfaceExpr::Var("Color".to_string())),
                default: None,
            }],
        };
        let output = print_decl(&decl);
        assert!(output.contains("extends Point"));
    }
    #[test]
    fn test_try_single_line_fits() {
        let mut pp = PrettyPrinter::new();
        let expr = SurfaceExpr::Var("x".to_string());
        let result = pp.try_single_line(&expr);
        assert_eq!(result, Some("x".to_string()));
    }
    #[test]
    fn test_try_single_line_too_long() {
        let config = PrettyConfig::new().with_max_width(5);
        let mut pp = PrettyPrinter::with_config(config);
        let expr = SurfaceExpr::Var("very_long_variable_name".to_string());
        let result = pp.try_single_line(&expr);
        assert!(result.is_none());
    }
    #[test]
    fn test_hide_implicit_binders() {
        let config = PrettyConfig::new().with_show_implicit(false);
        let binders = vec![
            Binder {
                name: "a".to_string(),
                ty: Some(mk_loc_box(SurfaceExpr::Sort(SortKind::Type))),
                info: BinderKind::Implicit,
            },
            Binder {
                name: "x".to_string(),
                ty: Some(mk_loc_box(SurfaceExpr::Var("a".to_string()))),
                info: BinderKind::Default,
            },
        ];
        let mut pp = PrettyPrinter::with_config(config);
        for b in &binders {
            pp.print_binder(b).expect("test operation should succeed");
        }
        let output = pp.output();
        assert!(!output.contains("{a"));
        assert!(output.contains("(x : a)"));
    }
    #[test]
    fn test_show_implicit_binders() {
        let config = PrettyConfig::new().with_show_implicit(true);
        let binder = Binder {
            name: "a".to_string(),
            ty: Some(mk_loc_box(SurfaceExpr::Sort(SortKind::Type))),
            info: BinderKind::Implicit,
        };
        let mut pp = PrettyPrinter::with_config(config);
        pp.print_binder(&binder)
            .expect("test operation should succeed");
        let output = pp.output();
        assert!(output.contains("{a : Type}"));
    }
    #[test]
    fn test_print_inductive_decl() {
        let decl = Decl::Inductive {
            name: "Bool".to_string(),
            univ_params: vec![],
            params: vec![],
            indices: vec![],
            ty: mk_loc(SurfaceExpr::Sort(SortKind::Type)),
            ctors: vec![
                Constructor {
                    name: "true".to_string(),
                    ty: mk_loc(SurfaceExpr::Var("Bool".to_string())),
                },
                Constructor {
                    name: "false".to_string(),
                    ty: mk_loc(SurfaceExpr::Var("Bool".to_string())),
                },
            ],
        };
        let output = print_decl(&decl);
        assert!(output.contains("inductive Bool"));
        assert!(output.contains("| true : Bool"));
        assert!(output.contains("| false : Bool"));
    }
    #[test]
    fn test_print_match_via_expr() {
        let expr = SurfaceExpr::Match(
            mk_loc_box(SurfaceExpr::Var("x".to_string())),
            vec![MatchArm {
                pattern: mk_loc(Pattern::Wild),
                guard: None,
                rhs: mk_loc(SurfaceExpr::Lit(Literal::Nat(0))),
            }],
        );
        let output = print_expr(&expr);
        assert!(output.contains("match x with"));
        assert!(output.contains("| _ => 0"));
    }
    #[test]
    fn test_print_do_via_expr() {
        let expr = SurfaceExpr::Do(vec![DoAction::Expr(mk_loc(SurfaceExpr::Var(
            "pure".to_string(),
        )))]);
        let output = print_expr(&expr);
        assert!(output.contains("do"));
        assert!(output.contains("pure"));
    }
    #[test]
    fn test_print_structure_with_defaults() {
        let decl = Decl::Structure {
            name: "Config".to_string(),
            univ_params: vec![],
            extends: vec![],
            fields: vec![FieldDecl {
                name: "width".to_string(),
                ty: mk_loc(SurfaceExpr::Var("Nat".to_string())),
                default: Some(mk_loc(SurfaceExpr::Lit(Literal::Nat(80)))),
            }],
        };
        let output = print_decl(&decl);
        assert!(output.contains("width : Nat := 80"));
    }
}
pub(super) fn render_doc(doc: &DocNode, indent: usize, _width: usize, out: &mut String) {
    match doc {
        DocNode::Empty => {}
        DocNode::Text(s) => out.push_str(s),
        DocNode::Line(extra) => {
            out.push('\n');
            out.push_str(&" ".repeat(indent + extra));
        }
        DocNode::Cat(a, b) => {
            render_doc(a, indent, _width, out);
            render_doc(b, indent, _width, out);
        }
        DocNode::Indent(n, doc) => {
            render_doc(doc, indent + n, _width, out);
        }
        DocNode::Group(doc) => {
            render_doc(doc, indent, _width, out);
        }
    }
}
/// A simple expression pretty-printer using a layout algorithm.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn pretty_expr(expr: &str, indent: usize, width: usize) -> String {
    if indent + expr.len() <= width {
        return expr.to_string();
    }
    expr.to_string()
}
#[cfg(test)]
mod prettyprint_ext_tests {
    use super::*;
    use crate::prettyprint::*;
    #[test]
    fn test_doc_node_text() {
        let doc = DocNode::text("hello");
        let out = doc.render(80);
        assert_eq!(out, "hello");
    }
    #[test]
    fn test_doc_node_cat() {
        let doc = DocNode::cat(DocNode::text("hello"), DocNode::text(" world"));
        let out = doc.render(80);
        assert_eq!(out, "hello world");
    }
    #[test]
    fn test_doc_node_line() {
        let doc = DocNode::cat(
            DocNode::text("a"),
            DocNode::cat(DocNode::line(0), DocNode::text("b")),
        );
        let out = doc.render(80);
        assert!(out.contains('\n'));
        assert!(out.contains("a"));
        assert!(out.contains("b"));
    }
    #[test]
    fn test_doc_formatter() {
        let mut fmt = DocFormatter::new(80);
        fmt.write_text("hello");
        fmt.newline(2);
        fmt.write_text("world");
        let out = fmt.finish();
        assert!(out.contains("hello"));
        assert!(out.contains("world"));
    }
    #[test]
    fn test_box_model_of_str() {
        let b = BoxModel::of_str("hello");
        assert_eq!(b.width, 5);
        assert_eq!(b.lines.len(), 1);
    }
    #[test]
    fn test_box_model_vstack() {
        let a = BoxModel::of_str("line1");
        let b = BoxModel::of_str("line2_longer");
        let stacked = a.vstack(b);
        assert_eq!(stacked.lines.len(), 2);
        assert_eq!(stacked.width, 12);
    }
    #[test]
    fn test_pretty_expr_short() {
        let out = pretty_expr("x + y", 0, 80);
        assert_eq!(out, "x + y");
    }
}
#[cfg(test)]
mod prettyprint_ext2_tests {
    use super::*;
    use crate::prettyprint::*;
    #[test]
    fn test_table_formatter() {
        let mut t = TableFormatter::new(" | ");
        t.add_row(vec!["Name", "Type", "Value"]);
        t.add_row(vec!["x", "Nat", "42"]);
        t.add_row(vec!["longname", "Bool", "true"]);
        let rendered = t.render();
        assert!(rendered.contains("Name"));
        assert!(rendered.contains("longname"));
        assert_eq!(rendered.lines().count(), 3);
    }
    #[test]
    fn test_breadcrumb_trail() {
        let mut trail = BreadcrumbTrail::new();
        trail.push("Decl");
        trail.push("Def");
        trail.push("Body");
        assert_eq!(trail.format(), "Decl > Def > Body");
        trail.pop();
        assert_eq!(trail.format(), "Decl > Def");
    }
    #[test]
    fn test_table_col_widths() {
        let mut t = TableFormatter::new(" ");
        t.add_row(vec!["a", "bb", "ccc"]);
        t.add_row(vec!["dddd", "e", "f"]);
        let widths = t.col_widths();
        assert_eq!(widths[0], 4);
        assert_eq!(widths[1], 2);
        assert_eq!(widths[2], 3);
    }
}
/// A line-wrapping formatter.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        if current.is_empty() {
            current.push_str(word);
        } else if current.chars().count() + 1 + word.chars().count() <= width {
            current.push(' ');
            current.push_str(word);
        } else {
            lines.push(current.clone());
            current = word.to_string();
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}
/// Pad a string on the left to a given width.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn left_pad(s: &str, width: usize, pad: char) -> String {
    let n = s.chars().count();
    if n >= width {
        return s.to_string();
    }
    format!("{}{}", pad.to_string().repeat(width - n), s)
}
/// Pad a string on the right to a given width.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn right_pad(s: &str, width: usize, pad: char) -> String {
    let n = s.chars().count();
    if n >= width {
        return s.to_string();
    }
    format!("{}{}", s, pad.to_string().repeat(width - n))
}
#[cfg(test)]
mod prettyprint_ansi_tests {
    use super::*;
    use crate::prettyprint::*;
    #[test]
    fn test_strip_ansi() {
        let s = AnsiHighlighter::keyword("def");
        let stripped = AnsiHighlighter::strip_ansi(&s);
        assert_eq!(stripped, "def");
    }
    #[test]
    fn test_wrap_text() {
        let lines = wrap_text("hello world foo bar", 10);
        assert!(!lines.is_empty());
        for line in &lines {
            assert!(line.chars().count() <= 10);
        }
    }
    #[test]
    fn test_left_pad() {
        assert_eq!(left_pad("42", 5, '0'), "00042");
        assert_eq!(left_pad("hello", 3, ' '), "hello");
    }
    #[test]
    fn test_right_pad() {
        assert_eq!(right_pad("hi", 5, '-'), "hi---");
    }
}
/// A side-by-side diff renderer.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn side_by_side(left: &str, right: &str, width: usize) -> String {
    let half = width / 2;
    let left_lines: Vec<&str> = left.lines().collect();
    let right_lines: Vec<&str> = right.lines().collect();
    let max_lines = left_lines.len().max(right_lines.len());
    let mut out = String::new();
    for i in 0..max_lines {
        let l = left_lines.get(i).copied().unwrap_or("");
        let r = right_lines.get(i).copied().unwrap_or("");
        out.push_str(&format!("{:width$} | {}\n", l, r, width = half));
    }
    out
}
#[cfg(test)]
mod prettyprint_indent_tests {
    use super::*;
    use crate::prettyprint::*;
    #[test]
    fn test_indent_manager() {
        let mut m = IndentManager::new(2);
        m.indent();
        assert_eq!(m.current(), "  ");
        m.indent();
        assert_eq!(m.current(), "    ");
        m.dedent();
        assert_eq!(m.current(), "  ");
    }
    #[test]
    fn test_apply_to() {
        let m = IndentManager::new(4);
        let mut m2 = m;
        m2.indent();
        let result = m2.apply_to("foo\nbar");
        assert!(result.starts_with("    foo"));
    }
    #[test]
    fn test_side_by_side() {
        let out = side_by_side("left\nlines", "right\ntext", 40);
        assert!(out.contains('|'));
        assert!(out.contains("left"));
        assert!(out.contains("right"));
    }
}
/// A simple line counter.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn count_lines(s: &str) -> usize {
    s.lines().count()
}
/// Returns the longest line in a string.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn longest_line(s: &str) -> &str {
    s.lines().max_by_key(|l| l.chars().count()).unwrap_or("")
}
/// Truncates lines longer than max_width.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn truncate_lines(s: &str, max_width: usize) -> String {
    s.lines()
        .map(|l| {
            if l.chars().count() > max_width {
                let truncated: String = l.chars().take(max_width - 3).collect();
                format!("{}...", truncated)
            } else {
                l.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
/// Repeat a string n times.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn repeat_str(s: &str, n: usize) -> String {
    s.repeat(n)
}
/// Centers a string in a field of width `width`, padding with spaces.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn center(s: &str, width: usize) -> String {
    let n = s.chars().count();
    if n >= width {
        return s.to_string();
    }
    let total_pad = width - n;
    let left_pad = total_pad / 2;
    let right_pad = total_pad - left_pad;
    format!("{}{}{}", " ".repeat(left_pad), s, " ".repeat(right_pad))
}
#[cfg(test)]
mod prettyprint_pad {
    use super::*;
    use crate::prettyprint::*;
    #[test]
    fn test_count_lines() {
        assert_eq!(count_lines("a\nb\nc"), 3);
    }
    #[test]
    fn test_longest_line() {
        assert_eq!(longest_line("ab\nfoo\nc"), "foo");
    }
    #[test]
    fn test_truncate_lines() {
        let out = truncate_lines("hello world", 8);
        assert!(out.ends_with("..."));
    }
    #[test]
    fn test_center() {
        assert_eq!(center("hi", 6), "  hi  ");
    }
}
/// A simple ANSI-aware string width calculator.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn visible_width(s: &str) -> usize {
    let mut w = 0;
    let mut in_escape = false;
    for c in s.chars() {
        if in_escape {
            if c == 'm' {
                in_escape = false;
            }
        } else if c == '\x1b' {
            in_escape = true;
        } else {
            w += 1;
        }
    }
    w
}
/// Wraps text to max_width, breaking on whitespace.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn word_wrap(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        if current.is_empty() {
            current.push_str(word);
        } else if current.len() + 1 + word.len() <= max_width {
            current.push(' ');
            current.push_str(word);
        } else {
            lines.push(current.clone());
            current = word.to_string();
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}
/// Pads a string on the right to the given width with a fill character.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn pad_right_char(s: &str, width: usize, fill: char) -> String {
    let n = s.chars().count();
    if n >= width {
        return s.to_string();
    }
    format!("{}{}", s, fill.to_string().repeat(width - n))
}
/// Pads a string on the left to the given width with a fill character.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn pad_left_char(s: &str, width: usize, fill: char) -> String {
    let n = s.chars().count();
    if n >= width {
        return s.to_string();
    }
    format!("{}{}", fill.to_string().repeat(width - n), s)
}
#[cfg(test)]
mod prettyprint_pad2 {
    use super::*;
    use crate::prettyprint::*;
    #[test]
    fn test_visible_width() {
        assert_eq!(visible_width("hello"), 5);
        assert_eq!(visible_width("\x1b[31mred\x1b[0m"), 3);
    }
    #[test]
    fn test_word_wrap() {
        let wrapped = word_wrap("one two three four five", 10);
        for line in &wrapped {
            assert!(line.len() <= 10);
        }
    }
    #[test]
    fn test_pad_right_char() {
        assert_eq!(pad_right_char("hi", 5, '.'), "hi...");
    }
    #[test]
    fn test_pad_left_char() {
        assert_eq!(pad_left_char("42", 5, '0'), "00042");
    }
}
