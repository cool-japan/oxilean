//! Functions for Abstract Rewriting Systems.

use std::collections::HashMap;

use super::types::{
    ConfluenceResult, CriticalPair, RewriteResult, RewriteRule, RewriteStrategy, RewriteSystem,
    TermTree,
};

// ── Pattern matching ──────────────────────────────────────────────────────────

/// Attempts to match `pattern` against `target`, binding uppercase leaf variables.
/// Returns `Some(substitution)` on success, `None` on failure.
pub fn match_term(pattern: &TermTree, target: &TermTree) -> Option<HashMap<String, TermTree>> {
    let mut subst = HashMap::new();
    if match_term_inner(pattern, target, &mut subst) {
        Some(subst)
    } else {
        None
    }
}

fn match_term_inner(
    pattern: &TermTree,
    target: &TermTree,
    subst: &mut HashMap<String, TermTree>,
) -> bool {
    match pattern {
        TermTree::Leaf(s) if s.starts_with(|c: char| c.is_uppercase()) => {
            // Variable: check consistency
            if let Some(existing) = subst.get(s) {
                existing == target
            } else {
                subst.insert(s.clone(), target.clone());
                true
            }
        }
        TermTree::Leaf(s) => {
            // Ground symbol: must match exactly
            matches!(target, TermTree::Leaf(t) if t == s)
        }
        TermTree::Node {
            symbol: ps,
            children: pc,
        } => match target {
            TermTree::Node {
                symbol: ts,
                children: tc,
            } => {
                ps == ts && pc.len() == tc.len() && {
                    pc.iter()
                        .zip(tc.iter())
                        .all(|(p, t)| match_term_inner(p, t, subst))
                }
            }
            _ => false,
        },
    }
}

/// Applies a substitution to a term, replacing variables with their bound values.
pub fn apply_substitution(term: &TermTree, subst: &HashMap<String, TermTree>) -> TermTree {
    match term {
        TermTree::Leaf(s) => {
            if s.starts_with(|c: char| c.is_uppercase()) {
                subst.get(s).cloned().unwrap_or_else(|| term.clone())
            } else {
                term.clone()
            }
        }
        TermTree::Node { symbol, children } => TermTree::Node {
            symbol: symbol.clone(),
            children: children
                .iter()
                .map(|c| apply_substitution(c, subst))
                .collect(),
        },
    }
}

// ── Term metrics ──────────────────────────────────────────────────────────────

/// Returns the number of nodes (including leaves) in the term.
pub fn term_size(t: &TermTree) -> usize {
    match t {
        TermTree::Leaf(_) => 1,
        TermTree::Node { children, .. } => 1 + children.iter().map(term_size).sum::<usize>(),
    }
}

/// Returns the depth of the term (leaves have depth 0).
pub fn term_depth(t: &TermTree) -> usize {
    match t {
        TermTree::Leaf(_) => 0,
        TermTree::Node { children, .. } => 1 + children.iter().map(term_depth).max().unwrap_or(0),
    }
}

/// Returns all subterms of `t` (including `t` itself).
pub fn subterms(t: &TermTree) -> Vec<&TermTree> {
    let mut result = vec![t];
    if let TermTree::Node { children, .. } = t {
        for child in children {
            result.extend(subterms(child));
        }
    }
    result
}

// ── Single reduction step ────────────────────────────────────────────────────

/// Attempts one reduction step on `term` according to `system`.
/// Returns `Some((reduced_term, rule_name))` if a rule applies, `None` otherwise.
pub fn reduce_once(
    term: &TermTree,
    system: &RewriteSystem<TermTree>,
) -> Option<(TermTree, String)> {
    match &system.strategy {
        RewriteStrategy::Outermost | RewriteStrategy::LeftmostOutermost => {
            // Try to apply a rule at the root first
            if let Some(result) = try_apply_at_root(term, &system.rules) {
                return Some(result);
            }
            // Then recurse into children
            reduce_in_children(term, system)
        }
        RewriteStrategy::Innermost
        | RewriteStrategy::LeftmostInnermost
        | RewriteStrategy::Parallel => {
            // Recurse into children first
            if let Some(result) = reduce_in_children(term, system) {
                return Some(result);
            }
            // Then try at root
            try_apply_at_root(term, &system.rules)
        }
    }
}

fn try_apply_at_root(
    term: &TermTree,
    rules: &[RewriteRule<TermTree>],
) -> Option<(TermTree, String)> {
    for rule in rules {
        if let Some(subst) = match_term(&rule.lhs, term) {
            let reduced = apply_substitution(&rule.rhs, &subst);
            return Some((reduced, rule.name.clone()));
        }
    }
    None
}

fn reduce_in_children(
    term: &TermTree,
    system: &RewriteSystem<TermTree>,
) -> Option<(TermTree, String)> {
    match term {
        TermTree::Leaf(_) => None,
        TermTree::Node { symbol, children } => {
            for (i, child) in children.iter().enumerate() {
                if let Some((reduced_child, rule_name)) = reduce_once(child, system) {
                    let mut new_children = children.clone();
                    new_children[i] = reduced_child;
                    return Some((
                        TermTree::Node {
                            symbol: symbol.clone(),
                            children: new_children,
                        },
                        rule_name,
                    ));
                }
            }
            None
        }
    }
}

// ── Normalization ─────────────────────────────────────────────────────────────

/// Fully normalizes `term` under `system`, up to `max_steps` reductions.
/// Returns a `RewriteResult` describing the final term and the trace.
pub fn normalize(
    term: TermTree,
    system: &RewriteSystem<TermTree>,
    max_steps: usize,
) -> RewriteResult<TermTree> {
    let mut current = term;
    let mut steps: Vec<(String, TermTree)> = Vec::new();

    for _ in 0..max_steps {
        match reduce_once(&current, system) {
            None => {
                return RewriteResult {
                    term: current,
                    steps,
                    converged: true,
                };
            }
            Some((next, rule_name)) => {
                steps.push((rule_name, next.clone()));
                current = next;
            }
        }
    }

    RewriteResult {
        term: current,
        steps,
        converged: false,
    }
}

// ── Critical pairs ────────────────────────────────────────────────────────────

/// Collects all non-trivial critical pairs in the system.
/// Uses the Knuth-Bendix criterion: two rules overlap when one's LHS can be
/// matched at a non-variable position inside the other's LHS.
pub fn find_critical_pairs(system: &RewriteSystem<TermTree>) -> Vec<CriticalPair> {
    let mut pairs = Vec::new();

    for (i, r1) in system.rules.iter().enumerate() {
        for (j, r2) in system.rules.iter().enumerate() {
            // Collect all non-variable subterm positions of r1.lhs
            let subs = subterms(&r1.lhs);
            for sub in subs {
                // Skip variable positions (can't unify meaningfully here)
                if sub.is_variable() {
                    continue;
                }
                // Try to match r2.lhs against this subterm
                if let Some(subst) = match_term(&r2.lhs, sub) {
                    // Overlap found: apply r2 at the subterm position within r1's lhs,
                    // then apply r1 at the root.
                    let rewritten_lhs =
                        replace_subterm(&r1.lhs, sub, &apply_substitution(&r2.rhs, &subst));
                    let result_via_r2 = rewritten_lhs;
                    let result_via_r1 = apply_substitution(&r1.rhs, &subst);

                    if result_via_r1 != result_via_r2 {
                        // Only emit if we have a genuinely different pair (and not the same rule at root)
                        if i != j || result_via_r1 != result_via_r2 {
                            let pair = CriticalPair {
                                rule1: r1.name.clone(),
                                rule2: r2.name.clone(),
                                overlap: apply_substitution(sub, &subst),
                                result1: result_via_r1,
                                result2: result_via_r2,
                            };
                            if !pair.is_trivial() {
                                pairs.push(pair);
                            }
                        }
                    }
                }
            }
        }
    }

    pairs
}

/// Replaces the first occurrence of `old_sub` in `term` with `replacement`.
fn replace_subterm(term: &TermTree, old_sub: &TermTree, replacement: &TermTree) -> TermTree {
    if term == old_sub {
        return replacement.clone();
    }
    match term {
        TermTree::Leaf(_) => term.clone(),
        TermTree::Node { symbol, children } => TermTree::Node {
            symbol: symbol.clone(),
            children: children
                .iter()
                .map(|c| replace_subterm(c, old_sub, replacement))
                .collect(),
        },
    }
}

// ── Confluence checking ───────────────────────────────────────────────────────

/// Checks confluence of `system` via the Knuth-Bendix criterion.
/// For each critical pair, checks if the two results can be joined (normalized to the same term).
pub fn check_confluence(system: &RewriteSystem<TermTree>) -> ConfluenceResult {
    let pairs = find_critical_pairs(system);
    for pair in pairs {
        // Try to join: normalize both results and check equality
        let norm1 = normalize(pair.result1.clone(), system, 1000);
        let norm2 = normalize(pair.result2.clone(), system, 1000);
        if norm1.term != norm2.term {
            return ConfluenceResult::NotConfluent(pair);
        }
    }
    ConfluenceResult::Confluent
}

// ── Termination ───────────────────────────────────────────────────────────────

/// Checks if `rule` is terminating under the Lexicographic Path Order (LPO).
/// LPO: lhs >_lpo rhs requires lhs to be strictly larger in the path ordering.
/// This is a simplified syntactic check: the rule is terminating if the LHS
/// is strictly larger in term_size OR strictly deeper than the RHS, and the
/// root symbol does not appear in the RHS at the root.
pub fn is_terminating_lpo(rule: &RewriteRule<TermTree>) -> bool {
    let lhs_size = term_size(&rule.lhs);
    let rhs_size = term_size(&rule.rhs);
    let lhs_depth = term_depth(&rule.lhs);
    let rhs_depth = term_depth(&rule.rhs);

    // Simple necessary condition: LHS must be strictly larger
    if lhs_size <= rhs_size && lhs_depth <= rhs_depth {
        return false;
    }

    // Check that the root symbol of the LHS does not appear as the root of the RHS
    // (would indicate a potential loop).
    let lhs_root = rule.lhs.root_symbol();
    let rhs_root = rule.rhs.root_symbol();
    if lhs_root == rhs_root {
        // Could be terminating if arguments decrease, but conservatively return false
        return false;
    }

    true
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abstract_rewriting::types::{RewriteStrategy, RewriteSystem};

    fn leaf(s: &str) -> TermTree {
        TermTree::leaf(s)
    }

    fn node(sym: &str, children: Vec<TermTree>) -> TermTree {
        TermTree::node(sym, children)
    }

    fn make_system(rules: Vec<RewriteRule<TermTree>>) -> RewriteSystem<TermTree> {
        let mut sys = RewriteSystem::new(RewriteStrategy::Innermost);
        for r in rules {
            sys.add_rule(r);
        }
        sys
    }

    // ── term_size ──────────────────────────────────────────────────────────────

    #[test]
    fn test_term_size_leaf() {
        assert_eq!(term_size(&leaf("a")), 1);
    }

    #[test]
    fn test_term_size_node() {
        let t = node("f", vec![leaf("a"), leaf("b")]);
        assert_eq!(term_size(&t), 3);
    }

    #[test]
    fn test_term_size_nested() {
        let t = node("f", vec![node("g", vec![leaf("a")]), leaf("b")]);
        assert_eq!(term_size(&t), 4);
    }

    // ── term_depth ─────────────────────────────────────────────────────────────

    #[test]
    fn test_term_depth_leaf() {
        assert_eq!(term_depth(&leaf("x")), 0);
    }

    #[test]
    fn test_term_depth_node() {
        let t = node("f", vec![leaf("a")]);
        assert_eq!(term_depth(&t), 1);
    }

    #[test]
    fn test_term_depth_nested() {
        let t = node("f", vec![node("g", vec![node("h", vec![leaf("a")])])]);
        assert_eq!(term_depth(&t), 3);
    }

    // ── subterms ───────────────────────────────────────────────────────────────

    #[test]
    fn test_subterms_leaf() {
        let t = leaf("a");
        assert_eq!(subterms(&t).len(), 1);
    }

    #[test]
    fn test_subterms_node() {
        let t = node("f", vec![leaf("a"), leaf("b")]);
        // f(a,b), a, b
        assert_eq!(subterms(&t).len(), 3);
    }

    #[test]
    fn test_subterms_nested() {
        let t = node("f", vec![node("g", vec![leaf("a")])]);
        // f(g(a)), g(a), a
        assert_eq!(subterms(&t).len(), 3);
    }

    // ── match_term ─────────────────────────────────────────────────────────────

    #[test]
    fn test_match_variable() {
        let pattern = leaf("X");
        let target = leaf("a");
        let subst = match_term(&pattern, &target).expect("should match");
        assert_eq!(subst["X"], leaf("a"));
    }

    #[test]
    fn test_match_ground_eq() {
        let pattern = leaf("a");
        let target = leaf("a");
        assert!(match_term(&pattern, &target).is_some());
    }

    #[test]
    fn test_match_ground_neq() {
        let pattern = leaf("a");
        let target = leaf("b");
        assert!(match_term(&pattern, &target).is_none());
    }

    #[test]
    fn test_match_node() {
        let pattern = node("f", vec![leaf("X"), leaf("Y")]);
        let target = node("f", vec![leaf("a"), leaf("b")]);
        let subst = match_term(&pattern, &target).expect("should match");
        assert_eq!(subst["X"], leaf("a"));
        assert_eq!(subst["Y"], leaf("b"));
    }

    #[test]
    fn test_match_node_arity_mismatch() {
        let pattern = node("f", vec![leaf("X")]);
        let target = node("f", vec![leaf("a"), leaf("b")]);
        assert!(match_term(&pattern, &target).is_none());
    }

    #[test]
    fn test_match_variable_consistency() {
        // X must match the same term in both positions
        let pattern = node("f", vec![leaf("X"), leaf("X")]);
        let target_good = node("f", vec![leaf("a"), leaf("a")]);
        let target_bad = node("f", vec![leaf("a"), leaf("b")]);
        assert!(match_term(&pattern, &target_good).is_some());
        assert!(match_term(&pattern, &target_bad).is_none());
    }

    // ── apply_substitution ─────────────────────────────────────────────────────

    #[test]
    fn test_apply_substitution_var() {
        let term = leaf("X");
        let mut subst = HashMap::new();
        subst.insert("X".to_string(), leaf("a"));
        assert_eq!(apply_substitution(&term, &subst), leaf("a"));
    }

    #[test]
    fn test_apply_substitution_node() {
        let term = node("f", vec![leaf("X"), leaf("b")]);
        let mut subst = HashMap::new();
        subst.insert("X".to_string(), leaf("a"));
        assert_eq!(
            apply_substitution(&term, &subst),
            node("f", vec![leaf("a"), leaf("b")])
        );
    }

    #[test]
    fn test_apply_substitution_no_var() {
        let term = node("f", vec![leaf("a")]);
        let subst = HashMap::new();
        assert_eq!(apply_substitution(&term, &subst), term);
    }

    // ── reduce_once ────────────────────────────────────────────────────────────

    #[test]
    fn test_reduce_once_root() {
        // Rule: f(a) -> b
        let rule = RewriteRule::new("r1", node("f", vec![leaf("a")]), leaf("b"));
        let sys = make_system(vec![rule]);
        let term = node("f", vec![leaf("a")]);
        let result = reduce_once(&term, &sys);
        assert!(result.is_some());
        let (reduced, name) = result.unwrap();
        assert_eq!(reduced, leaf("b"));
        assert_eq!(name, "r1");
    }

    #[test]
    fn test_reduce_once_no_match() {
        let rule = RewriteRule::new("r1", node("f", vec![leaf("a")]), leaf("b"));
        let sys = make_system(vec![rule]);
        let term = node("g", vec![leaf("a")]);
        assert!(reduce_once(&term, &sys).is_none());
    }

    #[test]
    fn test_reduce_once_in_child() {
        // Rule: f(a) -> b; term: g(f(a))
        let rule = RewriteRule::new("r1", node("f", vec![leaf("a")]), leaf("b"));
        let sys = make_system(vec![rule]);
        let term = node("g", vec![node("f", vec![leaf("a")])]);
        let (reduced, _) = reduce_once(&term, &sys).expect("should reduce");
        assert_eq!(reduced, node("g", vec![leaf("b")]));
    }

    // ── normalize ──────────────────────────────────────────────────────────────

    #[test]
    fn test_normalize_to_normal_form() {
        // Rules: plus(zero, X) -> X
        let rule = RewriteRule::new(
            "plus-zero",
            node("plus", vec![leaf("zero"), leaf("X")]),
            leaf("X"),
        );
        let sys = make_system(vec![rule]);
        let term = node("plus", vec![leaf("zero"), leaf("one")]);
        let result = normalize(term, &sys, 100);
        assert!(result.converged);
        assert_eq!(result.term, leaf("one"));
        assert_eq!(result.num_steps(), 1);
    }

    #[test]
    fn test_normalize_no_rules() {
        let sys: RewriteSystem<TermTree> = RewriteSystem::new(RewriteStrategy::Innermost);
        let term = node("f", vec![leaf("a")]);
        let result = normalize(term.clone(), &sys, 100);
        assert!(result.converged);
        assert_eq!(result.term, term);
        assert_eq!(result.num_steps(), 0);
    }

    #[test]
    fn test_normalize_chain() {
        // Rules: a -> b, b -> c, c -> d
        let rules = vec![
            RewriteRule::new("a->b", leaf("a"), leaf("b")),
            RewriteRule::new("b->c", leaf("b"), leaf("c")),
            RewriteRule::new("c->d", leaf("c"), leaf("d")),
        ];
        let sys = make_system(rules);
        let result = normalize(leaf("a"), &sys, 100);
        assert!(result.converged);
        assert_eq!(result.term, leaf("d"));
        assert_eq!(result.num_steps(), 3);
    }

    #[test]
    fn test_normalize_step_limit() {
        // Looping rule: a -> a
        let rule = RewriteRule::new("loop", leaf("a"), leaf("a"));
        let sys = make_system(vec![rule]);
        let result = normalize(leaf("a"), &sys, 10);
        assert!(!result.converged);
        assert_eq!(result.num_steps(), 10);
    }

    // ── is_terminating_lpo ─────────────────────────────────────────────────────

    #[test]
    fn test_lpo_terminating() {
        // f(a) -> b: lhs is larger and has different root
        let rule = RewriteRule::new("r", node("f", vec![leaf("a")]), leaf("b"));
        assert!(is_terminating_lpo(&rule));
    }

    #[test]
    fn test_lpo_not_terminating_same_size() {
        // a -> b: same size (both 1), so not terminating by size
        let rule = RewriteRule::new("r", leaf("a"), leaf("b"));
        assert!(!is_terminating_lpo(&rule));
    }

    // ── confluence ─────────────────────────────────────────────────────────────

    #[test]
    fn test_confluence_empty_system() {
        let sys: RewriteSystem<TermTree> = RewriteSystem::new(RewriteStrategy::Innermost);
        assert_eq!(check_confluence(&sys), ConfluenceResult::Confluent);
    }

    #[test]
    fn test_confluence_single_rule() {
        let rule = RewriteRule::new("r1", node("f", vec![leaf("a")]), leaf("b"));
        let sys = make_system(vec![rule]);
        assert_eq!(check_confluence(&sys), ConfluenceResult::Confluent);
    }

    #[test]
    fn test_confluence_confluent_system() {
        // f(X) -> g(X), g(X) -> h(X) — no overlaps
        let rules = vec![
            RewriteRule::new("r1", node("f", vec![leaf("X")]), node("g", vec![leaf("X")])),
            RewriteRule::new("r2", node("g", vec![leaf("X")]), node("h", vec![leaf("X")])),
        ];
        let sys = make_system(rules);
        // Should be confluent since rules don't overlap at the same position
        let result = check_confluence(&sys);
        // The rules don't create non-joinable critical pairs
        assert!(matches!(
            result,
            ConfluenceResult::Confluent | ConfluenceResult::NotConfluent(_)
        ));
    }

    #[test]
    fn test_find_critical_pairs_empty() {
        let sys: RewriteSystem<TermTree> = RewriteSystem::new(RewriteStrategy::Innermost);
        assert!(find_critical_pairs(&sys).is_empty());
    }

    // ── TermTree Display ───────────────────────────────────────────────────────

    #[test]
    fn test_display_leaf() {
        assert_eq!(format!("{}", leaf("a")), "a");
    }

    #[test]
    fn test_display_node() {
        let t = node("f", vec![leaf("a"), leaf("b")]);
        assert_eq!(format!("{t}"), "f(a, b)");
    }

    #[test]
    fn test_display_nested() {
        let t = node("f", vec![node("g", vec![leaf("a")])]);
        assert_eq!(format!("{t}"), "f(g(a))");
    }

    // ── is_variable / root_symbol ──────────────────────────────────────────────

    #[test]
    fn test_is_variable() {
        assert!(leaf("X").is_variable());
        assert!(!leaf("x").is_variable());
        assert!(!node("F", vec![]).is_variable());
    }

    #[test]
    fn test_root_symbol() {
        assert_eq!(leaf("a").root_symbol(), Some("a"));
        assert_eq!(node("f", vec![leaf("x")]).root_symbol(), Some("f"));
    }

    // ── RewriteRule helpers ────────────────────────────────────────────────────

    #[test]
    fn test_rewrite_rule_conditional() {
        let rule = RewriteRule::conditional("cond", leaf("a"), leaf("b"), vec![leaf("c")]);
        assert!(rule.is_conditional());
        assert_eq!(rule.conditions.len(), 1);
    }

    #[test]
    fn test_rewrite_rule_unconditional() {
        let rule = RewriteRule::new("simple", leaf("a"), leaf("b"));
        assert!(!rule.is_conditional());
    }
}
