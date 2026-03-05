//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    ConstructorEq, InjectionConfig, InjectionExtConfig1900, InjectionExtConfigVal1900,
    InjectionExtDiag1900, InjectionExtDiff1900, InjectionExtPass1900, InjectionExtPipeline1900,
    InjectionExtResult1900, InjectionResult, InjectionStats, NoConfusionResult, TacInjectBuilder,
    TacInjectCounterMap, TacInjectExtMap, TacInjectExtUtil, TacInjectStateMachine, TacInjectWindow,
    TacInjectWorkQueue, TacticInjectionAnalysisPass, TacticInjectionConfig,
    TacticInjectionConfigValue, TacticInjectionDiagnostics, TacticInjectionDiff,
    TacticInjectionPipeline, TacticInjectionResult,
};
use crate::basic::{MVarId, MetaContext, MetavarKind};
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{BinderInfo, Expr, Level, Name};

/// Maximum number of arguments per constructor for injection.
const MAX_CTOR_ARGS: usize = 64;
/// Maximum recursion depth for nested injection.
pub(super) const MAX_INJECTION_DEPTH: usize = 16;
/// Decompose a hypothesis of the form `C a1 ... = C' b1 ...`.
///
/// Returns the constructor names and their arguments, or None if the
/// hypothesis is not of this form.
pub fn decompose_constructor_eq(expr: &Expr) -> Option<ConstructorEq> {
    let (eq_type, lhs, rhs) = parse_eq_expr(expr)?;
    let (lhs_head, lhs_args) = collect_app_args(&lhs);
    let lhs_ctor = match &lhs_head {
        Expr::Const(name, _) => name.clone(),
        _ => return None,
    };
    let (rhs_head, rhs_args) = collect_app_args(&rhs);
    let rhs_ctor = match &rhs_head {
        Expr::Const(name, _) => name.clone(),
        _ => return None,
    };
    let type_name = infer_type_name_from_ctor(&lhs_ctor);
    Some(ConstructorEq {
        type_name,
        lhs_ctor,
        lhs_args,
        rhs_ctor,
        rhs_args,
        eq_type,
    })
}
/// Parse an expression as `@Eq α lhs rhs`.
pub(super) fn parse_eq_expr(expr: &Expr) -> Option<(Expr, Expr, Expr)> {
    if let Expr::App(f1, rhs) = expr {
        if let Expr::App(f2, lhs) = f1.as_ref() {
            if let Expr::App(eq_const, ty) = f2.as_ref() {
                if let Expr::Const(name, _) = &**eq_const {
                    if name.to_string().contains("Eq") || name.to_string().contains("eq") {
                        return Some((*ty.clone(), *lhs.clone(), *rhs.clone()));
                    }
                }
            }
        }
    }
    None
}
/// Collect application arguments.
pub(super) fn collect_app_args(expr: &Expr) -> (Expr, Vec<Expr>) {
    let mut args = Vec::new();
    let mut head = expr.clone();
    while let Expr::App(f, a) = head {
        args.push(*a);
        head = *f;
    }
    args.reverse();
    (head, args)
}
/// Rebuild an application from head and arguments.
pub(super) fn mk_app(head: Expr, args: Vec<Expr>) -> Expr {
    let mut result = head;
    for arg in args {
        result = Expr::App(Box::new(result), Box::new(arg));
    }
    result
}
/// Infer the inductive type name from a constructor name.
///
/// Uses naming convention: `TypeName.ctorName` -> `TypeName`.
pub(super) fn infer_type_name_from_ctor(ctor: &Name) -> Name {
    let s = ctor.to_string();
    if let Some(dot_pos) = s.rfind('.') {
        Name::str(&s[..dot_pos])
    } else {
        match s.as_str() {
            "zero" | "succ" => Name::str("Nat"),
            "nil" | "cons" => Name::str("List"),
            "true" | "false" => Name::str("Bool"),
            "none" | "some" => Name::str("Option"),
            "inl" | "inr" => Name::str("Sum"),
            "mk" => Name::str("Prod"),
            _ => Name::str("_unknown"),
        }
    }
}
/// Check if two constructor names are the same.
pub(super) fn ctors_equal(a: &Name, b: &Name) -> bool {
    a == b
}
/// Check if an expression is syntactically equal to another.
pub(super) fn exprs_equal(a: &Expr, b: &Expr) -> bool {
    match (a, b) {
        (Expr::Sort(l1), Expr::Sort(l2)) => l1 == l2,
        (Expr::BVar(i1), Expr::BVar(i2)) => i1 == i2,
        (Expr::FVar(id1), Expr::FVar(id2)) => id1 == id2,
        (Expr::Const(n1, ls1), Expr::Const(n2, ls2)) => n1 == n2 && ls1 == ls2,
        (Expr::App(f1, a1), Expr::App(f2, a2)) => exprs_equal(f1, f2) && exprs_equal(a1, a2),
        (Expr::Lam(bi1, n1, t1, b1), Expr::Lam(bi2, n2, t2, b2)) => {
            bi1 == bi2 && n1 == n2 && exprs_equal(t1, t2) && exprs_equal(b1, b2)
        }
        (Expr::Pi(bi1, n1, t1, b1), Expr::Pi(bi2, n2, t2, b2)) => {
            bi1 == bi2 && n1 == n2 && exprs_equal(t1, t2) && exprs_equal(b1, b2)
        }
        (Expr::Lit(l1), Expr::Lit(l2)) => l1 == l2,
        _ => false,
    }
}
/// Build the proof term for constructor injection.
///
/// Given `h : C a1 ... an = C b1 ... bn`, produce proofs of
/// `a1 = b1`, ..., `an = bn` using the injectivity of `C`.
pub fn build_injection_proof(
    ctor: &Name,
    lhs_args: &[Expr],
    rhs_args: &[Expr],
    hyp_name: &Name,
    eq_type: &Expr,
) -> Vec<Expr> {
    let n = lhs_args.len().min(rhs_args.len());
    let mut proofs = Vec::with_capacity(n);
    let type_name = infer_type_name_from_ctor(ctor);
    for i in 0..n {
        let no_confusion_name = type_name.clone().append_str("noConfusion");
        let no_confusion_const = Expr::Const(no_confusion_name, vec![Level::Zero]);
        let motive = build_injection_motive(n, i, lhs_args, rhs_args);
        let hyp_ref = Expr::Const(hyp_name.clone(), vec![]);
        let proof = mk_app(no_confusion_const, vec![eq_type.clone(), motive, hyp_ref]);
        proofs.push(proof);
    }
    proofs
}
/// Build the motive for injection: projects the i-th equality.
///
/// Produces: `fun (h0 : lhs[0] = rhs[0]) ... (hn : lhs[n] = rhs[n]) => h_target_idx`
pub(super) fn build_injection_motive(
    num_args: usize,
    target_idx: usize,
    lhs_args: &[Expr],
    rhs_args: &[Expr],
) -> Expr {
    let mut body = Expr::BVar(num_args as u32 - 1 - target_idx as u32);
    for i in (0..num_args).rev() {
        let name = Name::str(format!("h{}", i));
        let lhs = lhs_args.get(i).cloned().unwrap_or(Expr::BVar(0));
        let rhs = rhs_args.get(i).cloned().unwrap_or(Expr::BVar(0));
        let eq_ty = build_eq_expr(&Expr::Sort(Level::Zero), &lhs, &rhs);
        body = Expr::Lam(BinderInfo::Default, name, Box::new(eq_ty), Box::new(body));
    }
    body
}
/// Build the proof term for no_confusion (distinct constructors).
///
/// Given `h : C1 ... = C2 ...` with `C1 != C2`, produces a proof of `False`.
pub fn build_no_confusion_proof(
    type_name: &Name,
    _lhs_ctor: &Name,
    _rhs_ctor: &Name,
    hyp_name: &Name,
    eq_type: &Expr,
) -> Expr {
    let no_confusion_name = type_name.clone().append_str("noConfusion");
    let no_confusion_const = Expr::Const(no_confusion_name, vec![Level::Zero]);
    let false_type = Expr::Const(Name::str("False"), vec![]);
    let hyp_ref = Expr::Const(hyp_name.clone(), vec![]);
    mk_app(
        no_confusion_const,
        vec![eq_type.clone(), false_type, hyp_ref],
    )
}
/// Build an equality expression `@Eq α a b`.
pub(super) fn build_eq_expr(ty: &Expr, a: &Expr, b: &Expr) -> Expr {
    let eq_const = Expr::Const(Name::str("Eq"), vec![Level::Zero]);
    mk_app(eq_const, vec![ty.clone(), a.clone(), b.clone()])
}
/// Apply the injection tactic to a hypothesis.
///
/// If `hyp` is of the form `C a1 ... an = C b1 ... bn`, introduces
/// `a1 = b1`, ..., `an = bn` as new hypotheses.
pub fn tac_injection(
    hyp: &Name,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<InjectionResult> {
    let config = InjectionConfig::default();
    tac_injection_impl(hyp, &config, state, ctx, 0)
}
/// Apply injection with user-specified names for the new equalities.
pub fn tac_injection_with(
    hyp: &Name,
    names: &[Name],
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<InjectionResult> {
    let config = InjectionConfig::default().with_names(names.to_vec());
    tac_injection_impl(hyp, &config, state, ctx, 0)
}
/// Apply injection with full configuration.
pub fn tac_injection_with_config(
    hyp: &Name,
    config: &InjectionConfig,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<InjectionResult> {
    tac_injection_impl(hyp, config, state, ctx, 0)
}
/// Core injection implementation with depth tracking.
pub(super) fn tac_injection_impl(
    hyp: &Name,
    config: &InjectionConfig,
    state: &mut TacticState,
    ctx: &mut MetaContext,
    depth: usize,
) -> TacticResult<InjectionResult> {
    if depth >= config.max_depth {
        return Err(TacticError::Failed(
            "injection: maximum recursion depth exceeded".to_string(),
        ));
    }
    let _goal = state.current_goal()?;
    let hyp_type = find_hyp_type(hyp, ctx)?;
    let hyp_type = ctx.instantiate_mvars(&hyp_type);
    let ctor_eq = decompose_constructor_eq(&hyp_type).ok_or_else(|| {
        TacticError::GoalMismatch(format!(
            "injection: hypothesis '{}' is not a constructor equality",
            hyp
        ))
    })?;
    if !ctors_equal(&ctor_eq.lhs_ctor, &ctor_eq.rhs_ctor) {
        return Err(TacticError::GoalMismatch(format!(
            "injection: constructors differ ('{}' vs '{}'); use no_confusion instead",
            ctor_eq.lhs_ctor, ctor_eq.rhs_ctor
        )));
    }
    let num_args = ctor_eq.lhs_args.len().min(ctor_eq.rhs_args.len());
    if num_args == 0 {
        return Err(TacticError::Failed(
            "injection: constructor has no arguments".to_string(),
        ));
    }
    if num_args > MAX_CTOR_ARGS {
        return Err(TacticError::Failed(format!(
            "injection: constructor has too many arguments ({})",
            num_args
        )));
    }
    let mut stats = InjectionStats {
        constructors_matched: 1,
        ..InjectionStats::default()
    };
    let mut new_equalities = Vec::new();
    let mut new_goal_ids = Vec::new();
    let mut assigned_names = Vec::new();
    let _proofs = build_injection_proof(
        &ctor_eq.lhs_ctor,
        &ctor_eq.lhs_args,
        &ctor_eq.rhs_args,
        hyp,
        &ctor_eq.eq_type,
    );
    for i in 0..num_args {
        let lhs_arg = &ctor_eq.lhs_args[i];
        let rhs_arg = &ctor_eq.rhs_args[i];
        if exprs_equal(lhs_arg, rhs_arg) {
            continue;
        }
        let eq_name = if i < config.with_names.len() {
            config.with_names[i].clone()
        } else {
            Name::str(format!("h_inj_{}", i))
        };
        let eq_ty = build_eq_expr(&ctor_eq.eq_type, lhs_arg, rhs_arg);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(eq_ty.clone(), MetavarKind::Natural);
        let _fvar_id = ctx.mk_local_decl(eq_name.clone(), eq_ty.clone(), BinderInfo::Default);
        new_equalities.push((eq_name.clone(), eq_ty));
        new_goal_ids.push(mvar_id);
        assigned_names.push(eq_name);
        stats.equalities_produced += 1;
    }
    if config.clear_hyp {
        ctx.clear_local(hyp);
        stats.hypotheses_cleared += 1;
    }
    if config.recurse && depth + 1 < config.max_depth {
        let mut extra_equalities = Vec::new();
        let mut extra_goals = Vec::new();
        for (name, ty) in &new_equalities {
            if decompose_constructor_eq(ty).is_some() {
                match tac_injection_impl(name, config, state, ctx, depth + 1) {
                    Ok(sub_result) => {
                        extra_equalities.extend(sub_result.new_equalities);
                        extra_goals.extend(sub_result.goals_created);
                        stats.recursive_steps += 1;
                    }
                    Err(_) => {}
                }
            }
        }
        new_equalities.extend(extra_equalities);
        new_goal_ids.extend(extra_goals);
    }
    Ok(InjectionResult {
        new_equalities,
        goals_created: new_goal_ids,
        constructor: ctor_eq.lhs_ctor,
        num_args,
        assigned_names,
        stats,
    })
}
/// Apply the no_confusion tactic to a hypothesis.
///
/// If `hyp` is of the form `C1 ... = C2 ...` with `C1 != C2`,
/// closes the goal by contradiction.
pub fn tac_no_confusion(
    hyp: &Name,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<NoConfusionResult> {
    let _goal = state.current_goal()?;
    let hyp_type = find_hyp_type(hyp, ctx)?;
    let hyp_type = ctx.instantiate_mvars(&hyp_type);
    let ctor_eq = decompose_constructor_eq(&hyp_type).ok_or_else(|| {
        TacticError::GoalMismatch(format!(
            "no_confusion: hypothesis '{}' is not a constructor equality",
            hyp
        ))
    })?;
    if ctors_equal(&ctor_eq.lhs_ctor, &ctor_eq.rhs_ctor) {
        return Ok(NoConfusionResult {
            contradicted: false,
            type_name: ctor_eq.type_name,
            lhs_ctor: ctor_eq.lhs_ctor,
            rhs_ctor: ctor_eq.rhs_ctor,
            proof: None,
        });
    }
    let proof = build_no_confusion_proof(
        &ctor_eq.type_name,
        &ctor_eq.lhs_ctor,
        &ctor_eq.rhs_ctor,
        hyp,
        &ctor_eq.eq_type,
    );
    let false_elim = Expr::Const(Name::str("False.elim"), vec![Level::Zero]);
    let goal_mvar = state.current_goal()?;
    let goal_type = ctx
        .get_mvar_type(goal_mvar)
        .cloned()
        .unwrap_or(Expr::Sort(Level::Zero));
    let full_proof = mk_app(false_elim, vec![goal_type, proof.clone()]);
    state.close_goal(full_proof, ctx)?;
    Ok(NoConfusionResult {
        contradicted: true,
        type_name: ctor_eq.type_name,
        lhs_ctor: ctor_eq.lhs_ctor,
        rhs_ctor: ctor_eq.rhs_ctor,
        proof: Some(proof),
    })
}
/// Find a hypothesis by name in the local context.
pub(super) fn find_hyp_type(hyp: &Name, ctx: &MetaContext) -> TacticResult<Expr> {
    let hyps = ctx.get_local_hyps();
    for (name, ty) in hyps {
        if &name == hyp {
            return Ok(ty);
        }
    }
    Err(TacticError::UnknownHyp(hyp.clone()))
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::injection::*;
    use oxilean_kernel::Environment;
    fn mk_test_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    fn mk_const(name: &str) -> Expr {
        Expr::Const(Name::str(name), vec![])
    }
    fn mk_app2(f: Expr, a: Expr, b: Expr) -> Expr {
        Expr::App(Box::new(Expr::App(Box::new(f), Box::new(a))), Box::new(b))
    }
    fn mk_eq_expr(ty: Expr, lhs: Expr, rhs: Expr) -> Expr {
        let eq_const = Expr::Const(Name::str("Eq"), vec![Level::Zero]);
        mk_app(eq_const, vec![ty, lhs, rhs])
    }
    fn mk_ctor_app(ctor: &str, args: Vec<Expr>) -> Expr {
        mk_app(mk_const(ctor), args)
    }
    #[test]
    fn test_decompose_constructor_eq_same_ctor() {
        let nat_type = mk_const("Nat");
        let lhs = mk_ctor_app("Nat.succ", vec![mk_const("a")]);
        let rhs = mk_ctor_app("Nat.succ", vec![mk_const("b")]);
        let eq = mk_eq_expr(nat_type, lhs, rhs);
        let result = decompose_constructor_eq(&eq);
        assert!(result.is_some());
        let ctor_eq = result.expect("ctor_eq should be present");
        assert_eq!(ctor_eq.lhs_ctor, Name::str("Nat.succ"));
        assert_eq!(ctor_eq.rhs_ctor, Name::str("Nat.succ"));
        assert_eq!(ctor_eq.lhs_args.len(), 1);
        assert_eq!(ctor_eq.rhs_args.len(), 1);
    }
    #[test]
    fn test_decompose_constructor_eq_different_ctor() {
        let nat_type = mk_const("Nat");
        let lhs = mk_ctor_app("Nat.zero", vec![]);
        let rhs = mk_ctor_app("Nat.succ", vec![mk_const("n")]);
        let eq = mk_eq_expr(nat_type, lhs, rhs);
        let result = decompose_constructor_eq(&eq);
        assert!(result.is_some());
        let ctor_eq = result.expect("ctor_eq should be present");
        assert_eq!(ctor_eq.lhs_ctor, Name::str("Nat.zero"));
        assert_eq!(ctor_eq.rhs_ctor, Name::str("Nat.succ"));
    }
    #[test]
    fn test_decompose_constructor_eq_multi_arg() {
        let list_type = mk_const("List");
        let lhs = mk_ctor_app("List.cons", vec![mk_const("a"), mk_const("as")]);
        let rhs = mk_ctor_app("List.cons", vec![mk_const("b"), mk_const("bs")]);
        let eq = mk_eq_expr(list_type, lhs, rhs);
        let result = decompose_constructor_eq(&eq);
        assert!(result.is_some());
        let ctor_eq = result.expect("ctor_eq should be present");
        assert_eq!(ctor_eq.lhs_args.len(), 2);
        assert_eq!(ctor_eq.rhs_args.len(), 2);
    }
    #[test]
    fn test_decompose_constructor_eq_not_eq() {
        let expr = mk_const("P");
        assert!(decompose_constructor_eq(&expr).is_none());
    }
    #[test]
    fn test_decompose_constructor_eq_not_ctor() {
        let ty = mk_const("Nat");
        let lhs = Expr::BVar(0);
        let rhs = mk_const("Nat.zero");
        let eq = mk_eq_expr(ty, lhs, rhs);
        assert!(decompose_constructor_eq(&eq).is_none());
    }
    #[test]
    fn test_infer_type_name_dotted() {
        assert_eq!(
            infer_type_name_from_ctor(&Name::str("Nat.succ")),
            Name::str("Nat")
        );
        assert_eq!(
            infer_type_name_from_ctor(&Name::str("List.cons")),
            Name::str("List")
        );
        assert_eq!(
            infer_type_name_from_ctor(&Name::str("Option.some")),
            Name::str("Option")
        );
    }
    #[test]
    fn test_infer_type_name_heuristic() {
        assert_eq!(
            infer_type_name_from_ctor(&Name::str("zero")),
            Name::str("Nat")
        );
        assert_eq!(
            infer_type_name_from_ctor(&Name::str("succ")),
            Name::str("Nat")
        );
        assert_eq!(
            infer_type_name_from_ctor(&Name::str("nil")),
            Name::str("List")
        );
        assert_eq!(
            infer_type_name_from_ctor(&Name::str("cons")),
            Name::str("List")
        );
        assert_eq!(
            infer_type_name_from_ctor(&Name::str("true")),
            Name::str("Bool")
        );
        assert_eq!(
            infer_type_name_from_ctor(&Name::str("false")),
            Name::str("Bool")
        );
        assert_eq!(
            infer_type_name_from_ctor(&Name::str("none")),
            Name::str("Option")
        );
        assert_eq!(
            infer_type_name_from_ctor(&Name::str("some")),
            Name::str("Option")
        );
        assert_eq!(
            infer_type_name_from_ctor(&Name::str("inl")),
            Name::str("Sum")
        );
        assert_eq!(
            infer_type_name_from_ctor(&Name::str("inr")),
            Name::str("Sum")
        );
    }
    #[test]
    fn test_infer_type_name_unknown() {
        assert_eq!(
            infer_type_name_from_ctor(&Name::str("myctor")),
            Name::str("_unknown")
        );
    }
    #[test]
    fn test_injection_config_default() {
        let config = InjectionConfig::default();
        assert!(config.with_names.is_empty());
        assert!(!config.recurse);
        assert!(!config.clear_hyp);
        assert!(!config.subst);
    }
    #[test]
    fn test_injection_config_with_names() {
        let config = InjectionConfig::default().with_names(vec![Name::str("h1"), Name::str("h2")]);
        assert_eq!(config.with_names.len(), 2);
    }
    #[test]
    fn test_injection_config_recursive() {
        let config = InjectionConfig::default().recursive();
        assert!(config.recurse);
    }
    #[test]
    fn test_injection_config_clear() {
        let config = InjectionConfig::default().clear();
        assert!(config.clear_hyp);
    }
    #[test]
    fn test_injection_config_with_subst() {
        let config = InjectionConfig::default().with_subst();
        assert!(config.subst);
    }
    #[test]
    fn test_injection_stats_default() {
        let stats = InjectionStats::default();
        assert_eq!(stats.equalities_produced, 0);
        assert_eq!(stats.constructors_matched, 0);
        assert_eq!(stats.recursive_steps, 0);
        assert_eq!(stats.hypotheses_cleared, 0);
        assert_eq!(stats.substitutions, 0);
    }
    #[test]
    fn test_no_confusion_result_fields() {
        let result = NoConfusionResult {
            contradicted: true,
            type_name: Name::str("Nat"),
            lhs_ctor: Name::str("Nat.zero"),
            rhs_ctor: Name::str("Nat.succ"),
            proof: Some(mk_const("proof")),
        };
        assert!(result.contradicted);
        assert_eq!(result.type_name, Name::str("Nat"));
    }
    #[test]
    fn test_no_confusion_result_not_contradicted() {
        let result = NoConfusionResult {
            contradicted: false,
            type_name: Name::str("Nat"),
            lhs_ctor: Name::str("Nat.succ"),
            rhs_ctor: Name::str("Nat.succ"),
            proof: None,
        };
        assert!(!result.contradicted);
        assert!(result.proof.is_none());
    }
    #[test]
    fn test_injection_result_fields() {
        let result = InjectionResult {
            new_equalities: vec![(Name::str("h1"), mk_const("eq_type"))],
            goals_created: vec![MVarId(0)],
            constructor: Name::str("Nat.succ"),
            num_args: 1,
            assigned_names: vec![Name::str("h1")],
            stats: InjectionStats::default(),
        };
        assert_eq!(result.new_equalities.len(), 1);
        assert_eq!(result.goals_created.len(), 1);
        assert_eq!(result.constructor, Name::str("Nat.succ"));
    }
    #[test]
    fn test_build_injection_proof_single_arg() {
        let ctor = Name::str("Nat.succ");
        let lhs_args = vec![mk_const("a")];
        let rhs_args = vec![mk_const("b")];
        let hyp_name = Name::str("h");
        let eq_type = mk_const("Nat");
        let proofs = build_injection_proof(&ctor, &lhs_args, &rhs_args, &hyp_name, &eq_type);
        assert_eq!(proofs.len(), 1);
    }
    #[test]
    fn test_build_injection_proof_multi_arg() {
        let ctor = Name::str("List.cons");
        let lhs_args = vec![mk_const("a"), mk_const("as")];
        let rhs_args = vec![mk_const("b"), mk_const("bs")];
        let hyp_name = Name::str("h");
        let eq_type = mk_const("List");
        let proofs = build_injection_proof(&ctor, &lhs_args, &rhs_args, &hyp_name, &eq_type);
        assert_eq!(proofs.len(), 2);
    }
    #[test]
    fn test_build_injection_proof_empty() {
        let ctor = Name::str("Nat.zero");
        let lhs_args: Vec<Expr> = vec![];
        let rhs_args: Vec<Expr> = vec![];
        let hyp_name = Name::str("h");
        let eq_type = mk_const("Nat");
        let proofs = build_injection_proof(&ctor, &lhs_args, &rhs_args, &hyp_name, &eq_type);
        assert!(proofs.is_empty());
    }
    #[test]
    fn test_build_no_confusion_proof() {
        let proof = build_no_confusion_proof(
            &Name::str("Nat"),
            &Name::str("Nat.zero"),
            &Name::str("Nat.succ"),
            &Name::str("h"),
            &mk_const("Nat"),
        );
        assert!(matches!(proof, Expr::App(_, _)));
    }
    #[test]
    fn test_build_eq_expr() {
        let nat = mk_const("Nat");
        let a = mk_const("a");
        let b = mk_const("b");
        let eq = build_eq_expr(&nat, &a, &b);
        assert!(matches!(eq, Expr::App(_, _)));
    }
    #[test]
    fn test_build_injection_motive_single() {
        let lhs = vec![mk_const("a")];
        let rhs = vec![mk_const("b")];
        let motive = build_injection_motive(1, 0, &lhs, &rhs);
        assert!(matches!(motive, Expr::Lam(_, _, _, _)));
    }
    #[test]
    fn test_build_injection_motive_multi() {
        let lhs = vec![mk_const("a"), mk_const("b")];
        let rhs = vec![mk_const("c"), mk_const("d")];
        let motive = build_injection_motive(2, 1, &lhs, &rhs);
        assert!(matches!(motive, Expr::Lam(_, _, _, _)));
    }
    #[test]
    fn test_constructor_eq_fields() {
        let ctor_eq = ConstructorEq {
            type_name: Name::str("Nat"),
            lhs_ctor: Name::str("Nat.succ"),
            lhs_args: vec![mk_const("a")],
            rhs_ctor: Name::str("Nat.succ"),
            rhs_args: vec![mk_const("b")],
            eq_type: mk_const("Nat"),
        };
        assert_eq!(ctor_eq.type_name, Name::str("Nat"));
        assert_eq!(ctor_eq.lhs_args.len(), 1);
        assert_eq!(ctor_eq.rhs_args.len(), 1);
    }
    #[test]
    fn test_ctors_equal_same() {
        assert!(ctors_equal(&Name::str("Nat.succ"), &Name::str("Nat.succ")));
    }
    #[test]
    fn test_ctors_equal_different() {
        assert!(!ctors_equal(&Name::str("Nat.zero"), &Name::str("Nat.succ")));
    }
    #[test]
    fn test_exprs_equal_const() {
        assert!(exprs_equal(&mk_const("x"), &mk_const("x")));
        assert!(!exprs_equal(&mk_const("x"), &mk_const("y")));
    }
    #[test]
    fn test_exprs_equal_bvar() {
        assert!(exprs_equal(&Expr::BVar(0), &Expr::BVar(0)));
        assert!(!exprs_equal(&Expr::BVar(0), &Expr::BVar(1)));
    }
    #[test]
    fn test_exprs_equal_app() {
        let e1 = mk_app2(mk_const("f"), mk_const("a"), mk_const("b"));
        let e2 = mk_app2(mk_const("f"), mk_const("a"), mk_const("b"));
        assert!(exprs_equal(&e1, &e2));
    }
    #[test]
    fn test_parse_eq_expr_valid() {
        let ty = mk_const("Nat");
        let lhs = mk_const("a");
        let rhs = mk_const("b");
        let eq = mk_eq_expr(ty.clone(), lhs.clone(), rhs.clone());
        let result = parse_eq_expr(&eq);
        assert!(result.is_some());
        let (parsed_ty, parsed_lhs, parsed_rhs) = result.expect("result should be valid");
        assert!(exprs_equal(&parsed_ty, &ty));
        assert!(exprs_equal(&parsed_lhs, &lhs));
        assert!(exprs_equal(&parsed_rhs, &rhs));
    }
    #[test]
    fn test_parse_eq_expr_invalid() {
        assert!(parse_eq_expr(&mk_const("P")).is_none());
        assert!(parse_eq_expr(&Expr::BVar(0)).is_none());
    }
    #[test]
    fn test_collect_app_args_roundtrip() {
        let f = mk_const("f");
        let a = mk_const("a");
        let b = mk_const("b");
        let expr = mk_app2(f.clone(), a.clone(), b.clone());
        let (head, args) = collect_app_args(&expr);
        let rebuilt = mk_app(head, args);
        assert!(exprs_equal(&expr, &rebuilt));
    }
    #[test]
    fn test_tac_injection_unknown_hyp() {
        let mut ctx = mk_test_ctx();
        let ty = Expr::Sort(Level::Zero);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let result = tac_injection(&Name::str("nonexistent"), &mut state, &mut ctx);
        assert!(result.is_err());
    }
    #[test]
    fn test_tac_no_confusion_unknown_hyp() {
        let mut ctx = mk_test_ctx();
        let ty = Expr::Sort(Level::Zero);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let result = tac_no_confusion(&Name::str("nonexistent"), &mut state, &mut ctx);
        assert!(result.is_err());
    }
    #[test]
    fn test_tac_injection_non_constructor_eq() {
        let mut ctx = mk_test_ctx();
        let ty = mk_const("P");
        ctx.mk_local_decl(Name::str("h"), ty, BinderInfo::Default);
        let goal_ty = Expr::Sort(Level::Zero);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let result = tac_injection(&Name::str("h"), &mut state, &mut ctx);
        assert!(result.is_err());
    }
    #[test]
    fn test_tac_injection_same_ctor() {
        let mut ctx = mk_test_ctx();
        let nat = mk_const("Nat");
        let lhs = mk_ctor_app("Nat.succ", vec![mk_const("a")]);
        let rhs = mk_ctor_app("Nat.succ", vec![mk_const("b")]);
        let hyp_ty = mk_eq_expr(nat, lhs, rhs);
        ctx.mk_local_decl(Name::str("h"), hyp_ty, BinderInfo::Default);
        let goal_ty = Expr::Sort(Level::Zero);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let result = tac_injection(&Name::str("h"), &mut state, &mut ctx);
        assert!(result.is_ok());
        let inj_result = result.expect("inj_result should be present");
        assert_eq!(inj_result.constructor, Name::str("Nat.succ"));
        assert!(inj_result.stats.equalities_produced > 0);
    }
    #[test]
    fn test_tac_injection_different_ctor_fails() {
        let mut ctx = mk_test_ctx();
        let nat = mk_const("Nat");
        let lhs = mk_ctor_app("Nat.zero", vec![]);
        let rhs = mk_ctor_app("Nat.succ", vec![mk_const("n")]);
        let hyp_ty = mk_eq_expr(nat, lhs, rhs);
        ctx.mk_local_decl(Name::str("h"), hyp_ty, BinderInfo::Default);
        let goal_ty = Expr::Sort(Level::Zero);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let result = tac_injection(&Name::str("h"), &mut state, &mut ctx);
        assert!(result.is_err());
    }
    #[test]
    fn test_tac_no_confusion_different_ctor() {
        let mut ctx = mk_test_ctx();
        let nat = mk_const("Nat");
        let lhs = mk_ctor_app("Nat.zero", vec![]);
        let rhs = mk_ctor_app("Nat.succ", vec![mk_const("n")]);
        let hyp_ty = mk_eq_expr(nat, lhs, rhs);
        ctx.mk_local_decl(Name::str("h"), hyp_ty, BinderInfo::Default);
        let goal_ty = Expr::Sort(Level::Zero);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let result = tac_no_confusion(&Name::str("h"), &mut state, &mut ctx);
        assert!(result.is_ok());
        let nc_result = result.expect("nc_result should be present");
        assert!(nc_result.contradicted);
        assert!(nc_result.proof.is_some());
    }
    #[test]
    fn test_tac_no_confusion_same_ctor() {
        let mut ctx = mk_test_ctx();
        let nat = mk_const("Nat");
        let lhs = mk_ctor_app("Nat.succ", vec![mk_const("a")]);
        let rhs = mk_ctor_app("Nat.succ", vec![mk_const("b")]);
        let hyp_ty = mk_eq_expr(nat, lhs, rhs);
        ctx.mk_local_decl(Name::str("h"), hyp_ty, BinderInfo::Default);
        let goal_ty = Expr::Sort(Level::Zero);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let result = tac_no_confusion(&Name::str("h"), &mut state, &mut ctx);
        assert!(result.is_ok());
        let nc_result = result.expect("nc_result should be present");
        assert!(!nc_result.contradicted);
    }
    #[test]
    fn test_tac_injection_with_names() {
        let mut ctx = mk_test_ctx();
        let list_ty = mk_const("List");
        let lhs = mk_ctor_app("List.cons", vec![mk_const("a"), mk_const("as")]);
        let rhs = mk_ctor_app("List.cons", vec![mk_const("b"), mk_const("bs")]);
        let hyp_ty = mk_eq_expr(list_ty, lhs, rhs);
        ctx.mk_local_decl(Name::str("h"), hyp_ty, BinderInfo::Default);
        let goal_ty = Expr::Sort(Level::Zero);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let names = vec![Name::str("h_head"), Name::str("h_tail")];
        let result = tac_injection_with(&Name::str("h"), &names, &mut state, &mut ctx);
        assert!(result.is_ok());
        let inj_result = result.expect("inj_result should be present");
        assert_eq!(inj_result.assigned_names.len(), 2);
        assert_eq!(inj_result.assigned_names[0], Name::str("h_head"));
        assert_eq!(inj_result.assigned_names[1], Name::str("h_tail"));
    }
}
#[cfg(test)]
mod tacinject_ext2_tests {
    use super::*;
    use crate::tactic::injection::*;
    #[test]
    fn test_tacinject_ext_util_basic() {
        let mut u = TacInjectExtUtil::new("test");
        u.push(10);
        u.push(20);
        assert_eq!(u.sum(), 30);
        assert_eq!(u.len(), 2);
    }
    #[test]
    fn test_tacinject_ext_util_min_max() {
        let mut u = TacInjectExtUtil::new("mm");
        u.push(5);
        u.push(1);
        u.push(9);
        assert_eq!(u.min_val(), Some(1));
        assert_eq!(u.max_val(), Some(9));
    }
    #[test]
    fn test_tacinject_ext_util_flags() {
        let mut u = TacInjectExtUtil::new("flags");
        u.set_flag(3);
        assert!(u.has_flag(3));
        assert!(!u.has_flag(2));
    }
    #[test]
    fn test_tacinject_ext_util_pop() {
        let mut u = TacInjectExtUtil::new("pop");
        u.push(42);
        assert_eq!(u.pop(), Some(42));
        assert!(u.is_empty());
    }
    #[test]
    fn test_tacinject_ext_map_basic() {
        let mut m: TacInjectExtMap<i32> = TacInjectExtMap::new();
        m.insert("key", 42);
        assert_eq!(m.get("key"), Some(&42));
        assert!(m.contains("key"));
        assert!(!m.contains("other"));
    }
    #[test]
    fn test_tacinject_ext_map_get_or_default() {
        let mut m: TacInjectExtMap<i32> = TacInjectExtMap::new();
        m.insert("k", 5);
        assert_eq!(m.get_or_default("k"), 5);
        assert_eq!(m.get_or_default("missing"), 0);
    }
    #[test]
    fn test_tacinject_ext_map_keys_sorted() {
        let mut m: TacInjectExtMap<i32> = TacInjectExtMap::new();
        m.insert("z", 1);
        m.insert("a", 2);
        m.insert("m", 3);
        let keys = m.keys_sorted();
        assert_eq!(keys[0].as_str(), "a");
        assert_eq!(keys[2].as_str(), "z");
    }
    #[test]
    fn test_tacinject_window_mean() {
        let mut w = TacInjectWindow::new(3);
        w.push(1.0);
        w.push(2.0);
        w.push(3.0);
        assert!((w.mean() - 2.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacinject_window_evict() {
        let mut w = TacInjectWindow::new(2);
        w.push(10.0);
        w.push(20.0);
        w.push(30.0);
        assert_eq!(w.len(), 2);
        assert!((w.mean() - 25.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacinject_window_std_dev() {
        let mut w = TacInjectWindow::new(10);
        for i in 0..10 {
            w.push(i as f64);
        }
        assert!(w.std_dev() > 0.0);
    }
    #[test]
    fn test_tacinject_builder_basic() {
        let b = TacInjectBuilder::new("test")
            .add_item("a")
            .add_item("b")
            .set_config("key", "val");
        assert_eq!(b.item_count(), 2);
        assert!(b.has_config("key"));
        assert_eq!(b.get_config("key"), Some("val"));
    }
    #[test]
    fn test_tacinject_builder_summary() {
        let b = TacInjectBuilder::new("suite").add_item("x");
        let s = b.build_summary();
        assert!(s.contains("suite"));
    }
    #[test]
    fn test_tacinject_state_machine_start() {
        let mut sm = TacInjectStateMachine::new();
        assert!(sm.start());
        assert!(sm.state.is_running());
    }
    #[test]
    fn test_tacinject_state_machine_complete() {
        let mut sm = TacInjectStateMachine::new();
        sm.start();
        sm.complete();
        assert!(sm.state.is_terminal());
    }
    #[test]
    fn test_tacinject_state_machine_fail() {
        let mut sm = TacInjectStateMachine::new();
        sm.fail("oops");
        assert!(sm.state.is_terminal());
        assert_eq!(sm.state.error_msg(), Some("oops"));
    }
    #[test]
    fn test_tacinject_state_machine_no_transition_after_terminal() {
        let mut sm = TacInjectStateMachine::new();
        sm.complete();
        assert!(!sm.start());
    }
    #[test]
    fn test_tacinject_work_queue_basic() {
        let mut wq = TacInjectWorkQueue::new(10);
        wq.enqueue("task1".to_string());
        wq.enqueue("task2".to_string());
        assert_eq!(wq.pending_count(), 2);
        let t = wq.dequeue();
        assert_eq!(t, Some("task1".to_string()));
        assert_eq!(wq.processed_count(), 1);
    }
    #[test]
    fn test_tacinject_work_queue_capacity() {
        let mut wq = TacInjectWorkQueue::new(2);
        wq.enqueue("a".to_string());
        wq.enqueue("b".to_string());
        assert!(wq.is_full());
        assert!(!wq.enqueue("c".to_string()));
    }
    #[test]
    fn test_tacinject_counter_map_basic() {
        let mut cm = TacInjectCounterMap::new();
        cm.increment("apple");
        cm.increment("apple");
        cm.increment("banana");
        assert_eq!(cm.count("apple"), 2);
        assert_eq!(cm.count("banana"), 1);
        assert_eq!(cm.num_unique(), 2);
    }
    #[test]
    fn test_tacinject_counter_map_frequency() {
        let mut cm = TacInjectCounterMap::new();
        cm.increment("a");
        cm.increment("a");
        cm.increment("b");
        assert!((cm.frequency("a") - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_tacinject_counter_map_most_common() {
        let mut cm = TacInjectCounterMap::new();
        cm.increment("x");
        cm.increment("y");
        cm.increment("x");
        let (k, v) = cm.most_common().expect("most_common should succeed");
        assert_eq!(k.as_str(), "x");
        assert_eq!(v, 2);
    }
}
#[cfg(test)]
mod tacticinjection_analysis_tests {
    use super::*;
    use crate::tactic::injection::*;
    #[test]
    fn test_tacticinjection_result_ok() {
        let r = TacticInjectionResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticinjection_result_err() {
        let r = TacticInjectionResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticinjection_result_partial() {
        let r = TacticInjectionResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_tacticinjection_result_skipped() {
        let r = TacticInjectionResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_tacticinjection_analysis_pass_run() {
        let mut p = TacticInjectionAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_tacticinjection_analysis_pass_empty_input() {
        let mut p = TacticInjectionAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_tacticinjection_analysis_pass_success_rate() {
        let mut p = TacticInjectionAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_tacticinjection_analysis_pass_disable() {
        let mut p = TacticInjectionAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_tacticinjection_pipeline_basic() {
        let mut pipeline = TacticInjectionPipeline::new("main_pipeline");
        pipeline.add_pass(TacticInjectionAnalysisPass::new("pass1"));
        pipeline.add_pass(TacticInjectionAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_tacticinjection_pipeline_disabled_pass() {
        let mut pipeline = TacticInjectionPipeline::new("partial");
        let mut p = TacticInjectionAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(TacticInjectionAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_tacticinjection_diff_basic() {
        let mut d = TacticInjectionDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_tacticinjection_diff_summary() {
        let mut d = TacticInjectionDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_tacticinjection_config_set_get() {
        let mut cfg = TacticInjectionConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_tacticinjection_config_read_only() {
        let mut cfg = TacticInjectionConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_tacticinjection_config_remove() {
        let mut cfg = TacticInjectionConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_tacticinjection_diagnostics_basic() {
        let mut diag = TacticInjectionDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_tacticinjection_diagnostics_max_errors() {
        let mut diag = TacticInjectionDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_tacticinjection_diagnostics_clear() {
        let mut diag = TacticInjectionDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_tacticinjection_config_value_types() {
        let b = TacticInjectionConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = TacticInjectionConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = TacticInjectionConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = TacticInjectionConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = TacticInjectionConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod injection_ext_tests_1900 {
    use super::*;
    use crate::tactic::injection::*;
    #[test]
    fn test_injection_ext_result_ok_1900() {
        let r = InjectionExtResult1900::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_injection_ext_result_err_1900() {
        let r = InjectionExtResult1900::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_injection_ext_result_partial_1900() {
        let r = InjectionExtResult1900::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_injection_ext_result_skipped_1900() {
        let r = InjectionExtResult1900::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_injection_ext_pass_run_1900() {
        let mut p = InjectionExtPass1900::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_injection_ext_pass_empty_1900() {
        let mut p = InjectionExtPass1900::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_injection_ext_pass_rate_1900() {
        let mut p = InjectionExtPass1900::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_injection_ext_pass_disable_1900() {
        let mut p = InjectionExtPass1900::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_injection_ext_pipeline_basic_1900() {
        let mut pipeline = InjectionExtPipeline1900::new("main_pipeline");
        pipeline.add_pass(InjectionExtPass1900::new("pass1"));
        pipeline.add_pass(InjectionExtPass1900::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_injection_ext_pipeline_disabled_1900() {
        let mut pipeline = InjectionExtPipeline1900::new("partial");
        let mut p = InjectionExtPass1900::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(InjectionExtPass1900::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_injection_ext_diff_basic_1900() {
        let mut d = InjectionExtDiff1900::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_injection_ext_config_set_get_1900() {
        let mut cfg = InjectionExtConfig1900::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_injection_ext_config_read_only_1900() {
        let mut cfg = InjectionExtConfig1900::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_injection_ext_config_remove_1900() {
        let mut cfg = InjectionExtConfig1900::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_injection_ext_diagnostics_basic_1900() {
        let mut diag = InjectionExtDiag1900::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_injection_ext_diagnostics_max_errors_1900() {
        let mut diag = InjectionExtDiag1900::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_injection_ext_diagnostics_clear_1900() {
        let mut diag = InjectionExtDiag1900::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_injection_ext_config_value_types_1900() {
        let b = InjectionExtConfigVal1900::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = InjectionExtConfigVal1900::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = InjectionExtConfigVal1900::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = InjectionExtConfigVal1900::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = InjectionExtConfigVal1900::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
