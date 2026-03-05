//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::HashMap;

use super::types::{
    InductionVariable, LinearFunction, OSAnalysisCache, OSConstantFoldingHelper, OSDepGraph,
    OSDominatorTree, OSLivenessInfo, OSPassConfig, OSPassPhase, OSPassRegistry, OSPassStats,
    OSWorklist, SRExtCache, SRExtConstFolder, SRExtDepGraph, SRExtDomTree, SRExtLiveness,
    SRExtPassConfig, SRExtPassPhase, SRExtPassRegistry, SRExtPassStats, SRExtWorklist, SRX2Cache,
    SRX2ConstFolder, SRX2DepGraph, SRX2DomTree, SRX2Liveness, SRX2PassConfig, SRX2PassPhase,
    SRX2PassRegistry, SRX2PassStats, SRX2Worklist, StrengthConfig, StrengthReduceRule,
    StrengthReductionPass, StrengthReport,
};

/// Collect all let-bindings of the form `y = a * iv + b`.
pub(super) fn collect_linear_uses(
    expr: &LcnfExpr,
    iv: LcnfVarId,
) -> HashMap<LcnfVarId, LinearFunction> {
    let mut map = HashMap::new();
    collect_linear_uses_inner(expr, iv, &mut map);
    map
}
pub(super) fn collect_linear_uses_inner(
    expr: &LcnfExpr,
    iv: LcnfVarId,
    map: &mut HashMap<LcnfVarId, LinearFunction>,
) {
    match expr {
        LcnfExpr::Let {
            id, value, body, ..
        } => {
            if let LcnfLetValue::App(LcnfArg::Lit(LcnfLit::Str(fname)), args) = value {
                if fname == "mul" && args.len() == 2 {
                    let (a_coeff, has_iv) = extract_mul_iv(iv, args);
                    if has_iv {
                        map.insert(*id, LinearFunction::new(iv, a_coeff, 0));
                    }
                }
            }
            collect_linear_uses_inner(body, iv, map);
        }
        LcnfExpr::Case { alts, default, .. } => {
            for alt in alts {
                collect_linear_uses_inner(&alt.body, iv, map);
            }
            if let Some(d) = default {
                collect_linear_uses_inner(d, iv, map);
            }
        }
        LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(_, _) => {}
    }
}
pub(super) fn extract_mul_iv(iv: LcnfVarId, args: &[LcnfArg]) -> (i64, bool) {
    match (&args[0], &args[1]) {
        (LcnfArg::Var(v), LcnfArg::Lit(LcnfLit::Nat(c))) if *v == iv => (*c as i64, true),
        (LcnfArg::Lit(LcnfLit::Nat(c)), LcnfArg::Var(v)) if *v == iv => (*c as i64, true),
        _ => (0, false),
    }
}
pub(super) fn rewrite_linear_uses(
    expr: &LcnfExpr,
    _linears: &HashMap<LcnfVarId, LinearFunction>,
    _iv: &InductionVariable,
) -> LcnfExpr {
    expr.clone()
}
/// Return `true` iff `n` is a power of two (and > 0).
pub fn is_power_of_two(n: u64) -> bool {
    n > 0 && (n & (n - 1)) == 0
}
/// Integer log2 for a power-of-two value.
pub fn log2(n: u64) -> u32 {
    debug_assert!(is_power_of_two(n));
    n.trailing_zeros()
}
/// Extract the variable argument from an App (the operand that is not a
/// literal constant).
pub(super) fn var_arg_of(func: &LcnfArg, args: &[LcnfArg]) -> LcnfArg {
    if args.len() == 2 {
        if const_arg(&args[0]).is_some() {
            return args[1].clone();
        }
        if const_arg(&args[1]).is_some() {
            return args[0].clone();
        }
        return args[0].clone();
    }
    if args.len() == 1 {
        return args[0].clone();
    }
    func.clone()
}
/// If `arg` is a compile-time `Nat` literal, return its value.
pub(super) fn const_arg(arg: &LcnfArg) -> Option<u64> {
    match arg {
        LcnfArg::Lit(LcnfLit::Nat(n)) => Some(*n),
        _ => None,
    }
}
/// Decompose `c` into a sum of signed powers-of-two within `budget` terms.
///
/// Returns `Some(ops)` where each op is `(shift, sign)`, meaning
/// `x * c = sum_i (x << ops[i].0) * ops[i].1`.
///
/// Returns `None` if more than `budget` terms would be needed.
pub fn decompose_mul(c: u64, budget: u32) -> Option<Vec<(u32, i64)>> {
    if c == 0 {
        return Some(vec![]);
    }
    let mut ops: Vec<(u32, i64)> = vec![];
    let mut val = c as i64;
    let mut bit = 0u32;
    while val != 0 {
        if val & 1 != 0 {
            if (val & 3) == 3 {
                ops.push((bit, -1));
                val += 1;
            } else {
                ops.push((bit, 1));
                val -= 1;
            }
        }
        val >>= 1;
        bit += 1;
    }
    if ops.len() as u32 <= budget {
        Some(ops)
    } else {
        None
    }
}
/// Run strength reduction on a vector of declarations with default config.
pub fn optimize_strength(decls: &mut [LcnfFunDecl]) {
    StrengthReductionPass::default().run(decls);
}
#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn make_var(id: u64) -> LcnfVarId {
        LcnfVarId(id)
    }
    pub(super) fn app(op: &str, args: Vec<LcnfArg>) -> LcnfLetValue {
        LcnfLetValue::App(LcnfArg::Lit(LcnfLit::Str(op.into())), args)
    }
    pub(super) fn nat(n: u64) -> LcnfArg {
        LcnfArg::Lit(LcnfLit::Nat(n))
    }
    pub(super) fn var(id: u64) -> LcnfArg {
        LcnfArg::Var(make_var(id))
    }
    pub(super) fn let_expr(id: u64, value: LcnfLetValue, body: LcnfExpr) -> LcnfExpr {
        LcnfExpr::Let {
            id: make_var(id),
            name: format!("v{}", id),
            ty: LcnfType::Nat,
            value,
            body: Box::new(body),
        }
    }
    pub(super) fn ret(id: u64) -> LcnfExpr {
        LcnfExpr::Return(var(id))
    }
    #[test]
    pub(super) fn test_pow2_detection() {
        assert!(is_power_of_two(1));
        assert!(is_power_of_two(2));
        assert!(is_power_of_two(4));
        assert!(is_power_of_two(8));
        assert!(is_power_of_two(16));
        assert!(is_power_of_two(1024));
        assert!(!is_power_of_two(0));
        assert!(!is_power_of_two(3));
        assert!(!is_power_of_two(6));
        assert!(!is_power_of_two(7));
    }
    #[test]
    pub(super) fn test_log2() {
        assert_eq!(log2(1), 0);
        assert_eq!(log2(2), 1);
        assert_eq!(log2(4), 2);
        assert_eq!(log2(8), 3);
        assert_eq!(log2(256), 8);
    }
    #[test]
    pub(super) fn test_mul_by_pow2_becomes_shl() {
        let expr = let_expr(1, app("mul", vec![var(0), nat(4)]), ret(1));
        let mut pass = StrengthReductionPass::default();
        let result = pass.reduce_expr(&expr);
        if let LcnfExpr::Let { value, .. } = &result {
            match value {
                LcnfLetValue::App(LcnfArg::Lit(LcnfLit::Str(name)), args) => {
                    assert_eq!(name, "shl");
                    assert_eq!(args[1], nat(2));
                }
                _ => panic!("expected App(shl, ...)"),
            }
        }
        assert_eq!(pass.report().mul_reduced, 1);
    }
    #[test]
    pub(super) fn test_mul_by_pow2_const_on_left() {
        let expr = let_expr(1, app("mul", vec![nat(8), var(0)]), ret(1));
        let mut pass = StrengthReductionPass::default();
        let result = pass.reduce_expr(&expr);
        if let LcnfExpr::Let { value, .. } = &result {
            if let LcnfLetValue::App(LcnfArg::Lit(LcnfLit::Str(name)), args) = value {
                assert_eq!(name, "shl");
                assert_eq!(args[1], nat(3));
            } else {
                panic!("expected shl");
            }
        }
    }
    #[test]
    pub(super) fn test_div_by_pow2_becomes_lshr() {
        let expr = let_expr(1, app("div", vec![var(0), nat(16)]), ret(1));
        let mut pass = StrengthReductionPass::default();
        pass.reduce_expr(&expr);
        assert_eq!(pass.report().div_reduced, 1);
    }
    #[test]
    pub(super) fn test_mod_by_pow2_becomes_band() {
        let expr = let_expr(1, app("mod", vec![var(0), nat(8)]), ret(1));
        let mut pass = StrengthReductionPass::default();
        let result = pass.reduce_expr(&expr);
        if let LcnfExpr::Let { value, .. } = &result {
            if let LcnfLetValue::App(LcnfArg::Lit(LcnfLit::Str(name)), args) = value {
                assert_eq!(name, "band");
                assert_eq!(args[1], nat(7));
            } else {
                panic!("expected band");
            }
        }
    }
    #[test]
    pub(super) fn test_pow2_const_becomes_mul() {
        let expr = let_expr(1, app("pow", vec![var(0), nat(2)]), ret(1));
        let mut pass = StrengthReductionPass::default();
        let result = pass.reduce_expr(&expr);
        if let LcnfExpr::Let { value, .. } = &result {
            if let LcnfLetValue::App(LcnfArg::Lit(LcnfLit::Str(name)), args) = value {
                assert_eq!(name, "mul");
                assert_eq!(args[0], var(0));
                assert_eq!(args[1], var(0));
            } else {
                panic!("expected mul(x,x)");
            }
        }
        assert_eq!(pass.report().pow_reduced, 1);
    }
    #[test]
    pub(super) fn test_pow3_const_introduces_prefix() {
        let expr = let_expr(1, app("pow", vec![var(0), nat(3)]), ret(1));
        let mut pass = StrengthReductionPass::default();
        let result = pass.reduce_expr(&expr);
        if let LcnfExpr::Let {
            value: v0, body, ..
        } = &result
        {
            if let LcnfLetValue::App(LcnfArg::Lit(LcnfLit::Str(n0)), _) = v0 {
                assert_eq!(n0, "mul");
            } else {
                panic!("expected mul for square");
            }
            if let LcnfExpr::Let { value: v1, .. } = body.as_ref() {
                if let LcnfLetValue::App(LcnfArg::Lit(LcnfLit::Str(n1)), _) = v1 {
                    assert_eq!(n1, "mul");
                } else {
                    panic!("expected mul for cube");
                }
            }
        }
        assert_eq!(pass.report().pow_reduced, 1);
    }
    #[test]
    pub(super) fn test_neg_to_sub() {
        let expr = let_expr(1, app("sub", vec![nat(0), var(0)]), ret(1));
        let mut pass = StrengthReductionPass::default();
        let result = pass.reduce_expr(&expr);
        if let LcnfExpr::Let { value, .. } = &result {
            if let LcnfLetValue::App(LcnfArg::Lit(LcnfLit::Str(name)), _) = value {
                assert_eq!(name, "neg");
            } else {
                panic!("expected neg");
            }
        }
        assert_eq!(pass.report().neg_reduced, 1);
    }
    #[test]
    pub(super) fn test_add1_becomes_incr() {
        let expr = let_expr(1, app("add", vec![var(0), nat(1)]), ret(1));
        let mut pass = StrengthReductionPass::default();
        let result = pass.reduce_expr(&expr);
        if let LcnfExpr::Let { value, .. } = &result {
            if let LcnfLetValue::App(LcnfArg::Lit(LcnfLit::Str(name)), _) = value {
                assert_eq!(name, "incr");
            } else {
                panic!("expected incr");
            }
        }
        assert_eq!(pass.report().inc_reduced, 1);
    }
    #[test]
    pub(super) fn test_mul_by_non_pow2_constant() {
        let expr = let_expr(1, app("mul", vec![var(0), nat(3)]), ret(1));
        let mut pass = StrengthReductionPass::default();
        pass.reduce_expr(&expr);
        assert_eq!(pass.report().mul_reduced, 1);
    }
    #[test]
    pub(super) fn test_div_by_constant_magic() {
        let expr = let_expr(1, app("div", vec![var(0), nat(7)]), ret(1));
        let mut pass = StrengthReductionPass::default();
        let result = pass.reduce_expr(&expr);
        if let LcnfExpr::Let { value, .. } = &result {
            if let LcnfLetValue::App(LcnfArg::Lit(LcnfLit::Str(name)), _) = value {
                assert_eq!(name, "magic_div");
            } else {
                panic!("expected magic_div");
            }
        }
        assert_eq!(pass.report().div_reduced, 1);
    }
    #[test]
    pub(super) fn test_div_by_constant_disabled() {
        let expr = let_expr(1, app("div", vec![var(0), nat(7)]), ret(1));
        let mut pass = StrengthReductionPass::new(StrengthConfig {
            optimize_div: false,
            ..Default::default()
        });
        let result = pass.reduce_expr(&expr);
        if let LcnfExpr::Let { value, .. } = &result {
            if let LcnfLetValue::App(LcnfArg::Lit(LcnfLit::Str(name)), _) = value {
                assert_eq!(name, "div");
            }
        }
    }
    #[test]
    pub(super) fn test_no_reduction_for_unknown_op() {
        let expr = let_expr(1, app("custom_op", vec![var(0), nat(4)]), ret(1));
        let mut pass = StrengthReductionPass::default();
        pass.reduce_expr(&expr);
        assert_eq!(pass.report().mul_reduced, 0);
    }
    #[test]
    pub(super) fn test_run_on_empty_decls() {
        let mut decls: Vec<LcnfFunDecl> = vec![];
        StrengthReductionPass::default().run(&mut decls);
    }
    #[test]
    pub(super) fn test_decompose_mul_pow2() {
        let ops = decompose_mul(4, 3).expect("ops decomposition should succeed");
        assert_eq!(ops.len(), 1);
        assert_eq!(ops[0].0, 2);
    }
    #[test]
    pub(super) fn test_decompose_mul_3() {
        let ops = decompose_mul(3, 3);
        assert!(ops.is_some());
        assert!(ops.expect("value should be Some/Ok").len() <= 3);
    }
    #[test]
    pub(super) fn test_decompose_mul_over_budget() {
        let large = 0b10101010101010u64;
        let ops = decompose_mul(large, 2);
        assert!(ops.is_none());
    }
    #[test]
    pub(super) fn test_decompose_mul_zero() {
        let ops = decompose_mul(0, 3).expect("ops decomposition should succeed");
        assert!(ops.is_empty());
    }
    #[test]
    pub(super) fn test_detect_induction_vars_no_tail_call() {
        let decl = LcnfFunDecl {
            name: "f".into(),
            original_name: None,
            params: vec![LcnfParam {
                id: make_var(0),
                name: "n".into(),
                ty: LcnfType::Nat,
                erased: false,
                borrowed: false,
            }],
            ret_type: LcnfType::Nat,
            body: LcnfExpr::Return(var(0)),
            is_recursive: false,
            is_lifted: false,
            inline_cost: 1,
        };
        let pass = StrengthReductionPass::default();
        let ivs = pass.detect_induction_vars(&decl);
        assert!(ivs.is_empty());
    }
    #[test]
    pub(super) fn test_detect_induction_vars_with_tail_call() {
        let decl = LcnfFunDecl {
            name: "loop".into(),
            original_name: None,
            params: vec![LcnfParam {
                id: make_var(0),
                name: "i".into(),
                ty: LcnfType::Nat,
                erased: false,
                borrowed: false,
            }],
            ret_type: LcnfType::Nat,
            body: LcnfExpr::TailCall(LcnfArg::Lit(LcnfLit::Str("loop".into())), vec![var(0)]),
            is_recursive: true,
            is_lifted: false,
            inline_cost: 5,
        };
        let pass = StrengthReductionPass::default();
        let ivs = pass.detect_induction_vars(&decl);
        assert_eq!(ivs.len(), 1);
        assert_eq!(ivs[0].var, make_var(0));
    }
    #[test]
    pub(super) fn test_strength_config_display() {
        let c = StrengthConfig::default();
        let s = format!("{}", c);
        assert!(s.contains("max_shift=3"));
    }
    #[test]
    pub(super) fn test_strength_report_display() {
        let r = StrengthReport {
            mul_reduced: 2,
            div_reduced: 1,
            pow_reduced: 3,
            iv_reductions: 0,
            inc_reduced: 1,
            neg_reduced: 0,
        };
        let s = format!("{}", r);
        assert!(s.contains("mul=2"));
        assert!(s.contains("div=1"));
        assert!(s.contains("pow=3"));
    }
    #[test]
    pub(super) fn test_linear_function() {
        let lf = LinearFunction::new(make_var(0), 3, 5);
        assert_eq!(lf.eval(2), 11);
        assert!(!lf.is_identity());
        let id = LinearFunction::new(make_var(0), 1, 0);
        assert!(id.is_identity());
    }
    #[test]
    pub(super) fn test_strength_rule_display() {
        assert_eq!(
            format!("{}", StrengthReduceRule::MulByPow2(3)),
            "MulByPow2(3)"
        );
        assert_eq!(
            format!("{}", StrengthReduceRule::DivByPow2(2)),
            "DivByPow2(2)"
        );
        assert_eq!(format!("{}", StrengthReduceRule::Pow2Const), "Pow2Const");
        assert_eq!(format!("{}", StrengthReduceRule::NegToSub), "NegToSub");
        assert_eq!(
            format!("{}", StrengthReduceRule::AddSubToInc),
            "AddSubToInc"
        );
    }
    #[test]
    pub(super) fn test_optimize_strength_convenience() {
        let mut decls: Vec<LcnfFunDecl> = vec![];
        optimize_strength(&mut decls);
    }
}
#[cfg(test)]
mod OS_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = OSPassConfig::new("test_pass", OSPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = OSPassStats::new();
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
        let mut reg = OSPassRegistry::new();
        reg.register(OSPassConfig::new("pass_a", OSPassPhase::Analysis));
        reg.register(OSPassConfig::new("pass_b", OSPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = OSAnalysisCache::new(10);
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
        let mut wl = OSWorklist::new();
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
        let mut dt = OSDominatorTree::new(5);
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
        let mut liveness = OSLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(OSConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(OSConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(OSConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            OSConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(OSConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = OSDepGraph::new();
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
mod srext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_srext_phase_order() {
        assert_eq!(SRExtPassPhase::Early.order(), 0);
        assert_eq!(SRExtPassPhase::Middle.order(), 1);
        assert_eq!(SRExtPassPhase::Late.order(), 2);
        assert_eq!(SRExtPassPhase::Finalize.order(), 3);
        assert!(SRExtPassPhase::Early.is_early());
        assert!(!SRExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_srext_config_builder() {
        let c = SRExtPassConfig::new("p")
            .with_phase(SRExtPassPhase::Late)
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
    pub(super) fn test_srext_stats() {
        let mut s = SRExtPassStats::new();
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
    pub(super) fn test_srext_registry() {
        let mut r = SRExtPassRegistry::new();
        r.register(SRExtPassConfig::new("a").with_phase(SRExtPassPhase::Early));
        r.register(SRExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&SRExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_srext_cache() {
        let mut c = SRExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_srext_worklist() {
        let mut w = SRExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_srext_dom_tree() {
        let mut dt = SRExtDomTree::new(5);
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
    pub(super) fn test_srext_liveness() {
        let mut lv = SRExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_srext_const_folder() {
        let mut cf = SRExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_srext_dep_graph() {
        let mut g = SRExtDepGraph::new(4);
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
mod srx2_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_srx2_phase_order() {
        assert_eq!(SRX2PassPhase::Early.order(), 0);
        assert_eq!(SRX2PassPhase::Middle.order(), 1);
        assert_eq!(SRX2PassPhase::Late.order(), 2);
        assert_eq!(SRX2PassPhase::Finalize.order(), 3);
        assert!(SRX2PassPhase::Early.is_early());
        assert!(!SRX2PassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_srx2_config_builder() {
        let c = SRX2PassConfig::new("p")
            .with_phase(SRX2PassPhase::Late)
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
    pub(super) fn test_srx2_stats() {
        let mut s = SRX2PassStats::new();
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
    pub(super) fn test_srx2_registry() {
        let mut r = SRX2PassRegistry::new();
        r.register(SRX2PassConfig::new("a").with_phase(SRX2PassPhase::Early));
        r.register(SRX2PassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&SRX2PassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_srx2_cache() {
        let mut c = SRX2Cache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_srx2_worklist() {
        let mut w = SRX2Worklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_srx2_dom_tree() {
        let mut dt = SRX2DomTree::new(5);
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
    pub(super) fn test_srx2_liveness() {
        let mut lv = SRX2Liveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_srx2_const_folder() {
        let mut cf = SRX2ConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_srx2_dep_graph() {
        let mut g = SRX2DepGraph::new(4);
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
