//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    ConstructorAttempt, ConstructorExtConfig3500, ConstructorExtConfigVal3500,
    ConstructorExtDiag3500, ConstructorExtDiff3500, ConstructorExtPass3500,
    ConstructorExtPipeline3500, ConstructorExtResult3500, ConstructorInfo, ConstructorRegistry,
    ConstructorTable, ConstructorTacStats, TacCtorBuilder, TacCtorCounterMap, TacCtorExtMap,
    TacCtorExtUtil, TacCtorStateMachine, TacCtorWindow, TacCtorWorkQueue,
    TacticConstructorAnalysisPass, TacticConstructorConfig, TacticConstructorConfigValue,
    TacticConstructorDiagnostics, TacticConstructorDiff, TacticConstructorPipeline,
    TacticConstructorResult,
};
use crate::basic::{MVarId, MetaContext, MetavarKind};
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{Expr, Level, Name};

/// `constructor` — apply the first applicable constructor.
///
/// For a goal of type `T` where `T` is an inductive type,
/// applies the first constructor and creates subgoals for its arguments.
pub fn tac_constructor(
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<Vec<MVarId>> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let head_name = get_head_const(&target).ok_or_else(|| {
        TacticError::GoalMismatch("goal is not an inductive type application".into())
    })?;
    let ctor = get_first_constructor(&head_name)?;
    apply_constructor(&ctor.0, ctor.1, goal, state, ctx)
}
/// `left` — apply `Or.inl` to an `Or` goal.
pub fn tac_left(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<Vec<MVarId>> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let head = get_head_const(&target);
    if head.as_ref().map(|n| format!("{}", n)) != Some("Or".to_string()) {
        return Err(TacticError::GoalMismatch(
            "left: goal is not an Or type".into(),
        ));
    }
    apply_constructor(&Name::str("Or.inl"), 1, goal, state, ctx)
}
/// `right` — apply `Or.inr` to an `Or` goal.
pub fn tac_right(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<Vec<MVarId>> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let head = get_head_const(&target);
    if head.as_ref().map(|n| format!("{}", n)) != Some("Or".to_string()) {
        return Err(TacticError::GoalMismatch(
            "right: goal is not an Or type".into(),
        ));
    }
    apply_constructor(&Name::str("Or.inr"), 1, goal, state, ctx)
}
/// `exists e` / `exact ⟨e, _⟩` — provide the witness for an existential.
pub fn tac_existsi(
    witness: Expr,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<MVarId> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let head = get_head_const(&target);
    let is_exists = head
        .as_ref()
        .map(|n| {
            let s = format!("{}", n);
            s == "Exists" || s == "Sigma"
        })
        .unwrap_or(false);
    if !is_exists {
        return Err(TacticError::GoalMismatch(
            "existsi: goal is not an Exists/Sigma type".into(),
        ));
    }
    let sort_ty = Expr::Sort(Level::zero());
    let (proof_id, proof_expr) = ctx.mk_fresh_expr_mvar(sort_ty, MetavarKind::Natural);
    let ctor = Expr::Const(Name::str("Sigma.mk"), vec![Level::zero(), Level::zero()]);
    let proof = Expr::App(
        Box::new(Expr::App(Box::new(ctor), Box::new(witness))),
        Box::new(proof_expr),
    );
    ctx.assign_mvar(goal, proof);
    state.replace_goal(vec![proof_id]);
    Ok(proof_id)
}
/// Get the head constant of an application.
pub(super) fn get_head_const(expr: &Expr) -> Option<Name> {
    let mut e = expr;
    while let Expr::App(f, _) = e {
        e = f;
    }
    match e {
        Expr::Const(name, _) => Some(name.clone()),
        _ => None,
    }
}
/// Get the first constructor of an inductive type.
pub(super) fn get_first_constructor(type_name: &Name) -> TacticResult<(Name, u32)> {
    let name_str = format!("{}", type_name);
    match name_str.as_str() {
        "True" => Ok((Name::str("True.intro"), 0)),
        "And" => Ok((Name::str("And.intro"), 2)),
        "Or" => Ok((Name::str("Or.inl"), 1)),
        "Exists" | "Sigma" => Ok((Name::str("Sigma.mk"), 2)),
        "Unit" => Ok((Name::str("Unit.unit"), 0)),
        "Prod" => Ok((Name::str("Prod.mk"), 2)),
        "Nat" => Ok((Name::str("Nat.zero"), 0)),
        "Bool" => Ok((Name::str("Bool.true"), 0)),
        "List" => Ok((Name::str("List.nil"), 0)),
        "Option" => Ok((Name::str("Option.none"), 0)),
        _ => Err(TacticError::Failed(format!(
            "constructor: no known constructors for {}",
            type_name
        ))),
    }
}
/// Apply a specific constructor to close a goal.
pub(super) fn apply_constructor(
    ctor_name: &Name,
    num_fields: u32,
    goal: MVarId,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<Vec<MVarId>> {
    let mut subgoals = Vec::new();
    let mut app = Expr::Const(ctor_name.clone(), vec![Level::zero()]);
    let sort_ty = Expr::Sort(Level::zero());
    for _ in 0..num_fields {
        let (field_id, field_expr) = ctx.mk_fresh_expr_mvar(sort_ty.clone(), MetavarKind::Natural);
        subgoals.push(field_id);
        app = Expr::App(Box::new(app), Box::new(field_expr));
    }
    ctx.assign_mvar(goal, app);
    state.replace_goal(subgoals.clone());
    Ok(subgoals)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::constructor::*;
    use oxilean_kernel::Environment;
    fn mk_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    #[test]
    fn test_constructor_true() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("True"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let result = tac_constructor(&mut state, &mut ctx).expect("result should be present");
        assert!(result.is_empty());
        assert!(state.is_done());
    }
    #[test]
    fn test_constructor_and() {
        let mut ctx = mk_ctx();
        let p = Expr::Const(Name::str("P"), vec![]);
        let q = Expr::Const(Name::str("Q"), vec![]);
        let goal_ty = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("And"), vec![])),
                Box::new(p),
            )),
            Box::new(q),
        );
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let result = tac_constructor(&mut state, &mut ctx).expect("result should be present");
        assert_eq!(result.len(), 2);
        assert_eq!(state.num_goals(), 2);
    }
    #[test]
    fn test_left() {
        let mut ctx = mk_ctx();
        let p = Expr::Const(Name::str("P"), vec![]);
        let q = Expr::Const(Name::str("Q"), vec![]);
        let goal_ty = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Or"), vec![])),
                Box::new(p),
            )),
            Box::new(q),
        );
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let result = tac_left(&mut state, &mut ctx).expect("result should be present");
        assert_eq!(result.len(), 1);
        assert_eq!(state.num_goals(), 1);
    }
    #[test]
    fn test_right() {
        let mut ctx = mk_ctx();
        let p = Expr::Const(Name::str("P"), vec![]);
        let q = Expr::Const(Name::str("Q"), vec![]);
        let goal_ty = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Or"), vec![])),
                Box::new(p),
            )),
            Box::new(q),
        );
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let result = tac_right(&mut state, &mut ctx).expect("result should be present");
        assert_eq!(result.len(), 1);
        assert_eq!(state.num_goals(), 1);
    }
    #[test]
    fn test_left_wrong_type() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("Nat"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let result = tac_left(&mut state, &mut ctx);
        assert!(result.is_err());
    }
    #[test]
    fn test_existsi() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::App(
            Box::new(Expr::Const(Name::str("Exists"), vec![])),
            Box::new(Expr::Const(Name::str("P"), vec![])),
        );
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let witness = Expr::Const(Name::str("Nat.zero"), vec![]);
        let proof_id =
            tac_existsi(witness, &mut state, &mut ctx).expect("proof_id should be present");
        assert_eq!(state.num_goals(), 1);
        assert!(!ctx.is_mvar_assigned(proof_id));
    }
    #[test]
    fn test_existsi_wrong_type() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("Nat"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let witness = Expr::Const(Name::str("x"), vec![]);
        let result = tac_existsi(witness, &mut state, &mut ctx);
        assert!(result.is_err());
    }
    #[test]
    fn test_get_head_const() {
        let a = Expr::Const(Name::str("And"), vec![]);
        let pa = Expr::App(Box::new(a), Box::new(Expr::Const(Name::str("P"), vec![])));
        assert_eq!(get_head_const(&pa), Some(Name::str("And")));
        let bvar = Expr::BVar(0);
        assert_eq!(get_head_const(&bvar), None);
    }
}
/// Apply the nth constructor (0-indexed) to close a goal.
pub fn tac_nth_constructor(
    n: usize,
    state: &mut crate::tactic::state::TacticState,
    ctx: &mut MetaContext,
) -> crate::tactic::state::TacticResult<Vec<MVarId>> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| crate::tactic::state::TacticError::Internal("no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let head_name = get_head_const(&target).ok_or_else(|| {
        crate::tactic::state::TacticError::GoalMismatch(
            "goal is not an inductive type application".into(),
        )
    })?;
    let table = ConstructorTable::builtin();
    let type_str = format!("{}", head_name);
    let ctors = table.lookup(&type_str).ok_or_else(|| {
        crate::tactic::state::TacticError::Failed(format!(
            "no constructors known for {}",
            head_name
        ))
    })?;
    let ctor = ctors.get(n).ok_or_else(|| {
        crate::tactic::state::TacticError::Failed(format!(
            "constructor index {} out of range for {} (has {} constructors)",
            n,
            head_name,
            ctors.len()
        ))
    })?;
    apply_constructor(&ctor.name, ctor.num_fields, goal, state, ctx)
}
/// split tactic for And: applies And.intro and creates two sub-goals.
pub fn tac_split_and(
    state: &mut crate::tactic::state::TacticState,
    ctx: &mut MetaContext,
) -> crate::tactic::state::TacticResult<Vec<MVarId>> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| crate::tactic::state::TacticError::Internal("no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let head = get_head_const(&target);
    let head_str = head.as_ref().map(|n| format!("{}", n));
    match head_str.as_deref() {
        Some("And") => apply_constructor(&Name::str("And.intro"), 2, goal, state, ctx),
        Some("Iff") => apply_constructor(&Name::str("Iff.intro"), 2, goal, state, ctx),
        Some("Prod") => apply_constructor(&Name::str("Prod.mk"), 2, goal, state, ctx),
        _ => Err(crate::tactic::state::TacticError::GoalMismatch(
            "split: goal is not And, Iff, or Prod".into(),
        )),
    }
}
/// use tactic: introduce a witness for an existential.
pub fn tac_use(
    witness: Expr,
    state: &mut crate::tactic::state::TacticState,
    ctx: &mut MetaContext,
) -> crate::tactic::state::TacticResult<()> {
    tac_existsi(witness, state, ctx)?;
    Ok(())
}
/// Check whether a given name is a known constructor in the builtin table.
pub fn is_known_constructor(name: &Name) -> bool {
    let name_str = format!("{}", name);
    let table = ConstructorTable::builtin();
    table
        .entries
        .iter()
        .any(|(_, ctors)| ctors.iter().any(|c| format!("{}", c.name) == name_str))
}
/// Check if the goal type is a structure.
pub fn goal_is_structure(state: &crate::tactic::state::TacticState, ctx: &MetaContext) -> bool {
    let Ok(goal) = state.current_goal() else {
        return false;
    };
    let Some(target) = ctx.get_mvar_type(goal) else {
        return false;
    };
    let target = ctx.instantiate_mvars(target);
    let Some(head) = get_head_const(&target) else {
        return false;
    };
    let table = ConstructorTable::builtin();
    let type_str = format!("{}", head);
    table.has_unique_constructor(&type_str)
}
/// Get all constructors for the type of the current goal.
pub fn get_goal_constructors(
    state: &crate::tactic::state::TacticState,
    ctx: &MetaContext,
) -> crate::tactic::state::TacticResult<Vec<ConstructorInfo>> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| crate::tactic::state::TacticError::Internal("no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let head = get_head_const(&target).ok_or_else(|| {
        crate::tactic::state::TacticError::GoalMismatch("not an inductive type".into())
    })?;
    let table = ConstructorTable::builtin();
    let type_str = format!("{}", head);
    table.lookup(&type_str).cloned().ok_or_else(|| {
        crate::tactic::state::TacticError::Failed(format!("unknown inductive type {}", head))
    })
}
/// Tactic refine_ctor: creates holes for constructor fields.
pub fn tac_refine_ctor(
    ctor_name: &Name,
    num_fields: u32,
    state: &mut crate::tactic::state::TacticState,
    ctx: &mut MetaContext,
) -> crate::tactic::state::TacticResult<Vec<MVarId>> {
    let goal = state.current_goal()?;
    apply_constructor(ctor_name, num_fields, goal, state, ctx)
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    use crate::tactic::constructor::*;
    use oxilean_kernel::Environment;
    fn mk_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    #[test]
    fn test_constructor_table_builtin() {
        let table = ConstructorTable::builtin();
        assert!(table.num_types() > 5);
    }
    #[test]
    fn test_constructor_table_lookup_and() {
        let table = ConstructorTable::builtin();
        let ctors = table.lookup("And").expect("ctors should be present");
        assert_eq!(ctors.len(), 1);
        assert_eq!(format!("{}", ctors[0].name), "And.intro");
    }
    #[test]
    fn test_constructor_table_has_unique() {
        let table = ConstructorTable::builtin();
        assert!(table.has_unique_constructor("And"));
        assert!(!table.has_unique_constructor("Or"));
    }
    #[test]
    fn test_is_known_constructor_true_intro() {
        assert!(is_known_constructor(&Name::str("True.intro")));
    }
    #[test]
    fn test_is_known_constructor_unknown() {
        assert!(!is_known_constructor(&Name::str("MyCustomCtor")));
    }
    #[test]
    fn test_tac_nth_constructor_first() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Or"), vec![])),
                Box::new(Expr::Const(Name::str("P"), vec![])),
            )),
            Box::new(Expr::Const(Name::str("Q"), vec![])),
        );
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let result =
            tac_nth_constructor(0, &mut state, &mut ctx).expect("result should be present");
        assert_eq!(result.len(), 1);
    }
    #[test]
    fn test_tac_split_and() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("And"), vec![])),
                Box::new(Expr::Const(Name::str("P"), vec![])),
            )),
            Box::new(Expr::Const(Name::str("Q"), vec![])),
        );
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let result = tac_split_and(&mut state, &mut ctx).expect("result should be present");
        assert_eq!(result.len(), 2);
    }
    #[test]
    fn test_tac_split_non_and() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("Nat"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        assert!(tac_split_and(&mut state, &mut ctx).is_err());
    }
    #[test]
    fn test_constructor_info_builder() {
        let info = ConstructorInfo::new(Name::str("MyType.mk"), 3)
            .unique()
            .polymorphic();
        assert_eq!(info.num_fields, 3);
        assert!(info.is_unique && info.is_polymorphic);
    }
    #[test]
    fn test_get_goal_constructors_nat() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("Nat"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let state = TacticState::single(mvar_id);
        let ctors = get_goal_constructors(&state, &ctx).expect("ctors should be present");
        assert_eq!(ctors.len(), 2);
    }
    #[test]
    fn test_goal_is_structure_and() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("And"), vec![])),
                Box::new(Expr::Const(Name::str("P"), vec![])),
            )),
            Box::new(Expr::Const(Name::str("Q"), vec![])),
        );
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let state = TacticState::single(mvar_id);
        assert!(goal_is_structure(&state, &ctx));
    }
    #[test]
    fn test_tac_refine_ctor() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("And"), vec![])),
                Box::new(Expr::Const(Name::str("P"), vec![])),
            )),
            Box::new(Expr::Const(Name::str("Q"), vec![])),
        );
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let result = tac_refine_ctor(&Name::str("And.intro"), 2, &mut state, &mut ctx)
            .expect("result should be present");
        assert_eq!(result.len(), 2);
    }
}
/// Check if any goal in the state is for a structure type.
pub fn any_goal_is_structure(state: &crate::tactic::state::TacticState, ctx: &MetaContext) -> bool {
    for &goal_id in state.all_goals() {
        if let Some(ty) = ctx.get_mvar_type(goal_id) {
            let ty = ctx.instantiate_mvars(ty);
            if let Some(head) = get_head_const(&ty) {
                let table = ConstructorTable::builtin();
                let s = format!("{}", head);
                if table.has_unique_constructor(&s) {
                    return true;
                }
            }
        }
    }
    false
}
/// List all known inductive type names in the builtin table.
pub fn all_builtin_type_names() -> Vec<String> {
    let table = ConstructorTable::builtin();
    table.entries.iter().map(|(n, _)| n.clone()).collect()
}
/// Check if a type name is in the builtin table.
pub fn is_builtin_type(type_name: &str) -> bool {
    ConstructorTable::builtin().lookup(type_name).is_some()
}
/// Count the total number of constructors in the builtin table.
pub fn count_all_builtin_constructors() -> usize {
    let table = ConstructorTable::builtin();
    table.entries.iter().map(|(_, ctors)| ctors.len()).sum()
}
#[cfg(test)]
mod builtin_tests {
    use super::*;
    use crate::tactic::constructor::*;
    use oxilean_kernel::Environment;
    fn mk_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    #[test]
    fn test_all_builtin_type_names() {
        let names = all_builtin_type_names();
        assert!(names.contains(&"Nat".to_string()));
        assert!(names.contains(&"Bool".to_string()));
    }
    #[test]
    fn test_is_builtin_type_nat() {
        assert!(is_builtin_type("Nat"));
    }
    #[test]
    fn test_is_builtin_type_unknown() {
        assert!(!is_builtin_type("MyCustomType"));
    }
    #[test]
    fn test_count_all_builtin_constructors() {
        let count = count_all_builtin_constructors();
        assert!(count > 10);
    }
    #[test]
    fn test_any_goal_is_structure_true() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("And"), vec![])),
                Box::new(Expr::Const(Name::str("P"), vec![])),
            )),
            Box::new(Expr::Const(Name::str("Q"), vec![])),
        );
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let state = TacticState::single(mvar_id);
        assert!(any_goal_is_structure(&state, &ctx));
    }
    #[test]
    fn test_any_goal_is_structure_false() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Or"), vec![])),
                Box::new(Expr::Const(Name::str("P"), vec![])),
            )),
            Box::new(Expr::Const(Name::str("Q"), vec![])),
        );
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let state = TacticState::single(mvar_id);
        assert!(!any_goal_is_structure(&state, &ctx));
    }
    #[test]
    fn test_constructor_table_first_constructor() {
        let table = ConstructorTable::builtin();
        let first = table
            .first_constructor("Nat")
            .expect("first should be present");
        assert_eq!(format!("{}", first.name), "Nat.zero");
    }
    #[test]
    fn test_constructor_table_num_constructors() {
        let table = ConstructorTable::builtin();
        assert_eq!(table.num_constructors("Nat"), 2);
        assert_eq!(table.num_constructors("True"), 1);
    }
    #[test]
    fn test_tac_nth_constructor_out_of_range() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("True"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        assert!(tac_nth_constructor(1, &mut state, &mut ctx).is_err());
    }
    #[test]
    fn test_tac_use_exists() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::App(
            Box::new(Expr::Const(Name::str("Exists"), vec![])),
            Box::new(Expr::Const(Name::str("P"), vec![])),
        );
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let witness = Expr::Const(Name::str("Nat.zero"), vec![]);
        assert!(tac_use(witness, &mut state, &mut ctx).is_ok());
    }
}
/// Returns the number of implicit arguments a constructor expects.
#[allow(dead_code)]
pub fn constructor_implicit_count(name: &str) -> usize {
    match name {
        "Eq.refl" => 1,
        "And.intro" => 2,
        "Or.inl" | "Or.inr" => 2,
        "Iff.intro" => 2,
        "Exists.intro" => 2,
        "Prod.mk" => 2,
        "Sigma.mk" => 2,
        "Subtype.mk" => 2,
        "List.nil" => 1,
        "List.cons" => 1,
        "Option.none" => 1,
        "Option.some" => 1,
        "Sum.inl" | "Sum.inr" => 2,
        _ => 0,
    }
}
/// Checks whether a given constructor name is a "product-like" constructor
/// (i.e., it produces a term with multiple fields that can all be filled simultaneously).
#[allow(dead_code)]
pub fn is_product_like_ctor(name: &str) -> bool {
    matches!(
        name,
        "And.intro" | "Prod.mk" | "Sigma.mk" | "Subtype.mk" | "Iff.intro"
    )
}
/// Returns a human-readable description of what a constructor does.
#[allow(dead_code)]
pub fn constructor_description(name: &str) -> &'static str {
    match name {
        "And.intro" => "Proves a conjunction by providing both components",
        "Or.inl" => "Proves a disjunction using the left component",
        "Or.inr" => "Proves a disjunction using the right component",
        "Eq.refl" => "Proves an equality by reflexivity",
        "Iff.intro" => "Proves an iff by providing forward and backward directions",
        "Exists.intro" => "Proves an existential by providing a witness and proof",
        "Prod.mk" => "Constructs a product pair",
        "List.nil" => "Constructs an empty list",
        "List.cons" => "Constructs a list by prepending an element",
        _ => "Unknown constructor",
    }
}
/// Attempts to guess whether the goal type (given as a string) is solved
/// by a specific constructor name.
#[allow(dead_code)]
pub fn goal_matches_constructor(goal_str: &str, ctor: &str) -> bool {
    match ctor {
        "And.intro" => goal_str.contains(" ∧ ") || goal_str.contains(" And "),
        "Or.inl" | "Or.inr" => goal_str.contains(" ∨ ") || goal_str.contains(" Or "),
        "Eq.refl" => goal_str.contains(" = "),
        "Iff.intro" => goal_str.contains(" ↔ ") || goal_str.contains(" Iff "),
        "Exists.intro" => goal_str.contains("∃") || goal_str.contains("Exists"),
        "Prod.mk" => goal_str.contains(" × ") || goal_str.contains("Prod"),
        _ => false,
    }
}
/// Iterates over all known product-like builtin constructors and returns their names.
#[allow(dead_code)]
pub fn product_constructors() -> Vec<&'static str> {
    vec![
        "And.intro",
        "Prod.mk",
        "Sigma.mk",
        "Subtype.mk",
        "Iff.intro",
    ]
}
/// Returns true if the tactic state currently has any open goals that look like
/// they could be closed by a constructor application (based on goal string heuristics).
#[allow(dead_code)]
pub fn any_goal_closeable_by_ctor(goals: &[String], ctor: &str) -> bool {
    goals.iter().any(|g| goal_matches_constructor(g, ctor))
}
#[cfg(test)]
mod extra_ctor_tests {
    use super::*;
    use crate::tactic::constructor::*;
    #[test]
    fn test_implicit_count_and_intro() {
        assert_eq!(constructor_implicit_count("And.intro"), 2);
    }
    #[test]
    fn test_implicit_count_eq_refl() {
        assert_eq!(constructor_implicit_count("Eq.refl"), 1);
    }
    #[test]
    fn test_implicit_count_unknown() {
        assert_eq!(constructor_implicit_count("MyCustomCtor"), 0);
    }
    #[test]
    fn test_is_product_like_and() {
        assert!(is_product_like_ctor("And.intro"));
    }
    #[test]
    fn test_is_product_like_or() {
        assert!(!is_product_like_ctor("Or.inl"));
    }
    #[test]
    fn test_constructor_description_and() {
        let desc = constructor_description("And.intro");
        assert!(desc.contains("conjunction"));
    }
    #[test]
    fn test_goal_matches_and() {
        assert!(goal_matches_constructor("P ∧ Q", "And.intro"));
    }
    #[test]
    fn test_goal_matches_iff() {
        assert!(goal_matches_constructor("P ↔ Q", "Iff.intro"));
    }
    #[test]
    fn test_product_constructors_nonempty() {
        assert!(!product_constructors().is_empty());
    }
    #[test]
    fn test_any_goal_closeable() {
        let goals = vec!["P ∧ Q".to_string(), "R".to_string()];
        assert!(any_goal_closeable_by_ctor(&goals, "And.intro"));
    }
}
/// Infer the number of explicit arguments a constructor takes from its arity.
///
/// This is used to determine how many subgoals will be generated.
#[allow(dead_code)]
pub fn ctor_explicit_arity(ctor_name: &str) -> usize {
    match ctor_name {
        "And.intro" => 2,
        "Or.inl" | "Or.inr" => 1,
        "Iff.intro" => 2,
        "Exists.intro" => 2,
        "Sigma.mk" => 2,
        "Prod.mk" => 2,
        "Subtype.mk" => 2,
        "True.intro" => 0,
        "Eq.refl" => 1,
        "List.nil" => 0,
        "List.cons" => 2,
        "Nat.zero" => 0,
        "Nat.succ" => 1,
        _ => 1,
    }
}
/// Return the dual constructor for `Or`: inl ↔ inr.
#[allow(dead_code)]
pub fn or_dual(ctor_name: &str) -> Option<&'static str> {
    match ctor_name {
        "Or.inl" => Some("Or.inr"),
        "Or.inr" => Some("Or.inl"),
        _ => None,
    }
}
/// Check if a constructor closes the goal (produces 0 subgoals).
#[allow(dead_code)]
pub fn ctor_closes_goal(ctor_name: &str) -> bool {
    ctor_explicit_arity(ctor_name) == 0
}
/// Describe what a constructor expects (for error messages).
#[allow(dead_code)]
pub fn ctor_expects(ctor_name: &str) -> &'static str {
    match ctor_name {
        "And.intro" => "two proofs: P and Q",
        "Or.inl" => "a proof of the left disjunct",
        "Or.inr" => "a proof of the right disjunct",
        "Iff.intro" => "a forward and backward implication",
        "Exists.intro" => "a witness and a proof",
        "Sigma.mk" => "a value and a dependent pair",
        "Prod.mk" => "a left and right component",
        "True.intro" => "nothing",
        "Eq.refl" => "a term equal to itself",
        _ => "arguments",
    }
}
/// Format a human-readable summary of a constructor tactic outcome.
#[allow(dead_code)]
pub fn format_ctor_outcome(ctor_name: &str, num_goals: usize) -> String {
    if num_goals == 0 {
        format!("constructor `{}` closed the goal", ctor_name)
    } else {
        format!(
            "constructor `{}` generated {} subgoal(s)",
            ctor_name, num_goals
        )
    }
}
/// Return `true` if the goal type string looks like an `Iff` goal.
#[allow(dead_code)]
pub fn looks_like_iff(goal_type: &str) -> bool {
    goal_type.contains("↔") || goal_type.contains("Iff")
}
/// Return `true` if the goal type string looks like a `Prod` goal.
#[allow(dead_code)]
pub fn looks_like_prod(goal_type: &str) -> bool {
    goal_type.contains("×") || goal_type.contains("Prod")
}
/// Return the list of constructors to try for an `And`/`Iff`/`Prod` goal.
#[allow(dead_code)]
pub fn constructors_for_conjunction(goal_type: &str) -> Vec<&'static str> {
    if looks_like_iff(goal_type) {
        vec!["Iff.intro"]
    } else if looks_like_prod(goal_type) {
        vec!["Prod.mk"]
    } else {
        vec!["And.intro"]
    }
}
/// Validate that a constructor name is well-formed.
#[allow(dead_code)]
pub fn is_valid_ctor_name(s: &str) -> bool {
    !s.is_empty() && s.contains('.') && s.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
}
/// Priority for trying constructors (higher = try first).
#[allow(dead_code)]
pub fn ctor_priority(ctor_name: &str) -> u8 {
    match ctor_name {
        "True.intro" | "Eq.refl" => 10,
        "And.intro" | "Iff.intro" => 5,
        "Or.inl" | "Or.inr" => 3,
        _ => 1,
    }
}
/// Sort constructors by priority (highest first).
#[allow(dead_code)]
pub fn sort_ctors_by_priority(mut ctors: Vec<String>) -> Vec<String> {
    ctors.sort_by_key(|b| std::cmp::Reverse(ctor_priority(b)));
    ctors
}
#[cfg(test)]
mod extended_ctor_tests {
    use super::*;
    use crate::tactic::constructor::*;
    #[test]
    fn test_constructor_registry_standard() {
        let reg = ConstructorRegistry::standard();
        assert!(reg.contains("And"));
        assert!(reg.contains("Or"));
        assert!(reg.contains("Iff"));
    }
    #[test]
    fn test_constructor_registry_lookup() {
        let reg = ConstructorRegistry::standard();
        let ctors = reg.lookup("Or").expect("ctors should be present");
        assert_eq!(ctors.len(), 2);
    }
    #[test]
    fn test_constructor_registry_first_ctor() {
        let reg = ConstructorRegistry::standard();
        let first = reg.first_ctor("And").expect("first should be present");
        assert_eq!(*first, Name::str("And.intro"));
    }
    #[test]
    fn test_ctor_explicit_arity() {
        assert_eq!(ctor_explicit_arity("And.intro"), 2);
        assert_eq!(ctor_explicit_arity("Or.inl"), 1);
        assert_eq!(ctor_explicit_arity("True.intro"), 0);
    }
    #[test]
    fn test_ctor_closes_goal() {
        assert!(ctor_closes_goal("True.intro"));
        assert!(!ctor_closes_goal("And.intro"));
    }
    #[test]
    fn test_or_dual() {
        assert_eq!(or_dual("Or.inl"), Some("Or.inr"));
        assert_eq!(or_dual("Or.inr"), Some("Or.inl"));
        assert_eq!(or_dual("And.intro"), None);
    }
    #[test]
    fn test_format_ctor_outcome_closed() {
        let s = format_ctor_outcome("True.intro", 0);
        assert!(s.contains("closed"));
    }
    #[test]
    fn test_format_ctor_outcome_subgoals() {
        let s = format_ctor_outcome("And.intro", 2);
        assert!(s.contains("2 subgoal"));
    }
    #[test]
    fn test_looks_like_iff() {
        assert!(looks_like_iff("P ↔ Q"));
        assert!(looks_like_iff("Iff P Q"));
        assert!(!looks_like_iff("P ∧ Q"));
    }
    #[test]
    fn test_constructors_for_conjunction() {
        let c = constructors_for_conjunction("P ↔ Q");
        assert_eq!(c, vec!["Iff.intro"]);
        let c2 = constructors_for_conjunction("P ∧ Q");
        assert_eq!(c2, vec!["And.intro"]);
    }
    #[test]
    fn test_is_valid_ctor_name() {
        assert!(is_valid_ctor_name("And.intro"));
        assert!(!is_valid_ctor_name("andintro"));
        assert!(!is_valid_ctor_name(""));
    }
    #[test]
    fn test_ctor_priority() {
        assert!(ctor_priority("True.intro") > ctor_priority("Or.inl"));
    }
    #[test]
    fn test_sort_ctors_by_priority() {
        let ctors = vec![
            "Or.inl".to_string(),
            "True.intro".to_string(),
            "And.intro".to_string(),
        ];
        let sorted = sort_ctors_by_priority(ctors);
        assert_eq!(sorted[0], "True.intro");
    }
    #[test]
    fn test_ctor_tac_stats_merge() {
        let mut s1 = ConstructorTacStats {
            constructor_calls: 10,
            failures: 2,
            ..Default::default()
        };
        let s2 = ConstructorTacStats {
            constructor_calls: 5,
            failures: 1,
            ..Default::default()
        };
        s1.merge(&s2);
        assert_eq!(s1.constructor_calls, 15);
        assert_eq!(s1.failures, 3);
    }
    #[test]
    fn test_ctor_attempt_success() {
        let a = ConstructorAttempt::success(Name::str("And.intro"), 2);
        assert!(a.succeeded);
        assert_eq!(a.subgoal_count, 2);
        assert!(a.error.is_none());
    }
    #[test]
    fn test_ctor_attempt_failure() {
        let a = ConstructorAttempt::failure(Name::str("Or.inl"), "goal not Or");
        assert!(!a.succeeded);
        assert!(a.error.is_some());
    }
    #[test]
    fn test_ctor_expects() {
        let s = ctor_expects("And.intro");
        assert!(s.contains("two proofs"));
    }
}
#[cfg(test)]
mod tacctor_ext2_tests {
    use super::*;
    use crate::tactic::constructor::*;
    #[test]
    fn test_tacctor_ext_util_basic() {
        let mut u = TacCtorExtUtil::new("test");
        u.push(10);
        u.push(20);
        assert_eq!(u.sum(), 30);
        assert_eq!(u.len(), 2);
    }
    #[test]
    fn test_tacctor_ext_util_min_max() {
        let mut u = TacCtorExtUtil::new("mm");
        u.push(5);
        u.push(1);
        u.push(9);
        assert_eq!(u.min_val(), Some(1));
        assert_eq!(u.max_val(), Some(9));
    }
    #[test]
    fn test_tacctor_ext_util_flags() {
        let mut u = TacCtorExtUtil::new("flags");
        u.set_flag(3);
        assert!(u.has_flag(3));
        assert!(!u.has_flag(2));
    }
    #[test]
    fn test_tacctor_ext_util_pop() {
        let mut u = TacCtorExtUtil::new("pop");
        u.push(42);
        assert_eq!(u.pop(), Some(42));
        assert!(u.is_empty());
    }
    #[test]
    fn test_tacctor_ext_map_basic() {
        let mut m: TacCtorExtMap<i32> = TacCtorExtMap::new();
        m.insert("key", 42);
        assert_eq!(m.get("key"), Some(&42));
        assert!(m.contains("key"));
        assert!(!m.contains("other"));
    }
    #[test]
    fn test_tacctor_ext_map_get_or_default() {
        let mut m: TacCtorExtMap<i32> = TacCtorExtMap::new();
        m.insert("k", 5);
        assert_eq!(m.get_or_default("k"), 5);
        assert_eq!(m.get_or_default("missing"), 0);
    }
    #[test]
    fn test_tacctor_ext_map_keys_sorted() {
        let mut m: TacCtorExtMap<i32> = TacCtorExtMap::new();
        m.insert("z", 1);
        m.insert("a", 2);
        m.insert("m", 3);
        let keys = m.keys_sorted();
        assert_eq!(keys[0].as_str(), "a");
        assert_eq!(keys[2].as_str(), "z");
    }
    #[test]
    fn test_tacctor_window_mean() {
        let mut w = TacCtorWindow::new(3);
        w.push(1.0);
        w.push(2.0);
        w.push(3.0);
        assert!((w.mean() - 2.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacctor_window_evict() {
        let mut w = TacCtorWindow::new(2);
        w.push(10.0);
        w.push(20.0);
        w.push(30.0);
        assert_eq!(w.len(), 2);
        assert!((w.mean() - 25.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacctor_window_std_dev() {
        let mut w = TacCtorWindow::new(10);
        for i in 0..10 {
            w.push(i as f64);
        }
        assert!(w.std_dev() > 0.0);
    }
    #[test]
    fn test_tacctor_builder_basic() {
        let b = TacCtorBuilder::new("test")
            .add_item("a")
            .add_item("b")
            .set_config("key", "val");
        assert_eq!(b.item_count(), 2);
        assert!(b.has_config("key"));
        assert_eq!(b.get_config("key"), Some("val"));
    }
    #[test]
    fn test_tacctor_builder_summary() {
        let b = TacCtorBuilder::new("suite").add_item("x");
        let s = b.build_summary();
        assert!(s.contains("suite"));
    }
    #[test]
    fn test_tacctor_state_machine_start() {
        let mut sm = TacCtorStateMachine::new();
        assert!(sm.start());
        assert!(sm.state.is_running());
    }
    #[test]
    fn test_tacctor_state_machine_complete() {
        let mut sm = TacCtorStateMachine::new();
        sm.start();
        sm.complete();
        assert!(sm.state.is_terminal());
    }
    #[test]
    fn test_tacctor_state_machine_fail() {
        let mut sm = TacCtorStateMachine::new();
        sm.fail("oops");
        assert!(sm.state.is_terminal());
        assert_eq!(sm.state.error_msg(), Some("oops"));
    }
    #[test]
    fn test_tacctor_state_machine_no_transition_after_terminal() {
        let mut sm = TacCtorStateMachine::new();
        sm.complete();
        assert!(!sm.start());
    }
    #[test]
    fn test_tacctor_work_queue_basic() {
        let mut wq = TacCtorWorkQueue::new(10);
        wq.enqueue("task1".to_string());
        wq.enqueue("task2".to_string());
        assert_eq!(wq.pending_count(), 2);
        let t = wq.dequeue();
        assert_eq!(t, Some("task1".to_string()));
        assert_eq!(wq.processed_count(), 1);
    }
    #[test]
    fn test_tacctor_work_queue_capacity() {
        let mut wq = TacCtorWorkQueue::new(2);
        wq.enqueue("a".to_string());
        wq.enqueue("b".to_string());
        assert!(wq.is_full());
        assert!(!wq.enqueue("c".to_string()));
    }
    #[test]
    fn test_tacctor_counter_map_basic() {
        let mut cm = TacCtorCounterMap::new();
        cm.increment("apple");
        cm.increment("apple");
        cm.increment("banana");
        assert_eq!(cm.count("apple"), 2);
        assert_eq!(cm.count("banana"), 1);
        assert_eq!(cm.num_unique(), 2);
    }
    #[test]
    fn test_tacctor_counter_map_frequency() {
        let mut cm = TacCtorCounterMap::new();
        cm.increment("a");
        cm.increment("a");
        cm.increment("b");
        assert!((cm.frequency("a") - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_tacctor_counter_map_most_common() {
        let mut cm = TacCtorCounterMap::new();
        cm.increment("x");
        cm.increment("y");
        cm.increment("x");
        let (k, v) = cm.most_common().expect("most_common should succeed");
        assert_eq!(k.as_str(), "x");
        assert_eq!(v, 2);
    }
}
#[cfg(test)]
mod tacticconstructor_analysis_tests {
    use super::*;
    use crate::tactic::constructor::*;
    #[test]
    fn test_tacticconstructor_result_ok() {
        let r = TacticConstructorResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticconstructor_result_err() {
        let r = TacticConstructorResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticconstructor_result_partial() {
        let r = TacticConstructorResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_tacticconstructor_result_skipped() {
        let r = TacticConstructorResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_tacticconstructor_analysis_pass_run() {
        let mut p = TacticConstructorAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_tacticconstructor_analysis_pass_empty_input() {
        let mut p = TacticConstructorAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_tacticconstructor_analysis_pass_success_rate() {
        let mut p = TacticConstructorAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_tacticconstructor_analysis_pass_disable() {
        let mut p = TacticConstructorAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_tacticconstructor_pipeline_basic() {
        let mut pipeline = TacticConstructorPipeline::new("main_pipeline");
        pipeline.add_pass(TacticConstructorAnalysisPass::new("pass1"));
        pipeline.add_pass(TacticConstructorAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_tacticconstructor_pipeline_disabled_pass() {
        let mut pipeline = TacticConstructorPipeline::new("partial");
        let mut p = TacticConstructorAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(TacticConstructorAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_tacticconstructor_diff_basic() {
        let mut d = TacticConstructorDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_tacticconstructor_diff_summary() {
        let mut d = TacticConstructorDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_tacticconstructor_config_set_get() {
        let mut cfg = TacticConstructorConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_tacticconstructor_config_read_only() {
        let mut cfg = TacticConstructorConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_tacticconstructor_config_remove() {
        let mut cfg = TacticConstructorConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_tacticconstructor_diagnostics_basic() {
        let mut diag = TacticConstructorDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_tacticconstructor_diagnostics_max_errors() {
        let mut diag = TacticConstructorDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_tacticconstructor_diagnostics_clear() {
        let mut diag = TacticConstructorDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_tacticconstructor_config_value_types() {
        let b = TacticConstructorConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = TacticConstructorConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = TacticConstructorConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = TacticConstructorConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = TacticConstructorConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod constructor_ext_tests_3500 {
    use super::*;
    use crate::tactic::constructor::*;
    #[test]
    fn test_constructor_ext_result_ok_3500() {
        let r = ConstructorExtResult3500::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_constructor_ext_result_err_3500() {
        let r = ConstructorExtResult3500::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_constructor_ext_result_partial_3500() {
        let r = ConstructorExtResult3500::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_constructor_ext_result_skipped_3500() {
        let r = ConstructorExtResult3500::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_constructor_ext_pass_run_3500() {
        let mut p = ConstructorExtPass3500::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_constructor_ext_pass_empty_3500() {
        let mut p = ConstructorExtPass3500::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_constructor_ext_pass_rate_3500() {
        let mut p = ConstructorExtPass3500::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_constructor_ext_pass_disable_3500() {
        let mut p = ConstructorExtPass3500::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_constructor_ext_pipeline_basic_3500() {
        let mut pipeline = ConstructorExtPipeline3500::new("main_pipeline");
        pipeline.add_pass(ConstructorExtPass3500::new("pass1"));
        pipeline.add_pass(ConstructorExtPass3500::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_constructor_ext_pipeline_disabled_3500() {
        let mut pipeline = ConstructorExtPipeline3500::new("partial");
        let mut p = ConstructorExtPass3500::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(ConstructorExtPass3500::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_constructor_ext_diff_basic_3500() {
        let mut d = ConstructorExtDiff3500::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_constructor_ext_config_set_get_3500() {
        let mut cfg = ConstructorExtConfig3500::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_constructor_ext_config_read_only_3500() {
        let mut cfg = ConstructorExtConfig3500::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_constructor_ext_config_remove_3500() {
        let mut cfg = ConstructorExtConfig3500::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_constructor_ext_diagnostics_basic_3500() {
        let mut diag = ConstructorExtDiag3500::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_constructor_ext_diagnostics_max_errors_3500() {
        let mut diag = ConstructorExtDiag3500::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_constructor_ext_diagnostics_clear_3500() {
        let mut diag = ConstructorExtDiag3500::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_constructor_ext_config_value_types_3500() {
        let b = ConstructorExtConfigVal3500::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = ConstructorExtConfigVal3500::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = ConstructorExtConfigVal3500::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = ConstructorExtConfigVal3500::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = ConstructorExtConfigVal3500::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
