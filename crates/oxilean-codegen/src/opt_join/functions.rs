//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::{HashMap, HashSet};

use super::types::{
    CallSiteInfo, JoinPointConfig, JoinPointOptimizer, JoinPointStats, OJAnalysisCache,
    OJConstantFoldingHelper, OJDepGraph, OJDominatorTree, OJLivenessInfo, OJPassConfig,
    OJPassPhase, OJPassRegistry, OJPassStats, OJWorklist, OJoinConfig, OJoinDiagCollector,
    OJoinDiagMsg, OJoinEmitStats, OJoinEventLog, OJoinFeatures, OJoinIdGen, OJoinIncrKey,
    OJoinNameScope, OJoinPassTiming, OJoinProfiler, OJoinSourceBuffer, OJoinVersion, TailUse,
};

/// Analyze tail position usage of variables in an expression
pub(super) fn analyze_tail_uses(expr: &LcnfExpr, tail: bool) -> HashMap<LcnfVarId, TailUse> {
    let mut uses: HashMap<LcnfVarId, TailUse> = HashMap::new();
    match expr {
        LcnfExpr::Let {
            value, body, id, ..
        } => {
            collect_value_uses(value, &mut uses, false);
            let body_uses = analyze_tail_uses(body, tail);
            for (var, use_kind) in body_uses {
                if var != *id {
                    let current = uses.entry(var).or_insert(TailUse::Unused);
                    *current = current.merge(&use_kind);
                }
            }
        }
        LcnfExpr::Case {
            scrutinee,
            alts,
            default,
            ..
        } => {
            let current = uses.entry(*scrutinee).or_insert(TailUse::Unused);
            *current = current.merge(&TailUse::NonTail);
            for alt in alts {
                let alt_uses = analyze_tail_uses(&alt.body, tail);
                for (var, use_kind) in alt_uses {
                    let current = uses.entry(var).or_insert(TailUse::Unused);
                    *current = current.merge(&use_kind);
                }
            }
            if let Some(def) = default {
                let def_uses = analyze_tail_uses(def, tail);
                for (var, use_kind) in def_uses {
                    let current = uses.entry(var).or_insert(TailUse::Unused);
                    *current = current.merge(&use_kind);
                }
            }
        }
        LcnfExpr::Return(arg) => {
            if let LcnfArg::Var(v) = arg {
                let use_kind = if tail {
                    TailUse::TailOnly
                } else {
                    TailUse::NonTail
                };
                let current = uses.entry(*v).or_insert(TailUse::Unused);
                *current = current.merge(&use_kind);
            }
        }
        LcnfExpr::TailCall(func, args) => {
            if let LcnfArg::Var(v) = func {
                let use_kind = if tail {
                    TailUse::TailOnly
                } else {
                    TailUse::NonTail
                };
                let current = uses.entry(*v).or_insert(TailUse::Unused);
                *current = current.merge(&use_kind);
            }
            for arg in args {
                if let LcnfArg::Var(v) = arg {
                    let current = uses.entry(*v).or_insert(TailUse::Unused);
                    *current = current.merge(&TailUse::NonTail);
                }
            }
        }
        LcnfExpr::Unreachable => {}
    }
    uses
}
/// Collect variable uses from a let-value
pub(super) fn collect_value_uses(
    value: &LcnfLetValue,
    uses: &mut HashMap<LcnfVarId, TailUse>,
    _tail: bool,
) {
    let vars = extract_value_vars(value);
    for v in vars {
        let current = uses.entry(v).or_insert(TailUse::Unused);
        *current = current.merge(&TailUse::NonTail);
    }
}
/// Extract all variable references from a let-value
pub(super) fn extract_value_vars(value: &LcnfLetValue) -> Vec<LcnfVarId> {
    let mut vars = Vec::new();
    match value {
        LcnfLetValue::App(func, args) => {
            if let LcnfArg::Var(v) = func {
                vars.push(*v);
            }
            for a in args {
                if let LcnfArg::Var(v) = a {
                    vars.push(*v);
                }
            }
        }
        LcnfLetValue::Proj(_, _, v) => {
            vars.push(*v);
        }
        LcnfLetValue::Ctor(_, _, args) => {
            for a in args {
                if let LcnfArg::Var(v) = a {
                    vars.push(*v);
                }
            }
        }
        LcnfLetValue::FVar(v) => {
            vars.push(*v);
        }
        LcnfLetValue::Lit(_)
        | LcnfLetValue::Erased
        | LcnfLetValue::Reset(_)
        | LcnfLetValue::Reuse(_, _, _, _) => {}
    }
    vars
}
/// Analyze all call sites in a function body
pub(super) fn analyze_call_sites(
    expr: &LcnfExpr,
    caller: &str,
    in_tail: bool,
) -> Vec<CallSiteInfo> {
    let mut sites = Vec::new();
    match expr {
        LcnfExpr::Let { value, body, .. } => {
            if let LcnfLetValue::App(func, args) = value {
                let callee_var = if let LcnfArg::Var(v) = func {
                    Some(*v)
                } else {
                    None
                };
                sites.push(CallSiteInfo {
                    caller: caller.to_string(),
                    is_tail: false,
                    arg_count: args.len(),
                    callee_var,
                });
            }
            sites.extend(analyze_call_sites(body, caller, in_tail));
        }
        LcnfExpr::Case { alts, default, .. } => {
            for alt in alts {
                sites.extend(analyze_call_sites(&alt.body, caller, in_tail));
            }
            if let Some(def) = default {
                sites.extend(analyze_call_sites(def, caller, in_tail));
            }
        }
        LcnfExpr::TailCall(func, args) => {
            let callee_var = if let LcnfArg::Var(v) = func {
                Some(*v)
            } else {
                None
            };
            sites.push(CallSiteInfo {
                caller: caller.to_string(),
                is_tail: in_tail,
                arg_count: args.len(),
                callee_var,
            });
        }
        LcnfExpr::Return(_) | LcnfExpr::Unreachable => {}
    }
    sites
}
/// Collect all variable IDs that are used (referenced) in an expression
pub(super) fn collect_used_vars(expr: &LcnfExpr) -> HashSet<LcnfVarId> {
    let mut used = HashSet::new();
    collect_used_vars_inner(expr, &mut used);
    used
}
pub(super) fn collect_used_vars_inner(expr: &LcnfExpr, used: &mut HashSet<LcnfVarId>) {
    match expr {
        LcnfExpr::Let { value, body, .. } => {
            collect_value_used_vars(value, used);
            collect_used_vars_inner(body, used);
        }
        LcnfExpr::Case {
            scrutinee,
            alts,
            default,
            ..
        } => {
            used.insert(*scrutinee);
            for alt in alts {
                collect_used_vars_inner(&alt.body, used);
            }
            if let Some(def) = default {
                collect_used_vars_inner(def, used);
            }
        }
        LcnfExpr::Return(arg) => {
            if let LcnfArg::Var(v) = arg {
                used.insert(*v);
            }
        }
        LcnfExpr::TailCall(func, args) => {
            if let LcnfArg::Var(v) = func {
                used.insert(*v);
            }
            for a in args {
                if let LcnfArg::Var(v) = a {
                    used.insert(*v);
                }
            }
        }
        LcnfExpr::Unreachable => {}
    }
}
pub(super) fn collect_value_used_vars(value: &LcnfLetValue, used: &mut HashSet<LcnfVarId>) {
    match value {
        LcnfLetValue::App(func, args) => {
            if let LcnfArg::Var(v) = func {
                used.insert(*v);
            }
            for a in args {
                if let LcnfArg::Var(v) = a {
                    used.insert(*v);
                }
            }
        }
        LcnfLetValue::Proj(_, _, v) => {
            used.insert(*v);
        }
        LcnfLetValue::Ctor(_, _, args) => {
            for a in args {
                if let LcnfArg::Var(v) = a {
                    used.insert(*v);
                }
            }
        }
        LcnfLetValue::FVar(v) => {
            used.insert(*v);
        }
        LcnfLetValue::Lit(_)
        | LcnfLetValue::Erased
        | LcnfLetValue::Reset(_)
        | LcnfLetValue::Reuse(_, _, _, _) => {}
    }
}
/// Check whether a let-value is pure (no side effects)
pub(super) fn is_pure_value(value: &LcnfLetValue) -> bool {
    match value {
        LcnfLetValue::Lit(_)
        | LcnfLetValue::Erased
        | LcnfLetValue::FVar(_)
        | LcnfLetValue::Proj(_, _, _)
        | LcnfLetValue::Ctor(_, _, _) => true,
        LcnfLetValue::App(_, _) | LcnfLetValue::Reset(_) | LcnfLetValue::Reuse(_, _, _, _) => false,
    }
}
/// Check whether an expression references a given variable
pub(super) fn expr_uses_var(expr: &LcnfExpr, var: LcnfVarId) -> bool {
    match expr {
        LcnfExpr::Let {
            id, value, body, ..
        } => value_uses_var(value, var) || (*id != var && expr_uses_var(body, var)),
        LcnfExpr::Case {
            scrutinee,
            alts,
            default,
            ..
        } => {
            *scrutinee == var
                || alts.iter().any(|alt| expr_uses_var(&alt.body, var))
                || default.as_ref().is_some_and(|d| expr_uses_var(d, var))
        }
        LcnfExpr::Return(arg) => matches!(arg, LcnfArg::Var(v) if * v == var),
        LcnfExpr::TailCall(func, args) => {
            matches!(func, LcnfArg::Var(v) if * v == var)
                || args
                    .iter()
                    .any(|a| matches!(a, LcnfArg::Var(v) if * v == var))
        }
        LcnfExpr::Unreachable => false,
    }
}
/// Check whether a let-value references a given variable
pub(super) fn value_uses_var(value: &LcnfLetValue, var: LcnfVarId) -> bool {
    match value {
        LcnfLetValue::App(func, args) => {
            matches!(func, LcnfArg::Var(v) if * v == var)
                || args
                    .iter()
                    .any(|a| matches!(a, LcnfArg::Var(v) if * v == var))
        }
        LcnfLetValue::Proj(_, _, v) => *v == var,
        LcnfLetValue::Ctor(_, _, args) => args
            .iter()
            .any(|a| matches!(a, LcnfArg::Var(v) if * v == var)),
        LcnfLetValue::FVar(v) => *v == var,
        LcnfLetValue::Lit(_)
        | LcnfLetValue::Erased
        | LcnfLetValue::Reset(_)
        | LcnfLetValue::Reuse(_, _, _, _) => false,
    }
}
/// Count the number of LCNF instructions in an expression
pub(super) fn count_instructions(expr: &LcnfExpr) -> usize {
    match expr {
        LcnfExpr::Let { body, .. } => 1 + count_instructions(body),
        LcnfExpr::Case { alts, default, .. } => {
            let alts_size: usize = alts.iter().map(|a| count_instructions(&a.body)).sum();
            let def_size = default.as_ref().map(|d| count_instructions(d)).unwrap_or(0);
            1 + alts_size + def_size
        }
        LcnfExpr::Return(_) | LcnfExpr::TailCall(_, _) | LcnfExpr::Unreachable => 1,
    }
}
/// Compute the call graph from a set of declarations
pub(super) fn compute_call_graph(decls: &[LcnfFunDecl]) -> HashMap<String, HashSet<String>> {
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    let decl_names: HashSet<&str> = decls.iter().map(|d| d.name.as_str()).collect();
    for decl in decls {
        let mut callees = HashSet::new();
        collect_callees(&decl.body, &decl_names, &mut callees);
        graph.insert(decl.name.clone(), callees);
    }
    graph
}
/// Collect all callee function names from an expression.
///
/// Requires a var-to-name map to resolve function references; starts empty
/// and propagates name info through FVar copies encountered in let-bindings.
pub(super) fn collect_callees(
    expr: &LcnfExpr,
    known_fns: &HashSet<&str>,
    callees: &mut HashSet<String>,
) {
    let ctx: HashMap<LcnfVarId, String> = HashMap::new();
    collect_callees_ctx(expr, known_fns, callees, &ctx);
}
/// Inner helper that carries a var→name context for name propagation.
pub(super) fn collect_callees_ctx(
    expr: &LcnfExpr,
    known_fns: &HashSet<&str>,
    callees: &mut HashSet<String>,
    ctx: &HashMap<LcnfVarId, String>,
) {
    match expr {
        LcnfExpr::Let {
            id, value, body, ..
        } => {
            if let LcnfLetValue::App(LcnfArg::Var(v), _) = value {
                if let Some(name) = ctx.get(v) {
                    if known_fns.contains(name.as_str()) {
                        callees.insert(name.clone());
                    }
                }
            }
            if let LcnfLetValue::FVar(v) = value {
                if let Some(name) = ctx.get(v).cloned() {
                    let mut extended = ctx.clone();
                    extended.insert(*id, name);
                    collect_callees_ctx(body, known_fns, callees, &extended);
                    return;
                }
            }
            collect_callees_ctx(body, known_fns, callees, ctx);
        }
        LcnfExpr::Case { alts, default, .. } => {
            for alt in alts {
                collect_callees_ctx(&alt.body, known_fns, callees, ctx);
            }
            if let Some(def) = default {
                collect_callees_ctx(def, known_fns, callees, ctx);
            }
        }
        LcnfExpr::TailCall(LcnfArg::Var(v), _) => {
            if let Some(name) = ctx.get(v) {
                if known_fns.contains(name.as_str()) {
                    callees.insert(name.clone());
                }
            }
        }
        _ => {}
    }
}
/// Find self-recursive tail calls in a function
pub(super) fn find_self_recursive_tail_calls(
    expr: &LcnfExpr,
    fn_name: &str,
    var_to_name: &HashMap<LcnfVarId, String>,
) -> Vec<LcnfVarId> {
    let mut self_calls = Vec::new();
    match expr {
        LcnfExpr::Let { body, .. } => {
            self_calls.extend(find_self_recursive_tail_calls(body, fn_name, var_to_name));
        }
        LcnfExpr::Case { alts, default, .. } => {
            for alt in alts {
                self_calls.extend(find_self_recursive_tail_calls(
                    &alt.body,
                    fn_name,
                    var_to_name,
                ));
            }
            if let Some(def) = default {
                self_calls.extend(find_self_recursive_tail_calls(def, fn_name, var_to_name));
            }
        }
        LcnfExpr::TailCall(LcnfArg::Var(v), _) => {
            if let Some(name) = var_to_name.get(v) {
                if name == fn_name {
                    self_calls.push(*v);
                }
            }
        }
        _ => {}
    }
    self_calls
}
/// Determine whether a function is a join point candidate
/// (all call sites are in tail position)
pub(super) fn is_join_point_candidate(callee_id: LcnfVarId, call_sites: &[CallSiteInfo]) -> bool {
    let relevant: Vec<&CallSiteInfo> = call_sites
        .iter()
        .filter(|cs| cs.callee_var == Some(callee_id))
        .collect();
    if relevant.is_empty() {
        return false;
    }
    relevant.iter().all(|cs| cs.is_tail)
}
/// Main entry point: optimize join points in a module
pub fn optimize_join_points(module: &mut LcnfModule, config: &JoinPointConfig) {
    let mut optimizer = JoinPointOptimizer::new(config.clone());
    for decl in &mut module.fun_decls {
        optimizer.optimize_decl(decl);
    }
    if config.eliminate_dead_joins {
        eliminate_dead_functions(module);
    }
}
/// Remove function declarations that are never referenced
pub(super) fn eliminate_dead_functions(module: &mut LcnfModule) {
    if module.fun_decls.len() <= 1 {
        return;
    }
    let call_graph = compute_call_graph(&module.fun_decls);
    let mut reachable: HashSet<String> = HashSet::new();
    let mut worklist: Vec<String> = module
        .fun_decls
        .iter()
        .filter(|d| !d.is_lifted)
        .map(|d| d.name.clone())
        .collect();
    while let Some(fn_name) = worklist.pop() {
        if reachable.insert(fn_name.clone()) {
            if let Some(callees) = call_graph.get(&fn_name) {
                for callee in callees {
                    if !reachable.contains(callee) {
                        worklist.push(callee.clone());
                    }
                }
            }
        }
    }
    module.fun_decls.retain(|d| reachable.contains(&d.name));
}
/// Create a join point from a let-binding that is only used in tail position
pub(super) fn create_join_point(
    join_id: LcnfVarId,
    params: Vec<LcnfParam>,
    body: LcnfExpr,
    ret_type: LcnfType,
) -> LcnfFunDecl {
    let cost = count_instructions(&body);
    LcnfFunDecl {
        name: format!("_join_{}", join_id.0),
        original_name: None,
        params,
        ret_type,
        body,
        is_recursive: false,
        is_lifted: true,
        inline_cost: cost,
    }
}
/// Convert a tail-recursive function to use a loop with join points
pub(super) fn convert_to_loop(decl: &mut LcnfFunDecl) -> bool {
    if !decl.is_recursive {
        return false;
    }
    let var_to_name: HashMap<LcnfVarId, String> = HashMap::new();
    let self_calls = find_self_recursive_tail_calls(&decl.body, &decl.name, &var_to_name);
    !self_calls.is_empty()
}
#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn make_var(n: u64) -> LcnfVarId {
        LcnfVarId(n)
    }
    pub(super) fn make_param(n: u64, name: &str) -> LcnfParam {
        LcnfParam {
            id: LcnfVarId(n),
            name: name.to_string(),
            ty: LcnfType::Nat,
            erased: false,
            borrowed: false,
        }
    }
    pub(super) fn make_simple_let(id: u64, value: LcnfLetValue, body: LcnfExpr) -> LcnfExpr {
        LcnfExpr::Let {
            id: LcnfVarId(id),
            name: format!("x{}", id),
            ty: LcnfType::Nat,
            value,
            body: Box::new(body),
        }
    }
    pub(super) fn make_simple_decl(name: &str, body: LcnfExpr) -> LcnfFunDecl {
        LcnfFunDecl {
            name: name.to_string(),
            original_name: None,
            params: vec![make_param(0, "arg0")],
            ret_type: LcnfType::Nat,
            body,
            is_recursive: false,
            is_lifted: false,
            inline_cost: 1,
        }
    }
    #[test]
    pub(super) fn test_config_default() {
        let config = JoinPointConfig::default();
        assert_eq!(config.max_join_size, 10);
        assert!(config.inline_small_joins);
        assert!(config.detect_tail_calls);
        assert!(config.enable_contification);
    }
    #[test]
    pub(super) fn test_stats_default() {
        let stats = JoinPointStats::default();
        assert_eq!(stats.total_changes(), 0);
    }
    #[test]
    pub(super) fn test_tail_use_merge() {
        assert_eq!(TailUse::Unused.merge(&TailUse::TailOnly), TailUse::TailOnly);
        assert_eq!(
            TailUse::TailOnly.merge(&TailUse::TailOnly),
            TailUse::TailOnly
        );
        assert_eq!(TailUse::TailOnly.merge(&TailUse::NonTail), TailUse::Mixed);
        assert_eq!(TailUse::NonTail.merge(&TailUse::NonTail), TailUse::NonTail);
    }
    #[test]
    pub(super) fn test_is_pure_value() {
        assert!(is_pure_value(&LcnfLetValue::Lit(LcnfLit::Nat(42))));
        assert!(is_pure_value(&LcnfLetValue::Erased));
        assert!(is_pure_value(&LcnfLetValue::FVar(make_var(0))));
        assert!(is_pure_value(&LcnfLetValue::Proj(
            "foo".into(),
            0,
            make_var(0)
        )));
        assert!(!is_pure_value(&LcnfLetValue::App(
            LcnfArg::Var(make_var(0)),
            vec![]
        )));
    }
    #[test]
    pub(super) fn test_count_instructions() {
        let ret = LcnfExpr::Return(LcnfArg::Var(make_var(0)));
        assert_eq!(count_instructions(&ret), 1);
        let let_expr = make_simple_let(
            1,
            LcnfLetValue::Lit(LcnfLit::Nat(42)),
            LcnfExpr::Return(LcnfArg::Var(make_var(1))),
        );
        assert_eq!(count_instructions(&let_expr), 2);
    }
    #[test]
    pub(super) fn test_collect_used_vars() {
        let expr = make_simple_let(
            1,
            LcnfLetValue::FVar(make_var(0)),
            LcnfExpr::Return(LcnfArg::Var(make_var(1))),
        );
        let used = collect_used_vars(&expr);
        assert!(used.contains(&make_var(0)));
        assert!(used.contains(&make_var(1)));
    }
    #[test]
    pub(super) fn test_expr_uses_var() {
        let expr = LcnfExpr::Return(LcnfArg::Var(make_var(5)));
        assert!(expr_uses_var(&expr, make_var(5)));
        assert!(!expr_uses_var(&expr, make_var(6)));
    }
    #[test]
    pub(super) fn test_value_uses_var() {
        let val = LcnfLetValue::App(LcnfArg::Var(make_var(1)), vec![LcnfArg::Var(make_var(2))]);
        assert!(value_uses_var(&val, make_var(1)));
        assert!(value_uses_var(&val, make_var(2)));
        assert!(!value_uses_var(&val, make_var(3)));
    }
    #[test]
    pub(super) fn test_extract_value_vars() {
        let val = LcnfLetValue::App(
            LcnfArg::Var(make_var(1)),
            vec![LcnfArg::Var(make_var(2)), LcnfArg::Lit(LcnfLit::Nat(0))],
        );
        let vars = extract_value_vars(&val);
        assert_eq!(vars.len(), 2);
        assert!(vars.contains(&make_var(1)));
        assert!(vars.contains(&make_var(2)));
    }
    #[test]
    pub(super) fn test_detect_tail_calls() {
        let mut expr = make_simple_let(
            1,
            LcnfLetValue::App(LcnfArg::Var(make_var(10)), vec![LcnfArg::Var(make_var(0))]),
            LcnfExpr::Return(LcnfArg::Var(make_var(1))),
        );
        let mut optimizer = JoinPointOptimizer::new(JoinPointConfig::default());
        optimizer.detect_tail_calls_in_expr(&mut expr, "test");
        assert!(matches!(expr, LcnfExpr::TailCall(_, _)));
        assert_eq!(optimizer.stats.tail_calls_detected, 1);
    }
    #[test]
    pub(super) fn test_dead_join_elimination() {
        let mut expr = make_simple_let(
            1,
            LcnfLetValue::Lit(LcnfLit::Nat(42)),
            make_simple_let(
                2,
                LcnfLetValue::Lit(LcnfLit::Nat(100)),
                LcnfExpr::Return(LcnfArg::Var(make_var(2))),
            ),
        );
        let mut optimizer = JoinPointOptimizer::new(JoinPointConfig::default());
        optimizer.eliminate_dead_joins(&mut expr);
        assert!(matches!(expr, LcnfExpr::Let { id, .. } if id == make_var(2)));
    }
    #[test]
    pub(super) fn test_optimize_join_points_full() {
        let body = make_simple_let(
            1,
            LcnfLetValue::Lit(LcnfLit::Nat(42)),
            LcnfExpr::Return(LcnfArg::Var(make_var(1))),
        );
        let decl = make_simple_decl("test_fn", body);
        let mut module = LcnfModule {
            fun_decls: vec![decl],
            extern_decls: vec![],
            name: "test_mod".to_string(),
            metadata: LcnfModuleMetadata::default(),
        };
        let config = JoinPointConfig::default();
        optimize_join_points(&mut module, &config);
        assert_eq!(module.fun_decls.len(), 1);
    }
    #[test]
    pub(super) fn test_value_size() {
        let optimizer = JoinPointOptimizer::new(JoinPointConfig::default());
        assert_eq!(optimizer.value_size(&LcnfLetValue::Lit(LcnfLit::Nat(0))), 1);
        assert_eq!(optimizer.value_size(&LcnfLetValue::Erased), 1);
        assert_eq!(
            optimizer.value_size(&LcnfLetValue::App(
                LcnfArg::Var(make_var(0)),
                vec![LcnfArg::Var(make_var(1)), LcnfArg::Var(make_var(2))]
            )),
            3
        );
    }
    #[test]
    pub(super) fn test_analyze_tail_uses_return() {
        let expr = LcnfExpr::Return(LcnfArg::Var(make_var(5)));
        let uses = analyze_tail_uses(&expr, true);
        assert_eq!(uses.get(&make_var(5)), Some(&TailUse::TailOnly));
    }
    #[test]
    pub(super) fn test_analyze_tail_uses_non_tail() {
        let expr = LcnfExpr::Return(LcnfArg::Var(make_var(5)));
        let uses = analyze_tail_uses(&expr, false);
        assert_eq!(uses.get(&make_var(5)), Some(&TailUse::NonTail));
    }
    #[test]
    pub(super) fn test_call_site_analysis() {
        let body = make_simple_let(
            1,
            LcnfLetValue::App(LcnfArg::Var(make_var(10)), vec![LcnfArg::Var(make_var(0))]),
            LcnfExpr::Return(LcnfArg::Var(make_var(1))),
        );
        let sites = analyze_call_sites(&body, "test_fn", true);
        assert_eq!(sites.len(), 1);
        assert!(!sites[0].is_tail);
        assert_eq!(sites[0].arg_count, 1);
    }
    #[test]
    pub(super) fn test_compute_call_graph() {
        let body = LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0)));
        let decl1 = make_simple_decl("foo", body.clone());
        let decl2 = make_simple_decl("bar", body);
        let graph = compute_call_graph(&[decl1, decl2]);
        assert!(graph.contains_key("foo"));
        assert!(graph.contains_key("bar"));
    }
    #[test]
    pub(super) fn test_create_join_point() {
        let body = LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0)));
        let jp = create_join_point(make_var(100), vec![make_param(1, "p")], body, LcnfType::Nat);
        assert_eq!(jp.name, "_join_100");
        assert!(jp.is_lifted);
    }
    #[test]
    pub(super) fn test_convert_to_loop_non_recursive() {
        let body = LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0)));
        let mut decl = make_simple_decl("test", body);
        decl.is_recursive = false;
        assert!(!convert_to_loop(&mut decl));
    }
    #[test]
    pub(super) fn test_join_point_candidate() {
        let sites = vec![
            CallSiteInfo {
                caller: "f".to_string(),
                is_tail: true,
                arg_count: 1,
                callee_var: Some(make_var(5)),
            },
            CallSiteInfo {
                caller: "g".to_string(),
                is_tail: true,
                arg_count: 1,
                callee_var: Some(make_var(5)),
            },
        ];
        assert!(is_join_point_candidate(make_var(5), &sites));
        let mixed_sites = vec![
            CallSiteInfo {
                caller: "f".to_string(),
                is_tail: true,
                arg_count: 1,
                callee_var: Some(make_var(5)),
            },
            CallSiteInfo {
                caller: "g".to_string(),
                is_tail: false,
                arg_count: 1,
                callee_var: Some(make_var(5)),
            },
        ];
        assert!(!is_join_point_candidate(make_var(5), &mixed_sites));
    }
    #[test]
    pub(super) fn test_optimizer_fresh_id() {
        let mut opt = JoinPointOptimizer::new(JoinPointConfig::default());
        let id1 = opt.fresh_id();
        let id2 = opt.fresh_id();
        assert_ne!(id1, id2);
    }
    #[test]
    pub(super) fn test_case_tail_call_detection() {
        let mut expr = LcnfExpr::Case {
            scrutinee: make_var(0),
            scrutinee_ty: LcnfType::Nat,
            alts: vec![
                LcnfAlt {
                    ctor_name: "True".to_string(),
                    ctor_tag: 0,
                    params: vec![],
                    body: make_simple_let(
                        5,
                        LcnfLetValue::App(
                            LcnfArg::Var(make_var(10)),
                            vec![LcnfArg::Var(make_var(1))],
                        ),
                        LcnfExpr::Return(LcnfArg::Var(make_var(5))),
                    ),
                },
                LcnfAlt {
                    ctor_name: "False".to_string(),
                    ctor_tag: 1,
                    params: vec![],
                    body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0))),
                },
            ],
            default: None,
        };
        let mut optimizer = JoinPointOptimizer::new(JoinPointConfig::default());
        optimizer.detect_tail_calls_in_expr(&mut expr, "test");
        assert_eq!(optimizer.stats.tail_calls_detected, 1);
        if let LcnfExpr::Case { alts, .. } = &expr {
            assert!(matches!(alts[0].body, LcnfExpr::TailCall(_, _)));
        }
    }
    #[test]
    pub(super) fn test_nested_dead_elimination() {
        let mut expr = make_simple_let(
            1,
            LcnfLetValue::Lit(LcnfLit::Nat(1)),
            make_simple_let(
                2,
                LcnfLetValue::Lit(LcnfLit::Nat(2)),
                make_simple_let(
                    3,
                    LcnfLetValue::Lit(LcnfLit::Nat(3)),
                    LcnfExpr::Return(LcnfArg::Var(make_var(3))),
                ),
            ),
        );
        let mut optimizer = JoinPointOptimizer::new(JoinPointConfig::default());
        optimizer.eliminate_dead_joins(&mut expr);
        assert!(matches!(& expr, LcnfExpr::Let { id, .. } if * id == make_var(3)));
    }
    #[test]
    pub(super) fn test_unreachable_count() {
        let expr = LcnfExpr::Unreachable;
        assert_eq!(count_instructions(&expr), 1);
    }
    #[test]
    pub(super) fn test_tail_call_count() {
        let expr = LcnfExpr::TailCall(LcnfArg::Var(make_var(0)), vec![LcnfArg::Var(make_var(1))]);
        assert_eq!(count_instructions(&expr), 1);
    }
    #[test]
    pub(super) fn test_find_small_joins() {
        let expr = make_simple_let(
            1,
            LcnfLetValue::Lit(LcnfLit::Nat(42)),
            make_simple_let(
                2,
                LcnfLetValue::FVar(make_var(1)),
                LcnfExpr::Return(LcnfArg::Var(make_var(2))),
            ),
        );
        let optimizer = JoinPointOptimizer::new(JoinPointConfig::default());
        let joins = optimizer.find_small_joins(&expr);
        assert!(joins.contains_key(&make_var(1)));
        assert!(joins.contains_key(&make_var(2)));
    }
    #[test]
    pub(super) fn test_inline_small_joins() {
        let mut expr = make_simple_let(
            1,
            LcnfLetValue::Lit(LcnfLit::Nat(42)),
            make_simple_let(
                2,
                LcnfLetValue::FVar(make_var(1)),
                LcnfExpr::Return(LcnfArg::Var(make_var(2))),
            ),
        );
        let mut optimizer = JoinPointOptimizer::new(JoinPointConfig::default());
        optimizer.inline_small_joins(&mut expr);
    }
    #[test]
    pub(super) fn test_case_instruction_count() {
        let expr = LcnfExpr::Case {
            scrutinee: make_var(0),
            scrutinee_ty: LcnfType::Nat,
            alts: vec![
                LcnfAlt {
                    ctor_name: "A".to_string(),
                    ctor_tag: 0,
                    params: vec![],
                    body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(1))),
                },
                LcnfAlt {
                    ctor_name: "B".to_string(),
                    ctor_tag: 1,
                    params: vec![],
                    body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(2))),
                },
            ],
            default: None,
        };
        assert_eq!(count_instructions(&expr), 3);
    }
    #[test]
    pub(super) fn test_full_pipeline_with_case() {
        let body = LcnfExpr::Case {
            scrutinee: make_var(0),
            scrutinee_ty: LcnfType::Nat,
            alts: vec![LcnfAlt {
                ctor_name: "Zero".to_string(),
                ctor_tag: 0,
                params: vec![],
                body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0))),
            }],
            default: Some(Box::new(LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(1))))),
        };
        let decl = make_simple_decl("test_case", body);
        let mut module = LcnfModule {
            fun_decls: vec![decl],
            extern_decls: vec![],
            name: "test_mod".to_string(),
            metadata: LcnfModuleMetadata::default(),
        };
        let config = JoinPointConfig::default();
        optimize_join_points(&mut module, &config);
        assert_eq!(module.fun_decls.len(), 1);
    }
    #[test]
    pub(super) fn test_find_self_recursive_tail_calls() {
        let mut var_map = HashMap::new();
        var_map.insert(make_var(10), "my_fn".to_string());
        let expr = LcnfExpr::TailCall(LcnfArg::Var(make_var(10)), vec![LcnfArg::Var(make_var(0))]);
        let calls = find_self_recursive_tail_calls(&expr, "my_fn", &var_map);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0], make_var(10));
    }
    #[test]
    pub(super) fn test_collect_callees_empty() {
        let expr = LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0)));
        let known: HashSet<&str> = HashSet::new();
        let mut callees = HashSet::new();
        collect_callees(&expr, &known, &mut callees);
        assert!(callees.is_empty());
    }
    #[test]
    pub(super) fn test_multiple_iterations() {
        let body = make_simple_let(
            1,
            LcnfLetValue::Lit(LcnfLit::Nat(1)),
            make_simple_let(
                2,
                LcnfLetValue::Lit(LcnfLit::Nat(2)),
                make_simple_let(
                    3,
                    LcnfLetValue::App(LcnfArg::Var(make_var(10)), vec![LcnfArg::Var(make_var(2))]),
                    LcnfExpr::Return(LcnfArg::Var(make_var(3))),
                ),
            ),
        );
        let mut decl = make_simple_decl("multi_iter", body);
        let mut optimizer = JoinPointOptimizer::new(JoinPointConfig {
            max_iterations: 10,
            ..JoinPointConfig::default()
        });
        optimizer.optimize_decl(&mut decl);
        assert!(optimizer.stats.iterations > 0);
    }
}
#[cfg(test)]
mod tests_ojoin_extra {
    use super::*;
    #[test]
    pub(super) fn test_ojoin_config() {
        let mut cfg = OJoinConfig::new();
        cfg.set("mode", "release");
        cfg.set("verbose", "true");
        assert_eq!(cfg.get("mode"), Some("release"));
        assert!(cfg.get_bool("verbose"));
        assert!(cfg.get_int("mode").is_none());
        assert_eq!(cfg.len(), 2);
    }
    #[test]
    pub(super) fn test_ojoin_source_buffer() {
        let mut buf = OJoinSourceBuffer::new();
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
    pub(super) fn test_ojoin_name_scope() {
        let mut scope = OJoinNameScope::new();
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
    pub(super) fn test_ojoin_diag_collector() {
        let mut col = OJoinDiagCollector::new();
        col.emit(OJoinDiagMsg::warning("pass_a", "slow"));
        col.emit(OJoinDiagMsg::error("pass_b", "fatal"));
        assert!(col.has_errors());
        assert_eq!(col.errors().len(), 1);
        assert_eq!(col.warnings().len(), 1);
        col.clear();
        assert!(col.is_empty());
    }
    #[test]
    pub(super) fn test_ojoin_id_gen() {
        let mut gen = OJoinIdGen::new();
        assert_eq!(gen.next_id(), 0);
        assert_eq!(gen.next_id(), 1);
        gen.skip(10);
        assert_eq!(gen.next_id(), 12);
        gen.reset();
        assert_eq!(gen.peek_next(), 0);
    }
    #[test]
    pub(super) fn test_ojoin_incr_key() {
        let k1 = OJoinIncrKey::new(100, 200);
        let k2 = OJoinIncrKey::new(100, 200);
        let k3 = OJoinIncrKey::new(999, 200);
        assert!(k1.matches(&k2));
        assert!(!k1.matches(&k3));
    }
    #[test]
    pub(super) fn test_ojoin_profiler() {
        let mut p = OJoinProfiler::new();
        p.record(OJoinPassTiming::new("pass_a", 1000, 50, 200, 100));
        p.record(OJoinPassTiming::new("pass_b", 500, 30, 100, 200));
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
    pub(super) fn test_ojoin_event_log() {
        let mut log = OJoinEventLog::new(3);
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
    pub(super) fn test_ojoin_version() {
        let v = OJoinVersion::new(1, 2, 3).with_pre("alpha");
        assert!(!v.is_stable());
        assert_eq!(format!("{}", v), "1.2.3-alpha");
        let stable = OJoinVersion::new(2, 0, 0);
        assert!(stable.is_stable());
        assert!(stable.is_compatible_with(&OJoinVersion::new(2, 0, 0)));
        assert!(!stable.is_compatible_with(&OJoinVersion::new(3, 0, 0)));
    }
    #[test]
    pub(super) fn test_ojoin_features() {
        let mut f = OJoinFeatures::new();
        f.enable("sse2");
        f.enable("avx2");
        assert!(f.is_enabled("sse2"));
        assert!(!f.is_enabled("avx512"));
        f.disable("avx2");
        assert!(!f.is_enabled("avx2"));
        let mut g = OJoinFeatures::new();
        g.enable("sse2");
        g.enable("neon");
        let union = f.union(&g);
        assert!(union.is_enabled("sse2") && union.is_enabled("neon"));
        let inter = f.intersection(&g);
        assert!(inter.is_enabled("sse2"));
    }
    #[test]
    pub(super) fn test_ojoin_emit_stats() {
        let mut s = OJoinEmitStats::new();
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
mod OJ_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = OJPassConfig::new("test_pass", OJPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = OJPassStats::new();
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
        let mut reg = OJPassRegistry::new();
        reg.register(OJPassConfig::new("pass_a", OJPassPhase::Analysis));
        reg.register(OJPassConfig::new("pass_b", OJPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = OJAnalysisCache::new(10);
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
        let mut wl = OJWorklist::new();
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
        let mut dt = OJDominatorTree::new(5);
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
        let mut liveness = OJLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(OJConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(OJConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(OJConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            OJConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(OJConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = OJDepGraph::new();
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
