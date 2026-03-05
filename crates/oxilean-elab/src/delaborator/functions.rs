//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Environment, Expr, FVarId, Level, Name};
use oxilean_parse::{Binder, BinderKind, Located, Span, SurfaceExpr};
use std::collections::{HashMap, HashSet};

use super::types::{
    AbbreviationDetector, CollectedBinder, ContextualDelaborator, DeclDelaborator, DelabCache,
    DelabConfig, DelabContext, Delaborator, NotationEntry, NotationEntryExt, NotationRegistry,
    NotationTable, Prec, PrintOptions, PrintOptionsExt, SurfacePrinter,
};

/// Helper: check if a Name matches a dot-separated string.
pub fn name_is(name: &Name, target: &str) -> bool {
    format!("{}", name) == target
}
/// Create a `Located<SurfaceExpr>` with a dummy span.
pub fn mk_located(expr: SurfaceExpr) -> Located<SurfaceExpr> {
    Located {
        value: expr,
        span: Span {
            start: 0,
            end: 0,
            line: 0,
            column: 0,
        },
    }
}
/// Convert kernel BinderInfo to surface BinderKind.
pub fn binder_info_to_kind(bi: BinderInfo) -> BinderKind {
    match bi {
        BinderInfo::Default => BinderKind::Default,
        BinderInfo::Implicit => BinderKind::Implicit,
        BinderInfo::StrictImplicit => BinderKind::StrictImplicit,
        BinderInfo::InstImplicit => BinderKind::Instance,
    }
}
/// Collect the head function and all arguments from a curried application.
pub fn collect_app_args<'a>(f: &'a Expr, last_arg: &'a Expr) -> (&'a Expr, Vec<&'a Expr>) {
    let mut args = vec![last_arg];
    let mut current = f;
    while let Expr::App(inner_f, inner_arg) = current {
        args.push(inner_arg);
        current = inner_f;
    }
    args.reverse();
    (current, args)
}
/// Check if an expression contains a loose bound variable at the given index.
pub fn has_loose_bvar(expr: &Expr, target: u32) -> bool {
    match expr {
        Expr::BVar(idx) => *idx == target,
        Expr::Sort(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => false,
        Expr::App(f, arg) => has_loose_bvar(f, target) || has_loose_bvar(arg, target),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            has_loose_bvar(ty, target) || has_loose_bvar(body, target + 1)
        }
        Expr::Let(_, ty, val, body) => {
            has_loose_bvar(ty, target)
                || has_loose_bvar(val, target)
                || has_loose_bvar(body, target + 1)
        }
        Expr::Proj(_, _, e) => has_loose_bvar(e, target),
    }
}
/// Print a delaborated surface expression to a string.
pub fn print_surface_expr(expr: &SurfaceExpr, opts: &PrintOptions) -> String {
    let mut printer = SurfacePrinter::new(opts.clone());
    printer.print_expr(expr);
    printer.output
}
/// Delaborate a kernel expression to a pretty-printed string.
pub fn delab_to_string(env: &Environment, expr: &Expr) -> String {
    let mut ctx = DelabContext::new(env);
    let surface = Delaborator::delab(&mut ctx, expr);
    let opts = PrintOptions::default();
    print_surface_expr(&surface.value, &opts)
}
/// Delaborate with custom configuration.
pub fn delab_to_string_with_config(env: &Environment, expr: &Expr, config: DelabConfig) -> String {
    let mut ctx = DelabContext::with_config(env, config);
    let surface = Delaborator::delab(&mut ctx, expr);
    let opts = PrintOptions::default();
    print_surface_expr(&surface.value, &opts)
}
/// Delaborate a kernel expression to a surface expression.
pub fn delab_expr(env: &Environment, expr: &Expr) -> Located<SurfaceExpr> {
    let mut ctx = DelabContext::new(env);
    Delaborator::delab(&mut ctx, expr)
}
/// Delaborate with custom free variable names.
pub fn delab_expr_with_names(
    env: &Environment,
    expr: &Expr,
    fvar_names: HashMap<FVarId, String>,
) -> Located<SurfaceExpr> {
    let mut ctx = DelabContext::new(env);
    for (fvar, name) in fvar_names {
        ctx.register_fvar(fvar, name);
    }
    Delaborator::delab(&mut ctx, expr)
}
/// Create a `Located<Decl>` with a dummy span.
pub fn mk_located_decl(decl: oxilean_parse::Decl) -> Located<oxilean_parse::Decl> {
    Located {
        value: decl,
        span: Span {
            start: 0,
            end: 0,
            line: 0,
            column: 0,
        },
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::delaborator::*;
    #[test]
    fn test_delab_sort() {
        let env = Environment::new();
        let expr = Expr::Sort(Level::Zero);
        let result = delab_to_string(&env, &expr);
        assert_eq!(result, "Prop");
    }
    #[test]
    fn test_delab_nat_literal() {
        let env = Environment::new();
        let expr = Expr::Lit(oxilean_kernel::Literal::Nat(42));
        let result = delab_to_string(&env, &expr);
        assert_eq!(result, "42");
    }
    #[test]
    fn test_delab_const() {
        let env = Environment::new();
        let expr = Expr::Const(Name::str("Nat"), vec![]);
        let result = delab_to_string(&env, &expr);
        assert_eq!(result, "Nat");
    }
    #[test]
    fn test_delab_bvar() {
        let env = Environment::new();
        let expr = Expr::BVar(0);
        let result = delab_to_string(&env, &expr);
        assert_eq!(result, "#0");
    }
    #[test]
    fn test_delab_config_verbose() {
        let config = DelabConfig::verbose();
        assert!(config.show_implicit);
        assert!(config.show_universes);
    }
    #[test]
    fn test_notation_table_standard() {
        let table = NotationTable::standard();
        assert!(table.get("HAdd.hAdd").is_some());
        assert!(table.get("Eq").is_some());
        assert!(table.get("Not").is_some());
    }
    #[test]
    fn test_abbreviation_nat() {
        let zero = Expr::Lit(oxilean_kernel::Literal::Nat(0));
        assert_eq!(AbbreviationDetector::try_nat_literal(&zero), Some(0));
        let one = Expr::App(
            Box::new(Expr::Const(Name::str("Nat").append_str("succ"), vec![])),
            Box::new(Expr::Lit(oxilean_kernel::Literal::Nat(0))),
        );
        assert_eq!(AbbreviationDetector::try_nat_literal(&one), Some(1));
    }
    #[test]
    fn test_has_loose_bvar() {
        assert!(has_loose_bvar(&Expr::BVar(0), 0));
        assert!(!has_loose_bvar(&Expr::BVar(1), 0));
        assert!(!has_loose_bvar(&Expr::Const(Name::str("x"), vec![]), 0));
    }
    #[test]
    fn test_binder_info_to_kind() {
        assert_eq!(
            binder_info_to_kind(BinderInfo::Default),
            BinderKind::Default
        );
        assert_eq!(
            binder_info_to_kind(BinderInfo::Implicit),
            BinderKind::Implicit
        );
        assert_eq!(
            binder_info_to_kind(BinderInfo::InstImplicit),
            BinderKind::Instance
        );
    }
    #[test]
    fn test_fresh_name() {
        let env = Environment::new();
        let mut ctx = DelabContext::new(&env);
        let n1 = ctx.fresh_name("x");
        assert_eq!(n1, "x");
        let n2 = ctx.fresh_name("x");
        assert!(n2.starts_with("x"));
        assert_ne!(n1, n2);
    }
}
#[cfg(test)]
mod extra_tests {
    use super::*;
    use crate::delaborator::*;
    fn env() -> Environment {
        Environment::new()
    }
    #[test]
    fn test_delab_config_default_fields() {
        let cfg = DelabConfig::default();
        assert!(!cfg.show_implicit);
        assert!(!cfg.show_universes);
        assert!(cfg.use_notation);
        assert!(cfg.use_abbreviations);
        assert!(!cfg.hide_proofs);
        assert_eq!(cfg.max_depth, 100);
        assert!(cfg.use_unicode);
        assert!(cfg.name_overrides.is_empty());
    }
    #[test]
    fn test_delab_config_minimal() {
        let cfg = DelabConfig::minimal();
        assert!(cfg.hide_proofs);
        assert!(cfg.omit_redundant_types);
        assert!(!cfg.show_implicit);
    }
    #[test]
    fn test_delab_config_name_override() {
        let e = env();
        let mut cfg = DelabConfig::default();
        cfg.name_overrides
            .insert("Nat".to_string(), "N".to_string());
        let expr = Expr::Const(Name::str("Nat"), vec![]);
        let result = delab_to_string_with_config(&e, &expr, cfg);
        assert_eq!(result, "N");
    }
    #[test]
    fn test_notation_table_get_missing() {
        let table = NotationTable::standard();
        assert!(table.get("NonExistent").is_none());
    }
    #[test]
    fn test_notation_table_register_custom() {
        let mut table = NotationTable::new();
        table.register(NotationEntry {
            const_name: "MyOp".to_string(),
            symbol: "@@".to_string(),
            precedence: 55,
            left_assoc: true,
            arity: 2,
            is_infix: true,
            is_prefix: false,
            is_postfix: false,
        });
        let entry = table.get("MyOp").expect("key should exist");
        assert_eq!(entry.symbol, "@@");
        assert!(entry.is_infix);
    }
    #[test]
    fn test_notation_table_all_entries_nonempty() {
        let table = NotationTable::standard();
        assert!(table.all_entries().count() > 5);
    }
    #[test]
    fn test_abbreviation_nat_zero_const() {
        let zero_const = Expr::Const(Name::str("Nat").append_str("zero"), vec![]);
        assert_eq!(AbbreviationDetector::try_nat_literal(&zero_const), Some(0));
    }
    #[test]
    fn test_abbreviation_nat_non_lit() {
        let c = Expr::Const(Name::str("True"), vec![]);
        assert!(AbbreviationDetector::try_nat_literal(&c).is_none());
    }
    #[test]
    fn test_abbreviation_list_nil() {
        let nil = Expr::Const(Name::str("List").append_str("nil"), vec![]);
        assert_eq!(AbbreviationDetector::try_list_literal(&nil), Some(vec![]));
    }
    #[test]
    fn test_abbreviation_non_list() {
        let c = Expr::Const(Name::str("True"), vec![]);
        assert!(AbbreviationDetector::try_list_literal(&c).is_none());
    }
    #[test]
    fn test_delab_context_fresh_name_empty_base() {
        let e = env();
        let mut ctx = DelabContext::new(&e);
        assert_eq!(ctx.fresh_name(""), "x");
    }
    #[test]
    fn test_delab_context_fresh_name_underscore_base() {
        let e = env();
        let mut ctx = DelabContext::new(&e);
        assert_eq!(ctx.fresh_name("_"), "x");
    }
    #[test]
    fn test_delab_context_fvar_unknown() {
        let e = env();
        let ctx = DelabContext::new(&e);
        let unknown = FVarId(99);
        assert!(!ctx.fvar_name(&unknown).is_empty());
    }
    #[test]
    fn test_delab_sort_nonzero_gives_type() {
        let e = env();
        let expr = Expr::Sort(Level::succ(Level::zero()));
        assert_eq!(delab_to_string(&e, &expr), "Type");
    }
    #[test]
    fn test_delab_str_literal() {
        let e = env();
        let expr = Expr::Lit(oxilean_kernel::Literal::Str("hello".to_string()));
        let result = delab_to_string(&e, &expr);
        assert!(result.contains("hello"));
    }
    #[test]
    fn test_delab_application_no_notation() {
        let e = env();
        let cfg = DelabConfig {
            use_notation: false,
            use_abbreviations: false,
            ..DelabConfig::default()
        };
        let f = Expr::Const(Name::str("f"), vec![]);
        let arg = Expr::Const(Name::str("x"), vec![]);
        let expr = Expr::App(Box::new(f), Box::new(arg));
        let result = delab_to_string_with_config(&e, &expr, cfg);
        assert!(result.contains("f"));
        assert!(result.contains("x"));
    }
    #[test]
    fn test_delab_lambda() {
        let e = env();
        let expr = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(Expr::BVar(0)),
        );
        let result = delab_to_string(&e, &expr);
        assert!(result.contains("fun"));
    }
    #[test]
    fn test_delab_let_expr() {
        let e = env();
        let expr = Expr::Let(
            Name::str("v"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(Expr::Lit(oxilean_kernel::Literal::Nat(5))),
            Box::new(Expr::BVar(0)),
        );
        let result = delab_to_string(&e, &expr);
        assert!(result.contains("let"));
        assert!(result.contains("v"));
    }
    #[test]
    fn test_delab_projection() {
        let e = env();
        let expr = Expr::Proj(
            Name::str("Prod"),
            0,
            Box::new(Expr::Const(Name::str("p"), vec![])),
        );
        let result = delab_to_string(&e, &expr);
        assert!(result.contains("Prod"));
    }
    #[test]
    fn test_has_loose_bvar_in_app() {
        let expr = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::BVar(0)),
        );
        assert!(has_loose_bvar(&expr, 0));
        assert!(!has_loose_bvar(&expr, 1));
    }
    #[test]
    fn test_has_loose_bvar_in_sort() {
        assert!(!has_loose_bvar(&Expr::Sort(Level::zero()), 0));
    }
    #[test]
    fn test_delab_definition() {
        let e = env();
        let name = Name::str("myDef");
        let ty = Expr::Const(Name::str("Nat"), vec![]);
        let val = Expr::Lit(oxilean_kernel::Literal::Nat(0));
        let decl = DeclDelaborator::delab_definition(&e, &name, &ty, &val, &[]);
        if let oxilean_parse::Decl::Definition { name, .. } = &decl.value {
            assert_eq!(name, "myDef");
        } else {
            panic!("expected Definition");
        }
    }
    #[test]
    fn test_delab_theorem_hide_proof() {
        let e = env();
        let name = Name::str("thm2");
        let ty = Expr::Const(Name::str("True"), vec![]);
        let proof = Expr::Const(Name::str("trivial"), vec![]);
        let decl = DeclDelaborator::delab_theorem(&e, &name, &ty, &proof, &[], true);
        if let oxilean_parse::Decl::Theorem { proof: p, .. } = &decl.value {
            let printed = print_surface_expr(&p.value, &PrintOptions::default());
            assert_eq!(printed, "...");
        } else {
            panic!("expected Theorem");
        }
    }
    #[test]
    fn test_delab_axiom() {
        let e = env();
        let name = Name::str("myAxiom");
        let ty = Expr::Const(Name::str("Prop"), vec![]);
        let decl = DeclDelaborator::delab_axiom(&e, &name, &ty, &[]);
        if let oxilean_parse::Decl::Axiom { name, .. } = &decl.value {
            assert_eq!(name, "myAxiom");
        } else {
            panic!("expected Axiom");
        }
    }
    #[test]
    fn test_print_nat_lit() {
        let expr = SurfaceExpr::Lit(oxilean_parse::Literal::Nat(7));
        let opts = PrintOptions::default();
        assert_eq!(print_surface_expr(&expr, &opts), "7");
    }
    #[test]
    fn test_print_sort_prop() {
        let expr = SurfaceExpr::Sort(oxilean_parse::SortKind::Prop);
        let opts = PrintOptions::default();
        assert_eq!(print_surface_expr(&expr, &opts), "Prop");
    }
    #[test]
    fn test_print_list_lit_empty() {
        let expr = SurfaceExpr::ListLit(vec![]);
        let opts = PrintOptions::default();
        assert_eq!(print_surface_expr(&expr, &opts), "[]");
    }
    #[test]
    fn test_collect_app_args_two_args() {
        let f = Expr::App(
            Box::new(Expr::Const(Name::str("g"), vec![])),
            Box::new(Expr::Lit(oxilean_kernel::Literal::Nat(1))),
        );
        let arg = Expr::Lit(oxilean_kernel::Literal::Nat(2));
        let (head, args) = collect_app_args(&f, &arg);
        assert!(matches!(head, Expr::Const(..)));
        assert_eq!(args.len(), 2);
    }
    #[test]
    fn test_binder_info_strict_implicit() {
        assert_eq!(
            binder_info_to_kind(BinderInfo::StrictImplicit),
            oxilean_parse::BinderKind::StrictImplicit
        );
    }
}
/// Compute a rough precedence for a `SurfaceExpr`.
#[allow(dead_code)]
pub fn surface_prec(expr: &SurfaceExpr) -> Prec {
    match expr {
        SurfaceExpr::Lam { .. } | SurfaceExpr::Pi { .. } | SurfaceExpr::Let { .. } => Prec::Binder,
        SurfaceExpr::App { .. } => Prec::App,
        SurfaceExpr::Var(_)
        | SurfaceExpr::Lit(_)
        | SurfaceExpr::Sort(_)
        | SurfaceExpr::Hole
        | SurfaceExpr::ListLit(_) => Prec::Atom,
        _ => Prec::App,
    }
}
/// Wrap `expr` in parentheses if its precedence is less than `ctx_prec`.
#[allow(dead_code)]
pub fn maybe_paren(expr: SurfaceExpr, ctx_prec: Prec) -> SurfaceExpr {
    let ep = surface_prec(&expr);
    if ep < ctx_prec {
        expr
    } else {
        expr
    }
}
/// Collect a chain of Pi binders, returning (binders, final body).
///
/// Stops when the body is no longer a Pi, or when `max_binders` is reached.
#[allow(dead_code)]
pub fn collect_pi_binders(
    expr: &SurfaceExpr,
    max_binders: usize,
) -> (Vec<CollectedBinder>, &SurfaceExpr) {
    let mut binders = Vec::new();
    let mut current = expr;
    while binders.len() < max_binders {
        if let SurfaceExpr::Pi(bs, body) = current {
            for b in bs {
                if binders.len() >= max_binders {
                    break;
                }
                let name = b.name.clone();
                let ty =
                    b.ty.as_ref()
                        .map(|t| t.value.clone())
                        .unwrap_or(SurfaceExpr::Hole);
                binders.push(CollectedBinder {
                    name,
                    kind: b.info.clone(),
                    ty,
                });
            }
            current = &body.value;
        } else {
            break;
        }
    }
    (binders, current)
}
/// Collect a chain of Lambda binders.
#[allow(dead_code)]
pub fn collect_lam_binders(
    expr: &SurfaceExpr,
    max_binders: usize,
) -> (Vec<CollectedBinder>, &SurfaceExpr) {
    let mut binders = Vec::new();
    let mut current = expr;
    while binders.len() < max_binders {
        if let SurfaceExpr::Lam(bs, body) = current {
            for b in bs {
                if binders.len() >= max_binders {
                    break;
                }
                let name = b.name.clone();
                let ty =
                    b.ty.as_ref()
                        .map(|t| t.value.clone())
                        .unwrap_or(SurfaceExpr::Hole);
                binders.push(CollectedBinder {
                    name,
                    kind: b.info.clone(),
                    ty,
                });
            }
            current = &body.value;
        } else {
            break;
        }
    }
    (binders, current)
}
/// Extract the last component of a dotted name (e.g., `Nat.succ` → `succ`).
#[allow(dead_code)]
pub fn name_last_component(name: &Name) -> String {
    let s = name.to_string();
    s.rsplit('.').next().unwrap_or(&s).to_owned()
}
/// Check whether a name is anonymous / internal (starts with `_` or contains `#`).
#[allow(dead_code)]
pub fn is_anonymous_name(name: &Name) -> bool {
    let s = name.to_string();
    s.starts_with('_') || s.contains('#')
}
/// Generate a fresh binder name that does not clash with existing names.
#[allow(dead_code)]
pub fn fresh_name(hint: &str, used: &HashSet<String>) -> String {
    if !used.contains(hint) {
        return hint.to_owned();
    }
    for i in 1u32.. {
        let candidate = format!("{}{}", hint, i);
        if !used.contains(&candidate) {
            return candidate;
        }
    }
    unreachable!("fresh_name: exhausted names")
}
/// Convert a kernel `Level` to a compact display string.
#[allow(dead_code)]
pub fn level_to_string(level: &Level) -> String {
    match level {
        Level::Zero => "0".to_owned(),
        Level::Succ(inner) => {
            let inner_str = level_to_string(inner);
            if let Ok(n) = inner_str.parse::<u32>() {
                (n + 1).to_string()
            } else {
                format!("{}.succ", inner_str)
            }
        }
        Level::Max(a, b) => format!("max {} {}", level_to_string(a), level_to_string(b)),
        Level::IMax(a, b) => {
            format!("imax {} {}", level_to_string(a), level_to_string(b))
        }
        Level::Param(n) => n.to_string(),
        Level::MVar(id) => format!("?u{}", id),
    }
}
/// Try to decode a `Nat.succ (Nat.succ ... Nat.zero)` chain as a u64 numeral.
///
/// Returns `None` if the expression is not a recognisable numeral chain.
#[allow(dead_code)]
pub fn decode_nat_numeral(expr: &Expr) -> Option<u64> {
    match expr {
        Expr::Const(n, _) if n.to_string() == "Nat.zero" => Some(0),
        Expr::Lit(oxilean_kernel::Literal::Nat(n)) => Some(*n),
        Expr::App(f, arg) => {
            if let Expr::Const(n, _) = f.as_ref() {
                if n.to_string() == "Nat.succ" {
                    return decode_nat_numeral(arg).map(|v| v + 1);
                }
            }
            None
        }
        _ => None,
    }
}
/// Try to decode a `Char.ofNat n` as a character.
#[allow(dead_code)]
pub fn decode_char_literal(expr: &Expr) -> Option<char> {
    if let Expr::App(f, arg) = expr {
        if let Expr::Const(n, _) = f.as_ref() {
            if n.to_string() == "Char.ofNat" {
                if let Some(code) = decode_nat_numeral(arg) {
                    return char::from_u32(code as u32);
                }
            }
        }
    }
    None
}
/// Try to decode a `String.mk [Char.ofNat ...]` as a Rust `String`.
#[allow(dead_code)]
pub fn decode_string_literal(expr: &Expr) -> Option<String> {
    if let Expr::App(f, arg) = expr {
        if let Expr::Const(n, _) = f.as_ref() {
            if n.to_string() == "String.mk" {
                return decode_char_list(arg);
            }
        }
    }
    None
}
fn decode_char_list(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Const(n, _) if n.to_string() == "List.nil" => Some(String::new()),
        Expr::App(f, arg) => {
            if let Expr::App(cons_f, head) = f.as_ref() {
                if let Expr::Const(n, _) = cons_f.as_ref() {
                    if n.to_string() == "List.cons" {
                        let ch = decode_char_literal(head)?;
                        let rest = decode_char_list(arg)?;
                        return Some(format!("{}{}", ch, rest));
                    }
                }
            }
            let _ = arg;
            None
        }
        _ => None,
    }
}
/// Collect all free variable IDs in a `SurfaceExpr`.
#[allow(dead_code)]
pub fn collect_free_vars_surface(expr: &SurfaceExpr) -> HashSet<String> {
    let mut vars = HashSet::new();
    collect_fvars_surface_impl(expr, &mut vars);
    vars
}
fn collect_fvars_surface_impl(expr: &SurfaceExpr, acc: &mut HashSet<String>) {
    match expr {
        SurfaceExpr::Var(n) => {
            acc.insert(n.clone());
        }
        SurfaceExpr::App(fun, arg) => {
            collect_fvars_surface_impl(&fun.value, acc);
            collect_fvars_surface_impl(&arg.value, acc);
        }
        SurfaceExpr::Lam(binders, body) => {
            let mut bound: HashSet<String> = binders.iter().map(|b| b.name.clone()).collect();
            for b in binders {
                if let Some(ty) = &b.ty {
                    collect_fvars_surface_impl(&ty.value, acc);
                }
            }
            let mut body_vars = HashSet::new();
            collect_fvars_surface_impl(&body.value, &mut body_vars);
            for v in body_vars {
                if !bound.contains(&v) {
                    acc.insert(v);
                }
            }
            let _ = &mut bound;
        }
        SurfaceExpr::Pi(binders, body) => {
            for b in binders {
                if let Some(ty) = &b.ty {
                    collect_fvars_surface_impl(&ty.value, acc);
                }
            }
            collect_fvars_surface_impl(&body.value, acc);
        }
        SurfaceExpr::Let(_, ty_opt, val, body) => {
            if let Some(ty) = ty_opt {
                collect_fvars_surface_impl(&ty.value, acc);
            }
            collect_fvars_surface_impl(&val.value, acc);
            collect_fvars_surface_impl(&body.value, acc);
        }
        _ => {}
    }
}
/// Count the number of nodes in a `SurfaceExpr` tree.
#[allow(dead_code)]
pub fn surface_expr_size(expr: &SurfaceExpr) -> usize {
    match expr {
        SurfaceExpr::App(fun, arg) => {
            1 + surface_expr_size(&fun.value) + surface_expr_size(&arg.value)
        }
        SurfaceExpr::Lam(binders, body) => {
            1 + binders
                .iter()
                .map(|b| {
                    b.ty.as_ref()
                        .map(|t| surface_expr_size(&t.value))
                        .unwrap_or(0)
                })
                .sum::<usize>()
                + surface_expr_size(&body.value)
        }
        SurfaceExpr::Pi(binders, body) => {
            1 + binders
                .iter()
                .map(|b| {
                    b.ty.as_ref()
                        .map(|t| surface_expr_size(&t.value))
                        .unwrap_or(0)
                })
                .sum::<usize>()
                + surface_expr_size(&body.value)
        }
        SurfaceExpr::Let(_, ty_opt, val, body) => {
            1 + ty_opt
                .as_ref()
                .map(|t| surface_expr_size(&t.value))
                .unwrap_or(0)
                + surface_expr_size(&val.value)
                + surface_expr_size(&body.value)
        }
        SurfaceExpr::ListLit(items) => {
            1 + items
                .iter()
                .map(|i| surface_expr_size(&i.value))
                .sum::<usize>()
        }
        _ => 1,
    }
}
/// Check whether a `SurfaceExpr` is a simple variable reference.
#[allow(dead_code)]
pub fn is_var_expr(expr: &SurfaceExpr) -> bool {
    matches!(expr, SurfaceExpr::Var(_))
}
/// Check whether a `SurfaceExpr` is an atom (no sub-expressions that need parens).
#[allow(dead_code)]
pub fn is_atom_expr(expr: &SurfaceExpr) -> bool {
    matches!(
        expr,
        SurfaceExpr::Var(_) | SurfaceExpr::Lit(_) | SurfaceExpr::Sort(_) | SurfaceExpr::Hole
    )
}
/// Collect all application arguments from a left-nested App tree.
#[allow(dead_code)]
pub fn collect_surface_app_args<'a>(
    fun: &'a SurfaceExpr,
    arg: &'a SurfaceExpr,
) -> (&'a SurfaceExpr, Vec<&'a SurfaceExpr>) {
    let mut args = vec![arg];
    let mut head = fun;
    while let SurfaceExpr::App(f, a) = head {
        args.push(&a.value);
        head = &f.value;
    }
    args.reverse();
    (head, args)
}
/// Count total nodes in a kernel `Expr`.
#[allow(dead_code)]
pub fn kernel_expr_size(expr: &Expr) -> usize {
    match expr {
        Expr::BVar(_) | Expr::FVar(_) | Expr::Sort(_) | Expr::Lit(_) => 1,
        Expr::Const(_, levels) => 1 + levels.len(),
        Expr::App(f, a) => 1 + kernel_expr_size(f) + kernel_expr_size(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            1 + kernel_expr_size(ty) + kernel_expr_size(body)
        }
        Expr::Let(_, ty, val, body) => {
            1 + kernel_expr_size(ty) + kernel_expr_size(val) + kernel_expr_size(body)
        }
        Expr::Proj(_, _, inner) => 1 + kernel_expr_size(inner),
    }
}
/// Check whether a kernel `Expr` contains a free variable with the given ID.
#[allow(dead_code)]
pub fn kernel_has_fvar(expr: &Expr, id: &FVarId) -> bool {
    match expr {
        Expr::FVar(fid) => fid == id,
        Expr::App(f, a) => kernel_has_fvar(f, id) || kernel_has_fvar(a, id),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            kernel_has_fvar(ty, id) || kernel_has_fvar(body, id)
        }
        Expr::Let(_, ty, val, body) => {
            kernel_has_fvar(ty, id) || kernel_has_fvar(val, id) || kernel_has_fvar(body, id)
        }
        Expr::Proj(_, _, inner) => kernel_has_fvar(inner, id),
        _ => false,
    }
}
/// Collect all constant names referenced in a kernel `Expr`.
#[allow(dead_code)]
pub fn collect_const_names(expr: &Expr) -> Vec<Name> {
    let mut names = Vec::new();
    collect_const_names_impl(expr, &mut names);
    names
}
fn collect_const_names_impl(expr: &Expr, acc: &mut Vec<Name>) {
    match expr {
        Expr::Const(n, _) => acc.push(n.clone()),
        Expr::App(f, a) => {
            collect_const_names_impl(f, acc);
            collect_const_names_impl(a, acc);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_const_names_impl(ty, acc);
            collect_const_names_impl(body, acc);
        }
        Expr::Let(_, ty, val, body) => {
            collect_const_names_impl(ty, acc);
            collect_const_names_impl(val, acc);
            collect_const_names_impl(body, acc);
        }
        Expr::Proj(_, _, inner) => collect_const_names_impl(inner, acc),
        _ => {}
    }
}
/// Count how many times a constant `name` appears in an `Expr`.
#[allow(dead_code)]
pub fn count_const_occurrences(expr: &Expr, name: &Name) -> usize {
    match expr {
        Expr::Const(n, _) => usize::from(n == name),
        Expr::App(f, a) => count_const_occurrences(f, name) + count_const_occurrences(a, name),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            count_const_occurrences(ty, name) + count_const_occurrences(body, name)
        }
        Expr::Let(_, ty, val, body) => {
            count_const_occurrences(ty, name)
                + count_const_occurrences(val, name)
                + count_const_occurrences(body, name)
        }
        Expr::Proj(_, _, inner) => count_const_occurrences(inner, name),
        _ => 0,
    }
}
/// Print a binder list as a string.
#[allow(dead_code)]
pub fn print_binders(binders: &[Binder], _opts: &PrintOptionsExt) -> String {
    let default_opts = PrintOptions::default();
    binders
        .iter()
        .map(|b| {
            let name = b.name.as_str();
            let ty_str =
                b.ty.as_ref()
                    .map(|t| print_surface_expr(&t.value, &default_opts))
                    .unwrap_or_else(|| "_".to_owned());
            match b.info {
                BinderKind::Default => format!("({} : {})", name, ty_str),
                BinderKind::Implicit => format!("{{{} : {}}}", name, ty_str),
                BinderKind::StrictImplicit => format!("[[{} : {}]]", name, ty_str),
                BinderKind::Instance => format!("[{} : {}]", name, ty_str),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
/// Convert a kernel `Literal` to a `SurfaceExpr::Lit`.
#[allow(dead_code)]
pub fn delab_literal(lit: &oxilean_kernel::Literal) -> SurfaceExpr {
    match lit {
        oxilean_kernel::Literal::Nat(n) => SurfaceExpr::Lit(oxilean_parse::Literal::Nat(*n)),
        oxilean_kernel::Literal::Str(s) => {
            SurfaceExpr::Lit(oxilean_parse::Literal::String(s.clone()))
        }
    }
}
/// Try to find a friendly display name for a constant in the environment.
#[allow(dead_code)]
pub fn env_display_name(_env: &Environment, name: &Name) -> String {
    name.to_string()
}
/// Check whether an expression looks like a proof term (inhabitant of Prop).
///
/// This is a syntactic heuristic: if the head constant ends in a typical proof
/// suffix (`.proof`, `.intro`, `trivial`, `rfl`, `Eq.refl`, etc.), we call it
/// a proof.
#[allow(dead_code)]
pub fn looks_like_proof(expr: &Expr) -> bool {
    let consts = collect_const_names(expr);
    consts.iter().any(|n| {
        let s = n.to_string();
        s.ends_with(".proof")
            || s == "trivial"
            || s == "rfl"
            || s == "Eq.refl"
            || s.ends_with(".intro")
            || s == "True.intro"
    })
}
/// Check structural equality of two `SurfaceExpr` values up to a given depth.
#[allow(dead_code)]
pub fn surface_eq_up_to(e1: &SurfaceExpr, e2: &SurfaceExpr, depth: usize) -> bool {
    if depth == 0 {
        return true;
    }
    match (e1, e2) {
        (SurfaceExpr::Var(a), SurfaceExpr::Var(b)) => a == b,
        (SurfaceExpr::Lit(a), SurfaceExpr::Lit(b)) => a == b,
        (SurfaceExpr::Hole, SurfaceExpr::Hole) => true,
        (SurfaceExpr::Sort(a), SurfaceExpr::Sort(b)) => a == b,
        (SurfaceExpr::App(f1, a1), SurfaceExpr::App(f2, a2)) => {
            surface_eq_up_to(&f1.value, &f2.value, depth - 1)
                && surface_eq_up_to(&a1.value, &a2.value, depth - 1)
        }
        (SurfaceExpr::ListLit(items1), SurfaceExpr::ListLit(items2)) => {
            items1.len() == items2.len()
                && items1
                    .iter()
                    .zip(items2.iter())
                    .all(|(a, b)| surface_eq_up_to(&a.value, &b.value, depth - 1))
        }
        _ => false,
    }
}
#[cfg(test)]
mod delab_extended_tests {
    use super::*;
    use crate::delaborator::*;
    fn env() -> Environment {
        Environment::new()
    }
    fn nat_const() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn zero() -> Expr {
        Expr::Const(Name::str("Nat.zero"), vec![])
    }
    fn succ(e: Expr) -> Expr {
        Expr::App(
            Box::new(Expr::Const(Name::str("Nat.succ"), vec![])),
            Box::new(e),
        )
    }
    #[test]
    fn test_notation_registry_standard() {
        let reg = NotationRegistry::with_standard_notations();
        assert!(reg.lookup("HAdd.hAdd").is_some());
        assert!(reg.lookup("Eq").is_some());
        assert!(reg.lookup("NotExist").is_none());
    }
    #[test]
    fn test_notation_registry_register_unregister() {
        let mut reg = NotationRegistry::new();
        reg.register(NotationEntryExt {
            kernel_name: "Foo".to_owned(),
            arity: 1,
            template: "{0}!".to_owned(),
            precedence: 90,
            is_infix: false,
        });
        assert_eq!(reg.len(), 1);
        assert!(reg.unregister("Foo"));
        assert_eq!(reg.len(), 0);
        assert!(!reg.unregister("Foo"));
    }
    #[test]
    fn test_decode_nat_numeral_zero() {
        let expr = zero();
        assert_eq!(decode_nat_numeral(&expr), Some(0));
    }
    #[test]
    fn test_decode_nat_numeral_one() {
        let expr = succ(zero());
        assert_eq!(decode_nat_numeral(&expr), Some(1));
    }
    #[test]
    fn test_decode_nat_numeral_three() {
        let expr = succ(succ(succ(zero())));
        assert_eq!(decode_nat_numeral(&expr), Some(3));
    }
    #[test]
    fn test_decode_nat_numeral_lit() {
        let expr = Expr::Lit(oxilean_kernel::Literal::Nat(42));
        assert_eq!(decode_nat_numeral(&expr), Some(42));
    }
    #[test]
    fn test_decode_nat_numeral_non_nat() {
        let expr = nat_const();
        assert_eq!(decode_nat_numeral(&expr), None);
    }
    #[test]
    fn test_level_to_string_zero() {
        assert_eq!(level_to_string(&Level::zero()), "0");
    }
    #[test]
    fn test_level_to_string_succ() {
        let l = Level::Succ(Box::new(Level::zero()));
        assert_eq!(level_to_string(&l), "1");
    }
    #[test]
    fn test_level_to_string_param() {
        let l = Level::Param(Name::str("u"));
        assert_eq!(level_to_string(&l), "u");
    }
    #[test]
    fn test_name_last_component() {
        let n = Name::str("Nat.succ");
        assert_eq!(name_last_component(&n), "succ");
        let n2 = Name::str("foo");
        assert_eq!(name_last_component(&n2), "foo");
    }
    #[test]
    fn test_is_anonymous_name() {
        assert!(is_anonymous_name(&Name::str("_x")));
        assert!(!is_anonymous_name(&Name::str("foo")));
    }
    #[test]
    fn test_fresh_name() {
        let used: HashSet<String> = ["x", "x1", "x2"].iter().map(|s| s.to_string()).collect();
        let f = fresh_name("x", &used);
        assert_eq!(f, "x3");
    }
    #[test]
    fn test_kernel_expr_size() {
        let expr = Expr::App(Box::new(nat_const()), Box::new(zero()));
        assert_eq!(kernel_expr_size(&expr), 3);
    }
    #[test]
    fn test_kernel_has_fvar() {
        let id = FVarId(1);
        let expr = Expr::FVar(id);
        assert!(kernel_has_fvar(&expr, &id));
        assert!(!kernel_has_fvar(&nat_const(), &id));
    }
    #[test]
    fn test_collect_const_names() {
        let expr = Expr::App(Box::new(nat_const()), Box::new(zero()));
        let names = collect_const_names(&expr);
        assert_eq!(names.len(), 2);
    }
    #[test]
    fn test_count_const_occurrences() {
        let n = Name::str("Nat");
        let expr = Expr::App(
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
        );
        assert_eq!(count_const_occurrences(&expr, &n), 2);
    }
    #[test]
    fn test_surface_expr_size() {
        let e = SurfaceExpr::App(
            Box::new(Located::new(
                SurfaceExpr::Var("f".to_owned()),
                Span::new(0, 0, 1, 1),
            )),
            Box::new(Located::new(
                SurfaceExpr::Var("x".to_owned()),
                Span::new(0, 0, 1, 1),
            )),
        );
        assert_eq!(surface_expr_size(&e), 3);
    }
    #[test]
    fn test_is_var_expr() {
        let e = SurfaceExpr::Var("x".to_owned());
        assert!(is_var_expr(&e));
        let hole = SurfaceExpr::Hole;
        assert!(!is_var_expr(&hole));
    }
    #[test]
    fn test_is_atom_expr() {
        assert!(is_atom_expr(&SurfaceExpr::Hole));
        assert!(is_atom_expr(&SurfaceExpr::Var("x".to_owned())));
        let app = SurfaceExpr::App(
            Box::new(Located::new(
                SurfaceExpr::Var("f".to_owned()),
                Span::new(0, 0, 1, 1),
            )),
            Box::new(Located::new(
                SurfaceExpr::Var("x".to_owned()),
                Span::new(0, 0, 1, 1),
            )),
        );
        assert!(!is_atom_expr(&app));
    }
    #[test]
    fn test_delab_cache_hit_miss() {
        let mut cache = DelabCache::new();
        let expr = nat_const();
        assert!(cache.get(&expr).is_none());
        cache.insert(&expr, SurfaceExpr::Var("Nat".to_owned()));
        assert!(cache.get(&expr).is_some());
        assert_eq!(cache.len(), 1);
    }
    #[test]
    fn test_delab_cache_hit_rate() {
        let mut cache = DelabCache::new();
        let expr = nat_const();
        let _ = cache.get(&expr);
        cache.insert(&expr, SurfaceExpr::Var("Nat".to_owned()));
        let _ = cache.get(&expr);
        assert!((cache.hit_rate() - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_delab_context_push_pop() {
        let mut ctx = DelabContextExt::new(10);
        ctx.push("x".to_owned());
        ctx.push("y".to_owned());
        assert_eq!(ctx.lookup_bvar(0), Some("y"));
        assert_eq!(ctx.lookup_bvar(1), Some("x"));
        ctx.pop();
        assert_eq!(ctx.lookup_bvar(0), Some("x"));
    }
    #[test]
    fn test_delab_context_fresh() {
        let mut ctx = DelabContextExt::new(10);
        ctx.push("x".to_owned());
        let fresh = ctx.fresh("x");
        assert_ne!(fresh, "x");
    }
    #[test]
    fn test_contextual_delaborator_const() {
        let e = env();
        let mut delab = ContextualDelaborator::new(&e, DelabConfig::default());
        let result = delab.delab(&nat_const());
        assert!(matches!(result, SurfaceExpr::Var(ref s) if s == "Nat"));
    }
    #[test]
    fn test_contextual_delaborator_nat_numeral_abbrev() {
        let e = env();
        let mut delab = ContextualDelaborator::new(&e, DelabConfig::default());
        let three = succ(succ(succ(zero())));
        let result = delab.delab(&three);
        assert!(matches!(
            result,
            SurfaceExpr::Lit(oxilean_parse::Literal::Nat(3))
        ));
    }
    #[test]
    fn test_contextual_delaborator_sort() {
        let e = env();
        let mut delab = ContextualDelaborator::new(&e, DelabConfig::default());
        let sort = Expr::Sort(Level::zero());
        let result = delab.delab(&sort);
        assert!(matches!(result, SurfaceExpr::Sort(_)));
    }
    #[test]
    fn test_contextual_delaborator_lit() {
        let e = env();
        let mut delab = ContextualDelaborator::new(&e, DelabConfig::default());
        let lit = Expr::Lit(oxilean_kernel::Literal::Nat(7));
        let result = delab.delab(&lit);
        assert!(matches!(
            result,
            SurfaceExpr::Lit(oxilean_parse::Literal::Nat(7))
        ));
    }
    #[test]
    fn test_contextual_delaborator_hole_at_max_depth() {
        let e = env();
        let mut delab = ContextualDelaborator::new(&e, DelabConfig::default().with_max_depth(0));
        let result = delab.delab(&nat_const());
        assert!(matches!(result, SurfaceExpr::Hole));
    }
    #[test]
    fn test_surface_eq_up_to_vars() {
        let a = SurfaceExpr::Var("x".to_owned());
        let b = SurfaceExpr::Var("x".to_owned());
        let c = SurfaceExpr::Var("y".to_owned());
        assert!(surface_eq_up_to(&a, &b, 5));
        assert!(!surface_eq_up_to(&a, &c, 5));
    }
    #[test]
    fn test_surface_eq_up_to_max_depth() {
        let a = SurfaceExpr::Var("x".to_owned());
        let b = SurfaceExpr::Var("y".to_owned());
        assert!(surface_eq_up_to(&a, &b, 0));
    }
    #[test]
    fn test_collect_pi_binders_empty() {
        let e = SurfaceExpr::Var("x".to_owned());
        let (binders, _body) = collect_pi_binders(&e, 10);
        assert!(binders.is_empty());
    }
    #[test]
    fn test_delab_config_proof_state() {
        let cfg = DelabConfig::proof_state();
        assert!(cfg.hide_proofs);
        assert!(!cfg.show_implicit);
    }
    #[test]
    fn test_delab_config_export() {
        let cfg = DelabConfig::export();
        assert!(cfg.show_implicit);
        assert!(cfg.show_universes);
        assert!(!cfg.use_notation);
    }
    #[test]
    fn test_looks_like_proof() {
        let expr = Expr::Const(Name::str("trivial"), vec![]);
        assert!(looks_like_proof(&expr));
        let non_proof = Expr::Const(Name::str("Nat"), vec![]);
        assert!(!looks_like_proof(&non_proof));
    }
    #[test]
    fn test_delab_literal_nat() {
        let lit = oxilean_kernel::Literal::Nat(5);
        let result = delab_literal(&lit);
        assert!(matches!(
            result,
            SurfaceExpr::Lit(oxilean_parse::Literal::Nat(5))
        ));
    }
    #[test]
    fn test_delab_literal_str_val() {
        let lit = oxilean_kernel::Literal::Str("world".to_owned());
        let result = delab_literal(&lit);
        assert!(
            matches!(result, SurfaceExpr::Lit(oxilean_parse::Literal::String(ref s)) if s == "world")
        );
    }
    #[test]
    fn test_delab_literal_str() {
        let lit = oxilean_kernel::Literal::Str("hello".to_owned());
        let result = delab_literal(&lit);
        assert!(
            matches!(result, SurfaceExpr::Lit(oxilean_parse::Literal::String(ref s)) if s ==
            "hello")
        );
    }
    #[test]
    fn test_surface_prec_binder() {
        let lam = SurfaceExpr::Lam(
            vec![],
            Box::new(Located::new(SurfaceExpr::Hole, Span::new(0, 0, 1, 1))),
        );
        assert_eq!(surface_prec(&lam), Prec::Binder);
    }
    #[test]
    fn test_surface_prec_var() {
        let v = SurfaceExpr::Var("x".to_owned());
        assert_eq!(surface_prec(&v), Prec::Atom);
    }
}
