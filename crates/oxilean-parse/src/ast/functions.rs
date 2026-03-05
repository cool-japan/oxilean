//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

pub use crate::ast_impl::{Decl, Located, SurfaceExpr};

use super::types::{ImportDecl, ScopeDecl, SimpleNodeKindExt, TreeNodeExt};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    #[test]
    fn test_pos_start() {
        let p = Pos::start();
        assert_eq!(p.line, 1);
        assert_eq!(p.col, 1);
    }
    #[test]
    fn test_pos_next_col() {
        let p = Pos::new(1, 1).next_col();
        assert_eq!(p.col, 2);
    }
    #[test]
    fn test_pos_next_line() {
        let p = Pos::new(1, 5).next_line();
        assert_eq!(p.line, 2);
        assert_eq!(p.col, 1);
    }
    #[test]
    fn test_pos_is_before() {
        let p1 = Pos::new(1, 1);
        let p2 = Pos::new(2, 1);
        assert!(p1.is_before(&p2));
        assert!(!p2.is_before(&p1));
    }
    #[test]
    fn test_pos_display() {
        let p = Pos::new(10, 5);
        assert_eq!(format!("{}", p), "10:5");
    }
    #[test]
    fn test_byte_range_len() {
        let r = ByteRange::new(3, 7);
        assert_eq!(r.len(), 4);
    }
    #[test]
    fn test_byte_range_empty() {
        let r = ByteRange::empty();
        assert!(r.is_empty());
        assert_eq!(r.len(), 0);
    }
    #[test]
    fn test_byte_range_union() {
        let r1 = ByteRange::new(0, 5);
        let r2 = ByteRange::new(3, 10);
        let u = r1.union(r2);
        assert_eq!(u.start, 0);
        assert_eq!(u.end, 10);
    }
    #[test]
    fn test_byte_range_contains() {
        let r = ByteRange::new(2, 8);
        assert!(r.contains(5));
        assert!(!r.contains(8));
        assert!(!r.contains(1));
    }
    #[test]
    fn test_byte_range_slice() {
        let src = "hello world";
        let r = ByteRange::new(6, 11);
        assert_eq!(r.slice(src), "world");
    }
    #[test]
    fn test_byte_range_display() {
        let r = ByteRange::new(0, 5);
        assert_eq!(format!("{}", r), "[0..5)");
    }
    #[test]
    fn test_surface_ident_synthetic() {
        let id = SurfaceIdent::synthetic("foo");
        assert!(id.is_synthetic());
    }
    #[test]
    fn test_surface_ident_anonymous() {
        let id = SurfaceIdent::synthetic("_unused");
        assert!(id.is_anonymous());
    }
    #[test]
    fn test_surface_ident_qualified() {
        let id = SurfaceIdent::synthetic("List.nil");
        assert!(id.is_qualified());
    }
    #[test]
    fn test_surface_ident_split_last() {
        let id = SurfaceIdent::synthetic("Foo.Bar.baz");
        let (prefix, last) = id.split_last().expect("test operation should succeed");
        assert_eq!(prefix, "Foo.Bar");
        assert_eq!(last, "baz");
    }
    #[test]
    fn test_surface_ident_display() {
        let id = SurfaceIdent::synthetic("Nat");
        assert_eq!(format!("{}", id), "Nat");
    }
    #[test]
    fn test_visibility_default_is_public() {
        let v = Visibility::default();
        assert!(v.is_public());
        assert!(!v.is_restricted());
    }
    #[test]
    fn test_visibility_private_is_restricted() {
        let v = Visibility::Private;
        assert!(v.is_restricted());
    }
    #[test]
    fn test_visibility_display() {
        assert_eq!(format!("{}", Visibility::Public), "public");
        assert_eq!(format!("{}", Visibility::Private), "private");
    }
    #[test]
    fn test_fixity_display() {
        assert_eq!(format!("{}", Fixity::InfixLeft), "infixl");
        assert_eq!(format!("{}", Fixity::Prefix), "prefix");
    }
    #[test]
    fn test_prec_ordering() {
        assert!(Prec::MUL > Prec::ADD);
        assert!(Prec::APP > Prec::MUL);
    }
    #[test]
    fn test_prec_tighter() {
        let p = Prec::ADD;
        assert_eq!(p.tighter().value(), p.value() + 1);
    }
    #[test]
    fn test_notation_entry_is_infix() {
        let e = NotationEntry::new("+", "HAdd.hAdd", Fixity::InfixLeft, Prec::ADD);
        assert!(e.is_infix());
    }
    #[test]
    fn test_notation_entry_deprecated() {
        let e = NotationEntry::new("+", "HAdd.hAdd", Fixity::InfixLeft, Prec::ADD).deprecated();
        assert!(e.deprecated);
    }
    #[test]
    fn test_import_decl_dotted_path() {
        let imp = ImportDecl::new(
            vec!["Mathlib".to_string(), "Tactic".to_string()],
            ByteRange::empty(),
        );
        assert_eq!(imp.dotted_path(), "Mathlib.Tactic");
    }
    #[test]
    fn test_import_decl_is_root() {
        let imp = ImportDecl::new(vec!["Init".to_string()], ByteRange::empty());
        assert!(imp.is_root());
    }
    #[test]
    fn test_import_decl_display() {
        let imp = ImportDecl::new(
            vec!["Foo".to_string(), "Bar".to_string()],
            ByteRange::empty(),
        );
        assert_eq!(format!("{}", imp), "import Foo.Bar");
    }
    #[test]
    fn test_scope_decl_opens_scope() {
        assert!(ScopeDecl::Section("MySection".to_string()).opens_scope());
        assert!(ScopeDecl::Namespace("Foo".to_string()).opens_scope());
        assert!(!ScopeDecl::End("Foo".to_string()).opens_scope());
    }
    #[test]
    fn test_scope_decl_closes_scope() {
        assert!(ScopeDecl::End("Foo".to_string()).closes_scope());
        assert!(!ScopeDecl::Section("Foo".to_string()).closes_scope());
    }
    #[test]
    fn test_scope_decl_name() {
        let ns = ScopeDecl::Namespace("Bar".to_string());
        assert_eq!(ns.name(), Some("Bar"));
        let open = ScopeDecl::Open(vec!["Nat".to_string()]);
        assert_eq!(open.name(), None);
    }
    #[test]
    fn test_scope_decl_display() {
        let s = ScopeDecl::Namespace("Foo".to_string());
        assert_eq!(format!("{}", s), "namespace Foo");
    }
    #[test]
    fn test_attr_arg_ident() {
        let a = AttrArg::ident("simp");
        assert_eq!(a.as_ident(), Some("simp"));
        assert_eq!(a.as_num(), None);
    }
    #[test]
    fn test_attr_arg_num() {
        let a = AttrArg::num(42);
        assert_eq!(a.as_num(), Some(42));
    }
    #[test]
    fn test_attr_arg_display_str() {
        let a = AttrArg::str_arg("hello");
        assert_eq!(format!("{}", a), "\"hello\"");
    }
    #[test]
    fn test_attr_arg_list_display() {
        let a = AttrArg::List(vec![AttrArg::ident("a"), AttrArg::num(1)]);
        let s = format!("{}", a);
        assert!(s.contains("a"));
        assert!(s.contains("1"));
    }
    #[test]
    fn test_doc_comment_first_line() {
        let dc = DocComment::new("First line\nSecond line", ByteRange::empty());
        assert_eq!(dc.first_line(), "First line");
    }
    #[test]
    fn test_doc_comment_is_empty() {
        let dc = DocComment::new("  ", ByteRange::empty());
        assert!(dc.is_empty());
    }
    #[test]
    fn test_doc_comment_module_doc() {
        let dc = DocComment::module_doc("Module description");
        assert!(dc.is_module_doc);
    }
    #[test]
    fn test_doc_comment_display() {
        let dc = DocComment::new("A comment", ByteRange::empty());
        let s = format!("{}", dc);
        assert!(s.contains("A comment"));
    }
}
/// A visitor over surface AST nodes.
pub trait AstVisitor {
    /// Visit an expression.
    fn visit_expr(&mut self, _expr: &SurfaceExpr) {}
    /// Visit a declaration.
    fn visit_decl(&mut self, _decl: &Decl) {}
    /// Visit an import.
    fn visit_import(&mut self, _import: &ImportDecl) {}
    /// Called when entering a scope declaration.
    fn enter_scope(&mut self, _scope: &ScopeDecl) {}
    /// Called when leaving a scope declaration.
    fn leave_scope(&mut self, _scope: &ScopeDecl) {}
}
#[cfg(test)]
mod extra_tests {
    use super::*;
    use crate::ast::*;
    #[test]
    fn test_ast_node_kind_display_var() {
        assert_eq!(format!("{}", AstNodeKind::Var), "var");
    }
    #[test]
    fn test_ast_node_kind_display_def() {
        assert_eq!(format!("{}", AstNodeKind::DefDecl), "def");
    }
    #[test]
    fn test_operator_table_register_and_lookup() {
        let mut table = OperatorTable::new();
        let e = NotationEntry::new("+", "HAdd.hAdd", Fixity::InfixLeft, Prec::ADD);
        table.register(e);
        let results = table.lookup("+");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].symbol, "+");
    }
    #[test]
    fn test_operator_table_empty_lookup() {
        let table = OperatorTable::new();
        assert!(table.lookup("*").is_empty());
    }
    #[test]
    fn test_operator_table_len() {
        let mut table = OperatorTable::new();
        table.register(NotationEntry::new("+", "Add", Fixity::InfixLeft, Prec::ADD));
        table.register(NotationEntry::new("*", "Mul", Fixity::InfixLeft, Prec::MUL));
        assert_eq!(table.len(), 2);
    }
    #[test]
    fn test_operator_table_infix_entries() {
        let mut table = OperatorTable::new();
        table.register(NotationEntry::new("+", "Add", Fixity::InfixLeft, Prec::ADD));
        table.register(NotationEntry::new("-", "Neg", Fixity::Prefix, Prec::ADD));
        let infixes = table.infix_entries();
        assert_eq!(infixes.len(), 1);
    }
    #[test]
    fn test_scope_decl_open_display() {
        let s = ScopeDecl::Open(vec!["Nat".to_string(), "List".to_string()]);
        let txt = format!("{}", s);
        assert!(txt.contains("open"));
        assert!(txt.contains("Nat"));
    }
    #[test]
    fn test_notation_entry_display() {
        let e = NotationEntry::new("^", "HPow.hPow", Fixity::InfixRight, Prec::new(75));
        let s = format!("{}", e);
        assert!(s.contains("^"));
        assert!(s.contains("HPow.hPow"));
    }
    #[test]
    fn test_attr_arg_list() {
        let a = AttrArg::List(vec![AttrArg::ident("x"), AttrArg::ident("y")]);
        assert!(matches!(a, AttrArg::List(_)));
    }
    #[test]
    fn test_prec_min() {
        assert_eq!(Prec::MIN.value(), 0);
    }
    #[test]
    fn test_prec_atom() {
        assert_eq!(Prec::ATOM.value(), 1024);
    }
}
#[cfg(test)]
mod new_type_tests {
    use super::*;
    use crate::ast::*;
    #[test]
    fn test_span_dummy() {
        let s = Span::dummy();
        assert!(!s.is_non_empty());
    }
    #[test]
    fn test_span_union() {
        let s1 = Span {
            start: Pos::new(1, 1),
            end: Pos::new(1, 5),
            bytes: ByteRange::new(0, 4),
        };
        let s2 = Span {
            start: Pos::new(1, 3),
            end: Pos::new(1, 8),
            bytes: ByteRange::new(2, 7),
        };
        let u = s1.union(s2);
        assert_eq!(u.start, Pos::new(1, 1));
        assert_eq!(u.bytes.start, 0);
        assert_eq!(u.bytes.end, 7);
    }
    #[test]
    fn test_span_display() {
        let s = Span {
            start: Pos::new(1, 1),
            end: Pos::new(1, 10),
            bytes: ByteRange::new(0, 9),
        };
        let txt = format!("{}", s);
        assert!(txt.contains("1:1"));
        assert!(txt.contains("1:10"));
    }
    #[test]
    fn test_parse_error_display() {
        let e =
            ParseError::new("unexpected token", Pos::new(3, 7)).with_hint("try adding a semicolon");
        let s = e.display();
        assert!(s.contains("unexpected token"));
        assert!(s.contains("3:7"));
        assert!(s.contains("semicolon"));
    }
    #[test]
    fn test_parse_error_fmt() {
        let e = ParseError::new("bad input", Pos::new(1, 1));
        let s = format!("{}", e);
        assert!(s.contains("bad input"));
    }
    #[test]
    fn test_token_kind_tag_display() {
        assert_eq!(format!("{}", TokenKindTag::Ident), "ident");
        assert_eq!(format!("{}", TokenKindTag::Eof), "eof");
        assert_eq!(format!("{}", TokenKindTag::Num), "num");
    }
    #[test]
    fn test_ast_metadata_default() {
        let m = AstMetadata::empty();
        assert!(m.span.is_none());
        assert!(!m.synthetic);
        assert!(m.tags.is_empty());
    }
    #[test]
    fn test_ast_metadata_with_span() {
        let r = ByteRange::new(5, 15);
        let m = AstMetadata::with_span(r);
        assert_eq!(m.span, Some(ByteRange::new(5, 15)));
    }
    #[test]
    fn test_ast_metadata_tags() {
        let mut m = AstMetadata::empty();
        m.add_tag("hover");
        m.add_tag("lsp");
        assert!(m.has_tag("hover"));
        assert!(!m.has_tag("unknown"));
    }
    #[test]
    fn test_namespace_stack_push_pop() {
        let mut ns = NamespaceStack::new();
        assert!(ns.is_top_level());
        ns.push("Foo");
        ns.push("Bar");
        assert_eq!(ns.depth(), 2);
        assert_eq!(ns.current_path(), "Foo.Bar");
        ns.pop();
        assert_eq!(ns.current_path(), "Foo");
    }
    #[test]
    fn test_namespace_stack_qualify() {
        let mut ns = NamespaceStack::new();
        assert_eq!(ns.qualify("myFunc"), "myFunc");
        ns.push("MyNS");
        assert_eq!(ns.qualify("myFunc"), "MyNS.myFunc");
    }
    #[test]
    fn test_macro_expansion_record() {
        let mut exp = MacroExpansion::new("simp", ByteRange::empty());
        assert!(!exp.is_deep());
        for _ in 0..11 {
            exp.increment();
        }
        assert!(exp.is_deep());
        assert_eq!(exp.expansion_steps, 11);
    }
    #[test]
    fn test_macro_expansion_name() {
        let exp = MacroExpansion::new("ring", ByteRange::new(0, 4));
        assert_eq!(exp.macro_name, "ring");
        assert_eq!(exp.expansion_steps, 0);
    }
}
/// A simple visitor trait for AST nodes.
#[allow(dead_code)]
pub trait AstNodeVisitorExt {
    /// Visit a node with the given kind label.
    fn visit_node(&mut self, kind: &str, depth: usize);
}
pub fn flatten_tree_ext(
    node: &TreeNodeExt,
    depth: usize,
    out: &mut Vec<(SimpleNodeKindExt, String, usize)>,
) {
    out.push((node.kind.clone(), node.label.clone(), depth));
    for child in &node.children {
        flatten_tree_ext(child, depth + 1, out);
    }
}
/// A simple free variable collector.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn collect_free_vars_ext(node: &TreeNodeExt, bound: &[String]) -> Vec<String> {
    match node.kind {
        SimpleNodeKindExt::Leaf => {
            if !bound.contains(&node.label)
                && !node.label.is_empty()
                && node.label.chars().all(|c| c.is_alphanumeric() || c == '_')
            {
                vec![node.label.clone()]
            } else {
                Vec::new()
            }
        }
        SimpleNodeKindExt::Lam => {
            let mut new_bound = bound.to_vec();
            new_bound.push(node.label.clone());
            node.children
                .iter()
                .flat_map(|c| collect_free_vars_ext(c, &new_bound))
                .collect()
        }
        _ => node
            .children
            .iter()
            .flat_map(|c| collect_free_vars_ext(c, bound))
            .collect(),
    }
}
/// Computes a fingerprint hash for a tree node.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn tree_fingerprint_ext(node: &TreeNodeExt) -> u64 {
    let mut hash = 14695981039346656037u64;
    let kind_str = format!("{:?}", node.kind);
    for b in kind_str.bytes() {
        hash ^= b as u64;
        hash = hash.wrapping_mul(1099511628211u64);
    }
    for b in node.label.bytes() {
        hash ^= b as u64;
        hash = hash.wrapping_mul(1099511628211u64);
    }
    for child in &node.children {
        let child_hash = tree_fingerprint_ext(child);
        hash ^= child_hash;
        hash = hash.wrapping_mul(1099511628211u64);
    }
    hash
}
#[cfg(test)]
mod ast_ext_tests {
    use super::*;
    use crate::ast::*;
    #[test]
    fn test_with_pos() {
        let wp = WithPosExt::new(42u32, 0, 5);
        assert_eq!(wp.inner, 42);
        let wp2 = wp.map(|x| x * 2);
        assert_eq!(wp2.inner, 84);
    }
    #[test]
    fn test_tree_node_depth() {
        let leaf = TreeNodeExt::leaf("x");
        assert_eq!(leaf.depth(), 0);
        let lam = TreeNodeExt::lam("x", TreeNodeExt::leaf("x"));
        assert_eq!(lam.depth(), 1);
        let app = TreeNodeExt::app(TreeNodeExt::leaf("f"), TreeNodeExt::leaf("x"));
        assert_eq!(app.depth(), 1);
    }
    #[test]
    fn test_tree_node_size() {
        let leaf = TreeNodeExt::leaf("x");
        assert_eq!(leaf.size(), 1);
        let app = TreeNodeExt::app(TreeNodeExt::leaf("f"), TreeNodeExt::leaf("x"));
        assert_eq!(app.size(), 3);
    }
    #[test]
    fn test_counting_visitor() {
        let tree = TreeNodeExt::app(TreeNodeExt::leaf("f"), TreeNodeExt::leaf("x"));
        let mut visitor = CountingVisitorExt::new();
        tree.visit(&mut visitor, 0);
        assert_eq!(visitor.counts.get("App").copied().unwrap_or(0), 1);
        assert_eq!(visitor.counts.get("Leaf").copied().unwrap_or(0), 2);
    }
    #[test]
    fn test_flat_ast() {
        let tree = TreeNodeExt::lam("x", TreeNodeExt::leaf("x"));
        let flat = FlatAstExt::from_tree(&tree);
        assert_eq!(flat.len(), 2);
    }
    #[test]
    fn test_collect_free_vars() {
        let tree = TreeNodeExt::lam("x", TreeNodeExt::leaf("y"));
        let fvs = collect_free_vars_ext(&tree, &[]);
        assert!(fvs.contains(&"y".to_string()));
    }
    #[test]
    fn test_subst_table() {
        let mut subst = SubstTableExt::new();
        subst.add("x", TreeNodeExt::leaf("z"));
        let node = TreeNodeExt::leaf("x");
        let result = subst.apply(&node);
        assert_eq!(result.label, "z");
    }
    #[test]
    fn test_tree_fingerprint_stable() {
        let t1 = TreeNodeExt::leaf("x");
        let t2 = TreeNodeExt::leaf("x");
        assert_eq!(tree_fingerprint_ext(&t1), tree_fingerprint_ext(&t2));
        let t3 = TreeNodeExt::leaf("y");
        assert_ne!(tree_fingerprint_ext(&t1), tree_fingerprint_ext(&t3));
    }
}
#[cfg(test)]
mod ast_ext2_tests {
    use super::*;
    use crate::ast::*;
    #[test]
    fn test_complexity_metric() {
        let leaf = TreeNodeExt::leaf("x");
        let m = ComplexityMetric::default_metric();
        assert_eq!(m.compute(&leaf), 1);
        let app = TreeNodeExt::app(TreeNodeExt::leaf("f"), TreeNodeExt::leaf("x"));
        assert_eq!(m.compute(&app), 4);
    }
    #[test]
    fn test_type_annotation() {
        let ann = TypeAnnotation::new("x", "Nat");
        assert_eq!(ann.format(), "(x : Nat)");
    }
    #[test]
    fn test_universe_level() {
        let u0 = UniverseLevel::Zero;
        assert_eq!(u0.concrete(), Some(0));
        let u1 = UniverseLevel::Succ(Box::new(UniverseLevel::Zero));
        assert_eq!(u1.concrete(), Some(1));
        let umax = UniverseLevel::Max(
            Box::new(UniverseLevel::Succ(Box::new(UniverseLevel::Zero))),
            Box::new(UniverseLevel::Zero),
        );
        assert_eq!(umax.concrete(), Some(1));
        let uvar = UniverseLevel::Var("u".to_string());
        assert_eq!(uvar.concrete(), None);
    }
}
#[cfg(test)]
mod ast_ext3_tests {
    use super::*;
    use crate::ast::*;
    #[test]
    fn test_binder_ext_format() {
        let b = BinderExt::explicit("x", Some("Nat"));
        assert_eq!(b.format(), "(x : Nat)");
        let b2 = BinderExt::implicit("T", Some("Type"));
        assert_eq!(b2.format(), "{T : Type}");
    }
    #[test]
    fn test_telescope() {
        let t = Telescope::new()
            .add(BinderExt::explicit("x", Some("Nat")))
            .add(BinderExt::implicit("T", Some("Type")));
        assert_eq!(t.len(), 2);
        let fmt = t.format();
        assert!(fmt.contains("(x : Nat)"));
    }
    #[test]
    fn test_decl_header_ext() {
        let h = DeclHeaderExt::new("foo")
            .add_param(BinderExt::explicit("n", Some("Nat")))
            .with_return_type("Nat");
        assert_eq!(h.name, "foo");
        assert_eq!(h.params.len(), 1);
        assert_eq!(h.return_type.as_deref(), Some("Nat"));
    }
}
#[cfg(test)]
mod ast_final_tests {
    use super::*;
    use crate::ast::*;
    #[test]
    fn test_let_binding_ext() {
        let lb = LetBindingExt::new("x", "1 + 2", "x * x").with_ty("Nat");
        let fmt = lb.format();
        assert!(fmt.starts_with("let x : Nat"));
        assert!(fmt.contains("1 + 2"));
    }
    #[test]
    fn test_match_expr_ext() {
        let me = MatchExprExt::new("n")
            .add_arm("Nat.zero", "0")
            .add_arm("Nat.succ k", "k + 1");
        let fmt = me.format();
        assert!(fmt.contains("match n"));
        assert!(fmt.contains("| Nat.zero ->"));
        assert_eq!(me.arms.len(), 2);
    }
}
#[cfg(test)]
mod ast_pad {
    use super::*;
    use crate::ast::*;
    #[test]
    fn test_type_synonym() {
        let ts = TypeSynonym::new("Endo", vec!["α"], "α -> α");
        assert!(ts.format().contains("abbrev Endo"));
    }
    #[test]
    fn test_struct_field() {
        let f = StructField::new("width", "Nat").with_default("0");
        assert_eq!(f.name, "width");
        assert_eq!(f.default.as_deref(), Some("0"));
    }
}
#[cfg(test)]
mod ast_pad2 {
    use super::*;
    use crate::ast::*;
    #[test]
    fn test_binder_ext() {
        let b = BinderExtExt2::explicit("x", Some("Nat"));
        assert_eq!(b.format(), "(x : Nat)");
        let b2 = BinderExtExt2::implicit("α", None);
        assert_eq!(b2.format(), "{α}");
    }
    #[test]
    fn test_telescope() {
        let mut t = TelescopeExt2::new();
        t.push(BinderExtExt2::explicit("n", Some("Nat")));
        t.push(BinderExtExt2::implicit("α", Some("Type")));
        assert_eq!(t.len(), 2);
        let formatted = t.format();
        assert!(formatted.contains("(n : Nat)"));
    }
    #[test]
    fn test_let_binding_ext() {
        let lb = LetBindingExtExt2::new("x", Some("Nat"), "42", "x + 1");
        assert!(lb.format().contains("let x : Nat := 42"));
    }
}
#[cfg(test)]
mod ast_pad3 {
    use super::*;
    use crate::ast::*;
    #[test]
    fn test_match_expr_ext() {
        let m = MatchExprExtExt2::new("n", vec![("Nat.zero", "0"), ("Nat.succ k", "k + 1")]);
        assert_eq!(m.arm_count(), 2);
        assert!(m.patterns().contains(&"Nat.zero"));
    }
}
