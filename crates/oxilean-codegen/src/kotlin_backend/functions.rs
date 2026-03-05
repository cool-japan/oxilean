//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::HashSet;

use super::types::{
    KotlinExtCache, KotlinExtConstFolder, KotlinExtDepGraph, KotlinExtDomTree, KotlinExtLiveness,
    KotlinExtPassConfig, KotlinExtPassPhase, KotlinExtPassRegistry, KotlinExtPassStats,
    KotlinExtWorklist, KotlinStmt, KotlinType, KotlinX2Cache, KotlinX2ConstFolder,
    KotlinX2DepGraph, KotlinX2DomTree, KotlinX2Liveness, KotlinX2PassConfig, KotlinX2PassPhase,
    KotlinX2PassRegistry, KotlinX2PassStats, KotlinX2Worklist, KtAnalysisCache,
    KtConstantFoldingHelper, KtDepGraph, KtDominatorTree, KtLivenessInfo, KtPassConfig,
    KtPassPhase, KtPassRegistry, KtPassStats, KtWorklist,
};
use std::fmt;

/// Map an LCNF type to a Kotlin type.
pub(super) fn lcnf_type_to_kotlin(ty: &LcnfType) -> KotlinType {
    match ty {
        LcnfType::Nat => KotlinType::KtLong,
        LcnfType::LcnfString => KotlinType::KtString,
        LcnfType::Unit => KotlinType::KtUnit,
        LcnfType::Erased | LcnfType::Irrelevant => KotlinType::KtUnit,
        LcnfType::Object => KotlinType::KtAny,
        LcnfType::Var(name) => KotlinType::KtObject(name.clone()),
        LcnfType::Fun(params, ret) => {
            let kt_params: Vec<KotlinType> = params.iter().map(lcnf_type_to_kotlin).collect();
            let kt_ret = lcnf_type_to_kotlin(ret);
            KotlinType::KtFunc(kt_params, Box::new(kt_ret))
        }
        LcnfType::Ctor(name, _args) => KotlinType::KtObject(name.clone()),
    }
}
/// Emit a block of statements with a given indentation prefix.
pub(super) fn fmt_stmts(
    stmts: &[KotlinStmt],
    indent: &str,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    for stmt in stmts {
        fmt_stmt(stmt, indent, f)?;
    }
    Ok(())
}
/// Emit a single statement with indentation.
pub(super) fn fmt_stmt(stmt: &KotlinStmt, indent: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let inner = format!("{}    ", indent);
    match stmt {
        KotlinStmt::Val(name, ty, expr) => {
            writeln!(f, "{}val {}: {} = {}", indent, name, ty, expr)
        }
        KotlinStmt::Var(name, ty, expr) => {
            writeln!(f, "{}var {}: {} = {}", indent, name, ty, expr)
        }
        KotlinStmt::Assign(name, expr) => writeln!(f, "{}{} = {}", indent, name, expr),
        KotlinStmt::Return(expr) => writeln!(f, "{}return {}", indent, expr),
        KotlinStmt::Expr(expr) => writeln!(f, "{}{}", indent, expr),
        KotlinStmt::If(cond, then_stmts, else_stmts) => {
            writeln!(f, "{}if ({}) {{", indent, cond)?;
            fmt_stmts(then_stmts, &inner, f)?;
            if else_stmts.is_empty() {
                writeln!(f, "{}}}", indent)
            } else {
                writeln!(f, "{}}} else {{", indent)?;
                fmt_stmts(else_stmts, &inner, f)?;
                writeln!(f, "{}}}", indent)
            }
        }
        KotlinStmt::When(expr, branches, default_stmts) => {
            writeln!(f, "{}when ({}) {{", indent, expr)?;
            for (cond, stmts) in branches {
                writeln!(f, "{}{} -> {{", inner, cond)?;
                fmt_stmts(stmts, &format!("{}    ", inner), f)?;
                writeln!(f, "{}}}", inner)?;
            }
            if !default_stmts.is_empty() {
                writeln!(f, "{}else -> {{", inner)?;
                fmt_stmts(default_stmts, &format!("{}    ", inner), f)?;
                writeln!(f, "{}}}", inner)?;
            }
            writeln!(f, "{}}}", indent)
        }
    }
}
/// Set of Kotlin reserved keywords that must be mangled.
pub const KOTLIN_KEYWORDS: &[&str] = &[
    "abstract",
    "actual",
    "annotation",
    "as",
    "break",
    "by",
    "catch",
    "class",
    "companion",
    "const",
    "constructor",
    "continue",
    "crossinline",
    "data",
    "delegate",
    "do",
    "dynamic",
    "else",
    "enum",
    "expect",
    "external",
    "false",
    "field",
    "file",
    "final",
    "finally",
    "for",
    "fun",
    "get",
    "if",
    "import",
    "in",
    "infix",
    "init",
    "inline",
    "inner",
    "interface",
    "internal",
    "is",
    "it",
    "lateinit",
    "noinline",
    "null",
    "object",
    "open",
    "operator",
    "out",
    "override",
    "package",
    "param",
    "private",
    "property",
    "protected",
    "public",
    "receiver",
    "reified",
    "return",
    "sealed",
    "set",
    "setparam",
    "super",
    "suspend",
    "tailrec",
    "this",
    "throw",
    "true",
    "try",
    "typealias",
    "typeof",
    "val",
    "var",
    "vararg",
    "when",
    "where",
    "while",
];
pub(super) fn collect_ctor_names_from_expr(expr: &LcnfExpr, out: &mut HashSet<String>) {
    match expr {
        LcnfExpr::Let { value, body, .. } => {
            collect_ctor_names_from_value(value, out);
            collect_ctor_names_from_expr(body, out);
        }
        LcnfExpr::Case { alts, default, .. } => {
            for alt in alts {
                out.insert(alt.ctor_name.clone());
                collect_ctor_names_from_expr(&alt.body, out);
            }
            if let Some(d) = default {
                collect_ctor_names_from_expr(d, out);
            }
        }
        LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(_, _) => {}
    }
}
pub(super) fn collect_ctor_names_from_value(value: &LcnfLetValue, out: &mut HashSet<String>) {
    match value {
        LcnfLetValue::Ctor(name, _, _) => {
            out.insert(name.clone());
        }
        LcnfLetValue::Reuse(_, name, _, _) => {
            out.insert(name.clone());
        }
        _ => {}
    }
}
/// Minimal Kotlin runtime object emitted at the top of every generated file.
pub const KOTLIN_RUNTIME: &str = r#"
object OxiLeanRuntime {
    /** Called when pattern matching reaches a supposedly unreachable branch. */
    fun unreachable(): Nothing =
        throw IllegalStateException("OxiLean: unreachable code reached")

    /** Natural number addition (Long arithmetic). */
    fun natAdd(a: Long, b: Long): Long = a + b

    /** Natural number subtraction (saturating at 0). */
    fun natSub(a: Long, b: Long): Long = maxOf(0L, a - b)

    /** Natural number multiplication. */
    fun natMul(a: Long, b: Long): Long = a * b

    /** Natural number division (truncating). */
    fun natDiv(a: Long, b: Long): Long = if (b == 0L) 0L else a / b

    /** Natural number modulo. */
    fun natMod(a: Long, b: Long): Long = if (b == 0L) a else a % b

    /** Boolean decidable equality for Any. */
    fun decide(b: Boolean): Long = if (b) 1L else 0L

    /** String representation of a Long (Nat). */
    fun natToString(n: Long): String = n.toString()

    /** String append. */
    fun strAppend(a: String, b: String): String = a + b

    /** Pair constructor. */
    fun <A, B> mkPair(a: A, b: B): Pair<A, B> = Pair(a, b)

    /** List.cons: prepend element to list. */
    fun <A> cons(head: A, tail: List<A>): List<A> = listOf(head) + tail

    /** List.nil: empty list. */
    fun <A> nil(): List<A> = emptyList()
}
"#;
#[cfg(test)]
mod Kt_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = KtPassConfig::new("test_pass", KtPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = KtPassStats::new();
        stats.record_run(10, 100, 3);
        stats.record_run(20, 200, 5);
        assert_eq!(stats.total_runs, 2);
        assert!((stats.average_changes_per_run() - 15.0).abs() < 0.01);
        assert!((stats.success_rate() - 1.0).abs() < 0.01);
        let s = stats.format_summary();
        assert!(s.contains("Runs: 2/2"));
    }
    #[test]
    pub(super) fn test_pass_registry() {
        let mut reg = KtPassRegistry::new();
        reg.register(KtPassConfig::new("pass_a", KtPassPhase::Analysis));
        reg.register(KtPassConfig::new("pass_b", KtPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = KtAnalysisCache::new(10);
        cache.insert("key1".to_string(), vec![1, 2, 3]);
        assert!(cache.get("key1").is_some());
        assert!(cache.get("key2").is_none());
        assert!((cache.hit_rate() - 0.5).abs() < 0.01);
        cache.invalidate("key1");
        assert!(!cache.entries["key1"].valid);
        assert_eq!(cache.size(), 1);
    }
    #[test]
    pub(super) fn test_worklist() {
        let mut wl = KtWorklist::new();
        assert!(wl.push(1));
        assert!(wl.push(2));
        assert!(!wl.push(1));
        assert_eq!(wl.len(), 2);
        assert_eq!(wl.pop(), Some(1));
        assert!(!wl.contains(1));
        assert!(wl.contains(2));
    }
    #[test]
    pub(super) fn test_dominator_tree() {
        let mut dt = KtDominatorTree::new(5);
        dt.set_idom(1, 0);
        dt.set_idom(2, 0);
        dt.set_idom(3, 1);
        assert!(dt.dominates(0, 3));
        assert!(dt.dominates(1, 3));
        assert!(!dt.dominates(2, 3));
        assert!(dt.dominates(3, 3));
    }
    #[test]
    pub(super) fn test_liveness() {
        let mut liveness = KtLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(KtConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(KtConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(KtConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            KtConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(KtConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = KtDepGraph::new();
        g.add_dep(1, 2);
        g.add_dep(2, 3);
        g.add_dep(1, 3);
        assert_eq!(g.dependencies_of(2), vec![1]);
        let topo = g.topological_sort();
        assert_eq!(topo.len(), 3);
        assert!(!g.has_cycle());
        let pos: std::collections::HashMap<u32, usize> =
            topo.iter().enumerate().map(|(i, &n)| (n, i)).collect();
        assert!(pos[&1] < pos[&2]);
        assert!(pos[&1] < pos[&3]);
        assert!(pos[&2] < pos[&3]);
    }
}
#[cfg(test)]
mod kotlinext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_kotlinext_phase_order() {
        assert_eq!(KotlinExtPassPhase::Early.order(), 0);
        assert_eq!(KotlinExtPassPhase::Middle.order(), 1);
        assert_eq!(KotlinExtPassPhase::Late.order(), 2);
        assert_eq!(KotlinExtPassPhase::Finalize.order(), 3);
        assert!(KotlinExtPassPhase::Early.is_early());
        assert!(!KotlinExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_kotlinext_config_builder() {
        let c = KotlinExtPassConfig::new("p")
            .with_phase(KotlinExtPassPhase::Late)
            .with_max_iter(50)
            .with_debug(1);
        assert_eq!(c.name, "p");
        assert_eq!(c.max_iterations, 50);
        assert!(c.is_debug_enabled());
        assert!(c.enabled);
        let c2 = c.disabled();
        assert!(!c2.enabled);
    }
    #[test]
    pub(super) fn test_kotlinext_stats() {
        let mut s = KotlinExtPassStats::new();
        s.visit();
        s.visit();
        s.modify();
        s.iterate();
        assert_eq!(s.nodes_visited, 2);
        assert_eq!(s.nodes_modified, 1);
        assert!(s.changed);
        assert_eq!(s.iterations, 1);
        let e = s.efficiency();
        assert!((e - 0.5).abs() < 1e-9);
    }
    #[test]
    pub(super) fn test_kotlinext_registry() {
        let mut r = KotlinExtPassRegistry::new();
        r.register(KotlinExtPassConfig::new("a").with_phase(KotlinExtPassPhase::Early));
        r.register(KotlinExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&KotlinExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_kotlinext_cache() {
        let mut c = KotlinExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_kotlinext_worklist() {
        let mut w = KotlinExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_kotlinext_dom_tree() {
        let mut dt = KotlinExtDomTree::new(5);
        dt.set_idom(1, 0);
        dt.set_idom(2, 0);
        dt.set_idom(3, 1);
        dt.set_idom(4, 1);
        assert!(dt.dominates(0, 3));
        assert!(dt.dominates(1, 4));
        assert!(!dt.dominates(2, 3));
        assert_eq!(dt.depth_of(3), 2);
    }
    #[test]
    pub(super) fn test_kotlinext_liveness() {
        let mut lv = KotlinExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_kotlinext_const_folder() {
        let mut cf = KotlinExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_kotlinext_dep_graph() {
        let mut g = KotlinExtDepGraph::new(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        assert!(!g.has_cycle());
        assert_eq!(g.topo_sort(), Some(vec![0, 1, 2, 3]));
        assert_eq!(g.reachable(0).len(), 4);
        let sccs = g.scc();
        assert_eq!(sccs.len(), 4);
    }
}
#[cfg(test)]
mod kotlinx2_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_kotlinx2_phase_order() {
        assert_eq!(KotlinX2PassPhase::Early.order(), 0);
        assert_eq!(KotlinX2PassPhase::Middle.order(), 1);
        assert_eq!(KotlinX2PassPhase::Late.order(), 2);
        assert_eq!(KotlinX2PassPhase::Finalize.order(), 3);
        assert!(KotlinX2PassPhase::Early.is_early());
        assert!(!KotlinX2PassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_kotlinx2_config_builder() {
        let c = KotlinX2PassConfig::new("p")
            .with_phase(KotlinX2PassPhase::Late)
            .with_max_iter(50)
            .with_debug(1);
        assert_eq!(c.name, "p");
        assert_eq!(c.max_iterations, 50);
        assert!(c.is_debug_enabled());
        assert!(c.enabled);
        let c2 = c.disabled();
        assert!(!c2.enabled);
    }
    #[test]
    pub(super) fn test_kotlinx2_stats() {
        let mut s = KotlinX2PassStats::new();
        s.visit();
        s.visit();
        s.modify();
        s.iterate();
        assert_eq!(s.nodes_visited, 2);
        assert_eq!(s.nodes_modified, 1);
        assert!(s.changed);
        assert_eq!(s.iterations, 1);
        let e = s.efficiency();
        assert!((e - 0.5).abs() < 1e-9);
    }
    #[test]
    pub(super) fn test_kotlinx2_registry() {
        let mut r = KotlinX2PassRegistry::new();
        r.register(KotlinX2PassConfig::new("a").with_phase(KotlinX2PassPhase::Early));
        r.register(KotlinX2PassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&KotlinX2PassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_kotlinx2_cache() {
        let mut c = KotlinX2Cache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_kotlinx2_worklist() {
        let mut w = KotlinX2Worklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_kotlinx2_dom_tree() {
        let mut dt = KotlinX2DomTree::new(5);
        dt.set_idom(1, 0);
        dt.set_idom(2, 0);
        dt.set_idom(3, 1);
        dt.set_idom(4, 1);
        assert!(dt.dominates(0, 3));
        assert!(dt.dominates(1, 4));
        assert!(!dt.dominates(2, 3));
        assert_eq!(dt.depth_of(3), 2);
    }
    #[test]
    pub(super) fn test_kotlinx2_liveness() {
        let mut lv = KotlinX2Liveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_kotlinx2_const_folder() {
        let mut cf = KotlinX2ConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_kotlinx2_dep_graph() {
        let mut g = KotlinX2DepGraph::new(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        assert!(!g.has_cycle());
        assert_eq!(g.topo_sort(), Some(vec![0, 1, 2, 3]));
        assert_eq!(g.reachable(0).len(), 4);
        let sccs = g.scc();
        assert_eq!(sccs.len(), 4);
    }
}
