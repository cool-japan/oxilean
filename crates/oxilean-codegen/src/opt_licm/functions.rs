//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::HashSet;

use super::types::{
    HoistCandidate, InvariantPattern, LICMAnalysisCache, LICMConfig, LICMConstantFoldingHelper,
    LICMDepGraph, LICMDominatorTree, LICMLivenessInfo, LICMPass, LICMPassConfig, LICMPassPhase,
    LICMPassRegistry, LICMPassStats, LICMPhase, LICMProfileData, LICMReport, LICMWorklist,
    LicmConfigExt, LicmHoistCandidate, LoopBodyComplexity, LoopInvariantExpr, LoopNode,
    LoopStructure, LoopVersion, LoopVersioningConfig, PreheaderBlock, RedundantLoadInfo,
};

/// Collect the set of variables *used* (read) inside `expr`.
pub(super) fn collect_used_vars(expr: &LcnfExpr, out: &mut HashSet<LcnfVarId>) {
    match expr {
        LcnfExpr::Let { value, body, .. } => {
            collect_used_in_let_value(value, out);
            collect_used_vars(body, out);
        }
        LcnfExpr::Case {
            scrutinee,
            alts,
            default,
            ..
        } => {
            out.insert(*scrutinee);
            for alt in alts {
                collect_used_vars(&alt.body, out);
            }
            if let Some(d) = default {
                collect_used_vars(d, out);
            }
        }
        LcnfExpr::Return(arg) => collect_used_in_arg(arg, out),
        LcnfExpr::TailCall(f, args) => {
            collect_used_in_arg(f, out);
            for a in args {
                collect_used_in_arg(a, out);
            }
        }
        LcnfExpr::Unreachable => {}
    }
}
pub(super) fn collect_used_in_arg(arg: &LcnfArg, out: &mut HashSet<LcnfVarId>) {
    if let LcnfArg::Var(v) = arg {
        out.insert(*v);
    }
}
pub(super) fn collect_used_in_let_value(val: &LcnfLetValue, out: &mut HashSet<LcnfVarId>) {
    match val {
        LcnfLetValue::App(f, args) => {
            collect_used_in_arg(f, out);
            for a in args {
                collect_used_in_arg(a, out);
            }
        }
        LcnfLetValue::Proj(_, _, v) => {
            out.insert(*v);
        }
        LcnfLetValue::Ctor(_, _, args) => {
            for a in args {
                collect_used_in_arg(a, out);
            }
        }
        LcnfLetValue::FVar(v) => {
            out.insert(*v);
        }
        LcnfLetValue::Reset(v) => {
            out.insert(*v);
        }
        LcnfLetValue::Reuse(slot, _, _, args) => {
            out.insert(*slot);
            for a in args {
                collect_used_in_arg(a, out);
            }
        }
        LcnfLetValue::Lit(_) | LcnfLetValue::Erased => {}
    }
}
/// Collect the set of variables used as **call targets** (function position in
/// `App` / `TailCall`) inside `expr`.  This is a stricter variant of
/// `collect_used_vars` that only picks up variables that are *called*, not
/// those merely passed as arguments or returned.
pub(super) fn collect_call_targets(expr: &LcnfExpr, out: &mut HashSet<LcnfVarId>) {
    match expr {
        LcnfExpr::Let { value, body, .. } => {
            if let LcnfLetValue::App(LcnfArg::Var(v), _) = value {
                out.insert(*v);
            }
            collect_call_targets(body, out);
        }
        LcnfExpr::Case { alts, default, .. } => {
            for alt in alts {
                collect_call_targets(&alt.body, out);
            }
            if let Some(d) = default {
                collect_call_targets(d, out);
            }
        }
        LcnfExpr::TailCall(LcnfArg::Var(v), _) => {
            out.insert(*v);
        }
        _ => {}
    }
}
/// Collect the set of variables *defined* (written) inside `expr`.
pub(super) fn collect_defined_vars(expr: &LcnfExpr, out: &mut HashSet<LcnfVarId>) {
    match expr {
        LcnfExpr::Let { id, body, .. } => {
            out.insert(*id);
            collect_defined_vars(body, out);
        }
        LcnfExpr::Case { alts, default, .. } => {
            for alt in alts {
                collect_defined_vars(&alt.body, out);
            }
            if let Some(d) = default {
                collect_defined_vars(d, out);
            }
        }
        LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(..) => {}
    }
}
/// Collect the free variables of a `LcnfLetValue`.
pub(super) fn free_vars_of_let_value(val: &LcnfLetValue) -> HashSet<LcnfVarId> {
    let mut out = HashSet::new();
    collect_used_in_let_value(val, &mut out);
    out
}
/// Remove let-bindings whose `id` is in `hoisted` from `expr` in-place.
/// The binding itself is deleted; the body continues unchanged.
pub(super) fn remove_hoisted_bindings(expr: &mut LcnfExpr, hoisted: &HashSet<LcnfVarId>) {
    loop {
        match expr {
            LcnfExpr::Let { id, body, .. } if hoisted.contains(id) => {
                let new_expr = std::mem::replace(body.as_mut(), LcnfExpr::Unreachable);
                *expr = new_expr;
            }
            LcnfExpr::Let { body, .. } => {
                remove_hoisted_bindings(body, hoisted);
                break;
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts.iter_mut() {
                    remove_hoisted_bindings(&mut alt.body, hoisted);
                }
                if let Some(d) = default {
                    remove_hoisted_bindings(d, hoisted);
                }
                break;
            }
            LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(..) => break,
        }
    }
}
pub(super) fn var(n: u64) -> LcnfVarId {
    LcnfVarId(n)
}
pub(super) fn lit_nat(n: u64) -> LcnfLetValue {
    LcnfLetValue::Lit(LcnfLit::Nat(n))
}
pub(super) fn arg_var(n: u64) -> LcnfArg {
    LcnfArg::Var(LcnfVarId(n))
}
pub(super) fn arg_lit(n: u64) -> LcnfArg {
    LcnfArg::Lit(LcnfLit::Nat(n))
}
pub(super) fn make_decl(name: &str, body: LcnfExpr) -> LcnfFunDecl {
    LcnfFunDecl {
        name: name.to_string(),
        params: vec![],
        ret_type: LcnfType::Nat,
        original_name: None,
        is_recursive: false,
        is_lifted: false,
        inline_cost: 0,
        body,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    /// Build a simple self-recursive let that models a counted loop.
    ///
    /// ```text
    /// let loop_fn = <invariant: lit 42>
    ///               let inner = <loop body: lit 1>
    ///               return inner
    /// ```
    /// The variable `loop_fn` is used in the body to simulate recursion.
    pub(super) fn build_loop_with_invariant() -> LcnfFunDecl {
        let inner_body = LcnfExpr::Let {
            id: var(2),
            name: format!("x{}", var(2).0),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(1)),
            body: Box::new(LcnfExpr::Let {
                id: var(3),
                name: format!("x{}", var(3).0),
                ty: LcnfType::Nat,
                value: LcnfLetValue::App(arg_var(0), vec![arg_var(2)]),
                body: Box::new(LcnfExpr::Return(arg_var(3))),
            }),
        };
        let body_with_inv = LcnfExpr::Let {
            id: var(1),
            name: format!("x{}", var(1).0),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(42)),
            body: Box::new(inner_body),
        };
        let body = LcnfExpr::Let {
            id: var(0),
            name: format!("x{}", var(0).0),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(0)),
            body: Box::new(body_with_inv),
        };
        make_decl("loop_test", body)
    }
    #[test]
    pub(super) fn test_licm_default_config() {
        let cfg = LICMConfig::default();
        assert_eq!(cfg.min_savings_threshold, 0);
        assert!(!cfg.hoist_function_calls);
    }
    #[test]
    pub(super) fn test_licm_report_default() {
        let r = LICMReport::default();
        assert_eq!(r.loops_analyzed, 0);
        assert_eq!(r.expressions_hoisted, 0);
        assert_eq!(r.estimated_savings, 0);
    }
    #[test]
    pub(super) fn test_licm_report_merge() {
        let mut r1 = LICMReport {
            loops_analyzed: 2,
            expressions_hoisted: 3,
            estimated_savings: 30,
        };
        let r2 = LICMReport {
            loops_analyzed: 1,
            expressions_hoisted: 2,
            estimated_savings: 20,
        };
        r1.merge(&r2);
        assert_eq!(r1.loops_analyzed, 3);
        assert_eq!(r1.expressions_hoisted, 5);
        assert_eq!(r1.estimated_savings, 50);
    }
    #[test]
    pub(super) fn test_find_loops_empty_decl() {
        let decl = make_decl("empty", LcnfExpr::Return(arg_lit(0)));
        let pass = LICMPass::default();
        let loops = pass.find_loops(&decl);
        assert!(loops.is_empty());
    }
    #[test]
    pub(super) fn test_find_loops_detects_self_recursive() {
        let decl = build_loop_with_invariant();
        let pass = LICMPass::default();
        let loops = pass.find_loops(&decl);
        assert!(!loops.is_empty());
        assert_eq!(loops[0].header, var(0));
    }
    #[test]
    pub(super) fn test_is_loop_invariant_literal() {
        let pass = LICMPass::default();
        let lp = LoopStructure {
            header: var(0),
            body_vars: vec![var(1), var(2)].into_iter().collect(),
            exit_vars: HashSet::new(),
            nest_depth: 0,
        };
        assert!(pass.is_loop_invariant(&LcnfLetValue::Lit(LcnfLit::Nat(7)), &lp));
    }
    #[test]
    pub(super) fn test_is_loop_invariant_var_inside_loop() {
        let pass = LICMPass::default();
        let lp = LoopStructure {
            header: var(0),
            body_vars: vec![var(1)].into_iter().collect(),
            exit_vars: HashSet::new(),
            nest_depth: 0,
        };
        assert!(!pass.is_loop_invariant(&LcnfLetValue::FVar(var(1)), &lp));
    }
    #[test]
    pub(super) fn test_is_loop_invariant_var_outside_loop() {
        let pass = LICMPass::default();
        let lp = LoopStructure {
            header: var(0),
            body_vars: vec![var(1)].into_iter().collect(),
            exit_vars: HashSet::new(),
            nest_depth: 0,
        };
        assert!(pass.is_loop_invariant(&LcnfLetValue::FVar(var(99)), &lp));
    }
    #[test]
    pub(super) fn test_is_loop_invariant_call_blocked_by_config() {
        let pass = LICMPass::default();
        let lp = LoopStructure {
            header: var(0),
            body_vars: HashSet::new(),
            exit_vars: HashSet::new(),
            nest_depth: 0,
        };
        let val = LcnfLetValue::App(LcnfArg::Lit(LcnfLit::Nat(0)), vec![]);
        assert!(!pass.is_loop_invariant(&val, &lp));
    }
    #[test]
    pub(super) fn test_is_loop_invariant_call_allowed_by_config() {
        let mut cfg = LICMConfig::default();
        cfg.hoist_function_calls = true;
        let pass = LICMPass::new(cfg);
        let lp = LoopStructure {
            header: var(0),
            body_vars: HashSet::new(),
            exit_vars: HashSet::new(),
            nest_depth: 0,
        };
        let val = LcnfLetValue::App(LcnfArg::Lit(LcnfLit::Nat(0)), vec![]);
        assert!(pass.is_loop_invariant(&val, &lp));
    }
    #[test]
    pub(super) fn test_is_loop_invariant_recursive_call_never_hoisted() {
        let mut cfg = LICMConfig::default();
        cfg.hoist_function_calls = true;
        let pass = LICMPass::new(cfg);
        let lp = LoopStructure {
            header: var(0),
            body_vars: HashSet::new(),
            exit_vars: HashSet::new(),
            nest_depth: 0,
        };
        let val = LcnfLetValue::App(LcnfArg::Var(var(0)), vec![]);
        assert!(!pass.is_loop_invariant(&val, &lp));
    }
    #[test]
    pub(super) fn test_hoist_invariants_empty_loop() {
        let mut decl = make_decl("empty", LcnfExpr::Return(arg_lit(0)));
        let lp = LoopStructure {
            header: var(0),
            body_vars: HashSet::new(),
            exit_vars: HashSet::new(),
            nest_depth: 0,
        };
        let mut pass = LICMPass::default();
        pass.hoist_invariants(&mut decl, &lp);
        assert_eq!(pass.report().expressions_hoisted, 0);
    }
    #[test]
    pub(super) fn test_run_no_loops() {
        let mut decls = vec![make_decl("simple", LcnfExpr::Return(arg_lit(1)))];
        let mut pass = LICMPass::default();
        pass.run(&mut decls);
        let r = pass.report();
        assert_eq!(r.loops_analyzed, 0);
        assert_eq!(r.expressions_hoisted, 0);
    }
    #[test]
    pub(super) fn test_run_hoists_literal_from_loop() {
        let mut decl = build_loop_with_invariant();
        let pass_detect = LICMPass::default();
        let loops = pass_detect.find_loops(&decl);
        let mut pass = LICMPass::default();
        for lp in &loops {
            pass.hoist_invariants(&mut decl, lp);
        }
        let r = pass.report();
        assert!(r.expressions_hoisted >= 1);
    }
    #[test]
    pub(super) fn test_loop_structure_nest_depth() {
        let decl = build_loop_with_invariant();
        let pass = LICMPass::default();
        let loops = pass.find_loops(&decl);
        for lp in &loops {
            assert_eq!(lp.nest_depth, 0);
        }
    }
    #[test]
    pub(super) fn test_hoist_candidate_savings_estimate() {
        let candidate = HoistCandidate {
            expr: LoopInvariantExpr {
                var: var(5),
                value: lit_nat(99),
                ty: LcnfType::Nat,
                loop_depth: 0,
            },
            target_loop_header: var(0),
            savings_estimate: 10,
        };
        assert_eq!(candidate.savings_estimate, 10);
    }
    #[test]
    pub(super) fn test_preheader_block_construction() {
        let inv = LoopInvariantExpr {
            var: var(5),
            value: lit_nat(99),
            ty: LcnfType::Nat,
            loop_depth: 0,
        };
        let pb = PreheaderBlock {
            loop_header: var(0),
            preheader_lets: vec![inv.clone()],
        };
        assert_eq!(pb.preheader_lets.len(), 1);
        assert_eq!(pb.loop_header, var(0));
    }
    #[test]
    pub(super) fn test_collect_used_vars_return() {
        let expr = LcnfExpr::Return(arg_var(7));
        let mut used = HashSet::new();
        collect_used_vars(&expr, &mut used);
        assert!(used.contains(&var(7)));
    }
    #[test]
    pub(super) fn test_collect_defined_vars_let() {
        let expr = LcnfExpr::Let {
            id: var(3),
            name: format!("x{}", var(3).0),
            ty: LcnfType::Nat,
            value: lit_nat(0),
            body: Box::new(LcnfExpr::Return(arg_lit(0))),
        };
        let mut defined = HashSet::new();
        collect_defined_vars(&expr, &mut defined);
        assert!(defined.contains(&var(3)));
    }
    #[test]
    pub(super) fn test_remove_hoisted_bindings() {
        let mut expr = LcnfExpr::Let {
            id: var(1),
            name: format!("x{}", var(1).0),
            ty: LcnfType::Nat,
            value: lit_nat(42),
            body: Box::new(LcnfExpr::Return(arg_var(1))),
        };
        let mut hoisted = HashSet::new();
        hoisted.insert(var(1));
        remove_hoisted_bindings(&mut expr, &hoisted);
        assert!(matches!(expr, LcnfExpr::Return(_)));
    }
    #[test]
    pub(super) fn test_run_multiple_decls() {
        let mut decls = vec![
            make_decl("a", LcnfExpr::Return(arg_lit(0))),
            make_decl("b", LcnfExpr::Return(arg_lit(1))),
        ];
        let mut pass = LICMPass::default();
        pass.run(&mut decls);
        assert_eq!(pass.report().loops_analyzed, 0);
    }
}
/// Try to recognize the invariant pattern of a `LcnfLetValue`, given the
/// set of variables defined inside the loop.
#[allow(dead_code)]
pub fn recognize_invariant_pattern(
    val: &LcnfLetValue,
    body_vars: &HashSet<LcnfVarId>,
) -> Option<InvariantPattern> {
    match val {
        LcnfLetValue::Lit(_) => Some(InvariantPattern::Literal),
        LcnfLetValue::Erased => Some(InvariantPattern::Erased),
        LcnfLetValue::FVar(v) => {
            if !body_vars.contains(v) {
                Some(InvariantPattern::ExternalFVar)
            } else {
                None
            }
        }
        LcnfLetValue::Proj(_, _, base) => {
            if !body_vars.contains(base) {
                Some(InvariantPattern::ExternalProj)
            } else {
                None
            }
        }
        LcnfLetValue::Ctor(_, _, args) => {
            let all_external = args.iter().all(|a| match a {
                LcnfArg::Var(v) => !body_vars.contains(v),
                LcnfArg::Lit(_) | LcnfArg::Erased | LcnfArg::Type(_) => true,
            });
            if all_external {
                Some(InvariantPattern::InvariantCtor)
            } else {
                None
            }
        }
        _ => None,
    }
}
/// Summarise a `LICMReport` as a human-readable string.
#[allow(dead_code)]
pub fn format_licm_report(report: &LICMReport) -> String {
    format!(
        "LICM: {} loops analysed, {} expressions hoisted, {} estimated savings",
        report.loops_analyzed, report.expressions_hoisted, report.estimated_savings
    )
}
/// Returns `true` if the LICM report shows any improvement was made.
#[allow(dead_code)]
pub fn licm_made_changes(report: &LICMReport) -> bool {
    report.expressions_hoisted > 0
}
#[cfg(test)]
mod licm_utility_tests {
    use super::*;
    use crate::opt_licm::*;
    #[test]
    pub(super) fn test_redundant_load_info_new() {
        let info = RedundantLoadInfo::new();
        assert_eq!(info.redundant_count, 0);
        assert!(info.available_loads.is_empty());
    }
    #[test]
    pub(super) fn test_redundant_load_register_and_lookup() {
        let mut info = RedundantLoadInfo::new();
        info.register_load(LcnfVarId(1), 0, LcnfVarId(2));
        assert_eq!(info.lookup_load(LcnfVarId(1), 0), Some(LcnfVarId(2)));
        assert_eq!(info.lookup_load(LcnfVarId(1), 1), None);
    }
    #[test]
    pub(super) fn test_redundant_load_analysis_proj() {
        let mut info = RedundantLoadInfo::new();
        let expr = LcnfExpr::Let {
            id: LcnfVarId(2),
            name: "p1".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Proj("0".to_string(), 0, LcnfVarId(1)),
            body: Box::new(LcnfExpr::Let {
                id: LcnfVarId(3),
                name: "p2".to_string(),
                ty: LcnfType::Nat,
                value: LcnfLetValue::Proj("0".to_string(), 0, LcnfVarId(1)),
                body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(3)))),
            }),
        };
        info.analyze(&expr);
        assert_eq!(info.redundant_count, 1);
    }
    #[test]
    pub(super) fn test_loop_body_complexity_empty_return() {
        let expr = LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0)));
        let c = LoopBodyComplexity::compute(&expr);
        assert_eq!(c.let_count, 0);
        assert_eq!(c.case_count, 0);
        assert_eq!(c.score(), 0);
    }
    #[test]
    pub(super) fn test_loop_body_complexity_let_chain() {
        let expr = LcnfExpr::Let {
            id: LcnfVarId(1),
            name: "a".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(1)),
            body: Box::new(LcnfExpr::Let {
                id: LcnfVarId(2),
                name: "b".to_string(),
                ty: LcnfType::Nat,
                value: LcnfLetValue::App(
                    LcnfArg::Var(LcnfVarId(1)),
                    vec![LcnfArg::Var(LcnfVarId(1))],
                ),
                body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(2)))),
            }),
        };
        let c = LoopBodyComplexity::compute(&expr);
        assert_eq!(c.let_count, 2);
        assert_eq!(c.app_count, 1);
        assert!(c.score() > 0);
    }
    #[test]
    pub(super) fn test_loop_body_complexity_nested_case() {
        let inner_case = LcnfExpr::Case {
            scrutinee: LcnfVarId(1),
            scrutinee_ty: LcnfType::Nat,
            alts: vec![],
            default: Some(Box::new(LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0))))),
        };
        let outer = LcnfExpr::Case {
            scrutinee: LcnfVarId(0),
            scrutinee_ty: LcnfType::Nat,
            alts: vec![crate::lcnf::LcnfAlt {
                ctor_name: "A".to_string(),
                ctor_tag: 0,
                params: vec![],
                body: inner_case,
            }],
            default: None,
        };
        let c = LoopBodyComplexity::compute(&outer);
        assert_eq!(c.case_count, 2);
        assert_eq!(c.max_case_depth, 2);
    }
    #[test]
    pub(super) fn test_recognize_invariant_pattern_literal() {
        let body_vars: HashSet<LcnfVarId> = HashSet::new();
        let val = LcnfLetValue::Lit(LcnfLit::Nat(7));
        assert!(matches!(
            recognize_invariant_pattern(&val, &body_vars),
            Some(InvariantPattern::Literal)
        ));
    }
    #[test]
    pub(super) fn test_recognize_invariant_pattern_erased() {
        let body_vars: HashSet<LcnfVarId> = HashSet::new();
        assert!(matches!(
            recognize_invariant_pattern(&LcnfLetValue::Erased, &body_vars),
            Some(InvariantPattern::Erased)
        ));
    }
    #[test]
    pub(super) fn test_recognize_invariant_pattern_external_fvar() {
        let body_vars: HashSet<LcnfVarId> = vec![LcnfVarId(1)].into_iter().collect();
        let val = LcnfLetValue::FVar(LcnfVarId(99));
        assert!(matches!(
            recognize_invariant_pattern(&val, &body_vars),
            Some(InvariantPattern::ExternalFVar)
        ));
        let val2 = LcnfLetValue::FVar(LcnfVarId(1));
        assert!(recognize_invariant_pattern(&val2, &body_vars).is_none());
    }
    #[test]
    pub(super) fn test_recognize_invariant_pattern_external_proj() {
        let body_vars: HashSet<LcnfVarId> = vec![LcnfVarId(1)].into_iter().collect();
        let val = LcnfLetValue::Proj("0".to_string(), 0, LcnfVarId(50));
        assert!(matches!(
            recognize_invariant_pattern(&val, &body_vars),
            Some(InvariantPattern::ExternalProj)
        ));
    }
    #[test]
    pub(super) fn test_recognize_invariant_pattern_ctor_all_external() {
        let body_vars: HashSet<LcnfVarId> = HashSet::new();
        let val = LcnfLetValue::Ctor(
            "T".to_string(),
            0,
            vec![LcnfArg::Lit(LcnfLit::Nat(1)), LcnfArg::Lit(LcnfLit::Nat(2))],
        );
        assert!(matches!(
            recognize_invariant_pattern(&val, &body_vars),
            Some(InvariantPattern::InvariantCtor)
        ));
    }
    #[test]
    pub(super) fn test_recognize_invariant_pattern_ctor_internal_arg() {
        let body_vars: HashSet<LcnfVarId> = vec![LcnfVarId(5)].into_iter().collect();
        let val = LcnfLetValue::Ctor("T".to_string(), 0, vec![LcnfArg::Var(LcnfVarId(5))]);
        assert!(recognize_invariant_pattern(&val, &body_vars).is_none());
    }
    #[test]
    pub(super) fn test_format_licm_report() {
        let r = LICMReport {
            loops_analyzed: 3,
            expressions_hoisted: 5,
            estimated_savings: 50,
        };
        let s = format_licm_report(&r);
        assert!(s.contains("3 loops"));
        assert!(s.contains("5 expressions"));
        assert!(s.contains("50 estimated"));
    }
    #[test]
    pub(super) fn test_licm_made_changes_true() {
        let r = LICMReport {
            loops_analyzed: 1,
            expressions_hoisted: 2,
            estimated_savings: 20,
        };
        assert!(licm_made_changes(&r));
    }
    #[test]
    pub(super) fn test_licm_made_changes_false() {
        let r = LICMReport::default();
        assert!(!licm_made_changes(&r));
    }
    #[test]
    pub(super) fn test_preheader_block_empty() {
        let pb = PreheaderBlock {
            loop_header: LcnfVarId(0),
            preheader_lets: vec![],
        };
        let inner = LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(42)));
        let result = materialize_preheader(&pb, inner.clone());
        assert_eq!(result, inner);
    }
    #[test]
    pub(super) fn test_loop_nest_info_multiple_depths() {
        let loops = vec![
            LoopStructure {
                header: LcnfVarId(0),
                body_vars: HashSet::new(),
                exit_vars: HashSet::new(),
                nest_depth: 0,
            },
            LoopStructure {
                header: LcnfVarId(1),
                body_vars: vec![LcnfVarId(10)].into_iter().collect(),
                exit_vars: HashSet::new(),
                nest_depth: 1,
            },
            LoopStructure {
                header: LcnfVarId(2),
                body_vars: vec![LcnfVarId(20), LcnfVarId(21)].into_iter().collect(),
                exit_vars: HashSet::new(),
                nest_depth: 2,
            },
        ];
        let info = LoopNestInfo::from_loops(loops);
        assert_eq!(info.max_depth, 2);
        assert_eq!(info.total_body_vars, 3);
    }
}
#[cfg(test)]
mod licm_phase_tests {
    use super::*;
    use crate::opt_licm::*;
    #[test]
    pub(super) fn test_loop_version_structure() {
        let v = LoopVersion {
            cond_var: LcnfVarId(10),
            fast_path_header: LcnfVarId(20),
            slow_path_header: LcnfVarId(30),
        };
        assert_eq!(v.cond_var, LcnfVarId(10));
        assert_eq!(v.fast_path_header, LcnfVarId(20));
        assert_eq!(v.slow_path_header, LcnfVarId(30));
    }
    #[test]
    pub(super) fn test_loop_versioning_config_conservative() {
        let cfg = LoopVersioningConfig::conservative();
        assert_eq!(cfg.max_versions, 2);
        assert!((cfg.min_speedup_ratio - 1.5).abs() < 0.001);
    }
    #[test]
    pub(super) fn test_licm_profile_data_new() {
        let pd = LICMProfileData::new();
        assert!(pd.loop_counts.is_empty());
        assert!(pd.binding_counts.is_empty());
    }
    #[test]
    pub(super) fn test_licm_profile_data_record_and_query() {
        let mut pd = LICMProfileData::new();
        pd.record_loop(LcnfVarId(0), 1000);
        assert_eq!(pd.loop_count(LcnfVarId(0)), 1000);
        assert_eq!(pd.loop_count(LcnfVarId(99)), 1);
    }
    #[test]
    pub(super) fn test_licm_profile_data_dynamic_savings() {
        let mut pd = LICMProfileData::new();
        pd.record_loop(LcnfVarId(0), 50);
        let candidate = HoistCandidate {
            expr: LoopInvariantExpr {
                var: LcnfVarId(1),
                value: LcnfLetValue::Erased,
                ty: LcnfType::Nat,
                loop_depth: 0,
            },
            target_loop_header: LcnfVarId(0),
            savings_estimate: 10,
        };
        assert_eq!(pd.dynamic_savings(&candidate), 500);
    }
    #[test]
    pub(super) fn test_licm_phase_display() {
        assert_eq!(LICMPhase::LICMBeforeCSE.to_string(), "LICM-before-CSE");
        assert_eq!(LICMPhase::LICMAfterCSE.to_string(), "LICM-after-CSE");
        assert_eq!(LICMPhase::LICMIterative.to_string(), "LICM-iterative");
        assert_eq!(LICMPhase::LICMOnce.to_string(), "LICM-once");
    }
    #[test]
    pub(super) fn test_licm_phase_equality() {
        assert_eq!(LICMPhase::LICMOnce, LICMPhase::LICMOnce);
        assert_ne!(LICMPhase::LICMOnce, LICMPhase::LICMIterative);
    }
    #[test]
    pub(super) fn test_loop_versioning_config_default() {
        let cfg = LoopVersioningConfig::default();
        assert_eq!(cfg.max_versions, 0);
    }
    #[test]
    pub(super) fn test_licm_profile_data_record_binding() {
        let mut pd = LICMProfileData::new();
        pd.record_binding(LcnfVarId(5), 200);
        assert_eq!(
            *pd.binding_counts
                .get(&LcnfVarId(5))
                .expect("value should be present in map"),
            200
        );
    }
    #[test]
    pub(super) fn test_redundant_load_multiple_fields() {
        let mut info = RedundantLoadInfo::new();
        info.register_load(LcnfVarId(1), 0, LcnfVarId(10));
        info.register_load(LcnfVarId(1), 1, LcnfVarId(11));
        info.register_load(LcnfVarId(2), 0, LcnfVarId(12));
        assert_eq!(info.lookup_load(LcnfVarId(1), 0), Some(LcnfVarId(10)));
        assert_eq!(info.lookup_load(LcnfVarId(1), 1), Some(LcnfVarId(11)));
        assert_eq!(info.lookup_load(LcnfVarId(2), 0), Some(LcnfVarId(12)));
        assert_eq!(info.lookup_load(LcnfVarId(2), 1), None);
    }
    #[test]
    pub(super) fn test_loop_body_complexity_case_with_apps() {
        let expr = LcnfExpr::Case {
            scrutinee: LcnfVarId(0),
            scrutinee_ty: LcnfType::Nat,
            alts: vec![crate::lcnf::LcnfAlt {
                ctor_name: "A".to_string(),
                ctor_tag: 0,
                params: vec![],
                body: LcnfExpr::Let {
                    id: LcnfVarId(1),
                    name: "r".to_string(),
                    ty: LcnfType::Nat,
                    value: LcnfLetValue::App(LcnfArg::Lit(LcnfLit::Nat(0)), vec![]),
                    body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(1)))),
                },
            }],
            default: None,
        };
        let c = LoopBodyComplexity::compute(&expr);
        assert_eq!(c.case_count, 1);
        assert_eq!(c.let_count, 1);
        assert_eq!(c.app_count, 1);
    }
    #[test]
    pub(super) fn test_topo_sort_two_independent() {
        let c1 = HoistCandidate {
            expr: LoopInvariantExpr {
                var: LcnfVarId(1),
                value: LcnfLetValue::Lit(LcnfLit::Nat(1)),
                ty: LcnfType::Nat,
                loop_depth: 0,
            },
            target_loop_header: LcnfVarId(0),
            savings_estimate: 5,
        };
        let c2 = HoistCandidate {
            expr: LoopInvariantExpr {
                var: LcnfVarId(2),
                value: LcnfLetValue::Lit(LcnfLit::Nat(2)),
                ty: LcnfType::Nat,
                loop_depth: 0,
            },
            target_loop_header: LcnfVarId(0),
            savings_estimate: 5,
        };
        let sorted = topo_sort_candidates(&[c1, c2]);
        assert_eq!(sorted.len(), 2);
        let vars: Vec<LcnfVarId> = sorted.iter().map(|c| c.expr.var).collect();
        assert!(vars.contains(&LcnfVarId(1)));
        assert!(vars.contains(&LcnfVarId(2)));
    }
    #[test]
    pub(super) fn test_topo_sort_dependent_pair() {
        let c1 = HoistCandidate {
            expr: LoopInvariantExpr {
                var: LcnfVarId(1),
                value: LcnfLetValue::Lit(LcnfLit::Nat(42)),
                ty: LcnfType::Nat,
                loop_depth: 0,
            },
            target_loop_header: LcnfVarId(0),
            savings_estimate: 5,
        };
        let c2 = HoistCandidate {
            expr: LoopInvariantExpr {
                var: LcnfVarId(2),
                value: LcnfLetValue::FVar(LcnfVarId(1)),
                ty: LcnfType::Nat,
                loop_depth: 0,
            },
            target_loop_header: LcnfVarId(0),
            savings_estimate: 5,
        };
        let sorted = topo_sort_candidates(&[c1, c2]);
        assert_eq!(sorted.len(), 2);
        let pos1 = sorted
            .iter()
            .position(|c| c.expr.var == LcnfVarId(1))
            .expect("operation should succeed");
        let pos2 = sorted
            .iter()
            .position(|c| c.expr.var == LcnfVarId(2))
            .expect("operation should succeed");
        assert!(pos1 < pos2, "producer must precede consumer");
    }
    #[test]
    pub(super) fn test_licm_pass_v2_with_heuristics() {
        let mut pass = LICMPassV2::new();
        pass.heuristics.max_hoist_cost = 0;
        let mut decl = make_decl(
            "heuristic_test",
            LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(1))),
        );
        pass.run(std::slice::from_mut(&mut decl));
        assert_eq!(pass.report().loops_analyzed, 0);
    }
    #[test]
    pub(super) fn test_collect_used_vars_tailcall() {
        let expr = LcnfExpr::TailCall(
            LcnfArg::Var(LcnfVarId(1)),
            vec![LcnfArg::Var(LcnfVarId(2)), LcnfArg::Lit(LcnfLit::Nat(0))],
        );
        let mut used = HashSet::new();
        collect_used_vars(&expr, &mut used);
        assert!(used.contains(&LcnfVarId(1)));
        assert!(used.contains(&LcnfVarId(2)));
    }
    #[test]
    pub(super) fn test_collect_defined_vars_case() {
        let expr = LcnfExpr::Case {
            scrutinee: LcnfVarId(0),
            scrutinee_ty: LcnfType::Nat,
            alts: vec![crate::lcnf::LcnfAlt {
                ctor_name: "A".to_string(),
                ctor_tag: 0,
                params: vec![],
                body: LcnfExpr::Let {
                    id: LcnfVarId(10),
                    name: "x".to_string(),
                    ty: LcnfType::Nat,
                    value: LcnfLetValue::Lit(LcnfLit::Nat(0)),
                    body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(10)))),
                },
            }],
            default: None,
        };
        let mut defined = HashSet::new();
        collect_defined_vars(&expr, &mut defined);
        assert!(defined.contains(&LcnfVarId(10)));
    }
}
/// LICM pass version
#[allow(dead_code)]
pub const LICM_PASS_VERSION: &str = "1.0.0";
/// LICM loop invariant check helper
#[allow(dead_code)]
pub fn licm_is_loop_invariant_const(value: i64) -> bool {
    let _ = value;
    true
}
/// LICM hoist decision helper
#[allow(dead_code)]
pub fn licm_should_hoist(candidate: &LicmHoistCandidate, config: &LicmConfigExt) -> bool {
    if !config.enable_hoist {
        return false;
    }
    if !candidate.is_side_effect_free && !config.hoist_stores {
        return false;
    }
    if candidate.cost > config.max_hoist_cost {
        return false;
    }
    true
}
/// LICM trip count estimation
#[allow(dead_code)]
pub fn licm_estimate_trip_count(loop_node: &LoopNode) -> Option<u64> {
    loop_node.trip_count
}
/// LICM benefit estimator
#[allow(dead_code)]
pub fn licm_hoist_benefit(candidate: &LicmHoistCandidate, trip_count: u64) -> i64 {
    let savings_per_iter = candidate.cost as i64;
    let total_savings = savings_per_iter * trip_count as i64;
    total_savings - savings_per_iter
}
/// LICM version info
#[allow(dead_code)]
pub const LICM_VERSION_INFO: &str = "licm-pass-1.0.0";
/// LICM default max hoist cost
#[allow(dead_code)]
pub const LICM_DEFAULT_MAX_HOIST_COST: i32 = 100;
/// LICM min profitable trip count
#[allow(dead_code)]
pub const LICM_MIN_TRIP_COUNT: u64 = 2;
/// LICM loop depth-1 invariant check
#[allow(dead_code)]
pub fn licm_is_trivially_invariant(var: u32, loop_defs: &std::collections::HashSet<u32>) -> bool {
    !loop_defs.contains(&var)
}
/// LICM safe to hoist check
#[allow(dead_code)]
pub fn licm_is_safe_to_hoist(inst_id: u32, has_side_effects: bool, aliases_loop_mem: bool) -> bool {
    let _ = inst_id;
    !has_side_effects && !aliases_loop_mem
}
/// LICM loop-independent variable detection
#[allow(dead_code)]
pub fn licm_all_operands_invariant(
    operands: &[u32],
    invariants: &std::collections::HashSet<u32>,
) -> bool {
    operands.iter().all(|op| invariants.contains(op))
}
/// LICM loop exit block collector
#[allow(dead_code)]
pub fn licm_collect_loop_exits(
    loop_node: &LoopNode,
    cfg_succs: &std::collections::HashMap<u32, Vec<u32>>,
) -> Vec<u32> {
    let body: std::collections::HashSet<u32> = loop_node.body_blocks.iter().cloned().collect();
    let mut exits = Vec::new();
    for &blk in &loop_node.body_blocks {
        if let Some(succs) = cfg_succs.get(&blk) {
            for &s in succs {
                if !body.contains(&s) {
                    exits.push(s);
                }
            }
        }
    }
    exits
}
/// LICM dominated-by check (simplified)
#[allow(dead_code)]
pub fn licm_dominates(
    _dom: u32,
    _target: u32,
    _dom_tree: &std::collections::HashMap<u32, u32>,
) -> bool {
    true
}
/// LICM version
#[allow(dead_code)]
pub const LICM_BACKEND_VERSION: &str = "1.0.0";
#[cfg(test)]
mod LICM_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = LICMPassConfig::new("test_pass", LICMPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = LICMPassStats::new();
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
        let mut reg = LICMPassRegistry::new();
        reg.register(LICMPassConfig::new("pass_a", LICMPassPhase::Analysis));
        reg.register(LICMPassConfig::new("pass_b", LICMPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = LICMAnalysisCache::new(10);
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
        let mut wl = LICMWorklist::new();
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
        let mut dt = LICMDominatorTree::new(5);
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
        let mut liveness = LICMLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(LICMConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(LICMConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(LICMConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            LICMConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(LICMConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = LICMDepGraph::new();
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
/// LICM code version
#[allow(dead_code)]
pub const LICM_CODE_VERSION: &str = "1.0.0";
