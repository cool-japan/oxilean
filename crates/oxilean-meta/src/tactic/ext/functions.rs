//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    EqGoalInfo, EqTypeClass, ExtConfig, ExtLemma, ExtLemmaRegistry, ExtResult, NameGen,
    StructExtInfo, TacticExtAnalysisPass, TacticExtConfig, TacticExtConfigValue,
    TacticExtDiagnostics, TacticExtDiff, TacticExtPipeline, TacticExtResult,
};
use crate::basic::{MVarId, MetaContext, MetavarKind};
use crate::discr_tree::DiscrTree;
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{BinderInfo, Expr, Level, Name};

/// Default priority for extensionality lemmas.
pub const DEFAULT_EXT_PRIORITY: u32 = 1000;
/// High priority (tried first) for built-in ext lemmas.
pub const BUILTIN_EXT_PRIORITY: u32 = 100;
/// Low priority for fallback ext lemmas.
pub const FALLBACK_EXT_PRIORITY: u32 = 5000;
/// Maximum number of arguments a single ext lemma can introduce.
const MAX_EXT_LEMMA_PARAMS: usize = 64;
/// Default recursion depth for the ext tactic.
pub(super) const DEFAULT_EXT_DEPTH: usize = 5;
/// Register an extensionality lemma in the registry.
///
/// The lemma is added to the DiscrTree index, the by-name map, and the by-head map.
pub fn register_ext_lemma(registry: &mut ExtLemmaRegistry, lemma: ExtLemma) {
    let pattern = build_ext_pattern(&lemma);
    registry.lemmas.insert(&pattern, lemma.clone());
    registry.by_name.insert(lemma.name.clone(), lemma.clone());
    if let Some(ref head) = lemma.target_head {
        registry
            .by_head
            .entry(head.clone())
            .or_default()
            .push(lemma);
    }
}
/// Build the DiscrTree pattern for an ext lemma.
///
/// If the lemma has a target_head, builds `@Eq <head> _ _`; otherwise uses
/// the lemma_type directly.
pub(super) fn build_ext_pattern(lemma: &ExtLemma) -> Expr {
    if let Some(ref head) = lemma.target_head {
        mk_eq_pattern(&format!("{}", head))
    } else {
        lemma.lemma_type.clone()
    }
}
/// Build a default ExtLemmaRegistry populated with the standard built-in lemmas.
///
/// Includes:
/// - `funext`: function extensionality (f = g when ∀ x, f x = g x)
/// - `propext`: propositional extensionality (P = Q when P ↔ Q)
/// - `Set.ext`: set extensionality
/// - `Prod.ext`: product extensionality
/// - `Subtype.ext`: subtype extensionality
/// - `Sigma.ext`: sigma type extensionality
/// - `PProd.ext`: propositional product extensionality
/// - `And.ext`: conjunction extensionality
pub fn build_default_ext_registry(ctx: &MetaContext) -> ExtLemmaRegistry {
    let _ = ctx;
    let mut registry = ExtLemmaRegistry::new();
    register_ext_lemma(
        &mut registry,
        ExtLemma::builtin(Name::str("funext"), 1, "Pi"),
    );
    register_ext_lemma(
        &mut registry,
        ExtLemma::new(
            Name::str("propext"),
            Expr::Const(Name::str("propext"), vec![]),
            mk_eq_pattern("Prop"),
            BUILTIN_EXT_PRIORITY + 10,
            1,
            Some(Name::str("Prop")),
        ),
    );
    register_ext_lemma(
        &mut registry,
        ExtLemma::new(
            Name::str("Set.ext"),
            Expr::Const(Name::str("Set.ext"), vec![]),
            mk_eq_pattern("Set"),
            BUILTIN_EXT_PRIORITY + 20,
            1,
            Some(Name::str("Set")),
        ),
    );
    register_ext_lemma(
        &mut registry,
        ExtLemma::new(
            Name::str("Prod.ext"),
            Expr::Const(Name::str("Prod.ext"), vec![]),
            mk_eq_pattern("Prod"),
            BUILTIN_EXT_PRIORITY + 30,
            2,
            Some(Name::str("Prod")),
        ),
    );
    register_ext_lemma(
        &mut registry,
        ExtLemma::new(
            Name::str("Subtype.ext"),
            Expr::Const(Name::str("Subtype.ext"), vec![]),
            mk_eq_pattern("Subtype"),
            BUILTIN_EXT_PRIORITY + 40,
            1,
            Some(Name::str("Subtype")),
        ),
    );
    register_ext_lemma(
        &mut registry,
        ExtLemma::new(
            Name::str("Sigma.ext"),
            Expr::Const(Name::str("Sigma.ext"), vec![]),
            mk_eq_pattern("Sigma"),
            BUILTIN_EXT_PRIORITY + 50,
            2,
            Some(Name::str("Sigma")),
        ),
    );
    register_ext_lemma(
        &mut registry,
        ExtLemma::new(
            Name::str("PProd.ext"),
            Expr::Const(Name::str("PProd.ext"), vec![]),
            mk_eq_pattern("PProd"),
            BUILTIN_EXT_PRIORITY + 60,
            2,
            Some(Name::str("PProd")),
        ),
    );
    register_ext_lemma(
        &mut registry,
        ExtLemma::new(
            Name::str("And.ext"),
            Expr::Const(Name::str("And.ext"), vec![]),
            mk_eq_pattern("And"),
            BUILTIN_EXT_PRIORITY + 70,
            2,
            Some(Name::str("And")),
        ),
    );
    registry
}
/// Try to decompose an expression as `@Eq α lhs rhs`.
pub(super) fn analyze_eq_goal(target: &Expr) -> Option<EqGoalInfo> {
    if let Expr::App(eq_lhs_box, rhs) = target {
        if let Expr::App(eq_ty_box, lhs) = eq_lhs_box.as_ref() {
            if let Expr::App(eq_const_box, alpha) = eq_ty_box.as_ref() {
                if is_const_named(eq_const_box, "Eq") {
                    return Some(EqGoalInfo {
                        eq_type: (**alpha).clone(),
                        lhs: (**lhs).clone(),
                        rhs: (**rhs).clone(),
                    });
                }
            }
        }
    }
    None
}
/// Try to decompose an expression as `@Iff P Q`.
pub(super) fn analyze_iff_goal(target: &Expr) -> Option<(Expr, Expr)> {
    if let Expr::App(iff_p_box, q) = target {
        if let Expr::App(iff_const_box, p) = iff_p_box.as_ref() {
            if is_const_named(iff_const_box, "Iff") {
                return Some(((**p).clone(), (**q).clone()));
            }
        }
    }
    None
}
/// Check whether `expr` is `Const(name, _)`.
pub(super) fn is_const_named(expr: &Expr, name: &str) -> bool {
    matches!(expr, Expr::Const(n, _) if format!("{}", n) == name)
}
/// Get the head constant name of a (possibly applied) expression.
pub(super) fn get_head_const_name(expr: &Expr) -> Option<Name> {
    let mut e = expr;
    while let Expr::App(f, _) = e {
        e = f;
    }
    match e {
        Expr::Const(name, _) => Some(name.clone()),
        _ => None,
    }
}
/// Check whether the equality type is a function type (Pi).
pub(super) fn is_function_type(ty: &Expr) -> bool {
    matches!(ty, Expr::Pi(..))
}
/// Check whether the equality type is `Prop` (Sort(0)).
pub(super) fn is_prop_type(ty: &Expr) -> bool {
    matches!(ty, Expr::Sort(l) if * l == Level::zero())
}
/// Check whether the equality type looks like a `Set α` application.
pub(super) fn is_set_type(ty: &Expr) -> bool {
    get_head_const_name(ty)
        .map(|n| {
            let s = format!("{}", n);
            s == "Set" || s == "Finset"
        })
        .unwrap_or(false)
}
/// Check whether the equality type is a known structure.
pub(super) fn is_struct_type(ty: &Expr) -> Option<Name> {
    get_head_const_name(ty).and_then(|name| {
        if known_struct_ext(&name).is_some() {
            Some(name)
        } else {
            None
        }
    })
}
/// Classify the type of an equality goal for ext dispatch.
pub(super) fn classify_eq_type(eq_type: &Expr) -> EqTypeClass {
    if is_function_type(eq_type) {
        EqTypeClass::Function
    } else if is_prop_type(eq_type) {
        EqTypeClass::Prop
    } else if is_set_type(eq_type) {
        EqTypeClass::Set
    } else if let Some(name) = is_struct_type(eq_type) {
        EqTypeClass::Struct(name)
    } else {
        EqTypeClass::Unknown
    }
}
/// Lookup well-known structure extensionality info.
pub(super) fn known_struct_ext(name: &Name) -> Option<StructExtInfo> {
    let s = format!("{}", name);
    match s.as_str() {
        "Prod" => Some(StructExtInfo::new(
            Name::str("Prod"),
            vec![Name::str("fst"), Name::str("snd")],
            vec![
                Expr::Sort(Level::Succ(Box::new(Level::Zero))),
                Expr::Sort(Level::Succ(Box::new(Level::Zero))),
            ],
            2,
        )),
        "PProd" => Some(StructExtInfo::new(
            Name::str("PProd"),
            vec![Name::str("fst"), Name::str("snd")],
            vec![
                Expr::Sort(Level::Succ(Box::new(Level::Zero))),
                Expr::Sort(Level::Succ(Box::new(Level::Zero))),
            ],
            2,
        )),
        "Sigma" => Some(StructExtInfo::new(
            Name::str("Sigma"),
            vec![Name::str("fst"), Name::str("snd")],
            vec![
                Expr::Sort(Level::Succ(Box::new(Level::Zero))),
                Expr::Sort(Level::Succ(Box::new(Level::Zero))),
            ],
            2,
        )),
        "Subtype" => Some(StructExtInfo::new(
            Name::str("Subtype"),
            vec![Name::str("val")],
            vec![Expr::Sort(Level::Succ(Box::new(Level::Zero)))],
            2,
        )),
        "And" => Some(StructExtInfo::new(
            Name::str("And"),
            vec![Name::str("left"), Name::str("right")],
            vec![Expr::Sort(Level::zero()), Expr::Sort(Level::zero())],
            0,
        )),
        "Iff" => Some(StructExtInfo::new(
            Name::str("Iff"),
            vec![Name::str("mp"), Name::str("mpr")],
            vec![Expr::Sort(Level::zero()), Expr::Sort(Level::zero())],
            0,
        )),
        _ => None,
    }
}
/// Build `@Eq α lhs rhs`.
pub(super) fn mk_eq_expr(ty: Expr, lhs: Expr, rhs: Expr) -> Expr {
    let eq = Expr::Const(Name::str("Eq"), vec![Level::zero()]);
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::App(Box::new(eq), Box::new(ty))),
            Box::new(lhs),
        )),
        Box::new(rhs),
    )
}
/// Build a DiscrTree lookup pattern `@Eq <TypeHead> _ _` for a given type head.
pub(super) fn mk_eq_pattern(type_head: &str) -> Expr {
    let ty = Expr::Const(Name::str(type_head), vec![]);
    mk_eq_expr(ty, Expr::BVar(0), Expr::BVar(1))
}
/// Build `@Iff P Q`.
pub(super) fn mk_iff_expr(p: Expr, q: Expr) -> Expr {
    let iff = Expr::Const(Name::str("Iff"), vec![]);
    Expr::App(Box::new(Expr::App(Box::new(iff), Box::new(p))), Box::new(q))
}
/// Build `∀ (x : α), body` (a Pi type).
pub(super) fn mk_forall(name: Name, domain: Expr, body: Expr) -> Expr {
    Expr::Pi(BinderInfo::Default, name, Box::new(domain), Box::new(body))
}
/// Build `@Membership.mem _ _ _ element set`.
pub(super) fn mk_mem_expr(element: Expr, set: Expr) -> Expr {
    let mem = Expr::Const(Name::str("Membership.mem"), vec![]);
    Expr::App(
        Box::new(Expr::App(Box::new(mem), Box::new(element))),
        Box::new(set),
    )
}
/// Substitute BVar(idx) with `replacement` in `expr`.
pub(super) fn substitute_bvar(expr: &Expr, idx: u32, replacement: &Expr) -> Expr {
    match expr {
        Expr::BVar(n) => {
            if *n == idx {
                replacement.clone()
            } else if *n > idx {
                Expr::BVar(n - 1)
            } else {
                expr.clone()
            }
        }
        Expr::App(f, a) => {
            let f2 = substitute_bvar(f, idx, replacement);
            let a2 = substitute_bvar(a, idx, replacement);
            Expr::App(Box::new(f2), Box::new(a2))
        }
        Expr::Lam(bi, name, ty, body) => {
            let ty2 = substitute_bvar(ty, idx, replacement);
            let body2 = substitute_bvar(body, idx + 1, replacement);
            Expr::Lam(*bi, name.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Pi(bi, name, ty, body) => {
            let ty2 = substitute_bvar(ty, idx, replacement);
            let body2 = substitute_bvar(body, idx + 1, replacement);
            Expr::Pi(*bi, name.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Let(name, ty, val, body) => {
            let ty2 = substitute_bvar(ty, idx, replacement);
            let val2 = substitute_bvar(val, idx, replacement);
            let body2 = substitute_bvar(body, idx + 1, replacement);
            Expr::Let(name.clone(), Box::new(ty2), Box::new(val2), Box::new(body2))
        }
        Expr::Proj(name, i, e) => {
            let e2 = substitute_bvar(e, idx, replacement);
            Expr::Proj(name.clone(), *i, Box::new(e2))
        }
        _ => expr.clone(),
    }
}
/// Compute the codomain of a Pi type after substituting with `arg`.
pub(super) fn compute_codomain(pi_ty: &Expr, arg: &Expr) -> Expr {
    match pi_ty {
        Expr::Pi(_bi, _name, _dom, body) => substitute_bvar(body, 0, arg),
        _ => Expr::Sort(Level::zero()),
    }
}
/// Extract the element type from `Set α` (returns α or a placeholder).
pub(super) fn extract_set_element_type(ty: &Expr) -> Expr {
    if let Expr::App(_head, arg) = ty {
        arg.as_ref().clone()
    } else {
        Expr::Sort(Level::zero())
    }
}
/// Generate a fresh variable name from an optional user name and a counter.
pub(super) fn fresh_name(user_name: Option<Name>, prefix: &str, counter: usize) -> Name {
    user_name.unwrap_or_else(|| Name::str(format!("{}{}", prefix, counter)))
}
/// `funext` -- specialised function-extensionality tactic.
///
/// Transforms a goal `f = g` (where `f, g : α → β`) into `∀ x, f x = g x`,
/// then introduces `x` and changes the goal to `f x = g x`.
///
/// Returns the IDs of the newly created goals (typically one).
pub fn tac_funext(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<Vec<MVarId>> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let eq_info = analyze_eq_goal(&target)
        .ok_or_else(|| TacticError::GoalMismatch("funext: goal is not an equality".into()))?;
    if !is_function_type(&eq_info.eq_type) {
        return Err(TacticError::GoalMismatch(
            "funext: equality type is not a function type".into(),
        ));
    }
    let new_goals = apply_funext(eq_info, goal, state, ctx, None)?;
    Ok(new_goals)
}
/// Internal implementation of function extensionality application.
/// Returns the list of newly created goal IDs.
pub(super) fn apply_funext(
    eq_info: EqGoalInfo,
    goal: MVarId,
    state: &mut TacticState,
    ctx: &mut MetaContext,
    user_name: Option<Name>,
) -> TacticResult<Vec<MVarId>> {
    let (domain, codomain_binder_name) = match &eq_info.eq_type {
        Expr::Pi(_bi, binder_name, dom, _body) => (dom.as_ref().clone(), binder_name.clone()),
        _ => {
            return Err(TacticError::GoalMismatch(
                "funext: equality type is not a function type".into(),
            ));
        }
    };
    let intro_name = user_name.unwrap_or(codomain_binder_name);
    let fvar_id = ctx.mk_local_decl(intro_name.clone(), domain.clone(), BinderInfo::Default);
    let x = Expr::FVar(fvar_id);
    let lhs_app = Expr::App(Box::new(eq_info.lhs.clone()), Box::new(x.clone()));
    let rhs_app = Expr::App(Box::new(eq_info.rhs.clone()), Box::new(x.clone()));
    let codomain = compute_codomain(&eq_info.eq_type, &x);
    let new_target = mk_eq_expr(codomain, lhs_app, rhs_app);
    let (new_goal_id, new_goal_expr) = ctx.mk_fresh_expr_mvar(new_target, MetavarKind::Natural);
    let proof_body = Expr::Lam(
        BinderInfo::Default,
        intro_name,
        Box::new(domain),
        Box::new(new_goal_expr),
    );
    let funext_proof = Expr::App(
        Box::new(Expr::Const(
            Name::str("funext"),
            vec![Level::zero(), Level::zero()],
        )),
        Box::new(proof_body),
    );
    ctx.assign_mvar(goal, funext_proof);
    state.replace_goal(vec![new_goal_id]);
    Ok(vec![new_goal_id])
}
/// Apply propositional extensionality.
///
/// Transforms a goal `P = Q` (where P, Q : Prop) into two subgoals:
/// `P → Q` and `Q → P`.
pub fn tac_propext(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let eq_info = analyze_eq_goal(&target)
        .ok_or_else(|| TacticError::GoalMismatch("propext: goal is not an equality".into()))?;
    if !is_prop_type(&eq_info.eq_type) {
        return Err(TacticError::GoalMismatch(
            "propext: equality type is not Prop".into(),
        ));
    }
    apply_propext(eq_info, goal, state, ctx)
}
/// Internal implementation of propositional extensionality application.
pub(super) fn apply_propext(
    eq_info: EqGoalInfo,
    goal: MVarId,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<()> {
    let p = eq_info.lhs.clone();
    let q = eq_info.rhs.clone();
    let forward_ty = mk_forall(Name::str("h"), p.clone(), q.clone());
    let backward_ty = mk_forall(Name::str("h"), q.clone(), p.clone());
    let (fwd_id, fwd_expr) = ctx.mk_fresh_expr_mvar(forward_ty, MetavarKind::Natural);
    let (bwd_id, bwd_expr) = ctx.mk_fresh_expr_mvar(backward_ty, MetavarKind::Natural);
    let iff_intro = Expr::Const(Name::str("Iff.intro"), vec![]);
    let iff_proof = Expr::App(
        Box::new(Expr::App(Box::new(iff_intro), Box::new(fwd_expr))),
        Box::new(bwd_expr),
    );
    let propext_proof = Expr::App(
        Box::new(Expr::Const(Name::str("propext"), vec![])),
        Box::new(iff_proof),
    );
    ctx.assign_mvar(goal, propext_proof);
    state.replace_goal(vec![fwd_id, bwd_id]);
    Ok(())
}
/// Apply set extensionality: `S = T` → `∀ x, x ∈ S ↔ x ∈ T`.
pub(super) fn apply_set_ext(
    eq_info: EqGoalInfo,
    goal: MVarId,
    state: &mut TacticState,
    ctx: &mut MetaContext,
    user_name: Option<Name>,
) -> TacticResult<Vec<MVarId>> {
    let elem_type = extract_set_element_type(&eq_info.eq_type);
    let var_name = user_name.unwrap_or_else(|| Name::str("x"));
    let fvar_id = ctx.mk_local_decl(var_name.clone(), elem_type.clone(), BinderInfo::Default);
    let x = Expr::FVar(fvar_id);
    let mem_lhs = mk_mem_expr(x.clone(), eq_info.lhs.clone());
    let mem_rhs = mk_mem_expr(x, eq_info.rhs.clone());
    let iff_goal = mk_iff_expr(mem_lhs, mem_rhs);
    let new_target = mk_forall(var_name.clone(), elem_type, iff_goal);
    let (new_goal_id, new_goal_expr) = ctx.mk_fresh_expr_mvar(new_target, MetavarKind::Natural);
    let set_ext_proof = Expr::App(
        Box::new(Expr::Const(Name::str("Set.ext"), vec![Level::zero()])),
        Box::new(Expr::Lam(
            BinderInfo::Default,
            var_name,
            Box::new(Expr::Sort(Level::zero())),
            Box::new(new_goal_expr),
        )),
    );
    ctx.assign_mvar(goal, set_ext_proof);
    state.replace_goal(vec![new_goal_id]);
    Ok(vec![new_goal_id])
}
/// Apply structural extensionality: decompose `a = b` for a structure into
/// field-wise equality goals.
pub(super) fn apply_struct_ext(
    eq_info: EqGoalInfo,
    struct_info: &StructExtInfo,
    goal: MVarId,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<Vec<MVarId>> {
    let field_eqs = struct_info.field_equalities(&eq_info.lhs, &eq_info.rhs);
    let mut new_goal_ids = Vec::new();
    let mut field_proofs = Vec::new();
    for eq_target in &field_eqs {
        let (field_goal_id, field_goal_expr) =
            ctx.mk_fresh_expr_mvar(eq_target.clone(), MetavarKind::Natural);
        new_goal_ids.push(field_goal_id);
        field_proofs.push(field_goal_expr);
    }
    let ext_name = Name::str(format!("{}.ext", struct_info.struct_name));
    let mut proof = Expr::Const(ext_name, vec![Level::zero()]);
    for fp in &field_proofs {
        proof = Expr::App(Box::new(proof), Box::new(fp.clone()));
    }
    ctx.assign_mvar(goal, proof);
    state.replace_goal(new_goal_ids.clone());
    Ok(new_goal_ids)
}
/// Try to apply a custom extensionality lemma from the registry.
pub(super) fn apply_custom_ext(
    lemma: &ExtLemma,
    goal: MVarId,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<Vec<MVarId>> {
    let sort_ty = Expr::Sort(Level::zero());
    let mut new_goal_ids = Vec::new();
    let mut app = lemma.to_const_expr();
    let params = lemma.num_params.min(MAX_EXT_LEMMA_PARAMS);
    for _ in 0..params {
        let (arg_id, arg_expr) = ctx.mk_fresh_expr_mvar(sort_ty.clone(), MetavarKind::Natural);
        new_goal_ids.push(arg_id);
        app = Expr::App(Box::new(app), Box::new(arg_expr));
    }
    ctx.assign_mvar(goal, app);
    state.replace_goal(new_goal_ids.clone());
    Ok(new_goal_ids)
}
/// `ext` -- the main extensionality tactic.
///
/// Inspects the goal, looks up applicable ext lemmas, and applies the best match.
/// Uses the default configuration (depth 5, default lemmas enabled).
pub fn tac_ext(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<ExtResult> {
    let config = ExtConfig::default();
    tac_ext_with_config(&config, state, ctx)
}
/// Apply the ext tactic with a full configuration.
///
/// Supports custom lemmas, variable name supply, recursion depth control, and
/// toggling of the default lemma set.
pub fn tac_ext_with_config(
    config: &ExtConfig,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<ExtResult> {
    if state.is_done() {
        return Err(TacticError::NoGoals);
    }
    let registry = if config.use_default_lemmas {
        let mut reg = build_default_ext_registry(ctx);
        for lemma in &config.extra_lemmas {
            register_ext_lemma(&mut reg, lemma.clone());
        }
        reg
    } else {
        let mut reg = ExtLemmaRegistry::new();
        for lemma in &config.extra_lemmas {
            register_ext_lemma(&mut reg, lemma.clone());
        }
        reg
    };
    let mut mutable_config = config.clone();
    if config.max_depth <= 1 {
        apply_ext_step(&registry, &mut mutable_config, state, ctx, 1)
    } else {
        apply_ext_recursive(&registry, &mut mutable_config, config.max_depth, state, ctx)
    }
}
/// Apply ext once to the current goal. Returns an ExtResult.
pub(super) fn apply_ext_step(
    registry: &ExtLemmaRegistry,
    config: &mut ExtConfig,
    state: &mut TacticState,
    ctx: &mut MetaContext,
    depth: usize,
) -> TacticResult<ExtResult> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    if let Some(eq_info) = analyze_eq_goal(&target) {
        let class = classify_eq_type(&eq_info.eq_type);
        match class {
            EqTypeClass::Function => {
                let name = config.next_name();
                let new_goals = apply_funext(eq_info, goal, state, ctx, name)?;
                return Ok(ExtResult {
                    new_goals,
                    lemmas_applied: vec![Name::str("funext")],
                    depth_reached: depth,
                });
            }
            EqTypeClass::Prop => {
                apply_propext(eq_info, goal, state, ctx)?;
                return Ok(ExtResult {
                    new_goals: state.all_goals().to_vec(),
                    lemmas_applied: vec![Name::str("propext")],
                    depth_reached: depth,
                });
            }
            EqTypeClass::Set => {
                let name = config.next_name();
                let new_goals = apply_set_ext(eq_info, goal, state, ctx, name)?;
                return Ok(ExtResult {
                    new_goals,
                    lemmas_applied: vec![Name::str("Set.ext")],
                    depth_reached: depth,
                });
            }
            EqTypeClass::Struct(ref struct_name) => {
                if let Some(struct_info) = known_struct_ext(struct_name) {
                    let ext_lemma_name = Name::str(format!("{}.ext", struct_info.struct_name));
                    let new_goals = apply_struct_ext(eq_info, &struct_info, goal, state, ctx)?;
                    return Ok(ExtResult {
                        new_goals,
                        lemmas_applied: vec![ext_lemma_name],
                        depth_reached: depth,
                    });
                }
            }
            EqTypeClass::Unknown => {}
        }
        let candidates = registry.query(&target);
        for lemma in candidates {
            state.save();
            let saved_meta = ctx.save_state();
            match apply_custom_ext(lemma, goal, state, ctx) {
                Ok(new_goals) => {
                    return Ok(ExtResult {
                        new_goals,
                        lemmas_applied: vec![lemma.name.clone()],
                        depth_reached: depth,
                    });
                }
                Err(_) => {
                    ctx.restore_state(saved_meta);
                    let _ = state.restore();
                }
            }
        }
        return Err(TacticError::Failed(
            "ext: no applicable extensionality lemma found".into(),
        ));
    }
    if let Some((p, q)) = analyze_iff_goal(&target) {
        let forward_ty = mk_forall(Name::str("h"), p.clone(), q.clone());
        let backward_ty = mk_forall(Name::str("h"), q, p);
        let (fwd_id, fwd_expr) = ctx.mk_fresh_expr_mvar(forward_ty, MetavarKind::Natural);
        let (bwd_id, bwd_expr) = ctx.mk_fresh_expr_mvar(backward_ty, MetavarKind::Natural);
        let iff_intro = Expr::Const(Name::str("Iff.intro"), vec![]);
        let proof = Expr::App(
            Box::new(Expr::App(Box::new(iff_intro), Box::new(fwd_expr))),
            Box::new(bwd_expr),
        );
        ctx.assign_mvar(goal, proof);
        state.replace_goal(vec![fwd_id, bwd_id]);
        return Ok(ExtResult {
            new_goals: vec![fwd_id, bwd_id],
            lemmas_applied: vec![Name::str("Iff.intro")],
            depth_reached: depth,
        });
    }
    Err(TacticError::GoalMismatch(
        "ext: goal is not an equality or iff".into(),
    ))
}
/// Apply ext recursively up to `remaining_depth` times.
pub(super) fn apply_ext_recursive(
    registry: &ExtLemmaRegistry,
    config: &mut ExtConfig,
    remaining_depth: usize,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<ExtResult> {
    if remaining_depth == 0 {
        return Ok(ExtResult {
            new_goals: state.all_goals().to_vec(),
            lemmas_applied: Vec::new(),
            depth_reached: 0,
        });
    }
    if state.is_done() {
        return Ok(ExtResult {
            new_goals: vec![],
            lemmas_applied: vec![],
            depth_reached: 0,
        });
    }
    let current_depth = config.max_depth - remaining_depth + 1;
    match apply_ext_step(registry, config, state, ctx, current_depth) {
        Ok(step_result) => {
            let mut all_lemmas = step_result.lemmas_applied;
            let mut max_depth = step_result.depth_reached;
            let num_goals = state.num_goals();
            for i in 0..num_goals {
                if state.focus(i).is_err() {
                    break;
                }
                match apply_ext_recursive(registry, config, remaining_depth - 1, state, ctx) {
                    Ok(sub_result) => {
                        all_lemmas.extend(sub_result.lemmas_applied);
                        if sub_result.depth_reached > max_depth {
                            max_depth = sub_result.depth_reached;
                        }
                    }
                    Err(_) => {}
                }
            }
            if !state.all_goals().is_empty() {
                let _ = state.focus(0);
            }
            Ok(ExtResult {
                new_goals: state.all_goals().to_vec(),
                lemmas_applied: all_lemmas,
                depth_reached: max_depth,
            })
        }
        Err(e) => Err(e),
    }
}
/// Validate that an ExtLemma is well-formed (basic checks).
pub fn validate_ext_lemma(lemma: &ExtLemma) -> Result<(), String> {
    if format!("{}", lemma.name).is_empty() {
        return Err("ext lemma must have a non-empty name".into());
    }
    if lemma.num_params == 0 && lemma.priority > FALLBACK_EXT_PRIORITY {
        return Err("zero-argument ext lemma should not have fallback priority".into());
    }
    if lemma.num_params > MAX_EXT_LEMMA_PARAMS {
        return Err(format!(
            "ext lemma has too many params: {} (max {})",
            lemma.num_params, MAX_EXT_LEMMA_PARAMS
        ));
    }
    Ok(())
}
/// Return a human-readable description of why ext might fail on a goal.
pub fn ext_diagnostic(target: &Expr) -> String {
    if let Some(eq_info) = analyze_eq_goal(target) {
        let class = classify_eq_type(&eq_info.eq_type);
        match class {
            EqTypeClass::Function => {
                "goal is a function equality -- funext should apply".to_string()
            }
            EqTypeClass::Prop => {
                "goal is a propositional equality -- propext should apply".to_string()
            }
            EqTypeClass::Set => "goal is a set equality -- Set.ext should apply".to_string(),
            EqTypeClass::Struct(name) => {
                format!(
                    "goal is a structural equality for {} -- {}.ext should apply",
                    name, name
                )
            }
            EqTypeClass::Unknown => {
                "goal is an equality but no built-in ext lemma matches the type".to_string()
            }
        }
    } else if analyze_iff_goal(target).is_some() {
        "goal is an iff -- can be decomposed into two implications".to_string()
    } else {
        "goal is not an equality or iff -- ext does not apply".to_string()
    }
}
/// Check whether the ext tactic can make progress on the given target.
pub fn can_apply_ext(target: &Expr) -> bool {
    if let Some(eq_info) = analyze_eq_goal(target) {
        !matches!(classify_eq_type(&eq_info.eq_type), EqTypeClass::Unknown)
    } else {
        analyze_iff_goal(target).is_some()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::ext::*;
    use oxilean_kernel::Environment;
    fn mk_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    fn nat() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn prop() -> Expr {
        Expr::Sort(Level::zero())
    }
    fn nat_arrow_nat() -> Expr {
        Expr::Pi(
            BinderInfo::Default,
            Name::str("n"),
            Box::new(nat()),
            Box::new(nat()),
        )
    }
    fn mk_eq_nat(lhs: Expr, rhs: Expr) -> Expr {
        mk_eq_expr(nat(), lhs, rhs)
    }
    fn mk_eq_fn(lhs: Expr, rhs: Expr) -> Expr {
        mk_eq_expr(nat_arrow_nat(), lhs, rhs)
    }
    fn mk_eq_prop(lhs: Expr, rhs: Expr) -> Expr {
        mk_eq_expr(prop(), lhs, rhs)
    }
    fn mk_goal(ctx: &mut MetaContext, ty: Expr) -> (MVarId, TacticState) {
        let (id, _) = ctx.mk_fresh_expr_mvar(ty, MetavarKind::Natural);
        (id, TacticState::single(id))
    }
    fn const_expr(s: &str) -> Expr {
        Expr::Const(Name::str(s), vec![])
    }
    #[test]
    fn test_ext_lemma_new() {
        let lemma = ExtLemma::new(
            Name::str("test"),
            nat(),
            nat(),
            DEFAULT_EXT_PRIORITY,
            2,
            None,
        );
        assert_eq!(format!("{}", lemma.name), "test");
        assert_eq!(lemma.num_params, 2);
        assert!(lemma.is_default_priority());
    }
    #[test]
    fn test_ext_lemma_builtin() {
        let lemma = ExtLemma::builtin(Name::str("funext"), 1, "Pi");
        assert!(lemma.is_builtin_priority());
        assert!(lemma.targets_head(&Name::str("Pi")));
        assert!(!lemma.targets_head(&Name::str("Nat")));
    }
    #[test]
    fn test_ext_lemma_simple() {
        let lemma = ExtLemma::simple(Name::str("my_lemma"), 3, 500);
        assert_eq!(lemma.num_params, 3);
        assert_eq!(lemma.priority, 500);
        assert!(lemma.target_head.is_none());
    }
    #[test]
    fn test_ext_lemma_to_const_expr() {
        let lemma = ExtLemma::simple(Name::str("funext"), 1, 100);
        let e = lemma.to_const_expr();
        assert!(
            matches!(e, Expr::Const(n, levels) if format!("{}", n) == "funext" && levels
            .is_empty())
        );
    }
    #[test]
    fn test_ext_lemma_to_const_with_levels() {
        let lemma = ExtLemma::simple(Name::str("funext"), 1, 100);
        let e = lemma.to_const_expr_with_levels(vec![Level::zero()]);
        assert!(matches!(e, Expr::Const(_, levels) if levels.len() == 1));
    }
    #[test]
    fn test_ext_lemma_display() {
        let lemma = ExtLemma::simple(Name::str("test"), 2, 1000);
        let s = format!("{}", lemma);
        assert!(s.contains("test"));
        assert!(s.contains("1000"));
    }
    #[test]
    fn test_registry_new_empty() {
        let reg = ExtLemmaRegistry::new();
        assert!(reg.is_empty());
        assert_eq!(reg.num_lemmas(), 0);
    }
    #[test]
    fn test_registry_register_and_get() {
        let mut reg = ExtLemmaRegistry::new();
        let lemma = ExtLemma::simple(Name::str("test"), 1, 1000);
        register_ext_lemma(&mut reg, lemma);
        assert_eq!(reg.num_lemmas(), 1);
        assert!(!reg.is_empty());
        assert!(reg.contains(&Name::str("test")));
        assert!(reg.get(&Name::str("test")).is_some());
    }
    #[test]
    fn test_registry_remove() {
        let mut reg = ExtLemmaRegistry::new();
        let lemma = ExtLemma::simple(Name::str("test"), 1, 1000);
        register_ext_lemma(&mut reg, lemma);
        reg.remove(&Name::str("test"));
        assert!(!reg.contains(&Name::str("test")));
    }
    #[test]
    fn test_registry_query_by_head() {
        let mut reg = ExtLemmaRegistry::new();
        register_ext_lemma(&mut reg, ExtLemma::builtin(Name::str("funext"), 1, "Pi"));
        register_ext_lemma(&mut reg, ExtLemma::builtin(Name::str("propext"), 1, "Prop"));
        let pi_results = reg.query_by_head(&Name::str("Pi"));
        assert_eq!(pi_results.len(), 1);
        assert_eq!(format!("{}", pi_results[0].name), "funext");
        let prop_results = reg.query_by_head(&Name::str("Prop"));
        assert_eq!(prop_results.len(), 1);
    }
    #[test]
    fn test_registry_build_default() {
        let ctx = mk_ctx();
        let reg = build_default_ext_registry(&ctx);
        assert!(reg.num_lemmas() >= 4);
        assert!(reg.contains(&Name::str("funext")));
        assert!(reg.contains(&Name::str("propext")));
        assert!(reg.contains(&Name::str("Set.ext")));
        assert!(reg.contains(&Name::str("Prod.ext")));
    }
    #[test]
    fn test_registry_merge() {
        let mut reg1 = ExtLemmaRegistry::new();
        let mut reg2 = ExtLemmaRegistry::new();
        register_ext_lemma(&mut reg2, ExtLemma::simple(Name::str("a"), 1, 100));
        register_ext_lemma(&mut reg2, ExtLemma::simple(Name::str("b"), 1, 200));
        reg1.merge(&reg2);
        assert_eq!(reg1.num_lemmas(), 2);
    }
    #[test]
    fn test_registry_clear() {
        let ctx = mk_ctx();
        let mut reg = build_default_ext_registry(&ctx);
        reg.clear();
        assert!(reg.is_empty());
    }
    #[test]
    fn test_registry_all_names() {
        let mut reg = ExtLemmaRegistry::new();
        register_ext_lemma(&mut reg, ExtLemma::simple(Name::str("a"), 0, 100));
        register_ext_lemma(&mut reg, ExtLemma::simple(Name::str("b"), 0, 200));
        let names = reg.all_names();
        assert_eq!(names.len(), 2);
    }
    #[test]
    fn test_registry_summary() {
        let ctx = mk_ctx();
        let reg = build_default_ext_registry(&ctx);
        let summary = reg.summary();
        assert!(summary.total_lemmas >= 4);
        let s = format!("{}", summary);
        assert!(s.contains("lemmas"));
    }
    #[test]
    fn test_registry_best_for_head() {
        let mut reg = ExtLemmaRegistry::new();
        register_ext_lemma(
            &mut reg,
            ExtLemma::new(
                Name::str("low_pri"),
                nat(),
                nat(),
                5000,
                1,
                Some(Name::str("Pi")),
            ),
        );
        register_ext_lemma(
            &mut reg,
            ExtLemma::new(
                Name::str("high_pri"),
                nat(),
                nat(),
                100,
                1,
                Some(Name::str("Pi")),
            ),
        );
        let best = reg.best_for_head(&Name::str("Pi"));
        assert!(best.is_some());
        assert_eq!(
            format!("{}", best.expect("best should be valid").name),
            "high_pri"
        );
    }
    #[test]
    fn test_config_default() {
        let config = ExtConfig::default();
        assert_eq!(config.max_depth, DEFAULT_EXT_DEPTH);
        assert!(config.use_default_lemmas);
        assert!(config.extra_lemmas.is_empty());
        assert!(config.with_names.is_empty());
    }
    #[test]
    fn test_config_funext_only() {
        let config = ExtConfig::funext_only();
        assert!(!config.use_default_lemmas);
        assert_eq!(config.extra_lemmas.len(), 1);
    }
    #[test]
    fn test_config_propext_only() {
        let config = ExtConfig::propext_only();
        assert!(!config.use_default_lemmas);
        assert_eq!(config.extra_lemmas.len(), 1);
    }
    #[test]
    fn test_config_single_step() {
        let config = ExtConfig::single_step();
        assert_eq!(config.max_depth, 1);
    }
    #[test]
    fn test_config_with_names() {
        let config = ExtConfig::with_names(vec![Name::str("x"), Name::str("y")]);
        assert_eq!(config.with_names.len(), 2);
    }
    #[test]
    fn test_config_builders() {
        let config = ExtConfig::default()
            .set_max_depth(10)
            .set_use_defaults(false)
            .add_extra_lemmas(vec![ExtLemma::simple(Name::str("my"), 1, 100)]);
        assert_eq!(config.max_depth, 10);
        assert!(!config.use_default_lemmas);
        assert_eq!(config.extra_lemmas.len(), 1);
    }
    #[test]
    fn test_config_next_name() {
        let mut config = ExtConfig::with_names(vec![Name::str("x"), Name::str("y")]);
        assert_eq!(config.remaining_names(), 2);
        assert_eq!(config.next_name(), Some(Name::str("x")));
        assert_eq!(config.next_name(), Some(Name::str("y")));
        assert_eq!(config.next_name(), None);
        assert_eq!(config.remaining_names(), 0);
    }
    #[test]
    fn test_ext_result_no_progress() {
        let r = ExtResult::no_progress();
        assert!(!r.made_progress());
        assert_eq!(r.num_new_goals(), 0);
        assert_eq!(r.num_lemmas_applied(), 0);
    }
    #[test]
    fn test_ext_result_with_progress() {
        let r = ExtResult {
            new_goals: vec![MVarId(10), MVarId(11)],
            lemmas_applied: vec![Name::str("funext")],
            depth_reached: 1,
        };
        assert!(r.made_progress());
        assert_eq!(r.num_new_goals(), 2);
        assert_eq!(r.num_lemmas_applied(), 1);
        assert_eq!(r.depth_reached, 1);
    }
    #[test]
    fn test_ext_result_display() {
        let r = ExtResult {
            new_goals: vec![MVarId(1)],
            lemmas_applied: vec![Name::str("funext")],
            depth_reached: 1,
        };
        let s = format!("{}", r);
        assert!(s.contains("1 lemma"));
    }
    #[test]
    fn test_analyze_eq_goal_simple() {
        let goal = mk_eq_nat(const_expr("a"), const_expr("b"));
        let info = analyze_eq_goal(&goal).expect("info should be present");
        assert_eq!(info.lhs, const_expr("a"));
        assert_eq!(info.rhs, const_expr("b"));
    }
    #[test]
    fn test_analyze_eq_goal_not_eq() {
        assert!(analyze_eq_goal(&nat()).is_none());
    }
    #[test]
    fn test_analyze_iff_goal() {
        let p = const_expr("P");
        let q = const_expr("Q");
        let iff = mk_iff_expr(p.clone(), q.clone());
        let result = analyze_iff_goal(&iff);
        assert!(result.is_some());
        let (lhs, rhs) = result.expect("result should be valid");
        assert_eq!(lhs, p);
        assert_eq!(rhs, q);
    }
    #[test]
    fn test_classify_eq_types() {
        assert!(matches!(
            classify_eq_type(&nat_arrow_nat()),
            EqTypeClass::Function
        ));
        assert!(matches!(classify_eq_type(&prop()), EqTypeClass::Prop));
        assert!(matches!(classify_eq_type(&nat()), EqTypeClass::Unknown));
    }
    #[test]
    fn test_can_apply_ext_function() {
        let goal = mk_eq_fn(const_expr("f"), const_expr("g"));
        assert!(can_apply_ext(&goal));
    }
    #[test]
    fn test_can_apply_ext_prop() {
        let goal = mk_eq_prop(const_expr("P"), const_expr("Q"));
        assert!(can_apply_ext(&goal));
    }
    #[test]
    fn test_can_apply_ext_iff() {
        let goal = mk_iff_expr(const_expr("P"), const_expr("Q"));
        assert!(can_apply_ext(&goal));
    }
    #[test]
    fn test_can_apply_ext_not_applicable() {
        assert!(!can_apply_ext(&nat()));
    }
    #[test]
    fn test_funext_success() {
        let mut ctx = mk_ctx();
        let goal_ty = mk_eq_fn(const_expr("f"), const_expr("g"));
        let (_id, mut state) = mk_goal(&mut ctx, goal_ty);
        let new_goals = tac_funext(&mut state, &mut ctx).expect("new_goals should be present");
        assert_eq!(new_goals.len(), 1);
        assert_eq!(state.num_goals(), 1);
    }
    #[test]
    fn test_funext_not_function() {
        let mut ctx = mk_ctx();
        let goal_ty = mk_eq_nat(const_expr("a"), const_expr("b"));
        let (_id, mut state) = mk_goal(&mut ctx, goal_ty);
        assert!(tac_funext(&mut state, &mut ctx).is_err());
    }
    #[test]
    fn test_funext_not_equality() {
        let mut ctx = mk_ctx();
        let goal_ty = nat();
        let (_id, mut state) = mk_goal(&mut ctx, goal_ty);
        assert!(tac_funext(&mut state, &mut ctx).is_err());
    }
    #[test]
    fn test_funext_no_goals() {
        let mut ctx = mk_ctx();
        let mut state = TacticState::new(vec![]);
        assert!(tac_funext(&mut state, &mut ctx).is_err());
    }
    #[test]
    fn test_propext_success() {
        let mut ctx = mk_ctx();
        let goal_ty = mk_eq_prop(const_expr("P"), const_expr("Q"));
        let (_id, mut state) = mk_goal(&mut ctx, goal_ty);
        tac_propext(&mut state, &mut ctx).expect("value should be present");
        assert_eq!(state.num_goals(), 2);
    }
    #[test]
    fn test_propext_not_prop() {
        let mut ctx = mk_ctx();
        let goal_ty = mk_eq_nat(const_expr("a"), const_expr("b"));
        let (_id, mut state) = mk_goal(&mut ctx, goal_ty);
        assert!(tac_propext(&mut state, &mut ctx).is_err());
    }
    #[test]
    fn test_propext_not_equality() {
        let mut ctx = mk_ctx();
        let goal_ty = nat();
        let (_id, mut state) = mk_goal(&mut ctx, goal_ty);
        assert!(tac_propext(&mut state, &mut ctx).is_err());
    }
    #[test]
    fn test_ext_function_eq() {
        let mut ctx = mk_ctx();
        let goal_ty = mk_eq_fn(const_expr("f"), const_expr("g"));
        let (_id, mut state) = mk_goal(&mut ctx, goal_ty);
        let result = tac_ext(&mut state, &mut ctx).expect("result should be present");
        assert!(result.made_progress());
        assert_eq!(result.num_new_goals(), 1);
    }
    #[test]
    fn test_ext_prop_eq() {
        let mut ctx = mk_ctx();
        let goal_ty = mk_eq_prop(const_expr("P"), const_expr("Q"));
        let (_id, mut state) = mk_goal(&mut ctx, goal_ty);
        let result = tac_ext(&mut state, &mut ctx).expect("result should be present");
        assert!(result.made_progress());
        assert_eq!(state.num_goals(), 2);
    }
    #[test]
    fn test_ext_set_eq() {
        let mut ctx = mk_ctx();
        let set_nat = Expr::App(
            Box::new(Expr::Const(Name::str("Set"), vec![])),
            Box::new(nat()),
        );
        let goal_ty = mk_eq_expr(set_nat, const_expr("S"), const_expr("T"));
        let (_id, mut state) = mk_goal(&mut ctx, goal_ty);
        let result = tac_ext(&mut state, &mut ctx).expect("result should be present");
        assert!(result.made_progress());
    }
    #[test]
    fn test_ext_struct_eq() {
        let mut ctx = mk_ctx();
        let prod_ty = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Prod"), vec![])),
                Box::new(nat()),
            )),
            Box::new(nat()),
        );
        let goal_ty = mk_eq_expr(prod_ty, const_expr("a"), const_expr("b"));
        let (_id, mut state) = mk_goal(&mut ctx, goal_ty);
        let result = tac_ext(&mut state, &mut ctx).expect("result should be present");
        assert!(result.made_progress());
        assert_eq!(state.num_goals(), 2);
    }
    #[test]
    fn test_ext_iff_goal() {
        let mut ctx = mk_ctx();
        let goal_ty = mk_iff_expr(const_expr("P"), const_expr("Q"));
        let (_id, mut state) = mk_goal(&mut ctx, goal_ty);
        let result = tac_ext(&mut state, &mut ctx).expect("result should be present");
        assert!(result.made_progress());
        assert_eq!(state.num_goals(), 2);
    }
    #[test]
    fn test_ext_fails_on_nat() {
        let mut ctx = mk_ctx();
        let goal_ty = nat();
        let (_id, mut state) = mk_goal(&mut ctx, goal_ty);
        assert!(tac_ext(&mut state, &mut ctx).is_err());
    }
    #[test]
    fn test_ext_fails_unknown_eq() {
        let mut ctx = mk_ctx();
        let goal_ty = mk_eq_nat(const_expr("a"), const_expr("b"));
        let (_id, mut state) = mk_goal(&mut ctx, goal_ty);
        assert!(tac_ext(&mut state, &mut ctx).is_err());
    }
    #[test]
    fn test_ext_with_config_funext_only() {
        let mut ctx = mk_ctx();
        let goal_ty = mk_eq_fn(const_expr("f"), const_expr("g"));
        let (_id, mut state) = mk_goal(&mut ctx, goal_ty);
        let config = ExtConfig::funext_only();
        let result =
            tac_ext_with_config(&config, &mut state, &mut ctx).expect("result should be present");
        assert!(result.made_progress());
    }
    #[test]
    fn test_ext_with_config_no_defaults() {
        let mut ctx = mk_ctx();
        let goal_ty = mk_eq_fn(const_expr("f"), const_expr("g"));
        let (_id, mut state) = mk_goal(&mut ctx, goal_ty);
        let config = ExtConfig::default().set_use_defaults(false);
        let result = tac_ext_with_config(&config, &mut state, &mut ctx);
        assert!(result.is_ok());
    }
    #[test]
    fn test_ext_with_names() {
        let mut ctx = mk_ctx();
        let goal_ty = mk_eq_fn(const_expr("f"), const_expr("g"));
        let (_id, mut state) = mk_goal(&mut ctx, goal_ty);
        let config = ExtConfig::with_names(vec![Name::str("x")]);
        let result =
            tac_ext_with_config(&config, &mut state, &mut ctx).expect("result should be present");
        assert!(result.made_progress());
    }
    #[test]
    fn test_validate_ok() {
        let lemma = ExtLemma::simple(Name::str("test"), 1, 1000);
        assert!(validate_ext_lemma(&lemma).is_ok());
    }
    #[test]
    fn test_validate_zero_args_fallback() {
        let lemma = ExtLemma::new(
            Name::str("bad"),
            nat(),
            nat(),
            FALLBACK_EXT_PRIORITY + 1,
            0,
            None,
        );
        assert!(validate_ext_lemma(&lemma).is_err());
    }
    #[test]
    fn test_validate_too_many_params() {
        let lemma = ExtLemma::new(
            Name::str("big"),
            nat(),
            nat(),
            100,
            MAX_EXT_LEMMA_PARAMS + 1,
            None,
        );
        assert!(validate_ext_lemma(&lemma).is_err());
    }
    #[test]
    fn test_ext_diagnostic_function() {
        let goal = mk_eq_fn(const_expr("f"), const_expr("g"));
        let diag = ext_diagnostic(&goal);
        assert!(diag.contains("funext"));
    }
    #[test]
    fn test_ext_diagnostic_prop() {
        let goal = mk_eq_prop(const_expr("P"), const_expr("Q"));
        let diag = ext_diagnostic(&goal);
        assert!(diag.contains("propext"));
    }
    #[test]
    fn test_ext_diagnostic_not_applicable() {
        let diag = ext_diagnostic(&nat());
        assert!(diag.contains("not an equality"));
    }
    #[test]
    fn test_substitute_bvar_hit() {
        let body = Expr::BVar(0);
        let replacement = const_expr("x");
        assert_eq!(substitute_bvar(&body, 0, &replacement), replacement);
    }
    #[test]
    fn test_substitute_bvar_miss() {
        let body = Expr::BVar(1);
        let replacement = const_expr("x");
        assert_eq!(substitute_bvar(&body, 0, &replacement), Expr::BVar(0));
    }
    #[test]
    fn test_substitute_bvar_in_app() {
        let body = Expr::App(Box::new(Expr::BVar(0)), Box::new(const_expr("y")));
        let replacement = const_expr("x");
        let result = substitute_bvar(&body, 0, &replacement);
        let expected = Expr::App(Box::new(const_expr("x")), Box::new(const_expr("y")));
        assert_eq!(result, expected);
    }
    #[test]
    fn test_ext_then_close_prop() {
        let mut ctx = mk_ctx();
        let goal_ty = mk_eq_prop(const_expr("P"), const_expr("Q"));
        let (_id, mut state) = mk_goal(&mut ctx, goal_ty);
        tac_ext(&mut state, &mut ctx).expect("value should be present");
        assert_eq!(state.num_goals(), 2);
        let g1 = state.current_goal().expect("g1 should be present");
        state
            .close_goal(const_expr("proof_fwd"), &mut ctx)
            .expect("value should be present");
        assert_eq!(state.num_goals(), 1);
        assert!(ctx.is_mvar_assigned(g1));
        let g2 = state.current_goal().expect("g2 should be present");
        state
            .close_goal(const_expr("proof_bwd"), &mut ctx)
            .expect("value should be present");
        assert!(state.is_done());
        assert!(ctx.is_mvar_assigned(g2));
    }
    #[test]
    fn test_ext_then_close_funext() {
        let mut ctx = mk_ctx();
        let goal_ty = mk_eq_fn(const_expr("f"), const_expr("g"));
        let (_id, mut state) = mk_goal(&mut ctx, goal_ty);
        tac_ext(&mut state, &mut ctx).expect("value should be present");
        assert_eq!(state.num_goals(), 1);
        let g = state.current_goal().expect("g should be present");
        state
            .close_goal(const_expr("proof"), &mut ctx)
            .expect("value should be present");
        assert!(state.is_done());
        assert!(ctx.is_mvar_assigned(g));
    }
    #[test]
    fn test_priority_ordering() {
        const _: () = assert!(BUILTIN_EXT_PRIORITY < DEFAULT_EXT_PRIORITY);
        const _: () = assert!(DEFAULT_EXT_PRIORITY < FALLBACK_EXT_PRIORITY);
    }
    #[test]
    fn test_name_gen_user_names() {
        let mut gen = NameGen::new(vec![Name::str("a"), Name::str("b")], "x");
        assert_eq!(gen.next(), Name::str("a"));
        assert_eq!(gen.next(), Name::str("b"));
        assert_eq!(gen.next(), Name::str("x0"));
    }
    #[test]
    fn test_name_gen_auto() {
        let mut gen = NameGen::new(vec![], "h");
        assert_eq!(gen.next(), Name::str("h0"));
        assert_eq!(gen.next(), Name::str("h1"));
    }
    #[test]
    fn test_name_gen_remaining() {
        let gen = NameGen::new(vec![Name::str("a")], "x");
        assert_eq!(gen.remaining_user_names(), 1);
    }
    #[test]
    fn test_known_struct_prod() {
        let info = known_struct_ext(&Name::str("Prod"));
        assert!(info.is_some());
        assert_eq!(info.expect("info should be valid").num_fields(), 2);
    }
    #[test]
    fn test_known_struct_subtype() {
        let info = known_struct_ext(&Name::str("Subtype"));
        assert!(info.is_some());
        assert_eq!(info.expect("info should be valid").num_fields(), 1);
    }
    #[test]
    fn test_known_struct_unknown() {
        assert!(known_struct_ext(&Name::str("MyCustomType")).is_none());
    }
    #[test]
    fn test_ext_no_goals() {
        let mut ctx = mk_ctx();
        let mut state = TacticState::new(vec![]);
        assert!(tac_ext(&mut state, &mut ctx).is_err());
    }
    #[test]
    fn test_propext_no_goals() {
        let mut ctx = mk_ctx();
        let mut state = TacticState::new(vec![]);
        assert!(tac_propext(&mut state, &mut ctx).is_err());
    }
}
#[cfg(test)]
mod tacticext_analysis_tests {
    use super::*;
    use crate::tactic::ext::*;
    #[test]
    fn test_tacticext_result_ok() {
        let r = TacticExtResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticext_result_err() {
        let r = TacticExtResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticext_result_partial() {
        let r = TacticExtResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_tacticext_result_skipped() {
        let r = TacticExtResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_tacticext_analysis_pass_run() {
        let mut p = TacticExtAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_tacticext_analysis_pass_empty_input() {
        let mut p = TacticExtAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_tacticext_analysis_pass_success_rate() {
        let mut p = TacticExtAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_tacticext_analysis_pass_disable() {
        let mut p = TacticExtAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_tacticext_pipeline_basic() {
        let mut pipeline = TacticExtPipeline::new("main_pipeline");
        pipeline.add_pass(TacticExtAnalysisPass::new("pass1"));
        pipeline.add_pass(TacticExtAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_tacticext_pipeline_disabled_pass() {
        let mut pipeline = TacticExtPipeline::new("partial");
        let mut p = TacticExtAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(TacticExtAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_tacticext_diff_basic() {
        let mut d = TacticExtDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_tacticext_diff_summary() {
        let mut d = TacticExtDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_tacticext_config_set_get() {
        let mut cfg = TacticExtConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_tacticext_config_read_only() {
        let mut cfg = TacticExtConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_tacticext_config_remove() {
        let mut cfg = TacticExtConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_tacticext_diagnostics_basic() {
        let mut diag = TacticExtDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_tacticext_diagnostics_max_errors() {
        let mut diag = TacticExtDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_tacticext_diagnostics_clear() {
        let mut diag = TacticExtDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_tacticext_config_value_types() {
        let b = TacticExtConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = TacticExtConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = TacticExtConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = TacticExtConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = TacticExtConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
