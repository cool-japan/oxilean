//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    CalcExtConfig1600, CalcExtConfigVal1600, CalcExtDiag1600, CalcExtDiff1600, CalcExtPass1600,
    CalcExtPipeline1600, CalcExtResult1600, CalcProof, CalcStep, ConvSide, RelationKind,
    TacCalcBuilder, TacCalcCounterMap, TacCalcExtMap, TacCalcExtUtil, TacCalcStateMachine,
    TacCalcWindow, TacCalcWorkQueue, TacticCalcAnalysisPass, TacticCalcConfig,
    TacticCalcConfigValue, TacticCalcDiagnostics, TacticCalcDiff, TacticCalcPipeline,
    TacticCalcResult, TypedCalcChain, TypedCalcStep,
};
use crate::basic::{MVarId, MetaContext, MetavarKind};
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{Expr, Level, Name};

/// Apply a calc proof to close the current goal.
pub fn tac_calc(
    calc_proof: &CalcProof,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<()> {
    let proof = calc_proof.build_proof()?;
    let goal = state.current_goal()?;
    ctx.assign_mvar(goal, proof.clone());
    state.close_goal(proof, ctx)?;
    Ok(())
}
/// Enter conv mode on the current goal.
///
/// The goal must be of the form `lhs R rhs` (typically `lhs = rhs`).
/// Returns the focused subexpression and a way to reconstruct.
pub fn enter_conv(
    side: ConvSide,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<(MVarId, Expr)> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let (lhs, rhs) = parse_eq_goal(&target)?;
    let focused = match side {
        ConvSide::Lhs => lhs,
        ConvSide::Rhs => rhs,
    };
    let (new_id, _new_expr) = ctx.mk_fresh_expr_mvar(focused.clone(), MetavarKind::Natural);
    Ok((new_id, focused))
}
/// Build a transitivity proof.
///
/// Dispatches to the appropriate transitivity lemma based on the two relations
/// being composed.  Standard cases:
/// - Eq + Eq  → `Eq.trans`
/// - Le + Le  → `le_trans`
/// - Lt + Lt  → `lt_trans`
/// - Lt + Le  → `lt_of_lt_of_le`
/// - Le + Lt  → `lt_of_le_of_lt`
#[allow(clippy::too_many_arguments)]
pub(super) fn build_trans(
    _ty: &Expr,
    _a: &Expr,
    _b: &Expr,
    _c: &Expr,
    proof1: Expr,
    proof2: Expr,
    rel1: &Name,
    rel2: &Name,
) -> Expr {
    let r1 = rel1.to_string();
    let r2 = rel2.to_string();
    let trans_name = match (r1.as_str(), r2.as_str()) {
        ("Eq", "Eq") | ("eq", "eq") => "Eq.trans",
        ("LE.le", "LE.le") | ("le", "le") | ("Le", "Le") => "le_trans",
        ("LT.lt", "LT.lt") | ("lt", "lt") | ("Lt", "Lt") => "lt_trans",
        ("LT.lt", "LE.le") | ("lt", "le") | ("Lt", "Le") => "lt_of_lt_of_le",
        ("LE.le", "LT.lt") | ("le", "lt") | ("Le", "Lt") => "lt_of_le_of_lt",
        _ => "Eq.trans",
    };
    let trans = Expr::Const(Name::str(trans_name), vec![Level::zero()]);
    Expr::App(
        Box::new(Expr::App(Box::new(trans), Box::new(proof1))),
        Box::new(proof2),
    )
}
/// Parse an equality goal.
pub(super) fn parse_eq_goal(expr: &Expr) -> TacticResult<(Expr, Expr)> {
    if let Expr::App(eq_a_lhs, rhs) = expr {
        if let Expr::App(eq_a, lhs) = eq_a_lhs.as_ref() {
            if let Expr::App(eq_const, _alpha) = eq_a.as_ref() {
                if matches!(
                    eq_const.as_ref(), Expr::Const(name, _) if * name == Name::str("Eq")
                ) {
                    return Ok((*lhs.clone(), *rhs.clone()));
                }
            }
        }
    }
    Err(TacticError::GoalMismatch(
        "conv: goal is not an equality".into(),
    ))
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::calc::*;
    use oxilean_kernel::Environment;
    fn _mk_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    fn nat_ty() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    #[test]
    fn test_calc_proof_new() {
        let start = Expr::Const(Name::str("a"), vec![]);
        let calc = CalcProof::new(start.clone(), nat_ty());
        assert_eq!(calc.num_steps(), 0);
        assert_eq!(calc.current(), &start);
    }
    #[test]
    fn test_calc_proof_add_step() {
        let start = Expr::Const(Name::str("a"), vec![]);
        let mut calc = CalcProof::new(start, nat_ty());
        let b = Expr::Const(Name::str("b"), vec![]);
        calc.add_step(CalcStep {
            relation: Name::str("Eq"),
            rhs: b.clone(),
            proof: Expr::Const(Name::str("h1"), vec![]),
        });
        assert_eq!(calc.num_steps(), 1);
        assert_eq!(calc.current(), &b);
    }
    #[test]
    fn test_calc_proof_single_step() {
        let start = Expr::Const(Name::str("a"), vec![]);
        let mut calc = CalcProof::new(start, nat_ty());
        let proof = Expr::Const(Name::str("h1"), vec![]);
        calc.add_step(CalcStep {
            relation: Name::str("Eq"),
            rhs: Expr::Const(Name::str("b"), vec![]),
            proof: proof.clone(),
        });
        let result = calc.build_proof().expect("result should be present");
        assert_eq!(result, proof);
    }
    #[test]
    fn test_calc_proof_multi_step() {
        let start = Expr::Const(Name::str("a"), vec![]);
        let mut calc = CalcProof::new(start, nat_ty());
        calc.add_step(CalcStep {
            relation: Name::str("Eq"),
            rhs: Expr::Const(Name::str("b"), vec![]),
            proof: Expr::Const(Name::str("h1"), vec![]),
        });
        calc.add_step(CalcStep {
            relation: Name::str("Eq"),
            rhs: Expr::Const(Name::str("c"), vec![]),
            proof: Expr::Const(Name::str("h2"), vec![]),
        });
        let result = calc.build_proof().expect("result should be present");
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_calc_proof_empty() {
        let start = Expr::Const(Name::str("a"), vec![]);
        let calc = CalcProof::new(start, nat_ty());
        assert!(calc.build_proof().is_err());
    }
    #[test]
    fn test_parse_eq_goal() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let eq_goal = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Eq"), vec![Level::zero()])),
                    Box::new(nat_ty()),
                )),
                Box::new(a.clone()),
            )),
            Box::new(b.clone()),
        );
        let (lhs, rhs) = parse_eq_goal(&eq_goal).expect("value should be present");
        assert_eq!(lhs, a);
        assert_eq!(rhs, b);
    }
    #[test]
    fn test_parse_eq_goal_non_eq() {
        let expr = Expr::Const(Name::str("Nat"), vec![]);
        assert!(parse_eq_goal(&expr).is_err());
    }
    #[test]
    fn test_conv_side() {
        let _lhs = ConvSide::Lhs;
        let _rhs = ConvSide::Rhs;
    }
}
/// Normalize a calc chain by merging consecutive Eq steps.
pub fn normalize_calc_chain(chain: &TypedCalcChain) -> TypedCalcChain {
    if chain.steps.len() < 2 {
        return chain.clone();
    }
    let mut result = TypedCalcChain::new(chain.start.clone(), chain.ty.clone());
    let mut i = 0;
    while i < chain.steps.len() {
        let step = &chain.steps[i];
        if step.kind == RelationKind::Eq && i + 1 < chain.steps.len() {
            let next = &chain.steps[i + 1];
            if next.kind == RelationKind::Eq {
                let merged_proof = Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Eq.trans"), vec![Level::zero()])),
                        Box::new(step.proof.clone()),
                    )),
                    Box::new(next.proof.clone()),
                );
                result.steps.push(TypedCalcStep {
                    kind: RelationKind::Eq,
                    rhs: next.rhs.clone(),
                    proof: merged_proof,
                    annotation: None,
                });
                i += 2;
                continue;
            }
        }
        result.steps.push(step.clone());
        i += 1;
    }
    result
}
/// Check if a calc chain is trivially closed.
pub fn is_trivial_chain(chain: &TypedCalcChain) -> bool {
    chain.steps.is_empty() || chain.current() == &chain.start
}
/// Pretty-print a TypedCalcChain as a human-readable string.
pub fn pretty_print_chain(chain: &TypedCalcChain) -> String {
    let mut out = format!("{:?}", chain.start);
    for step in &chain.steps {
        let sym = match &step.kind {
            RelationKind::Eq => "= ",
            RelationKind::Le => "<= ",
            RelationKind::Lt => "< ",
            RelationKind::Ge => ">= ",
            RelationKind::Gt => "> ",
            RelationKind::Iff => "<-> ",
            RelationKind::Custom(_) => "~ ",
        };
        let ann = step.annotation.as_deref().unwrap_or("_");
        out.push_str(&format!("\n  {}{:?}  by {}", sym, step.rhs, ann));
    }
    out
}
/// Reverse a calc chain (only valid if all steps are Eq).
pub fn reverse_eq_chain(chain: &TypedCalcChain) -> Option<TypedCalcChain> {
    if chain.steps.iter().any(|s| s.kind != RelationKind::Eq) {
        return None;
    }
    let mut new_steps: Vec<TypedCalcStep> = Vec::new();
    let exprs: Vec<&Expr> = std::iter::once(&chain.start)
        .chain(chain.steps.iter().map(|s| &s.rhs))
        .collect();
    for i in (0..chain.steps.len()).rev() {
        let symm_proof = Expr::App(
            Box::new(Expr::Const(Name::str("Eq.symm"), vec![Level::zero()])),
            Box::new(chain.steps[i].proof.clone()),
        );
        new_steps.push(TypedCalcStep {
            kind: RelationKind::Eq,
            rhs: exprs[i].clone(),
            proof: symm_proof,
            annotation: None,
        });
    }
    let new_start = chain.current().clone();
    let mut result = TypedCalcChain::new(new_start, chain.ty.clone());
    result.steps = new_steps;
    Some(result)
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    use crate::tactic::calc::*;
    use oxilean_kernel::Environment;
    fn _mk_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    fn cnst(s: &str) -> Expr {
        Expr::Const(Name::str(s), vec![])
    }
    #[test]
    fn test_relation_kind_from_name_eq() {
        assert_eq!(RelationKind::from_name(&Name::str("Eq")), RelationKind::Eq);
    }
    #[test]
    fn test_relation_kind_from_name_le() {
        assert_eq!(
            RelationKind::from_name(&Name::str("LE.le")),
            RelationKind::Le
        );
    }
    #[test]
    fn test_trans_lemma_eq_eq() {
        assert_eq!(
            RelationKind::Eq.trans_lemma(&RelationKind::Eq),
            Some(Name::str("Eq.trans"))
        );
    }
    #[test]
    fn test_typed_chain_single_step() {
        let chain = TypedCalcChain::new(cnst("a"), cnst("Nat")).step(
            RelationKind::Eq,
            cnst("b"),
            cnst("h1"),
        );
        assert_eq!(chain.build().expect("build should succeed"), cnst("h1"));
    }
    #[test]
    fn test_typed_chain_two_steps() {
        let chain = TypedCalcChain::new(cnst("a"), cnst("Nat"))
            .step(RelationKind::Eq, cnst("b"), cnst("h1"))
            .step(RelationKind::Eq, cnst("c"), cnst("h2"));
        assert!(matches!(
            chain.build().expect("build should succeed"),
            Expr::App(_, _)
        ));
    }
    #[test]
    fn test_typed_chain_empty_error() {
        assert!(TypedCalcChain::new(cnst("a"), cnst("Nat")).build().is_err());
    }
    #[test]
    fn test_overall_relation_weakest() {
        let chain = TypedCalcChain::new(cnst("a"), cnst("Nat"))
            .step(RelationKind::Eq, cnst("b"), cnst("h1"))
            .step(RelationKind::Le, cnst("c"), cnst("h2"));
        assert_eq!(chain.overall_relation(), Some(RelationKind::Le));
    }
    #[test]
    fn test_normalize_calc_chain_merge() {
        let chain = TypedCalcChain::new(cnst("a"), cnst("Nat"))
            .step(RelationKind::Eq, cnst("b"), cnst("h1"))
            .step(RelationKind::Eq, cnst("c"), cnst("h2"))
            .step(RelationKind::Le, cnst("d"), cnst("h3"));
        let normalized = normalize_calc_chain(&chain);
        assert_eq!(normalized.len(), 2);
    }
    #[test]
    fn test_is_trivial_chain_empty() {
        assert!(is_trivial_chain(&TypedCalcChain::new(
            cnst("a"),
            cnst("Nat")
        )));
    }
    #[test]
    fn test_relation_strength_ordering() {
        assert!(RelationKind::Eq.strength() > RelationKind::Le.strength());
        assert!(RelationKind::Le.strength() > RelationKind::Lt.strength());
    }
    #[test]
    fn test_reverse_eq_chain() {
        let chain = TypedCalcChain::new(cnst("a"), cnst("Nat"))
            .step(RelationKind::Eq, cnst("b"), cnst("h1"))
            .step(RelationKind::Eq, cnst("c"), cnst("h2"));
        let reversed = reverse_eq_chain(&chain).expect("reversed should be present");
        assert_eq!(*reversed.current(), cnst("a"));
    }
    #[test]
    fn test_reverse_non_eq_chain_none() {
        let chain = TypedCalcChain::new(cnst("a"), cnst("Nat")).step(
            RelationKind::Le,
            cnst("b"),
            cnst("h1"),
        );
        assert!(reverse_eq_chain(&chain).is_none());
    }
    #[test]
    fn test_pretty_print_chain() {
        let chain = TypedCalcChain::new(cnst("a"), cnst("Nat")).step_ann(
            RelationKind::Eq,
            cnst("b"),
            cnst("h1"),
            "proof1",
        );
        let s = pretty_print_chain(&chain);
        assert!(s.contains("= ") && s.contains("proof1"));
    }
}
/// Check if a calc step's proof is a trivial refl proof.
pub fn is_refl_step(step: &TypedCalcStep) -> bool {
    matches!(
        & step.proof, Expr::Const(n, _) if format!("{}", n) == "refl" || format!("{}", n)
        == "Eq.refl"
    )
}
/// Extract the lhs and rhs of an equality from a CalcProof.
pub fn calc_proof_endpoints(proof: &CalcProof) -> (&Expr, &Expr) {
    let end = proof.current();
    (&proof.start, end)
}
/// Count the number of steps in a CalcProof.
pub fn calc_proof_depth(proof: &CalcProof) -> usize {
    proof.num_steps()
}
/// Validate that a TypedCalcChain connects start to end correctly.
pub fn validate_chain_endpoints(chain: &TypedCalcChain, expected_end: &Expr) -> bool {
    chain.current() == expected_end
}
/// Create a single equality step.
pub fn eq_step(_lhs: Expr, rhs: Expr, proof: Expr) -> CalcStep {
    CalcStep {
        relation: Name::str("Eq"),
        rhs,
        proof,
    }
}
/// Create a less-than-or-equal step.
pub fn le_step(rhs: Expr, proof: Expr) -> CalcStep {
    CalcStep {
        relation: Name::str("LE.le"),
        rhs,
        proof,
    }
}
#[cfg(test)]
mod step_tests {
    use super::*;
    use crate::tactic::calc::*;
    use oxilean_kernel::Environment;
    fn _mk_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    #[test]
    fn test_is_refl_step_true() {
        let step = TypedCalcStep {
            kind: RelationKind::Eq,
            rhs: Expr::BVar(0),
            proof: Expr::Const(Name::str("refl"), vec![]),
            annotation: None,
        };
        assert!(is_refl_step(&step));
    }
    #[test]
    fn test_is_refl_step_false() {
        let step = TypedCalcStep {
            kind: RelationKind::Eq,
            rhs: Expr::BVar(0),
            proof: Expr::Const(Name::str("h"), vec![]),
            annotation: None,
        };
        assert!(!is_refl_step(&step));
    }
    #[test]
    fn test_calc_proof_endpoints() {
        let start = Expr::Const(Name::str("a"), vec![]);
        let ty = Expr::Const(Name::str("Nat"), vec![]);
        let mut cp = CalcProof::new(start.clone(), ty);
        cp.add_step(CalcStep {
            relation: Name::str("Eq"),
            rhs: Expr::Const(Name::str("b"), vec![]),
            proof: Expr::Const(Name::str("h"), vec![]),
        });
        let (lhs, rhs) = calc_proof_endpoints(&cp);
        assert_eq!(lhs, &start);
        assert_eq!(rhs, &Expr::Const(Name::str("b"), vec![]));
    }
    #[test]
    fn test_calc_proof_depth() {
        let start = Expr::BVar(0);
        let ty = Expr::Sort(Level::zero());
        let mut cp = CalcProof::new(start, ty);
        cp.add_step(CalcStep {
            relation: Name::str("Eq"),
            rhs: Expr::BVar(1),
            proof: Expr::BVar(2),
        });
        cp.add_step(CalcStep {
            relation: Name::str("Eq"),
            rhs: Expr::BVar(3),
            proof: Expr::BVar(4),
        });
        assert_eq!(calc_proof_depth(&cp), 2);
    }
    #[test]
    fn test_validate_chain_endpoints() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let chain = TypedCalcChain::new(a, Expr::Sort(Level::zero())).step(
            RelationKind::Eq,
            b.clone(),
            Expr::BVar(0),
        );
        assert!(validate_chain_endpoints(&chain, &b));
    }
    #[test]
    fn test_eq_step_creates_eq() {
        let s = eq_step(Expr::BVar(0), Expr::BVar(1), Expr::BVar(2));
        assert_eq!(format!("{}", s.relation), "Eq");
    }
    #[test]
    fn test_le_step_creates_le() {
        let s = le_step(Expr::BVar(1), Expr::BVar(2));
        assert_eq!(format!("{}", s.relation), "LE.le");
    }
    #[test]
    fn test_typed_chain_is_empty() {
        let chain = TypedCalcChain::new(Expr::BVar(0), Expr::Sort(Level::zero()));
        assert!(chain.is_empty());
    }
    #[test]
    fn test_typed_chain_len() {
        let chain = TypedCalcChain::new(Expr::BVar(0), Expr::Sort(Level::zero())).step(
            RelationKind::Eq,
            Expr::BVar(1),
            Expr::BVar(2),
        );
        assert_eq!(chain.len(), 1);
    }
}
/// Estimate the proof size of a typed calc chain.
///
/// Each step contributes its proof term size to the total.
pub fn estimate_chain_proof_size(chain: &TypedCalcChain) -> usize {
    chain.steps.iter().map(|s| expr_size(&s.proof)).sum()
}
pub(super) fn expr_size(expr: &Expr) -> usize {
    match expr {
        Expr::App(f, a) => 1 + expr_size(f) + expr_size(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => 1 + expr_size(ty) + expr_size(body),
        Expr::Let(_, ty, val, body) => 1 + expr_size(ty) + expr_size(val) + expr_size(body),
        Expr::Proj(_, _, e) => 1 + expr_size(e),
        _ => 1,
    }
}
/// Apply a rewrite rule to all rhs expressions in a chain.
pub fn map_chain_rhs<F>(chain: &TypedCalcChain, mut f: F) -> TypedCalcChain
where
    F: FnMut(&Expr) -> Expr,
{
    let new_start = f(&chain.start);
    let new_steps = chain
        .steps
        .iter()
        .map(|s| TypedCalcStep {
            kind: s.kind.clone(),
            rhs: f(&s.rhs),
            proof: s.proof.clone(),
            annotation: s.annotation.clone(),
        })
        .collect();
    TypedCalcChain {
        start: new_start,
        ty: chain.ty.clone(),
        steps: new_steps,
    }
}
#[cfg(test)]
mod size_tests {
    use super::*;
    use crate::tactic::calc::*;
    #[test]
    fn test_estimate_chain_proof_size_empty() {
        let chain = TypedCalcChain::new(Expr::BVar(0), Expr::Sort(Level::zero()));
        assert_eq!(estimate_chain_proof_size(&chain), 0);
    }
    #[test]
    fn test_estimate_chain_proof_size_one() {
        let chain = TypedCalcChain::new(Expr::BVar(0), Expr::Sort(Level::zero())).step(
            RelationKind::Eq,
            Expr::BVar(1),
            Expr::Const(Name::str("h"), vec![]),
        );
        assert_eq!(estimate_chain_proof_size(&chain), 1);
    }
    #[test]
    fn test_map_chain_rhs() {
        let chain = TypedCalcChain::new(Expr::BVar(0), Expr::Sort(Level::zero())).step(
            RelationKind::Eq,
            Expr::BVar(1),
            Expr::BVar(0),
        );
        let mapped = map_chain_rhs(&chain, |_| Expr::BVar(99));
        assert_eq!(mapped.start, Expr::BVar(99));
        assert_eq!(mapped.steps[0].rhs, Expr::BVar(99));
    }
}
/// Convert a `CalcProof` into a `TypedCalcChain`.
pub fn calc_proof_to_typed_chain(proof: &CalcProof) -> TypedCalcChain {
    let mut chain = TypedCalcChain::new(proof.start.clone(), proof.ty.clone());
    for step in &proof.steps {
        chain.steps.push(TypedCalcStep {
            kind: RelationKind::from_name(&step.relation),
            rhs: step.rhs.clone(),
            proof: step.proof.clone(),
            annotation: None,
        });
    }
    chain
}
/// Convert a `TypedCalcChain` into a `CalcProof`.
pub fn typed_chain_to_calc_proof(chain: &TypedCalcChain) -> CalcProof {
    let mut proof = CalcProof::new(chain.start.clone(), chain.ty.clone());
    for step in &chain.steps {
        proof.add_step(CalcStep {
            relation: step.kind.to_name(),
            rhs: step.rhs.clone(),
            proof: step.proof.clone(),
        });
    }
    proof
}
/// Split a calc chain at a given index.
///
/// Returns two chains: the prefix (steps 0..idx) and suffix (steps idx..).
pub fn split_calc_chain(chain: &TypedCalcChain, idx: usize) -> (TypedCalcChain, TypedCalcChain) {
    let prefix_steps = chain.steps[..idx.min(chain.steps.len())].to_vec();
    let suffix_steps = chain.steps[idx.min(chain.steps.len())..].to_vec();
    let prefix_start = chain.start.clone();
    let suffix_start = prefix_steps
        .last()
        .map(|s| s.rhs.clone())
        .unwrap_or_else(|| chain.start.clone());
    let mut prefix = TypedCalcChain::new(prefix_start, chain.ty.clone());
    prefix.steps = prefix_steps;
    let mut suffix = TypedCalcChain::new(suffix_start, chain.ty.clone());
    suffix.steps = suffix_steps;
    (prefix, suffix)
}
/// Concatenate two calc chains (suffix.start should equal prefix.end).
pub fn concat_calc_chains(prefix: &TypedCalcChain, suffix: &TypedCalcChain) -> TypedCalcChain {
    let mut result = TypedCalcChain::new(prefix.start.clone(), prefix.ty.clone());
    result.steps.extend(prefix.steps.iter().cloned());
    result.steps.extend(suffix.steps.iter().cloned());
    result
}
/// Count the number of steps in a chain that use a given relation.
pub fn count_relation_steps(chain: &TypedCalcChain, kind: &RelationKind) -> usize {
    chain.steps.iter().filter(|s| &s.kind == kind).count()
}
/// Extract all rhs expressions from a calc chain (excluding start).
pub fn calc_chain_rhss(chain: &TypedCalcChain) -> Vec<&Expr> {
    chain.steps.iter().map(|s| &s.rhs).collect()
}
/// Replace the proof in a step with a new proof.
pub fn with_proof(step: TypedCalcStep, new_proof: Expr) -> TypedCalcStep {
    TypedCalcStep {
        proof: new_proof,
        ..step
    }
}
#[cfg(test)]
mod conversion_tests {
    use super::*;
    use crate::tactic::calc::*;
    use oxilean_kernel::Environment;
    fn _mk_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    fn cnst(s: &str) -> Expr {
        Expr::Const(Name::str(s), vec![])
    }
    #[test]
    fn test_calc_proof_to_typed_chain() {
        let mut cp = CalcProof::new(cnst("a"), cnst("Nat"));
        cp.add_step(CalcStep {
            relation: Name::str("Eq"),
            rhs: cnst("b"),
            proof: cnst("h"),
        });
        let chain = calc_proof_to_typed_chain(&cp);
        assert_eq!(chain.len(), 1);
        assert_eq!(chain.start, cnst("a"));
    }
    #[test]
    fn test_typed_chain_to_calc_proof() {
        let chain = TypedCalcChain::new(cnst("a"), cnst("Nat")).step(
            RelationKind::Le,
            cnst("b"),
            cnst("h"),
        );
        let cp = typed_chain_to_calc_proof(&chain);
        assert_eq!(cp.num_steps(), 1);
        assert_eq!(format!("{}", cp.steps[0].relation), "LE.le");
    }
    #[test]
    fn test_split_calc_chain() {
        let chain = TypedCalcChain::new(cnst("a"), cnst("Nat"))
            .step(RelationKind::Eq, cnst("b"), cnst("h1"))
            .step(RelationKind::Eq, cnst("c"), cnst("h2"))
            .step(RelationKind::Le, cnst("d"), cnst("h3"));
        let (prefix, suffix) = split_calc_chain(&chain, 2);
        assert_eq!(prefix.len(), 2);
        assert_eq!(suffix.len(), 1);
    }
    #[test]
    fn test_concat_calc_chains() {
        let c1 = TypedCalcChain::new(cnst("a"), cnst("Nat")).step(
            RelationKind::Eq,
            cnst("b"),
            cnst("h1"),
        );
        let c2 = TypedCalcChain::new(cnst("b"), cnst("Nat")).step(
            RelationKind::Eq,
            cnst("c"),
            cnst("h2"),
        );
        let combined = concat_calc_chains(&c1, &c2);
        assert_eq!(combined.len(), 2);
    }
    #[test]
    fn test_count_relation_steps() {
        let chain = TypedCalcChain::new(cnst("a"), cnst("Nat"))
            .step(RelationKind::Eq, cnst("b"), cnst("h1"))
            .step(RelationKind::Le, cnst("c"), cnst("h2"))
            .step(RelationKind::Eq, cnst("d"), cnst("h3"));
        assert_eq!(count_relation_steps(&chain, &RelationKind::Eq), 2);
        assert_eq!(count_relation_steps(&chain, &RelationKind::Le), 1);
    }
    #[test]
    fn test_calc_chain_rhss() {
        let chain = TypedCalcChain::new(cnst("a"), cnst("Nat"))
            .step(RelationKind::Eq, cnst("b"), cnst("h1"))
            .step(RelationKind::Eq, cnst("c"), cnst("h2"));
        let rhss = calc_chain_rhss(&chain);
        assert_eq!(rhss.len(), 2);
        assert_eq!(*rhss[0], cnst("b"));
    }
    #[test]
    fn test_relation_kind_to_name() {
        assert_eq!(format!("{}", RelationKind::Eq.to_name()), "Eq");
        assert_eq!(format!("{}", RelationKind::Le.to_name()), "LE.le");
    }
    #[test]
    fn test_relation_kind_implies_eq() {
        assert!(RelationKind::Eq.implies_eq());
        assert!(!RelationKind::Le.implies_eq());
    }
    #[test]
    fn test_relation_kind_is_ordering() {
        assert!(RelationKind::Le.is_ordering());
        assert!(RelationKind::Lt.is_ordering());
        assert!(!RelationKind::Eq.is_ordering());
        assert!(!RelationKind::Iff.is_ordering());
    }
    #[test]
    fn test_with_proof() {
        let step = TypedCalcStep {
            kind: RelationKind::Eq,
            rhs: cnst("b"),
            proof: cnst("old_proof"),
            annotation: None,
        };
        let new_step = with_proof(step, cnst("new_proof"));
        assert_eq!(new_step.proof, cnst("new_proof"));
        assert_eq!(new_step.rhs, cnst("b"));
    }
    #[test]
    fn test_split_at_zero() {
        let chain = TypedCalcChain::new(cnst("a"), cnst("Nat")).step(
            RelationKind::Eq,
            cnst("b"),
            cnst("h"),
        );
        let (prefix, suffix) = split_calc_chain(&chain, 0);
        assert_eq!(prefix.len(), 0);
        assert_eq!(suffix.len(), 1);
    }
    #[test]
    fn test_split_at_end() {
        let chain = TypedCalcChain::new(cnst("a"), cnst("Nat")).step(
            RelationKind::Eq,
            cnst("b"),
            cnst("h"),
        );
        let (prefix, suffix) = split_calc_chain(&chain, 10);
        assert_eq!(prefix.len(), 1);
        assert_eq!(suffix.len(), 0);
    }
}
#[cfg(test)]
mod taccalc_ext2_tests {
    use super::*;
    use crate::tactic::calc::*;
    #[test]
    fn test_taccalc_ext_util_basic() {
        let mut u = TacCalcExtUtil::new("test");
        u.push(10);
        u.push(20);
        assert_eq!(u.sum(), 30);
        assert_eq!(u.len(), 2);
    }
    #[test]
    fn test_taccalc_ext_util_min_max() {
        let mut u = TacCalcExtUtil::new("mm");
        u.push(5);
        u.push(1);
        u.push(9);
        assert_eq!(u.min_val(), Some(1));
        assert_eq!(u.max_val(), Some(9));
    }
    #[test]
    fn test_taccalc_ext_util_flags() {
        let mut u = TacCalcExtUtil::new("flags");
        u.set_flag(3);
        assert!(u.has_flag(3));
        assert!(!u.has_flag(2));
    }
    #[test]
    fn test_taccalc_ext_util_pop() {
        let mut u = TacCalcExtUtil::new("pop");
        u.push(42);
        assert_eq!(u.pop(), Some(42));
        assert!(u.is_empty());
    }
    #[test]
    fn test_taccalc_ext_map_basic() {
        let mut m: TacCalcExtMap<i32> = TacCalcExtMap::new();
        m.insert("key", 42);
        assert_eq!(m.get("key"), Some(&42));
        assert!(m.contains("key"));
        assert!(!m.contains("other"));
    }
    #[test]
    fn test_taccalc_ext_map_get_or_default() {
        let mut m: TacCalcExtMap<i32> = TacCalcExtMap::new();
        m.insert("k", 5);
        assert_eq!(m.get_or_default("k"), 5);
        assert_eq!(m.get_or_default("missing"), 0);
    }
    #[test]
    fn test_taccalc_ext_map_keys_sorted() {
        let mut m: TacCalcExtMap<i32> = TacCalcExtMap::new();
        m.insert("z", 1);
        m.insert("a", 2);
        m.insert("m", 3);
        let keys = m.keys_sorted();
        assert_eq!(keys[0].as_str(), "a");
        assert_eq!(keys[2].as_str(), "z");
    }
    #[test]
    fn test_taccalc_window_mean() {
        let mut w = TacCalcWindow::new(3);
        w.push(1.0);
        w.push(2.0);
        w.push(3.0);
        assert!((w.mean() - 2.0).abs() < 1e-10);
    }
    #[test]
    fn test_taccalc_window_evict() {
        let mut w = TacCalcWindow::new(2);
        w.push(10.0);
        w.push(20.0);
        w.push(30.0);
        assert_eq!(w.len(), 2);
        assert!((w.mean() - 25.0).abs() < 1e-10);
    }
    #[test]
    fn test_taccalc_window_std_dev() {
        let mut w = TacCalcWindow::new(10);
        for i in 0..10 {
            w.push(i as f64);
        }
        assert!(w.std_dev() > 0.0);
    }
    #[test]
    fn test_taccalc_builder_basic() {
        let b = TacCalcBuilder::new("test")
            .add_item("a")
            .add_item("b")
            .set_config("key", "val");
        assert_eq!(b.item_count(), 2);
        assert!(b.has_config("key"));
        assert_eq!(b.get_config("key"), Some("val"));
    }
    #[test]
    fn test_taccalc_builder_summary() {
        let b = TacCalcBuilder::new("suite").add_item("x");
        let s = b.build_summary();
        assert!(s.contains("suite"));
    }
    #[test]
    fn test_taccalc_state_machine_start() {
        let mut sm = TacCalcStateMachine::new();
        assert!(sm.start());
        assert!(sm.state.is_running());
    }
    #[test]
    fn test_taccalc_state_machine_complete() {
        let mut sm = TacCalcStateMachine::new();
        sm.start();
        sm.complete();
        assert!(sm.state.is_terminal());
    }
    #[test]
    fn test_taccalc_state_machine_fail() {
        let mut sm = TacCalcStateMachine::new();
        sm.fail("oops");
        assert!(sm.state.is_terminal());
        assert_eq!(sm.state.error_msg(), Some("oops"));
    }
    #[test]
    fn test_taccalc_state_machine_no_transition_after_terminal() {
        let mut sm = TacCalcStateMachine::new();
        sm.complete();
        assert!(!sm.start());
    }
    #[test]
    fn test_taccalc_work_queue_basic() {
        let mut wq = TacCalcWorkQueue::new(10);
        wq.enqueue("task1".to_string());
        wq.enqueue("task2".to_string());
        assert_eq!(wq.pending_count(), 2);
        let t = wq.dequeue();
        assert_eq!(t, Some("task1".to_string()));
        assert_eq!(wq.processed_count(), 1);
    }
    #[test]
    fn test_taccalc_work_queue_capacity() {
        let mut wq = TacCalcWorkQueue::new(2);
        wq.enqueue("a".to_string());
        wq.enqueue("b".to_string());
        assert!(wq.is_full());
        assert!(!wq.enqueue("c".to_string()));
    }
    #[test]
    fn test_taccalc_counter_map_basic() {
        let mut cm = TacCalcCounterMap::new();
        cm.increment("apple");
        cm.increment("apple");
        cm.increment("banana");
        assert_eq!(cm.count("apple"), 2);
        assert_eq!(cm.count("banana"), 1);
        assert_eq!(cm.num_unique(), 2);
    }
    #[test]
    fn test_taccalc_counter_map_frequency() {
        let mut cm = TacCalcCounterMap::new();
        cm.increment("a");
        cm.increment("a");
        cm.increment("b");
        assert!((cm.frequency("a") - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_taccalc_counter_map_most_common() {
        let mut cm = TacCalcCounterMap::new();
        cm.increment("x");
        cm.increment("y");
        cm.increment("x");
        let (k, v) = cm.most_common().expect("most_common should succeed");
        assert_eq!(k.as_str(), "x");
        assert_eq!(v, 2);
    }
}
#[cfg(test)]
mod tacticcalc_analysis_tests {
    use super::*;
    use crate::tactic::calc::*;
    #[test]
    fn test_tacticcalc_result_ok() {
        let r = TacticCalcResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticcalc_result_err() {
        let r = TacticCalcResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticcalc_result_partial() {
        let r = TacticCalcResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_tacticcalc_result_skipped() {
        let r = TacticCalcResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_tacticcalc_analysis_pass_run() {
        let mut p = TacticCalcAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_tacticcalc_analysis_pass_empty_input() {
        let mut p = TacticCalcAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_tacticcalc_analysis_pass_success_rate() {
        let mut p = TacticCalcAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_tacticcalc_analysis_pass_disable() {
        let mut p = TacticCalcAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_tacticcalc_pipeline_basic() {
        let mut pipeline = TacticCalcPipeline::new("main_pipeline");
        pipeline.add_pass(TacticCalcAnalysisPass::new("pass1"));
        pipeline.add_pass(TacticCalcAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_tacticcalc_pipeline_disabled_pass() {
        let mut pipeline = TacticCalcPipeline::new("partial");
        let mut p = TacticCalcAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(TacticCalcAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_tacticcalc_diff_basic() {
        let mut d = TacticCalcDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_tacticcalc_diff_summary() {
        let mut d = TacticCalcDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_tacticcalc_config_set_get() {
        let mut cfg = TacticCalcConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_tacticcalc_config_read_only() {
        let mut cfg = TacticCalcConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_tacticcalc_config_remove() {
        let mut cfg = TacticCalcConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_tacticcalc_diagnostics_basic() {
        let mut diag = TacticCalcDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_tacticcalc_diagnostics_max_errors() {
        let mut diag = TacticCalcDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_tacticcalc_diagnostics_clear() {
        let mut diag = TacticCalcDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_tacticcalc_config_value_types() {
        let b = TacticCalcConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = TacticCalcConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = TacticCalcConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = TacticCalcConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = TacticCalcConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod calc_ext_tests_1600 {
    use super::*;
    use crate::tactic::calc::*;
    #[test]
    fn test_calc_ext_result_ok_1600() {
        let r = CalcExtResult1600::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_calc_ext_result_err_1600() {
        let r = CalcExtResult1600::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_calc_ext_result_partial_1600() {
        let r = CalcExtResult1600::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_calc_ext_result_skipped_1600() {
        let r = CalcExtResult1600::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_calc_ext_pass_run_1600() {
        let mut p = CalcExtPass1600::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_calc_ext_pass_empty_1600() {
        let mut p = CalcExtPass1600::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_calc_ext_pass_rate_1600() {
        let mut p = CalcExtPass1600::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_calc_ext_pass_disable_1600() {
        let mut p = CalcExtPass1600::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_calc_ext_pipeline_basic_1600() {
        let mut pipeline = CalcExtPipeline1600::new("main_pipeline");
        pipeline.add_pass(CalcExtPass1600::new("pass1"));
        pipeline.add_pass(CalcExtPass1600::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_calc_ext_pipeline_disabled_1600() {
        let mut pipeline = CalcExtPipeline1600::new("partial");
        let mut p = CalcExtPass1600::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(CalcExtPass1600::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_calc_ext_diff_basic_1600() {
        let mut d = CalcExtDiff1600::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_calc_ext_config_set_get_1600() {
        let mut cfg = CalcExtConfig1600::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_calc_ext_config_read_only_1600() {
        let mut cfg = CalcExtConfig1600::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_calc_ext_config_remove_1600() {
        let mut cfg = CalcExtConfig1600::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_calc_ext_diagnostics_basic_1600() {
        let mut diag = CalcExtDiag1600::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_calc_ext_diagnostics_max_errors_1600() {
        let mut diag = CalcExtDiag1600::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_calc_ext_diagnostics_clear_1600() {
        let mut diag = CalcExtDiag1600::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_calc_ext_config_value_types_1600() {
        let b = CalcExtConfigVal1600::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = CalcExtConfigVal1600::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = CalcExtConfigVal1600::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = CalcExtConfigVal1600::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = CalcExtConfigVal1600::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
