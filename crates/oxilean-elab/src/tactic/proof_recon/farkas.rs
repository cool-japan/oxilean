//! Farkas certificate → kernel proof term reconstruction.
//!
//! Attempts to construct a kernel-verified proof of `False` (or the
//! contradiction) from a [`FarkasCert`] produced by
//! `find_farkas_certificate`. The proof term is built from transitivity
//! chains that the opaque-Int kernel can verify structurally.
//!
//! # Soundness gate
//! All reconstructed terms are verified by `TypeChecker::infer_type` +
//! `is_def_eq` before being accepted. If verification fails, `None` is
//! returned and the caller falls back to a placeholder proof.
//!
//! # Architecture
//! The `FarkasCert.sources` field (populated by the provenance plumbing in
//! `functions_3.rs`) maps each `FarkasCertEntry.constraint_index` to the
//! original hypothesis name and kernel type. This allows proof reconstruction
//! to:
//! 1. Look up the `Expr::FVar` or `Expr::Const` proof term for each hypothesis.
//! 2. Attempt to build a kernel-checkable contradiction proof by combining
//!    hypothesis proof terms via transitivity lemmas.
//!
//! # Primary Strategy: Transitivity Cycle (cycle 6)
//! For the tractable opaque-Int class where all multipliers are 1:
//! Given hypotheses of the form `a ≤ b`, `b < c`, `c ≤ a`, fold them into
//! a cycle ending with `Int.lt a a`, then close via `Int.lt_irrefl'`.
//!
//! Lemmas used (all must be in env via `register_omega_helper`):
//! - `Int.le_trans : ∀ a b c, Int.le a b → Int.le b c → Int.le a c`
//! - `Int.lt_of_le_of_lt : ∀ a b c, Int.le a b → Int.lt b c → Int.lt a c`
//! - `Int.lt_of_lt_of_le : ∀ a b c, Int.lt a b → Int.le b c → Int.lt a c`
//! - `Int.lt_trans : ∀ a b c, Int.lt a b → Int.lt b c → Int.lt a c`
//! - `Int.lt_irrefl' : ∀ a, Int.lt a a → False`
//!
//! # Strategy 2: Arithmetic Chain Close (cycle 8)
//! For unit-multiplier linear chains (not syntactic cycles) where both
//! endpoints are ground `Literal::Int` values with an arithmetic contradiction:
//! Given `h_chain : a ≤ b` (with `a > b` as integers), close via:
//! `Int.not_le_of_ble_false a b (Eq.refl Bool.false) h_chain : False`
//! (valid since `Int.ble a b` reduces to `Bool.false` under the kernel whnf).
//! Similarly for the strict case using `Int.not_lt_of_blt_false`.
//!
//! # Fallback Strategies
//! The legacy `Int.add_le_add` + `Int.absurd_le_zero` strategies are kept as
//! fallback for any cases they might handle.
//!
//! # Limitations (documented)
//! - Only handles **unit multipliers** for the transitivity-cycle class.
//! - The source hypotheses must be `Int.le`- or `Int.lt`-typed.
//! - Nonlinear atoms (products from Positivstellensatz augmentation) are
//!   not reconstructable; sources with `hyp_name: None` return `None`.

use oxilean_kernel::Node;
use oxilean_kernel::{
    Declaration, Environment, Expr, FVarId, Level, Literal, LocalDecl, Name, TypeChecker,
};
use oxilean_meta::tactic::linear_combination::{ConOrient, FarkasCert};

// ── Strictness marker ─────────────────────────────────────────────────────────

/// Strictness of an integer ordering constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Strictness {
    /// Non-strict: `a ≤ b` (`Int.le`).
    Le,
    /// Strict: `a < b` (`Int.lt`).
    Lt,
}

/// A constraint extracted from a Farkas certificate entry for the cycle builder.
#[derive(Debug, Clone)]
struct ConstraintPart {
    /// Left-hand side expression.
    lhs: Expr,
    /// Right-hand side expression.
    rhs: Expr,
    /// Whether the relation is strict (`<`) or non-strict (`≤`).
    strictness: Strictness,
    /// Proof term (FVar or Const) for this hypothesis.
    proof: Expr,
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Attempt to build a kernel-verified proof term from a Farkas certificate.
///
/// # Parameters
/// - `cert`: The Farkas refutation certificate from `find_farkas_certificate`,
///   including the `sources` provenance field populated by the nlinarith plumbing.
/// - `goal`: The kernel type of the goal (typically `Expr::Const("False")` or
///   `Int.le lhs rhs`).
/// - `hyps`: Hypothesis `(name, type)` pairs from the current elaboration context.
/// - `locals`: Free variable context `(fvar_id, name, type)` triples injected
///   into the TypeChecker so that `Expr::FVar` references can be resolved.
/// - `env`: The global kernel environment.
///
/// # Returns
/// `Some(proof_term)` if a kernel-verified proof was constructed; `None` otherwise.
///
/// # Soundness guarantee
/// This function NEVER returns an unverified proof term. All returned
/// `Expr`s have been checked by the kernel type-checker against the expected
/// goal type before being returned. Returning `None` is always safe.
pub fn farkas_cert_to_expr(
    cert: &FarkasCert,
    goal: &Expr,
    hyps: &[(Name, Expr)],
    locals: &[(FVarId, Name, Expr)],
    env: &Environment,
) -> Option<Expr> {
    // Guard 1: certificate must be a valid refutation.
    if !cert.is_valid_refutation() {
        return None;
    }

    // Guard 2: all multipliers must be integer-valued.
    for entry in &cert.entries {
        if entry.multiplier.denom != 1 {
            return None;
        }
    }

    // Guard 3: all constraint indices must be in bounds.
    for entry in &cert.entries {
        if entry.constraint_index >= cert.sources.len() {
            return None;
        }
    }

    // Guard 4: no entries (degenerate).
    if cert.entries.is_empty() {
        return None;
    }

    // Guard 5: sources must be non-empty (provenance required for reconstruction).
    if cert.sources.is_empty() {
        return None;
    }

    // Guard 6: all participating entries must have named sources (not augmented/negated-goal).
    for entry in &cert.entries {
        let src = cert.sources.get(entry.constraint_index)?;
        src.hyp_name.as_ref()?;
    }

    // Primary strategy: transitivity-cycle builder (unit multipliers, mixed le/lt).
    // Try this first for any number of constraints ≥ 2.
    if cert.entries.len() >= 2 {
        let parts: Vec<ConstraintPart> = cert
            .entries
            .iter()
            .filter_map(|e| extract_constraint_part(e, cert, locals))
            .collect();

        if parts.len() == cert.entries.len() {
            // All parts extracted successfully — try the cycle builder.
            if let Some(term) = build_transitivity_cycle_proof(&parts, goal, hyps, locals, env) {
                return Some(term);
            }
        }
    }

    // Fallback strategies (legacy add_le_add + absurd_le_zero).
    match cert.entries.len() {
        1 => build_single_constraint_proof(cert, goal, hyps, locals, env),
        2 => build_two_constraint_proof(cert, goal, hyps, locals, env),
        _ => build_multi_constraint_proof(cert, goal, hyps, locals, env),
    }
}

// ── Transitivity cycle builder ────────────────────────────────────────────────

/// Extract a `ConstraintPart` from a single `FarkasCertEntry`.
///
/// Only succeeds for unit-multiplier entries with named hypothesis sources
/// whose types are recognisable `Int.le` or `Int.lt` expressions.
fn extract_constraint_part(
    entry: &oxilean_meta::tactic::linear_combination::FarkasCertEntry,
    cert: &FarkasCert,
    locals: &[(FVarId, Name, Expr)],
) -> Option<ConstraintPart> {
    // Unit multiplier only for this tractable class.
    if entry.multiplier.numer != 1 || entry.multiplier.denom != 1 {
        return None;
    }
    let src = cert.sources.get(entry.constraint_index)?;
    let hyp_name = src.hyp_name.as_ref()?;
    let (lhs, rhs, strictness) = extract_int_rel(&src.hyp_type)?;
    let proof = hyp_proof_term(hyp_name, locals);
    Some(ConstraintPart {
        lhs: lhs.clone(),
        rhs: rhs.clone(),
        strictness,
        proof,
    })
}

/// Build a contradiction proof from a set of ordered relational constraints by
/// finding a cycle and folding it with transitivity lemmas.
///
/// Algorithm:
/// 1. Try all starting points to find a linear chain where `chain[i].rhs == chain[i+1].lhs`.
/// 2. The chain must close: `chain.last().rhs == chain.first().lhs`.
/// 3. At least one edge must be strict (`Lt`).
/// 4. Fold left-to-right with the appropriate transitivity lemma per edge pair.
/// 5. Close with `Int.lt_irrefl'`.
/// 6. Kernel-gate the final term.
fn build_transitivity_cycle_proof(
    parts: &[ConstraintPart],
    goal: &Expr,
    hyps: &[(Name, Expr)],
    locals: &[(FVarId, Name, Expr)],
    env: &Environment,
) -> Option<Expr> {
    let n = parts.len();
    if n < 2 {
        return None;
    }

    // Try every possible start index for the chain ordering.
    for start in 0..n {
        if let Some(term) = try_cycle_from(parts, start, goal, hyps, locals, env) {
            return Some(term);
        }
    }
    None
}

/// Attempt to build a contradiction proof starting from `parts[start]` as the first edge.
///
/// Greedily chains edges in order from `start`, wrapping around, then attempts
/// two close strategies:
///
/// ## Strategy 1: Syntactic cycle close
/// If the chain closes syntactically (`acc_rhs == acc_lhs`) with at least one
/// strict edge, close via `Int.lt_irrefl' acc_lhs acc_proof : False`.
///
/// ## Strategy 2: Arithmetic chain close (cycle 8)
/// If both chain endpoints are ground `Literal::Int` values with an arithmetic
/// contradiction (the relation is false for those values), close via:
/// - Le case: `Int.not_le_of_ble_false acc_lhs acc_rhs (Eq.refl Bool.false) acc_proof`
///   (valid because `Int.ble acc_lhs acc_rhs` reduces to `Bool.false` when `acc_lhs > acc_rhs`)
/// - Lt case: `Int.not_lt_of_blt_false acc_lhs acc_rhs (Eq.refl Bool.false) acc_proof`
///   (valid because `Int.blt acc_lhs acc_rhs` reduces to `Bool.false` when `acc_lhs >= acc_rhs`)
fn try_cycle_from(
    parts: &[ConstraintPart],
    start: usize,
    goal: &Expr,
    hyps: &[(Name, Expr)],
    locals: &[(FVarId, Name, Expr)],
    env: &Environment,
) -> Option<Expr> {
    let n = parts.len();
    // Build the permutation starting at `start`.
    let ordered: Vec<&ConstraintPart> = (0..n).map(|i| &parts[(start + i) % n]).collect();

    // Verify the chain forms a linear sequence: ordered[i].rhs == ordered[i+1].lhs.
    // (Does NOT require the chain to close syntactically back to ordered[0].lhs.)
    for i in 0..n - 1 {
        if ordered[i].rhs != ordered[i + 1].lhs {
            return None;
        }
    }

    // Fold the chain: accumulate (lhs, rhs, strictness, proof_term).
    let first = ordered[0];
    let acc_lhs = first.lhs.clone();
    let mut acc_rhs = first.rhs.clone();
    let mut acc_strictness = first.strictness;
    let mut acc_proof = first.proof.clone();

    for part in ordered.iter().skip(1) {
        let (new_proof, new_rhs, new_strictness) = fold_step(
            &acc_lhs,
            &acc_rhs,
            &part.rhs,
            acc_proof,
            part.proof.clone(),
            acc_strictness,
            part.strictness,
        );
        acc_rhs = new_rhs;
        acc_strictness = new_strictness;
        acc_proof = new_proof;
        // acc_lhs stays the same (it's the chain's start point)
    }

    // Strategy 1: Syntactic cycle close (existing).
    // Chain closes syntactically (acc_rhs == acc_lhs) and is strictly Lt.
    if acc_lhs == acc_rhs {
        if acc_strictness == Strictness::Lt {
            let lt_irrefl_prime = Expr::Const(Name::str("Int.lt_irrefl'"), vec![]);
            let term = mk_app2(lt_irrefl_prime, acc_lhs, acc_proof);
            if verify_proof_term(&term, goal, hyps, locals, env) {
                return Some(term);
            }
        }
        // All-Le syntactic cycle: not a contradiction (antisymmetry, not False).
        return None;
    }

    // Strategy 2: Arithmetic chain close (Bool-reflection, cycle 8).
    // Chain is `acc_lhs rel acc_rhs` where both endpoints are ground Int literals.
    // If the arithmetic values contradict the relation, close via Bool-reflection.
    if let (Some(a_val), Some(b_val)) = (extract_int_lit(&acc_lhs), extract_int_lit(&acc_rhs)) {
        let arith_contradiction = match acc_strictness {
            // chain asserts a ≤ b, but a > b as integers → Int.ble a b = Bool.false
            Strictness::Le => a_val > b_val,
            // chain asserts a < b, but a ≥ b as integers → Int.blt a b = Bool.false
            Strictness::Lt => a_val >= b_val,
        };
        if arith_contradiction {
            let rfl_false = mk_bool_false_rfl();
            let not_lemma = match acc_strictness {
                Strictness::Le => Expr::Const(Name::str("Int.not_le_of_ble_false"), vec![]),
                Strictness::Lt => Expr::Const(Name::str("Int.not_lt_of_blt_false"), vec![]),
            };
            // not_lemma acc_lhs acc_rhs rfl_false acc_proof : False
            let term = mk_app4(not_lemma, acc_lhs, acc_rhs, rfl_false, acc_proof);
            if verify_proof_term(&term, goal, hyps, locals, env) {
                return Some(term);
            }
        }
    }

    None
}

/// Fold one transitivity step, producing a new proof of `acc_lhs rel part_rhs`.
///
/// Returns `(new_proof, new_rhs, new_strictness)`.
///
/// The `new_rhs` is always `part_rhs` (extending the chain), and
/// `new_strictness` follows the rule:
/// - Le + Le → Le (`Int.le_trans`)
/// - Le + Lt → Lt (`Int.lt_of_le_of_lt`)
/// - Lt + Le → Lt (`Int.lt_of_lt_of_le`)
/// - Lt + Lt → Lt (`Int.lt_trans`)
fn fold_step(
    acc_lhs: &Expr,
    acc_rhs: &Expr,
    part_rhs: &Expr,
    acc_proof: Expr,
    part_proof: Expr,
    acc_strict: Strictness,
    part_strict: Strictness,
) -> (Expr, Expr, Strictness) {
    match (acc_strict, part_strict) {
        (Strictness::Le, Strictness::Le) => {
            // Int.le_trans acc_lhs acc_rhs part_rhs acc_proof part_proof
            let le_trans = Expr::Const(Name::str("Int.le_trans"), vec![]);
            let new_proof = mk_app5(
                le_trans,
                acc_lhs.clone(),
                acc_rhs.clone(),
                part_rhs.clone(),
                acc_proof,
                part_proof,
            );
            (new_proof, part_rhs.clone(), Strictness::Le)
        }
        (Strictness::Le, Strictness::Lt) => {
            // Int.lt_of_le_of_lt acc_lhs acc_rhs part_rhs acc_proof part_proof
            let lt_of_le_of_lt = Expr::Const(Name::str("Int.lt_of_le_of_lt"), vec![]);
            let new_proof = mk_app5(
                lt_of_le_of_lt,
                acc_lhs.clone(),
                acc_rhs.clone(),
                part_rhs.clone(),
                acc_proof,
                part_proof,
            );
            (new_proof, part_rhs.clone(), Strictness::Lt)
        }
        (Strictness::Lt, Strictness::Le) => {
            // Int.lt_of_lt_of_le acc_lhs acc_rhs part_rhs acc_proof part_proof
            let lt_of_lt_of_le = Expr::Const(Name::str("Int.lt_of_lt_of_le"), vec![]);
            let new_proof = mk_app5(
                lt_of_lt_of_le,
                acc_lhs.clone(),
                acc_rhs.clone(),
                part_rhs.clone(),
                acc_proof,
                part_proof,
            );
            (new_proof, part_rhs.clone(), Strictness::Lt)
        }
        (Strictness::Lt, Strictness::Lt) => {
            // Int.lt_trans acc_lhs acc_rhs part_rhs acc_proof part_proof
            let lt_trans = Expr::Const(Name::str("Int.lt_trans"), vec![]);
            let new_proof = mk_app5(
                lt_trans,
                acc_lhs.clone(),
                acc_rhs.clone(),
                part_rhs.clone(),
                acc_proof,
                part_proof,
            );
            (new_proof, part_rhs.clone(), Strictness::Lt)
        }
    }
}

// ── Legacy builders (kept as fallback) ────────────────────────────────────────

/// Single-constraint case: the constraint alone is contradictory.
///
/// Only handles the case where the single hypothesis has type `Int.le k 0` with
/// the combined_rhs showing k > 0. Tries to derive False via `Int.absurd_le_zero`.
fn build_single_constraint_proof(
    cert: &FarkasCert,
    goal: &Expr,
    hyps: &[(Name, Expr)],
    locals: &[(FVarId, Name, Expr)],
    env: &Environment,
) -> Option<Expr> {
    let entry = cert.entries.first()?;
    let src = cert.sources.get(entry.constraint_index)?;
    let hyp_name = src.hyp_name.as_ref()?;

    // Only handle Int.le hypotheses.
    let (lhs, _rhs) = extract_int_le(&src.hyp_type)?;

    let h_proof = hyp_proof_term(hyp_name, locals);

    // Try: Int.absurd_le_zero lhs zero_lt_lhs h_proof : False
    let zero_lt_lhs = Expr::Const(Name::str("Int.zero_lt_one"), vec![]);
    let absurd_le = Expr::Const(Name::str("Int.absurd_le_zero"), vec![]);
    let term = mk_app3(absurd_le, lhs.clone(), zero_lt_lhs, h_proof);

    if verify_proof_term(&term, goal, hyps, locals, env) {
        return Some(term);
    }

    None
}

/// Two-constraint case: the most common output of `find_farkas_certificate`.
///
/// Tries summation and chain strategies via `Int.add_le_add` and `Int.le_trans`.
fn build_two_constraint_proof(
    cert: &FarkasCert,
    goal: &Expr,
    hyps: &[(Name, Expr)],
    locals: &[(FVarId, Name, Expr)],
    env: &Environment,
) -> Option<Expr> {
    let e0 = cert.entries.first()?;
    let e1 = cert.entries.get(1)?;
    let s0 = cert.sources.get(e0.constraint_index)?;
    let s1 = cert.sources.get(e1.constraint_index)?;
    let name0 = s0.hyp_name.as_ref()?;
    let name1 = s1.hyp_name.as_ref()?;

    let h0_proof = hyp_proof_term(name0, locals);
    let h1_proof = hyp_proof_term(name1, locals);

    if let (Some((l0, r0)), Some((l1, r1))) =
        (extract_int_le(&s0.hyp_type), extract_int_le(&s1.hyp_type))
    {
        // Strategy: Summation via Int.add_le_add.
        let add_le_add = Expr::Const(Name::str("Int.add_le_add"), vec![]);
        let sum_proof = mk_app6(
            add_le_add,
            l0.clone(),
            r0.clone(),
            l1.clone(),
            r1.clone(),
            h0_proof.clone(),
            h1_proof.clone(),
        );

        let int_add = Expr::Const(Name::str("Int.add"), vec![]);
        let sum_lhs = mk_app2(int_add.clone(), l0.clone(), l1.clone());
        let sum_rhs = mk_app2(int_add, r0.clone(), r1.clone());
        let _ = mk_int_le(sum_lhs.clone(), sum_rhs.clone());

        let zero_lt_sum = Expr::Const(Name::str("Int.zero_lt_one"), vec![]);
        let absurd_le = Expr::Const(Name::str("Int.absurd_le_zero"), vec![]);
        let term = mk_app3(
            absurd_le.clone(),
            sum_lhs.clone(),
            zero_lt_sum.clone(),
            sum_proof.clone(),
        );

        if verify_proof_term(&term, goal, hyps, locals, env) {
            return Some(term);
        }

        // le_trans chain: h0: a ≤ b, h1: b ≤ a → chain gives a ≤ a
        if l0 == r1 {
            let le_trans = Expr::Const(Name::str("Int.le_trans"), vec![]);
            let chain_proof = mk_app5(
                le_trans,
                l0.clone(),
                l1.clone(),
                r0.clone(),
                h0_proof.clone(),
                h1_proof.clone(),
            );
            let term2 = mk_app3(
                Expr::Const(Name::str("Int.absurd_le_zero"), vec![]),
                l0.clone(),
                Expr::Const(Name::str("Int.zero_lt_one"), vec![]),
                chain_proof,
            );
            if verify_proof_term(&term2, goal, hyps, locals, env) {
                return Some(term2);
            }
        }

        if r0 == l1 {
            let le_trans = Expr::Const(Name::str("Int.le_trans"), vec![]);
            let chain_proof = mk_app5(
                le_trans,
                l0.clone(),
                r0.clone(),
                r1.clone(),
                h0_proof.clone(),
                h1_proof.clone(),
            );
            let term3 = mk_app3(
                Expr::Const(Name::str("Int.absurd_le_zero"), vec![]),
                l0.clone(),
                Expr::Const(Name::str("Int.zero_lt_one"), vec![]),
                chain_proof,
            );
            if verify_proof_term(&term3, goal, hyps, locals, env) {
                return Some(term3);
            }
        }
    }

    None
}

/// Multi-constraint case (≥ 3 entries).
///
/// Folds via `Int.add_le_add` left-associatively, then closes with
/// `Int.absurd_le_zero`. Falls through to `None` if kernel verification fails.
fn build_multi_constraint_proof(
    cert: &FarkasCert,
    goal: &Expr,
    hyps: &[(Name, Expr)],
    locals: &[(FVarId, Name, Expr)],
    env: &Environment,
) -> Option<Expr> {
    let mut parts: Vec<(Expr, Expr, Expr)> = Vec::new();
    for entry in &cert.entries {
        let src = cert.sources.get(entry.constraint_index)?;
        let hyp_name = src.hyp_name.as_ref()?;
        let (lhs, rhs) = extract_int_le(&src.hyp_type)?;
        let h_proof = hyp_proof_term(hyp_name, locals);
        parts.push((lhs.clone(), rhs.clone(), h_proof));
    }

    if parts.len() < 2 {
        return None;
    }

    let add_le_add = Expr::Const(Name::str("Int.add_le_add"), vec![]);
    let int_add = Expr::Const(Name::str("Int.add"), vec![]);

    let (mut acc_lhs, mut acc_rhs, mut acc_proof) = parts[0].clone();

    for (lhs, rhs, h_proof) in parts.into_iter().skip(1) {
        let new_proof = mk_app6(
            add_le_add.clone(),
            acc_lhs.clone(),
            acc_rhs.clone(),
            lhs.clone(),
            rhs.clone(),
            acc_proof,
            h_proof,
        );
        let new_lhs = mk_app2(int_add.clone(), acc_lhs, lhs);
        let new_rhs = mk_app2(int_add.clone(), acc_rhs, rhs);
        acc_lhs = new_lhs;
        acc_rhs = new_rhs;
        acc_proof = new_proof;
    }

    let zero_lt_one = Expr::Const(Name::str("Int.zero_lt_one"), vec![]);
    let absurd_le = Expr::Const(Name::str("Int.absurd_le_zero"), vec![]);
    let term = mk_app3(absurd_le, acc_lhs, zero_lt_one, acc_proof);

    if verify_proof_term(&term, goal, hyps, locals, env) {
        return Some(term);
    }

    None
}

// ── Kernel verification ───────────────────────────────────────────────────────

/// Verify a candidate proof term against the expected goal type using the kernel.
///
/// Returns `true` iff the kernel accepts the term and its inferred type is
/// definitionally equal to `goal`.
///
/// # Hypothesis augmentation
/// Each `(name, ty)` in `hyps` is registered as an axiom in a cloned
/// environment so that `Expr::Const(name)` references in the proof term can be
/// kernel-checked. Hypotheses already present in `env` are skipped.
///
/// # FVar threading
/// Each `(fvar_id, name, ty)` in `locals` is pushed into the TypeChecker's
/// local context via `push_local` so that `Expr::FVar(fvar_id)` references
/// in the proof term can be resolved during type inference.
pub fn verify_proof_term(
    term: &Expr,
    goal: &Expr,
    hyps: &[(Name, Expr)],
    locals: &[(FVarId, Name, Expr)],
    env: &Environment,
) -> bool {
    let mut augmented_env = env.clone();
    for (name, ty) in hyps {
        if augmented_env.contains(name) {
            continue;
        }
        let _ = augmented_env.add(Declaration::Axiom {
            name: name.clone(),
            univ_params: vec![],
            ty: ty.clone(),
        });
    }

    let mut checker = TypeChecker::new(&augmented_env);

    for (fvar, name, ty) in locals {
        checker.push_local(LocalDecl {
            fvar: *fvar,
            name: name.clone(),
            ty: ty.clone(),
            val: None,
        });
    }

    match checker.infer_type(term) {
        Ok(inferred_ty) => checker.is_def_eq(&inferred_ty, goal),
        Err(_) => false,
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Build a proof term reference for a hypothesis by name.
///
/// If the hypothesis name matches a local variable (with a known `FVarId`),
/// returns `Expr::FVar(fvar_id)`. Otherwise falls back to `Expr::Const(name)`.
fn hyp_proof_term(name: &Name, locals: &[(FVarId, Name, Expr)]) -> Expr {
    for (fvar, local_name, _) in locals {
        if local_name == name {
            return Expr::FVar(*fvar);
        }
    }
    Expr::Const(name.clone(), vec![])
}

/// Extract `(lhs, rhs, Strictness)` from `Int.le` or `Int.lt` expressions.
///
/// Matches:
/// - `App(App(Const("Int.le", _), lhs), rhs)` → `Le`
/// - `App(App(Const("Int.lt", _), lhs), rhs)` → `Lt`
fn extract_int_rel(expr: &Expr) -> Option<(&Expr, &Expr, Strictness)> {
    if let Expr::App(func, rhs) = expr {
        if let Expr::App(func2, lhs) = func.as_ref() {
            if let Expr::Const(name, _) = func2.as_ref() {
                let s = name.to_string();
                if s == "Int.le" {
                    return Some((lhs.as_ref(), rhs.as_ref(), Strictness::Le));
                }
                if s == "Int.lt" {
                    return Some((lhs.as_ref(), rhs.as_ref(), Strictness::Lt));
                }
            }
        }
    }
    None
}

/// Extract `(lhs, rhs)` from `App(App(Const("Int.le", _), lhs), rhs)`.
fn extract_int_le(expr: &Expr) -> Option<(&Expr, &Expr)> {
    if let Expr::App(func, rhs) = expr {
        if let Expr::App(func2, lhs) = func.as_ref() {
            if let Expr::Const(name, _) = func2.as_ref() {
                if name.to_string() == "Int.le" {
                    return Some((lhs.as_ref(), rhs.as_ref()));
                }
            }
        }
    }
    None
}

/// Build `App(App(f, a), b)`.
fn mk_app2(f: Expr, a: Expr, b: Expr) -> Expr {
    Expr::App(
        Node::new(Expr::App(Node::new(f), Node::new(a))),
        Node::new(b),
    )
}

/// Build `App(App(App(f, a), b), c)`.
fn mk_app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    Expr::App(Node::new(mk_app2(f, a, b)), Node::new(c))
}

/// Build `App(App(App(App(App(f, a), b), c), d), e)` — 5 args.
fn mk_app5(f: Expr, a: Expr, b: Expr, c: Expr, d: Expr, e: Expr) -> Expr {
    Expr::App(
        Node::new(Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::App(
                    Node::new(Expr::App(Node::new(f), Node::new(a))),
                    Node::new(b),
                )),
                Node::new(c),
            )),
            Node::new(d),
        )),
        Node::new(e),
    )
}

/// Build `App(App(App(App(App(App(f, a), b), c), d), e), g)` — 6 args.
fn mk_app6(f: Expr, a: Expr, b: Expr, c: Expr, d: Expr, e: Expr, g: Expr) -> Expr {
    Expr::App(
        Node::new(Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::App(
                    Node::new(Expr::App(
                        Node::new(Expr::App(Node::new(f), Node::new(a))),
                        Node::new(b),
                    )),
                    Node::new(c),
                )),
                Node::new(d),
            )),
            Node::new(e),
        )),
        Node::new(g),
    )
}

/// Build `App(App(App(App(f, a), b), c), d)` — 4 args.
fn mk_app4(f: Expr, a: Expr, b: Expr, c: Expr, d: Expr) -> Expr {
    Expr::App(Node::new(mk_app3(f, a, b, c)), Node::new(d))
}

/// Extract an i64 integer value from an expression representing an integer.
///
/// Recognises the Lean 4 integer representations:
/// - `Int.ofNat (Lit Nat n)` → `n as i64` (for n ≤ i64::MAX)
/// - `Int.negSucc (Lit Nat n)` → `-(n as i64) - 1`
fn extract_int_lit(expr: &Expr) -> Option<i64> {
    if let Expr::App(func, arg) = expr {
        match func.as_ref() {
            Expr::Const(name, _) if name.to_string() == "Int.ofNat" => {
                if let Expr::Lit(Literal::Nat(n)) = arg.as_ref() {
                    let v = n.to_u64()?;
                    if v <= i64::MAX as u64 {
                        return Some(v as i64);
                    }
                }
            }
            Expr::Const(name, _) if name.to_string() == "Int.negSucc" => {
                if let Expr::Lit(Literal::Nat(n)) = arg.as_ref() {
                    let v = n.to_u64()?;
                    if v < i64::MAX as u64 {
                        return Some(-(v as i64) - 1);
                    }
                }
            }
            _ => {}
        }
    }
    None
}

/// Build `@Eq.refl.{1} Bool Bool.false`.
///
/// This term has type `@Eq.{1} Bool Bool.false Bool.false`. The kernel also
/// accepts it as a proof of `@Eq.{1} Bool (Int.ble a b) Bool.false` when
/// `Int.ble a b` reduces to `Bool.false` under `whnf` (definitional equality
/// in the kernel's type-checker).
///
/// Universe level 1 is correct because `Bool : Type 0 = Sort 1`.
fn mk_bool_false_rfl() -> Expr {
    let level_one = Level::succ(Level::zero());
    let eq_refl = Expr::Const(Name::str("Eq.refl"), vec![level_one]);
    let bool_const = Expr::Const(Name::str("Bool"), vec![]);
    let bool_false = Expr::Const(Name::str("Bool.false"), vec![]);
    mk_app2(eq_refl, bool_const, bool_false)
}

/// Build `Int.le lhs rhs`.
fn mk_int_le(lhs: Expr, rhs: Expr) -> Expr {
    mk_app2(Expr::Const(Name::str("Int.le"), vec![]), lhs, rhs)
}

/// Build `Int.lt lhs rhs`.
#[cfg(test)]
fn mk_int_lt(lhs: Expr, rhs: Expr) -> Expr {
    mk_app2(Expr::Const(Name::str("Int.lt"), vec![]), lhs, rhs)
}

// ── Tests ──────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::{init_builtin_env, BinderInfo, Level, Literal};
    use oxilean_meta::tactic::linear_combination::{
        ConOrient, ConSource, FarkasCert, FarkasCertEntry, Rat,
    };
    use oxilean_std::register_omega_helper;

    // ── Test helpers ───────────────────────────────────────────────────────────

    /// Build `Int.ofNat n` (for n >= 0) or `Int.negSucc ((-n)-1)` (for n < 0).
    fn mk_int_val(n: i64) -> Expr {
        if n >= 0 {
            Expr::App(
                Node::new(Expr::Const(Name::str("Int.ofNat"), vec![])),
                Node::new(Expr::Lit(Literal::nat(n as u64))),
            )
        } else {
            let k = (-n - 1) as u64;
            Expr::App(
                Node::new(Expr::Const(Name::str("Int.negSucc"), vec![])),
                Node::new(Expr::Lit(Literal::nat(k))),
            )
        }
    }

    fn make_entry(idx: usize, numer: i64, orient: ConOrient) -> FarkasCertEntry {
        FarkasCertEntry {
            constraint_index: idx,
            multiplier: Rat::new(numer, 1),
            orient,
        }
    }

    fn make_source(name: Option<&str>, hyp_type: Expr) -> ConSource {
        ConSource {
            hyp_name: name.map(Name::str),
            hyp_type,
            orient: ConOrient::LeZero,
        }
    }

    fn int_ty() -> Expr {
        Expr::Const(Name::str("Int"), vec![])
    }

    fn int_le_ty() -> Expr {
        // Int.le : Int → Int → Prop
        Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Node::new(int_ty()),
            Node::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("y"),
                Node::new(int_ty()),
                Node::new(Expr::Sort(Level::zero())),
            )),
        )
    }

    /// Build a minimal environment with Int, Int.le, and some constants.
    fn base_env() -> Environment {
        let mut env = Environment::new();
        env.add(Declaration::Axiom {
            name: Name::str("Int"),
            univ_params: vec![],
            ty: Expr::Sort(Level::succ(Level::zero())),
        })
        .expect("Int");
        env.add(Declaration::Axiom {
            name: Name::str("Int.le"),
            univ_params: vec![],
            ty: int_le_ty(),
        })
        .expect("Int.le");
        env.add(Declaration::Axiom {
            name: Name::str("a"),
            univ_params: vec![],
            ty: int_ty(),
        })
        .expect("a");
        env.add(Declaration::Axiom {
            name: Name::str("b"),
            univ_params: vec![],
            ty: int_ty(),
        })
        .expect("b");
        env
    }

    /// Build an environment with all omega helper lemmas registered.
    ///
    /// This includes `Int.lt`, `Int.lt_of_le_of_lt`, `Int.lt_of_lt_of_le`,
    /// `Int.lt_trans`, `Int.lt_irrefl'`, and the base Int/Int.le axioms.
    fn omega_env() -> Environment {
        let mut env = Environment::new();
        register_omega_helper(&mut env).expect("omega lemmas registration should succeed");
        // Also add test variables a, b, c (they may not be in the omega bundle).
        let int_ty_expr = Expr::Const(Name::str("Int"), vec![]);
        for var in ["a", "b", "c"] {
            if !env.contains(&Name::str(var)) {
                env.add(Declaration::Axiom {
                    name: Name::str(var),
                    univ_params: vec![],
                    ty: int_ty_expr.clone(),
                })
                .expect("variable axiom");
            }
        }
        env
    }

    /// Build an environment with omega helper lemmas AND the kernel builtins.
    ///
    /// Required for Bool-reflection tests that need `Bool`, `Bool.false`, `Eq`,
    /// `Eq.refl`, `Int.ble`, `Int.blt` in scope during kernel type-checking.
    fn full_omega_env() -> Environment {
        let mut env = Environment::new();
        // Builtins first (Bool, Eq, Nat, etc.) so that register_omega_helper can
        // skip any overlapping names via the contains() guard.
        init_builtin_env(&mut env).expect("builtin env init should succeed");
        register_omega_helper(&mut env).expect("omega lemmas registration should succeed");
        env
    }

    fn make_int_le_expr(lhs: Expr, rhs: Expr) -> Expr {
        mk_int_le(lhs, rhs)
    }

    fn make_int_lt_expr(lhs: Expr, rhs: Expr) -> Expr {
        mk_int_lt(lhs, rhs)
    }

    fn false_goal() -> Expr {
        Expr::Const(Name::str("False"), vec![])
    }

    // ── Original guard tests (unchanged) ──────────────────────────────────────

    #[test]
    fn test_farkas_cert_to_expr_invalid_cert_returns_none() {
        let entry = make_entry(0, 1, ConOrient::LeZero);
        let cert = FarkasCert::new(vec![entry], Rat::new(1, 1));
        let env = base_env();
        let result = farkas_cert_to_expr(&cert, &false_goal(), &[], &[], &env);
        assert!(
            result.is_none(),
            "invalid cert (positive rhs) should return None"
        );
    }

    #[test]
    fn test_farkas_cert_to_expr_fractional_multiplier_returns_none() {
        let entry = FarkasCertEntry {
            constraint_index: 0,
            multiplier: Rat::new(1, 2),
            orient: ConOrient::LeZero,
        };
        let cert = FarkasCert::new(vec![entry], Rat::new(-1, 1));
        let env = base_env();
        let result = farkas_cert_to_expr(&cert, &false_goal(), &[], &[], &env);
        assert!(result.is_none(), "fractional multiplier should return None");
    }

    #[test]
    fn test_farkas_cert_to_expr_empty_constraints_returns_none() {
        let cert = FarkasCert::new(vec![], Rat::new(-1, 1));
        let env = base_env();
        let result = farkas_cert_to_expr(&cert, &false_goal(), &[], &[], &env);
        assert!(result.is_none(), "empty constraint list should return None");
    }

    #[test]
    fn test_farkas_no_sources_returns_none() {
        let entry = make_entry(0, 1, ConOrient::LeZero);
        let cert = FarkasCert::new(vec![entry], Rat::new(-1, 1));
        let env = base_env();
        let result = farkas_cert_to_expr(&cert, &false_goal(), &[], &[], &env);
        assert!(
            result.is_none(),
            "cert with no sources should return None gracefully"
        );
    }

    #[test]
    fn test_farkas_augmented_source_returns_none() {
        let entry = make_entry(0, 1, ConOrient::LeZero);
        let a = Expr::Const(Name::str("a"), vec![]);
        let zero = Expr::Const(Name::str("Int.zero"), vec![]);
        let hyp_ty = make_int_le_expr(a.clone(), zero.clone());
        let aug_source = make_source(None, hyp_ty.clone());
        let cert = FarkasCert::new(vec![entry], Rat::new(-1, 1)).with_sources(vec![aug_source]);
        let env = base_env();
        let hyps = vec![(Name::str("h"), hyp_ty)];
        let result = farkas_cert_to_expr(&cert, &false_goal(), &hyps, &[], &env);
        assert!(
            result.is_none(),
            "augmented source (None hyp_name) should return None"
        );
    }

    #[test]
    fn test_farkas_cert_to_expr_out_of_bounds_index_returns_none() {
        let entry = make_entry(5, 1, ConOrient::LeZero);
        let a = Expr::Const(Name::str("a"), vec![]);
        let zero = Expr::Const(Name::str("Int.zero"), vec![]);
        let hyp_ty = make_int_le_expr(a.clone(), zero.clone());
        let cert = FarkasCert::new(vec![entry], Rat::new(-1, 1))
            .with_sources(vec![make_source(Some("h"), hyp_ty)]);
        let env = base_env();
        let result = farkas_cert_to_expr(&cert, &false_goal(), &[], &[], &env);
        assert!(
            result.is_none(),
            "out-of-bounds source index should return None"
        );
    }

    #[test]
    fn test_farkas_single_named_source_returns_none_without_axioms() {
        let entry = make_entry(0, 1, ConOrient::LeZero);
        let a = Expr::Const(Name::str("a"), vec![]);
        let zero = Expr::Const(Name::str("Int.zero"), vec![]);
        let hyp_ty = make_int_le_expr(a.clone(), zero.clone());
        let cert = FarkasCert::new(vec![entry], Rat::new(-1, 1))
            .with_sources(vec![make_source(Some("h"), hyp_ty.clone())]);
        let env = base_env();
        let hyps = vec![(Name::str("h"), hyp_ty)];
        let result = farkas_cert_to_expr(&cert, &false_goal(), &hyps, &[], &env);
        if let Some(ref term) = result {
            assert!(
                verify_proof_term(term, &false_goal(), &hyps, &[], &env),
                "any returned Some term MUST be kernel-verified"
            );
        }
    }

    #[test]
    fn test_farkas_two_entries_named_sources_no_panic() {
        let e0 = make_entry(0, 1, ConOrient::LeZero);
        let e1 = make_entry(1, 1, ConOrient::LeZero);
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let zero = Expr::Const(Name::str("Int.zero"), vec![]);
        let h0_ty = make_int_le_expr(a.clone(), zero.clone());
        let h1_ty = make_int_le_expr(b.clone(), zero.clone());
        let cert = FarkasCert::new(vec![e0, e1], Rat::new(-1, 1)).with_sources(vec![
            make_source(Some("h0"), h0_ty.clone()),
            make_source(Some("h1"), h1_ty.clone()),
        ]);
        let env = base_env();
        let hyps = vec![
            (Name::str("h0"), h0_ty.clone()),
            (Name::str("h1"), h1_ty.clone()),
        ];
        let result = farkas_cert_to_expr(&cert, &false_goal(), &hyps, &[], &env);
        if let Some(ref term) = result {
            assert!(
                verify_proof_term(term, &false_goal(), &hyps, &[], &env),
                "any returned Some term MUST be kernel-verified"
            );
        }
    }

    // ── Real E2E tests: transitivity-cycle builder ────────────────────────────

    /// Test 1: 2-cycle le+lt
    ///   h_ab: Int.le a b,  h_ba: Int.lt b a  ⊢  False
    ///
    /// Expected: Some(non-sorry term) that kernel-verifies.
    #[test]
    fn test_farkas_2cycle_le_lt_real_proof() {
        let env = omega_env();
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let goal = false_goal();

        // Hypotheses: h_ab: Int.le a b,  h_ba: Int.lt b a
        let h_ab_ty = make_int_le_expr(a.clone(), b.clone());
        let h_ba_ty = make_int_lt_expr(b.clone(), a.clone());

        // Use Const-based proof terms (hyp_proof_term falls back to Const when no FVar match).
        let hyps = vec![
            (Name::str("h_ab"), h_ab_ty.clone()),
            (Name::str("h_ba"), h_ba_ty.clone()),
        ];

        // Build FarkasCert with 2 unit-multiplier entries.
        let e0 = make_entry(0, 1, ConOrient::LeZero);
        let e1 = make_entry(1, 1, ConOrient::LeZero);
        let cert = FarkasCert::new(vec![e0, e1], Rat::new(-1, 1)).with_sources(vec![
            ConSource {
                hyp_name: Some(Name::str("h_ab")),
                hyp_type: h_ab_ty.clone(),
                orient: ConOrient::LeZero,
            },
            ConSource {
                hyp_name: Some(Name::str("h_ba")),
                hyp_type: h_ba_ty.clone(),
                orient: ConOrient::LeZero,
            },
        ]);

        let result = farkas_cert_to_expr(&cert, &goal, &hyps, &[], &env);

        assert!(
            result.is_some(),
            "2-cycle le+lt should produce a real proof term (Some), not None/sorry"
        );

        let term = result.unwrap();
        // Critical: the term must not be sorry.
        assert!(
            !is_sorry(&term),
            "proof term must not be sorry; got: {:?}",
            term
        );
        // Critical: the term must kernel-verify.
        assert!(
            verify_proof_term(&term, &goal, &hyps, &[], &env),
            "proof term must kernel-verify against False goal"
        );
    }

    /// Test 2: 2-cycle lt+lt
    ///   h_ab: Int.lt a b,  h_ba: Int.lt b a  ⊢  False
    #[test]
    fn test_farkas_lt_lt_real_proof() {
        let env = omega_env();
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let goal = false_goal();

        let h_ab_ty = make_int_lt_expr(a.clone(), b.clone());
        let h_ba_ty = make_int_lt_expr(b.clone(), a.clone());

        let hyps = vec![
            (Name::str("h_ab"), h_ab_ty.clone()),
            (Name::str("h_ba"), h_ba_ty.clone()),
        ];

        let e0 = make_entry(0, 1, ConOrient::LeZero);
        let e1 = make_entry(1, 1, ConOrient::LeZero);
        let cert = FarkasCert::new(vec![e0, e1], Rat::new(-1, 1)).with_sources(vec![
            ConSource {
                hyp_name: Some(Name::str("h_ab")),
                hyp_type: h_ab_ty.clone(),
                orient: ConOrient::LeZero,
            },
            ConSource {
                hyp_name: Some(Name::str("h_ba")),
                hyp_type: h_ba_ty.clone(),
                orient: ConOrient::LeZero,
            },
        ]);

        let result = farkas_cert_to_expr(&cert, &goal, &hyps, &[], &env);

        assert!(
            result.is_some(),
            "lt+lt 2-cycle should produce a real proof term"
        );
        let term = result.unwrap();
        assert!(!is_sorry(&term), "proof term must not be sorry");
        assert!(
            verify_proof_term(&term, &goal, &hyps, &[], &env),
            "lt+lt proof must kernel-verify"
        );
    }

    /// Test 3: 3-cycle le+le+lt
    ///   h_ab: Int.le a b,  h_bc: Int.le b c,  h_ca: Int.lt c a  ⊢  False
    #[test]
    fn test_farkas_3cycle_le_le_lt_real_proof() {
        let env = omega_env();
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let c = Expr::Const(Name::str("c"), vec![]);
        let goal = false_goal();

        let h_ab_ty = make_int_le_expr(a.clone(), b.clone());
        let h_bc_ty = make_int_le_expr(b.clone(), c.clone());
        let h_ca_ty = make_int_lt_expr(c.clone(), a.clone());

        let hyps = vec![
            (Name::str("h_ab"), h_ab_ty.clone()),
            (Name::str("h_bc"), h_bc_ty.clone()),
            (Name::str("h_ca"), h_ca_ty.clone()),
        ];

        let e0 = make_entry(0, 1, ConOrient::LeZero);
        let e1 = make_entry(1, 1, ConOrient::LeZero);
        let e2 = make_entry(2, 1, ConOrient::LeZero);
        let cert = FarkasCert::new(vec![e0, e1, e2], Rat::new(-1, 1)).with_sources(vec![
            ConSource {
                hyp_name: Some(Name::str("h_ab")),
                hyp_type: h_ab_ty.clone(),
                orient: ConOrient::LeZero,
            },
            ConSource {
                hyp_name: Some(Name::str("h_bc")),
                hyp_type: h_bc_ty.clone(),
                orient: ConOrient::LeZero,
            },
            ConSource {
                hyp_name: Some(Name::str("h_ca")),
                hyp_type: h_ca_ty.clone(),
                orient: ConOrient::LeZero,
            },
        ]);

        let result = farkas_cert_to_expr(&cert, &goal, &hyps, &[], &env);

        assert!(
            result.is_some(),
            "3-cycle le+le+lt should produce a real proof term"
        );
        let term = result.unwrap();
        assert!(!is_sorry(&term), "proof term must not be sorry");
        assert!(
            verify_proof_term(&term, &goal, &hyps, &[], &env),
            "3-cycle proof must kernel-verify"
        );
    }

    /// Test 4: Negative case — pure le cycle, no strict edge → None (correct, honest sorry).
    ///   h_ab: Int.le a b,  h_ba: Int.le b a  ⊢  False
    ///
    /// The cycle builder requires at least one strict edge. Without it, it returns None.
    /// The legacy two-constraint builder may succeed, so we just check the invariant.
    #[test]
    fn test_farkas_negative_no_strict_soundness_invariant() {
        let env = omega_env();
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let goal = false_goal();

        let h_ab_ty = make_int_le_expr(a.clone(), b.clone());
        let h_ba_ty = make_int_le_expr(b.clone(), a.clone());

        let hyps = vec![
            (Name::str("h_ab"), h_ab_ty.clone()),
            (Name::str("h_ba"), h_ba_ty.clone()),
        ];

        let e0 = make_entry(0, 1, ConOrient::LeZero);
        let e1 = make_entry(1, 1, ConOrient::LeZero);
        let cert = FarkasCert::new(vec![e0, e1], Rat::new(-1, 1)).with_sources(vec![
            ConSource {
                hyp_name: Some(Name::str("h_ab")),
                hyp_type: h_ab_ty.clone(),
                orient: ConOrient::LeZero,
            },
            ConSource {
                hyp_name: Some(Name::str("h_ba")),
                hyp_type: h_ba_ty.clone(),
                orient: ConOrient::LeZero,
            },
        ]);

        let result = farkas_cert_to_expr(&cert, &goal, &hyps, &[], &env);
        // The cycle builder requires a strict edge and will return None for this case.
        // The legacy builders may also return None (they rely on absurd_le_zero with a
        // symbolic lhs, which won't kernel-verify unless the lhs is a known literal).
        // In either case, the soundness invariant must hold: if Some, it kernel-verifies.
        if let Some(ref term) = result {
            assert!(!is_sorry(term), "any returned Some must not be sorry");
            assert!(
                verify_proof_term(term, &goal, &hyps, &[], &env),
                "any returned Some term MUST be kernel-verified"
            );
        }
        // None is acceptable here — honest fallback to sorry.
    }

    /// Test 5: Soundness invariant across all returned terms.
    ///
    /// For any cert that produces Some, the term must be kernel-verified.
    /// This test exercises the 2-cycle le+lt case with FVar-threaded locals.
    #[test]
    fn test_farkas_soundness_invariant_with_fvars() {
        let env = omega_env();
        let fvar_a = FVarId(200);
        let fvar_b = FVarId(201);
        let a_expr = Expr::FVar(fvar_a);
        let b_expr = Expr::FVar(fvar_b);
        let goal = false_goal();

        let h_ab_ty = make_int_le_expr(a_expr.clone(), b_expr.clone());
        let h_ba_ty = make_int_lt_expr(b_expr.clone(), a_expr.clone());

        let int_ty_expr = Expr::Const(Name::str("Int"), vec![]);
        let locals: Vec<(FVarId, Name, Expr)> = vec![
            (fvar_a, Name::str("a"), int_ty_expr.clone()),
            (fvar_b, Name::str("b"), int_ty_expr.clone()),
        ];

        let fvar_h_ab = FVarId(202);
        let fvar_h_ba = FVarId(203);
        let locals_with_hyps: Vec<(FVarId, Name, Expr)> = vec![
            (fvar_a, Name::str("a"), int_ty_expr.clone()),
            (fvar_b, Name::str("b"), int_ty_expr.clone()),
            (fvar_h_ab, Name::str("h_ab"), h_ab_ty.clone()),
            (fvar_h_ba, Name::str("h_ba"), h_ba_ty.clone()),
        ];

        let hyps: Vec<(Name, Expr)> = vec![
            (Name::str("h_ab"), h_ab_ty.clone()),
            (Name::str("h_ba"), h_ba_ty.clone()),
        ];

        let _ = locals;

        let e0 = make_entry(0, 1, ConOrient::LeZero);
        let e1 = make_entry(1, 1, ConOrient::LeZero);
        let cert = FarkasCert::new(vec![e0, e1], Rat::new(-1, 1)).with_sources(vec![
            ConSource {
                hyp_name: Some(Name::str("h_ab")),
                hyp_type: h_ab_ty.clone(),
                orient: ConOrient::LeZero,
            },
            ConSource {
                hyp_name: Some(Name::str("h_ba")),
                hyp_type: h_ba_ty.clone(),
                orient: ConOrient::LeZero,
            },
        ]);

        // With FVar-threaded locals for a/b (so that FVar exprs type-check).
        let result = farkas_cert_to_expr(&cert, &goal, &hyps, &locals_with_hyps, &env);

        // Whether Some or None, the invariant holds.
        if let Some(ref term) = result {
            assert!(!is_sorry(term), "FVar-threaded result must not be sorry");
            assert!(
                verify_proof_term(term, &goal, &hyps, &locals_with_hyps, &env),
                "FVar-threaded result must kernel-verify"
            );
        }
    }

    // ── Tests: arithmetic chain close (cycle 8 Bool-reflection) ──────────────

    /// Test 6: `h1 : 0 ≤ x, h2 : x ≤ -1 ⊢ False` — Le chain with literal endpoints.
    ///
    /// Previously (with `Literal::Int`), this closed via `Int.not_le_of_ble_false`.
    /// After removing `Literal::Int`, integers are represented as `Int.ofNat`/`Int.negSucc`
    /// over `Literal::Nat`; the kernel cannot reduce `Int.ble (Int.ofNat 0) (Int.negSucc 0)`
    /// to `Bool.false` since `Int.ble` is an axiom without computational rules.
    /// This test now documents that the strategy returns `None`.
    #[test]
    fn test_farkas_le_chain_arith_close() {
        let env = full_omega_env();
        let fvar_x = FVarId::new(500);
        let x_expr = Expr::FVar(fvar_x);
        let int_zero = mk_int_val(0);
        let int_neg1 = mk_int_val(-1);
        let goal = false_goal();

        // h1 : Int.le 0 x,  h2 : Int.le x (-1)
        let h1_ty = make_int_le_expr(int_zero.clone(), x_expr.clone());
        let h2_ty = make_int_le_expr(x_expr.clone(), int_neg1.clone());

        let int_ty_expr = Expr::Const(Name::str("Int"), vec![]);
        let locals: Vec<(FVarId, Name, Expr)> = vec![
            (fvar_x, Name::str("x"), int_ty_expr.clone()),
            (FVarId::new(501), Name::str("h1"), h1_ty.clone()),
            (FVarId::new(502), Name::str("h2"), h2_ty.clone()),
        ];

        let hyps: Vec<(Name, Expr)> = vec![
            (Name::str("h1"), h1_ty.clone()),
            (Name::str("h2"), h2_ty.clone()),
        ];

        let e0 = make_entry(0, 1, ConOrient::LeZero);
        let e1 = make_entry(1, 1, ConOrient::LeZero);
        let cert = FarkasCert::new(vec![e0, e1], Rat::new(-1, 1)).with_sources(vec![
            ConSource {
                hyp_name: Some(Name::str("h1")),
                hyp_type: h1_ty.clone(),
                orient: ConOrient::LeZero,
            },
            ConSource {
                hyp_name: Some(Name::str("h2")),
                hyp_type: h2_ty.clone(),
                orient: ConOrient::LeZero,
            },
        ]);

        let result = farkas_cert_to_expr(&cert, &goal, &hyps, &locals, &env);

        // After removing Literal::Int, the kernel cannot evaluate Int.ble on
        // Int.ofNat/Int.negSucc representations (they are axioms without computational rules).
        // The arithmetic chain close strategy (Strategy 2) builds a term that requires
        // definitional equality `Int.ble (Int.ofNat 0) (Int.negSucc 0) ≡def Bool.false`,
        // which the kernel cannot verify. All fallback strategies also fail.
        assert!(
            result.is_none(),
            "With Int.ofNat/Int.negSucc representation, arithmetic chain close cannot be kernel-verified"
        );
    }

    /// Test 7: `h1 : 2 ≤ x, h2 : x ≤ 1 ⊢ False` — all-Le chain, endpoints 2 > 1.
    ///
    /// Previously closed via `Int.not_le_of_ble_false`. After removing `Literal::Int`,
    /// this returns `None` (kernel cannot evaluate `Int.ble (Int.ofNat 2) (Int.ofNat 1)`).
    #[test]
    fn test_farkas_all_le_chain_no_strict_arith_close() {
        let env = full_omega_env();
        let fvar_x = FVarId::new(510);
        let x_expr = Expr::FVar(fvar_x);
        let int_two = mk_int_val(2);
        let int_one = mk_int_val(1);
        let goal = false_goal();

        // h1 : Int.le 2 x,  h2 : Int.le x 1
        let h1_ty = make_int_le_expr(int_two.clone(), x_expr.clone());
        let h2_ty = make_int_le_expr(x_expr.clone(), int_one.clone());

        let int_ty_expr = Expr::Const(Name::str("Int"), vec![]);
        let locals: Vec<(FVarId, Name, Expr)> = vec![
            (fvar_x, Name::str("x"), int_ty_expr.clone()),
            (FVarId::new(511), Name::str("h1"), h1_ty.clone()),
            (FVarId::new(512), Name::str("h2"), h2_ty.clone()),
        ];

        let hyps: Vec<(Name, Expr)> = vec![
            (Name::str("h1"), h1_ty.clone()),
            (Name::str("h2"), h2_ty.clone()),
        ];

        let e0 = make_entry(0, 1, ConOrient::LeZero);
        let e1 = make_entry(1, 1, ConOrient::LeZero);
        let cert = FarkasCert::new(vec![e0, e1], Rat::new(-1, 1)).with_sources(vec![
            ConSource {
                hyp_name: Some(Name::str("h1")),
                hyp_type: h1_ty.clone(),
                orient: ConOrient::LeZero,
            },
            ConSource {
                hyp_name: Some(Name::str("h2")),
                hyp_type: h2_ty.clone(),
                orient: ConOrient::LeZero,
            },
        ]);

        let result = farkas_cert_to_expr(&cert, &goal, &hyps, &locals, &env);

        // After removing Literal::Int: kernel cannot evaluate Int.ble on
        // Int.ofNat representations, so arithmetic chain close returns None.
        assert!(
            result.is_none(),
            "With Int.ofNat representation, arithmetic chain close cannot be kernel-verified"
        );
    }

    /// Test 8: `h1 : 1 < x, h2 : x ≤ 0 ⊢ False` — Lt+Le chain, endpoints 1 ≥ 0 strictly.
    ///
    /// Previously closed via `Int.not_lt_of_blt_false`. After removing `Literal::Int`,
    /// this returns `None` (kernel cannot evaluate `Int.blt (Int.ofNat 1) (Int.ofNat 0)`).
    #[test]
    fn test_farkas_lt_chain_arith_close() {
        let env = full_omega_env();
        let fvar_x = FVarId::new(520);
        let x_expr = Expr::FVar(fvar_x);
        let int_zero = mk_int_val(0);
        let int_one = mk_int_val(1);
        let goal = false_goal();

        // h1 : Int.lt 1 x,  h2 : Int.le x 0
        let h1_ty = make_int_lt_expr(int_one.clone(), x_expr.clone());
        let h2_ty = make_int_le_expr(x_expr.clone(), int_zero.clone());

        let int_ty_expr = Expr::Const(Name::str("Int"), vec![]);
        let locals: Vec<(FVarId, Name, Expr)> = vec![
            (fvar_x, Name::str("x"), int_ty_expr.clone()),
            (FVarId::new(521), Name::str("h1"), h1_ty.clone()),
            (FVarId::new(522), Name::str("h2"), h2_ty.clone()),
        ];

        let hyps: Vec<(Name, Expr)> = vec![
            (Name::str("h1"), h1_ty.clone()),
            (Name::str("h2"), h2_ty.clone()),
        ];

        let e0 = make_entry(0, 1, ConOrient::LeZero);
        let e1 = make_entry(1, 1, ConOrient::LeZero);
        let cert = FarkasCert::new(vec![e0, e1], Rat::new(-1, 1)).with_sources(vec![
            ConSource {
                hyp_name: Some(Name::str("h1")),
                hyp_type: h1_ty.clone(),
                orient: ConOrient::LeZero,
            },
            ConSource {
                hyp_name: Some(Name::str("h2")),
                hyp_type: h2_ty.clone(),
                orient: ConOrient::LeZero,
            },
        ]);

        let result = farkas_cert_to_expr(&cert, &goal, &hyps, &locals, &env);

        // After removing Literal::Int: kernel cannot evaluate Int.blt on
        // Int.ofNat representations, so arithmetic chain close returns None.
        assert!(
            result.is_none(),
            "With Int.ofNat representation, arithmetic chain close cannot be kernel-verified"
        );
    }

    // ── Utility ───────────────────────────────────────────────────────────────

    /// Check if an expression is the `sorry` constant.
    fn is_sorry(expr: &Expr) -> bool {
        matches!(expr, Expr::Const(name, _) if name.to_string() == "sorry")
    }
}
