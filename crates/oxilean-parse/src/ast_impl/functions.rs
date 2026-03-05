//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

#[allow(unused_imports)]
use crate::ast::{SimpleNodeKindExt, TreeNodeExt};

use super::types::{AttributeKind, TreePathExt, TreeStats, TreeStatsExt2};

/// Parse an attribute name string into an `AttributeKind`.
#[allow(dead_code)]
pub fn parse_attribute_kind(s: &str) -> AttributeKind {
    match s {
        "simp" => AttributeKind::Simp,
        "ext" => AttributeKind::Ext,
        "instance" => AttributeKind::Instance,
        "reducible" => AttributeKind::Reducible,
        "irreducible" => AttributeKind::Irreducible,
        "inline" => AttributeKind::Inline,
        "noinline" => AttributeKind::NoInline,
        "specialize" => AttributeKind::SpecializeAttr,
        other => AttributeKind::Custom(other.to_string()),
    }
}
/// A reference to a tactic used at the expression level (e.g., `by exact rfl`).
pub type TacticRef = String;
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast_impl::*;
    use crate::tokens::{Span, StringPart};
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
        mk_located(SurfaceExpr::Lit(Literal::Nat(n)))
    }
    #[test]
    fn test_surface_expr_var() {
        let expr = SurfaceExpr::Var("x".to_string());
        assert!(matches!(expr, SurfaceExpr::Var(_)));
    }
    #[test]
    fn test_surface_expr_lit() {
        let expr = SurfaceExpr::Lit(Literal::Nat(42));
        assert_eq!(expr, SurfaceExpr::Lit(Literal::Nat(42)));
    }
    #[test]
    fn test_located() {
        let loc = Located::new(SurfaceExpr::Var("x".to_string()), mk_span());
        assert_eq!(loc.value, SurfaceExpr::Var("x".to_string()));
    }
    #[test]
    fn test_binder() {
        let binder = Binder {
            name: "x".to_string(),
            ty: None,
            info: BinderKind::Default,
        };
        assert_eq!(binder.name, "x");
        assert!(binder.ty.is_none());
    }
    #[test]
    fn test_sort_kind_display() {
        assert_eq!(format!("{}", SortKind::Type), "Type");
        assert_eq!(format!("{}", SortKind::Prop), "Prop");
        assert_eq!(format!("{}", SortKind::TypeU("u".to_string())), "Type u");
    }
    #[test]
    fn test_literal_display() {
        assert_eq!(format!("{}", Literal::Nat(42)), "42");
        assert_eq!(
            format!("{}", Literal::String("hello".to_string())),
            "\"hello\""
        );
        assert_eq!(format!("{}", Literal::Char('x')), "'x'");
    }
    #[test]
    fn test_field_decl() {
        let field = FieldDecl {
            name: "x".to_string(),
            ty: Located::new(SurfaceExpr::Var("Nat".to_string()), mk_span()),
            default: None,
        };
        assert_eq!(field.name, "x");
        assert!(field.default.is_none());
    }
    #[test]
    fn test_new_expr_variants() {
        let _have = SurfaceExpr::Have(
            "h".to_string(),
            Box::new(Located::new(SurfaceExpr::Var("T".to_string()), mk_span())),
            Box::new(Located::new(SurfaceExpr::Hole, mk_span())),
            Box::new(Located::new(
                SurfaceExpr::Var("body".to_string()),
                mk_span(),
            )),
        );
        let _ctor = SurfaceExpr::AnonymousCtor(vec![
            Located::new(SurfaceExpr::Lit(Literal::Nat(1)), mk_span()),
            Located::new(SurfaceExpr::Lit(Literal::Nat(2)), mk_span()),
        ]);
        let _list = SurfaceExpr::ListLit(vec![Located::new(
            SurfaceExpr::Lit(Literal::Nat(1)),
            mk_span(),
        )]);
        let _tuple = SurfaceExpr::Tuple(vec![
            Located::new(SurfaceExpr::Lit(Literal::Nat(1)), mk_span()),
            Located::new(SurfaceExpr::Lit(Literal::Nat(2)), mk_span()),
        ]);
    }
    #[test]
    fn test_new_decl_variants() {
        let span = mk_span();
        let _structure = Decl::Structure {
            name: "Point".to_string(),
            univ_params: vec![],
            extends: vec![],
            fields: vec![FieldDecl {
                name: "x".to_string(),
                ty: Located::new(SurfaceExpr::Var("Nat".to_string()), span.clone()),
                default: None,
            }],
        };
        let _class = Decl::ClassDecl {
            name: "Monad".to_string(),
            univ_params: vec!["u".to_string()],
            extends: vec![],
            fields: vec![],
        };
        let _var = Decl::Variable {
            binders: vec![Binder {
                name: "n".to_string(),
                ty: Some(Box::new(Located::new(
                    SurfaceExpr::Var("Nat".to_string()),
                    span.clone(),
                ))),
                info: BinderKind::Default,
            }],
        };
        let _check = Decl::HashCmd {
            cmd: "check".to_string(),
            arg: Located::new(SurfaceExpr::Var("Nat".to_string()), span),
        };
    }
    #[test]
    fn test_mutual_decl() {
        let span = mk_span();
        let def1 = mk_located(Decl::Definition {
            name: "even".to_string(),
            univ_params: vec![],
            ty: Some(mk_var("Nat_to_Bool")),
            val: mk_var("even_body"),
            where_clauses: vec![],
            attrs: vec![],
        });
        let def2 = mk_located(Decl::Definition {
            name: "odd".to_string(),
            univ_params: vec![],
            ty: Some(Located::new(
                SurfaceExpr::Var("Nat_to_Bool".to_string()),
                span,
            )),
            val: mk_var("odd_body"),
            where_clauses: vec![],
            attrs: vec![],
        });
        let mutual = Decl::Mutual {
            decls: vec![def1, def2],
        };
        assert!(mutual.is_mutual());
        match &mutual {
            Decl::Mutual { decls } => assert_eq!(decls.len(), 2),
            _ => panic!("expected Mutual"),
        }
    }
    #[test]
    fn test_derive_decl() {
        let derive = Decl::Derive {
            instances: vec!["DecidableEq".to_string(), "Repr".to_string()],
            type_name: "Color".to_string(),
        };
        match &derive {
            Decl::Derive {
                instances,
                type_name,
            } => {
                assert_eq!(instances.len(), 2);
                assert_eq!(type_name, "Color");
            }
            _ => panic!("expected Derive"),
        }
    }
    #[test]
    fn test_notation_decl() {
        let notation = Decl::NotationDecl {
            kind: AstNotationKind::Infixl,
            prec: Some(65),
            name: "+".to_string(),
            notation: "HAdd.hAdd".to_string(),
        };
        match &notation {
            Decl::NotationDecl {
                kind,
                prec,
                name,
                notation,
            } => {
                assert_eq!(*kind, AstNotationKind::Infixl);
                assert_eq!(*prec, Some(65));
                assert_eq!(name, "+");
                assert_eq!(notation, "HAdd.hAdd");
            }
            _ => panic!("expected NotationDecl"),
        }
    }
    #[test]
    fn test_universe_decl() {
        let univ = Decl::Universe {
            names: vec!["u".to_string(), "v".to_string()],
        };
        match &univ {
            Decl::Universe { names } => {
                assert_eq!(names.len(), 2);
                assert_eq!(names[0], "u");
                assert_eq!(names[1], "v");
            }
            _ => panic!("expected Universe"),
        }
    }
    #[test]
    fn test_where_clause() {
        let wc = WhereClause {
            name: "helper".to_string(),
            params: vec![Binder {
                name: "x".to_string(),
                ty: Some(Box::new(mk_var("Nat"))),
                info: BinderKind::Default,
            }],
            ty: Some(mk_var("Nat")),
            val: mk_nat(42),
        };
        assert_eq!(wc.name, "helper");
        assert_eq!(wc.params.len(), 1);
        assert!(wc.ty.is_some());
    }
    #[test]
    fn test_definition_with_where_clauses() {
        let def = Decl::Definition {
            name: "foo".to_string(),
            univ_params: vec![],
            ty: Some(mk_var("Nat")),
            val: mk_var("bar_result"),
            where_clauses: vec![WhereClause {
                name: "bar".to_string(),
                params: vec![],
                ty: None,
                val: mk_nat(10),
            }],
            attrs: vec![AttributeKind::Simp],
        };
        match &def {
            Decl::Definition {
                where_clauses,
                attrs,
                ..
            } => {
                assert_eq!(where_clauses.len(), 1);
                assert_eq!(where_clauses[0].name, "bar");
                assert_eq!(attrs.len(), 1);
                assert_eq!(attrs[0], AttributeKind::Simp);
            }
            _ => panic!("expected Definition"),
        }
    }
    #[test]
    fn test_theorem_with_attrs() {
        let thm = Decl::Theorem {
            name: "add_comm".to_string(),
            univ_params: vec![],
            ty: mk_var("Prop_statement"),
            proof: mk_var("proof_term"),
            where_clauses: vec![],
            attrs: vec![AttributeKind::Simp, AttributeKind::Ext],
        };
        let attrs = thm.typed_attrs();
        assert_eq!(attrs.len(), 2);
        assert_eq!(attrs[0], AttributeKind::Simp);
        assert_eq!(attrs[1], AttributeKind::Ext);
    }
    #[test]
    fn test_axiom_with_attrs() {
        let ax = Decl::Axiom {
            name: "propext".to_string(),
            univ_params: vec![],
            ty: mk_var("Prop_ext_type"),
            attrs: vec![AttributeKind::Reducible],
        };
        let attrs = ax.typed_attrs();
        assert_eq!(attrs.len(), 1);
        assert_eq!(attrs[0], AttributeKind::Reducible);
    }
    #[test]
    fn test_attribute_kind_display() {
        assert_eq!(format!("{}", AttributeKind::Simp), "simp");
        assert_eq!(format!("{}", AttributeKind::Ext), "ext");
        assert_eq!(format!("{}", AttributeKind::Instance), "instance");
        assert_eq!(format!("{}", AttributeKind::Reducible), "reducible");
        assert_eq!(format!("{}", AttributeKind::Irreducible), "irreducible");
        assert_eq!(format!("{}", AttributeKind::Inline), "inline");
        assert_eq!(format!("{}", AttributeKind::NoInline), "noinline");
        assert_eq!(format!("{}", AttributeKind::SpecializeAttr), "specialize");
        assert_eq!(
            format!("{}", AttributeKind::Custom("my_attr".to_string())),
            "my_attr"
        );
    }
    #[test]
    fn test_attribute_kind_name() {
        assert_eq!(AttributeKind::Simp.name(), "simp");
        assert_eq!(AttributeKind::Custom("foo".to_string()).name(), "foo");
    }
    #[test]
    fn test_attribute_kind_is_custom() {
        assert!(!AttributeKind::Simp.is_custom());
        assert!(AttributeKind::Custom("foo".to_string()).is_custom());
    }
    #[test]
    fn test_parse_attribute_kind() {
        assert_eq!(parse_attribute_kind("simp"), AttributeKind::Simp);
        assert_eq!(parse_attribute_kind("ext"), AttributeKind::Ext);
        assert_eq!(parse_attribute_kind("inline"), AttributeKind::Inline);
        assert_eq!(
            parse_attribute_kind("my_custom"),
            AttributeKind::Custom("my_custom".to_string())
        );
    }
    #[test]
    fn test_return_expr() {
        let ret = SurfaceExpr::Return(Box::new(mk_nat(42)));
        match &ret {
            SurfaceExpr::Return(inner) => {
                assert_eq!(inner.value, SurfaceExpr::Lit(Literal::Nat(42)));
            }
            _ => panic!("expected Return"),
        }
        let display = format!("{}", ret);
        assert!(display.contains("return"));
    }
    #[test]
    fn test_string_interp_expr() {
        let interp = SurfaceExpr::StringInterp(vec![
            StringPart::Literal("hello ".to_string()),
            StringPart::Interpolation(vec![]),
        ]);
        match &interp {
            SurfaceExpr::StringInterp(parts) => {
                assert_eq!(parts.len(), 2);
            }
            _ => panic!("expected StringInterp"),
        }
    }
    #[test]
    fn test_range_expr_full() {
        let range = SurfaceExpr::Range(Some(Box::new(mk_nat(1))), Some(Box::new(mk_nat(10))));
        let display = format!("{}", range);
        assert!(display.contains(".."));
    }
    #[test]
    fn test_range_expr_open_start() {
        let range = SurfaceExpr::Range(None, Some(Box::new(mk_nat(10))));
        let display = format!("{}", range);
        assert!(display.starts_with(".."));
    }
    #[test]
    fn test_range_expr_open_end() {
        let range = SurfaceExpr::Range(Some(Box::new(mk_nat(1))), None);
        let display = format!("{}", range);
        assert!(display.contains(".."));
    }
    #[test]
    fn test_by_tactic_expr() {
        let by_tac = SurfaceExpr::ByTactic(vec![
            mk_located("exact".to_string()),
            mk_located("rfl".to_string()),
        ]);
        match &by_tac {
            SurfaceExpr::ByTactic(tactics) => {
                assert_eq!(tactics.len(), 2);
                assert_eq!(tactics[0].value, "exact");
                assert_eq!(tactics[1].value, "rfl");
            }
            _ => panic!("expected ByTactic"),
        }
        let display = format!("{}", by_tac);
        assert!(display.contains("by"));
    }
    #[test]
    fn test_calc_expr() {
        let step = CalcStep {
            lhs: mk_var("a"),
            rel: "=".to_string(),
            rhs: mk_var("b"),
            proof: mk_var("proof_ab"),
        };
        let calc = SurfaceExpr::Calc(vec![step]);
        match &calc {
            SurfaceExpr::Calc(steps) => {
                assert_eq!(steps.len(), 1);
                assert_eq!(steps[0].rel, "=");
            }
            _ => panic!("expected Calc"),
        }
        let display = format!("{}", calc);
        assert!(display.contains("calc"));
    }
    #[test]
    fn test_calc_step_constructor() {
        let step = CalcStep::new(mk_var("x"), "<".to_string(), mk_var("y"), mk_var("proof"));
        assert_eq!(step.rel, "<");
    }
    #[test]
    fn test_where_clause_constructor() {
        let wc = WhereClause::new("aux".to_string(), vec![], None, mk_nat(0));
        assert_eq!(wc.name, "aux");
        assert!(wc.params.is_empty());
        assert!(wc.ty.is_none());
    }
    #[test]
    fn test_surface_expr_helpers() {
        let v = SurfaceExpr::var("x");
        assert!(v.is_var());
        assert_eq!(v.as_var(), Some("x"));
        let n = SurfaceExpr::nat(42);
        assert!(!n.is_var());
        let s = SurfaceExpr::string("hello");
        assert!(matches!(s, SurfaceExpr::Lit(Literal::String(_))));
        #[allow(clippy::approx_constant)]
        let f = SurfaceExpr::float(3.14);
        assert!(matches!(f, SurfaceExpr::Lit(Literal::Float(_))));
        let h = SurfaceExpr::Hole;
        assert!(h.is_hole());
    }
    #[test]
    fn test_located_map() {
        let loc = Located::new(42, mk_span());
        let mapped = loc.map(|x| x * 2);
        assert_eq!(mapped.value, 84);
    }
    #[test]
    fn test_located_display() {
        let loc = Located::new(SurfaceExpr::Var("x".to_string()), mk_span());
        let display = format!("{}", loc);
        assert_eq!(display, "x");
    }
    #[test]
    fn test_decl_name() {
        let def = Decl::Definition {
            name: "foo".to_string(),
            univ_params: vec![],
            ty: None,
            val: mk_var("bar"),
            where_clauses: vec![],
            attrs: vec![],
        };
        assert_eq!(def.name(), Some("foo"));
        let import = Decl::Import {
            path: vec!["M".to_string()],
        };
        assert_eq!(import.name(), None);
        let derive = Decl::Derive {
            instances: vec!["Repr".to_string()],
            type_name: "MyType".to_string(),
        };
        assert_eq!(derive.name(), Some("MyType"));
    }
    #[test]
    fn test_ast_notation_kind_display() {
        assert_eq!(format!("{}", AstNotationKind::Prefix), "prefix");
        assert_eq!(format!("{}", AstNotationKind::Postfix), "postfix");
        assert_eq!(format!("{}", AstNotationKind::Infixl), "infixl");
        assert_eq!(format!("{}", AstNotationKind::Infixr), "infixr");
        assert_eq!(format!("{}", AstNotationKind::Notation), "notation");
    }
    #[test]
    fn test_binder_kind_display() {
        assert_eq!(format!("{}", BinderKind::Default), "explicit");
        assert_eq!(format!("{}", BinderKind::Implicit), "implicit");
        assert_eq!(format!("{}", BinderKind::Instance), "instance");
        assert_eq!(format!("{}", BinderKind::StrictImplicit), "strict_implicit");
    }
    #[test]
    fn test_where_clause_display() {
        let wc = WhereClause {
            name: "helper".to_string(),
            params: vec![Binder {
                name: "n".to_string(),
                ty: Some(Box::new(mk_var("Nat"))),
                info: BinderKind::Default,
            }],
            ty: Some(mk_var("Nat")),
            val: mk_nat(0),
        };
        let display = format!("{}", wc);
        assert!(display.contains("helper"));
        assert!(display.contains("(n : ...)"));
    }
    #[test]
    fn test_literal_float_display() {
        #[allow(clippy::approx_constant)]
        let val = 3.14;
        assert_eq!(format!("{}", Literal::Float(val)), "3.14");
    }
    #[test]
    fn test_do_action_return() {
        let action = DoAction::Return(mk_nat(42));
        match &action {
            DoAction::Return(expr) => {
                assert_eq!(expr.value, SurfaceExpr::Lit(Literal::Nat(42)));
            }
            _ => panic!("expected DoAction::Return"),
        }
    }
    #[test]
    fn test_surface_expr_display_comprehensive() {
        assert_eq!(format!("{}", SurfaceExpr::Sort(SortKind::Type)), "Type");
        assert_eq!(format!("{}", SurfaceExpr::Hole), "_");
        let proj = SurfaceExpr::Proj(Box::new(mk_var("p")), "x".to_string());
        assert_eq!(format!("{}", proj), "p.x");
        let if_expr = SurfaceExpr::If(
            Box::new(mk_var("cond")),
            Box::new(mk_var("t")),
            Box::new(mk_var("f")),
        );
        let display = format!("{}", if_expr);
        assert!(display.contains("if"));
        assert!(display.contains("then"));
        assert!(display.contains("else"));
    }
    #[test]
    fn test_all_notation_kinds_are_distinct() {
        let kinds = [
            AstNotationKind::Prefix,
            AstNotationKind::Postfix,
            AstNotationKind::Infixl,
            AstNotationKind::Infixr,
            AstNotationKind::Notation,
        ];
        for i in 0..kinds.len() {
            for j in (i + 1)..kinds.len() {
                assert_ne!(kinds[i], kinds[j]);
            }
        }
    }
    #[test]
    fn test_multiple_calc_steps() {
        let steps = vec![
            CalcStep::new(mk_var("a"), "=".to_string(), mk_var("b"), mk_var("p1")),
            CalcStep::new(mk_var("_"), "<".to_string(), mk_var("c"), mk_var("p2")),
            CalcStep::new(mk_var("_"), "<=".to_string(), mk_var("d"), mk_var("p3")),
        ];
        let calc = SurfaceExpr::Calc(steps);
        match &calc {
            SurfaceExpr::Calc(steps) => assert_eq!(steps.len(), 3),
            _ => panic!("expected Calc"),
        }
    }
}
/// A mutable transformation pass over tree nodes.
#[allow(dead_code)]
pub trait TreeTransformExt {
    /// Transform a tree node, returning a new (possibly different) node.
    fn transform(&mut self, node: TreeNodeExt) -> TreeNodeExt;
}
/// Applies a tree transform to a root node.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn apply_transform_ext<T: TreeTransformExt>(root: TreeNodeExt, t: &mut T) -> TreeNodeExt {
    t.transform(root)
}
/// A tree equality checker (structural equality).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn trees_equal_ext(a: &TreeNodeExt, b: &TreeNodeExt) -> bool {
    if a.kind != b.kind {
        return false;
    }
    if a.label != b.label {
        return false;
    }
    if a.children.len() != b.children.len() {
        return false;
    }
    a.children
        .iter()
        .zip(b.children.iter())
        .all(|(ac, bc)| trees_equal_ext(ac, bc))
}
/// Follow a path in a tree, returning the node at that path.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn follow_path_ext<'a>(root: &'a TreeNodeExt, path: &TreePathExt) -> Option<&'a TreeNodeExt> {
    let mut node = root;
    for &idx in &path.0 {
        node = node.children.get(idx)?;
    }
    Some(node)
}
/// Counts occurrences of a given label in a tree.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn count_label_ext(node: &TreeNodeExt, label: &str) -> usize {
    let mut count = if node.label == label { 1 } else { 0 };
    for child in &node.children {
        count += count_label_ext(child, label);
    }
    count
}
#[cfg(test)]
mod ast_impl_ext_tests {
    use super::*;
    use crate::ast_impl::*;
    use crate::tokens::{Span, StringPart};
    #[test]
    fn test_identity_transform() {
        let tree = TreeNodeExt::lam("x", TreeNodeExt::leaf("x"));
        let mut t = IdentityTransformExt;
        let result = t.transform(tree.clone());
        assert!(trees_equal_ext(&result, &tree));
    }
    #[test]
    fn test_rename_transform() {
        let tree = TreeNodeExt::app(TreeNodeExt::leaf("x"), TreeNodeExt::leaf("y"));
        let mut t = RenameTransformExt::new("x", "z");
        let result = t.transform(tree);
        assert_eq!(result.children[0].label, "z");
        assert_eq!(result.children[1].label, "y");
        assert_eq!(t.count, 1);
    }
    #[test]
    fn test_trees_equal() {
        let t1 = TreeNodeExt::leaf("x");
        let t2 = TreeNodeExt::leaf("x");
        let t3 = TreeNodeExt::leaf("y");
        assert!(trees_equal_ext(&t1, &t2));
        assert!(!trees_equal_ext(&t1, &t3));
    }
    #[test]
    fn test_tree_path() {
        let path = TreePathExt::root().child(0).child(1);
        assert_eq!(path.depth(), 2);
        assert_eq!(path.0, vec![0, 1]);
    }
    #[test]
    fn test_follow_path() {
        let tree = TreeNodeExt::app(TreeNodeExt::leaf("f"), TreeNodeExt::leaf("x"));
        let path = TreePathExt::root().child(0);
        let node = follow_path_ext(&tree, &path).expect("test operation should succeed");
        assert_eq!(node.label, "f");
    }
    #[test]
    fn test_count_label() {
        let tree = TreeNodeExt::app(TreeNodeExt::leaf("x"), TreeNodeExt::leaf("x"));
        assert_eq!(count_label_ext(&tree, "x"), 2);
        assert_eq!(count_label_ext(&tree, "y"), 0);
    }
    #[test]
    fn test_zipper_down_up() {
        let tree = TreeNodeExt::app(TreeNodeExt::leaf("f"), TreeNodeExt::leaf("x"));
        let zipper = TreeZipperExt::new(tree);
        let z2 = zipper.down(0).expect("test operation should succeed");
        assert_eq!(z2.focus.label, "f");
        let (z3, child) = z2.up().expect("test operation should succeed");
        assert_eq!(child.label, "f");
        assert_eq!(z3.focus.kind, SimpleNodeKindExt::App);
    }
}
pub fn collect_stats(node: &TreeNodeExt, depth: usize, stats: &mut TreeStats) {
    stats.nodes += 1;
    stats.total_size += 1;
    if node.children.is_empty() {
        stats.leaves += 1;
    }
    if depth > stats.max_depth {
        stats.max_depth = depth;
    }
    for child in &node.children {
        collect_stats(child, depth + 1, stats);
    }
}
/// A tree shape fingerprint for structural equality testing.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn shape_fingerprint(node: &TreeNodeExt) -> String {
    if node.children.is_empty() {
        return "L".to_string();
    }
    let child_shapes: Vec<String> = node.children.iter().map(shape_fingerprint).collect();
    format!("({})", child_shapes.join(","))
}
#[cfg(test)]
mod ast_impl_ext2_tests {
    use super::*;
    use crate::ast_impl::*;
    use crate::tokens::{Span, StringPart};
    #[test]
    fn test_transform_memo() {
        let mut memo = TransformMemo::new();
        let node = TreeNodeExt::leaf("x");
        memo.store(42, node.clone());
        assert_eq!(memo.len(), 1);
        let cached = memo.get(42).expect("key should exist");
        assert_eq!(cached.label, "x");
    }
    #[test]
    fn test_tree_stats() {
        let tree = TreeNodeExt::app(TreeNodeExt::leaf("f"), TreeNodeExt::leaf("x"));
        let stats = TreeStats::from_tree(&tree);
        assert_eq!(stats.nodes, 3);
        assert_eq!(stats.leaves, 2);
        assert_eq!(stats.max_depth, 1);
    }
    #[test]
    fn test_shape_fingerprint() {
        let leaf = TreeNodeExt::leaf("x");
        assert_eq!(shape_fingerprint(&leaf), "L");
        let app = TreeNodeExt::app(TreeNodeExt::leaf("f"), TreeNodeExt::leaf("x"));
        assert_eq!(shape_fingerprint(&app), "(L,L)");
    }
    #[test]
    fn test_rename_transform_ext() {
        let tree = TreeNodeExt::app(TreeNodeExt::leaf("x"), TreeNodeExt::leaf("y"));
        let mut t = RenameTransformExt::new("x", "z");
        let result = apply_transform_ext(tree, &mut t);
        assert_eq!(result.children[0].label, "z");
        assert_eq!(t.count, 1);
    }
}
/// An accumulator for collecting all labels in a tree.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn collect_all_labels(node: &TreeNodeExt) -> Vec<String> {
    let mut labels = vec![node.label.clone()];
    for child in &node.children {
        labels.extend(collect_all_labels(child));
    }
    labels
}
/// Returns the leaves of a tree in pre-order.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn collect_leaves(node: &TreeNodeExt) -> Vec<&TreeNodeExt> {
    if node.children.is_empty() {
        return vec![node];
    }
    let mut leaves = Vec::new();
    for child in &node.children {
        leaves.extend(collect_leaves(child));
    }
    leaves
}
/// A deep clone of a tree (already derived via Clone, so this is just an alias).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn deep_clone(node: &TreeNodeExt) -> TreeNodeExt {
    node.clone()
}
/// Returns true if the tree contains a node with the given label.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn contains_label(node: &TreeNodeExt, label: &str) -> bool {
    if node.label == label {
        return true;
    }
    node.children.iter().any(|c| contains_label(c, label))
}
#[cfg(test)]
mod ast_impl_ext3_tests {
    use super::*;
    use crate::ast_impl::*;
    use crate::tokens::{Span, StringPart};
    #[test]
    fn test_tree_cursor() {
        let root = TreeNodeExt::app(TreeNodeExt::leaf("f"), TreeNodeExt::leaf("x"));
        let cursor = TreeCursor::new(root);
        assert_eq!(cursor.child_count(), 2);
        assert_eq!(cursor.depth(), 0);
    }
    #[test]
    fn test_collect_all_labels() {
        let tree = TreeNodeExt::app(TreeNodeExt::leaf("f"), TreeNodeExt::leaf("x"));
        let labels = collect_all_labels(&tree);
        assert!(labels.contains(&"f".to_string()));
        assert!(labels.contains(&"x".to_string()));
    }
    #[test]
    fn test_collect_leaves() {
        let tree = TreeNodeExt::lam("x", TreeNodeExt::leaf("x"));
        let leaves = collect_leaves(&tree);
        assert_eq!(leaves.len(), 1);
        assert_eq!(leaves[0].label, "x");
    }
    #[test]
    fn test_contains_label() {
        let tree = TreeNodeExt::app(TreeNodeExt::leaf("f"), TreeNodeExt::leaf("x"));
        assert!(contains_label(&tree, "f"));
        assert!(!contains_label(&tree, "g"));
    }
}
/// A memoised depth computation for tree nodes.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn memoised_depth(
    node: &TreeNodeExt,
    memo: &mut std::collections::HashMap<u64, usize>,
) -> usize {
    let key = crate::ast::tree_fingerprint_ext(node);
    if let Some(&d) = memo.get(&key) {
        return d;
    }
    let d = if node.children.is_empty() {
        0
    } else {
        1 + node
            .children
            .iter()
            .map(|c| memoised_depth(c, memo))
            .max()
            .unwrap_or(0)
    };
    memo.insert(key, d);
    d
}
/// A tree serialiser that produces a compact string representation.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn serialise_tree(node: &TreeNodeExt) -> String {
    if node.children.is_empty() {
        return format!("{{{}}}", node.label);
    }
    let children: Vec<String> = node.children.iter().map(serialise_tree).collect();
    format!("{{{}:{}}}", node.label, children.join(","))
}
/// A tree deserialiser (simplified: expects serialise_tree format).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn deserialise_tree_leaf(s: &str) -> TreeNodeExt {
    TreeNodeExt::leaf(s.trim_matches(|c| c == '{' || c == '}'))
}
/// Computes the height of a tree (same as depth).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn tree_height(node: &TreeNodeExt) -> usize {
    if node.children.is_empty() {
        return 0;
    }
    1 + node.children.iter().map(tree_height).max().unwrap_or(0)
}
/// Returns the branching factor of a tree (average children per non-leaf node).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn branching_factor(node: &TreeNodeExt) -> f64 {
    let mut total_children = 0usize;
    let mut internal_nodes = 0usize;
    count_branching(node, &mut total_children, &mut internal_nodes);
    if internal_nodes == 0 {
        return 0.0;
    }
    total_children as f64 / internal_nodes as f64
}
pub(super) fn count_branching(node: &TreeNodeExt, total: &mut usize, internals: &mut usize) {
    if !node.children.is_empty() {
        *internals += 1;
        *total += node.children.len();
    }
    for child in &node.children {
        count_branching(child, total, internals);
    }
}
#[cfg(test)]
mod ast_impl_final_tests {
    use super::*;
    use crate::ast_impl::*;
    use crate::tokens::{Span, StringPart};
    #[test]
    fn test_serialise_tree() {
        let leaf = TreeNodeExt::leaf("x");
        assert_eq!(serialise_tree(&leaf), "{x}");
        let app = TreeNodeExt::app(TreeNodeExt::leaf("f"), TreeNodeExt::leaf("x"));
        let s = serialise_tree(&app);
        assert!(s.starts_with("{@:"));
    }
    #[test]
    fn test_tree_height() {
        let leaf = TreeNodeExt::leaf("x");
        assert_eq!(tree_height(&leaf), 0);
        let app = TreeNodeExt::app(TreeNodeExt::leaf("f"), TreeNodeExt::leaf("x"));
        assert_eq!(tree_height(&app), 1);
    }
    #[test]
    fn test_branching_factor() {
        let leaf = TreeNodeExt::leaf("x");
        assert_eq!(branching_factor(&leaf), 0.0);
        let app = TreeNodeExt::app(TreeNodeExt::leaf("f"), TreeNodeExt::leaf("x"));
        assert!((branching_factor(&app) - 2.0).abs() < 0.01);
    }
    #[test]
    fn test_memoised_depth() {
        let tree = TreeNodeExt::app(TreeNodeExt::leaf("f"), TreeNodeExt::leaf("x"));
        let mut memo = std::collections::HashMap::new();
        let d = memoised_depth(&tree, &mut memo);
        assert_eq!(d, 1);
    }
}
/// Map a function over all leaf labels.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn map_leaves<F: Fn(&str) -> String>(node: &TreeNodeExt, f: &F) -> TreeNodeExt {
    if node.children.is_empty() {
        return TreeNodeExt::leaf(&f(&node.label));
    }
    let children = node.children.iter().map(|c| map_leaves(c, f)).collect();
    TreeNodeExt {
        kind: node.kind.clone(),
        label: node.label.clone(),
        children,
    }
}
/// Returns true if any leaf satisfies a predicate.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn any_leaf<F: Fn(&str) -> bool>(node: &TreeNodeExt, pred: &F) -> bool {
    if node.children.is_empty() {
        return pred(&node.label);
    }
    node.children.iter().any(|c| any_leaf(c, pred))
}
#[cfg(test)]
mod ast_impl_pad {
    use super::*;
    use crate::ast_impl::*;
    use crate::tokens::{Span, StringPart};
    #[test]
    fn test_node_cache() {
        let mut c = NodeCache::new();
        c.store("x", 42);
        assert_eq!(c.get("x"), Some(42));
        assert_eq!(c.get("y"), None);
    }
    #[test]
    fn test_map_leaves() {
        let tree = TreeNodeExt::app(TreeNodeExt::leaf("x"), TreeNodeExt::leaf("y"));
        let mapped = map_leaves(&tree, &|s: &str| s.to_uppercase());
        assert_eq!(mapped.children[0].label, "X");
        assert_eq!(mapped.children[1].label, "Y");
    }
    #[test]
    fn test_any_leaf() {
        let tree = TreeNodeExt::app(TreeNodeExt::leaf("x"), TreeNodeExt::leaf("y"));
        assert!(any_leaf(&tree, &|s: &str| s == "x"));
        assert!(!any_leaf(&tree, &|s: &str| s == "z"));
    }
}
/// Compute tree statistics for a TreeNodeExt.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn compute_tree_stats(node: &TreeNodeExt) -> TreeStatsExt2 {
    fn go(node: &TreeNodeExt, depth: usize, stats: &mut TreeStatsExt2) {
        stats.node_count += 1;
        if depth > stats.max_depth {
            stats.max_depth = depth;
        }
        if node.children.is_empty() {
            stats.leaf_count += 1;
        } else {
            let b = node.children.len();
            if b > stats.max_branching {
                stats.max_branching = b;
            }
            for child in &node.children {
                go(child, depth + 1, stats);
            }
        }
    }
    let mut stats = TreeStatsExt2::default();
    go(node, 0, &mut stats);
    stats
}
/// Compute a string fingerprint for the shape of a tree (ignoring labels).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn shape_fingerprint_ext2(node: &TreeNodeExt) -> String {
    if node.children.is_empty() {
        return "L".to_string();
    }
    let children_fp: String = node
        .children
        .iter()
        .map(shape_fingerprint_ext2)
        .collect::<Vec<_>>()
        .join(",");
    format!("N({})", children_fp)
}
/// Deep-clones a tree (same as Clone but explicit).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn deep_clone_ext2(node: &TreeNodeExt) -> TreeNodeExt {
    node.clone()
}
/// Returns all labels in the tree in pre-order.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn collect_all_labels_ext2(node: &TreeNodeExt) -> Vec<String> {
    let mut labels = vec![node.label.clone()];
    for child in &node.children {
        labels.extend(collect_all_labels_ext2(child));
    }
    labels
}
/// Returns all leaf labels in the tree in left-to-right order.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn collect_leaves_ext2(node: &TreeNodeExt) -> Vec<String> {
    if node.children.is_empty() {
        return vec![node.label.clone()];
    }
    node.children.iter().flat_map(collect_leaves_ext2).collect()
}
/// Returns the height of a tree (number of edges from root to deepest leaf).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn tree_height_ext2(node: &TreeNodeExt) -> usize {
    if node.children.is_empty() {
        return 0;
    }
    1 + node
        .children
        .iter()
        .map(tree_height_ext2)
        .max()
        .unwrap_or(0)
}
/// Returns true if a tree contains a node with the given label.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn contains_label_ext2(node: &TreeNodeExt, label: &str) -> bool {
    if node.label == label {
        return true;
    }
    node.children.iter().any(|c| contains_label_ext2(c, label))
}
#[cfg(test)]
mod ast_impl_pad2 {
    use super::*;
    use crate::ast_impl::*;
    use crate::tokens::{Span, StringPart};
    #[test]
    fn test_transform_memo() {
        let mut m = TransformMemoExt2::new();
        m.insert("foo", "FOO");
        assert_eq!(m.get("foo"), Some("FOO"));
        assert!(m.has("foo"));
        assert!(!m.has("bar"));
    }
    #[test]
    fn test_compute_tree_stats() {
        let tree = TreeNodeExt::app(TreeNodeExt::leaf("x"), TreeNodeExt::leaf("y"));
        let stats = compute_tree_stats(&tree);
        assert_eq!(stats.node_count, 3);
        assert_eq!(stats.leaf_count, 2);
        assert_eq!(stats.max_depth, 1);
    }
    #[test]
    fn test_shape_fingerprint_ext2() {
        let tree = TreeNodeExt::app(TreeNodeExt::leaf("x"), TreeNodeExt::leaf("y"));
        let fp = shape_fingerprint_ext2(&tree);
        assert!(fp.starts_with("N("));
        assert!(fp.contains("L"));
    }
    #[test]
    fn test_collect_labels_and_leaves() {
        let tree = TreeNodeExt::app(TreeNodeExt::leaf("a"), TreeNodeExt::leaf("b"));
        let labels = collect_all_labels_ext2(&tree);
        assert!(labels.contains(&"a".to_string()));
        assert!(labels.contains(&"b".to_string()));
        let leaves = collect_leaves_ext2(&tree);
        assert_eq!(leaves, vec!["a".to_string(), "b".to_string()]);
    }
    #[test]
    fn test_tree_height_ext2_and_contains() {
        let leaf = TreeNodeExt::leaf("x");
        assert_eq!(tree_height_ext2(&leaf), 0);
        let tree = TreeNodeExt::app(TreeNodeExt::leaf("a"), TreeNodeExt::leaf("b"));
        assert_eq!(tree_height_ext2(&tree), 1);
        assert!(contains_label_ext2(&tree, "a"));
        assert!(!contains_label_ext2(&tree, "z"));
    }
}
#[cfg(test)]
mod ast_impl_pad3 {
    use super::*;
    use crate::ast_impl::*;
    use crate::tokens::{Span, StringPart};
    #[test]
    fn test_tree_zipper() {
        let tree = TreeNodeExt::app(TreeNodeExt::leaf("a"), TreeNodeExt::leaf("b"));
        let z = TreeZipper::new(tree);
        assert_eq!(z.depth(), 0);
        let z2 = z.down(0).expect("test operation should succeed");
        assert_eq!(z2.depth(), 1);
        assert_eq!(z2.label(), "a");
    }
}
