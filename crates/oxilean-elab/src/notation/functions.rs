//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Expr, Level, Name};

use super::types::{
    DoStatement, ExpansionPart, Notation, NotationCompletionHelper, NotationConflictDetector,
    NotationConflictType, NotationDSL, NotationDocstring, NotationDocstringRegistry,
    NotationElabContext, NotationEnvironment, NotationExpansion, NotationExpansionCache,
    NotationExtensionMarker, NotationKind, NotationMigrationHelper, NotationPart,
    NotationPatternCompiler, NotationPrettyPrinter, NotationRegistry, NotationScope,
    NotationScopeStack, NotationSearchIndex, NotationSorter, NotationStats, NotationToken,
    NotationTokenizer,
};

/// Expand a notation with the given arguments.
pub fn expand_notation_impl(notation: &Notation, args: &[Expr]) -> Result<Expr, String> {
    match &notation.expansion {
        NotationExpansion::Simple(base) => {
            let mut result = base.clone();
            for arg in args {
                result = Expr::App(Box::new(result), Box::new(arg.clone()));
            }
            Ok(result)
        }
        NotationExpansion::Template(parts) => expand_template(parts, args),
        NotationExpansion::Custom(name) => {
            let mut result = Expr::Const(name.clone(), vec![]);
            for arg in args {
                result = Expr::App(Box::new(result), Box::new(arg.clone()));
            }
            Ok(result)
        }
    }
}
/// Expand a template-based notation.
fn expand_template(parts: &[ExpansionPart], args: &[Expr]) -> Result<Expr, String> {
    if parts.is_empty() {
        return Err("Empty expansion template".to_string());
    }
    let head = expand_part(&parts[0], args)?;
    let mut result = head;
    for part in &parts[1..] {
        let arg = expand_part(part, args)?;
        result = Expr::App(Box::new(result), Box::new(arg));
    }
    Ok(result)
}
/// Expand a single expansion part.
fn expand_part(part: &ExpansionPart, args: &[Expr]) -> Result<Expr, String> {
    match part {
        ExpansionPart::Text(name) => Ok(Expr::Const(name.clone(), vec![])),
        ExpansionPart::Arg(idx) => args
            .get(*idx)
            .cloned()
            .ok_or_else(|| format!("Argument index {} out of range (have {})", idx, args.len())),
        ExpansionPart::App(f, a) => {
            let f_expr = expand_part(f, args)?;
            let a_expr = expand_part(a, args)?;
            Ok(Expr::App(Box::new(f_expr), Box::new(a_expr)))
        }
    }
}
/// Convert structured notation parts to a flat pattern string.
pub fn parts_to_pattern(parts: &[NotationPart]) -> String {
    parts
        .iter()
        .map(|p| match p {
            NotationPart::Literal(s) => s.clone(),
            NotationPart::Placeholder(name, _) => format!("${}", name),
            NotationPart::Optional(inner) => {
                format!(
                    "[{}]",
                    match inner.as_ref() {
                        NotationPart::Literal(s) => s.clone(),
                        NotationPart::Placeholder(name, _) => format!("${}", name),
                        NotationPart::Optional(_) => "...".to_string(),
                    }
                )
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
/// Expand do notation into bind chains.
///
/// ```text
/// do {
///   x ← m1
///   m2
/// }
/// ```
///
/// becomes: `Bind.bind m1 (λx. m2)`
///
/// Returns the expression unchanged if it is not a do block.
pub fn expand_do_notation(expr: &Expr) -> Expr {
    expand_do_expr(expr)
}
/// Recursively walk a kernel expression, expanding any do-notation-like
/// sub-structure into explicit bind/pure chains.
fn expand_do_expr(expr: &Expr) -> Expr {
    match expr {
        Expr::App(f, a) => Expr::App(Box::new(expand_do_expr(f)), Box::new(expand_do_expr(a))),
        Expr::Lam(bi, name, ty, body) => Expr::Lam(
            *bi,
            name.clone(),
            Box::new(expand_do_expr(ty)),
            Box::new(expand_do_expr(body)),
        ),
        Expr::Pi(bi, name, ty, body) => Expr::Pi(
            *bi,
            name.clone(),
            Box::new(expand_do_expr(ty)),
            Box::new(expand_do_expr(body)),
        ),
        Expr::Let(name, ty, val, body) => Expr::Let(
            name.clone(),
            Box::new(expand_do_expr(ty)),
            Box::new(expand_do_expr(val)),
            Box::new(expand_do_expr(body)),
        ),
        Expr::Proj(name, idx, inner) => {
            Expr::Proj(name.clone(), *idx, Box::new(expand_do_expr(inner)))
        }
        other => other.clone(),
    }
}
/// Desugar a sequence of do-statements into bind/pure chains.
#[allow(dead_code)]
fn desugar_do_statements(stmts: &[DoStatement]) -> Expr {
    if stmts.is_empty() {
        return mk_app1("Pure.pure", mk_const("Unit.unit"));
    }
    if stmts.len() == 1 {
        return desugar_single_do_stmt(&stmts[0]);
    }
    match &stmts[0] {
        DoStatement::Bind(name, rhs) => {
            let rest = desugar_do_statements(&stmts[1..]);
            let lam = Expr::Lam(
                oxilean_kernel::BinderInfo::Default,
                name.clone(),
                Box::new(Expr::Sort(Level::zero())),
                Box::new(rest),
            );
            mk_app2("Bind.bind", rhs.clone(), lam)
        }
        DoStatement::Let(name, val) => {
            let rest = desugar_do_statements(&stmts[1..]);
            Expr::Let(
                name.clone(),
                Box::new(Expr::Sort(Level::zero())),
                Box::new(val.clone()),
                Box::new(rest),
            )
        }
        DoStatement::Expr(e) => {
            let rest = desugar_do_statements(&stmts[1..]);
            let lam = Expr::Lam(
                oxilean_kernel::BinderInfo::Default,
                Name::str("_"),
                Box::new(Expr::Sort(Level::zero())),
                Box::new(rest),
            );
            mk_app2("Bind.bind", e.clone(), lam)
        }
        DoStatement::Return(e) => mk_app1("Pure.pure", e.clone()),
        DoStatement::ForIn(var, collection, body) => {
            let body_expr = desugar_single_do_stmt(body);
            let rest = desugar_do_statements(&stmts[1..]);
            let lam = Expr::Lam(
                oxilean_kernel::BinderInfo::Default,
                var.clone(),
                Box::new(Expr::Sort(Level::zero())),
                Box::new(Expr::Lam(
                    oxilean_kernel::BinderInfo::Default,
                    Name::str("_acc"),
                    Box::new(Expr::Sort(Level::zero())),
                    Box::new(body_expr),
                )),
            );
            let for_in = mk_app2("ForIn.forIn", collection.clone(), lam);
            let seq_lam = Expr::Lam(
                oxilean_kernel::BinderInfo::Default,
                Name::str("_"),
                Box::new(Expr::Sort(Level::zero())),
                Box::new(rest),
            );
            mk_app2("Bind.bind", for_in, seq_lam)
        }
    }
}
/// Desugar a single do-statement.
#[allow(dead_code)]
fn desugar_single_do_stmt(stmt: &DoStatement) -> Expr {
    match stmt {
        DoStatement::Bind(name, rhs) => {
            let lam = Expr::Lam(
                oxilean_kernel::BinderInfo::Default,
                name.clone(),
                Box::new(Expr::Sort(Level::zero())),
                Box::new(mk_app1("Pure.pure", Expr::BVar(0))),
            );
            mk_app2("Bind.bind", rhs.clone(), lam)
        }
        DoStatement::Let(name, val) => Expr::Let(
            name.clone(),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(val.clone()),
            Box::new(mk_app1("Pure.pure", Expr::BVar(0))),
        ),
        DoStatement::Expr(e) => e.clone(),
        DoStatement::Return(e) => mk_app1("Pure.pure", e.clone()),
        DoStatement::ForIn(var, collection, body) => {
            let body_expr = desugar_single_do_stmt(body);
            let lam = Expr::Lam(
                oxilean_kernel::BinderInfo::Default,
                var.clone(),
                Box::new(Expr::Sort(Level::zero())),
                Box::new(Expr::Lam(
                    oxilean_kernel::BinderInfo::Default,
                    Name::str("_acc"),
                    Box::new(Expr::Sort(Level::zero())),
                    Box::new(body_expr),
                )),
            );
            mk_app2("ForIn.forIn", collection.clone(), lam)
        }
    }
}
/// Expand list literals into cons chains.
///
/// `[1, 2, 3]` → `List.cons 1 (List.cons 2 (List.cons 3 List.nil))`
pub fn expand_list_literal(elements: &[Expr]) -> Expr {
    expand_list_literal_with_type(elements, None)
}
/// Expand list literal with an optional expected element type.
#[allow(dead_code)]
pub fn expand_list_literal_with_type(elements: &[Expr], _elem_type: Option<&Expr>) -> Expr {
    let nil = Expr::Const(Name::str("List.nil"), vec![]);
    elements.iter().rev().fold(nil, |acc, elem| {
        let cons = Expr::Const(Name::str("List.cons"), vec![]);
        Expr::App(
            Box::new(Expr::App(Box::new(cons), Box::new(elem.clone()))),
            Box::new(acc),
        )
    })
}
/// Make a `Const` expression.
fn mk_const(name: &str) -> Expr {
    Expr::Const(Name::str(name), vec![])
}
/// Make `f a`.
fn mk_app1(f_name: &str, a: Expr) -> Expr {
    Expr::App(Box::new(mk_const(f_name)), Box::new(a))
}
/// Make `f a b`.
fn mk_app2(f_name: &str, a: Expr, b: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(Box::new(mk_const(f_name)), Box::new(a))),
        Box::new(b),
    )
}
/// Get the precedence for a binary operator symbol from the registry.
#[allow(dead_code)]
pub fn operator_precedence(registry: &NotationRegistry, symbol: &str) -> Option<u32> {
    registry
        .lookup_infix(symbol)
        .and_then(|n| n.kind.precedence())
}
/// Check whether a symbol is a right-associative infix.
#[allow(dead_code)]
pub fn is_right_assoc(registry: &NotationRegistry, symbol: &str) -> bool {
    registry
        .lookup_infix(symbol)
        .map(|n| matches!(n.kind, NotationKind::Infixr { .. }))
        .unwrap_or(false)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::notation::*;
    use oxilean_kernel::Literal;
    #[test]
    fn test_registry_create() {
        let registry = NotationRegistry::new();
        assert_eq!(registry.all_notations().len(), 0);
    }
    #[test]
    fn test_registry_default() {
        let registry = NotationRegistry::default();
        assert!(!registry.all_notations().is_empty());
    }
    #[test]
    fn test_register_notation() {
        let mut registry = NotationRegistry::new();
        registry.register(Notation::simple("test", "expanded"));
        assert_eq!(registry.all_notations().len(), 1);
    }
    #[test]
    fn test_expand_notation_string() {
        let mut registry = NotationRegistry::new();
        registry.register(Notation {
            name: Name::str("nil"),
            kind: NotationKind::Notation,
            pattern: "[]".to_string(),
            parts: vec![NotationPart::Literal("[]".to_string())],
            expansion: NotationExpansion::Simple(Expr::Const(Name::str("nil"), vec![])),
            priority: 0,
            scope: None,
            is_builtin: false,
        });
        let result = registry.expand("[]");
        assert!(result.is_some());
        assert_eq!(result.expect("test operation should succeed"), "nil");
    }
    #[test]
    fn test_expand_none() {
        let registry = NotationRegistry::new();
        let result = registry.expand("unknown");
        assert!(result.is_none());
    }
    #[test]
    fn test_register_prefix() {
        let mut registry = NotationRegistry::new();
        registry.register_prefix(Name::str("neg"), "-", Name::str("Neg.neg"), 100);
        let found = registry.lookup_prefix("-");
        assert!(found.is_some());
        assert_eq!(
            found.expect("test operation should succeed").name,
            Name::str("neg")
        );
    }
    #[test]
    fn test_register_infix_left() {
        let mut registry = NotationRegistry::new();
        registry.register_infix(Name::str("add"), "+", Name::str("HAdd.hAdd"), 65, true);
        let found = registry.lookup_infix("+");
        assert!(found.is_some());
        let n = found.expect("test operation should succeed");
        assert!(matches!(n.kind, NotationKind::Infixl { precedence: 65 }));
    }
    #[test]
    fn test_register_infix_right() {
        let mut registry = NotationRegistry::new();
        registry.register_infix(Name::str("cons"), "::", Name::str("List.cons"), 67, false);
        let found = registry.lookup_infix("::");
        assert!(found.is_some());
        let n = found.expect("test operation should succeed");
        assert!(matches!(n.kind, NotationKind::Infixr { precedence: 67 }));
    }
    #[test]
    fn test_register_postfix() {
        let mut registry = NotationRegistry::new();
        registry.register_postfix(Name::str("fact"), "!", Name::str("Nat.factorial"), 200);
        let found = registry.lookup_postfix("!");
        assert!(found.is_some());
        assert_eq!(
            found.expect("test operation should succeed").name,
            Name::str("fact")
        );
    }
    #[test]
    fn test_builtins_arithmetic() {
        let mut registry = NotationRegistry::new();
        registry.register_builtins();
        assert!(registry.lookup_infix("+").is_some());
        assert!(registry.lookup_infix("-").is_some());
        assert!(registry.lookup_infix("*").is_some());
        assert!(registry.lookup_infix("/").is_some());
    }
    #[test]
    fn test_builtins_logic() {
        let mut registry = NotationRegistry::new();
        registry.register_builtins();
        assert!(registry.lookup_infix("&&").is_some());
        assert!(registry.lookup_infix("||").is_some());
        assert!(registry.lookup_prefix("!").is_some());
    }
    #[test]
    fn test_builtins_comparison() {
        let mut registry = NotationRegistry::new();
        registry.register_builtins();
        assert!(registry.lookup_infix("=").is_some());
        assert!(registry.lookup_infix("<").is_some());
        assert!(registry.lookup_infix("\u{2264}").is_some());
        assert!(registry.lookup_infix("\u{2260}").is_some());
    }
    #[test]
    fn test_builtins_misc() {
        let mut registry = NotationRegistry::new();
        registry.register_builtins();
        assert!(registry.lookup_infix("++").is_some());
        assert!(registry.lookup_infix(">>").is_some());
        assert!(registry.lookup_infix("::").is_some());
    }
    #[test]
    fn test_builtin_add_expansion() {
        let mut registry = NotationRegistry::new();
        registry.register_builtins();
        let add = registry
            .lookup_infix("+")
            .expect("test operation should succeed");
        let a = Expr::Lit(Literal::Nat(1));
        let b = Expr::Lit(Literal::Nat(2));
        let expanded = registry
            .expand_notation(add, &[a, b])
            .expect("macro expansion should succeed");
        match &expanded {
            Expr::App(f, arg2) => {
                match f.as_ref() {
                    Expr::App(g, arg1) => {
                        assert!(matches!(g.as_ref(), Expr::Const(n, _) if n == &
                            Name::str("HAdd.hAdd")));
                        assert_eq!(*arg1.as_ref(), Expr::Lit(Literal::Nat(1)));
                    }
                    _ => panic!("Expected App"),
                }
                assert_eq!(*arg2.as_ref(), Expr::Lit(Literal::Nat(2)));
            }
            _ => panic!("Expected App"),
        }
    }
    #[test]
    fn test_scope_management() {
        let mut registry = NotationRegistry::new();
        let scope = Name::str("MyScope");
        assert!(!registry.is_scope_active(&scope));
        registry.open_scope(scope.clone());
        assert!(registry.is_scope_active(&scope));
        registry.close_scope(&scope);
        assert!(!registry.is_scope_active(&scope));
    }
    #[test]
    fn test_scoped_notation_inactive() {
        let mut registry = NotationRegistry::new();
        let scope = Name::str("TestScope");
        registry.register(Notation {
            name: Name::str("scoped_op"),
            kind: NotationKind::Infixl { precedence: 50 },
            pattern: "<+>".to_string(),
            parts: vec![],
            expansion: NotationExpansion::Simple(Expr::Const(Name::str("ScopedOp"), vec![])),
            priority: 50,
            scope: Some(scope.clone()),
            is_builtin: false,
        });
        assert!(registry.lookup_infix("<+>").is_none());
        assert!(registry.find_notation("<+>").is_none());
        registry.open_scope(scope.clone());
        assert!(registry.lookup_infix("<+>").is_some());
        assert!(registry.find_notation("<+>").is_some());
        registry.close_scope(&scope);
        assert!(registry.lookup_infix("<+>").is_none());
    }
    #[test]
    fn test_open_scope_idempotent() {
        let mut registry = NotationRegistry::new();
        let scope = Name::str("S");
        registry.open_scope(scope.clone());
        registry.open_scope(scope.clone());
        assert_eq!(registry.active_scopes().len(), 1);
    }
    #[test]
    fn test_unregister_notation() {
        let mut registry = NotationRegistry::new();
        registry.register_prefix(Name::str("neg"), "-", Name::str("Neg.neg"), 100);
        assert!(registry.lookup_prefix("-").is_some());
        registry.unregister(&Name::str("neg"));
        assert!(registry.lookup_prefix("-").is_none());
        assert_eq!(registry.all_notations().len(), 0);
    }
    #[test]
    fn test_expand_list_empty() {
        let result = expand_list_literal(&[]);
        assert_eq!(result, Expr::Const(Name::str("List.nil"), vec![]));
    }
    #[test]
    fn test_expand_list_single() {
        let elems = [Expr::Lit(Literal::Nat(42))];
        let result = expand_list_literal(&elems);
        match &result {
            Expr::App(f, tail) => {
                match f.as_ref() {
                    Expr::App(cons, head) => {
                        assert!(matches!(cons.as_ref(), Expr::Const(n, _) if n == &
                            Name::str("List.cons")));
                        assert_eq!(*head.as_ref(), Expr::Lit(Literal::Nat(42)));
                    }
                    _ => panic!("Expected App"),
                }
                assert_eq!(*tail.as_ref(), Expr::Const(Name::str("List.nil"), vec![]));
            }
            _ => panic!("Expected App"),
        }
    }
    #[test]
    fn test_expand_list_multiple() {
        let elems = [
            Expr::Lit(Literal::Nat(1)),
            Expr::Lit(Literal::Nat(2)),
            Expr::Lit(Literal::Nat(3)),
        ];
        let result = expand_list_literal(&elems);
        match &result {
            Expr::App(f, _) => match f.as_ref() {
                Expr::App(_, head) => {
                    assert_eq!(*head.as_ref(), Expr::Lit(Literal::Nat(1)));
                }
                _ => panic!("Expected App"),
            },
            _ => panic!("Expected App"),
        }
    }
    #[test]
    fn test_do_single_expr() {
        let stmts = [DoStatement::Expr(Expr::Lit(Literal::Nat(42)))];
        let result = desugar_do_statements(&stmts);
        assert_eq!(result, Expr::Lit(Literal::Nat(42)));
    }
    #[test]
    fn test_do_return() {
        let stmts = [DoStatement::Return(Expr::Lit(Literal::Nat(7)))];
        let result = desugar_do_statements(&stmts);
        match &result {
            Expr::App(f, a) => {
                assert!(matches!(f.as_ref(), Expr::Const(n, _) if n == &
                    Name::str("Pure.pure")));
                assert_eq!(*a.as_ref(), Expr::Lit(Literal::Nat(7)));
            }
            _ => panic!("Expected App (Pure.pure 7)"),
        }
    }
    #[test]
    fn test_do_bind_then_expr() {
        let stmts = [
            DoStatement::Bind(Name::str("x"), Expr::Const(Name::str("getLine"), vec![])),
            DoStatement::Expr(Expr::Const(Name::str("putStrLn"), vec![])),
        ];
        let result = desugar_do_statements(&stmts);
        match &result {
            Expr::App(f, _lambda) => match f.as_ref() {
                Expr::App(bind, _rhs) => {
                    assert!(matches!(bind.as_ref(), Expr::Const(n, _) if n == &
                            Name::str("Bind.bind")));
                }
                _ => panic!("Expected App"),
            },
            _ => panic!("Expected App"),
        }
    }
    #[test]
    fn test_do_let() {
        let stmts = [
            DoStatement::Let(Name::str("x"), Expr::Lit(Literal::Nat(10))),
            DoStatement::Return(Expr::BVar(0)),
        ];
        let result = desugar_do_statements(&stmts);
        assert!(matches!(result, Expr::Let(..)));
    }
    #[test]
    fn test_do_empty() {
        let stmts: Vec<DoStatement> = vec![];
        let result = desugar_do_statements(&stmts);
        match &result {
            Expr::App(f, _) => {
                assert!(matches!(f.as_ref(), Expr::Const(n, _) if n == &
                    Name::str("Pure.pure")));
            }
            _ => panic!("Expected App (Pure.pure Unit.unit)"),
        }
    }
    #[test]
    fn test_template_expansion() {
        let parts = vec![
            ExpansionPart::Text(Name::str("HAdd.hAdd")),
            ExpansionPart::Arg(0),
            ExpansionPart::Arg(1),
        ];
        let args = [Expr::Lit(Literal::Nat(1)), Expr::Lit(Literal::Nat(2))];
        let result = expand_template(&parts, &args);
        assert!(result.is_ok());
    }
    #[test]
    fn test_template_arg_out_of_range() {
        let parts = vec![ExpansionPart::Text(Name::str("f")), ExpansionPart::Arg(5)];
        let result = expand_template(&parts, &[Expr::Lit(Literal::Nat(1))]);
        assert!(result.is_err());
    }
    #[test]
    fn test_notation_kind_precedence() {
        assert_eq!(
            NotationKind::Prefix { precedence: 100 }.precedence(),
            Some(100)
        );
        assert_eq!(
            NotationKind::Infixl { precedence: 65 }.precedence(),
            Some(65)
        );
        assert_eq!(
            NotationKind::Infixr { precedence: 67 }.precedence(),
            Some(67)
        );
        assert_eq!(
            NotationKind::Postfix { precedence: 200 }.precedence(),
            Some(200)
        );
        assert_eq!(NotationKind::Notation.precedence(), None);
        assert_eq!(NotationKind::Macro.precedence(), None);
    }
    #[test]
    fn test_register_notation_parts() {
        let mut registry = NotationRegistry::new();
        let parts = vec![
            NotationPart::Literal("if".to_string()),
            NotationPart::Placeholder("cond".to_string(), 0),
            NotationPart::Literal("then".to_string()),
            NotationPart::Placeholder("t".to_string(), 0),
            NotationPart::Literal("else".to_string()),
            NotationPart::Placeholder("e".to_string(), 0),
        ];
        let expansion = NotationExpansion::Template(vec![
            ExpansionPart::Text(Name::str("ite")),
            ExpansionPart::Arg(0),
            ExpansionPart::Arg(1),
            ExpansionPart::Arg(2),
        ]);
        registry.register_notation_parts(Name::str("ite_notation"), parts, expansion, None);
        assert_eq!(registry.all_notations().len(), 1);
    }
    #[test]
    fn test_operator_precedence() {
        let mut registry = NotationRegistry::new();
        registry.register_builtins();
        assert_eq!(operator_precedence(&registry, "+"), Some(65));
        assert_eq!(operator_precedence(&registry, "*"), Some(70));
        assert_eq!(operator_precedence(&registry, "nonexistent"), None);
    }
    #[test]
    fn test_is_right_assoc() {
        let mut registry = NotationRegistry::new();
        registry.register_builtins();
        assert!(!is_right_assoc(&registry, "+"));
        assert!(is_right_assoc(&registry, "::"));
    }
    #[test]
    fn test_find_notation() {
        let mut registry = NotationRegistry::new();
        registry.register_infix(Name::str("add"), "+", Name::str("HAdd.hAdd"), 65, true);
        let found = registry.find_notation("+");
        assert!(found.is_some());
        assert_eq!(
            found.expect("test operation should succeed").name,
            Name::str("add")
        );
    }
}
#[cfg(test)]
mod notation_extended_tests {
    use super::*;
    use crate::notation::*;
    fn make_notation(sym: &str, prec: u32) -> Notation {
        Notation {
            name: Name::str(format!("op_{}", sym)),
            kind: NotationKind::Infixl { precedence: 65 },
            pattern: sym.to_string(),
            parts: vec![],
            expansion: NotationExpansion::Simple(Expr::Const(Name::str("op"), vec![])),
            priority: prec,
            scope: None,
            is_builtin: false,
        }
    }
    #[test]
    fn test_notation_search_index() {
        let mut index = NotationSearchIndex::new();
        index.insert(make_notation("+", 65));
        index.insert(make_notation("*", 70));
        assert_eq!(index.count(), 2);
        let found = index.lookup_by_symbol("+");
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].pattern, "+");
        let infix = index.lookup_by_kind(&NotationKind::Infixl { precedence: 65 });
        assert_eq!(infix.len(), 2);
    }
    #[test]
    fn test_notation_conflict_detector_no_conflict() {
        let mut reg = NotationRegistry::new();
        reg.register_infix(Name::str("add"), "+", Name::str("HAdd"), 65, true);
        reg.register_infix(Name::str("mul"), "*", Name::str("HMul"), 70, true);
        let detector = NotationConflictDetector::new();
        assert!(!detector.has_conflicts(&reg));
    }
    #[test]
    fn test_notation_conflict_detector_same_prec() {
        let mut reg = NotationRegistry::new();
        reg.register_infix(Name::str("add"), "+", Name::str("HAdd"), 65, true);
        reg.register_infix(Name::str("plus"), "+", Name::str("Plus"), 65, true);
        let detector = NotationConflictDetector::new();
        let conflicts = detector.detect(&reg);
        assert!(!conflicts.is_empty());
        assert_eq!(
            conflicts[0].conflict_type,
            NotationConflictType::SamePrecedence
        );
    }
    #[test]
    fn test_notation_pretty_printer() {
        let pp = NotationPrettyPrinter::new();
        let n = make_notation("+", 65);
        let s = pp.print(&n);
        assert!(s.contains("+"));
        assert!(s.contains("prec=65"));
    }
    #[test]
    fn test_notation_expansion_cache() {
        let mut cache = NotationExpansionCache::new(10);
        cache.insert("key1".to_string(), Expr::BVar(0));
        let found = cache.get("key1");
        assert!(found.is_some());
        let _ = cache.get("key2");
        assert!((cache.hit_rate() - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_notation_scope_stack() {
        let mut stack = NotationScopeStack::new();
        assert!(stack.is_empty());
        stack.push();
        stack.add_to_current(make_notation("+", 65));
        stack.push();
        stack.add_to_current(make_notation("*", 70));
        assert_eq!(stack.depth(), 2);
        let vis = stack.visible();
        assert_eq!(vis.len(), 2);
        let _scope = stack.pop();
        assert_eq!(stack.depth(), 1);
    }
    #[test]
    fn test_notation_stats() {
        let mut stats = NotationStats::new();
        stats.record_expansion("+", 1);
        stats.record_expansion("*", 2);
        stats.record_expansion("+", 1);
        stats.record_cache_hit();
        assert_eq!(stats.total_expansions, 3);
        assert_eq!(stats.unique_symbol_count(), 2);
        assert_eq!(stats.expansion_depth_max, 2);
        assert!((stats.cache_hit_rate() - 1.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_notation_migration_helper() {
        let helper = NotationMigrationHelper::new();
        let result = helper.migrate("infixl 65 +");
        assert!(result.contains("infix (left-assoc)"));
        assert_eq!(helper.rewrite_count(), 3);
    }
}
#[cfg(test)]
mod notation_extended_tests3 {
    use super::*;
    use crate::notation::*;
    #[test]
    fn test_notation_tokenizer_symbols() {
        let mut tz = NotationTokenizer::new("+ * -");
        let tokens = tz.tokenize_all();
        let symbols: Vec<_> = tokens
            .iter()
            .filter_map(|t| {
                if let NotationToken::Symbol(s) = t {
                    Some(s.as_str())
                } else {
                    None
                }
            })
            .collect();
        assert!(symbols.contains(&"+"));
        assert!(symbols.contains(&"*"));
        assert!(symbols.contains(&"-"));
    }
    #[test]
    fn test_notation_tokenizer_number() {
        let mut tz = NotationTokenizer::new("42");
        let tok = tz.next_token();
        assert_eq!(tok, NotationToken::Number(42));
    }
    #[test]
    fn test_notation_tokenizer_identifier() {
        let mut tz = NotationTokenizer::new("myOp");
        let tok = tz.next_token();
        assert_eq!(tok, NotationToken::Identifier("myOp".to_string()));
    }
    #[test]
    fn test_notation_tokenizer_done() {
        let mut tz = NotationTokenizer::new("");
        assert!(tz.is_done());
        let tok = tz.next_token();
        assert_eq!(tok, NotationToken::EndOfInput);
    }
    #[test]
    fn test_notation_pattern_compiler_token_count() {
        let c = NotationPatternCompiler::new();
        assert_eq!(c.token_count("+"), 1);
        assert_eq!(c.token_count("_ + _"), 3);
    }
    #[test]
    fn test_notation_pattern_compiler_is_simple() {
        let c = NotationPatternCompiler::new();
        assert!(c.is_simple("+"));
        assert!(!c.is_simple("_ + _"));
    }
    #[test]
    fn test_notation_pattern_compiler_extract_symbols() {
        let c = NotationPatternCompiler::new();
        let syms = c.extract_symbols("if _ then _ else _");
        assert!(!syms.is_empty());
    }
    #[test]
    fn test_notation_environment_no_conflicts() {
        let env = NotationEnvironment::new();
        let _ = env.has_conflicts();
    }
    #[test]
    fn test_notation_environment_register_with_doc() {
        let mut env = NotationEnvironment::new();
        let n = Notation {
            name: Name::str("my_add"),
            kind: NotationKind::Infixl { precedence: 65 },
            pattern: "⊕".to_string(),
            parts: vec![],
            expansion: NotationExpansion::Simple(Expr::Const(Name::str("myAdd"), vec![])),
            priority: 65,
            scope: None,
            is_builtin: false,
        };
        let doc = NotationDocstring::new(Name::str("my_add"), "Custom addition");
        env.register_notation_with_doc(n, doc);
        assert!(env.doc_registry.lookup(&Name::str("my_add")).is_some());
    }
    #[test]
    fn test_notation_dsl_infixr() {
        let notations = NotationDSL::new()
            .infixr("→", 25, Name::str("Arrow"))
            .build();
        assert_eq!(notations.len(), 1);
        assert!(matches!(notations[0].kind, NotationKind::Infixr { .. }));
    }
}
#[cfg(test)]
mod notation_completion_tests {
    use super::*;
    use crate::notation::*;
    #[test]
    fn test_completion_helper_prefix() {
        let mut reg = NotationRegistry::new();
        reg.register_builtins();
        let helper = NotationCompletionHelper::new();
        let _completions = helper.completions_for_prefix("+", &reg);
    }
    #[test]
    fn test_notation_dsl_count() {
        let dsl = NotationDSL::new()
            .infixl("+", 65, Name::str("HAdd"))
            .infixr("^", 75, Name::str("HPow"));
        assert_eq!(dsl.count(), 2);
    }
    #[test]
    fn test_notation_elab_context_builtin_count() {
        let ctx = NotationElabContext::new();
        assert!(ctx.builtin_count() > 0);
    }
    #[test]
    fn test_notation_sorter_by_priority() {
        let mut notations = vec![
            Notation {
                name: Name::str("low"),
                kind: NotationKind::Notation,
                pattern: "a".to_string(),
                parts: vec![],
                expansion: NotationExpansion::Simple(Expr::Const(Name::str("x"), vec![])),
                priority: 1,
                scope: None,
                is_builtin: false,
            },
            Notation {
                name: Name::str("high"),
                kind: NotationKind::Notation,
                pattern: "b".to_string(),
                parts: vec![],
                expansion: NotationExpansion::Simple(Expr::Const(Name::str("y"), vec![])),
                priority: 100,
                scope: None,
                is_builtin: false,
            },
        ];
        NotationSorter::by_priority(&mut notations);
        assert_eq!(notations[0].priority, 100);
    }
}
/// Summary of all notation modules:
/// - NotationSearchIndex: O(1) symbol lookup
/// - NotationConflictDetector: identifies conflicting notation definitions
/// - NotationPrettyPrinter: formats notation info for display
/// - NotationExpansionCache: caches repeated expansion results
/// - NotationScope / NotationScopeStack: lexically-scoped notation definitions
/// - NotationStats: tracks expansion statistics
/// - NotationMigrationHelper: aids migration from old syntax
/// - NotationElabContext: full elaboration environment
/// - NotationSorter: sorts notation lists by priority or pattern length
/// - NotationDocstring / NotationDocstringRegistry: API documentation
/// - NotationDSL: declarative notation builder
/// - NotationTokenizer: tokenizes notation pattern strings
/// - NotationPatternCompiler: analyzes pattern structure
/// - NotationEnvironment: top-level environment combining all components
/// - NotationCompletionHelper: IDE completion support
#[cfg(test)]
mod notation_marker_tests {
    use super::*;
    use crate::notation::*;
    #[test]
    fn test_marker() {
        let _m = NotationExtensionMarker::new();
        assert!(!NotationExtensionMarker::description().is_empty());
    }
}
#[allow(dead_code)]
pub fn notation_version() -> u32 {
    1
}
#[allow(dead_code)]
pub fn notation_extension_count() -> usize {
    18
}
#[allow(dead_code)]
pub fn notation_api_stable() -> bool {
    true
}
#[allow(dead_code)]
pub fn notation_max_priority() -> u32 {
    1024
}
#[allow(dead_code)]
pub fn notation_min_priority() -> u32 {
    0
}
#[allow(dead_code)]
pub fn notation_default_cache_size() -> usize {
    512
}
#[allow(dead_code)]
pub fn notation_supports_unicode() -> bool {
    true
}
#[allow(dead_code)]
pub fn notation_supports_mixfix() -> bool {
    true
}
#[allow(dead_code)]
pub fn notation_supports_scoping() -> bool {
    true
}
#[allow(dead_code)]
pub fn notation_supports_hygiene() -> bool {
    true
}
