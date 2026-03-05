//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    AppCounter, BVarCollector, ExprStats, MetaDbgCache, MetaDbgLogger, MetaDbgPriorityQueue,
    MetaDbgRegistry, MetaDbgStats, MetaDbgUtil0, MetaDebugAnalysisPass, MetaDebugConfig,
    MetaDebugConfigValue, MetaDebugDiagnostics, MetaDebugDiff, MetaDebugExtConfig3300,
    MetaDebugExtConfigVal3300, MetaDebugExtDiag3300, MetaDebugExtDiff3300, MetaDebugExtPass3300,
    MetaDebugExtPipeline3300, MetaDebugExtResult3300, MetaDebugPipeline, MetaDebugResult,
    MetaTracer, TraceEntry, TraceLevel, TraceLog,
};
use oxilean_kernel::{Expr, Level};

/// Pretty-print an Expr in a compact S-expression style for debugging.
///
/// Format examples: `Const("Nat")`, `BVar(0)`, `App(Const("f"), BVar(0))`
pub fn expr_debug(e: &Expr) -> String {
    match e {
        Expr::BVar(i) => format!("BVar({})", i),
        Expr::FVar(id) => format!("FVar({})", id.0),
        Expr::Sort(l) => format!("Sort({})", level_debug(l)),
        Expr::Const(n, levels) => {
            if levels.is_empty() {
                format!("Const(\"{}\")", n)
            } else {
                let ls: Vec<String> = levels.iter().map(level_debug).collect();
                format!("Const(\"{}\", [{}])", n, ls.join(", "))
            }
        }
        Expr::App(f, a) => format!("App({}, {})", expr_debug(f), expr_debug(a)),
        Expr::Lam(bi, n, ty, body) => {
            format!(
                "Lam({:?}, \"{}\", {}, {})",
                bi,
                n,
                expr_debug(ty),
                expr_debug(body)
            )
        }
        Expr::Pi(bi, n, ty, body) => {
            format!(
                "Pi({:?}, \"{}\", {}, {})",
                bi,
                n,
                expr_debug(ty),
                expr_debug(body)
            )
        }
        Expr::Let(n, ty, val, body) => {
            format!(
                "Let(\"{}\", {}, {}, {})",
                n,
                expr_debug(ty),
                expr_debug(val),
                expr_debug(body)
            )
        }
        Expr::Lit(lit) => format!("Lit({})", lit),
        Expr::Proj(name, idx, e) => {
            format!("Proj(\"{}\", {}, {})", name, idx, expr_debug(e))
        }
    }
}
/// Pretty-print an Expr as an indented tree.
///
/// Shows the full structure of an expression using multi-line indented form.
pub fn expr_tree(e: &Expr, indent: usize) -> String {
    let pad = " ".repeat(indent * 2);
    match e {
        Expr::BVar(i) => format!("{}BVar({})", pad, i),
        Expr::FVar(id) => format!("{}FVar({})", pad, id.0),
        Expr::Sort(l) => format!("{}Sort({})", pad, level_debug(l)),
        Expr::Const(n, levels) => {
            if levels.is_empty() {
                format!("{}Const(\"{}\")", pad, n)
            } else {
                let ls: Vec<String> = levels.iter().map(level_debug).collect();
                format!("{}Const(\"{}\", [{}])", pad, n, ls.join(", "))
            }
        }
        Expr::App(f, a) => {
            let mut out = format!("{}App\n", pad);
            out.push_str(&expr_tree(f, indent + 1));
            out.push('\n');
            out.push_str(&expr_tree(a, indent + 1));
            out
        }
        Expr::Lam(bi, n, ty, body) => {
            let mut out = format!("{}Lam({:?}, \"{}\")\n", pad, bi, n);
            out.push_str(&format!("{}  ty:\n", pad));
            out.push_str(&expr_tree(ty, indent + 2));
            out.push('\n');
            out.push_str(&format!("{}  body:\n", pad));
            out.push_str(&expr_tree(body, indent + 2));
            out
        }
        Expr::Pi(bi, n, ty, body) => {
            let mut out = format!("{}Pi({:?}, \"{}\")\n", pad, bi, n);
            out.push_str(&format!("{}  ty:\n", pad));
            out.push_str(&expr_tree(ty, indent + 2));
            out.push('\n');
            out.push_str(&format!("{}  body:\n", pad));
            out.push_str(&expr_tree(body, indent + 2));
            out
        }
        Expr::Let(n, ty, val, body) => {
            let mut out = format!("{}Let(\"{}\")\n", pad, n);
            out.push_str(&format!("{}  ty:\n", pad));
            out.push_str(&expr_tree(ty, indent + 2));
            out.push('\n');
            out.push_str(&format!("{}  val:\n", pad));
            out.push_str(&expr_tree(val, indent + 2));
            out.push('\n');
            out.push_str(&format!("{}  body:\n", pad));
            out.push_str(&expr_tree(body, indent + 2));
            out
        }
        Expr::Lit(lit) => format!("{}Lit({})", pad, lit),
        Expr::Proj(name, idx, inner) => {
            let mut out = format!("{}Proj(\"{}\", {})\n", pad, name, idx);
            out.push_str(&expr_tree(inner, indent + 1));
            out
        }
    }
}
/// Count total AST nodes in an expression.
pub fn node_count(e: &Expr) -> usize {
    match e {
        Expr::BVar(_) | Expr::FVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => 1,
        Expr::App(f, a) => 1 + node_count(f) + node_count(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            1 + node_count(ty) + node_count(body)
        }
        Expr::Let(_, ty, val, body) => 1 + node_count(ty) + node_count(val) + node_count(body),
        Expr::Proj(_, _, inner) => 1 + node_count(inner),
    }
}
/// Maximum depth of an expression tree.
pub fn tree_depth(e: &Expr) -> usize {
    match e {
        Expr::BVar(_) | Expr::FVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => 1,
        Expr::App(f, a) => 1 + tree_depth(f).max(tree_depth(a)),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            1 + tree_depth(ty).max(tree_depth(body))
        }
        Expr::Let(_, ty, val, body) => {
            1 + tree_depth(ty).max(tree_depth(val)).max(tree_depth(body))
        }
        Expr::Proj(_, _, inner) => 1 + tree_depth(inner),
    }
}
/// List all free constant names in an expression (sorted, deduplicated).
pub fn free_consts(e: &Expr) -> Vec<String> {
    let mut names = Vec::new();
    collect_consts(e, &mut names);
    names.sort();
    names.dedup();
    names
}
pub(super) fn collect_consts(e: &Expr, out: &mut Vec<String>) {
    match e {
        Expr::Const(n, _) => out.push(n.to_string()),
        Expr::App(f, a) => {
            collect_consts(f, out);
            collect_consts(a, out);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_consts(ty, out);
            collect_consts(body, out);
        }
        Expr::Let(_, ty, val, body) => {
            collect_consts(ty, out);
            collect_consts(val, out);
            collect_consts(body, out);
        }
        Expr::Proj(_, _, inner) => collect_consts(inner, out),
        _ => {}
    }
}
/// List all BVar indices present (sorted, deduplicated).
pub fn bvar_indices(e: &Expr) -> Vec<u32> {
    let mut indices = Vec::new();
    collect_bvars(e, &mut indices);
    indices.sort();
    indices.dedup();
    indices
}
pub(super) fn collect_bvars(e: &Expr, out: &mut Vec<u32>) {
    match e {
        Expr::BVar(i) => out.push(*i),
        Expr::App(f, a) => {
            collect_bvars(f, out);
            collect_bvars(a, out);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_bvars(ty, out);
            collect_bvars(body, out);
        }
        Expr::Let(_, ty, val, body) => {
            collect_bvars(ty, out);
            collect_bvars(val, out);
            collect_bvars(body, out);
        }
        Expr::Proj(_, _, inner) => collect_bvars(inner, out),
        _ => {}
    }
}
/// Check if expression is "closed" (no free BVars at depth 0).
pub fn is_closed(e: &Expr) -> bool {
    !has_open_bvar(e, 0)
}
/// Returns true if the expression has a BVar with index >= depth (i.e. not bound by any
/// enclosing binder at or above `depth`).
pub(super) fn has_open_bvar(e: &Expr, depth: u32) -> bool {
    match e {
        Expr::BVar(i) => *i >= depth,
        Expr::App(f, a) => has_open_bvar(f, depth) || has_open_bvar(a, depth),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            has_open_bvar(ty, depth) || has_open_bvar(body, depth + 1)
        }
        Expr::Let(_, ty, val, body) => {
            has_open_bvar(ty, depth) || has_open_bvar(val, depth) || has_open_bvar(body, depth + 1)
        }
        Expr::Proj(_, _, inner) => has_open_bvar(inner, depth),
        _ => false,
    }
}
/// Format a Level for debugging.
pub fn level_debug(l: &Level) -> String {
    match l {
        Level::Zero => "0".to_string(),
        Level::Succ(inner) => {
            let mut n = 1u32;
            let mut cur = inner.as_ref();
            loop {
                match cur {
                    Level::Zero => return n.to_string(),
                    Level::Succ(next) => {
                        n += 1;
                        cur = next.as_ref();
                    }
                    other => return format!("succ({})", level_debug(other)),
                }
            }
        }
        Level::Max(l1, l2) => format!("max({}, {})", level_debug(l1), level_debug(l2)),
        Level::IMax(l1, l2) => format!("imax({}, {})", level_debug(l1), level_debug(l2)),
        Level::Param(n) => format!("param({})", n),
        Level::MVar(id) => format!("mvar({:?})", id),
    }
}
/// Compare two expressions and return a list of differences.
///
/// Each difference is formatted as: `"Path <path>: <a> vs <b>"`
pub fn expr_diff(a: &Expr, b: &Expr) -> Vec<String> {
    let mut diffs = Vec::new();
    diff_rec(a, b, "root", &mut diffs);
    diffs
}
pub(super) fn diff_rec(a: &Expr, b: &Expr, path: &str, diffs: &mut Vec<String>) {
    if a == b {
        return;
    }
    match (a, b) {
        (Expr::App(f1, a1), Expr::App(f2, a2)) => {
            diff_rec(f1, f2, &format!("{}.f", path), diffs);
            diff_rec(a1, a2, &format!("{}.a", path), diffs);
        }
        (Expr::Lam(bi1, n1, ty1, b1), Expr::Lam(bi2, n2, ty2, b2)) => {
            if bi1 != bi2 || n1 != n2 {
                diffs.push(format!(
                    "Path {}: Lam({:?}, \"{}\") vs Lam({:?}, \"{}\")",
                    path, bi1, n1, bi2, n2
                ));
            }
            diff_rec(ty1, ty2, &format!("{}.lam_ty", path), diffs);
            diff_rec(b1, b2, &format!("{}.lam_body", path), diffs);
        }
        (Expr::Pi(bi1, n1, ty1, b1), Expr::Pi(bi2, n2, ty2, b2)) => {
            if bi1 != bi2 || n1 != n2 {
                diffs.push(format!(
                    "Path {}: Pi({:?}, \"{}\") vs Pi({:?}, \"{}\")",
                    path, bi1, n1, bi2, n2
                ));
            }
            diff_rec(ty1, ty2, &format!("{}.pi_ty", path), diffs);
            diff_rec(b1, b2, &format!("{}.pi_body", path), diffs);
        }
        (Expr::Let(n1, ty1, v1, b1), Expr::Let(n2, ty2, v2, b2)) => {
            if n1 != n2 {
                diffs.push(format!("Path {}: Let(\"{}\") vs Let(\"{}\")", path, n1, n2));
            }
            diff_rec(ty1, ty2, &format!("{}.let_ty", path), diffs);
            diff_rec(v1, v2, &format!("{}.let_val", path), diffs);
            diff_rec(b1, b2, &format!("{}.let_body", path), diffs);
        }
        (Expr::Proj(n1, i1, e1), Expr::Proj(n2, i2, e2)) => {
            if n1 != n2 || i1 != i2 {
                diffs.push(format!(
                    "Path {}: Proj(\"{}\", {}) vs Proj(\"{}\", {})",
                    path, n1, i1, n2, i2
                ));
            }
            diff_rec(e1, e2, &format!("{}.proj", path), diffs);
        }
        _ => {
            diffs.push(format!(
                "Path {}: {} vs {}",
                path,
                expr_debug(a),
                expr_debug(b)
            ));
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::meta_debug::*;
    use oxilean_kernel::{BinderInfo, Literal, Name};
    fn mk_const(name: &str) -> Expr {
        Expr::Const(Name::str(name), vec![])
    }
    fn mk_app(f: Expr, a: Expr) -> Expr {
        Expr::App(Box::new(f), Box::new(a))
    }
    fn mk_lam(name: &str, ty: Expr, body: Expr) -> Expr {
        Expr::Lam(
            BinderInfo::Default,
            Name::str(name),
            Box::new(ty),
            Box::new(body),
        )
    }
    #[test]
    fn test_expr_debug_const() {
        let e = mk_const("Nat");
        assert_eq!(expr_debug(&e), "Const(\"Nat\")");
    }
    #[test]
    fn test_expr_debug_app() {
        let e = mk_app(mk_const("f"), mk_const("a"));
        let s = expr_debug(&e);
        assert!(s.contains("App"));
        assert!(s.contains("Const(\"f\")"));
        assert!(s.contains("Const(\"a\")"));
    }
    #[test]
    fn test_node_count() {
        assert_eq!(node_count(&mk_const("Nat")), 1);
        assert_eq!(node_count(&mk_app(mk_const("f"), mk_const("a"))), 3);
        let nested = mk_app(mk_app(mk_const("f"), mk_const("a")), mk_const("b"));
        assert_eq!(node_count(&nested), 5);
    }
    #[test]
    fn test_tree_depth() {
        assert_eq!(tree_depth(&mk_const("Nat")), 1);
        assert_eq!(tree_depth(&mk_app(mk_const("f"), mk_const("a"))), 2);
        let nested = mk_app(mk_app(mk_const("f"), mk_const("a")), mk_const("b"));
        assert_eq!(tree_depth(&nested), 3);
    }
    #[test]
    fn test_free_consts() {
        let e = mk_app(mk_const("f"), mk_app(mk_const("a"), mk_const("f")));
        let consts = free_consts(&e);
        assert_eq!(consts, vec!["a".to_string(), "f".to_string()]);
    }
    #[test]
    fn test_bvar_indices() {
        let e = mk_app(Expr::BVar(2), Expr::BVar(0));
        let indices = bvar_indices(&e);
        assert_eq!(indices, vec![0, 2]);
    }
    #[test]
    fn test_meta_tracer() {
        let mut tracer = MetaTracer::new();
        assert!(!tracer.enabled);
        assert_eq!(tracer.len(), 0);
        tracer.enable();
        assert!(tracer.enabled);
        tracer.enter("check");
        tracer.record("expr", &mk_const("Nat"));
        tracer.exit();
        assert_eq!(tracer.len(), 3);
        let dump = tracer.dump();
        assert!(dump.contains("check"));
        assert!(dump.contains("Nat"));
        tracer.clear();
        assert_eq!(tracer.len(), 0);
    }
    #[test]
    fn test_expr_diff() {
        let a = mk_const("Nat");
        let b = mk_const("Nat");
        assert!(expr_diff(&a, &b).is_empty());
        let c = mk_const("Int");
        let diffs = expr_diff(&a, &c);
        assert!(!diffs.is_empty());
        assert!(diffs[0].contains("Nat"));
        assert!(diffs[0].contains("Int"));
        let app1 = mk_app(mk_const("f"), mk_const("Nat"));
        let app2 = mk_app(mk_const("f"), mk_const("Int"));
        let diffs2 = expr_diff(&app1, &app2);
        assert!(!diffs2.is_empty());
        assert!(diffs2
            .iter()
            .any(|d| d.contains("Nat") && d.contains("Int")));
        let lit1 = Expr::Lit(Literal::Nat(42));
        let lit2 = Expr::Lit(Literal::Nat(99));
        let diffs3 = expr_diff(&lit1, &lit2);
        assert!(!diffs3.is_empty());
    }
}
/// Compact serialization of an expression to a string.
#[allow(dead_code)]
pub fn serialize_expr_compact(e: &Expr) -> String {
    match e {
        Expr::BVar(i) => format!("B{}", i),
        Expr::FVar(id) => format!("F{}", id.0),
        Expr::Sort(_) => "S".to_string(),
        Expr::Const(n, _) => format!("C[{}]", n),
        Expr::Lit(lit) => format!("L[{:?}]", lit),
        Expr::App(f, a) => {
            format!(
                "@({},{})",
                serialize_expr_compact(f),
                serialize_expr_compact(a)
            )
        }
        Expr::Lam(_, n, t, b) => {
            format!(
                "λ{}:{}.{}",
                n,
                serialize_expr_compact(t),
                serialize_expr_compact(b)
            )
        }
        Expr::Pi(_, n, t, b) => {
            format!(
                "Π{}:{}.{}",
                n,
                serialize_expr_compact(t),
                serialize_expr_compact(b)
            )
        }
        Expr::Let(n, _, _t, b) => format!("let {}:{}", n, serialize_expr_compact(b)),
        Expr::Proj(n, i, e) => {
            format!("proj[{}.{}]({})", n, i, serialize_expr_compact(e))
        }
    }
}
/// Estimate the "weight" of an expression (how hard to process).
#[allow(dead_code)]
pub fn expr_weight(e: &Expr) -> usize {
    match e {
        Expr::BVar(_) | Expr::FVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => 1,
        Expr::App(f, a) => 1 + expr_weight(f) + expr_weight(a),
        Expr::Lam(_, _, t, b) | Expr::Pi(_, _, t, b) | Expr::Let(_, _, t, b) => {
            2 + expr_weight(t) + expr_weight(b)
        }
        Expr::Proj(_, _, inner) => 1 + expr_weight(inner),
    }
}
/// A visitor for expressions.
#[allow(dead_code)]
pub trait ExprVisitor {
    fn visit_bvar(&mut self, _idx: u32) {}
    fn visit_fvar(&mut self, _id: u32) {}
    fn visit_sort(&mut self) {}
    fn visit_const(&mut self, _name: &str) {}
    fn visit_lit(&mut self) {}
    fn visit_app(&mut self) {}
    fn visit_lam(&mut self) {}
    fn visit_pi(&mut self) {}
    fn visit_let(&mut self) {}
}
/// Walk an expression applying a visitor.
#[allow(dead_code)]
pub fn walk_expr<V: ExprVisitor>(e: &Expr, visitor: &mut V) {
    match e {
        Expr::BVar(i) => visitor.visit_bvar(*i),
        Expr::FVar(id) => visitor.visit_fvar(id.0 as u32),
        Expr::Sort(_) => visitor.visit_sort(),
        Expr::Const(n, _) => visitor.visit_const(&n.to_string()),
        Expr::Lit(_) => visitor.visit_lit(),
        Expr::App(f, a) => {
            visitor.visit_app();
            walk_expr(f, visitor);
            walk_expr(a, visitor);
        }
        Expr::Lam(_, _, t, b) => {
            visitor.visit_lam();
            walk_expr(t, visitor);
            walk_expr(b, visitor);
        }
        Expr::Pi(_, _, t, b) => {
            visitor.visit_pi();
            walk_expr(t, visitor);
            walk_expr(b, visitor);
        }
        Expr::Let(_, _, t, b) => {
            visitor.visit_let();
            walk_expr(t, visitor);
            walk_expr(b, visitor);
        }
        Expr::Proj(_, _, inner) => walk_expr(inner, visitor),
    }
}
/// Format an expression with indentation for hierarchical display.
#[allow(dead_code)]
pub fn format_indented(e: &Expr, indent: usize) -> String {
    let pad = " ".repeat(indent * 2);
    match e {
        Expr::BVar(i) => format!("{}BVar({})", pad, i),
        Expr::FVar(id) => format!("{}FVar({})", pad, id.0),
        Expr::Sort(l) => format!("{}Sort({})", pad, level_debug(l)),
        Expr::Const(n, _) => format!("{}Const({})", pad, n),
        Expr::Lit(_) => format!("{}Lit", pad),
        Expr::App(f, a) => {
            format!(
                "{}App\n{}\n{}",
                pad,
                format_indented(f, indent + 1),
                format_indented(a, indent + 1)
            )
        }
        Expr::Lam(_, n, t, b) => {
            format!(
                "{}Lam({})\n{}\n{}",
                pad,
                n,
                format_indented(t, indent + 1),
                format_indented(b, indent + 1)
            )
        }
        Expr::Pi(_, n, t, b) => {
            format!(
                "{}Pi({})\n{}\n{}",
                pad,
                n,
                format_indented(t, indent + 1),
                format_indented(b, indent + 1)
            )
        }
        Expr::Let(n, _, t, b) => {
            format!(
                "{}Let({})\n{}\n{}",
                pad,
                n,
                format_indented(t, indent + 1),
                format_indented(b, indent + 1)
            )
        }
        Expr::Proj(n, i, inner) => {
            format!(
                "{}Proj({}.{})\n{}",
                pad,
                n,
                i,
                format_indented(inner, indent + 1)
            )
        }
    }
}
/// Check if two expressions are alpha-equal (ignoring binder names).
#[allow(dead_code)]
pub fn alpha_eq(a: &Expr, b: &Expr) -> bool {
    match (a, b) {
        (Expr::BVar(i), Expr::BVar(j)) => i == j,
        (Expr::FVar(i), Expr::FVar(j)) => i == j,
        (Expr::Const(n1, _), Expr::Const(n2, _)) => n1 == n2,
        (Expr::Sort(_), Expr::Sort(_)) => true,
        (Expr::Lit(l1), Expr::Lit(l2)) => format!("{:?}", l1) == format!("{:?}", l2),
        (Expr::App(f1, a1), Expr::App(f2, a2)) => alpha_eq(f1, f2) && alpha_eq(a1, a2),
        (Expr::Lam(_, _, t1, b1), Expr::Lam(_, _, t2, b2))
        | (Expr::Pi(_, _, t1, b1), Expr::Pi(_, _, t2, b2)) => alpha_eq(t1, t2) && alpha_eq(b1, b2),
        (Expr::Let(_, _, t1, b1), Expr::Let(_, _, t2, b2)) => alpha_eq(t1, t2) && alpha_eq(b1, b2),
        _ => false,
    }
}
#[cfg(test)]
mod meta_debug_extended_tests {
    use super::*;
    use crate::meta_debug::*;
    use oxilean_kernel::Name;
    #[test]
    fn test_trace_log_add_entry() {
        let mut log = TraceLog::new(TraceLevel::Debug);
        log.debug("test", "main.rs");
        assert_eq!(log.num_entries(), 1);
    }
    #[test]
    fn test_trace_log_filter_by_level() {
        let mut log = TraceLog::new(TraceLevel::Trace);
        log.error("err", "a.rs");
        log.info("info", "b.rs");
        log.debug("debug", "c.rs");
        let errors = log.filter_level(&TraceLevel::Error);
        assert_eq!(errors.len(), 1);
    }
    #[test]
    fn test_trace_log_has_errors() {
        let mut log = TraceLog::new(TraceLevel::Error);
        assert!(!log.has_errors());
        log.error("oops", "x.rs");
        assert!(log.has_errors());
    }
    #[test]
    fn test_trace_level_ordering() {
        assert!(TraceLevel::Error < TraceLevel::Info);
        assert!(TraceLevel::Debug < TraceLevel::Trace);
    }
    #[test]
    fn test_expr_stats_bvar() {
        let e = Expr::BVar(0);
        let stats = ExprStats::compute(&e);
        assert_eq!(stats.num_bvars, 1);
        assert_eq!(stats.node_count, 1);
        assert!(!stats.is_closed());
    }
    #[test]
    fn test_expr_stats_const() {
        let e = Expr::Const(Name::str("Nat"), vec![]);
        let stats = ExprStats::compute(&e);
        assert_eq!(stats.num_consts, 1);
        assert!(stats.is_closed());
    }
    #[test]
    fn test_expr_stats_app() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let x = Expr::BVar(0);
        let e = Expr::App(Box::new(f), Box::new(x));
        let stats = ExprStats::compute(&e);
        assert_eq!(stats.num_apps, 1);
        assert_eq!(stats.num_bvars, 1);
    }
    #[test]
    fn test_serialize_expr_compact_bvar() {
        let e = Expr::BVar(3);
        assert_eq!(serialize_expr_compact(&e), "B3");
    }
    #[test]
    fn test_serialize_expr_compact_const() {
        let e = Expr::Const(Name::str("Nat"), vec![]);
        assert!(serialize_expr_compact(&e).contains("Nat"));
    }
    #[test]
    fn test_expr_weight_simple() {
        assert_eq!(expr_weight(&Expr::BVar(0)), 1);
        let f = Expr::BVar(0);
        let a = Expr::BVar(1);
        let app = Expr::App(Box::new(f), Box::new(a));
        assert_eq!(expr_weight(&app), 3);
    }
    #[test]
    fn test_app_counter() {
        let f = Expr::BVar(0);
        let a = Expr::BVar(1);
        let e = Expr::App(Box::new(f), Box::new(a));
        let mut counter = AppCounter(0);
        walk_expr(&e, &mut counter);
        assert_eq!(counter.0, 1);
    }
    #[test]
    fn test_bvar_collector() {
        let e = Expr::App(Box::new(Expr::BVar(0)), Box::new(Expr::BVar(1)));
        let mut collector = BVarCollector(Vec::new());
        walk_expr(&e, &mut collector);
        assert_eq!(collector.0.len(), 2);
        assert!(collector.0.contains(&0));
        assert!(collector.0.contains(&1));
    }
    #[test]
    fn test_alpha_eq_bvars() {
        assert!(alpha_eq(&Expr::BVar(0), &Expr::BVar(0)));
        assert!(!alpha_eq(&Expr::BVar(0), &Expr::BVar(1)));
    }
    #[test]
    fn test_alpha_eq_consts() {
        let a = Expr::Const(Name::str("Nat"), vec![]);
        let b = Expr::Const(Name::str("Nat"), vec![]);
        let c = Expr::Const(Name::str("Int"), vec![]);
        assert!(alpha_eq(&a, &b));
        assert!(!alpha_eq(&a, &c));
    }
    #[test]
    fn test_format_indented_bvar() {
        let e = Expr::BVar(5);
        let s = format_indented(&e, 0);
        assert!(s.contains("BVar(5)"));
    }
    #[test]
    fn test_trace_entry_with_context() {
        let entry = TraceEntry::new(TraceLevel::Info, "hello", "test.rs").with_context("ctx info");
        assert_eq!(entry.context, Some("ctx info".to_string()));
    }
    #[test]
    fn test_expr_stats_is_ground() {
        let e = Expr::Const(Name::str("x"), vec![]);
        let stats = ExprStats::compute(&e);
        assert!(stats.is_ground());
    }
}
/// Compute a simple hash of a MetaDbg name.
#[allow(dead_code)]
pub fn metadbg_hash(name: &str) -> u64 {
    let mut h: u64 = 14695981039346656037;
    for b in name.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(1099511628211);
    }
    h
}
/// Check if a MetaDbg name is valid.
#[allow(dead_code)]
pub fn metadbg_is_valid_name(name: &str) -> bool {
    !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_')
}
/// Count the occurrences of a character in a MetaDbg string.
#[allow(dead_code)]
pub fn metadbg_count_char(s: &str, c: char) -> usize {
    s.chars().filter(|&ch| ch == c).count()
}
/// Truncate a MetaDbg string to a maximum length.
#[allow(dead_code)]
pub fn metadbg_truncate(s: &str, max_len: usize) -> &str {
    if s.len() <= max_len {
        s
    } else {
        &s[..max_len]
    }
}
/// Join MetaDbg strings with a separator.
#[allow(dead_code)]
pub fn metadbg_join(parts: &[&str], sep: &str) -> String {
    parts.join(sep)
}
#[cfg(test)]
mod metadbg_ext_tests {
    use super::*;
    use crate::meta_debug::*;
    #[test]
    fn test_metadbg_util_new() {
        let u = MetaDbgUtil0::new(1, "test", 42);
        assert_eq!(u.id, 1);
        assert_eq!(u.name, "test");
        assert_eq!(u.value, 42);
        assert!(u.is_active());
    }
    #[test]
    fn test_metadbg_util_tag() {
        let u = MetaDbgUtil0::new(2, "tagged", 10).with_tag("important");
        assert!(u.has_tag("important"));
        assert_eq!(u.tag_count(), 1);
    }
    #[test]
    fn test_metadbg_util_disable() {
        let u = MetaDbgUtil0::new(3, "disabled", 100).disable();
        assert!(!u.is_active());
        assert_eq!(u.score(), 0);
    }
    #[test]
    fn test_metadbg_registry_register() {
        let mut reg = MetaDbgRegistry::new(10);
        let u = MetaDbgUtil0::new(1, "a", 1);
        assert!(reg.register(u));
        assert_eq!(reg.count(), 1);
    }
    #[test]
    fn test_metadbg_registry_lookup() {
        let mut reg = MetaDbgRegistry::new(10);
        reg.register(MetaDbgUtil0::new(5, "five", 5));
        assert!(reg.lookup(5).is_some());
        assert!(reg.lookup(99).is_none());
    }
    #[test]
    fn test_metadbg_registry_capacity() {
        let mut reg = MetaDbgRegistry::new(2);
        reg.register(MetaDbgUtil0::new(1, "a", 1));
        reg.register(MetaDbgUtil0::new(2, "b", 2));
        assert!(reg.is_full());
        assert!(!reg.register(MetaDbgUtil0::new(3, "c", 3)));
    }
    #[test]
    fn test_metadbg_registry_score() {
        let mut reg = MetaDbgRegistry::new(10);
        reg.register(MetaDbgUtil0::new(1, "a", 10));
        reg.register(MetaDbgUtil0::new(2, "b", 20));
        assert_eq!(reg.total_score(), 30);
    }
    #[test]
    fn test_metadbg_cache_hit_miss() {
        let mut cache = MetaDbgCache::new();
        cache.insert("key1", 42);
        assert_eq!(cache.get("key1"), Some(42));
        assert_eq!(cache.get("key2"), None);
        assert_eq!(cache.hits, 1);
        assert_eq!(cache.misses, 1);
    }
    #[test]
    fn test_metadbg_cache_hit_rate() {
        let mut cache = MetaDbgCache::new();
        cache.insert("k", 1);
        cache.get("k");
        cache.get("k");
        cache.get("nope");
        assert!((cache.hit_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_metadbg_cache_clear() {
        let mut cache = MetaDbgCache::new();
        cache.insert("k", 1);
        cache.clear();
        assert_eq!(cache.size(), 0);
        assert_eq!(cache.hits, 0);
    }
    #[test]
    fn test_metadbg_logger_basic() {
        let mut logger = MetaDbgLogger::new(100);
        logger.log("msg1");
        logger.log("msg2");
        assert_eq!(logger.count(), 2);
        assert_eq!(logger.last(), Some("msg2"));
    }
    #[test]
    fn test_metadbg_logger_capacity() {
        let mut logger = MetaDbgLogger::new(2);
        logger.log("a");
        logger.log("b");
        logger.log("c");
        assert_eq!(logger.count(), 2);
    }
    #[test]
    fn test_metadbg_stats_success() {
        let mut stats = MetaDbgStats::new();
        stats.record_success(100);
        stats.record_success(200);
        assert_eq!(stats.total_ops, 2);
        assert_eq!(stats.successful_ops, 2);
        assert!((stats.success_rate() - 1.0).abs() < 1e-9);
    }
    #[test]
    fn test_metadbg_stats_failure() {
        let mut stats = MetaDbgStats::new();
        stats.record_success(100);
        stats.record_failure();
        assert!((stats.success_rate() - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_metadbg_stats_merge() {
        let mut a = MetaDbgStats::new();
        let mut b = MetaDbgStats::new();
        a.record_success(100);
        b.record_failure();
        a.merge(&b);
        assert_eq!(a.total_ops, 2);
    }
    #[test]
    fn test_metadbg_priority_queue() {
        let mut pq = MetaDbgPriorityQueue::new();
        pq.push(MetaDbgUtil0::new(1, "low", 1), 1);
        pq.push(MetaDbgUtil0::new(2, "high", 2), 100);
        let (_, p) = pq.pop().expect("collection should not be empty");
        assert_eq!(p, 100);
    }
    #[test]
    fn test_metadbg_hash() {
        let h1 = metadbg_hash("foo");
        let h2 = metadbg_hash("foo");
        assert_eq!(h1, h2);
        let h3 = metadbg_hash("bar");
        assert_ne!(h1, h3);
    }
    #[test]
    fn test_metadbg_valid_name() {
        assert!(metadbg_is_valid_name("foo_bar"));
        assert!(!metadbg_is_valid_name("foo-bar"));
        assert!(!metadbg_is_valid_name(""));
    }
    #[test]
    fn test_metadbg_join() {
        let parts = ["a", "b", "c"];
        assert_eq!(metadbg_join(&parts, ", "), "a, b, c");
    }
}
#[cfg(test)]
mod metadebug_analysis_tests {
    use super::*;
    use crate::meta_debug::*;
    #[test]
    fn test_metadebug_result_ok() {
        let r = MetaDebugResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_metadebug_result_err() {
        let r = MetaDebugResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_metadebug_result_partial() {
        let r = MetaDebugResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_metadebug_result_skipped() {
        let r = MetaDebugResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_metadebug_analysis_pass_run() {
        let mut p = MetaDebugAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_metadebug_analysis_pass_empty_input() {
        let mut p = MetaDebugAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_metadebug_analysis_pass_success_rate() {
        let mut p = MetaDebugAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_metadebug_analysis_pass_disable() {
        let mut p = MetaDebugAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_metadebug_pipeline_basic() {
        let mut pipeline = MetaDebugPipeline::new("main_pipeline");
        pipeline.add_pass(MetaDebugAnalysisPass::new("pass1"));
        pipeline.add_pass(MetaDebugAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_metadebug_pipeline_disabled_pass() {
        let mut pipeline = MetaDebugPipeline::new("partial");
        let mut p = MetaDebugAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(MetaDebugAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_metadebug_diff_basic() {
        let mut d = MetaDebugDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_metadebug_diff_summary() {
        let mut d = MetaDebugDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_metadebug_config_set_get() {
        let mut cfg = MetaDebugConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_metadebug_config_read_only() {
        let mut cfg = MetaDebugConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_metadebug_config_remove() {
        let mut cfg = MetaDebugConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_metadebug_diagnostics_basic() {
        let mut diag = MetaDebugDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_metadebug_diagnostics_max_errors() {
        let mut diag = MetaDebugDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_metadebug_diagnostics_clear() {
        let mut diag = MetaDebugDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_metadebug_config_value_types() {
        let b = MetaDebugConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = MetaDebugConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = MetaDebugConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = MetaDebugConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = MetaDebugConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod meta_debug_ext_tests_3300 {
    use super::*;
    use crate::meta_debug::*;
    #[test]
    fn test_meta_debug_ext_result_ok_3300() {
        let r = MetaDebugExtResult3300::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_meta_debug_ext_result_err_3300() {
        let r = MetaDebugExtResult3300::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_meta_debug_ext_result_partial_3300() {
        let r = MetaDebugExtResult3300::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_meta_debug_ext_result_skipped_3300() {
        let r = MetaDebugExtResult3300::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_meta_debug_ext_pass_run_3300() {
        let mut p = MetaDebugExtPass3300::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_meta_debug_ext_pass_empty_3300() {
        let mut p = MetaDebugExtPass3300::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_meta_debug_ext_pass_rate_3300() {
        let mut p = MetaDebugExtPass3300::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_meta_debug_ext_pass_disable_3300() {
        let mut p = MetaDebugExtPass3300::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_meta_debug_ext_pipeline_basic_3300() {
        let mut pipeline = MetaDebugExtPipeline3300::new("main_pipeline");
        pipeline.add_pass(MetaDebugExtPass3300::new("pass1"));
        pipeline.add_pass(MetaDebugExtPass3300::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_meta_debug_ext_pipeline_disabled_3300() {
        let mut pipeline = MetaDebugExtPipeline3300::new("partial");
        let mut p = MetaDebugExtPass3300::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(MetaDebugExtPass3300::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_meta_debug_ext_diff_basic_3300() {
        let mut d = MetaDebugExtDiff3300::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_meta_debug_ext_config_set_get_3300() {
        let mut cfg = MetaDebugExtConfig3300::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_meta_debug_ext_config_read_only_3300() {
        let mut cfg = MetaDebugExtConfig3300::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_meta_debug_ext_config_remove_3300() {
        let mut cfg = MetaDebugExtConfig3300::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_meta_debug_ext_diagnostics_basic_3300() {
        let mut diag = MetaDebugExtDiag3300::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_meta_debug_ext_diagnostics_max_errors_3300() {
        let mut diag = MetaDebugExtDiag3300::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_meta_debug_ext_diagnostics_clear_3300() {
        let mut diag = MetaDebugExtDiag3300::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_meta_debug_ext_config_value_types_3300() {
        let b = MetaDebugExtConfigVal3300::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = MetaDebugExtConfigVal3300::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = MetaDebugExtConfigVal3300::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = MetaDebugExtConfigVal3300::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = MetaDebugExtConfigVal3300::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
