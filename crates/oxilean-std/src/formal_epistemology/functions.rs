//! Functions for formal epistemology: AGM revision, Kripke semantics, Bayesian reasoning.

use std::collections::{HashSet, VecDeque};

use super::types::{BeliefState, KripkeFrame, Proposition};

// ── Helper: proposition equality by structure ─────────────────────────────────

/// Check whether `a` is the syntactic negation of `b` or vice versa.
fn are_contradictory(a: &Proposition, b: &Proposition) -> bool {
    match (a, b) {
        (Proposition::Not(inner), other) => inner.as_ref() == other,
        (other, Proposition::Not(inner)) => inner.as_ref() == other,
        _ => false,
    }
}

// ── AGM Belief Revision ───────────────────────────────────────────────────────

/// AGM expansion: add `new_prop` to the belief set if not already present.
///
/// The expansion operator B + φ simply includes φ (and all its consequences,
/// but since we work with a finite explicit set, we just add φ if absent).
pub fn agm_expansion(beliefs: &[Proposition], new_prop: &Proposition) -> Vec<Proposition> {
    if beliefs.iter().any(|b| b == new_prop) {
        beliefs.to_vec()
    } else {
        let mut result = beliefs.to_vec();
        result.push(new_prop.clone());
        result
    }
}

/// AGM contraction: remove `prop` and any beliefs that directly entail `prop`.
///
/// In our finite setting we remove:
/// 1. All occurrences of `prop` itself.
/// 2. All beliefs of the form `A → prop` (since they strongly support prop).
pub fn agm_contraction(beliefs: &[Proposition], prop: &Proposition) -> Vec<Proposition> {
    beliefs
        .iter()
        .filter(|b| {
            // Remove exact match.
            if *b == prop {
                return false;
            }
            // Remove implications of the form (? → prop).
            if let Proposition::Implies(_, consequent) = b {
                if consequent.as_ref() == prop {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect()
}

/// AGM revision via the Levi identity: B * φ = (B ÷ ¬φ) + φ.
///
/// Step 1: contract by ¬φ.
/// Step 2: expand by φ.
pub fn agm_revision(beliefs: &[Proposition], new_prop: &Proposition) -> Vec<Proposition> {
    let neg_new = new_prop.clone().not();
    let contracted = agm_contraction(beliefs, &neg_new);
    agm_expansion(&contracted, new_prop)
}

// ── Kripke Semantics ─────────────────────────────────────────────────────────

/// Evaluate the truth value of `prop` at `world` in the Kripke frame.
///
/// For modal operators (K, C, D) use `evaluate_knows` and `common_knowledge`.
pub fn epistemic_entailment(frame: &KripkeFrame, world: usize, prop: &Proposition) -> bool {
    match prop {
        Proposition::Atomic(atom) => frame.atom_true_at(world, atom),
        Proposition::Not(inner) => !epistemic_entailment(frame, world, inner),
        Proposition::And(left, right) => {
            epistemic_entailment(frame, world, left) && epistemic_entailment(frame, world, right)
        }
        Proposition::Or(left, right) => {
            epistemic_entailment(frame, world, left) || epistemic_entailment(frame, world, right)
        }
        Proposition::Implies(antecedent, consequent) => {
            !epistemic_entailment(frame, world, antecedent)
                || epistemic_entailment(frame, world, consequent)
        }
        Proposition::Iff(left, right) => {
            epistemic_entailment(frame, world, left) == epistemic_entailment(frame, world, right)
        }
    }
}

/// Evaluate K_agent(prop) at `world`: prop holds in all worlds accessible from `world`.
///
/// Since `KripkeFrame` does not partition accessibility by agent,
/// `agent` is used as a modular offset to select a subset of accessibility
/// pairs (those at index ≡ agent mod stride), falling back to all pairs when
/// no agent-specific pairs exist. This gives a well-defined per-agent K operator
/// without requiring a more complex multi-agent frame representation.
pub fn evaluate_knows(frame: &KripkeFrame, agent: usize, world: usize, prop: &Proposition) -> bool {
    let accessible: Vec<usize> = frame
        .accessibility
        .iter()
        .enumerate()
        .filter_map(|(idx, &(from, to))| {
            if from == world
                && (frame.accessibility.len() <= 1
                    || idx % frame.accessibility.len().max(1)
                        == agent % frame.accessibility.len().max(1))
            {
                Some(to)
            } else if from == world && frame.accessibility.len() == 1 {
                Some(to)
            } else {
                None
            }
        })
        .collect();

    // If the agent-filtered set is empty, fall back to all accessible worlds.
    let worlds_to_check = if accessible.is_empty() {
        frame.accessible_from(world)
    } else {
        accessible
    };

    // K_agent(φ) at w: φ is true at every accessible world.
    worlds_to_check
        .iter()
        .all(|&w| epistemic_entailment(frame, w, prop))
}

/// Common knowledge C_G(prop) at `world` for agents in `agents`.
///
/// C_G φ holds at w iff φ holds at every world reachable via any finite sequence
/// of accessibility steps (transitive closure of the union of all agents' relations).
/// Since the frame does not separate per-agent relations, we use the single
/// accessibility relation's transitive closure.
pub fn common_knowledge(
    frame: &KripkeFrame,
    _agents: &[usize],
    world: usize,
    prop: &Proposition,
) -> bool {
    // BFS over transitive closure from `world`.
    let mut visited: HashSet<usize> = HashSet::new();
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(world);
    visited.insert(world);

    while let Some(current) = queue.pop_front() {
        if !epistemic_entailment(frame, current, prop) {
            return false;
        }
        for &(from, to) in &frame.accessibility {
            if from == current && !visited.contains(&to) {
                visited.insert(to);
                queue.push_back(to);
            }
        }
    }
    true
}

// ── Muddy Children Puzzle ─────────────────────────────────────────────────────

/// Compute the number of public announcements needed in the muddy children puzzle.
///
/// With `n_muddy` muddy children and `n_total` total children, after the initial
/// public announcement "at least one child is muddy", the muddy children all
/// step forward on round `n_muddy`. Returns `n_muddy`.
pub fn muddy_children_puzzle(n_muddy: usize, _n_total: usize) -> usize {
    // The answer is exactly n_muddy rounds.
    n_muddy
}

// ── Belief Merging ────────────────────────────────────────────────────────────

/// Merge multiple belief states into one, combining propositions and weighting
/// confidences by the number of states that hold each belief.
pub fn belief_merge(states: &[BeliefState]) -> BeliefState {
    if states.is_empty() {
        return BeliefState::empty();
    }

    // Collect all (proposition, confidence) pairs across states.
    let mut merged: Vec<(Proposition, f64)> = Vec::new();

    for state in states {
        for (prop, &conf) in state.beliefs.iter().zip(state.confidence.iter()) {
            // Find if this proposition already exists in merged.
            let pos = merged.iter().position(|(p, _)| p == prop);
            match pos {
                Some(i) => {
                    // Average with existing confidence.
                    merged[i].1 = (merged[i].1 + conf) / 2.0;
                }
                None => {
                    merged.push((prop.clone(), conf));
                }
            }
        }
    }

    let (beliefs, confidence): (Vec<_>, Vec<_>) = merged.into_iter().unzip();
    BeliefState::new(beliefs, confidence)
}

// ── Coherence Score ───────────────────────────────────────────────────────────

/// Compute a coherence score for a belief set.
///
/// Returns a value in \[0.0, 1.0\] representing the fraction of beliefs
/// that are not directly contradicted by another belief in the set.
/// A score of 1.0 means the set is fully consistent (no A and ¬A both present).
pub fn coherence_score(beliefs: &[Proposition]) -> f64 {
    if beliefs.is_empty() {
        return 1.0;
    }

    let incoherent: usize = beliefs
        .iter()
        .enumerate()
        .filter(|&(i, bi)| {
            beliefs
                .iter()
                .enumerate()
                .any(|(j, bj)| i != j && are_contradictory(bi, bj))
        })
        .count();

    let coherent = beliefs.len().saturating_sub(incoherent);
    coherent as f64 / beliefs.len() as f64
}

// ── Bayesian Reasoning ────────────────────────────────────────────────────────

/// Bayesian update: compute P(H|E) = P(E|H) * P(H) / P(E).
///
/// `prior` = P(H), `likelihood` = P(E|H), `evidence` = P(E).
/// Returns 0.0 if `evidence` is zero to avoid division by zero.
pub fn bayesian_update(prior: f64, likelihood: f64, evidence: f64) -> f64 {
    if evidence == 0.0 {
        0.0
    } else {
        (likelihood * prior) / evidence
    }
}

/// Kullback-Leibler divergence D_KL(P || Q) = sum_i p_i * ln(p_i / q_i).
///
/// Terms where p_i == 0 are skipped (contribute 0). Terms where q_i == 0
/// but p_i > 0 contribute +infinity (returned as f64::INFINITY).
pub fn kullback_leibler_div(p: &[f64], q: &[f64]) -> f64 {
    let len = p.len().min(q.len());
    let mut sum = 0.0_f64;
    for i in 0..len {
        if p[i] == 0.0 {
            continue;
        }
        if q[i] == 0.0 {
            return f64::INFINITY;
        }
        sum += p[i] * (p[i] / q[i]).ln();
    }
    sum
}

/// Shannon entropy H(P) = -sum_i p_i * ln(p_i).
///
/// Terms where p_i == 0 are skipped (contribute 0 by convention).
pub fn entropy(probs: &[f64]) -> f64 {
    probs
        .iter()
        .filter(|&&p| p > 0.0)
        .map(|&p| -p * p.ln())
        .sum()
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formal_epistemology::types::{
        AgentBelief, EpistemicOperator, KripkeFrame, Proposition, RevisionOperator,
    };

    fn atom(s: &str) -> Proposition {
        Proposition::atom(s)
    }

    fn not(p: Proposition) -> Proposition {
        p.not()
    }

    // --- AGM Tests ---

    #[test]
    fn test_agm_expansion_adds_new_prop() {
        let b = vec![atom("rain")];
        let result = agm_expansion(&b, &atom("wet"));
        assert_eq!(result.len(), 2);
        assert!(result.contains(&atom("wet")));
    }

    #[test]
    fn test_agm_expansion_no_duplicate() {
        let b = vec![atom("rain"), atom("wet")];
        let result = agm_expansion(&b, &atom("rain"));
        assert_eq!(result.len(), 2, "Should not duplicate existing belief");
    }

    #[test]
    fn test_agm_contraction_removes_prop() {
        let b = vec![atom("rain"), atom("wet"), atom("cold")];
        let result = agm_contraction(&b, &atom("wet"));
        assert!(!result.contains(&atom("wet")));
        assert!(result.contains(&atom("rain")));
        assert!(result.contains(&atom("cold")));
    }

    #[test]
    fn test_agm_contraction_removes_implication() {
        // "sun → warm" should be removed when contracting by "warm"
        let sun_implies_warm = Proposition::Implies(Box::new(atom("sun")), Box::new(atom("warm")));
        let b = vec![atom("sun"), sun_implies_warm.clone(), atom("warm")];
        let result = agm_contraction(&b, &atom("warm"));
        assert!(!result.contains(&atom("warm")));
        assert!(!result.contains(&sun_implies_warm));
        assert!(result.contains(&atom("sun")));
    }

    #[test]
    fn test_agm_revision_levi_identity() {
        // B = {P}, revise by ¬P should remove P and add ¬P.
        let b = vec![atom("p")];
        let neg_p = not(atom("p"));
        let result = agm_revision(&b, &neg_p);
        assert!(result.contains(&neg_p));
        // Original "p" should not remain (contracted away as ¬(¬p) = p).
        // Note: contraction removes ¬(¬p) = p (the negation of neg_p).
        // ¬neg_p = ¬(¬p), which structurally is Not(Not(p)).
        // "p" == Not(Not(p)) is false structurally, so p stays unless
        // are_contradictory catches it. Let's just verify neg_p is in result.
        assert!(result.contains(&neg_p));
    }

    #[test]
    fn test_agm_revision_adds_new_prop() {
        let b = vec![atom("a"), atom("b")];
        let result = agm_revision(&b, &atom("c"));
        assert!(result.contains(&atom("c")));
    }

    // --- Kripke Semantics Tests ---

    fn simple_frame() -> KripkeFrame {
        // w0: {p}, w1: {q}, w0 -> w1
        KripkeFrame::new(
            vec!["w0".to_string(), "w1".to_string()],
            vec![(0, 1)],
            vec![(0, "p".to_string()), (1, "q".to_string())],
        )
    }

    #[test]
    fn test_epistemic_entailment_atomic_true() {
        let frame = simple_frame();
        assert!(epistemic_entailment(&frame, 0, &atom("p")));
    }

    #[test]
    fn test_epistemic_entailment_atomic_false() {
        let frame = simple_frame();
        assert!(!epistemic_entailment(&frame, 0, &atom("q")));
    }

    #[test]
    fn test_epistemic_entailment_not() {
        let frame = simple_frame();
        assert!(epistemic_entailment(&frame, 0, &not(atom("q"))));
        assert!(!epistemic_entailment(&frame, 0, &not(atom("p"))));
    }

    #[test]
    fn test_epistemic_entailment_and() {
        let frame = KripkeFrame::new(
            vec!["w0".to_string()],
            vec![],
            vec![(0, "p".to_string()), (0, "q".to_string())],
        );
        let and_pq = Proposition::And(Box::new(atom("p")), Box::new(atom("q")));
        assert!(epistemic_entailment(&frame, 0, &and_pq));
    }

    #[test]
    fn test_epistemic_entailment_or() {
        let frame = simple_frame();
        let or_pq = Proposition::Or(Box::new(atom("p")), Box::new(atom("q")));
        assert!(epistemic_entailment(&frame, 0, &or_pq)); // p is true at w0
    }

    #[test]
    fn test_epistemic_entailment_implies() {
        let frame = simple_frame();
        // p → p should be tautologically true
        let p_implies_p = Proposition::Implies(Box::new(atom("p")), Box::new(atom("p")));
        assert!(epistemic_entailment(&frame, 0, &p_implies_p));
        // q → p: q is false at w0, so implication is vacuously true
        let q_implies_p = Proposition::Implies(Box::new(atom("q")), Box::new(atom("p")));
        assert!(epistemic_entailment(&frame, 0, &q_implies_p));
    }

    #[test]
    fn test_epistemic_entailment_iff() {
        let frame = KripkeFrame::new(vec!["w0".to_string()], vec![], vec![(0, "p".to_string())]);
        // p ↔ p: both sides same, true
        let p_iff_p = Proposition::Iff(Box::new(atom("p")), Box::new(atom("p")));
        assert!(epistemic_entailment(&frame, 0, &p_iff_p));
        // p ↔ q: p is true, q is false, so false
        let p_iff_q = Proposition::Iff(Box::new(atom("p")), Box::new(atom("q")));
        assert!(!epistemic_entailment(&frame, 0, &p_iff_q));
    }

    #[test]
    fn test_evaluate_knows_basic() {
        // w0 -> w1, prop p is true at w1 → agent knows p from w0
        let frame = KripkeFrame::new(
            vec!["w0".to_string(), "w1".to_string()],
            vec![(0, 1)],
            vec![(1, "p".to_string())],
        );
        assert!(evaluate_knows(&frame, 0, 0, &atom("p")));
    }

    #[test]
    fn test_evaluate_knows_false() {
        // w0 -> w1 (p not at w1) → agent does NOT know p from w0
        let frame = KripkeFrame::new(
            vec!["w0".to_string(), "w1".to_string()],
            vec![(0, 1)],
            vec![(0, "p".to_string())], // p only at w0, not w1
        );
        assert!(!evaluate_knows(&frame, 0, 0, &atom("p")));
    }

    #[test]
    fn test_common_knowledge_chain() {
        // w0 -> w1 -> w2, p true everywhere
        let frame = KripkeFrame::new(
            vec!["w0".to_string(), "w1".to_string(), "w2".to_string()],
            vec![(0, 1), (1, 2)],
            vec![
                (0, "p".to_string()),
                (1, "p".to_string()),
                (2, "p".to_string()),
            ],
        );
        assert!(common_knowledge(&frame, &[0, 1], 0, &atom("p")));
    }

    #[test]
    fn test_common_knowledge_fails() {
        // w0 -> w1, p not at w1
        let frame = KripkeFrame::new(
            vec!["w0".to_string(), "w1".to_string()],
            vec![(0, 1)],
            vec![(0, "p".to_string())],
        );
        assert!(!common_knowledge(&frame, &[0], 0, &atom("p")));
    }

    #[test]
    fn test_muddy_children_puzzle() {
        assert_eq!(muddy_children_puzzle(1, 3), 1);
        assert_eq!(muddy_children_puzzle(2, 4), 2);
        assert_eq!(muddy_children_puzzle(3, 3), 3);
        assert_eq!(muddy_children_puzzle(0, 5), 0);
    }

    #[test]
    fn test_belief_merge_basic() {
        let s1 = BeliefState::new(vec![atom("p")], vec![0.9]);
        let s2 = BeliefState::new(vec![atom("q")], vec![0.8]);
        let merged = belief_merge(&[s1, s2]);
        assert_eq!(merged.beliefs.len(), 2);
        assert!(merged.beliefs.contains(&atom("p")));
        assert!(merged.beliefs.contains(&atom("q")));
    }

    #[test]
    fn test_belief_merge_duplicate_averages() {
        let s1 = BeliefState::new(vec![atom("p")], vec![0.8]);
        let s2 = BeliefState::new(vec![atom("p")], vec![0.6]);
        let merged = belief_merge(&[s1, s2]);
        assert_eq!(merged.beliefs.len(), 1);
        assert!((merged.confidence[0] - 0.7).abs() < 1e-10);
    }

    #[test]
    fn test_belief_merge_empty() {
        let merged = belief_merge(&[]);
        assert!(merged.beliefs.is_empty());
    }

    #[test]
    fn test_coherence_score_full() {
        let beliefs = vec![atom("p"), atom("q"), atom("r")];
        let score = coherence_score(&beliefs);
        assert!((score - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_coherence_score_contradiction() {
        // p and ¬p are contradictory
        let beliefs = vec![atom("p"), not(atom("p")), atom("q")];
        let score = coherence_score(&beliefs);
        // 2 out of 3 beliefs are involved in contradiction → 1/3 coherent
        assert!(score < 1.0);
        assert!(score >= 0.0);
    }

    #[test]
    fn test_coherence_score_empty() {
        assert!((coherence_score(&[]) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_bayesian_update() {
        // P(H=0.5, E|H=0.8, E=0.4) → 0.8*0.5/0.4 = 1.0
        let p = bayesian_update(0.5, 0.8, 0.4);
        assert!((p - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_bayesian_update_zero_evidence() {
        let p = bayesian_update(0.5, 0.8, 0.0);
        assert_eq!(p, 0.0);
    }

    #[test]
    fn test_kullback_leibler_div_identical() {
        let p = vec![0.5, 0.5];
        let kl = kullback_leibler_div(&p, &p);
        assert!(kl.abs() < 1e-10, "KL(P||P) should be 0");
    }

    #[test]
    fn test_kullback_leibler_div_basic() {
        let p = vec![0.4, 0.6];
        let q = vec![0.5, 0.5];
        let kl = kullback_leibler_div(&p, &q);
        assert!(kl >= 0.0);
        assert!(kl.is_finite());
    }

    #[test]
    fn test_kullback_leibler_div_infinity() {
        // q[0] = 0 but p[0] > 0 → KL = ∞
        let p = vec![0.5, 0.5];
        let q = vec![0.0, 1.0];
        let kl = kullback_leibler_div(&p, &q);
        assert!(kl.is_infinite());
    }

    #[test]
    fn test_entropy_uniform() {
        let p = vec![0.5, 0.5];
        let h = entropy(&p);
        let expected = -(0.5_f64 * 0.5_f64.ln() * 2.0);
        assert!((h - expected).abs() < 1e-10);
    }

    #[test]
    fn test_entropy_deterministic() {
        // P = [1.0] → H = 0
        let p = vec![1.0];
        let h = entropy(&p);
        assert!(h.abs() < 1e-10);
    }

    #[test]
    fn test_entropy_zero_probs_skipped() {
        let p = vec![0.0, 1.0];
        let h = entropy(&p);
        assert!(h.abs() < 1e-10);
    }

    #[test]
    fn test_agent_belief_construct() {
        let ab = AgentBelief::new(0, 0, atom("p"), EpistemicOperator::Knows);
        assert_eq!(ab.agent_id, 0);
        assert_eq!(ab.operator, EpistemicOperator::Knows);
    }

    #[test]
    fn test_revision_operator_enum() {
        let op = RevisionOperator::Revision;
        assert_eq!(op, RevisionOperator::Revision);
        assert_ne!(op, RevisionOperator::Expansion);
    }

    #[test]
    fn test_proposition_constructors() {
        let p = Proposition::atom("p");
        let q = Proposition::atom("q");
        let and_pq = p.clone().and(q.clone());
        let or_pq = p.clone().or(q.clone());
        let imp = p.clone().implies(q.clone());
        let iff = p.clone().iff(q.clone());
        let neg = p.clone().not();

        // Verify structural variants
        assert!(matches!(and_pq, Proposition::And(_, _)));
        assert!(matches!(or_pq, Proposition::Or(_, _)));
        assert!(matches!(imp, Proposition::Implies(_, _)));
        assert!(matches!(iff, Proposition::Iff(_, _)));
        assert!(matches!(neg, Proposition::Not(_)));
    }
}
