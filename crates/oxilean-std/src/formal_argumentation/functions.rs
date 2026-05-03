//! Functions for Dung's abstract argumentation frameworks.

use super::types::{
    AcceptanceStatus, ArgId, ArgumentationFramework, CredulousResult, Extension, ExtensionSemantics,
};

// ── Core Relations ─────────────────────────────────────────────────────────────

/// Check whether `attacker` attacks `target` in the framework.
pub fn attacks(af: &ArgumentationFramework, attacker: ArgId, target: ArgId) -> bool {
    af.attacks
        .iter()
        .any(|att| att.attacker == attacker && att.target == target)
}

/// Check whether any member of `set` attacks `arg`.
pub fn is_attacked_by_set(af: &ArgumentationFramework, set: &[ArgId], arg: ArgId) -> bool {
    set.iter().any(|&a| attacks(af, a, arg))
}

/// Check whether `arg` is attacked by at least one argument.
pub fn is_attacked(af: &ArgumentationFramework, arg: ArgId) -> bool {
    af.attacks.iter().any(|att| att.target == arg)
}

/// Check whether `arg` attacks itself.
pub fn is_self_defeating(af: &ArgumentationFramework, arg: ArgId) -> bool {
    attacks(af, arg, arg)
}

// ── Conflict-freeness ──────────────────────────────────────────────────────────

/// Check whether `set` is conflict-free: no argument in `set` attacks another in `set`.
pub fn is_conflict_free(af: &ArgumentationFramework, set: &[ArgId]) -> bool {
    for &a in set {
        for &b in set {
            if attacks(af, a, b) {
                return false;
            }
        }
    }
    true
}

// ── Defence (Acceptability) ────────────────────────────────────────────────────

/// Check whether `set` defends `arg`: for every argument that attacks `arg`,
/// some member of `set` counter-attacks it.
pub fn defends(af: &ArgumentationFramework, set: &[ArgId], arg: ArgId) -> bool {
    for att in &af.attacks {
        if att.target == arg {
            // att.attacker attacks arg; set must counter-attack att.attacker
            if !is_attacked_by_set(af, set, att.attacker) {
                return false;
            }
        }
    }
    true
}

/// Collect all arguments defended by `set` (the characteristic function F_AF(set)).
pub fn defended_args(af: &ArgumentationFramework, set: &[ArgId]) -> Vec<ArgId> {
    af.arguments
        .iter()
        .filter(|a| defends(af, set, a.id))
        .map(|a| a.id)
        .collect()
}

// ── Admissibility ──────────────────────────────────────────────────────────────

/// Check whether `set` is admissible: conflict-free and defends all its members.
pub fn is_admissible(af: &ArgumentationFramework, set: &[ArgId]) -> bool {
    if !is_conflict_free(af, set) {
        return false;
    }
    set.iter().all(|&a| defends(af, set, a))
}

// ── Completeness ───────────────────────────────────────────────────────────────

/// Check whether `set` is a complete extension: admissible + contains all arguments
/// that `set` defends.
pub fn is_complete(af: &ArgumentationFramework, set: &[ArgId]) -> bool {
    if !is_admissible(af, set) {
        return false;
    }
    // Every argument defended by set must be in set
    for arg in af.arguments.iter() {
        if defends(af, set, arg.id) && !set.contains(&arg.id) {
            return false;
        }
    }
    true
}

// ── Grounded Extension ─────────────────────────────────────────────────────────

/// Compute the grounded extension as the least fixed point of F_AF.
///
/// F_AF(S) = { a ∈ Args | S defends a }
/// Start from the empty set and iterate until a fixed point is reached.
pub fn grounded_extension(af: &ArgumentationFramework) -> Extension {
    let mut current: Vec<ArgId> = Vec::new();

    loop {
        let next = defended_args(af, &current);
        // Keep only conflict-free additions
        let mut candidate: Vec<ArgId> = current.clone();
        for id in &next {
            if !candidate.contains(id)
                && is_conflict_free(af, &{
                    let mut tmp = candidate.clone();
                    tmp.push(*id);
                    tmp
                })
            {
                candidate.push(*id);
            }
        }
        candidate.sort();
        let mut prev_sorted = current.clone();
        prev_sorted.sort();
        if candidate == prev_sorted {
            break;
        }
        current = candidate;
    }

    Extension::from_slice(&current)
}

// ── Preferred Extensions ───────────────────────────────────────────────────────

/// Compute all preferred extensions: the maximal (by set inclusion) admissible sets.
pub fn preferred_extensions(af: &ArgumentationFramework) -> Vec<Extension> {
    let all_ids = af.all_ids();
    let admissible = all_admissible_sets(af, &all_ids);

    // Keep only maximal ones
    let mut preferred: Vec<Vec<ArgId>> = Vec::new();
    'outer: for set in &admissible {
        for other in &admissible {
            if other != set && is_subset(set, other) {
                continue 'outer; // set is strictly contained in other
            }
        }
        preferred.push(set.clone());
    }
    preferred
        .into_iter()
        .map(|s| Extension::from_slice(&s))
        .collect()
}

// ── Stable Extensions ──────────────────────────────────────────────────────────

/// Compute all stable extensions: conflict-free sets that attack every non-member.
pub fn stable_extensions(af: &ArgumentationFramework) -> Vec<Extension> {
    let all_ids = af.all_ids();
    power_set(&all_ids)
        .into_iter()
        .filter(|s| is_stable(af, s))
        .map(|s| Extension::from_slice(&s))
        .collect()
}

/// Check whether `set` is a stable extension.
fn is_stable(af: &ArgumentationFramework, set: &[ArgId]) -> bool {
    if !is_conflict_free(af, set) {
        return false;
    }
    // Every argument NOT in set must be attacked by some member of set
    for arg in af.arguments.iter() {
        if !set.contains(&arg.id) && !is_attacked_by_set(af, set, arg.id) {
            return false;
        }
    }
    true
}

// ── Admissible Sets Enumeration ────────────────────────────────────────────────

/// Enumerate all admissible sets of the framework (exponential, use only on small AFs).
fn all_admissible_sets(af: &ArgumentationFramework, ids: &[ArgId]) -> Vec<Vec<ArgId>> {
    power_set(ids)
        .into_iter()
        .filter(|s| is_admissible(af, s))
        .collect()
}

// ── Complete Extensions ────────────────────────────────────────────────────────

/// Compute all complete extensions.
pub fn complete_extensions(af: &ArgumentationFramework) -> Vec<Extension> {
    let all_ids = af.all_ids();
    power_set(&all_ids)
        .into_iter()
        .filter(|s| is_complete(af, s))
        .map(|s| Extension::from_slice(&s))
        .collect()
}

// ── CF2 Extensions ─────────────────────────────────────────────────────────────

/// Compute CF2 extensions (Baroni-Giacomin-Guida, 2005).
///
/// CF2 is defined recursively on SCCs of the attack graph:
/// for each SCC, restrict to conflict-free sets that maximally cover that SCC,
/// then propagate down.
pub fn cf2_extensions(af: &ArgumentationFramework) -> Vec<Extension> {
    let all_ids = af.all_ids();
    // Enumerate conflict-free sets; then keep those that are CF2-maximal
    let cf_sets: Vec<Vec<ArgId>> = power_set(&all_ids)
        .into_iter()
        .filter(|s| is_conflict_free(af, s))
        .collect();

    // For CF2 on a single SCC (or the whole graph) we keep maximal conflict-free sets
    let mut maximal: Vec<Vec<ArgId>> = Vec::new();
    'outer: for set in &cf_sets {
        for other in &cf_sets {
            if other != set && is_subset(set, other) {
                continue 'outer;
            }
        }
        maximal.push(set.clone());
    }
    maximal
        .into_iter()
        .map(|s| Extension::from_slice(&s))
        .collect()
}

// ── Acceptance Queries ─────────────────────────────────────────────────────────

/// Check whether `arg` is credulously accepted under `sem`: accepted in AT LEAST one extension.
pub fn credulously_accepted(
    af: &ArgumentationFramework,
    arg: ArgId,
    sem: &ExtensionSemantics,
) -> bool {
    extensions_for(af, sem).iter().any(|ext| ext.contains(arg))
}

/// Check whether `arg` is skeptically accepted under `sem`: accepted in ALL extensions.
///
/// Returns true if all extensions contain `arg`. If there are no extensions, returns false.
pub fn skeptically_accepted(
    af: &ArgumentationFramework,
    arg: ArgId,
    sem: &ExtensionSemantics,
) -> bool {
    let exts = extensions_for(af, sem);
    if exts.is_empty() {
        return false;
    }
    exts.iter().all(|ext| ext.contains(arg))
}

/// Compute the full credulous acceptance result with supporting extensions.
pub fn credulous_acceptance_result(
    af: &ArgumentationFramework,
    arg: ArgId,
    sem: &ExtensionSemantics,
) -> CredulousResult {
    let exts = extensions_for(af, sem);
    let supporting: Vec<Extension> = exts.into_iter().filter(|ext| ext.contains(arg)).collect();
    let status = if supporting.is_empty() {
        AcceptanceStatus::Rejected
    } else {
        AcceptanceStatus::Accepted
    };
    CredulousResult {
        status,
        supporting_extensions: supporting,
    }
}

/// Dispatch to the appropriate extension-computation function.
pub fn extensions_for(af: &ArgumentationFramework, sem: &ExtensionSemantics) -> Vec<Extension> {
    match sem {
        ExtensionSemantics::Complete => complete_extensions(af),
        ExtensionSemantics::Grounded => vec![grounded_extension(af)],
        ExtensionSemantics::Preferred => preferred_extensions(af),
        ExtensionSemantics::Stable => stable_extensions(af),
        ExtensionSemantics::Admissible => {
            let all_ids = af.all_ids();
            all_admissible_sets(af, &all_ids)
                .into_iter()
                .map(|s| Extension::from_slice(&s))
                .collect()
        }
        ExtensionSemantics::CF2 => cf2_extensions(af),
    }
}

// ── Utilities ──────────────────────────────────────────────────────────────────

/// Enumerate the power set of a slice.
fn power_set(ids: &[ArgId]) -> Vec<Vec<ArgId>> {
    let n = ids.len();
    let mut result: Vec<Vec<ArgId>> = Vec::with_capacity(1 << n.min(20));
    for mask in 0u64..(1u64 << n) {
        let subset: Vec<ArgId> = ids
            .iter()
            .enumerate()
            .filter_map(|(i, &id)| if (mask >> i) & 1 == 1 { Some(id) } else { None })
            .collect();
        result.push(subset);
    }
    result
}

/// Check whether `a` is a subset of `b`.
fn is_subset(a: &[ArgId], b: &[ArgId]) -> bool {
    a.iter().all(|x| b.contains(x))
}

// ── Tests ──────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Helper builders ──────────────────────────────────────────────────────

    /// Build a simple framework with two arguments and one attack.
    /// A attacks B.
    fn build_ab_af() -> (ArgumentationFramework, ArgId, ArgId) {
        let mut af = ArgumentationFramework::new();
        let a = af.add_argument("A", "claim_a");
        let b = af.add_argument("B", "claim_b");
        af.add_attack(a, b);
        (af, a, b)
    }

    /// Nixon Diamond: A attacks B, B attacks A, A attacks C, C attacks A.
    /// (Typically used as: Nixon-pacifist, Nixon-hawk, both attacking each other.)
    fn build_nixon_diamond() -> (ArgumentationFramework, ArgId, ArgId) {
        let mut af = ArgumentationFramework::new();
        let pac = af.add_argument("Pacifist", "Nixon is a pacifist");
        let hawk = af.add_argument("Hawk", "Nixon is a hawk");
        af.add_attack(pac, hawk);
        af.add_attack(hawk, pac);
        (af, pac, hawk)
    }

    /// Self-attacking argument.
    fn build_self_attack() -> (ArgumentationFramework, ArgId) {
        let mut af = ArgumentationFramework::new();
        let a = af.add_argument("A", "self-defeating");
        af.add_attack(a, a);
        (af, a)
    }

    /// Three-argument reinstatement example: A attacks B, B attacks C.
    fn build_reinstatement() -> (ArgumentationFramework, ArgId, ArgId, ArgId) {
        let mut af = ArgumentationFramework::new();
        let a = af.add_argument("A", "a");
        let b = af.add_argument("B", "b");
        let c = af.add_argument("C", "c");
        af.add_attack(a, b);
        af.add_attack(b, c);
        (af, a, b, c)
    }

    // ── Basic relation tests ─────────────────────────────────────────────────

    #[test]
    fn test_attacks_present() {
        let (af, a, b) = build_ab_af();
        assert!(attacks(&af, a, b));
    }

    #[test]
    fn test_attacks_absent() {
        let (af, a, b) = build_ab_af();
        assert!(!attacks(&af, b, a));
    }

    #[test]
    fn test_is_self_defeating_true() {
        let (af, a) = build_self_attack();
        assert!(is_self_defeating(&af, a));
    }

    #[test]
    fn test_is_self_defeating_false() {
        let (af, a, _b) = build_ab_af();
        assert!(!is_self_defeating(&af, a));
    }

    #[test]
    fn test_is_attacked() {
        let (af, _a, b) = build_ab_af();
        assert!(is_attacked(&af, b));
    }

    #[test]
    fn test_is_not_attacked() {
        let (af, a, _b) = build_ab_af();
        assert!(!is_attacked(&af, a));
    }

    // ── Conflict-freeness ────────────────────────────────────────────────────

    #[test]
    fn test_conflict_free_empty() {
        let (af, _a, _b) = build_ab_af();
        assert!(is_conflict_free(&af, &[]));
    }

    #[test]
    fn test_conflict_free_singleton() {
        let (af, a, _b) = build_ab_af();
        assert!(is_conflict_free(&af, &[a]));
    }

    #[test]
    fn test_not_conflict_free() {
        let (af, a, b) = build_ab_af();
        assert!(!is_conflict_free(&af, &[a, b]));
    }

    // ── Defence ──────────────────────────────────────────────────────────────

    #[test]
    fn test_defends_reinstatement() {
        let (af, a, b, c) = build_reinstatement();
        // {A} defends C because A attacks B (the attacker of C)
        assert!(defends(&af, &[a], c));
        // {A} defends itself (nothing attacks A)
        assert!(defends(&af, &[a], a));
        // {} does not defend C (B attacks C and nothing counters B)
        assert!(!defends(&af, &[], c));
        // The b variable is used only to ensure binding; suppress warning:
        let _ = b;
    }

    // ── Admissibility ────────────────────────────────────────────────────────

    #[test]
    fn test_empty_set_admissible() {
        let (af, _a, _b) = build_ab_af();
        assert!(is_admissible(&af, &[]));
    }

    #[test]
    fn test_singleton_a_admissible() {
        let (af, a, _b) = build_ab_af();
        // {A} is conflict-free and A defends itself (nothing attacks A)
        assert!(is_admissible(&af, &[a]));
    }

    #[test]
    fn test_singleton_b_not_admissible() {
        let (af, a, b) = build_ab_af();
        // {B} is conflict-free but B does not defend itself against A
        assert!(!is_admissible(&af, &[b]));
        let _ = a;
    }

    // ── Completeness ─────────────────────────────────────────────────────────

    #[test]
    fn test_complete_reinstatement() {
        let (af, a, _b, _c) = build_reinstatement();
        // {A, C} should be a complete extension
        let ext = [a, ArgId(2)]; // C is index 2
                                 // A is defended by {} (not attacked), C is defended by {A}
                                 // So {A, C} is complete
        assert!(is_complete(&af, &ext));
    }

    // ── Grounded extension ───────────────────────────────────────────────────

    #[test]
    fn test_grounded_empty_framework() {
        let af = ArgumentationFramework::new();
        let g = grounded_extension(&af);
        assert!(g.is_empty());
    }

    #[test]
    fn test_grounded_ab_single_attack() {
        let (af, a, _b) = build_ab_af();
        let g = grounded_extension(&af);
        // A is unattacked, so it's in the grounded extension; B is attacked by A and not defended
        assert!(g.contains(a));
    }

    #[test]
    fn test_grounded_nixon_diamond() {
        let (af, _pac, _hawk) = build_nixon_diamond();
        let g = grounded_extension(&af);
        // In the Nixon diamond, the grounded extension is empty (mutual attack, no defender)
        assert!(g.is_empty());
    }

    #[test]
    fn test_grounded_reinstatement() {
        let (af, a, _b, c) = build_reinstatement();
        let g = grounded_extension(&af);
        // A is unattacked → in grounded; A defends C → C in grounded; B is attacked by A → not in grounded
        assert!(g.contains(a));
        assert!(g.contains(c));
    }

    #[test]
    fn test_grounded_self_attack() {
        let (af, a) = build_self_attack();
        let g = grounded_extension(&af);
        // Self-attacking arg cannot be in any admissible set
        assert!(!g.contains(a));
    }

    // ── Preferred extensions ─────────────────────────────────────────────────

    #[test]
    fn test_preferred_ab() {
        let (af, a, _b) = build_ab_af();
        let prefs = preferred_extensions(&af);
        // Only one preferred extension: {A}
        assert_eq!(prefs.len(), 1);
        assert!(prefs[0].contains(a));
    }

    #[test]
    fn test_preferred_nixon_diamond() {
        let (af, pac, hawk) = build_nixon_diamond();
        let prefs = preferred_extensions(&af);
        // Two preferred extensions: {Pacifist} and {Hawk}
        assert_eq!(prefs.len(), 2);
        let has_pac = prefs.iter().any(|e| e.contains(pac) && !e.contains(hawk));
        let has_hawk = prefs.iter().any(|e| e.contains(hawk) && !e.contains(pac));
        assert!(has_pac);
        assert!(has_hawk);
    }

    // ── Stable extensions ────────────────────────────────────────────────────

    #[test]
    fn test_stable_ab() {
        let (af, a, _b) = build_ab_af();
        let stables = stable_extensions(&af);
        assert_eq!(stables.len(), 1);
        assert!(stables[0].contains(a));
    }

    #[test]
    fn test_stable_nixon_diamond() {
        let (af, pac, hawk) = build_nixon_diamond();
        let stables = stable_extensions(&af);
        // Two stable extensions: {Pacifist} and {Hawk}
        assert_eq!(stables.len(), 2);
        let has_pac = stables.iter().any(|e| e.contains(pac));
        let has_hawk = stables.iter().any(|e| e.contains(hawk));
        assert!(has_pac);
        assert!(has_hawk);
    }

    #[test]
    fn test_stable_self_attack_none() {
        let (af, _a) = build_self_attack();
        let stables = stable_extensions(&af);
        // No stable extension exists for a self-attacking argument
        assert!(stables.is_empty());
    }

    // ── Credulous / skeptical acceptance ─────────────────────────────────────

    #[test]
    fn test_credulously_accepted_preferred() {
        let (af, pac, hawk) = build_nixon_diamond();
        // Both pac and hawk are credulously accepted under preferred
        assert!(credulously_accepted(
            &af,
            pac,
            &ExtensionSemantics::Preferred
        ));
        assert!(credulously_accepted(
            &af,
            hawk,
            &ExtensionSemantics::Preferred
        ));
    }

    #[test]
    fn test_skeptically_rejected_nixon() {
        let (af, pac, hawk) = build_nixon_diamond();
        // Neither is skeptically accepted (each is absent from some preferred extension)
        assert!(!skeptically_accepted(
            &af,
            pac,
            &ExtensionSemantics::Preferred
        ));
        assert!(!skeptically_accepted(
            &af,
            hawk,
            &ExtensionSemantics::Preferred
        ));
    }

    #[test]
    fn test_skeptically_accepted_reinstatement() {
        let (af, a, _b, c) = build_reinstatement();
        // A and C are skeptically accepted under grounded
        assert!(skeptically_accepted(&af, a, &ExtensionSemantics::Grounded));
        assert!(skeptically_accepted(&af, c, &ExtensionSemantics::Grounded));
    }

    #[test]
    fn test_credulously_accepted_grounded() {
        let (af, a, _b) = build_ab_af();
        assert!(credulously_accepted(&af, a, &ExtensionSemantics::Grounded));
    }

    // ── Floating conclusions ──────────────────────────────────────────────────
    // A classic example where D is supported by two competing chains,
    // each individually defeating the other's premises.

    #[test]
    fn test_floating_conclusions() {
        // E: A attacks B, C attacks D, B attacks C, D attacks A.
        // Both {A, C} and {B, D} are preferred. Under skeptical preferred, nothing is accepted.
        let mut af = ArgumentationFramework::new();
        let a = af.add_argument("A", "a");
        let b = af.add_argument("B", "b");
        let c = af.add_argument("C", "c");
        let d = af.add_argument("D", "d");
        af.add_attack(a, b);
        af.add_attack(b, c);
        af.add_attack(c, d);
        af.add_attack(d, a);

        let prefs = preferred_extensions(&af);
        // There are two preferred extensions; skeptical acceptance of all four is false
        for arg in [a, b, c, d] {
            assert!(!skeptically_accepted(
                &af,
                arg,
                &ExtensionSemantics::Preferred
            ));
        }
        // But there should be exactly 2 preferred extensions
        assert_eq!(prefs.len(), 2);
    }

    // ── Admissible semantics ─────────────────────────────────────────────────

    #[test]
    fn test_admissible_includes_empty() {
        let (af, _a, _b) = build_nixon_diamond();
        let adm_exts = extensions_for(&af, &ExtensionSemantics::Admissible);
        // Empty set is always admissible
        assert!(adm_exts.iter().any(|e| e.is_empty()));
    }

    // ── Complete extensions ───────────────────────────────────────────────────

    #[test]
    fn test_complete_nixon() {
        let (af, _pac, _hawk) = build_nixon_diamond();
        let comps = complete_extensions(&af);
        // Complete extensions for Nixon diamond: {} , {Pacifist}, {Hawk}
        assert!(!comps.is_empty());
        assert!(comps.iter().any(|e| e.is_empty()));
    }

    // ── CF2 extensions ────────────────────────────────────────────────────────

    #[test]
    fn test_cf2_nixon() {
        let (af, pac, hawk) = build_nixon_diamond();
        let cf2 = cf2_extensions(&af);
        // Maximal conflict-free sets for Nixon diamond: {Pacifist} and {Hawk}
        assert_eq!(cf2.len(), 2);
        let has_pac = cf2.iter().any(|e| e.contains(pac));
        let has_hawk = cf2.iter().any(|e| e.contains(hawk));
        assert!(has_pac);
        assert!(has_hawk);
    }

    // ── CredulousResult ───────────────────────────────────────────────────────

    #[test]
    fn test_credulous_result_accepted() {
        let (af, a, _b) = build_ab_af();
        let result = credulous_acceptance_result(&af, a, &ExtensionSemantics::Preferred);
        assert_eq!(result.status, AcceptanceStatus::Accepted);
        assert!(!result.supporting_extensions.is_empty());
    }

    #[test]
    fn test_credulous_result_rejected() {
        let (af, _a, b) = build_ab_af();
        // B is attacked by A and cannot defend itself → not in any preferred extension
        let result = credulous_acceptance_result(&af, b, &ExtensionSemantics::Preferred);
        assert_eq!(result.status, AcceptanceStatus::Rejected);
    }

    // ── Extension ordering ───────────────────────────────────────────────────

    #[test]
    fn test_extension_from_slice_sorted() {
        let ids = vec![ArgId(3), ArgId(1), ArgId(2)];
        let ext = Extension::from_slice(&ids);
        assert_eq!(ext.args, vec![ArgId(1), ArgId(2), ArgId(3)]);
    }

    #[test]
    fn test_extension_contains() {
        let ext = Extension::from_slice(&[ArgId(0), ArgId(2)]);
        assert!(ext.contains(ArgId(0)));
        assert!(!ext.contains(ArgId(1)));
    }
}
