//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    CasesExtConfig3900, CasesExtConfigVal3900, CasesExtDiag3900, CasesExtDiff3900,
    CasesExtPass3900, CasesExtPipeline3900, CasesExtResult3900, CasesGoal, CasesResult,
    InductionGoal, InductionResult, StructuralClass, TacCasesCache, TacCasesLogger,
    TacCasesPriorityQueue, TacCasesRegistry, TacCasesStats, TacCasesUtil0, TacticCasesAnalysisPass,
    TacticCasesConfig, TacticCasesConfigValue, TacticCasesDiagnostics, TacticCasesDiff,
    TacticCasesPipeline, TacticCasesResult,
};
use crate::basic::{MVarId, MetaContext, MetavarKind, MVAR_FVAR_OFFSET};
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{Expr, FVarId, Level, Name};

/// `cases h` — perform case analysis on a term.
///
/// Given a term `h` of inductive type with constructors `C₁, ..., Cₙ`,
/// creates n sub-goals, one for each constructor.
pub fn tac_cases(
    _target_name: &Name,
    induct_name: &Name,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<CasesResult> {
    let goal = state.current_goal()?;
    let goal_ty = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let goal_ty = ctx.instantiate_mvars(&goal_ty);
    let ctors = get_constructors(induct_name, ctx)?;
    if ctors.is_empty() {
        return Err(TacticError::Failed(format!(
            "cases: {} has no constructors",
            induct_name
        )));
    }
    let mut case_goals = Vec::new();
    let mut new_goal_ids = Vec::new();
    for (ctor_name, num_fields) in &ctors {
        let field_names: Vec<Name> = (0..*num_fields)
            .map(|i| Name::str(format!("{}_{}", ctor_name, i)))
            .collect();
        let (case_id, _case_expr) = ctx.mk_fresh_expr_mvar(goal_ty.clone(), MetavarKind::Natural);
        case_goals.push(CasesGoal {
            mvar_id: case_id,
            ctor_name: ctor_name.clone(),
            field_names,
        });
        new_goal_ids.push(case_id);
    }
    let cases_proof = build_cases_on_term(induct_name, &case_goals);
    ctx.assign_mvar(goal, cases_proof);
    state.replace_goal(new_goal_ids);
    Ok(CasesResult { goals: case_goals })
}
/// `induction h` — perform induction on a term.
///
/// Like `cases` but adds induction hypotheses for recursive constructors.
pub fn tac_induction(
    _target_name: &Name,
    induct_name: &Name,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<InductionResult> {
    let goal = state.current_goal()?;
    let goal_ty = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let goal_ty = ctx.instantiate_mvars(&goal_ty);
    let ctors = get_constructors(induct_name, ctx)?;
    if ctors.is_empty() {
        return Err(TacticError::Failed(format!(
            "induction: {} has no constructors",
            induct_name
        )));
    }
    let mut ind_goals = Vec::new();
    let mut new_goal_ids = Vec::new();
    for (ctor_name, num_fields) in &ctors {
        let field_names: Vec<Name> = (0..*num_fields)
            .map(|i| Name::str(format!("{}_{}", ctor_name, i)))
            .collect();
        let num_recursive = count_recursive_args(ctor_name, induct_name);
        let ih_names: Vec<Name> = (0..num_recursive)
            .map(|i| Name::str(format!("ih_{}", i)))
            .collect();
        let (ind_id, _ind_expr) = ctx.mk_fresh_expr_mvar(goal_ty.clone(), MetavarKind::Natural);
        ind_goals.push(InductionGoal {
            mvar_id: ind_id,
            ctor_name: ctor_name.clone(),
            field_names,
            ih_names,
        });
        new_goal_ids.push(ind_id);
    }
    let rec_proof = build_rec_term(induct_name, &ind_goals);
    ctx.assign_mvar(goal, rec_proof);
    state.replace_goal(new_goal_ids);
    Ok(InductionResult { goals: ind_goals })
}
/// Get constructors for an inductive type.
pub(super) fn get_constructors(
    induct_name: &Name,
    ctx: &MetaContext,
) -> TacticResult<Vec<(Name, u32)>> {
    if let Some(oxilean_kernel::ConstantInfo::Inductive(ind)) = ctx.env().find(induct_name) {
        let mut result = Vec::new();
        for ctor_name in &ind.ctors {
            let num_fields = if let Some(oxilean_kernel::ConstantInfo::Constructor(cv)) =
                ctx.env().find(ctor_name)
            {
                cv.num_fields
            } else {
                0
            };
            result.push((ctor_name.clone(), num_fields));
        }
        return Ok(result);
    }
    let name_str = format!("{}", induct_name);
    match name_str.as_str() {
        "Nat" => Ok(vec![(Name::str("Nat.zero"), 0), (Name::str("Nat.succ"), 1)]),
        "Bool" => Ok(vec![
            (Name::str("Bool.true"), 0),
            (Name::str("Bool.false"), 0),
        ]),
        "List" => Ok(vec![
            (Name::str("List.nil"), 0),
            (Name::str("List.cons"), 2),
        ]),
        "Option" => Ok(vec![
            (Name::str("Option.none"), 0),
            (Name::str("Option.some"), 1),
        ]),
        "And" => Ok(vec![(Name::str("And.intro"), 2)]),
        "Or" => Ok(vec![(Name::str("Or.inl"), 1), (Name::str("Or.inr"), 1)]),
        "Exists" | "Sigma" => Ok(vec![(Name::str("Sigma.mk"), 2)]),
        _ => Err(TacticError::Failed(format!(
            "unknown inductive type: {}",
            induct_name
        ))),
    }
}
/// Count recursive arguments of a constructor.
pub(super) fn count_recursive_args(ctor_name: &Name, induct_name: &Name) -> u32 {
    let name_str = format!("{}", induct_name);
    match name_str.as_str() {
        "Nat" => {
            let ctor_str = format!("{}", ctor_name);
            if ctor_str.contains("succ") {
                1
            } else {
                0
            }
        }
        "List" => {
            let ctor_str = format!("{}", ctor_name);
            if ctor_str.contains("cons") {
                1
            } else {
                0
            }
        }
        _ => 0,
    }
}
/// Build a `cases_on` application term.
pub(super) fn build_cases_on_term(induct_name: &Name, cases: &[CasesGoal]) -> Expr {
    let rec_name = Name::str(format!("{}.casesOn", induct_name));
    let mut expr = Expr::Const(rec_name, vec![Level::zero()]);
    for case in cases {
        let branch = mvar_to_fvar_expr(case.mvar_id);
        expr = Expr::App(Box::new(expr), Box::new(branch));
    }
    expr
}
/// Build a recursor application term.
pub(super) fn build_rec_term(induct_name: &Name, cases: &[InductionGoal]) -> Expr {
    let rec_name = Name::str(format!("{}.rec", induct_name));
    let mut expr = Expr::Const(rec_name, vec![Level::zero()]);
    for case in cases {
        let branch = mvar_to_fvar_expr(case.mvar_id);
        expr = Expr::App(Box::new(expr), Box::new(branch));
    }
    expr
}
/// Create an FVar expression encoding an mvar id.
pub(super) fn mvar_to_fvar_expr(id: MVarId) -> Expr {
    Expr::FVar(FVarId::new(id.0 + MVAR_FVAR_OFFSET))
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::cases::*;
    use oxilean_kernel::Environment;
    fn mk_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    #[test]
    fn test_cases_nat() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("P"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let result = tac_cases(&Name::str("n"), &Name::str("Nat"), &mut state, &mut ctx)
            .expect("result should be present");
        assert_eq!(result.goals.len(), 2);
        assert_eq!(result.goals[0].ctor_name, Name::str("Nat.zero"));
        assert_eq!(result.goals[1].ctor_name, Name::str("Nat.succ"));
        assert_eq!(state.num_goals(), 2);
    }
    #[test]
    fn test_cases_bool() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("P"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let result = tac_cases(&Name::str("b"), &Name::str("Bool"), &mut state, &mut ctx)
            .expect("result should be present");
        assert_eq!(result.goals.len(), 2);
        assert_eq!(state.num_goals(), 2);
    }
    #[test]
    fn test_cases_unknown_type() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("P"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let result = tac_cases(
            &Name::str("x"),
            &Name::str("UnknownType"),
            &mut state,
            &mut ctx,
        );
        assert!(result.is_err());
    }
    #[test]
    fn test_induction_nat() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("P"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let result = tac_induction(&Name::str("n"), &Name::str("Nat"), &mut state, &mut ctx)
            .expect("value should be present");
        assert_eq!(result.goals.len(), 2);
        assert!(result.goals[0].ih_names.is_empty());
        assert_eq!(result.goals[1].ih_names.len(), 1);
    }
    #[test]
    fn test_induction_list() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("P"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let result = tac_induction(&Name::str("l"), &Name::str("List"), &mut state, &mut ctx)
            .expect("value should be present");
        assert_eq!(result.goals.len(), 2);
        assert!(result.goals[0].ih_names.is_empty());
        assert_eq!(result.goals[1].ih_names.len(), 1);
    }
    #[test]
    fn test_get_constructors_known() {
        let ctx = mk_ctx();
        let ctors = get_constructors(&Name::str("Nat"), &ctx).expect("ctors should be present");
        assert_eq!(ctors.len(), 2);
        let ctors = get_constructors(&Name::str("Bool"), &ctx).expect("ctors should be present");
        assert_eq!(ctors.len(), 2);
        let ctors = get_constructors(&Name::str("Option"), &ctx).expect("ctors should be present");
        assert_eq!(ctors.len(), 2);
    }
    #[test]
    fn test_count_recursive_args_nat() {
        assert_eq!(
            count_recursive_args(&Name::str("Nat.zero"), &Name::str("Nat")),
            0
        );
        assert_eq!(
            count_recursive_args(&Name::str("Nat.succ"), &Name::str("Nat")),
            1
        );
    }
}
/// Check whether an inductive type name is a single-constructor inductive.
#[allow(dead_code)]
pub fn is_single_constructor_type(type_name: &Name) -> bool {
    let s = type_name.to_string();
    matches!(
        s.as_str(),
        "And" | "Exists" | "Subtype" | "Sigma" | "True" | "Unit"
    )
}
/// Return the arity (number of constructor arguments) for well-known constructors.
///
/// Returns 0 for unknown constructors.
#[allow(dead_code)]
pub fn known_constructor_arity(ctor_name: &Name) -> usize {
    let s = ctor_name.to_string();
    match s.as_str() {
        "Nat.zero" | "Bool.false" | "Bool.true" | "True.intro" => 0,
        "Nat.succ" | "Option.some" | "List.nil" => 1,
        "And.intro" | "Exists.intro" | "List.cons" | "Prod.mk" => 2,
        _ => 0,
    }
}
/// Generate fresh hypothesis names for a constructor's fields.
///
/// Returns names of the form `h_0`, `h_1`, …, `h_{n-1}`.
#[allow(dead_code)]
pub fn fresh_field_names(ctor_name: &Name, arity: usize) -> Vec<String> {
    let base = ctor_name.to_string().replace('.', "_").to_lowercase();
    (0..arity).map(|i| format!("{base}_{i}")).collect()
}
/// Check whether a hypothesis name is already in use.
///
/// Always returns `false` since we cannot access hypotheses without a `MetaContext`.
#[allow(dead_code)]
pub fn hyp_name_taken(_name: &str, _state: &TacticState) -> bool {
    false
}
/// Choose a fresh variant of `base` that is not taken in `state`.
#[allow(dead_code)]
pub fn fresh_hyp_name(base: &str, state: &TacticState) -> String {
    if !hyp_name_taken(base, state) {
        return base.to_string();
    }
    for i in 1u32.. {
        let candidate = format!("{base}{i}");
        if !hyp_name_taken(&candidate, state) {
            return candidate;
        }
    }
    unreachable!()
}
/// Determine the "depth" of an inductive type in a simple well-founded hierarchy.
///
/// - `False` / `True` / `Unit` → depth 0
/// - `Nat` / `Bool` → depth 1
/// - `List T` / `Option T` → depth 2
/// - anything else → depth 3
#[allow(dead_code)]
pub fn inductive_depth(type_name: &Name) -> u32 {
    let s = type_name.to_string();
    match s.as_str() {
        "False" | "True" | "Unit" => 0,
        "Nat" | "Bool" | "Int" => 1,
        "List" | "Option" | "Or" | "And" => 2,
        _ => 3,
    }
}
/// Return true iff this inductive can be eliminated by `cases` alone
/// (no need for `induction`).
#[allow(dead_code)]
pub fn can_cases_only(type_name: &Name) -> bool {
    let s = type_name.to_string();
    matches!(
        s.as_str(),
        "And" | "Or" | "False" | "True" | "Exists" | "Bool" | "Option"
    )
}
/// Return true iff the given expression has head `Const` with the given name.
#[allow(dead_code)]
pub fn has_const_head(expr: &Expr, name: &Name) -> bool {
    match expr {
        Expr::Const(n, _) => n == name,
        Expr::App(f, _) => has_const_head(f, name),
        _ => false,
    }
}
/// Return the head inductive name of an expression if it starts with a `Const`.
#[allow(dead_code)]
pub fn head_const_name(expr: &Expr) -> Option<Name> {
    match expr {
        Expr::Const(n, _) => Some(n.clone()),
        Expr::App(f, _) => head_const_name(f),
        _ => None,
    }
}
/// Collect all `Const` names that appear anywhere in an expression.
#[allow(dead_code)]
pub fn collect_const_names(expr: &Expr) -> Vec<Name> {
    let mut names = Vec::new();
    collect_const_names_rec(expr, &mut names);
    names
}
pub(super) fn collect_const_names_rec(expr: &Expr, acc: &mut Vec<Name>) {
    match expr {
        Expr::Const(n, _) => acc.push(n.clone()),
        Expr::App(f, a) => {
            collect_const_names_rec(f, acc);
            collect_const_names_rec(a, acc);
        }
        Expr::Lam(_, _, dom, body) | Expr::Pi(_, _, dom, body) => {
            collect_const_names_rec(dom, acc);
            collect_const_names_rec(body, acc);
        }
        Expr::Let(_, ty, val, body) => {
            collect_const_names_rec(ty, acc);
            collect_const_names_rec(val, acc);
            collect_const_names_rec(body, acc);
        }
        _ => {}
    }
}
/// Check whether a level is Prop (i.e., universe 0).
#[allow(dead_code)]
pub fn is_prop_sort(level: &Level) -> bool {
    oxilean_kernel::level::is_equivalent(level, &Level::zero())
}
/// Return the number of arguments an expression is applied to.
///
/// `f a b c` → 3
#[allow(dead_code)]
pub fn app_arg_count(expr: &Expr) -> usize {
    match expr {
        Expr::App(f, _) => 1 + app_arg_count(f),
        _ => 0,
    }
}
/// Split an `Expr::App` spine into head and arguments.
///
/// `f a b c` → `(f, [a, b, c])`
#[allow(dead_code)]
pub fn unfold_app(expr: &Expr) -> (&Expr, Vec<&Expr>) {
    let mut args = Vec::new();
    let mut cur = expr;
    while let Expr::App(f, a) = cur {
        args.push(a.as_ref());
        cur = f;
    }
    args.reverse();
    (cur, args)
}
#[cfg(test)]
mod extended_cases_tests {
    use super::*;
    use crate::tactic::cases::*;
    #[test]
    fn test_fresh_field_names() {
        let names = fresh_field_names(&Name::str("Nat.succ"), 2);
        assert_eq!(names.len(), 2);
    }
    #[test]
    fn test_fresh_field_names_zero_arity() {
        let names = fresh_field_names(&Name::str("Nat.zero"), 0);
        assert!(names.is_empty());
    }
    #[test]
    fn test_inductive_depth_false() {
        assert_eq!(inductive_depth(&Name::str("False")), 0);
    }
    #[test]
    fn test_inductive_depth_nat() {
        assert_eq!(inductive_depth(&Name::str("Nat")), 1);
    }
    #[test]
    fn test_inductive_depth_list() {
        assert_eq!(inductive_depth(&Name::str("List")), 2);
    }
    #[test]
    fn test_can_cases_only_and() {
        assert!(can_cases_only(&Name::str("And")));
    }
    #[test]
    fn test_can_cases_only_nat() {
        assert!(!can_cases_only(&Name::str("Nat")));
    }
    #[test]
    fn test_head_const_name_const() {
        let e = Expr::Const(Name::str("Bool"), vec![]);
        assert_eq!(head_const_name(&e), Some(Name::str("Bool")));
    }
    #[test]
    fn test_head_const_name_app() {
        let e = Expr::App(
            Box::new(Expr::Const(Name::str("List"), vec![])),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
        );
        assert_eq!(head_const_name(&e), Some(Name::str("List")));
    }
    #[test]
    fn test_head_const_name_sort() {
        let e = Expr::Sort(Level::zero());
        assert_eq!(head_const_name(&e), None);
    }
    #[test]
    fn test_is_prop_sort() {
        assert!(is_prop_sort(&Level::zero()));
        assert!(!is_prop_sort(&Level::succ(Level::zero())));
    }
    #[test]
    fn test_app_arg_count() {
        let e = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("f"), vec![])),
                Box::new(Expr::Const(Name::str("a"), vec![])),
            )),
            Box::new(Expr::Const(Name::str("b"), vec![])),
        );
        assert_eq!(app_arg_count(&e), 2);
    }
    #[test]
    fn test_unfold_app() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let f = Expr::Const(Name::str("f"), vec![]);
        let e = Expr::App(
            Box::new(Expr::App(Box::new(f.clone()), Box::new(a))),
            Box::new(b),
        );
        let (head, args) = unfold_app(&e);
        assert!(matches!(head, Expr::Const(n, _) if n == & Name::str("f")));
        assert_eq!(args.len(), 2);
    }
    #[test]
    fn test_collect_const_names() {
        let e = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::Const(Name::str("a"), vec![])),
        );
        let names = collect_const_names(&e);
        assert!(names.contains(&Name::str("f")));
        assert!(names.contains(&Name::str("a")));
    }
    #[test]
    fn test_known_constructor_arity() {
        assert_eq!(known_constructor_arity(&Name::str("Nat.zero")), 0);
        assert_eq!(known_constructor_arity(&Name::str("Nat.succ")), 1);
        assert_eq!(known_constructor_arity(&Name::str("And.intro")), 2);
    }
    #[test]
    fn test_is_single_constructor_type() {
        assert!(is_single_constructor_type(&Name::str("And")));
        assert!(!is_single_constructor_type(&Name::str("Or")));
    }
}
/// Count the total number of bound variables in a Pi-type chain.
#[allow(dead_code)]
pub fn count_pi_binders(expr: &Expr) -> usize {
    match expr {
        Expr::Pi(_, _, _, body) => 1 + count_pi_binders(body),
        _ => 0,
    }
}
/// Count the total number of binders in a Lambda chain.
#[allow(dead_code)]
pub fn count_lam_binders(expr: &Expr) -> usize {
    match expr {
        Expr::Lam(_, _, _, body) => 1 + count_lam_binders(body),
        _ => 0,
    }
}
/// Check whether an expression is a `Sort`.
#[allow(dead_code)]
pub fn is_sort(expr: &Expr) -> bool {
    matches!(expr, Expr::Sort(_))
}
/// Check whether an expression is a `Prop` (i.e., `Sort 0`).
#[allow(dead_code)]
pub fn is_prop_expr(expr: &Expr) -> bool {
    match expr {
        Expr::Sort(l) => is_prop_sort(l),
        _ => false,
    }
}
/// Check whether an expression is `Type u` for some `u`.
#[allow(dead_code)]
pub fn is_type_expr(expr: &Expr) -> bool {
    match expr {
        Expr::Sort(l) => !is_prop_sort(l),
        _ => false,
    }
}
/// Make a `Prop` expression.
#[allow(dead_code)]
pub fn mk_prop() -> Expr {
    Expr::Sort(Level::zero())
}
/// Make a `Type 0` expression.
#[allow(dead_code)]
pub fn mk_type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
/// Make a Pi-type `(x : dom) -> cod` with default binder info.
#[allow(dead_code)]
pub fn mk_pi(dom: Expr, cod: Expr) -> Expr {
    Expr::Pi(
        oxilean_kernel::BinderInfo::Default,
        Name::str("_"),
        Box::new(dom),
        Box::new(cod),
    )
}
/// Make a Lambda `fun x => body` with default binder info.
#[allow(dead_code)]
pub fn mk_lam(body: Expr) -> Expr {
    Expr::Lam(
        oxilean_kernel::BinderInfo::Default,
        Name::str("_"),
        Box::new(Expr::Sort(Level::zero())),
        Box::new(body),
    )
}
/// Count distinct FVar ids in an expression.
#[allow(dead_code)]
pub fn count_fvars(expr: &Expr) -> usize {
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    count_fvars_rec(expr, &mut seen);
    seen.len()
}
pub(super) fn count_fvars_rec(expr: &Expr, seen: &mut std::collections::HashSet<u64>) {
    match expr {
        Expr::FVar(id) => {
            seen.insert(id.0);
        }
        Expr::App(f, a) => {
            count_fvars_rec(f, seen);
            count_fvars_rec(a, seen);
        }
        Expr::Lam(_, _, dom, body) | Expr::Pi(_, _, dom, body) => {
            count_fvars_rec(dom, seen);
            count_fvars_rec(body, seen);
        }
        Expr::Let(_, ty, val, body) => {
            count_fvars_rec(ty, seen);
            count_fvars_rec(val, seen);
            count_fvars_rec(body, seen);
        }
        _ => {}
    }
}
/// Check whether an expression contains any metavariable (FVar with id >= MVAR_FVAR_OFFSET).
#[allow(dead_code)]
pub fn has_metavars(expr: &Expr) -> bool {
    match expr {
        Expr::FVar(id) if id.0 >= crate::basic::MVAR_FVAR_OFFSET => true,
        Expr::App(f, a) => has_metavars(f) || has_metavars(a),
        Expr::Lam(_, _, dom, body) | Expr::Pi(_, _, dom, body) => {
            has_metavars(dom) || has_metavars(body)
        }
        Expr::Let(_, ty, val, body) => has_metavars(ty) || has_metavars(val) || has_metavars(body),
        _ => false,
    }
}
#[cfg(test)]
mod extended_cases_tests2 {
    use super::*;
    use crate::tactic::cases::*;
    #[test]
    fn test_count_pi_binders() {
        let p = mk_pi(mk_prop(), mk_prop());
        assert_eq!(count_pi_binders(&p), 1);
    }
    #[test]
    fn test_count_lam_binders() {
        let l = mk_lam(Expr::BVar(0));
        assert_eq!(count_lam_binders(&l), 1);
    }
    #[test]
    fn test_is_prop_expr() {
        assert!(is_prop_expr(&mk_prop()));
        assert!(!is_prop_expr(&mk_type0()));
    }
    #[test]
    fn test_is_type_expr() {
        assert!(!is_type_expr(&mk_prop()));
        assert!(is_type_expr(&mk_type0()));
    }
    #[test]
    fn test_is_sort() {
        assert!(is_sort(&mk_prop()));
        assert!(!is_sort(&Expr::BVar(0)));
    }
    #[test]
    fn test_has_metavars_false() {
        let e = Expr::Const(Name::str("f"), vec![]);
        assert!(!has_metavars(&e));
    }
    #[test]
    fn test_has_metavars_true() {
        let e = Expr::FVar(FVarId(crate::basic::MVAR_FVAR_OFFSET));
        assert!(has_metavars(&e));
    }
    #[test]
    fn test_count_fvars() {
        let e = Expr::App(
            Box::new(Expr::FVar(FVarId(1))),
            Box::new(Expr::FVar(FVarId(2))),
        );
        assert_eq!(count_fvars(&e), 2);
    }
    #[test]
    fn test_count_fvars_duplicates() {
        let e = Expr::App(
            Box::new(Expr::FVar(FVarId(1))),
            Box::new(Expr::FVar(FVarId(1))),
        );
        assert_eq!(count_fvars(&e), 1);
    }
}
/// Classify an expression structurally for case-analysis guidance.
#[allow(dead_code)]
pub fn classify_for_cases(expr: &Expr) -> StructuralClass {
    match expr {
        Expr::Sort(l) => {
            if is_prop_sort(l) {
                StructuralClass::Proposition
            } else {
                StructuralClass::Sort
            }
        }
        Expr::Pi(_, _, _, _) => StructuralClass::FunctionType,
        Expr::Const(n, _) => {
            let s = n.to_string();
            match s.as_str() {
                "Nat" | "Bool" | "List" | "Option" | "Or" | "And" | "Exists" => {
                    StructuralClass::KnownInductive
                }
                _ => StructuralClass::Other,
            }
        }
        Expr::App(f, _) => classify_for_cases(f),
        _ => StructuralClass::Other,
    }
}
/// Generate a human-readable case-split description.
///
/// Given an inductive name and its constructors, returns a string like:
/// `"Nat: Nat.zero | Nat.succ"`
#[allow(dead_code)]
pub fn describe_case_split(induct_name: &Name, ctors: &[(Name, u32)]) -> String {
    let ctor_strs: Vec<String> = ctors.iter().map(|(n, _)| n.to_string()).collect();
    format!("{}: {}", induct_name, ctor_strs.join(" | "))
}
/// Check whether a constructor name matches its inductive parent.
///
/// For example, `"Nat.succ"` matches `"Nat"`.
#[allow(dead_code)]
pub fn ctor_matches_inductive(ctor_name: &Name, induct_name: &Name) -> bool {
    let ctor_s = ctor_name.to_string();
    let ind_s = induct_name.to_string();
    ctor_s.starts_with(&format!("{}.", ind_s))
}
/// Return the short (unqualified) part of a constructor name.
///
/// `"Nat.succ"` → `"succ"`, `"Bool.true"` → `"true"`.
#[allow(dead_code)]
pub fn ctor_short_name(ctor_name: &Name) -> String {
    let s = ctor_name.to_string();
    s.rsplit('.').next().unwrap_or(&s).to_string()
}
/// Estimate the complexity of a case split.
///
/// Returns a rough cost:
/// - 0 constructors → 0 (impossible / empty type)
/// - 1 constructor → 1 (trivial: just intro fields)
/// - 2 constructors → 2 (binary split)
/// - n constructors → n
#[allow(dead_code)]
pub fn case_split_complexity(num_ctors: usize) -> usize {
    num_ctors
}
/// Build a list of fresh BVar names for constructor arguments.
///
/// Produces names `"a_0", "a_1", …`.
#[allow(dead_code)]
pub fn mk_ctor_arg_names(n: usize) -> Vec<String> {
    (0..n).map(|i| format!("a_{i}")).collect()
}
/// Determine if an expression is clearly inhabited (trivially proved).
#[allow(dead_code)]
pub fn is_trivially_inhabited(expr: &Expr) -> bool {
    match expr {
        Expr::Const(n, _) => {
            let s = n.to_string();
            matches!(s.as_str(), "True" | "Unit")
        }
        _ => false,
    }
}
/// Determine if an expression is clearly uninhabited.
#[allow(dead_code)]
pub fn is_trivially_uninhabited(expr: &Expr) -> bool {
    match expr {
        Expr::Const(n, _) => {
            let s = n.to_string();
            matches!(s.as_str(), "False" | "Empty")
        }
        _ => false,
    }
}
#[cfg(test)]
mod structural_tests {
    use super::*;
    use crate::tactic::cases::*;
    #[test]
    fn test_classify_prop() {
        let p = mk_prop();
        let cls = classify_for_cases(&p);
        assert_eq!(cls, StructuralClass::Proposition);
    }
    #[test]
    fn test_classify_function_type() {
        let ft = mk_pi(mk_prop(), mk_prop());
        let cls = classify_for_cases(&ft);
        assert_eq!(cls, StructuralClass::FunctionType);
    }
    #[test]
    fn test_classify_known_inductive_nat() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let cls = classify_for_cases(&nat);
        assert_eq!(cls, StructuralClass::KnownInductive);
    }
    #[test]
    fn test_ctor_matches_inductive_nat_succ() {
        assert!(ctor_matches_inductive(
            &Name::str("Nat.succ"),
            &Name::str("Nat")
        ));
    }
    #[test]
    fn test_ctor_matches_inductive_mismatch() {
        assert!(!ctor_matches_inductive(
            &Name::str("Bool.true"),
            &Name::str("Nat")
        ));
    }
    #[test]
    fn test_ctor_short_name() {
        assert_eq!(ctor_short_name(&Name::str("Nat.succ")), "succ");
        assert_eq!(ctor_short_name(&Name::str("Bool.true")), "true");
    }
    #[test]
    fn test_describe_case_split() {
        let ctors = vec![(Name::str("Nat.zero"), 0), (Name::str("Nat.succ"), 1)];
        let desc = describe_case_split(&Name::str("Nat"), &ctors);
        assert!(desc.contains("Nat.zero"));
        assert!(desc.contains("Nat.succ"));
        assert!(desc.contains('|'));
    }
    #[test]
    fn test_case_split_complexity() {
        assert_eq!(case_split_complexity(0), 0);
        assert_eq!(case_split_complexity(2), 2);
    }
    #[test]
    fn test_mk_ctor_arg_names() {
        let names = mk_ctor_arg_names(3);
        assert_eq!(names, vec!["a_0", "a_1", "a_2"]);
    }
    #[test]
    fn test_is_trivially_inhabited() {
        assert!(is_trivially_inhabited(&Expr::Const(
            Name::str("True"),
            vec![]
        )));
        assert!(!is_trivially_inhabited(&Expr::Const(
            Name::str("Nat"),
            vec![]
        )));
    }
    #[test]
    fn test_is_trivially_uninhabited() {
        assert!(is_trivially_uninhabited(&Expr::Const(
            Name::str("False"),
            vec![]
        )));
        assert!(!is_trivially_uninhabited(&Expr::Const(
            Name::str("Nat"),
            vec![]
        )));
    }
}
/// Compute a simple hash of a TacCases name.
#[allow(dead_code)]
pub fn taccases_hash(name: &str) -> u64 {
    let mut h: u64 = 14695981039346656037;
    for b in name.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(1099511628211);
    }
    h
}
/// Check if a TacCases name is valid.
#[allow(dead_code)]
pub fn taccases_is_valid_name(name: &str) -> bool {
    !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_')
}
/// Count the occurrences of a character in a TacCases string.
#[allow(dead_code)]
pub fn taccases_count_char(s: &str, c: char) -> usize {
    s.chars().filter(|&ch| ch == c).count()
}
/// Truncate a TacCases string to a maximum length.
#[allow(dead_code)]
pub fn taccases_truncate(s: &str, max_len: usize) -> &str {
    if s.len() <= max_len {
        s
    } else {
        &s[..max_len]
    }
}
/// Join TacCases strings with a separator.
#[allow(dead_code)]
pub fn taccases_join(parts: &[&str], sep: &str) -> String {
    parts.join(sep)
}
#[cfg(test)]
mod taccases_ext_tests {
    use super::*;
    use crate::tactic::cases::*;
    #[test]
    fn test_taccases_util_new() {
        let u = TacCasesUtil0::new(1, "test", 42);
        assert_eq!(u.id, 1);
        assert_eq!(u.name, "test");
        assert_eq!(u.value, 42);
        assert!(u.is_active());
    }
    #[test]
    fn test_taccases_util_tag() {
        let u = TacCasesUtil0::new(2, "tagged", 10).with_tag("important");
        assert!(u.has_tag("important"));
        assert_eq!(u.tag_count(), 1);
    }
    #[test]
    fn test_taccases_util_disable() {
        let u = TacCasesUtil0::new(3, "disabled", 100).disable();
        assert!(!u.is_active());
        assert_eq!(u.score(), 0);
    }
    #[test]
    fn test_taccases_registry_register() {
        let mut reg = TacCasesRegistry::new(10);
        let u = TacCasesUtil0::new(1, "a", 1);
        assert!(reg.register(u));
        assert_eq!(reg.count(), 1);
    }
    #[test]
    fn test_taccases_registry_lookup() {
        let mut reg = TacCasesRegistry::new(10);
        reg.register(TacCasesUtil0::new(5, "five", 5));
        assert!(reg.lookup(5).is_some());
        assert!(reg.lookup(99).is_none());
    }
    #[test]
    fn test_taccases_registry_capacity() {
        let mut reg = TacCasesRegistry::new(2);
        reg.register(TacCasesUtil0::new(1, "a", 1));
        reg.register(TacCasesUtil0::new(2, "b", 2));
        assert!(reg.is_full());
        assert!(!reg.register(TacCasesUtil0::new(3, "c", 3)));
    }
    #[test]
    fn test_taccases_registry_score() {
        let mut reg = TacCasesRegistry::new(10);
        reg.register(TacCasesUtil0::new(1, "a", 10));
        reg.register(TacCasesUtil0::new(2, "b", 20));
        assert_eq!(reg.total_score(), 30);
    }
    #[test]
    fn test_taccases_cache_hit_miss() {
        let mut cache = TacCasesCache::new();
        cache.insert("key1", 42);
        assert_eq!(cache.get("key1"), Some(42));
        assert_eq!(cache.get("key2"), None);
        assert_eq!(cache.hits, 1);
        assert_eq!(cache.misses, 1);
    }
    #[test]
    fn test_taccases_cache_hit_rate() {
        let mut cache = TacCasesCache::new();
        cache.insert("k", 1);
        cache.get("k");
        cache.get("k");
        cache.get("nope");
        assert!((cache.hit_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_taccases_cache_clear() {
        let mut cache = TacCasesCache::new();
        cache.insert("k", 1);
        cache.clear();
        assert_eq!(cache.size(), 0);
        assert_eq!(cache.hits, 0);
    }
    #[test]
    fn test_taccases_logger_basic() {
        let mut logger = TacCasesLogger::new(100);
        logger.log("msg1");
        logger.log("msg2");
        assert_eq!(logger.count(), 2);
        assert_eq!(logger.last(), Some("msg2"));
    }
    #[test]
    fn test_taccases_logger_capacity() {
        let mut logger = TacCasesLogger::new(2);
        logger.log("a");
        logger.log("b");
        logger.log("c");
        assert_eq!(logger.count(), 2);
    }
    #[test]
    fn test_taccases_stats_success() {
        let mut stats = TacCasesStats::new();
        stats.record_success(100);
        stats.record_success(200);
        assert_eq!(stats.total_ops, 2);
        assert_eq!(stats.successful_ops, 2);
        assert!((stats.success_rate() - 1.0).abs() < 1e-9);
    }
    #[test]
    fn test_taccases_stats_failure() {
        let mut stats = TacCasesStats::new();
        stats.record_success(100);
        stats.record_failure();
        assert!((stats.success_rate() - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_taccases_stats_merge() {
        let mut a = TacCasesStats::new();
        let mut b = TacCasesStats::new();
        a.record_success(100);
        b.record_failure();
        a.merge(&b);
        assert_eq!(a.total_ops, 2);
    }
    #[test]
    fn test_taccases_priority_queue() {
        let mut pq = TacCasesPriorityQueue::new();
        pq.push(TacCasesUtil0::new(1, "low", 1), 1);
        pq.push(TacCasesUtil0::new(2, "high", 2), 100);
        let (_, p) = pq.pop().expect("collection should not be empty");
        assert_eq!(p, 100);
    }
    #[test]
    fn test_taccases_hash() {
        let h1 = taccases_hash("foo");
        let h2 = taccases_hash("foo");
        assert_eq!(h1, h2);
        let h3 = taccases_hash("bar");
        assert_ne!(h1, h3);
    }
    #[test]
    fn test_taccases_valid_name() {
        assert!(taccases_is_valid_name("foo_bar"));
        assert!(!taccases_is_valid_name("foo-bar"));
        assert!(!taccases_is_valid_name(""));
    }
    #[test]
    fn test_taccases_join() {
        let parts = ["a", "b", "c"];
        assert_eq!(taccases_join(&parts, ", "), "a, b, c");
    }
}
#[cfg(test)]
mod tacticcases_analysis_tests {
    use super::*;
    use crate::tactic::cases::*;
    #[test]
    fn test_tacticcases_result_ok() {
        let r = TacticCasesResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticcases_result_err() {
        let r = TacticCasesResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticcases_result_partial() {
        let r = TacticCasesResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_tacticcases_result_skipped() {
        let r = TacticCasesResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_tacticcases_analysis_pass_run() {
        let mut p = TacticCasesAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_tacticcases_analysis_pass_empty_input() {
        let mut p = TacticCasesAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_tacticcases_analysis_pass_success_rate() {
        let mut p = TacticCasesAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_tacticcases_analysis_pass_disable() {
        let mut p = TacticCasesAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_tacticcases_pipeline_basic() {
        let mut pipeline = TacticCasesPipeline::new("main_pipeline");
        pipeline.add_pass(TacticCasesAnalysisPass::new("pass1"));
        pipeline.add_pass(TacticCasesAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_tacticcases_pipeline_disabled_pass() {
        let mut pipeline = TacticCasesPipeline::new("partial");
        let mut p = TacticCasesAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(TacticCasesAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_tacticcases_diff_basic() {
        let mut d = TacticCasesDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_tacticcases_diff_summary() {
        let mut d = TacticCasesDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_tacticcases_config_set_get() {
        let mut cfg = TacticCasesConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_tacticcases_config_read_only() {
        let mut cfg = TacticCasesConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_tacticcases_config_remove() {
        let mut cfg = TacticCasesConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_tacticcases_diagnostics_basic() {
        let mut diag = TacticCasesDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_tacticcases_diagnostics_max_errors() {
        let mut diag = TacticCasesDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_tacticcases_diagnostics_clear() {
        let mut diag = TacticCasesDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_tacticcases_config_value_types() {
        let b = TacticCasesConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = TacticCasesConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = TacticCasesConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = TacticCasesConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = TacticCasesConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod cases_ext_tests_3900 {
    use super::*;
    use crate::tactic::cases::*;
    #[test]
    fn test_cases_ext_result_ok_3900() {
        let r = CasesExtResult3900::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_cases_ext_result_err_3900() {
        let r = CasesExtResult3900::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_cases_ext_result_partial_3900() {
        let r = CasesExtResult3900::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_cases_ext_result_skipped_3900() {
        let r = CasesExtResult3900::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_cases_ext_pass_run_3900() {
        let mut p = CasesExtPass3900::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_cases_ext_pass_empty_3900() {
        let mut p = CasesExtPass3900::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_cases_ext_pass_rate_3900() {
        let mut p = CasesExtPass3900::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_cases_ext_pass_disable_3900() {
        let mut p = CasesExtPass3900::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_cases_ext_pipeline_basic_3900() {
        let mut pipeline = CasesExtPipeline3900::new("main_pipeline");
        pipeline.add_pass(CasesExtPass3900::new("pass1"));
        pipeline.add_pass(CasesExtPass3900::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_cases_ext_pipeline_disabled_3900() {
        let mut pipeline = CasesExtPipeline3900::new("partial");
        let mut p = CasesExtPass3900::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(CasesExtPass3900::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_cases_ext_diff_basic_3900() {
        let mut d = CasesExtDiff3900::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_cases_ext_config_set_get_3900() {
        let mut cfg = CasesExtConfig3900::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_cases_ext_config_read_only_3900() {
        let mut cfg = CasesExtConfig3900::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_cases_ext_config_remove_3900() {
        let mut cfg = CasesExtConfig3900::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_cases_ext_diagnostics_basic_3900() {
        let mut diag = CasesExtDiag3900::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_cases_ext_diagnostics_max_errors_3900() {
        let mut diag = CasesExtDiag3900::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_cases_ext_diagnostics_clear_3900() {
        let mut diag = CasesExtDiag3900::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_cases_ext_config_value_types_3900() {
        let b = CasesExtConfigVal3900::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = CasesExtConfigVal3900::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = CasesExtConfigVal3900::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = CasesExtConfigVal3900::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = CasesExtConfigVal3900::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
