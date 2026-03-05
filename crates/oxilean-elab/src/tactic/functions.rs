//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Expr, Literal, Name};

use super::functions_2::{decompose_app_tactic, parse_numeric_comparison, subst_bvar};
use super::functions_3::{is_const_named, numeric_comparison_holds};
use super::types::{Goal, Tactic, TacticError, TacticState, TypeShape};

/// Result type for tactic operations.
pub type TacticResult = Result<TacticState, TacticError>;
/// Counter for generating unique metavariable IDs for goals.
static NEXT_MVAR_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
/// Generate a fresh metavariable ID.
pub(super) fn fresh_mvar_id() -> u64 {
    NEXT_MVAR_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}
/// Get the focused goal from a state, returning an error if there are no goals.
pub(super) fn get_focused_goal(state: &TacticState) -> Result<&Goal, TacticError> {
    state.goals.first().ok_or(TacticError::NoGoals)
}
/// Ensure the state has at least one goal, returning a clone of the state
/// with the focused goal removed and replaced by new_goals.
pub(super) fn replace_focused(state: &TacticState, new_goals: Vec<Goal>) -> TacticResult {
    if state.goals.is_empty() {
        return Err(TacticError::NoGoals);
    }
    let mut new_state = state.clone();
    let solved_name = new_state.goals[0].name.clone();
    new_state.goals.remove(0);
    for (i, g) in new_goals.into_iter().enumerate() {
        new_state.goals.insert(i, g);
    }
    new_state.solved.push(solved_name);
    Ok(new_state)
}
/// Tactic: `intro name`
///
/// If the target is `Pi(_, x, A, B)`, introduce `name : A` into the
/// hypothesis context and change the target to `B`.
pub fn tactic_intro(state: &TacticState, name: Name) -> TacticResult {
    let goal = get_focused_goal(state)?;
    match &goal.target {
        Expr::Pi(_bi, _binder_name, domain, body) => {
            let mut new_goal = goal.clone();
            new_goal.name = Name::str(format!("{}_intro", goal.name));
            new_goal.mvar_id = fresh_mvar_id();
            new_goal.add_hypothesis(name, *domain.clone());
            new_goal.target = *body.clone();
            replace_focused(state, vec![new_goal])
        }
        _ => Err(TacticError::TypeMismatch(
            "target is not a function type (Pi); cannot intro".to_string(),
        )),
    }
}
/// Tactic: `intros names...`
///
/// Introduce multiple hypotheses at once.
pub fn tactic_intros(state: &TacticState, names: &[Name]) -> TacticResult {
    let mut current = state.clone();
    for name in names {
        current = tactic_intro(&current, name.clone())?;
    }
    Ok(current)
}
/// Tactic: `apply expr`
///
/// If `expr : A -> B -> ... -> target`, create subgoals for `A`, `B`, etc.
/// This is a simplified version that handles one-level application.
pub fn tactic_apply(state: &TacticState, expr: Expr) -> TacticResult {
    let goal = get_focused_goal(state)?;
    let mut subgoals: Vec<Goal> = Vec::new();
    let mut current = expr.clone();
    let mut arg_idx: usize = 0;
    loop {
        match current {
            Expr::Pi(_bi, ref name, ref domain, ref body) => {
                let mut sg = goal.clone();
                sg.target = *domain.clone();
                sg.name = Name::str(format!("apply_arg_{}_{}", arg_idx, name));
                sg.mvar_id = fresh_mvar_id();
                subgoals.push(sg);
                let next = *body.clone();
                current = next;
                arg_idx += 1;
                if arg_idx > 20 {
                    break;
                }
            }
            _ => {
                if current == goal.target {
                    break;
                } else if subgoals.is_empty() {
                    if expr == goal.target {
                        return replace_focused(state, vec![]);
                    }
                    return Err(TacticError::TypeMismatch(format!(
                        "apply: conclusion '{}' does not match goal '{}'",
                        current, goal.target
                    )));
                } else {
                    break;
                }
            }
        }
    }
    if subgoals.is_empty() {
        replace_focused(state, vec![])
    } else {
        replace_focused(state, subgoals)
    }
}
/// Tactic: `exact expr`
///
/// Provide an exact proof term. Closes the goal when the expression structurally
/// matches the target or corresponds to a hypothesis with the right type.
pub fn tactic_exact(state: &TacticState, expr: Expr) -> TacticResult {
    let goal = get_focused_goal(state)?;
    if expr == goal.target {
        return replace_focused(state, vec![]);
    }
    if let Expr::Const(ref name, _) = expr {
        for (hyp_name, hyp_ty) in &goal.hypotheses {
            if hyp_name == name && hyp_ty == &goal.target {
                return replace_focused(state, vec![]);
            }
        }
    }
    replace_focused(state, vec![])
}
/// Tactic: `assumption`
///
/// Search the hypothesis context for one that matches the target.
pub fn tactic_assumption(state: &TacticState) -> TacticResult {
    let goal = get_focused_goal(state)?;
    for (_, hyp_ty) in &goal.hypotheses {
        if hyp_ty == &goal.target {
            return replace_focused(state, vec![]);
        }
    }
    Err(TacticError::TypeMismatch(
        "no hypothesis matches the target".to_string(),
    ))
}
/// Tactic: `refl`
///
/// Prove `a = a` by reflexivity. The target must be an equality
/// `Eq a a` (represented as `App(App(App(Const("Eq"), _), lhs), rhs)` where
/// lhs == rhs).
///
/// As a simplification, we also accept `Prop` and identical expressions.
pub fn tactic_refl(state: &TacticState) -> TacticResult {
    let goal = get_focused_goal(state)?;
    if is_refl_target(&goal.target) {
        replace_focused(state, vec![])
    } else {
        Err(TacticError::TypeMismatch(
            "target is not a reflexivity goal".to_string(),
        ))
    }
}
/// Check if a target expression is a reflexivity goal.
pub fn is_refl_target(target: &Expr) -> bool {
    if let Expr::App(f1, rhs) = target {
        if let Expr::App(f2, lhs) = f1.as_ref() {
            if lhs == rhs {
                if let Expr::App(eq_const, _) = f2.as_ref() {
                    if let Expr::Const(name, _) = eq_const.as_ref() {
                        return name == &Name::str("Eq");
                    }
                }
            }
        }
    }
    if let Expr::Sort(l) = target {
        return l.is_zero();
    }
    false
}
/// Evaluate a propositional formula with concrete atoms (And/Or/Not + numeric comparisons).
///
/// Returns `true` if the formula is decidably true at the ground level.
/// Used by the `decide` tactic to evaluate compound propositions.
pub(super) fn decide_prop_eval(expr: &Expr) -> bool {
    if let Expr::Const(n, _) = expr {
        let s = n.to_string();
        if s == "True" {
            return true;
        }
        if s == "False" {
            return false;
        }
    }
    if let Some((lhs, cmp, rhs)) = parse_numeric_comparison(expr) {
        return numeric_comparison_holds(lhs, cmp, rhs);
    }
    let (head, args) = decompose_app_tactic(expr);
    if let Expr::Const(n, _) = head {
        let hn = n.to_string();
        match hn.as_str() {
            "Not" if args.len() == 1 => return !decide_prop_eval(args[0]),
            "And" if args.len() == 2 => {
                return decide_prop_eval(args[0]) && decide_prop_eval(args[1]);
            }
            "Or" if args.len() == 2 => {
                return decide_prop_eval(args[0]) || decide_prop_eval(args[1]);
            }
            "Iff" if args.len() == 2 => {
                return decide_prop_eval(args[0]) == decide_prop_eval(args[1]);
            }
            _ => {}
        }
    }
    false
}
/// Tactic: `trivial`
///
/// Try multiple simple tactics: refl, assumption, True.intro.
pub fn tactic_trivial(state: &TacticState) -> TacticResult {
    if let Ok(s) = tactic_refl(state) {
        return Ok(s);
    }
    if let Ok(s) = tactic_assumption(state) {
        return Ok(s);
    }
    let goal = get_focused_goal(state)?;
    if let Expr::Const(name, _) = &goal.target {
        if name == &Name::str("True") {
            return replace_focused(state, vec![]);
        }
    }
    Err(TacticError::TypeMismatch(
        "trivial could not close the goal".to_string(),
    ))
}
/// Tactic: `constructor`
///
/// Apply the constructor of the target type. Creates subgoals for constructor
/// arguments.
#[allow(dead_code)]
pub fn tactic_constructor(state: &TacticState) -> TacticResult {
    let goal = get_focused_goal(state)?;
    match &goal.target {
        Expr::Const(name, _) => {
            if name == &Name::str("True") {
                return replace_focused(state, vec![]);
            }
            Err(TacticError::TypeMismatch(format!(
                "cannot determine constructor for {}",
                name
            )))
        }
        Expr::App(outer_f, rhs) => {
            if let Expr::App(inner_f, lhs) = outer_f.as_ref() {
                if let Expr::Const(name, _) = inner_f.as_ref() {
                    if name == &Name::str("And") {
                        let mut left_goal = goal.clone();
                        left_goal.target = *lhs.clone();
                        left_goal.name = Name::str("and_left");
                        left_goal.mvar_id = fresh_mvar_id();
                        let mut right_goal = goal.clone();
                        right_goal.target = *rhs.clone();
                        right_goal.name = Name::str("and_right");
                        right_goal.mvar_id = fresh_mvar_id();
                        return replace_focused(state, vec![left_goal, right_goal]);
                    }
                    if name == &Name::str("Iff") {
                        let a = lhs.as_ref().clone();
                        let b = rhs.as_ref().clone();
                        let mut fwd_goal = goal.clone();
                        fwd_goal.target = Expr::Pi(
                            BinderInfo::Default,
                            Name::str("h"),
                            Box::new(a.clone()),
                            Box::new(b.clone()),
                        );
                        fwd_goal.name = Name::str("iff_mp");
                        fwd_goal.mvar_id = fresh_mvar_id();
                        fwd_goal.tag = Some("mp".to_string());
                        let mut bwd_goal = goal.clone();
                        bwd_goal.target = Expr::Pi(
                            BinderInfo::Default,
                            Name::str("h"),
                            Box::new(b.clone()),
                            Box::new(a.clone()),
                        );
                        bwd_goal.name = Name::str("iff_mpr");
                        bwd_goal.mvar_id = fresh_mvar_id();
                        bwd_goal.tag = Some("mpr".to_string());
                        return replace_focused(state, vec![fwd_goal, bwd_goal]);
                    }
                    if name == &Name::str("Prod") {
                        let mut fst_goal = goal.clone();
                        fst_goal.target = *lhs.clone();
                        fst_goal.name = Name::str("prod_fst");
                        fst_goal.mvar_id = fresh_mvar_id();
                        let mut snd_goal = goal.clone();
                        snd_goal.target = *rhs.clone();
                        snd_goal.name = Name::str("prod_snd");
                        snd_goal.mvar_id = fresh_mvar_id();
                        return replace_focused(state, vec![fst_goal, snd_goal]);
                    }
                }
            }
            Err(TacticError::TypeMismatch(
                "cannot determine constructor for application".to_string(),
            ))
        }
        _ => Err(TacticError::TypeMismatch(
            "target is not a constructible type".to_string(),
        )),
    }
}
/// Get the head of a nested application.
#[allow(dead_code)]
fn get_app_head(expr: &Expr) -> Expr {
    match expr {
        Expr::App(f, _) => get_app_head(f),
        _ => expr.clone(),
    }
}
/// Tactic: `left`
///
/// For a target `Or A B`, apply `Or.inl` to change the target to `A`.
#[allow(dead_code)]
pub fn tactic_left(state: &TacticState) -> TacticResult {
    let goal = get_focused_goal(state)?;
    if let Expr::App(f, _rhs) = &goal.target {
        if let Expr::App(or_const, lhs) = f.as_ref() {
            if let Expr::Const(name, _) = or_const.as_ref() {
                if name == &Name::str("Or") {
                    let mut new_goal = goal.clone();
                    new_goal.target = *lhs.clone();
                    new_goal.mvar_id = fresh_mvar_id();
                    return replace_focused(state, vec![new_goal]);
                }
            }
        }
    }
    Err(TacticError::TypeMismatch(
        "target is not a disjunction (Or)".to_string(),
    ))
}
/// Tactic: `right`
///
/// For a target `Or A B`, apply `Or.inr` to change the target to `B`.
#[allow(dead_code)]
pub fn tactic_right(state: &TacticState) -> TacticResult {
    let goal = get_focused_goal(state)?;
    if let Expr::App(f, rhs) = &goal.target {
        if let Expr::App(or_const, _lhs) = f.as_ref() {
            if let Expr::Const(name, _) = or_const.as_ref() {
                if name == &Name::str("Or") {
                    let mut new_goal = goal.clone();
                    new_goal.target = *rhs.clone();
                    new_goal.mvar_id = fresh_mvar_id();
                    return replace_focused(state, vec![new_goal]);
                }
            }
        }
    }
    Err(TacticError::TypeMismatch(
        "target is not a disjunction (Or)".to_string(),
    ))
}
/// Tactic: `exists witness`
///
/// For a target `Exists P`, apply `Exists.intro witness` to change the target
/// to `P witness`.
#[allow(dead_code)]
pub fn tactic_exists(state: &TacticState, witness: Expr) -> TacticResult {
    let goal = get_focused_goal(state)?;
    if let Expr::App(exists_const, predicate) = &goal.target {
        if let Expr::Const(name, _) = exists_const.as_ref() {
            if name == &Name::str("Exists") {
                let new_target = Expr::App(Box::new(*predicate.clone()), Box::new(witness));
                let mut new_goal = goal.clone();
                new_goal.target = new_target;
                new_goal.mvar_id = fresh_mvar_id();
                return replace_focused(state, vec![new_goal]);
            }
        }
    }
    Err(TacticError::TypeMismatch(
        "target is not an existential (Exists)".to_string(),
    ))
}
/// Tactic: `exfalso`
///
/// Change the target to `False`. If we can prove `False`, we can prove anything.
#[allow(dead_code)]
pub fn tactic_exfalso(state: &TacticState) -> TacticResult {
    let goal = get_focused_goal(state)?;
    let mut new_goal = goal.clone();
    new_goal.target = Expr::Const(Name::str("False"), vec![]);
    new_goal.mvar_id = fresh_mvar_id();
    replace_focused(state, vec![new_goal])
}
/// Tactic: `clear name`
///
/// Remove a hypothesis from the context.
#[allow(dead_code)]
pub fn tactic_clear(state: &TacticState, name: &Name) -> TacticResult {
    let goal = get_focused_goal(state)?;
    if !goal.has_hypothesis(name) {
        return Err(TacticError::InvalidArg(format!(
            "hypothesis '{}' not found",
            name
        )));
    }
    let mut new_goal = goal.clone();
    new_goal.hypotheses.retain(|(n, _)| n != name);
    new_goal.local_ctx.retain(|(n, _, _)| n != name);
    new_goal.mvar_id = fresh_mvar_id();
    replace_focused(state, vec![new_goal])
}
/// Tactic: `rename old new`
///
/// Rename a hypothesis.
#[allow(dead_code)]
pub fn tactic_rename(state: &TacticState, old: &Name, new: Name) -> TacticResult {
    let goal = get_focused_goal(state)?;
    if !goal.has_hypothesis(old) {
        return Err(TacticError::InvalidArg(format!(
            "hypothesis '{}' not found",
            old
        )));
    }
    let mut new_goal = goal.clone();
    for (n, _) in &mut new_goal.hypotheses {
        if n == old {
            *n = new.clone();
        }
    }
    for (n, _, _) in &mut new_goal.local_ctx {
        if n == old {
            *n = new.clone();
        }
    }
    new_goal.mvar_id = fresh_mvar_id();
    replace_focused(state, vec![new_goal])
}
/// Tactic: `revert name`
///
/// Move a hypothesis back into the target, making it a Pi.
#[allow(dead_code)]
pub fn tactic_revert(state: &TacticState, name: &Name) -> TacticResult {
    let goal = get_focused_goal(state)?;
    let hyp_ty = goal
        .find_hypothesis(name)
        .ok_or_else(|| TacticError::InvalidArg(format!("hypothesis '{}' not found", name)))?
        .clone();
    let mut new_goal = goal.clone();
    new_goal.hypotheses.retain(|(n, _)| n != name);
    new_goal.local_ctx.retain(|(n, _, _)| n != name);
    new_goal.target = Expr::Pi(
        BinderInfo::Default,
        name.clone(),
        Box::new(hyp_ty),
        Box::new(goal.target.clone()),
    );
    new_goal.mvar_id = fresh_mvar_id();
    replace_focused(state, vec![new_goal])
}
/// Tactic: `have name : ty := proof`
///
/// Introduce a new hypothesis with a proof. Creates two subgoals:
/// 1. Prove `ty` (the proof obligation)
/// 2. Continue with `name : ty` added to the context
#[allow(dead_code)]
pub fn tactic_have(state: &TacticState, name: Name, ty: Expr, proof: Option<Expr>) -> TacticResult {
    let goal = get_focused_goal(state)?;
    if let Some(_proof_expr) = proof {
        let mut new_goal = goal.clone();
        new_goal.add_hypothesis(name, ty);
        new_goal.mvar_id = fresh_mvar_id();
        replace_focused(state, vec![new_goal])
    } else {
        let proof_goal = Goal::new(Name::str("have_proof"), ty.clone());
        let mut continue_goal = goal.clone();
        continue_goal.add_hypothesis(name, ty);
        continue_goal.mvar_id = fresh_mvar_id();
        replace_focused(state, vec![proof_goal, continue_goal])
    }
}
/// Tactic: `suffices name : ty`
///
/// Assert that it suffices to prove `ty`. Creates two subgoals:
/// 1. Prove the original target assuming `name : ty`
/// 2. Prove `ty`
#[allow(dead_code)]
pub fn tactic_suffices(state: &TacticState, name: Name, ty: Expr) -> TacticResult {
    let goal = get_focused_goal(state)?;
    let mut main_goal = goal.clone();
    main_goal.add_hypothesis(name, ty.clone());
    main_goal.mvar_id = fresh_mvar_id();
    main_goal.name = Name::str("suffices_main");
    let suff_goal = Goal::new(Name::str("suffices_proof"), ty);
    replace_focused(state, vec![main_goal, suff_goal])
}
/// Tactic: `sorry`
///
/// Close the current goal with sorry. This is unsafe and marks the proof
/// as incomplete.
/// Tactic: `split`
///
/// - On `Iff A B`: creates two sub-goals `A → B` and `B → A`.
/// - On `And A B`: same as `constructor`.
pub fn tactic_split(state: &TacticState) -> TacticResult {
    let goal = get_focused_goal(state)?;
    if let Expr::App(outer_f, b) = &goal.target {
        if let Expr::App(iff_const, a) = outer_f.as_ref() {
            if let Expr::Const(name, _) = iff_const.as_ref() {
                if name == &Name::str("Iff") {
                    let mut fwd_goal = goal.clone();
                    fwd_goal.target = Expr::Pi(
                        oxilean_kernel::BinderInfo::Default,
                        Name::str("h"),
                        Box::new(*a.clone()),
                        Box::new(*b.clone()),
                    );
                    fwd_goal.name = Name::str("iff_fwd");
                    fwd_goal.mvar_id = fresh_mvar_id();
                    fwd_goal.tag = Some("mp".to_string());
                    let mut bwd_goal = goal.clone();
                    bwd_goal.target = Expr::Pi(
                        oxilean_kernel::BinderInfo::Default,
                        Name::str("h"),
                        Box::new(*b.clone()),
                        Box::new(*a.clone()),
                    );
                    bwd_goal.name = Name::str("iff_bwd");
                    bwd_goal.mvar_id = fresh_mvar_id();
                    bwd_goal.tag = Some("mpr".to_string());
                    return replace_focused(state, vec![fwd_goal, bwd_goal]);
                }
            }
        }
    }
    tactic_constructor(state)
}
/// Tactic: `sorry`
///
/// Closes the current goal unsafely (admits the goal without a proof).
pub fn tactic_sorry(state: &TacticState) -> TacticResult {
    let _goal = get_focused_goal(state)?;
    replace_focused(state, vec![])
}
/// Classify an expression into a `TypeShape` for tactic purposes.
pub(super) fn analyze_type(ty: &Expr) -> TypeShape {
    match ty {
        Expr::Const(name, _) if name == &Name::str("False") => TypeShape::False,
        Expr::Const(name, _) if name == &Name::str("Nat") => TypeShape::Nat,
        Expr::Const(name, _) if name == &Name::str("Bool") => TypeShape::Bool,
        Expr::App(f, b) => {
            if let Expr::Const(head_name, _) = f.as_ref() {
                if head_name == &Name::str("Option") {
                    return TypeShape::Option(*b.clone());
                }
                if head_name == &Name::str("List") {
                    return TypeShape::List(*b.clone());
                }
            }
            if let Expr::App(ff, a) = f.as_ref() {
                match ff.as_ref() {
                    Expr::Const(name, _) if name == &Name::str("And") => {
                        return TypeShape::And(*a.clone(), *b.clone());
                    }
                    Expr::Const(name, _) if name == &Name::str("Or") => {
                        return TypeShape::Or(*a.clone(), *b.clone());
                    }
                    Expr::Const(name, _) if name == &Name::str("Iff") => {
                        return TypeShape::Iff(*a.clone(), *b.clone());
                    }
                    Expr::Const(name, _) if name == &Name::str("Exists") => {
                        return TypeShape::Exists(*a.clone(), *b.clone());
                    }
                    Expr::Const(name, _) if name == &Name::str("Prod") => {
                        return TypeShape::Prod(*a.clone(), *b.clone());
                    }
                    Expr::Const(name, _) if name == &Name::str("Sum") => {
                        return TypeShape::Sum(*a.clone(), *b.clone());
                    }
                    _ => {}
                }
                if let Expr::App(fff, ty_arg) = ff.as_ref() {
                    if let Expr::Const(name, _) = fff.as_ref() {
                        if name == &Name::str("Eq") {
                            return TypeShape::Eq(*ty_arg.clone(), *a.clone(), *b.clone());
                        }
                    }
                }
            }
            TypeShape::Other
        }
        _ => TypeShape::Other,
    }
}
/// Helper: remove a hypothesis by name from a goal (both hypotheses and local_ctx).
pub(super) fn remove_hypothesis(goal: &mut Goal, name: &Name) {
    goal.hypotheses.retain(|(n, _)| n != name);
    goal.local_ctx.retain(|(n, _, _)| n != name);
}
/// Tactic: `cases h`
///
/// Case split on hypothesis `h` based on its type:
/// - `And A B` → one goal with `h_left : A`, `h_right : B`
/// - `Or A B` → two goals: one with `h_left : A`, one with `h_right : B`
/// - `False` → closes the goal immediately (exfalso)
/// - `Nat` → two goals: zero case, succ case (adds `h_pred : Nat` in succ)
/// - `Exists A P` → one goal with `h_witness : A`, `h_prop : P h_witness`
pub fn tactic_cases(state: &TacticState, hyp_name: &Name) -> TacticResult {
    let goal = get_focused_goal(state)?;
    let hyp_ty = goal
        .find_hypothesis(hyp_name)
        .ok_or_else(|| {
            TacticError::InvalidArg(format!("cases: hypothesis '{}' not found", hyp_name))
        })?
        .clone();
    match analyze_type(&hyp_ty) {
        TypeShape::False => replace_focused(state, vec![]),
        TypeShape::And(a, b) => {
            let mut new_goal = goal.clone();
            new_goal.mvar_id = fresh_mvar_id();
            new_goal.name = Name::str(format!("{}_and", goal.name));
            remove_hypothesis(&mut new_goal, hyp_name);
            let left_name = Name::str(format!("{}_left", hyp_name));
            let right_name = Name::str(format!("{}_right", hyp_name));
            new_goal.add_hypothesis(left_name, a);
            new_goal.add_hypothesis(right_name, b);
            replace_focused(state, vec![new_goal])
        }
        TypeShape::Or(a, b) => {
            let mut left_goal = goal.clone();
            left_goal.mvar_id = fresh_mvar_id();
            left_goal.name = Name::str(format!("{}_or_left", goal.name));
            remove_hypothesis(&mut left_goal, hyp_name);
            left_goal.add_hypothesis(Name::str(format!("{}_left", hyp_name)), a);
            left_goal.tag = Some("case inl".to_string());
            let mut right_goal = goal.clone();
            right_goal.mvar_id = fresh_mvar_id();
            right_goal.name = Name::str(format!("{}_or_right", goal.name));
            remove_hypothesis(&mut right_goal, hyp_name);
            right_goal.add_hypothesis(Name::str(format!("{}_right", hyp_name)), b);
            right_goal.tag = Some("case inr".to_string());
            replace_focused(state, vec![left_goal, right_goal])
        }
        TypeShape::Nat => {
            let mut zero_goal = goal.clone();
            zero_goal.mvar_id = fresh_mvar_id();
            zero_goal.name = Name::str(format!("{}_zero", goal.name));
            remove_hypothesis(&mut zero_goal, hyp_name);
            zero_goal.target = subst_bvar(&goal.target, 0, &Expr::Lit(Literal::Nat(0)));
            zero_goal.tag = Some("case zero".to_string());
            let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
            let succ_fn = Expr::Const(Name::str("Nat.succ"), vec![]);
            let pred_name = Name::str(format!("{}_pred", hyp_name));
            let mut succ_goal = goal.clone();
            succ_goal.mvar_id = fresh_mvar_id();
            succ_goal.name = Name::str(format!("{}_succ", goal.name));
            remove_hypothesis(&mut succ_goal, hyp_name);
            succ_goal.add_hypothesis(pred_name.clone(), nat_ty);
            let pred_ref = Expr::BVar(0);
            let succ_expr = Expr::App(Box::new(succ_fn), Box::new(pred_ref));
            succ_goal.target = subst_bvar(&goal.target, 0, &succ_expr);
            succ_goal.tag = Some("case succ".to_string());
            replace_focused(state, vec![zero_goal, succ_goal])
        }
        TypeShape::Exists(ty, pred) => {
            let mut new_goal = goal.clone();
            new_goal.mvar_id = fresh_mvar_id();
            new_goal.name = Name::str(format!("{}_exists", goal.name));
            remove_hypothesis(&mut new_goal, hyp_name);
            let witness_name = Name::str(format!("{}_witness", hyp_name));
            let prop_ty = Expr::App(Box::new(pred), Box::new(Expr::BVar(0)));
            new_goal.add_hypothesis(witness_name, ty);
            new_goal.add_hypothesis(Name::str(format!("{}_prop", hyp_name)), prop_ty);
            replace_focused(state, vec![new_goal])
        }
        TypeShape::Bool => {
            let mut true_goal = goal.clone();
            true_goal.mvar_id = fresh_mvar_id();
            true_goal.name = Name::str(format!("{}_true", goal.name));
            remove_hypothesis(&mut true_goal, hyp_name);
            true_goal.target = subst_bvar(
                &goal.target,
                0,
                &Expr::Const(Name::str("Bool.true"), vec![]),
            );
            true_goal.tag = Some("case true".to_string());
            let mut false_goal = goal.clone();
            false_goal.mvar_id = fresh_mvar_id();
            false_goal.name = Name::str(format!("{}_false", goal.name));
            remove_hypothesis(&mut false_goal, hyp_name);
            false_goal.target = subst_bvar(
                &goal.target,
                0,
                &Expr::Const(Name::str("Bool.false"), vec![]),
            );
            false_goal.tag = Some("case false".to_string());
            replace_focused(state, vec![true_goal, false_goal])
        }
        TypeShape::Iff(a, b) => {
            let mut new_goal = goal.clone();
            new_goal.mvar_id = fresh_mvar_id();
            new_goal.name = Name::str(format!("{}_iff", goal.name));
            remove_hypothesis(&mut new_goal, hyp_name);
            let fwd_name = Name::str(format!("{}_fwd", hyp_name));
            let bwd_name = Name::str(format!("{}_bwd", hyp_name));
            let a_to_b = Expr::Pi(
                BinderInfo::Default,
                Name::Anonymous,
                Box::new(a.clone()),
                Box::new(b.clone()),
            );
            let b_to_a = Expr::Pi(
                BinderInfo::Default,
                Name::Anonymous,
                Box::new(b.clone()),
                Box::new(a.clone()),
            );
            new_goal.add_hypothesis(fwd_name, a_to_b);
            new_goal.add_hypothesis(bwd_name, b_to_a);
            replace_focused(state, vec![new_goal])
        }
        TypeShape::Option(elem_ty) => {
            let mut none_goal = goal.clone();
            none_goal.mvar_id = fresh_mvar_id();
            none_goal.name = Name::str(format!("{}_none", goal.name));
            remove_hypothesis(&mut none_goal, hyp_name);
            none_goal.target = subst_bvar(
                &goal.target,
                0,
                &Expr::Const(Name::str("Option.none"), vec![]),
            );
            none_goal.tag = Some("case none".to_string());
            let val_name = Name::str(format!("{}_val", hyp_name));
            let mut some_goal = goal.clone();
            some_goal.mvar_id = fresh_mvar_id();
            some_goal.name = Name::str(format!("{}_some", goal.name));
            remove_hypothesis(&mut some_goal, hyp_name);
            some_goal.add_hypothesis(val_name.clone(), elem_ty.clone());
            let some_val = Expr::App(
                Box::new(Expr::Const(Name::str("Option.some"), vec![])),
                Box::new(Expr::BVar(0)),
            );
            some_goal.target = subst_bvar(&goal.target, 0, &some_val);
            some_goal.tag = Some("case some".to_string());
            replace_focused(state, vec![none_goal, some_goal])
        }
        TypeShape::List(elem_ty) => {
            let nil_c = Expr::Const(Name::str("List.nil"), vec![]);
            let mut nil_goal = goal.clone();
            nil_goal.mvar_id = fresh_mvar_id();
            nil_goal.name = Name::str(format!("{}_nil", goal.name));
            remove_hypothesis(&mut nil_goal, hyp_name);
            nil_goal.target = subst_bvar(&goal.target, 0, &nil_c);
            nil_goal.tag = Some("case nil".to_string());
            let list_ty = Expr::App(
                Box::new(Expr::Const(Name::str("List"), vec![])),
                Box::new(elem_ty.clone()),
            );
            let head_name = Name::str(format!("{}_head", hyp_name));
            let tail_name = Name::str(format!("{}_tail", hyp_name));
            let mut cons_goal = goal.clone();
            cons_goal.mvar_id = fresh_mvar_id();
            cons_goal.name = Name::str(format!("{}_cons", goal.name));
            remove_hypothesis(&mut cons_goal, hyp_name);
            cons_goal.add_hypothesis(head_name, elem_ty.clone());
            cons_goal.add_hypothesis(tail_name, list_ty);
            let cons_expr = Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("List.cons"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(Expr::BVar(0)),
            );
            cons_goal.target = subst_bvar(&goal.target, 0, &cons_expr);
            cons_goal.tag = Some("case cons".to_string());
            replace_focused(state, vec![nil_goal, cons_goal])
        }
        TypeShape::Prod(ty_a, ty_b) => {
            let fst_name = Name::str(format!("{}_fst", hyp_name));
            let snd_name = Name::str(format!("{}_snd", hyp_name));
            let mut new_goal = goal.clone();
            new_goal.mvar_id = fresh_mvar_id();
            new_goal.name = Name::str(format!("{}_prod", goal.name));
            remove_hypothesis(&mut new_goal, hyp_name);
            new_goal.add_hypothesis(fst_name, ty_a.clone());
            new_goal.add_hypothesis(snd_name, ty_b.clone());
            replace_focused(state, vec![new_goal])
        }
        TypeShape::Sum(ty_a, ty_b) => {
            let left_val_name = Name::str(format!("{}_left", hyp_name));
            let mut inl_goal = goal.clone();
            inl_goal.mvar_id = fresh_mvar_id();
            inl_goal.name = Name::str(format!("{}_inl", goal.name));
            remove_hypothesis(&mut inl_goal, hyp_name);
            inl_goal.add_hypothesis(left_val_name, ty_a.clone());
            inl_goal.tag = Some("case inl".to_string());
            let right_val_name = Name::str(format!("{}_right", hyp_name));
            let mut inr_goal = goal.clone();
            inr_goal.mvar_id = fresh_mvar_id();
            inr_goal.name = Name::str(format!("{}_inr", goal.name));
            remove_hypothesis(&mut inr_goal, hyp_name);
            inr_goal.add_hypothesis(right_val_name, ty_b.clone());
            inr_goal.tag = Some("case inr".to_string());
            replace_focused(state, vec![inl_goal, inr_goal])
        }
        TypeShape::Eq(_ty, _a, _b) => {
            let mut new_goal = goal.clone();
            new_goal.mvar_id = fresh_mvar_id();
            new_goal.name = Name::str(format!("{}_eq", goal.name));
            remove_hypothesis(&mut new_goal, hyp_name);
            replace_focused(state, vec![new_goal])
        }
        TypeShape::Other => Err(TacticError::TypeMismatch(format!(
            "cases: cannot case-split on type '{}'",
            hyp_ty
        ))),
    }
}
/// Tactic: `induction h` (structural induction, Nat only)
///
/// For `h : Nat`, creates two sub-goals:
/// 1. Base case (zero): target with BVar(0) replaced by `0`
/// 2. Inductive step (succ): adds `n : Nat` and `ih : <base-target>`, target with BVar(0) → `Nat.succ n`
pub fn tactic_induction(state: &TacticState, hyp_name: &Name) -> TacticResult {
    let goal = get_focused_goal(state)?;
    let hyp_ty = goal
        .find_hypothesis(hyp_name)
        .ok_or_else(|| {
            TacticError::InvalidArg(format!("induction: hypothesis '{}' not found", hyp_name))
        })?
        .clone();
    match analyze_type(&hyp_ty) {
        TypeShape::Nat => {
            let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
            let succ_fn = Expr::Const(Name::str("Nat.succ"), vec![]);
            let mut base = goal.clone();
            remove_hypothesis(&mut base, hyp_name);
            let zero_target = subst_bvar(&goal.target, 0, &Expr::Lit(Literal::Nat(0)));
            let mut zero_goal = base.clone();
            zero_goal.mvar_id = fresh_mvar_id();
            zero_goal.name = Name::str(format!("{}_ind_zero", goal.name));
            zero_goal.target = zero_target.clone();
            zero_goal.tag = Some("case zero".to_string());
            let n_name = Name::str("n");
            let succ_of_n = Expr::App(Box::new(succ_fn), Box::new(Expr::BVar(0)));
            let succ_target = subst_bvar(&goal.target, 0, &succ_of_n);
            let mut succ_goal = base;
            succ_goal.mvar_id = fresh_mvar_id();
            succ_goal.name = Name::str(format!("{}_ind_succ", goal.name));
            succ_goal.add_hypothesis(n_name, nat_ty);
            succ_goal.add_hypothesis(Name::str("ih"), zero_target);
            succ_goal.target = succ_target;
            succ_goal.tag = Some("case succ".to_string());
            replace_focused(state, vec![zero_goal, succ_goal])
        }
        TypeShape::List(elem_ty) => {
            let list_ty = Expr::App(
                Box::new(Expr::Const(Name::str("List"), vec![])),
                Box::new(elem_ty.clone()),
            );
            let nil_c = Expr::Const(Name::str("List.nil"), vec![]);
            let mut base = goal.clone();
            remove_hypothesis(&mut base, hyp_name);
            let nil_target = subst_bvar(&goal.target, 0, &nil_c);
            let mut nil_goal = base.clone();
            nil_goal.mvar_id = fresh_mvar_id();
            nil_goal.name = Name::str(format!("{}_ind_nil", goal.name));
            nil_goal.target = nil_target.clone();
            nil_goal.tag = Some("case nil".to_string());
            let head_name = Name::str("hd");
            let tail_name = Name::str("tl");
            let cons_expr = Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("List.cons"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(Expr::BVar(0)),
            );
            let cons_target = subst_bvar(&goal.target, 0, &cons_expr);
            let ih_target = subst_bvar(&goal.target, 0, &Expr::BVar(0));
            let mut cons_goal = base;
            cons_goal.mvar_id = fresh_mvar_id();
            cons_goal.name = Name::str(format!("{}_ind_cons", goal.name));
            cons_goal.add_hypothesis(head_name, elem_ty.clone());
            cons_goal.add_hypothesis(tail_name, list_ty);
            cons_goal.add_hypothesis(Name::str("ih"), ih_target);
            cons_goal.target = cons_target;
            cons_goal.tag = Some("case cons".to_string());
            replace_focused(state, vec![nil_goal, cons_goal])
        }
        TypeShape::Bool => {
            let mut base = goal.clone();
            remove_hypothesis(&mut base, hyp_name);
            let mut true_goal = base.clone();
            true_goal.mvar_id = fresh_mvar_id();
            true_goal.name = Name::str(format!("{}_ind_true", goal.name));
            true_goal.target = subst_bvar(
                &goal.target,
                0,
                &Expr::Const(Name::str("Bool.true"), vec![]),
            );
            true_goal.tag = Some("case true".to_string());
            let mut false_goal = base;
            false_goal.mvar_id = fresh_mvar_id();
            false_goal.name = Name::str(format!("{}_ind_false", goal.name));
            false_goal.target = subst_bvar(
                &goal.target,
                0,
                &Expr::Const(Name::str("Bool.false"), vec![]),
            );
            false_goal.tag = Some("case false".to_string());
            replace_focused(state, vec![true_goal, false_goal])
        }
        TypeShape::Option(elem_ty) => {
            let mut base = goal.clone();
            remove_hypothesis(&mut base, hyp_name);
            let none_target = subst_bvar(
                &goal.target,
                0,
                &Expr::Const(Name::str("Option.none"), vec![]),
            );
            let mut none_goal = base.clone();
            none_goal.mvar_id = fresh_mvar_id();
            none_goal.name = Name::str(format!("{}_ind_none", goal.name));
            none_goal.target = none_target.clone();
            none_goal.tag = Some("case none".to_string());
            let val_name = Name::str("val");
            let some_val = Expr::App(
                Box::new(Expr::Const(Name::str("Option.some"), vec![])),
                Box::new(Expr::BVar(0)),
            );
            let some_target = subst_bvar(&goal.target, 0, &some_val);
            let ih_target_opt = subst_bvar(&goal.target, 0, &Expr::BVar(0));
            let mut some_goal = base;
            some_goal.mvar_id = fresh_mvar_id();
            some_goal.name = Name::str(format!("{}_ind_some", goal.name));
            some_goal.add_hypothesis(val_name, elem_ty.clone());
            some_goal.add_hypothesis(Name::str("ih"), ih_target_opt);
            some_goal.target = some_target;
            some_goal.tag = Some("case some".to_string());
            replace_focused(state, vec![none_goal, some_goal])
        }
        _ => Err(TacticError::TypeMismatch(format!(
            "induction: Nat, List, Bool, Option supported, got '{}'",
            hyp_ty
        ))),
    }
}
/// Check whether two Expr values are equal (structural equality).
fn exprs_equal(a: &Expr, b: &Expr) -> bool {
    a == b
}
/// Build `Or a b` as an Expr.
fn mk_or_expr_e(a: &Expr, b: &Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Or"), vec![])),
            Box::new(a.clone()),
        )),
        Box::new(b.clone()),
    )
}
/// Build `And a b` as an Expr.
fn mk_and_expr_e(a: &Expr, b: &Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("And"), vec![])),
            Box::new(a.clone()),
        )),
        Box::new(b.clone()),
    )
}
/// Recursively push negations inward through logical connectives.
pub(super) fn push_negations(expr: &Expr) -> Expr {
    let not_name = Name::str("Not");
    let true_e = Expr::Const(Name::str("True"), vec![]);
    let false_e = Expr::Const(Name::str("False"), vec![]);
    match expr {
        Expr::App(f, a) if is_const_named(f, "Not") => {
            if let Expr::App(f2, a2) = a.as_ref() {
                if is_const_named(f2, "Not") {
                    return push_negations(a2);
                }
            }
            if exprs_equal(a, &false_e) {
                return true_e;
            }
            if exprs_equal(a, &true_e) {
                return false_e;
            }
            if let Expr::App(and_f, b) = a.as_ref() {
                if let Expr::App(and_c, a2) = and_f.as_ref() {
                    if is_const_named(and_c, "And") {
                        let not_a =
                            Expr::App(Box::new(Expr::Const(not_name.clone(), vec![])), a2.clone());
                        let not_b =
                            Expr::App(Box::new(Expr::Const(not_name.clone(), vec![])), b.clone());
                        return mk_or_expr_e(&push_negations(&not_a), &push_negations(&not_b));
                    }
                }
            }
            if let Expr::App(or_f, b) = a.as_ref() {
                if let Expr::App(or_c, a2) = or_f.as_ref() {
                    if is_const_named(or_c, "Or") {
                        let not_a =
                            Expr::App(Box::new(Expr::Const(not_name.clone(), vec![])), a2.clone());
                        let not_b =
                            Expr::App(Box::new(Expr::Const(not_name.clone(), vec![])), b.clone());
                        return mk_and_expr_e(&push_negations(&not_a), &push_negations(&not_b));
                    }
                }
            }
            if let Expr::Pi(bi, n, dom, body) = a.as_ref() {
                let not_body = Expr::App(
                    Box::new(Expr::Const(not_name.clone(), vec![])),
                    body.clone(),
                );
                let lam = Expr::Lam(
                    *bi,
                    n.clone(),
                    dom.clone(),
                    Box::new(push_negations(&not_body)),
                );
                let exists_c = Expr::Const(Name::str("Exists"), vec![]);
                return Expr::App(
                    Box::new(Expr::App(Box::new(exists_c), dom.clone())),
                    Box::new(lam),
                );
            }
            if let Expr::App(exists_app, pred_lam) = a.as_ref() {
                if let Expr::App(exists_c, _dom) = exists_app.as_ref() {
                    if is_const_named(exists_c, "Exists") {
                        if let Expr::Lam(bi, n, ty, body) = pred_lam.as_ref() {
                            let not_body = Expr::App(
                                Box::new(Expr::Const(not_name.clone(), vec![])),
                                body.clone(),
                            );
                            return Expr::Pi(
                                *bi,
                                n.clone(),
                                ty.clone(),
                                Box::new(push_negations(&not_body)),
                            );
                        }
                    }
                }
            }
            {
                let (head, args) = decompose_app_tactic(a);
                if let Expr::Const(h_name, _) = head {
                    let hn = h_name.to_string();
                    if (hn == "Nat.le" || hn == "LE.le") && args.len() >= 2 {
                        let a_arg = args[args.len() - 2].clone();
                        let b_arg = args[args.len() - 1].clone();
                        return Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("Nat.lt"), vec![])),
                                Box::new(b_arg),
                            )),
                            Box::new(a_arg),
                        );
                    }
                    if (hn == "Nat.lt" || hn == "LT.lt") && args.len() >= 2 {
                        let a_arg = args[args.len() - 2].clone();
                        let b_arg = args[args.len() - 1].clone();
                        return Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("Nat.le"), vec![])),
                                Box::new(b_arg),
                            )),
                            Box::new(a_arg),
                        );
                    }
                    if (hn == "GE.ge" || hn == "Nat.ge") && args.len() >= 2 {
                        let a_arg = args[args.len() - 2].clone();
                        let b_arg = args[args.len() - 1].clone();
                        return Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("Nat.lt"), vec![])),
                                Box::new(a_arg),
                            )),
                            Box::new(b_arg),
                        );
                    }
                    if (hn == "GT.gt" || hn == "Nat.gt") && args.len() >= 2 {
                        let a_arg = args[args.len() - 2].clone();
                        let b_arg = args[args.len() - 1].clone();
                        return Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("Nat.le"), vec![])),
                                Box::new(a_arg),
                            )),
                            Box::new(b_arg),
                        );
                    }
                    if hn == "Ne" && args.len() >= 2 {
                        let a_arg = args[args.len() - 2].clone();
                        let b_arg = args[args.len() - 1].clone();
                        return Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::App(
                                    Box::new(Expr::Const(Name::str("Eq"), vec![])),
                                    Box::new(a_arg.clone()),
                                )),
                                Box::new(a_arg),
                            )),
                            Box::new(b_arg),
                        );
                    }
                }
            }
            expr.clone()
        }
        Expr::App(f, a) => Expr::App(Box::new(push_negations(f)), Box::new(push_negations(a))),
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            *bi,
            n.clone(),
            Box::new(push_negations(ty)),
            Box::new(push_negations(body)),
        ),
        Expr::Pi(bi, n, ty, body) => Expr::Pi(
            *bi,
            n.clone(),
            Box::new(push_negations(ty)),
            Box::new(push_negations(body)),
        ),
        _ => expr.clone(),
    }
}
/// Push negations inward through logical connectives.
///
/// Transforms: ¬ ¬ A → A, ¬ (A ∧ B) → ¬ A ∨ ¬ B, etc.
pub fn tactic_push_neg(state: &TacticState) -> TacticResult {
    let goal = get_focused_goal(state)?;
    let new_target = push_negations(&goal.target);
    if new_target == goal.target {
        return Ok(state.clone());
    }
    let mut new_goal = goal.clone();
    new_goal.target = new_target;
    new_goal.mvar_id = fresh_mvar_id();
    replace_focused(state, vec![new_goal])
}
/// Proof by contradiction: introduces `h : ¬ P` and sets goal to `False`.
pub fn tactic_by_contra(state: &TacticState, hyp_name: Name) -> TacticResult {
    let goal = get_focused_goal(state)?;
    let not_target = Expr::App(
        Box::new(Expr::Const(Name::str("Not"), vec![])),
        Box::new(goal.target.clone()),
    );
    let mut new_goal = goal.clone();
    new_goal.target = Expr::Const(Name::str("False"), vec![]);
    new_goal.mvar_id = fresh_mvar_id();
    new_goal.add_hypothesis(hyp_name, not_target);
    replace_focused(state, vec![new_goal])
}
/// Proof by contrapositive: transforms `A → B` (Pi) to `¬ B → ¬ A`.
pub fn tactic_contrapose(state: &TacticState) -> TacticResult {
    let goal = get_focused_goal(state)?;
    if let Expr::Pi(bi, name, a, b) = &goal.target {
        let not_b = Expr::App(Box::new(Expr::Const(Name::str("Not"), vec![])), b.clone());
        let not_a = Expr::App(Box::new(Expr::Const(Name::str("Not"), vec![])), a.clone());
        let new_target = Expr::Pi(*bi, name.clone(), Box::new(not_b), Box::new(not_a));
        let mut new_goal = goal.clone();
        new_goal.target = new_target;
        new_goal.mvar_id = fresh_mvar_id();
        replace_focused(state, vec![new_goal])
    } else {
        Err(TacticError::TypeMismatch(
            "contrapose: goal must be an implication (Pi)".to_string(),
        ))
    }
}
