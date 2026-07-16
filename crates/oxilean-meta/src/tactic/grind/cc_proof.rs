//! Nieuwenhuis-Oliveras (NO) proof reconstruction for congruence closure.
//!
//! Given two E-nodes known to be equal, [`explain`] traces through the
//! proof forest to produce a sequence of [`ExplainStep`]s.
//! [`build_eq_proof_with_locals`] converts those steps into a fully-applied
//! kernel proof term using `Eq.refl`, `Eq.symm`, `Eq.trans`, and `congr`.
//!
//! # Correctness contract
//! Every produced term is fully applied: all implicit arguments of
//! `Eq.refl`, `Eq.symm`, `Eq.trans`, and `congr` are supplied positionally.
//! The caller kernel-verifies before accepting; if verification fails the
//! elaborator falls back to `sorry`.

use oxilean_kernel::Node;
use std::collections::HashMap;

use oxilean_kernel::{Expr, FVarId, LocalDecl, Name, TypeChecker};

use super::types::{CongruenceClosure, ENodeId, ProofLabel};

// ── Explanation output types ──────────────────────────────────────────────────

/// A single step in an equality explanation.
#[derive(Debug, Clone)]
pub struct ExplainStep {
    /// Left-hand side expression.
    pub lhs: Expr,
    /// Right-hand side expression.
    pub rhs: Expr,
    /// How this equality is justified.
    pub reason: ExplainReason,
}

/// Why two expressions are equal in one step.
#[derive(Debug, Clone)]
pub enum ExplainReason {
    /// Reflexivity: `lhs` is syntactically identical to `rhs`.
    Refl,
    /// Direct hypothesis `h : hyp_lhs = hyp_rhs`.
    ///
    /// `fwd = true`  means this step is `lhs = rhs` in the hyp's direction.
    /// `fwd = false` means we need `Eq.symm` because the edge was reversed.
    Hyp { name: Name, fwd: bool },
    /// Congruence: `f a₁..aₙ = f b₁..bₙ` because each `aᵢ = bᵢ`.
    ///
    /// `arg_pairs[i] = (aᵢ, bᵢ, steps_proving_aᵢ=bᵢ)`.
    Congruence {
        arg_pairs: Vec<(Expr, Expr, Vec<ExplainStep>)>,
    },
}

// ── explain ───────────────────────────────────────────────────────────────────

/// Explain why E-nodes `a` and `b` are in the same equivalence class.
///
/// Returns a sequence of [`ExplainStep`]s that chain-prove `expr(a) = expr(b)`.
/// Returns an empty vec only when `a == b` (caller treats as Refl).
pub fn explain(cc: &CongruenceClosure, a: ENodeId, b: ENodeId) -> Vec<ExplainStep> {
    if a == b {
        return Vec::new();
    }

    // Collect the path from each node to its proof-forest root.
    let path_a = collect_proof_path(cc, a);
    let path_b = collect_proof_path(cc, b);

    // Build a lookup: node → index in path_b.
    let path_b_map: HashMap<ENodeId, usize> = path_b
        .iter()
        .enumerate()
        .map(|(i, (n, _))| (*n, i))
        .collect();

    // Find the LCA: first node in path_a that also appears in path_b.
    let lca_idx_a = match path_a.iter().position(|(n, _)| path_b_map.contains_key(n)) {
        Some(i) => i,
        None => return Vec::new(), // not connected (shouldn't happen when are_equal holds)
    };
    let lca_node = path_a[lca_idx_a].0;
    let lca_idx_b = path_b_map[&lca_node];

    let mut steps = Vec::new();

    // Forward segment: a → lca (following proof-forest parent edges).
    for i in 0..lca_idx_a {
        let from_node = path_a[i].0;
        let (to_node, label_opt) = &path_a[i + 1];
        if let Some(label) = label_opt {
            if let Some(step) = label_to_step(cc, from_node, *to_node, label, true) {
                steps.push(step);
            }
        }
    }

    // Backward segment: lca → b (path_b reversed, so edges go backwards).
    // path_b is [b, p1, p2, ..., lca].  We want lca→...→b so we traverse
    // path_b[lca_idx_b-1 downto 0], each edge going *parent → child* (reversed).
    for i in (0..lca_idx_b).rev() {
        let child_node = path_b[i].0;
        let (parent_node, label_opt) = &path_b[i + 1];
        if let Some(label) = label_opt {
            // Edge was parent→child in the forest; we traverse child←parent, i.e. reversed.
            if let Some(step) = label_to_step(cc, *parent_node, child_node, label, false) {
                steps.push(step);
            }
        }
    }

    steps
}

/// Collect the chain from `start` to its proof-forest root.
///
/// Returns `[(start, None), (p1, Some(label_start→p1)), ..., (root, Some(...))]`.
/// The label at index `i` is the label on the edge from `path[i-1]` to `path[i]`.
fn collect_proof_path(
    cc: &CongruenceClosure,
    start: ENodeId,
) -> Vec<(ENodeId, Option<ProofLabel>)> {
    let mut path = Vec::new();
    let mut cur = start;
    path.push((cur, None));
    while let Some(entry) = cc.proof_parent.get(cur.0 as usize) {
        match entry {
            Some((parent, label)) => {
                path.push((*parent, Some(label.clone())));
                cur = *parent;
            }
            None => break,
        }
    }
    path
}

/// Convert a proof-forest edge label into an [`ExplainStep`].
///
/// `from`/`to` are the ENodeIds at either end of the edge.
/// `forward = true` means we're traversing in the edge's recorded direction.
fn label_to_step(
    cc: &CongruenceClosure,
    from: ENodeId,
    to: ENodeId,
    label: &ProofLabel,
    forward: bool,
) -> Option<ExplainStep> {
    let from_expr = cc.get_node(from)?.origin_expr.clone()?;
    let to_expr = cc.get_node(to)?.origin_expr.clone()?;

    let (lhs, rhs) = if forward {
        (from_expr.clone(), to_expr.clone())
    } else {
        (to_expr.clone(), from_expr.clone())
    };

    match label {
        ProofLabel::Input { hyp_name, fwd, .. } => {
            // `fwd=true` means edge records the hyp's own direction (lhs→rhs).
            // If we're traversing forward AND edge is fwd, step is in hyp direction → fwd=true.
            // Otherwise we need Eq.symm.
            let effective_fwd = *fwd == forward;
            Some(ExplainStep {
                lhs,
                rhs,
                reason: ExplainReason::Hyp {
                    name: hyp_name.clone(),
                    fwd: effective_fwd,
                },
            })
        }
        ProofLabel::Congruence { n1, n2 } => {
            // n1 = left-side app ENodeId, n2 = right-side app ENodeId.
            let node1 = cc.get_node(*n1)?;
            let node2 = cc.get_node(*n2)?;
            let n1_expr = node1.origin_expr.clone()?;
            let n2_expr = node2.origin_expr.clone()?;

            // Decompose both into (func_spine, args_list).
            let (f1, args1) = decompose_app_spine(&n1_expr);
            let (f2, args2) = decompose_app_spine(&n2_expr);

            if f1 != f2 || args1.len() != args2.len() {
                return None; // shouldn't happen in a well-formed CC
            }

            let mut arg_pairs = Vec::new();
            for (a_expr, b_expr) in args1.iter().zip(args2.iter()) {
                let sub_steps = if a_expr == b_expr {
                    Vec::new()
                } else {
                    let a_nid = cc.lookup_expr(a_expr)?;
                    let b_nid = cc.lookup_expr(b_expr)?;
                    explain(cc, a_nid, b_nid)
                };
                arg_pairs.push((a_expr.clone(), b_expr.clone(), sub_steps));
            }

            // Respect forward/backward direction for the overall congruence step.
            let (actual_lhs, actual_rhs) = if forward {
                (n1_expr, n2_expr)
            } else {
                (n2_expr, n1_expr)
            };

            // Flip arg_pairs if we're going backwards.
            let final_pairs = if forward {
                arg_pairs
            } else {
                arg_pairs
                    .into_iter()
                    .map(|(a, b, sub)| {
                        // Reverse the sub-proof direction too.
                        let rev_steps = reverse_steps(sub);
                        (b, a, rev_steps)
                    })
                    .collect()
            };

            Some(ExplainStep {
                lhs: actual_lhs,
                rhs: actual_rhs,
                reason: ExplainReason::Congruence {
                    arg_pairs: final_pairs,
                },
            })
        }
    }
}

/// Reverse a sequence of explanation steps (flip lhs/rhs, apply Eq.symm).
fn reverse_steps(steps: Vec<ExplainStep>) -> Vec<ExplainStep> {
    steps
        .into_iter()
        .rev()
        .map(|s| ExplainStep {
            lhs: s.rhs,
            rhs: s.lhs,
            reason: reverse_reason(s.reason),
        })
        .collect()
}

/// Reverse an explanation reason.
fn reverse_reason(reason: ExplainReason) -> ExplainReason {
    match reason {
        ExplainReason::Refl => ExplainReason::Refl,
        ExplainReason::Hyp { name, fwd } => ExplainReason::Hyp { name, fwd: !fwd },
        ExplainReason::Congruence { arg_pairs } => ExplainReason::Congruence {
            arg_pairs: arg_pairs
                .into_iter()
                .map(|(a, b, sub)| (b, a, reverse_steps(sub)))
                .collect(),
        },
    }
}

/// Decompose a left-nested App spine: `App(App(f, a), b)` → `(f, [a, b])`.
pub(super) fn decompose_app_spine(expr: &Expr) -> (Expr, Vec<Expr>) {
    let mut args = Vec::new();
    let mut cur = expr.clone();
    loop {
        match cur {
            Expr::App(func, arg) => {
                args.push((*arg).clone());
                cur = (*func).clone();
            }
            other => {
                args.reverse();
                return (other, args);
            }
        }
    }
}

// ── Proof-term builder ────────────────────────────────────────────────────────

/// Build a kernel-correct proof term for the equality chain described by `steps`,
/// using `env` for type inference and `local_types` for FVar types.
///
/// Returns `None` if any type inference fails or `steps` is inconsistent.
/// An empty `steps` slice means reflexivity — caller supplies `Eq.refl`.
pub fn build_eq_proof_with_locals(
    steps: &[ExplainStep],
    env: &oxilean_kernel::Environment,
    hyp_fvars: &[(Name, FVarId)],
    local_types: &[(FVarId, Name, Expr)],
) -> Option<Expr> {
    if steps.is_empty() {
        return None;
    }
    let mut checker = TypeChecker::new(env);
    for (fvar, name, ty) in local_types {
        checker.push_local(LocalDecl {
            fvar: *fvar,
            name: name.clone(),
            ty: ty.clone(),
            val: None,
        });
    }
    build_steps_with_checker(steps, &mut checker, hyp_fvars)
}

/// Chain a sequence of equality steps into a single proof term.
///
/// If `steps` has one element, returns that element's proof.
/// Otherwise chains with `@Eq.trans`.
fn build_steps_with_checker(
    steps: &[ExplainStep],
    checker: &mut TypeChecker<'_>,
    hyp_fvars: &[(Name, FVarId)],
) -> Option<Expr> {
    if steps.is_empty() {
        return None;
    }

    let mut acc = build_one_step(&steps[0], checker, hyp_fvars)?;
    let mut acc_lhs = steps[0].lhs.clone();
    let mut acc_rhs = steps[0].rhs.clone();

    for step in &steps[1..] {
        let step_proof = build_one_step(step, checker, hyp_fvars)?;
        // Build @Eq.trans T acc_lhs acc_rhs step.rhs acc step_proof.
        let t = infer_or_none(checker, &acc_lhs)?;
        acc = mk_eq_trans_full(
            t,
            acc_lhs,
            acc_rhs.clone(),
            step.rhs.clone(),
            acc,
            step_proof,
        );
        acc_lhs = acc_rhs;
        acc_rhs = step.rhs.clone();
    }

    Some(acc)
}

/// Build a proof term for a single [`ExplainStep`].
fn build_one_step(
    step: &ExplainStep,
    checker: &mut TypeChecker<'_>,
    hyp_fvars: &[(Name, FVarId)],
) -> Option<Expr> {
    match &step.reason {
        ExplainReason::Refl => {
            let t = infer_or_none(checker, &step.lhs)?;
            Some(mk_eq_refl_full(t, step.lhs.clone()))
        }

        ExplainReason::Hyp { name, fwd } => {
            // Resolve the hypothesis to a FVar or fall back to a Const.
            let h = if let Some(&(_, fid)) = hyp_fvars.iter().find(|(n, _)| n == name) {
                Expr::FVar(fid)
            } else {
                Expr::Const(name.clone(), vec![])
            };
            if *fwd {
                Some(h)
            } else {
                // We need Eq.symm: h proves (step.rhs = step.lhs), we want (step.lhs = step.rhs).
                // @Eq.symm T (step.rhs) (step.lhs) h
                let t = infer_or_none(checker, &step.lhs)?;
                Some(mk_eq_symm_full(t, step.rhs.clone(), step.lhs.clone(), h))
            }
        }

        ExplainReason::Congruence { arg_pairs } => {
            build_congruence_proof_for_step(step, arg_pairs, checker, hyp_fvars)
        }
    }
}

/// Build a congruence proof term for `step.lhs = step.rhs`.
///
/// `step.lhs` must be `f a₁..aₙ` and `step.rhs` must be `f b₁..bₙ`.
/// For each `i`, `arg_pairs[i] = (aᵢ, bᵢ, sub_steps_for_aᵢ=bᵢ)`.
fn build_congruence_proof_for_step(
    step: &ExplainStep,
    arg_pairs: &[(Expr, Expr, Vec<ExplainStep>)],
    checker: &mut TypeChecker<'_>,
    hyp_fvars: &[(Name, FVarId)],
) -> Option<Expr> {
    let (f_lhs, args_lhs) = decompose_app_spine(&step.lhs);
    let (f_rhs, args_rhs) = decompose_app_spine(&step.rhs);

    if f_lhs != f_rhs || args_lhs.len() != args_rhs.len() {
        return None;
    }

    let n = args_lhs.len();

    if n == 0 || arg_pairs.is_empty() {
        // No arguments — reflexivity on `f`.
        let t = infer_or_none(checker, &step.lhs)?;
        return Some(mk_eq_refl_full(t, step.lhs.clone()));
    }

    // We fold over the argument list using `@congr`:
    //   @congr α β f_cur g_cur aᵢ bᵢ hf ha
    // where hf proves f_cur = g_cur (built up incrementally) and
    //       ha proves aᵢ = bᵢ.
    //
    // Start: f_cur = f_lhs, g_cur = f_rhs,
    //        hf = @Eq.refl (type of f_lhs→...) f_lhs.

    let f_ty = infer_or_none(checker, &f_lhs)?;
    let mut cur_lhs = f_lhs.clone(); // f applied to a₁..aᵢ₋₁ on LHS
    let mut cur_rhs = f_rhs.clone(); // f applied to b₁..bᵢ₋₁ on RHS
    let mut cur_proof = mk_eq_refl_full(f_ty, f_lhs.clone());

    for i in 0..n {
        let ai = &args_lhs[i];
        let bi = &args_rhs[i];

        // Proof of aᵢ = bᵢ.
        let hi = if ai == bi {
            let ai_ty = infer_or_none(checker, ai)?;
            mk_eq_refl_full(ai_ty, ai.clone())
        } else if i < arg_pairs.len() {
            let (_, _, ref sub_steps) = arg_pairs[i];
            if sub_steps.is_empty() {
                // Empty sub-steps means reflexivity.
                let ai_ty = infer_or_none(checker, ai)?;
                mk_eq_refl_full(ai_ty, ai.clone())
            } else {
                build_steps_with_checker(sub_steps, checker, hyp_fvars)?
            }
        } else {
            // No sub-proof provided — treat as reflexivity.
            let ai_ty = infer_or_none(checker, ai)?;
            mk_eq_refl_full(ai_ty, ai.clone())
        };

        // alpha = type(aᵢ), beta = type(f_cur aᵢ).
        let alpha = infer_or_none(checker, ai)?;
        let cur_lhs_applied = Expr::App(Node::new(cur_lhs.clone()), Node::new(ai.clone()));
        let beta = infer_or_none(checker, &cur_lhs_applied)?;

        // @congr alpha beta cur_lhs cur_rhs ai bi cur_proof hi
        cur_proof = mk_congr_full(
            alpha,
            beta,
            cur_lhs.clone(),
            cur_rhs.clone(),
            ai.clone(),
            bi.clone(),
            cur_proof,
            hi,
        );
        cur_lhs = cur_lhs_applied;
        cur_rhs = Expr::App(Node::new(cur_rhs.clone()), Node::new(bi.clone()));
    }

    Some(cur_proof)
}

// ── Fully-applied kernel term constructors ────────────────────────────────────

/// `@Eq.refl T a` — two explicit args.
///
/// `Eq.refl : {α : Sort u} → (a : α) → @Eq α a a`
/// Positional supply: [T, a].
pub(super) fn mk_eq_refl_full(t: Expr, a: Expr) -> Expr {
    let refl = Expr::Const(Name::str("Eq.refl"), vec![]);
    Expr::App(
        Node::new(Expr::App(Node::new(refl), Node::new(t))),
        Node::new(a),
    )
}

/// `@Eq.symm T a b h` — four explicit args.
///
/// `Eq.symm : {α}{a b:α}, @Eq α a b → @Eq α b a`
/// Positional: [T, a, b, h].
pub(super) fn mk_eq_symm_full(t: Expr, a: Expr, b: Expr, h: Expr) -> Expr {
    let symm = Expr::Const(Name::str("Eq.symm"), vec![]);
    Expr::App(
        Node::new(Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::App(Node::new(symm), Node::new(t))),
                Node::new(a),
            )),
            Node::new(b),
        )),
        Node::new(h),
    )
}

/// `@Eq.trans T a b c p q` — six explicit args.
///
/// `Eq.trans : {α}{a b c:α}, @Eq α a b → @Eq α b c → @Eq α a c`
/// Positional: [T, a, b, c, p, q].
pub(super) fn mk_eq_trans_full(t: Expr, a: Expr, b: Expr, c: Expr, p: Expr, q: Expr) -> Expr {
    let trans = Expr::Const(Name::str("Eq.trans"), vec![]);
    Expr::App(
        Node::new(Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::App(
                    Node::new(Expr::App(
                        Node::new(Expr::App(Node::new(trans), Node::new(t))),
                        Node::new(a),
                    )),
                    Node::new(b),
                )),
                Node::new(c),
            )),
            Node::new(p),
        )),
        Node::new(q),
    )
}

/// `@congr α β f g a b hf ha` — eight explicit args.
///
/// `congr : {α β}{f g:α→β}{a b:α}, @Eq (α→β) f g → @Eq α a b → @Eq β (f a) (g b)`
/// Positional: [α, β, f, g, a, b, hf, ha].
#[allow(clippy::too_many_arguments)]
pub(super) fn mk_congr_full(
    alpha: Expr,
    beta: Expr,
    f: Expr,
    g: Expr,
    a: Expr,
    b: Expr,
    hf: Expr,
    ha: Expr,
) -> Expr {
    let congr = Expr::Const(Name::str("congr"), vec![]);
    Expr::App(
        Node::new(Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::App(
                    Node::new(Expr::App(
                        Node::new(Expr::App(
                            Node::new(Expr::App(
                                Node::new(Expr::App(Node::new(congr), Node::new(alpha))),
                                Node::new(beta),
                            )),
                            Node::new(f),
                        )),
                        Node::new(g),
                    )),
                    Node::new(a),
                )),
                Node::new(b),
            )),
            Node::new(hf),
        )),
        Node::new(ha),
    )
}

/// Attempt to infer the type of `expr`, returning `None` on failure.
fn infer_or_none(checker: &mut TypeChecker<'_>, expr: &Expr) -> Option<Expr> {
    checker.infer_type(expr).ok()
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::{BinderInfo, Declaration, EnvError, Environment, Level};

    use super::super::types::ProofLabel;
    use super::super::types::{CongruenceClosure, MergeReason};

    // ── Helpers ───────────────────────────────────────────────────────────────

    fn cst(s: &str) -> Expr {
        Expr::Const(Name::str(s), vec![])
    }

    fn app(f: Expr, a: Expr) -> Expr {
        Expr::App(Node::new(f), Node::new(a))
    }

    fn sort1() -> Expr {
        Expr::Sort(Level::succ(Level::zero()))
    }

    fn pi_impl(dom: Expr, body: Expr) -> Expr {
        Expr::Pi(
            BinderInfo::Implicit,
            Name::str("_"),
            Node::new(dom),
            Node::new(body),
        )
    }

    fn pi_def(name: &str, dom: Expr, body: Expr) -> Expr {
        Expr::Pi(
            BinderInfo::Default,
            Name::str(name),
            Node::new(dom),
            Node::new(body),
        )
    }

    fn bv(n: u32) -> Expr {
        Expr::BVar(n)
    }

    /// Build `@Eq α a b`.
    fn eq_app(alpha: Expr, a: Expr, b: Expr) -> Expr {
        app(
            app(
                app(
                    Expr::Const(Name::str("Eq"), vec![Level::succ(Level::zero())]),
                    alpha,
                ),
                a,
            ),
            b,
        )
    }

    /// Register the minimal axioms needed for cc proof reconstruction.
    ///
    /// Axiom types exactly match the cc_helper definitions in oxilean-std:
    /// Eq, Eq.refl, Eq.symm, Eq.trans, and congr.
    fn make_cc_env() -> Result<Environment, EnvError> {
        let mut env = Environment::new();

        // Eq : ∀ (α : Sort 1), α → α → Prop
        // = {Sort1} → bv(0) → bv(1) → Prop
        let eq_ty = pi_def(
            "α",
            sort1(),
            pi_def("a", bv(0), pi_def("b", bv(1), Expr::Sort(Level::zero()))),
        );
        env.add(Declaration::Axiom {
            name: Name::str("Eq"),
            univ_params: vec![],
            ty: eq_ty,
        })?;

        // Eq.refl : {α : Sort 1} → (a : α) → @Eq α a a
        // After {α}: α=bv(0). After a: a=bv(0), α=bv(1). Conclusion: eq_app(bv(1), bv(0), bv(0))
        let refl_ty = pi_impl(sort1(), pi_def("a", bv(0), eq_app(bv(1), bv(0), bv(0))));
        env.add(Declaration::Axiom {
            name: Name::str("Eq.refl"),
            univ_params: vec![],
            ty: refl_ty,
        })?;

        // Eq.symm : {α : Sort 1} → {a : α} → {b : α} → @Eq α a b → @Eq α b a
        // After {α}: α=bv(0). After {a}: a=bv(0), α=bv(1). After {b}: b=bv(0), a=bv(1), α=bv(2).
        // h: eq_app(bv(2), bv(1), bv(0)). After h: bv(3), bv(1), bv(2). Concl: eq_app(bv(3), bv(1), bv(2))
        let symm_ty = pi_impl(
            sort1(),
            pi_impl(
                bv(0),
                pi_impl(
                    bv(1),
                    pi_def(
                        "h",
                        eq_app(bv(2), bv(1), bv(0)),
                        eq_app(bv(3), bv(1), bv(2)),
                    ),
                ),
            ),
        );
        env.add(Declaration::Axiom {
            name: Name::str("Eq.symm"),
            univ_params: vec![],
            ty: symm_ty,
        })?;

        // Eq.trans : {α} → {a : α} → {b : α} → {c : α} → a=b → b=c → a=c
        // After {α}: α=bv(0). After {a}: a=bv(0), α=bv(1). After {b}: b=bv(0), a=bv(1), α=bv(2).
        // After {c}: c=bv(0), b=bv(1), a=bv(2), α=bv(3).
        // h1: eq_app(bv(3), bv(2), bv(1)). After h1: bv(4), bv(2), bv(1), bv(0)... wait.
        // After h1: h1=bv(0), c=bv(1), b=bv(2), a=bv(3), α=bv(4).
        // h2: eq_app(bv(4), bv(2), bv(1)).
        // After h2: h2=bv(0), h1=bv(1), c=bv(2), b=bv(3), a=bv(4), α=bv(5).
        // concl: eq_app(bv(5), bv(4), bv(2))
        let trans_ty = pi_impl(
            sort1(),
            pi_impl(
                bv(0),
                pi_impl(
                    bv(1),
                    pi_impl(
                        bv(2),
                        pi_def(
                            "h1",
                            eq_app(bv(3), bv(2), bv(1)),
                            pi_def(
                                "h2",
                                eq_app(bv(4), bv(2), bv(1)),
                                eq_app(bv(5), bv(4), bv(2)),
                            ),
                        ),
                    ),
                ),
            ),
        );
        env.add(Declaration::Axiom {
            name: Name::str("Eq.trans"),
            univ_params: vec![],
            ty: trans_ty,
        })?;

        // congr : {α β : Sort 1} → {f g : α→β} → {a b : α} →
        //         @Eq (α→β) f g → @Eq α a b → @Eq β (f a) (g b)
        //
        // After {α}: α=bv(0). After {β}: β=bv(0), α=bv(1).
        // After {f : α→β}: f=bv(0), β=bv(1), α=bv(2). f's type = pi_def("_", bv(2), bv(1)).
        // After {g : α→β}: g=bv(0), f=bv(1), β=bv(2), α=bv(3). g's type = pi_def("_", bv(3), bv(2)).
        // After {a : α}: a=bv(0), g=bv(1), f=bv(2), β=bv(3), α=bv(4). a's type = bv(4).
        // After {b : α}: b=bv(0), a=bv(1), g=bv(2), f=bv(3), β=bv(4), α=bv(5). b's type = bv(5).
        // hf: @Eq (α→β) f g = eq_app(pi_def("_", bv(5), bv(4)), bv(3), bv(2)).
        // After hf: hf=bv(0), b=bv(1), a=bv(2), g=bv(3), f=bv(4), β=bv(5), α=bv(6).
        // ha: @Eq α a b = eq_app(bv(6), bv(2), bv(1)).
        // After ha: ha=bv(0), hf=bv(1), b=bv(2), a=bv(3), g=bv(4), f=bv(5), β=bv(6), α=bv(7).
        // concl: @Eq β (f a) (g b) = eq_app(bv(6), app(bv(5), bv(3)), app(bv(4), bv(2))).
        // congr : {α β : Sort 1} → {f g : α→β} → {a b : α} →
        //         @Eq (α→β) f g → @Eq α a b → @Eq β (f a) (g b)
        //
        // De Bruijn trace (carefully):
        // After {α}: α=bv(0).
        // After {β}: β=bv(0), α=bv(1).
        // f binder, type = α→β: in ctx [β,α], dom=bv(1)=α.
        //   cod is inside the `_` binder: ctx [_,β,α]: _=bv(0), β=bv(1), α=bv(2). β=bv(1).
        //   f's type = pi_def("_", bv(1), bv(1)). dom uses outer ctx bv(1)=α; cod bv(1) in inner ctx = β. ✓
        // After {f}: f=bv(0), β=bv(1), α=bv(2).
        // g binder, type = α→β: in ctx [f,β,α], dom=bv(2)=α.
        //   cod inside `_`: ctx [_,f,β,α]: _=bv(0), f=bv(1), β=bv(2), α=bv(3). β=bv(2).
        //   g's type = pi_def("_", bv(2), bv(2)). ✓
        // After {g}: g=bv(0), f=bv(1), β=bv(2), α=bv(3).
        // a binder: a=bv(0), type=α=bv(3). ✓
        // After {a}: a=bv(0), g=bv(1), f=bv(2), β=bv(3), α=bv(4).
        // b binder: b=bv(0), type=α=bv(4). ✓
        // After {b}: b=bv(0), a=bv(1), g=bv(2), f=bv(3), β=bv(4), α=bv(5).
        // hf binder: type = @Eq (α→β) f g.
        //   α→β in ctx [b,a,g,f,β,α]: dom=bv(5)=α; cod inside `_`: ctx [_,b,a,g,f,β,α]:
        //     _=bv(0), b=bv(1), a=bv(2), g=bv(3), f=bv(4), β=bv(5), α=bv(6). β=bv(5).
        //   α→β = pi_def("_", bv(5), bv(5)). ✓
        //   f=bv(3), g=bv(2). hf's type = eq_app(pi_def("_", bv(5), bv(5)), bv(3), bv(2)). ✓
        // After hf: hf=bv(0), b=bv(1), a=bv(2), g=bv(3), f=bv(4), β=bv(5), α=bv(6).
        // ha binder: @Eq α a b = eq_app(bv(6), bv(2), bv(1)). ✓
        // After ha: ha=bv(0), hf=bv(1), b=bv(2), a=bv(3), g=bv(4), f=bv(5), β=bv(6), α=bv(7).
        // Conclusion: @Eq β (f a) (g b) = eq_app(bv(6), app(bv(5), bv(3)), app(bv(4), bv(2))). ✓
        let congr_ty = pi_impl(
            sort1(),
            pi_impl(
                sort1(),
                pi_impl(
                    // f : α → β; dom=bv(1)=α (outer), cod=bv(1)=β (inner).
                    pi_def("_", bv(1), bv(1)),
                    pi_impl(
                        // g : α → β; dom=bv(2)=α (outer), cod=bv(2)=β (inner).
                        pi_def("_", bv(2), bv(2)),
                        pi_impl(
                            // a : α = bv(3) in ctx [g,f,β,α]
                            bv(3),
                            pi_impl(
                                // b : α = bv(4) in ctx [a,g,f,β,α]
                                bv(4),
                                pi_def(
                                    "hf",
                                    // @Eq (α→β) f g; α→β=pi_def("_",bv(5),bv(5)), f=bv(3), g=bv(2)
                                    eq_app(pi_def("_", bv(5), bv(5)), bv(3), bv(2)),
                                    pi_def(
                                        "ha",
                                        // @Eq α a b; α=bv(6), a=bv(2), b=bv(1)
                                        eq_app(bv(6), bv(2), bv(1)),
                                        // @Eq β (f a) (g b); β=bv(6), f=bv(5), a=bv(3), g=bv(4), b=bv(2)
                                        eq_app(bv(6), app(bv(5), bv(3)), app(bv(4), bv(2))),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        );
        env.add(Declaration::Axiom {
            name: Name::str("congr"),
            univ_params: vec![],
            ty: congr_ty,
        })?;

        Ok(env)
    }

    /// Register a type `T` as an axiom (opaque constant of type `Sort 1`).
    fn register_type(env: &mut Environment, name: &str) -> Result<(), EnvError> {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty: sort1(),
        })
    }

    /// Register a constant `c : T`.
    fn register_const(env: &mut Environment, name: &str, ty_name: &str) -> Result<(), EnvError> {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty: cst(ty_name),
        })
    }

    /// Register a function `f : A → B`.
    fn register_fn(
        env: &mut Environment,
        fname: &str,
        dom: &str,
        cod: &str,
    ) -> Result<(), EnvError> {
        env.add(Declaration::Axiom {
            name: Name::str(fname),
            univ_params: vec![],
            ty: pi_def("_", cst(dom), cst(cod)),
        })
    }

    // ── Unit tests for NO explain ─────────────────────────────────────────────

    #[test]
    fn test_explain_same_node_empty() {
        let mut cc = CongruenceClosure::new();
        let a = cst("a");
        let na = cc.add_term(&a);
        let steps = explain(&cc, na, na);
        assert!(steps.is_empty(), "same node should return empty steps");
    }

    #[test]
    fn test_explain_hypothesis_direct() {
        // h : a = b → explain(a, b) should yield one Hyp step.
        let a = cst("a");
        let b = cst("b");
        let mut cc = CongruenceClosure::new();
        let na = cc.add_term(&a);
        let nb = cc.add_term(&b);
        // Add proof-forest edge directly (since we can't call merge_with_reason on
        // a non-existent real eq hyp in the empty env — use the raw forest API).
        cc.proof_parent[na.0 as usize] = Some((
            nb,
            ProofLabel::Input {
                hyp_name: Name::str("h"),
                lhs_expr: a.clone(),
                rhs_expr: b.clone(),
                fwd: true,
            },
        ));
        let steps = explain(&cc, na, nb);
        assert_eq!(steps.len(), 1);
        assert!(matches!(
            &steps[0].reason,
            ExplainReason::Hyp { fwd: true, .. }
        ));
        assert_eq!(steps[0].lhs, a);
        assert_eq!(steps[0].rhs, b);
    }

    #[test]
    fn test_explain_symmetry() {
        // If we add edge a→b (fwd=true) and ask for explain(b, a), we should
        // get a step with fwd=false (needs Eq.symm).
        let a = cst("a");
        let b = cst("b");
        let mut cc = CongruenceClosure::new();
        let na = cc.add_term(&a);
        let nb = cc.add_term(&b);
        cc.proof_parent[na.0 as usize] = Some((
            nb,
            ProofLabel::Input {
                hyp_name: Name::str("h"),
                lhs_expr: a.clone(),
                rhs_expr: b.clone(),
                fwd: true,
            },
        ));
        let steps = explain(&cc, nb, na);
        assert_eq!(steps.len(), 1);
        // Traversed backward → fwd should be false.
        assert!(
            matches!(&steps[0].reason, ExplainReason::Hyp { fwd: false, .. }),
            "got {:?}",
            steps[0].reason
        );
    }

    #[test]
    fn test_explain_transitivity() {
        // h1: a=b, h2: b=c. explain(a, c) should give 2 steps.
        let a = cst("a");
        let b = cst("b");
        let c = cst("c");
        let mut cc = CongruenceClosure::new();
        let na = cc.add_term(&a);
        let nb = cc.add_term(&b);
        let nc = cc.add_term(&c);
        // Forest: a→b, b→c (both with fwd=true).
        cc.proof_parent[na.0 as usize] = Some((
            nb,
            ProofLabel::Input {
                hyp_name: Name::str("h1"),
                lhs_expr: a.clone(),
                rhs_expr: b.clone(),
                fwd: true,
            },
        ));
        cc.proof_parent[nb.0 as usize] = Some((
            nc,
            ProofLabel::Input {
                hyp_name: Name::str("h2"),
                lhs_expr: b.clone(),
                rhs_expr: c.clone(),
                fwd: true,
            },
        ));
        let steps = explain(&cc, na, nc);
        assert_eq!(steps.len(), 2);
        assert!(
            matches!(&steps[0].reason, ExplainReason::Hyp { name, .. } if name == &Name::str("h1"))
        );
        assert!(
            matches!(&steps[1].reason, ExplainReason::Hyp { name, .. } if name == &Name::str("h2"))
        );
    }

    // ── Proof-term builder tests ──────────────────────────────────────────────

    /// Helper: verify that `term` has type `goal_ty` in the given env.
    fn verify_type(env: &Environment, term: &Expr, goal_ty: &Expr) -> bool {
        let mut checker = TypeChecker::new(env);
        match checker.infer_type(term) {
            Ok(ty) => checker.is_def_eq(&ty, goal_ty),
            Err(_) => false,
        }
    }

    #[test]
    fn test_build_proof_refl() {
        let env = make_cc_env().expect("env creation should succeed");
        let step = ExplainStep {
            lhs: cst("a"),
            rhs: cst("a"),
            reason: ExplainReason::Refl,
        };
        // Without knowing type of "a" we can't build a refl proof — need "a" in env.
        // Register: A : Sort 1, a : A.
        let mut env = env;
        register_type(&mut env, "A").expect("register A");
        register_const(&mut env, "a", "A").expect("register a");

        let proof = build_eq_proof_with_locals(std::slice::from_ref(&step), &env, &[], &[]);
        assert!(proof.is_some(), "should produce a proof term");
        let term = proof.expect("proof term");
        let goal = eq_app(cst("A"), cst("a"), cst("a"));
        assert!(
            verify_type(&env, &term, &goal),
            "refl proof should typecheck"
        );
    }

    #[test]
    fn test_build_proof_hypothesis() {
        let mut env = make_cc_env().expect("env creation");
        register_type(&mut env, "A").expect("register A");
        register_const(&mut env, "a", "A").expect("register a");
        register_const(&mut env, "b", "A").expect("register b");

        // h : a = b — register as an axiom.
        let hyp_ty = eq_app(cst("A"), cst("a"), cst("b"));
        env.add(Declaration::Axiom {
            name: Name::str("h"),
            univ_params: vec![],
            ty: hyp_ty.clone(),
        })
        .expect("register h");

        let step = ExplainStep {
            lhs: cst("a"),
            rhs: cst("b"),
            reason: ExplainReason::Hyp {
                name: Name::str("h"),
                fwd: true,
            },
        };
        let proof = build_eq_proof_with_locals(&[step], &env, &[], &[]);
        assert!(proof.is_some());
        let term = proof.expect("proof term");
        // h itself proves a=b, so term should be Const("h").
        assert_eq!(term, cst("h"));
        assert!(verify_type(&env, &term, &hyp_ty));
    }

    #[test]
    fn test_build_proof_symmetry() {
        let mut env = make_cc_env().expect("env creation");
        register_type(&mut env, "A").expect("register A");
        register_const(&mut env, "a", "A").expect("register a");
        register_const(&mut env, "b", "A").expect("register b");

        // h : a = b, want b = a.
        let hyp_ty = eq_app(cst("A"), cst("a"), cst("b"));
        env.add(Declaration::Axiom {
            name: Name::str("h"),
            univ_params: vec![],
            ty: hyp_ty,
        })
        .expect("register h");

        let step = ExplainStep {
            lhs: cst("b"),
            rhs: cst("a"),
            // fwd=false means h proved rhs→lhs (a→b), so we need Eq.symm.
            reason: ExplainReason::Hyp {
                name: Name::str("h"),
                fwd: false,
            },
        };
        let proof = build_eq_proof_with_locals(&[step], &env, &[], &[]);
        assert!(proof.is_some(), "should build Eq.symm proof");
        let term = proof.expect("proof term");
        let goal = eq_app(cst("A"), cst("b"), cst("a"));
        assert!(
            verify_type(&env, &term, &goal),
            "symm proof should typecheck as b=a"
        );
    }

    #[test]
    fn test_build_proof_transitivity() {
        let mut env = make_cc_env().expect("env creation");
        register_type(&mut env, "A").expect("register A");
        register_const(&mut env, "a", "A").expect("register a");
        register_const(&mut env, "b", "A").expect("register b");
        register_const(&mut env, "c", "A").expect("register c");

        let h1_ty = eq_app(cst("A"), cst("a"), cst("b"));
        let h2_ty = eq_app(cst("A"), cst("b"), cst("c"));
        env.add(Declaration::Axiom {
            name: Name::str("h1"),
            univ_params: vec![],
            ty: h1_ty,
        })
        .expect("h1");
        env.add(Declaration::Axiom {
            name: Name::str("h2"),
            univ_params: vec![],
            ty: h2_ty,
        })
        .expect("h2");

        let steps = vec![
            ExplainStep {
                lhs: cst("a"),
                rhs: cst("b"),
                reason: ExplainReason::Hyp {
                    name: Name::str("h1"),
                    fwd: true,
                },
            },
            ExplainStep {
                lhs: cst("b"),
                rhs: cst("c"),
                reason: ExplainReason::Hyp {
                    name: Name::str("h2"),
                    fwd: true,
                },
            },
        ];

        let proof = build_eq_proof_with_locals(&steps, &env, &[], &[]);
        assert!(proof.is_some(), "trans proof should be produced");
        let term = proof.expect("proof term");
        let goal = eq_app(cst("A"), cst("a"), cst("c"));
        assert!(
            verify_type(&env, &term, &goal),
            "trans proof should typecheck as a=c"
        );
    }

    #[test]
    fn test_build_proof_single_congruence() {
        // h : a = b ⊢ f a = f b — should produce a congr-folded proof.
        let mut env = make_cc_env().expect("env creation");
        register_type(&mut env, "A").expect("register A");
        register_type(&mut env, "B").expect("register B");
        register_const(&mut env, "a", "A").expect("register a");
        register_const(&mut env, "b", "A").expect("register b");
        register_fn(&mut env, "f", "A", "B").expect("register f");

        let h_ty = eq_app(cst("A"), cst("a"), cst("b"));
        env.add(Declaration::Axiom {
            name: Name::str("h"),
            univ_params: vec![],
            ty: h_ty,
        })
        .expect("h");

        let fa = app(cst("f"), cst("a"));
        let fb = app(cst("f"), cst("b"));

        let h_step = ExplainStep {
            lhs: cst("a"),
            rhs: cst("b"),
            reason: ExplainReason::Hyp {
                name: Name::str("h"),
                fwd: true,
            },
        };

        let congr_step = ExplainStep {
            lhs: fa.clone(),
            rhs: fb.clone(),
            reason: ExplainReason::Congruence {
                arg_pairs: vec![(cst("a"), cst("b"), vec![h_step])],
            },
        };

        let proof = build_eq_proof_with_locals(&[congr_step], &env, &[], &[]);
        assert!(proof.is_some(), "congruence proof should be produced");
        let term = proof.expect("proof term");
        let goal = eq_app(cst("B"), fa, fb);
        assert!(
            verify_type(&env, &term, &goal),
            "congruence proof should typecheck as f a = f b"
        );
    }

    #[test]
    fn test_build_proof_multi_arg_congruence() {
        // h1: a=c, h2: b=d ⊢ g a b = g c d
        // Need g : A → A → B.
        let mut env = make_cc_env().expect("env creation");
        register_type(&mut env, "A").expect("A");
        register_type(&mut env, "B").expect("B");
        register_const(&mut env, "a", "A").expect("a");
        register_const(&mut env, "b", "A").expect("b");
        register_const(&mut env, "c", "A").expect("c");
        register_const(&mut env, "d", "A").expect("d");
        // g : A → A → B
        env.add(Declaration::Axiom {
            name: Name::str("g"),
            univ_params: vec![],
            ty: pi_def("_", cst("A"), pi_def("_", cst("A"), cst("B"))),
        })
        .expect("g");

        let h1_ty = eq_app(cst("A"), cst("a"), cst("c"));
        let h2_ty = eq_app(cst("A"), cst("b"), cst("d"));
        env.add(Declaration::Axiom {
            name: Name::str("h1"),
            univ_params: vec![],
            ty: h1_ty,
        })
        .expect("h1");
        env.add(Declaration::Axiom {
            name: Name::str("h2"),
            univ_params: vec![],
            ty: h2_ty,
        })
        .expect("h2");

        let gab = app(app(cst("g"), cst("a")), cst("b"));
        let gcd = app(app(cst("g"), cst("c")), cst("d"));

        let congr_step = ExplainStep {
            lhs: gab.clone(),
            rhs: gcd.clone(),
            reason: ExplainReason::Congruence {
                arg_pairs: vec![
                    (
                        cst("a"),
                        cst("c"),
                        vec![ExplainStep {
                            lhs: cst("a"),
                            rhs: cst("c"),
                            reason: ExplainReason::Hyp {
                                name: Name::str("h1"),
                                fwd: true,
                            },
                        }],
                    ),
                    (
                        cst("b"),
                        cst("d"),
                        vec![ExplainStep {
                            lhs: cst("b"),
                            rhs: cst("d"),
                            reason: ExplainReason::Hyp {
                                name: Name::str("h2"),
                                fwd: true,
                            },
                        }],
                    ),
                ],
            },
        };

        let proof = build_eq_proof_with_locals(&[congr_step], &env, &[], &[]);
        assert!(
            proof.is_some(),
            "multi-arg congruence proof should be produced"
        );
        let term = proof.expect("proof term");
        let goal = eq_app(cst("B"), gab, gcd);
        assert!(
            verify_type(&env, &term, &goal),
            "multi-arg congruence proof should typecheck as g a b = g c d"
        );
    }

    #[test]
    fn test_build_proof_nested_congruence() {
        // h : a = b ⊢ g (f a) = g (f b)
        let mut env = make_cc_env().expect("env creation");
        register_type(&mut env, "A").expect("A");
        register_type(&mut env, "B").expect("B");
        register_const(&mut env, "a", "A").expect("a");
        register_const(&mut env, "b", "A").expect("b");
        register_fn(&mut env, "f", "A", "B").expect("f");
        register_fn(&mut env, "g", "B", "A").expect("g");

        let h_ty = eq_app(cst("A"), cst("a"), cst("b"));
        env.add(Declaration::Axiom {
            name: Name::str("h"),
            univ_params: vec![],
            ty: h_ty,
        })
        .expect("h");

        let fa = app(cst("f"), cst("a"));
        let fb = app(cst("f"), cst("b"));
        let gfa = app(cst("g"), fa.clone());
        let gfb = app(cst("g"), fb.clone());

        // Inner step: a = b.
        let h_step = ExplainStep {
            lhs: cst("a"),
            rhs: cst("b"),
            reason: ExplainReason::Hyp {
                name: Name::str("h"),
                fwd: true,
            },
        };
        // Middle step: f a = f b.
        let f_step = ExplainStep {
            lhs: fa.clone(),
            rhs: fb.clone(),
            reason: ExplainReason::Congruence {
                arg_pairs: vec![(cst("a"), cst("b"), vec![h_step])],
            },
        };
        // Outer step: g (f a) = g (f b).
        let outer_step = ExplainStep {
            lhs: gfa.clone(),
            rhs: gfb.clone(),
            reason: ExplainReason::Congruence {
                arg_pairs: vec![(fa.clone(), fb.clone(), vec![f_step])],
            },
        };

        let proof = build_eq_proof_with_locals(&[outer_step], &env, &[], &[]);
        assert!(
            proof.is_some(),
            "nested congruence proof should be produced"
        );
        let term = proof.expect("proof term");
        let goal = eq_app(cst("A"), gfa, gfb);
        assert!(
            verify_type(&env, &term, &goal),
            "nested congruence proof should typecheck"
        );
    }

    #[test]
    fn test_explain_and_build_via_cc() {
        // Integration test: add a=b to CC via merge_with_reason and explain.
        let a = cst("a");
        let b = cst("b");
        let mut cc = CongruenceClosure::new();
        let na = cc.add_term(&a);
        let nb = cc.add_term(&b);
        // Manually set a proof-forest edge (simulating what merge_with_reason will do).
        cc.proof_parent[na.0 as usize] = Some((
            nb,
            ProofLabel::Input {
                hyp_name: Name::str("h"),
                lhs_expr: a.clone(),
                rhs_expr: b.clone(),
                fwd: true,
            },
        ));
        let steps = explain(&cc, na, nb);
        assert_eq!(steps.len(), 1);
        assert_eq!(steps[0].lhs, a);
        assert_eq!(steps[0].rhs, b);
    }

    #[test]
    fn test_decompose_app_spine_basic() {
        // App(App(f, a), b) → (f, [a, b])
        let f = cst("f");
        let a = cst("a");
        let b = cst("b");
        let expr = app(app(f.clone(), a.clone()), b.clone());
        let (head, args) = decompose_app_spine(&expr);
        assert_eq!(head, f);
        assert_eq!(args, vec![a, b]);
    }

    #[test]
    fn test_decompose_app_spine_leaf() {
        let f = cst("f");
        let (head, args) = decompose_app_spine(&f);
        assert_eq!(head, f);
        assert!(args.is_empty());
    }
}
