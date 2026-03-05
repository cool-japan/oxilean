//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Environment, Expr, Level, Name};
use std::collections::{BTreeMap, HashMap, HashSet};

use super::types::{
    ArgDecrease, MutualRecGroup, PreDefAnalyzer, PreDefConfig, ProofObligation, RecCall,
    RecursionDetector, RecursionKind, StructuralRecParam, SubtermRelation, TerminationChecker,
    TerminationError, TerminationResult, WellFoundedOrder,
};

/// Find all recursive calls in a body expression.
///
/// Walks the expression tree and collects all applications whose
/// head is one of the given function names.
pub fn find_recursive_calls(body: &Expr, fn_names: &HashSet<Name>) -> Vec<RecCall> {
    let mut calls = Vec::new();
    find_recursive_calls_inner(body, fn_names, &mut calls, false, None, 0);
    calls
}
/// Inner recursive helper for finding recursive calls.
fn find_recursive_calls_inner(
    expr: &Expr,
    fn_names: &HashSet<Name>,
    out: &mut Vec<RecCall>,
    in_match: bool,
    guard: Option<&Name>,
    depth: u32,
) {
    match expr {
        Expr::App(func, arg) => {
            if let Some((callee, args)) = collect_app_spine(expr, fn_names) {
                let mut call = RecCall::new(callee, args).with_nesting_depth(depth);
                if in_match {
                    if let Some(g) = guard {
                        call = call.with_guard(g.clone());
                    }
                }
                out.push(call);
            }
            find_recursive_calls_inner(func, fn_names, out, in_match, guard, depth);
            find_recursive_calls_inner(arg, fn_names, out, in_match, guard, depth);
        }
        Expr::Lam(_, _, ty, body) => {
            find_recursive_calls_inner(ty, fn_names, out, in_match, guard, depth);
            find_recursive_calls_inner(body, fn_names, out, in_match, guard, depth + 1);
        }
        Expr::Pi(_, _, ty, body) => {
            find_recursive_calls_inner(ty, fn_names, out, in_match, guard, depth);
            find_recursive_calls_inner(body, fn_names, out, in_match, guard, depth);
        }
        Expr::Let(_, ty, val, body) => {
            find_recursive_calls_inner(ty, fn_names, out, in_match, guard, depth);
            find_recursive_calls_inner(val, fn_names, out, in_match, guard, depth);
            find_recursive_calls_inner(body, fn_names, out, in_match, guard, depth);
        }
        Expr::Proj(_, _, base) => {
            find_recursive_calls_inner(base, fn_names, out, in_match, guard, depth);
        }
        _ => {}
    }
}
/// Collect the function and arguments from a spine of applications.
///
/// Given `App(App(App(f, a1), a2), a3)`, returns `(f_name, [a1, a2, a3])`
/// if `f` is a `Const` whose name is in `fn_names`.
fn collect_app_spine(expr: &Expr, fn_names: &HashSet<Name>) -> Option<(Name, Vec<Expr>)> {
    let mut args = Vec::new();
    let mut current = expr;
    loop {
        match current {
            Expr::App(func, arg) => {
                args.push(arg.as_ref().clone());
                current = func.as_ref();
            }
            Expr::Const(name, _) if fn_names.contains(name) => {
                args.reverse();
                return Some((name.clone(), args));
            }
            _ => return None,
        }
    }
}
/// Check if a call argument is structurally smaller than the parameter.
///
/// An argument is structurally smaller if it is:
/// - A projection of the parameter (field access)
/// - A constructor argument of the parameter (pattern match variable)
/// - A bound variable introduced by a pattern match on the parameter
pub fn check_structural_decrease(param: &Name, call_arg: &Expr) -> bool {
    match call_arg {
        Expr::Proj(_, _, base) => is_param_ref(param, base),
        Expr::BVar(_) => true,
        Expr::App(func, arg) => {
            is_constructor_app(func) && check_structural_decrease(param, arg)
                || is_param_ref(param, call_arg)
        }
        Expr::FVar(_) => false,
        Expr::Const(_, _) | Expr::Lit(_) | Expr::Sort(_) => false,
        Expr::Lam(_, _, _, body) => check_structural_decrease(param, body),
        Expr::Pi(_, _, _, _) => false,
        Expr::Let(_, _, val, body) => {
            check_structural_decrease(param, val) || check_structural_decrease(param, body)
        }
    }
}
/// Check if an expression refers to the given parameter.
fn is_param_ref(param: &Name, expr: &Expr) -> bool {
    match expr {
        Expr::FVar(_) => false,
        Expr::Const(name, _) => name == param,
        _ => false,
    }
}
/// Check if a function expression is a constructor application.
fn is_constructor_app(expr: &Expr) -> bool {
    match expr {
        Expr::Const(name, _) => {
            let name_str = format!("{}", name);
            name_str.contains(".mk")
                || name_str.contains(".succ")
                || name_str.contains(".cons")
                || name_str.contains(".zero")
                || name_str.contains(".nil")
                || name_str.contains(".none")
                || name_str.contains(".some")
                || name_str.contains(".inl")
                || name_str.contains(".inr")
        }
        Expr::App(func, _) => is_constructor_app(func),
        _ => false,
    }
}
/// Classify how an argument changes relative to the caller's parameters.
pub fn classify_argument_decrease(
    arg: &Expr,
    caller_params: &[(Name, Expr)],
    _env: &Environment,
) -> ArgDecrease {
    for (param_name, _) in caller_params {
        if is_param_ref(param_name, arg) {
            return ArgDecrease::Equal;
        }
    }
    match arg {
        Expr::Proj(_, _, base) => {
            for (param_name, _) in caller_params {
                if is_param_ref(param_name, base) {
                    return ArgDecrease::Decreasing;
                }
            }
            ArgDecrease::Unknown
        }
        Expr::BVar(_) => ArgDecrease::Decreasing,
        Expr::App(func, _) => {
            let _ = is_constructor_app(func);
            ArgDecrease::Unknown
        }
        _ => ArgDecrease::Unknown,
    }
}
/// Build a Lean-style `fix` combinator term for a recursive definition.
///
/// Transforms:
/// ```text
/// def f (params) : ret_type := body
/// ```
/// into:
/// ```text
/// @Nat.rec ret_type base_case (fun n ih => step_case) param
/// ```
/// for structural recursion on `Nat`, or into:
/// ```text
/// @WellFounded.fix A C rel_wf (fun x ih => body) param
/// ```
/// for well-founded recursion.
pub fn build_fix_term(
    name: &Name,
    params: &[(Name, Expr)],
    body: &Expr,
    rec_param_idx: usize,
    ret_type: &Expr,
) -> Expr {
    let n_params = params.len();
    if rec_param_idx >= n_params {
        return body.clone();
    }
    let (_rec_param_name, rec_param_ty) = &params[rec_param_idx];
    let recursor_name = match get_recursor_name(rec_param_ty) {
        Some(name) => name,
        None => {
            return build_generic_fix(name, params, body, rec_param_idx, ret_type);
        }
    };
    let rec_const = Expr::Const(recursor_name, vec![]);
    let motive = Expr::Lam(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(rec_param_ty.clone()),
        Box::new(ret_type.clone()),
    );
    let app = Expr::App(Box::new(rec_const), Box::new(motive));
    let step = Expr::Lam(
        BinderInfo::Default,
        Name::str("n"),
        Box::new(rec_param_ty.clone()),
        Box::new(Expr::Lam(
            BinderInfo::Default,
            Name::str("ih"),
            Box::new(ret_type.clone()),
            Box::new(body.clone()),
        )),
    );
    let app_with_step = Expr::App(Box::new(app), Box::new(step));
    let rec_param_expr = Expr::BVar((n_params - 1 - rec_param_idx) as u32);
    Expr::App(Box::new(app_with_step), Box::new(rec_param_expr))
}
/// Build a generic fix combinator when no specific recursor is available.
fn build_generic_fix(
    name: &Name,
    params: &[(Name, Expr)],
    body: &Expr,
    rec_param_idx: usize,
    ret_type: &Expr,
) -> Expr {
    let fix_const = Expr::Const(Name::str("fix"), vec![]);
    let mut functional = body.clone();
    for (i, (param_name, param_ty)) in params.iter().enumerate().rev() {
        functional = Expr::Lam(
            BinderInfo::Default,
            param_name.clone(),
            Box::new(param_ty.clone()),
            Box::new(functional),
        );
        let _ = i;
    }
    let self_type = build_function_type(params, ret_type);
    functional = Expr::Lam(
        BinderInfo::Default,
        name.clone(),
        Box::new(self_type),
        Box::new(functional),
    );
    let _ = rec_param_idx;
    Expr::App(Box::new(fix_const), Box::new(functional))
}
/// Build a function type from parameters and return type.
///
/// `(a : A) -> (b : B) -> ... -> R`
fn build_function_type(params: &[(Name, Expr)], ret_type: &Expr) -> Expr {
    let mut result = ret_type.clone();
    for (param_name, param_ty) in params.iter().rev() {
        result = Expr::Pi(
            BinderInfo::Default,
            param_name.clone(),
            Box::new(param_ty.clone()),
            Box::new(result),
        );
    }
    result
}
/// Get the recursor name for an inductive type.
fn get_recursor_name(ty: &Expr) -> Option<Name> {
    match ty {
        Expr::Const(name, _) => Some(name.clone().append_str("rec")),
        Expr::App(func, _) => get_recursor_name(func),
        _ => None,
    }
}
/// Generate tactic-like termination proof obligations.
///
/// When a recursive definition cannot be automatically proven to terminate,
/// the user can supply a `termination_by` clause. This function generates
/// the proof obligations that the user must discharge.
///
/// Returns a list of proof obligations, each consisting of:
/// - The goal to prove (e.g., `measure arg' < measure arg`)
/// - The available hypotheses
/// - Source location information
pub fn tactic_like_termination_proof(
    name: &Name,
    params: &[(Name, Expr)],
    body: &Expr,
    measure: &Expr,
    relation: &Expr,
    domain_type: &Expr,
) -> Vec<ProofObligation> {
    let fn_names: HashSet<Name> = [name.clone()].into_iter().collect();
    let calls = find_recursive_calls(body, &fn_names);
    let mut obligations = Vec::new();
    for (i, call) in calls.iter().enumerate() {
        if call.callee != *name {
            continue;
        }
        let call_measure = if call.args.len() == 1 {
            Expr::App(Box::new(measure.clone()), Box::new(call.args[0].clone()))
        } else {
            let mut m = measure.clone();
            for arg in &call.args {
                m = Expr::App(Box::new(m), Box::new(arg.clone()));
            }
            m
        };
        let caller_measure = if params.len() == 1 {
            Expr::App(Box::new(measure.clone()), Box::new(Expr::BVar(0)))
        } else {
            let mut m = measure.clone();
            for (j, _) in params.iter().enumerate() {
                m = Expr::App(Box::new(m), Box::new(Expr::BVar(j as u32)));
            }
            m
        };
        let goal = Expr::App(
            Box::new(Expr::App(
                Box::new(relation.clone()),
                Box::new(call_measure),
            )),
            Box::new(caller_measure),
        );
        let mut obligation = ProofObligation::new(
            format!(
                "termination proof for '{}', call #{}: show measure decreases",
                name,
                i + 1,
            ),
            goal,
        );
        for (param_name, param_ty) in params {
            obligation = obligation.with_hypothesis(param_name.clone(), param_ty.clone());
        }
        obligation = obligation.with_hypothesis(Name::str("_inst"), domain_type.clone());
        obligations.push(obligation);
    }
    obligations
}
/// Wrap a body expression with lambda binders for all parameters.
pub fn wrap_with_params(params: &[(Name, Expr)], body: &Expr) -> Expr {
    let mut result = body.clone();
    for (param_name, param_ty) in params.iter().rev() {
        result = Expr::Lam(
            BinderInfo::Default,
            param_name.clone(),
            Box::new(param_ty.clone()),
            Box::new(result),
        );
    }
    result
}
/// Unwrap lambda binders from an expression, collecting parameter info.
pub fn unwrap_lambdas(expr: &Expr) -> (Vec<(Name, Expr)>, &Expr) {
    let mut params = Vec::new();
    let mut current = expr;
    loop {
        match current {
            Expr::Lam(_, name, ty, body) => {
                params.push((name.clone(), (**ty).clone()));
                current = body;
            }
            _ => return (params, current),
        }
    }
}
/// Unwrap pi binders from a type expression, collecting parameter info.
pub fn unwrap_pis(expr: &Expr) -> (Vec<(Name, Expr)>, &Expr) {
    let mut params = Vec::new();
    let mut current = expr;
    loop {
        match current {
            Expr::Pi(_, name, ty, body) => {
                params.push((name.clone(), (**ty).clone()));
                current = body;
            }
            _ => return (params, current),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::predef::*;
    use oxilean_kernel::{Environment, Expr, Level, Literal, Name};
    fn mk_nat() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    #[allow(dead_code)]
    fn mk_nat_succ(e: Expr) -> Expr {
        Expr::App(
            Box::new(Expr::Const(Name::str("Nat.succ"), vec![])),
            Box::new(e),
        )
    }
    #[allow(dead_code)]
    fn mk_nat_zero() -> Expr {
        Expr::Const(Name::str("Nat.zero"), vec![])
    }
    fn mk_app(f: Expr, a: Expr) -> Expr {
        Expr::App(Box::new(f), Box::new(a))
    }
    #[test]
    fn test_recursion_kind_display() {
        let rk = RecursionKind::Structural(Name::str("n"));
        assert!(format!("{}", rk).contains("structural"));
        let rk = RecursionKind::NonRecursive;
        assert_eq!(format!("{}", rk), "non-recursive");
        let rk = RecursionKind::Mutual(vec![Name::str("f"), Name::str("g")]);
        assert!(format!("{}", rk).contains("mutual"));
    }
    #[test]
    fn test_structural_rec_param() {
        let param = StructuralRecParam::new(Name::str("n"), 0, Name::str("Nat"))
            .with_num_type_params(0)
            .with_direct(true);
        assert_eq!(param.param_idx, 0);
        assert!(param.is_direct);
        assert!(format!("{}", param).contains("param 'n'"));
    }
    #[test]
    fn test_rec_call() {
        let call = RecCall::new(Name::str("fib"), vec![Expr::BVar(0)])
            .with_guard(Name::str("Nat.succ"))
            .with_decreasing_arg(0)
            .with_nesting_depth(1);
        assert!(call.in_match_branch);
        assert_eq!(call.guard_ctor, Some(Name::str("Nat.succ")));
        assert!(call.has_decreasing_arg());
        assert_eq!(call.nesting_depth, 1);
    }
    #[test]
    fn test_find_no_recursive_calls() {
        let body = Expr::Lit(Literal::Nat(42));
        let names: HashSet<Name> = [Name::str("f")].into_iter().collect();
        let calls = find_recursive_calls(&body, &names);
        assert!(calls.is_empty());
    }
    #[test]
    fn test_find_simple_recursive_call() {
        let body = mk_app(Expr::Const(Name::str("f"), vec![]), Expr::BVar(0));
        let names: HashSet<Name> = [Name::str("f")].into_iter().collect();
        let calls = find_recursive_calls(&body, &names);
        assert!(!calls.is_empty());
        assert_eq!(calls[0].callee, Name::str("f"));
    }
    #[test]
    fn test_find_nested_recursive_calls() {
        let inner_call = mk_app(Expr::Const(Name::str("f"), vec![]), Expr::BVar(0));
        let outer_call = mk_app(Expr::Const(Name::str("f"), vec![]), inner_call);
        let names: HashSet<Name> = [Name::str("f")].into_iter().collect();
        let calls = find_recursive_calls(&outer_call, &names);
        assert!(calls.len() >= 2);
    }
    #[test]
    fn test_structural_decrease_bvar() {
        assert!(check_structural_decrease(&Name::str("n"), &Expr::BVar(0)));
    }
    #[test]
    fn test_structural_decrease_projection() {
        let proj = Expr::Proj(
            Name::str("field"),
            0,
            Box::new(Expr::Const(Name::str("n"), vec![])),
        );
        assert!(check_structural_decrease(&Name::str("n"), &proj));
    }
    #[test]
    fn test_structural_decrease_literal_fails() {
        assert!(!check_structural_decrease(
            &Name::str("n"),
            &Expr::Lit(Literal::Nat(5))
        ));
    }
    #[test]
    fn test_termination_checker_non_recursive() {
        let env = Environment::new();
        let mut checker = TerminationChecker::with_defaults(&env);
        let params = vec![(Name::str("n"), mk_nat())];
        let body = Expr::Lit(Literal::Nat(0));
        let result = checker.check(&Name::str("f"), &params, &body, &env);
        assert!(result.is_ok());
        let result = result.expect("test operation should succeed");
        assert_eq!(result.kind, RecursionKind::NonRecursive);
        assert!(result.is_safe);
    }
    #[test]
    fn test_termination_checker_structural_nat() {
        let env = Environment::new();
        let mut checker = TerminationChecker::with_defaults(&env);
        let params = vec![(Name::str("n"), mk_nat())];
        let body = mk_app(Expr::Const(Name::str("f"), vec![]), Expr::BVar(0));
        let result = checker.check(&Name::str("f"), &params, &body, &env);
        assert!(result.is_ok());
        let result = result.expect("test operation should succeed");
        match &result.kind {
            RecursionKind::Structural(name) => {
                assert_eq!(*name, Name::str("n"));
            }
            _ => panic!("expected structural recursion, got {:?}", result.kind),
        }
    }
    #[test]
    fn test_well_founded_order_validate() {
        let nat_lt = Expr::Const(Name::str("Nat.lt"), vec![]);
        let nat_lt_wf = Expr::Const(Name::str("Nat.lt.wf"), vec![]);
        let nat_type = mk_nat();
        let order = WellFoundedOrder::new(nat_lt, nat_lt_wf, nat_type);
        assert!(order.validate().is_ok());
    }
    #[test]
    fn test_well_founded_order_build_fix() {
        let nat_lt = Expr::Const(Name::str("Nat.lt"), vec![]);
        let nat_lt_wf = Expr::Const(Name::str("Nat.lt.wf"), vec![]);
        let nat_type = mk_nat();
        let order = WellFoundedOrder::new(nat_lt, nat_lt_wf, nat_type);
        let body = Expr::Lit(Literal::Nat(0));
        let fix = order.build_wf_fix(&body, &Name::str("n"));
        assert!(matches!(fix, Expr::App(_, _)));
    }
    #[test]
    fn test_mutual_rec_group_basic() {
        let mut group = MutualRecGroup::new();
        group.add_function(
            Name::str("even"),
            mk_nat(),
            Expr::Lit(Literal::Nat(0)),
            vec![(Name::str("n"), mk_nat())],
        );
        group.add_function(
            Name::str("odd"),
            mk_nat(),
            Expr::Lit(Literal::Nat(1)),
            vec![(Name::str("n"), mk_nat())],
        );
        assert_eq!(group.size(), 2);
        assert_eq!(group.index_of(&Name::str("even")), Some(0));
        assert_eq!(group.index_of(&Name::str("odd")), Some(1));
    }
    #[test]
    fn test_mutual_rec_group_decrease_matrix() {
        let mut group = MutualRecGroup::new();
        group.add_function(
            Name::str("even"),
            mk_nat(),
            Expr::Lit(Literal::Nat(0)),
            vec![(Name::str("n"), mk_nat())],
        );
        group.add_function(
            Name::str("odd"),
            mk_nat(),
            Expr::Lit(Literal::Nat(1)),
            vec![(Name::str("n"), mk_nat())],
        );
        group.record_decrease(0, 1, vec![ArgDecrease::Decreasing]);
        group.record_decrease(1, 0, vec![ArgDecrease::Decreasing]);
        let result = group.validate_mutual_termination();
        assert!(result.is_ok());
    }
    #[test]
    fn test_mutual_rec_group_format_matrix() {
        let mut group = MutualRecGroup::new();
        group.add_function(
            Name::str("f"),
            mk_nat(),
            Expr::Lit(Literal::Nat(0)),
            vec![(Name::str("n"), mk_nat())],
        );
        group.record_decrease(0, 0, vec![ArgDecrease::Decreasing]);
        let formatted = group.format_matrix();
        assert!(formatted.contains("f -> f"));
        assert!(formatted.contains("<"));
    }
    #[test]
    fn test_arg_decrease_display() {
        assert_eq!(format!("{}", ArgDecrease::Decreasing), "<");
        assert_eq!(format!("{}", ArgDecrease::Equal), "=");
        assert_eq!(format!("{}", ArgDecrease::Unknown), "?");
        assert_eq!(format!("{}", ArgDecrease::Missing), "-");
    }
    #[test]
    fn test_predef_config_defaults() {
        let config = PreDefConfig::new();
        assert_eq!(config.max_depth, 100);
        assert!(config.auto_wf);
        assert!(!config.allow_partial);
        assert!(!config.generate_proof_obligations);
    }
    #[test]
    fn test_predef_config_builder() {
        let config = PreDefConfig::new()
            .with_max_depth(50)
            .with_proof_obligations()
            .with_partial();
        assert_eq!(config.max_depth, 50);
        assert!(config.generate_proof_obligations);
        assert!(config.allow_partial);
    }
    #[test]
    fn test_termination_result_non_recursive() {
        let result = TerminationResult::non_recursive();
        assert_eq!(result.kind, RecursionKind::NonRecursive);
        assert!(result.is_safe);
        assert!(result.calls.is_empty());
    }
    #[test]
    fn test_termination_result_structural() {
        let param = StructuralRecParam::new(Name::str("n"), 0, Name::str("Nat"));
        let result = TerminationResult::structural(param, vec![]);
        match result.kind {
            RecursionKind::Structural(name) => assert_eq!(name, Name::str("n")),
            _ => panic!("expected structural"),
        }
        assert!(result.structural_param.is_some());
    }
    #[test]
    fn test_proof_obligation() {
        let obligation = ProofObligation::new("show n < m".to_string(), Expr::Sort(Level::Zero))
            .with_hypothesis(Name::str("n"), mk_nat())
            .with_source_range(10, 20)
            .mark_discharged();
        assert_eq!(obligation.context.len(), 1);
        assert_eq!(obligation.source_range, Some((10, 20)));
        assert!(obligation.auto_discharged);
        assert!(format!("{}", obligation).contains("show n < m"));
    }
    #[test]
    fn test_build_fix_term_nat() {
        let params = vec![(Name::str("n"), mk_nat())];
        let body = mk_app(Expr::Const(Name::str("f"), vec![]), Expr::BVar(0));
        let ret_type = mk_nat();
        let fix = build_fix_term(&Name::str("f"), &params, &body, 0, &ret_type);
        assert!(matches!(fix, Expr::App(_, _)));
    }
    #[test]
    fn test_build_fix_term_invalid_idx() {
        let params = vec![(Name::str("n"), mk_nat())];
        let body = Expr::Lit(Literal::Nat(0));
        let ret_type = mk_nat();
        let fix = build_fix_term(&Name::str("f"), &params, &body, 5, &ret_type);
        assert_eq!(fix, body);
    }
    #[test]
    fn test_subterm_relation_nat() {
        let mut rel = SubtermRelation::new(Name::str("Nat"));
        rel.add_constructor(Name::str("Nat.zero"), 0, vec![]);
        rel.add_constructor(Name::str("Nat.succ"), 1, vec![0]);
        assert!(!rel.is_recursive_arg(&Name::str("Nat.zero"), 0));
        assert!(rel.is_recursive_arg(&Name::str("Nat.succ"), 0));
        assert_eq!(rel.get_recursive_args(&Name::str("Nat.succ")), &[0]);
    }
    #[test]
    fn test_subterm_relation_standard() {
        let relations = SubtermRelation::build_standard();
        assert!(relations.len() >= 4);
        let nat_rel = relations
            .iter()
            .find(|r| r.inductive_type == Name::str("Nat"))
            .expect("test operation should succeed");
        assert!(nat_rel.is_recursive_arg(&Name::str("Nat.succ"), 0));
        let list_rel = relations
            .iter()
            .find(|r| r.inductive_type == Name::str("List"))
            .expect("test operation should succeed");
        assert!(list_rel.is_recursive_arg(&Name::str("List.cons"), 1));
    }
    #[test]
    fn test_recursion_detector_non_recursive() {
        let body = Expr::Lit(Literal::Nat(42));
        assert!(!RecursionDetector::is_recursive(&Name::str("f"), &body));
    }
    #[test]
    fn test_recursion_detector_recursive() {
        let body = mk_app(Expr::Const(Name::str("f"), vec![]), Expr::BVar(0));
        assert!(RecursionDetector::is_recursive(&Name::str("f"), &body));
        assert_eq!(RecursionDetector::count_calls(&Name::str("f"), &body), 1);
    }
    #[test]
    fn test_recursion_detector_mutual() {
        let body = mk_app(Expr::Const(Name::str("g"), vec![]), Expr::BVar(0));
        let names: HashSet<Name> = [Name::str("f"), Name::str("g")].into_iter().collect();
        assert!(RecursionDetector::is_mutually_recursive(&names, &body));
    }
    #[test]
    fn test_wrap_with_params() {
        let params = vec![(Name::str("a"), mk_nat()), (Name::str("b"), mk_nat())];
        let body = Expr::Lit(Literal::Nat(0));
        let wrapped = wrap_with_params(&params, &body);
        assert!(matches!(wrapped, Expr::Lam(_, _, _, _)));
    }
    #[test]
    fn test_unwrap_lambdas() {
        let body = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(mk_nat()),
            Box::new(Expr::Lam(
                BinderInfo::Default,
                Name::str("y"),
                Box::new(mk_nat()),
                Box::new(Expr::Lit(Literal::Nat(0))),
            )),
        );
        let (params, inner) = unwrap_lambdas(&body);
        assert_eq!(params.len(), 2);
        assert_eq!(params[0].0, Name::str("x"));
        assert_eq!(params[1].0, Name::str("y"));
        assert!(matches!(inner, Expr::Lit(Literal::Nat(0))));
    }
    #[test]
    fn test_unwrap_pis() {
        let ty = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(mk_nat()),
            Box::new(mk_nat()),
        );
        let (params, ret) = unwrap_pis(&ty);
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].0, Name::str("x"));
        assert!(matches!(ret, Expr::Const(_, _)));
    }
    #[test]
    fn test_predef_analyzer_non_recursive() {
        let env = Environment::new();
        let mut analyzer = PreDefAnalyzer::new(&env);
        let params = vec![(Name::str("n"), mk_nat())];
        let body = Expr::Lit(Literal::Nat(0));
        let ret_type = mk_nat();
        let result = analyzer.analyze(&Name::str("f"), &params, &body, &ret_type);
        assert!(result.is_ok());
        assert_eq!(
            result.expect("test operation should succeed").kind,
            RecursionKind::NonRecursive
        );
    }
    #[test]
    fn test_predef_analyzer_structural() {
        let env = Environment::new();
        let mut analyzer = PreDefAnalyzer::new(&env);
        let params = vec![(Name::str("n"), mk_nat())];
        let body = mk_app(Expr::Const(Name::str("f"), vec![]), Expr::BVar(0));
        let ret_type = mk_nat();
        let result = analyzer.analyze(&Name::str("f"), &params, &body, &ret_type);
        assert!(result.is_ok());
        let result = result.expect("test operation should succeed");
        assert!(matches!(result.kind, RecursionKind::Structural(_)));
        assert!(result.result_term.is_some());
    }
    #[test]
    fn test_predef_analyzer_mutual() {
        let env = Environment::new();
        let mut analyzer = PreDefAnalyzer::new(&env);
        let mut group = MutualRecGroup::new();
        group.add_function(
            Name::str("even"),
            mk_nat(),
            Expr::Lit(Literal::Nat(0)),
            vec![(Name::str("n"), mk_nat())],
        );
        group.add_function(
            Name::str("odd"),
            mk_nat(),
            Expr::Lit(Literal::Nat(1)),
            vec![(Name::str("n"), mk_nat())],
        );
        group.record_decrease(0, 1, vec![ArgDecrease::Decreasing]);
        group.record_decrease(1, 0, vec![ArgDecrease::Decreasing]);
        let result = analyzer.analyze_mutual(&mut group);
        assert!(result.is_ok());
        assert!(matches!(
            result.expect("test operation should succeed").kind,
            RecursionKind::Mutual(_)
        ));
    }
    #[test]
    fn test_tactic_termination_proof_no_calls() {
        let params = vec![(Name::str("n"), mk_nat())];
        let body = Expr::Lit(Literal::Nat(0));
        let measure = Expr::Const(Name::str("id"), vec![]);
        let relation = Expr::Const(Name::str("Nat.lt"), vec![]);
        let domain_type = mk_nat();
        let obligations = tactic_like_termination_proof(
            &Name::str("f"),
            &params,
            &body,
            &measure,
            &relation,
            &domain_type,
        );
        assert!(obligations.is_empty());
    }
    #[test]
    fn test_tactic_termination_proof_with_calls() {
        let params = vec![(Name::str("n"), mk_nat())];
        let body = mk_app(Expr::Const(Name::str("f"), vec![]), Expr::BVar(0));
        let measure = Expr::Const(Name::str("id"), vec![]);
        let relation = Expr::Const(Name::str("Nat.lt"), vec![]);
        let domain_type = mk_nat();
        let obligations = tactic_like_termination_proof(
            &Name::str("f"),
            &params,
            &body,
            &measure,
            &relation,
            &domain_type,
        );
        assert!(!obligations.is_empty());
        assert!(obligations[0].description.contains("termination proof"));
    }
    #[test]
    fn test_termination_error_display() {
        let err = TerminationError::NoDecreasingArg(Name::str("f"));
        assert!(format!("{}", err).contains("no structurally decreasing"));
        let err = TerminationError::EmptyMutualGroup;
        assert!(format!("{}", err).contains("empty"));
        let err = TerminationError::MaxDepthExceeded(100);
        assert!(format!("{}", err).contains("100"));
        let err = TerminationError::CallNotDecreasing {
            caller: Name::str("f"),
            callee: Name::str("f"),
            reason: "arg not smaller".to_string(),
        };
        assert!(format!("{}", err).contains("does not decrease"));
    }
    #[test]
    fn test_empty_params() {
        let env = Environment::new();
        let mut checker = TerminationChecker::with_defaults(&env);
        let params: Vec<(Name, Expr)> = vec![];
        let body = Expr::Lit(Literal::Nat(0));
        let result = checker.check(&Name::str("f"), &params, &body, &env);
        assert!(result.is_ok());
    }
    #[test]
    fn test_deeply_nested_body() {
        let env = Environment::new();
        let mut checker = TerminationChecker::with_defaults(&env);
        let mut body: Expr = Expr::Lit(Literal::Nat(0));
        for _ in 0..10 {
            body = Expr::Let(
                Name::str("tmp"),
                Box::new(mk_nat()),
                Box::new(body.clone()),
                Box::new(body),
            );
        }
        let params = vec![(Name::str("n"), mk_nat())];
        let result = checker.check(&Name::str("f"), &params, &body, &env);
        assert!(result.is_ok());
        assert_eq!(
            result.expect("test operation should succeed").kind,
            RecursionKind::NonRecursive
        );
    }
    #[test]
    fn test_build_function_type() {
        let params = vec![(Name::str("a"), mk_nat()), (Name::str("b"), mk_nat())];
        let ret = mk_nat();
        let ty = build_function_type(&params, &ret);
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
}
