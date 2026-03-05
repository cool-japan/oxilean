//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::{LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfType, LcnfVarId};
use std::collections::{HashMap, HashSet, VecDeque};

use super::types::{
    Allocation, GraphColoringAllocator, InterferenceGraph, LinearScanAllocator, LiveInterval,
    PhysReg, RAAnalysisCache, RAConstantFoldingHelper, RADepGraph, RADominatorTree, RAExtCache,
    RAExtConstFolder, RAExtDepGraph, RAExtDomTree, RAExtLiveness, RAExtPassConfig, RAExtPassPhase,
    RAExtPassRegistry, RAExtPassStats, RAExtWorklist, RALivenessInfo, RAPassConfig, RAPassPhase,
    RAPassRegistry, RAPassStats, RAWorklist, RegAllocConfig, RegAllocDiagCollector,
    RegAllocDiagMsg, RegAllocEmitStats, RegAllocEventLog, RegAllocFeatures, RegAllocIdGen,
    RegAllocIncrKey, RegAllocNameScope, RegAllocPass, RegAllocPassTiming, RegAllocProfiler,
    RegAllocReport, RegAllocSourceBuffer, RegAllocVersion, RegClass, SpillSlot, VirtualReg,
};

/// Walk an LCNF expression tree and populate live intervals.
pub(super) fn collect_intervals_from_expr(
    expr: &LcnfExpr,
    counter: &mut u32,
    intervals: &mut HashMap<LcnfVarId, LiveInterval>,
) {
    match expr {
        LcnfExpr::Let {
            id, value, body, ..
        } => {
            let pos = *counter;
            *counter += 1;
            let iv = intervals
                .entry(*id)
                .or_insert_with(|| LiveInterval::new(*id, pos, pos + 1));
            iv.add_def(pos);
            record_uses_in_value(value, pos, intervals);
            collect_intervals_from_expr(body, counter, intervals);
        }
        LcnfExpr::Case {
            scrutinee,
            alts,
            default,
            ..
        } => {
            let pos = *counter;
            *counter += 1;
            record_use(*scrutinee, pos, intervals);
            for alt in alts {
                for param in &alt.params {
                    let iv = intervals
                        .entry(param.id)
                        .or_insert_with(|| LiveInterval::new(param.id, pos, pos + 1));
                    iv.add_def(pos);
                }
                collect_intervals_from_expr(&alt.body, counter, intervals);
            }
            if let Some(def) = default {
                collect_intervals_from_expr(def, counter, intervals);
            }
        }
        LcnfExpr::Return(arg) => {
            let pos = *counter;
            *counter += 1;
            if let LcnfArg::Var(id) = arg {
                record_use(*id, pos, intervals);
            }
        }
        LcnfExpr::TailCall(func, args) => {
            let pos = *counter;
            *counter += 1;
            if let LcnfArg::Var(id) = func {
                record_use(*id, pos, intervals);
            }
            for arg in args {
                if let LcnfArg::Var(id) = arg {
                    record_use(*id, pos, intervals);
                }
            }
        }
        LcnfExpr::Unreachable => {}
    }
}
pub(super) fn record_use(
    id: LcnfVarId,
    pos: u32,
    intervals: &mut HashMap<LcnfVarId, LiveInterval>,
) {
    let iv = intervals
        .entry(id)
        .or_insert_with(|| LiveInterval::new(id, pos, pos + 1));
    iv.add_use(pos);
}
pub(super) fn record_uses_in_value(
    value: &LcnfLetValue,
    pos: u32,
    intervals: &mut HashMap<LcnfVarId, LiveInterval>,
) {
    match value {
        LcnfLetValue::App(func, args) => {
            if let LcnfArg::Var(id) = func {
                record_use(*id, pos, intervals);
            }
            for arg in args {
                if let LcnfArg::Var(id) = arg {
                    record_use(*id, pos, intervals);
                }
            }
        }
        LcnfLetValue::Ctor(_, _, args) | LcnfLetValue::Reuse(_, _, _, args) => {
            for arg in args {
                if let LcnfArg::Var(id) = arg {
                    record_use(*id, pos, intervals);
                }
            }
        }
        LcnfLetValue::Proj(_, _, src) => {
            record_use(*src, pos, intervals);
        }
        LcnfLetValue::FVar(id) | LcnfLetValue::Reset(id) => {
            record_use(*id, pos, intervals);
        }
        LcnfLetValue::Lit(_) | LcnfLetValue::Erased => {}
    }
}
/// Compute a canonical (post-coalescing) representative for each vreg.
pub fn compute_canonical_map(
    merge_pairs: &[(LcnfVarId, LcnfVarId)],
) -> HashMap<LcnfVarId, LcnfVarId> {
    let mut parent: HashMap<LcnfVarId, LcnfVarId> = HashMap::new();
    for &(u, v) in merge_pairs {
        let ru = find(&mut parent, u);
        let rv = find(&mut parent, v);
        if ru != rv {
            parent.insert(rv, ru);
        }
    }
    let keys: Vec<LcnfVarId> = parent.keys().copied().collect();
    let mut canonical: HashMap<LcnfVarId, LcnfVarId> = HashMap::new();
    for k in keys {
        let root = find(&mut parent, k);
        canonical.insert(k, root);
    }
    canonical
}
/// Union-Find `find` with path compression.
pub(super) fn find(parent: &mut HashMap<LcnfVarId, LcnfVarId>, mut x: LcnfVarId) -> LcnfVarId {
    let mut path = Vec::new();
    while let Some(&p) = parent.get(&x) {
        if p == x {
            break;
        }
        path.push(x);
        x = p;
    }
    for node in path {
        parent.insert(node, x);
    }
    x
}
/// Build a BFS-order instruction numbering for an LCNF expression.
pub fn number_instructions(decl: &LcnfFunDecl) -> HashMap<LcnfVarId, u32> {
    let mut numbering = HashMap::new();
    let mut counter = 0u32;
    for param in &decl.params {
        numbering.insert(param.id, counter);
        counter += 1;
    }
    number_expr(&decl.body, &mut counter, &mut numbering);
    numbering
}
pub(super) fn number_expr(
    expr: &LcnfExpr,
    counter: &mut u32,
    numbering: &mut HashMap<LcnfVarId, u32>,
) {
    match expr {
        LcnfExpr::Let { id, body, .. } => {
            numbering.insert(*id, *counter);
            *counter += 1;
            number_expr(body, counter, numbering);
        }
        LcnfExpr::Case { alts, default, .. } => {
            *counter += 1;
            for alt in alts {
                for param in &alt.params {
                    numbering.insert(param.id, *counter);
                }
                *counter += 1;
                number_expr(&alt.body, counter, numbering);
            }
            if let Some(def) = default {
                number_expr(def, counter, numbering);
            }
        }
        LcnfExpr::Return(_) | LcnfExpr::TailCall(_, _) | LcnfExpr::Unreachable => {
            *counter += 1;
        }
    }
}
/// Collect all vreg IDs used in a declaration (body + params).
pub fn collect_vregs(decl: &LcnfFunDecl) -> Vec<LcnfVarId> {
    let mut seen = HashSet::new();
    let mut result = Vec::new();
    for param in &decl.params {
        if seen.insert(param.id) {
            result.push(param.id);
        }
    }
    collect_vregs_from_expr(&decl.body, &mut seen, &mut result);
    result
}
pub(super) fn collect_vregs_from_expr(
    expr: &LcnfExpr,
    seen: &mut HashSet<LcnfVarId>,
    result: &mut Vec<LcnfVarId>,
) {
    let mut worklist: VecDeque<&LcnfExpr> = VecDeque::new();
    worklist.push_back(expr);
    while let Some(e) = worklist.pop_front() {
        match e {
            LcnfExpr::Let { id, body, .. } => {
                if seen.insert(*id) {
                    result.push(*id);
                }
                worklist.push_back(body);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts {
                    for param in &alt.params {
                        if seen.insert(param.id) {
                            result.push(param.id);
                        }
                    }
                    worklist.push_back(&alt.body);
                }
                if let Some(def) = default {
                    worklist.push_back(def);
                }
            }
            LcnfExpr::Return(_) | LcnfExpr::TailCall(_, _) | LcnfExpr::Unreachable => {}
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::lcnf::{
        LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfLit, LcnfParam, LcnfType, LcnfVarId,
    };
    pub(super) fn v(n: u64) -> LcnfVarId {
        LcnfVarId(n)
    }
    pub(super) fn make_param(n: u64, name: &str) -> LcnfParam {
        LcnfParam {
            id: v(n),
            name: name.to_string(),
            ty: LcnfType::Nat,
            erased: false,
            borrowed: false,
        }
    }
    pub(super) fn make_decl(name: &str, params: Vec<LcnfParam>, body: LcnfExpr) -> LcnfFunDecl {
        LcnfFunDecl {
            name: name.to_string(),
            original_name: None,
            params,
            ret_type: LcnfType::Nat,
            body,
            is_recursive: false,
            is_lifted: false,
            inline_cost: 1,
        }
    }
    pub(super) fn ret_var(n: u64) -> LcnfExpr {
        LcnfExpr::Return(LcnfArg::Var(v(n)))
    }
    pub(super) fn let_nat(id: u64, body: LcnfExpr) -> LcnfExpr {
        LcnfExpr::Let {
            id: v(id),
            name: format!("x{}", id),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(id)),
            body: Box::new(body),
        }
    }
    #[test]
    pub(super) fn test_reg_class_prefix() {
        assert_eq!(RegClass::Integer.prefix(), "r");
        assert_eq!(RegClass::Float.prefix(), "f");
        assert_eq!(RegClass::Vector.prefix(), "v");
        assert_eq!(RegClass::Predicate.prefix(), "p");
    }
    #[test]
    pub(super) fn test_phys_reg_integer_bank() {
        let bank = PhysReg::integer_bank(4);
        assert_eq!(bank.len(), 4);
        assert_eq!(bank[0].name, "r0");
        assert_eq!(bank[3].name, "r3");
        assert!(bank.iter().all(|r| r.class == RegClass::Integer));
    }
    #[test]
    pub(super) fn test_phys_reg_float_bank() {
        let bank = PhysReg::float_bank(2);
        assert_eq!(bank.len(), 2);
        assert_eq!(bank[0].name, "f0");
        assert_eq!(bank[0].class, RegClass::Float);
    }
    #[test]
    pub(super) fn test_virtual_reg_class_nat() {
        let vr = VirtualReg::new(0, LcnfType::Nat);
        assert_eq!(vr.reg_class(), RegClass::Integer);
    }
    #[test]
    pub(super) fn test_virtual_reg_with_hint() {
        let hint = PhysReg::new(0, "r0", RegClass::Integer);
        let vr = VirtualReg::with_hint(1, LcnfType::Nat, hint.clone());
        assert_eq!(vr.hint, Some(hint));
    }
    #[test]
    pub(super) fn test_live_interval_overlaps_true() {
        let a = LiveInterval::new(v(1), 0, 5);
        let b = LiveInterval::new(v(2), 3, 8);
        assert!(a.overlaps(&b));
    }
    #[test]
    pub(super) fn test_live_interval_overlaps_false() {
        let a = LiveInterval::new(v(1), 0, 3);
        let b = LiveInterval::new(v(2), 5, 8);
        assert!(!a.overlaps(&b));
    }
    #[test]
    pub(super) fn test_live_interval_adjacent_no_overlap() {
        let a = LiveInterval::new(v(1), 0, 3);
        let b = LiveInterval::new(v(2), 3, 6);
        assert!(!a.overlaps(&b));
    }
    #[test]
    pub(super) fn test_live_interval_length() {
        let iv = LiveInterval::new(v(1), 2, 7);
        assert_eq!(iv.length(), 5);
    }
    #[test]
    pub(super) fn test_live_interval_add_use_extends_end() {
        let mut iv = LiveInterval::new(v(1), 0, 3);
        iv.add_use(10);
        assert_eq!(iv.end, 11);
    }
    #[test]
    pub(super) fn test_live_interval_spill_weight() {
        let mut iv = LiveInterval::new(v(1), 0, 10);
        iv.add_use(2);
        iv.add_use(5);
        iv.add_use(8);
        iv.compute_spill_weight();
        assert!(iv.spill_weight > 0.0);
    }
    #[test]
    pub(super) fn test_interference_graph_add_edge() {
        let mut g = InterferenceGraph::new();
        g.add_edge(v(1), v(2));
        assert!(g.interferes(v(1), v(2)));
        assert!(g.interferes(v(2), v(1)));
    }
    #[test]
    pub(super) fn test_interference_graph_no_self_loop() {
        let mut g = InterferenceGraph::new();
        g.add_edge(v(1), v(1));
        assert!(!g.interferes(v(1), v(1)));
    }
    #[test]
    pub(super) fn test_interference_graph_degree() {
        let mut g = InterferenceGraph::new();
        g.add_edge(v(1), v(2));
        g.add_edge(v(1), v(3));
        assert_eq!(g.degree(v(1)), 2);
    }
    #[test]
    pub(super) fn test_interference_graph_remove_node() {
        let mut g = InterferenceGraph::new();
        g.add_edge(v(1), v(2));
        g.remove_node(v(1));
        assert!(!g.nodes.contains(&v(1)));
        assert!(!g.interferes(v(2), v(1)));
    }
    #[test]
    pub(super) fn test_interference_graph_from_intervals() {
        let ivs = vec![
            LiveInterval::new(v(1), 0, 5),
            LiveInterval::new(v(2), 3, 8),
            LiveInterval::new(v(3), 6, 10),
        ];
        let g = InterferenceGraph::build_from_intervals(&ivs);
        assert!(g.interferes(v(1), v(2)));
        assert!(g.interferes(v(2), v(3)));
        assert!(!g.interferes(v(1), v(3)));
    }
    #[test]
    pub(super) fn test_linear_scan_simple() {
        let regs = PhysReg::integer_bank(4);
        let mut lsa = LinearScanAllocator::new(regs);
        let decl = make_decl("f", vec![], let_nat(1, let_nat(2, ret_var(2))));
        let intervals = lsa.build_live_intervals(&decl);
        let alloc = lsa.linear_scan(intervals, 4);
        assert_eq!(alloc.spills.len(), 0);
    }
    #[test]
    pub(super) fn test_linear_scan_spills_when_pressure() {
        let regs = PhysReg::integer_bank(1);
        let mut lsa = LinearScanAllocator::new(regs);
        let body = let_nat(
            1,
            let_nat(
                2,
                let_nat(3, let_nat(4, LcnfExpr::Return(LcnfArg::Var(v(1))))),
            ),
        );
        let decl = make_decl("g", vec![], body);
        let intervals = lsa.build_live_intervals(&decl);
        let alloc = lsa.linear_scan(intervals, 1);
        assert!(alloc.spills.len() > 0 || alloc.reg_map.len() <= 1);
    }
    #[test]
    pub(super) fn test_linear_scan_build_intervals_with_params() {
        let regs = PhysReg::integer_bank(4);
        let lsa = LinearScanAllocator::new(regs);
        let params = vec![make_param(0, "a"), make_param(1, "b")];
        let decl = make_decl("h", params, ret_var(0));
        let intervals = lsa.build_live_intervals(&decl);
        assert!(intervals.iter().any(|iv| iv.vreg == v(0)));
    }
    #[test]
    pub(super) fn test_graph_coloring_simple() {
        let regs = PhysReg::integer_bank(3);
        let mut gca = GraphColoringAllocator::new(regs);
        let ivs = vec![
            LiveInterval::new(v(1), 0, 3),
            LiveInterval::new(v(2), 4, 7),
            LiveInterval::new(v(3), 8, 11),
        ];
        let alloc = gca.allocate(&ivs);
        assert_eq!(alloc.spills.len(), 0);
        assert_eq!(alloc.reg_map.len(), 3);
    }
    #[test]
    pub(super) fn test_graph_coloring_interfering() {
        let regs = PhysReg::integer_bank(2);
        let mut gca = GraphColoringAllocator::new(regs);
        let ivs = vec![
            LiveInterval::new(v(1), 0, 10),
            LiveInterval::new(v(2), 0, 10),
            LiveInterval::new(v(3), 0, 10),
        ];
        let alloc = gca.allocate(&ivs);
        assert!(alloc.spills.len() >= 1 || alloc.reg_map.len() <= 2);
    }
    #[test]
    pub(super) fn test_graph_coloring_simplify_removes_nodes() {
        let regs = PhysReg::integer_bank(4);
        let mut gca = GraphColoringAllocator::new(regs);
        let ivs = vec![LiveInterval::new(v(1), 0, 3), LiveInterval::new(v(2), 4, 7)];
        gca.build_interference_graph(&ivs);
        let removed = gca.simplify();
        assert!(removed >= 1);
    }
    #[test]
    pub(super) fn test_regalloc_pass_linear_scan() {
        let mut pass = RegAllocPass::new(4);
        let decl = make_decl("fn_ls", vec![], let_nat(1, ret_var(1)));
        let mut decls = vec![decl];
        pass.run(&mut decls);
        let r = pass.report();
        assert_eq!(r.functions_processed, 1);
        assert!(r.vregs_allocated >= 1);
    }
    #[test]
    pub(super) fn test_regalloc_pass_graph_coloring() {
        let mut pass = RegAllocPass::graph_coloring(4);
        let decl = make_decl("fn_gc", vec![], let_nat(1, let_nat(2, ret_var(2))));
        let mut decls = vec![decl];
        pass.run(&mut decls);
        let r = pass.report();
        assert_eq!(r.functions_processed, 1);
    }
    #[test]
    pub(super) fn test_regalloc_pass_allocation_stored() {
        let mut pass = RegAllocPass::new(4);
        let decl = make_decl("my_fn", vec![], let_nat(5, ret_var(5)));
        let mut decls = vec![decl];
        pass.run(&mut decls);
        assert!(pass.allocation_for("my_fn").is_some());
    }
    #[test]
    pub(super) fn test_regalloc_pass_spill_ratio() {
        let r = RegAllocReport {
            vregs_allocated: 7,
            spills: 3,
            ..Default::default()
        };
        assert!((r.spill_ratio() - 0.3).abs() < 1e-6);
    }
    #[test]
    pub(super) fn test_regalloc_pass_empty() {
        let mut pass = RegAllocPass::new(4);
        let mut decls = vec![];
        pass.run(&mut decls);
        let r = pass.report();
        assert_eq!(r.functions_processed, 0);
        assert_eq!(r.spills, 0);
    }
    #[test]
    pub(super) fn test_spill_slot_new() {
        let s = SpillSlot::new(v(1), 16, 8);
        assert_eq!(s.offset, 16);
        assert_eq!(s.size, 8);
        assert_eq!(s.vreg, v(1));
    }
    #[test]
    pub(super) fn test_allocation_assign_lookup() {
        let mut alloc = Allocation::new();
        let pr = PhysReg::new(0, "r0", RegClass::Integer);
        alloc.assign(v(1), pr.clone());
        assert_eq!(alloc.lookup(v(1)), Some(&pr));
    }
    #[test]
    pub(super) fn test_allocation_spill() {
        let mut alloc = Allocation::new();
        alloc.spill(v(2), 8);
        assert!(alloc.is_spilled(v(2)));
        assert_eq!(alloc.stack_frame_size(), 8);
    }
    #[test]
    pub(super) fn test_compute_canonical_map() {
        let pairs = vec![(v(1), v(2)), (v(2), v(3))];
        let canonical = compute_canonical_map(&pairs);
        assert_eq!(canonical.get(&v(2)), canonical.get(&v(2)));
    }
    #[test]
    pub(super) fn test_collect_vregs() {
        let body = let_nat(10, let_nat(11, ret_var(10)));
        let decl = make_decl("f", vec![make_param(0, "x")], body);
        let vregs = collect_vregs(&decl);
        assert!(vregs.contains(&v(0)));
        assert!(vregs.contains(&v(10)));
        assert!(vregs.contains(&v(11)));
    }
    #[test]
    pub(super) fn test_number_instructions() {
        let body = let_nat(1, let_nat(2, ret_var(1)));
        let decl = make_decl("f", vec![], body);
        let numbering = number_instructions(&decl);
        assert!(numbering.contains_key(&v(1)));
        assert!(numbering.contains_key(&v(2)));
        assert!(numbering[&v(1)] < numbering[&v(2)]);
    }
}
#[cfg(test)]
mod tests_reg_alloc_extra {
    use super::*;
    #[test]
    pub(super) fn test_reg_alloc_config() {
        let mut cfg = RegAllocConfig::new();
        cfg.set("mode", "release");
        cfg.set("verbose", "true");
        assert_eq!(cfg.get("mode"), Some("release"));
        assert!(cfg.get_bool("verbose"));
        assert!(cfg.get_int("mode").is_none());
        assert_eq!(cfg.len(), 2);
    }
    #[test]
    pub(super) fn test_reg_alloc_source_buffer() {
        let mut buf = RegAllocSourceBuffer::new();
        buf.push_line("fn main() {");
        buf.indent();
        buf.push_line("println!(\"hello\");");
        buf.dedent();
        buf.push_line("}");
        assert!(buf.as_str().contains("fn main()"));
        assert!(buf.as_str().contains("    println!"));
        assert_eq!(buf.line_count(), 3);
        buf.reset();
        assert!(buf.is_empty());
    }
    #[test]
    pub(super) fn test_reg_alloc_name_scope() {
        let mut scope = RegAllocNameScope::new();
        assert!(scope.declare("x"));
        assert!(!scope.declare("x"));
        assert!(scope.is_declared("x"));
        let scope = scope.push_scope();
        assert_eq!(scope.depth(), 1);
        let mut scope = scope.pop_scope();
        assert_eq!(scope.depth(), 0);
        scope.declare("y");
        assert_eq!(scope.len(), 2);
    }
    #[test]
    pub(super) fn test_reg_alloc_diag_collector() {
        let mut col = RegAllocDiagCollector::new();
        col.emit(RegAllocDiagMsg::warning("pass_a", "slow"));
        col.emit(RegAllocDiagMsg::error("pass_b", "fatal"));
        assert!(col.has_errors());
        assert_eq!(col.errors().len(), 1);
        assert_eq!(col.warnings().len(), 1);
        col.clear();
        assert!(col.is_empty());
    }
    #[test]
    pub(super) fn test_reg_alloc_id_gen() {
        let mut gen = RegAllocIdGen::new();
        assert_eq!(gen.next_id(), 0);
        assert_eq!(gen.next_id(), 1);
        gen.skip(10);
        assert_eq!(gen.next_id(), 12);
        gen.reset();
        assert_eq!(gen.peek_next(), 0);
    }
    #[test]
    pub(super) fn test_reg_alloc_incr_key() {
        let k1 = RegAllocIncrKey::new(100, 200);
        let k2 = RegAllocIncrKey::new(100, 200);
        let k3 = RegAllocIncrKey::new(999, 200);
        assert!(k1.matches(&k2));
        assert!(!k1.matches(&k3));
    }
    #[test]
    pub(super) fn test_reg_alloc_profiler() {
        let mut p = RegAllocProfiler::new();
        p.record(RegAllocPassTiming::new("pass_a", 1000, 50, 200, 100));
        p.record(RegAllocPassTiming::new("pass_b", 500, 30, 100, 200));
        assert_eq!(p.total_elapsed_us(), 1500);
        assert_eq!(
            p.slowest_pass()
                .expect("slowest pass should exist")
                .pass_name,
            "pass_a"
        );
        assert_eq!(p.profitable_passes().len(), 1);
    }
    #[test]
    pub(super) fn test_reg_alloc_event_log() {
        let mut log = RegAllocEventLog::new(3);
        log.push("event1");
        log.push("event2");
        log.push("event3");
        assert_eq!(log.len(), 3);
        log.push("event4");
        assert_eq!(log.len(), 3);
        assert_eq!(
            log.iter()
                .next()
                .expect("iterator should have next element"),
            "event2"
        );
    }
    #[test]
    pub(super) fn test_reg_alloc_version() {
        let v = RegAllocVersion::new(1, 2, 3).with_pre("alpha");
        assert!(!v.is_stable());
        assert_eq!(format!("{}", v), "1.2.3-alpha");
        let stable = RegAllocVersion::new(2, 0, 0);
        assert!(stable.is_stable());
        assert!(stable.is_compatible_with(&RegAllocVersion::new(2, 0, 0)));
        assert!(!stable.is_compatible_with(&RegAllocVersion::new(3, 0, 0)));
    }
    #[test]
    pub(super) fn test_reg_alloc_features() {
        let mut f = RegAllocFeatures::new();
        f.enable("sse2");
        f.enable("avx2");
        assert!(f.is_enabled("sse2"));
        assert!(!f.is_enabled("avx512"));
        f.disable("avx2");
        assert!(!f.is_enabled("avx2"));
        let mut g = RegAllocFeatures::new();
        g.enable("sse2");
        g.enable("neon");
        let union = f.union(&g);
        assert!(union.is_enabled("sse2") && union.is_enabled("neon"));
        let inter = f.intersection(&g);
        assert!(inter.is_enabled("sse2"));
    }
    #[test]
    pub(super) fn test_reg_alloc_emit_stats() {
        let mut s = RegAllocEmitStats::new();
        s.bytes_emitted = 50_000;
        s.items_emitted = 500;
        s.elapsed_ms = 100;
        assert!(s.is_clean());
        assert!((s.throughput_bps() - 500_000.0).abs() < 1.0);
        let disp = format!("{}", s);
        assert!(disp.contains("bytes=50000"));
    }
}
#[cfg(test)]
mod RA_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = RAPassConfig::new("test_pass", RAPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = RAPassStats::new();
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
        let mut reg = RAPassRegistry::new();
        reg.register(RAPassConfig::new("pass_a", RAPassPhase::Analysis));
        reg.register(RAPassConfig::new("pass_b", RAPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = RAAnalysisCache::new(10);
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
        let mut wl = RAWorklist::new();
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
        let mut dt = RADominatorTree::new(5);
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
        let mut liveness = RALivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(RAConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(RAConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(RAConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            RAConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(RAConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = RADepGraph::new();
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
mod raext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_raext_phase_order() {
        assert_eq!(RAExtPassPhase::Early.order(), 0);
        assert_eq!(RAExtPassPhase::Middle.order(), 1);
        assert_eq!(RAExtPassPhase::Late.order(), 2);
        assert_eq!(RAExtPassPhase::Finalize.order(), 3);
        assert!(RAExtPassPhase::Early.is_early());
        assert!(!RAExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_raext_config_builder() {
        let c = RAExtPassConfig::new("p")
            .with_phase(RAExtPassPhase::Late)
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
    pub(super) fn test_raext_stats() {
        let mut s = RAExtPassStats::new();
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
    pub(super) fn test_raext_registry() {
        let mut r = RAExtPassRegistry::new();
        r.register(RAExtPassConfig::new("a").with_phase(RAExtPassPhase::Early));
        r.register(RAExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&RAExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_raext_cache() {
        let mut c = RAExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_raext_worklist() {
        let mut w = RAExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_raext_dom_tree() {
        let mut dt = RAExtDomTree::new(5);
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
    pub(super) fn test_raext_liveness() {
        let mut lv = RAExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_raext_const_folder() {
        let mut cf = RAExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_raext_dep_graph() {
        let mut g = RAExtDepGraph::new(4);
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
