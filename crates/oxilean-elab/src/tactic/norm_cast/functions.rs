//! Functions implementing the `norm_cast` tactic.

use super::types::{
    CastNormResult, CastRewriteStep, CastRule, CastRuleDB, NormCastConfig, NormCastResult,
    NormCastTacticState,
};

// ──────────────────────────────────────────────────────────────────────────────
// Cast-site identification
// ──────────────────────────────────────────────────────────────────────────────

/// Find all cast application sites in `expr`.
///
/// Returns a list of `(start, end, cast_fn_name)` where `start..end` is the
/// byte range of the outermost cast application in the expression string.
///
/// The simple grammar we recognise is:
/// ```text
/// cast_fn '(' inner ')'
/// ```
/// This is intentionally surface-level string matching; a full implementation
/// would work on a typed AST.
pub fn identify_casts(expr: &str) -> Vec<(usize, usize, String)> {
    let mut result = Vec::new();
    let bytes = expr.as_bytes();
    let len = bytes.len();

    // Walk the string looking for identifier tokens followed by '('.
    let mut i = 0;
    while i < len {
        // Skip whitespace.
        if bytes[i].is_ascii_whitespace() {
            i += 1;
            continue;
        }

        // Try to read an identifier (alphanumeric + '_' + '.').
        if bytes[i].is_ascii_alphabetic() || bytes[i] == b'_' {
            let ident_start = i;
            while i < len
                && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_' || bytes[i] == b'.')
            {
                i += 1;
            }
            let ident = &expr[ident_start..i];

            // Skip whitespace between ident and '('.
            let mut j = i;
            while j < len && bytes[j].is_ascii_whitespace() {
                j += 1;
            }

            if j < len && bytes[j] == b'(' {
                // This looks like a function application.  See if the identifier
                // is cast-shaped: contains "cast", "coe", "to_", "of_", or the
                // last path segment starts with "of" (e.g. `Int.ofNat`).
                let lower = ident.to_lowercase();
                let last_segment = lower.rsplit('.').next().unwrap_or(&lower);
                let is_cast_like = lower.contains("cast")
                    || lower.contains("coe")
                    || lower.starts_with("to_")
                    || lower.starts_with("of_")
                    || lower.contains("_cast")
                    || last_segment.starts_with("of")
                    || last_segment.starts_with("to")
                    || last_segment.starts_with("from");
                if is_cast_like {
                    // Find the matching ')'.
                    if let Some(end) = find_matching_paren(expr, j) {
                        result.push((ident_start, end + 1, ident.to_string()));
                        // Continue scanning after this cast site.
                        i = end + 1;
                        continue;
                    }
                }
            }
        } else {
            i += 1;
        }
    }
    result
}

/// Given the position of a `(` in `expr`, find the position of the matching `)`.
/// Returns `None` if the parenthesis is unbalanced.
pub(super) fn find_matching_paren(expr: &str, open: usize) -> Option<usize> {
    let bytes = expr.as_bytes();
    debug_assert_eq!(bytes[open], b'(');
    let mut depth = 0usize;
    for (i, &byte) in bytes.iter().enumerate().skip(open) {
        match byte {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

// ──────────────────────────────────────────────────────────────────────────────
// Redundant-cast detection
// ──────────────────────────────────────────────────────────────────────────────

/// Return `true` when both type strings are equal (a cast from `T` to `T` is
/// always redundant).
pub fn is_redundant_cast(inner_type: &str, outer_type: &str) -> bool {
    inner_type.trim() == outer_type.trim()
}

// ──────────────────────────────────────────────────────────────────────────────
// Nested-cast collapsing
// ──────────────────────────────────────────────────────────────────────────────

/// Remove `X → X → Y` cast chains: if we see `cast_XY(cast_XY(e))` where both
/// casts use the same function, collapse it to `cast_XY(e)`.
///
/// The function iterates until no further collapse is possible (fixed-point).
pub fn squash_nested_casts(expr: &str) -> String {
    let mut current = expr.to_string();
    loop {
        let next = squash_one_pass(&current);
        if next == current {
            return current;
        }
        current = next;
    }
}

/// One pass of nested-cast squashing.
fn squash_one_pass(expr: &str) -> String {
    let sites = identify_casts(expr);
    // Work backwards through the sites so byte offsets remain valid.
    let mut result = expr.to_string();
    for &(outer_start, outer_end, ref outer_fn) in sites.iter().rev() {
        // The content inside the outer cast's parentheses.
        let paren_open = expr[outer_start..outer_end]
            .find('(')
            .map(|p| outer_start + p);
        let Some(paren_open) = paren_open else {
            continue;
        };
        let inner_content = &result[paren_open + 1..outer_end - 1];

        // Does the inner content start with the same cast function?
        let inner_prefix = format!("{}(", outer_fn);
        let inner_prefix_nosp = outer_fn.as_str();
        let trimmed = inner_content.trim_start();
        if trimmed.starts_with(&inner_prefix) || trimmed.starts_with(inner_prefix_nosp) {
            // Find where the inner cast starts.
            let inner_start_rel = inner_content.len() - trimmed.len();
            let inner_fn_end_rel = inner_start_rel + outer_fn.len();
            // Locate the opening paren of the inner call.
            let after_fn = inner_content[inner_fn_end_rel..].trim_start();
            if after_fn.starts_with('(') {
                // Find the matching ')' for the inner call (within inner_content).
                let inner_open_abs = paren_open
                    + 1
                    + inner_fn_end_rel
                    + (inner_content[inner_fn_end_rel..].len() - after_fn.len());
                if let Some(inner_close_abs) = find_matching_paren(&result, inner_open_abs) {
                    // inner_content[inner_open..inner_close] is the inner argument.
                    let inner_arg = result[inner_open_abs + 1..inner_close_abs].to_string();
                    // Replace the whole outer_start..outer_end with cast_fn(inner_arg).
                    let replacement = format!("{}({})", outer_fn, inner_arg);
                    result.replace_range(outer_start..outer_end, &replacement);
                    return result; // Restart the outer loop.
                }
            }
        }
    }
    result
}

// ──────────────────────────────────────────────────────────────────────────────
// Rule-based cast pushing
// ──────────────────────────────────────────────────────────────────────────────

/// Apply a single `CastRule` to push one cast inward in `expr`.
///
/// The rule matches occurrences of `rule.cast_fn(...)` and, when the
/// argument matches a known distributive pattern, rewrites it.
///
/// Returns the rewritten expression, or `None` if no site was found.
pub fn push_cast_inward(expr: &str, rule: &CastRule) -> Option<String> {
    // Look for `cast_fn(...)` in the expression.
    let needle = format!("{}(", rule.cast_fn);
    let start = expr.find(&needle)?;

    let paren_open = start + rule.cast_fn.len();
    let paren_close = find_matching_paren(expr, paren_open)?;
    let inner = &expr[paren_open + 1..paren_close];

    // Try a set of distributive rewrites based on the inner expression shape.
    let rewritten = try_distribute_cast(rule, inner)?;

    let mut result = expr.to_string();
    result.replace_range(start..paren_close + 1, &rewritten);
    Some(result)
}

/// Attempt to distribute `cast` over an inner expression.
///
/// Handles:
/// - `cast(a + b)` → `cast(a) + cast(b)` (for numeric casts)
/// - `cast(a * b)` → `cast(a) * cast(b)`
/// - `cast(a - b)` → `cast(a) - cast(b)`
/// - `cast(0)`     → `0` / `cast(1)` → `1` for identity-like casts
fn try_distribute_cast(rule: &CastRule, inner: &str) -> Option<String> {
    let trimmed = inner.trim();

    // Identity for literals (numeric).
    if let Ok(n) = trimmed.parse::<i64>() {
        // A cast of a numeric literal stays as-is (the literal itself
        // is the normal form for most numeric tower casts).
        return Some(n.to_string());
    }

    // Distribute over binary arithmetic operators.
    for op in &["+", "-", "*"] {
        if let Some((lhs, rhs)) = split_binary_op(trimmed, op) {
            let result = format!("{}({}) {} {}({})", rule.cast_fn, lhs, op, rule.cast_fn, rhs);
            return Some(result);
        }
    }

    None
}

/// Split `expr` at the top-level binary operator `op` (respecting parentheses).
fn split_binary_op<'a>(expr: &'a str, op: &str) -> Option<(&'a str, &'a str)> {
    let bytes = expr.as_bytes();
    let op_bytes = op.as_bytes();
    let mut depth = 0usize;
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth = depth.saturating_sub(1),
            _ => {
                if depth == 0 && bytes[i..].starts_with(op_bytes) {
                    // Make sure it is a standalone operator (surrounded by
                    // whitespace or the edge of the string) to avoid
                    // mis-matching `*` inside `**`.
                    let before_ok = i == 0 || !bytes[i - 1].is_ascii_alphanumeric();
                    let after_idx = i + op_bytes.len();
                    let after_ok =
                        after_idx >= bytes.len() || !bytes[after_idx].is_ascii_alphanumeric();
                    if before_ok && after_ok {
                        let lhs = expr[..i].trim();
                        let rhs = expr[after_idx..].trim();
                        if !lhs.is_empty() && !rhs.is_empty() {
                            return Some((lhs, rhs));
                        }
                    }
                }
            }
        }
        i += 1;
    }
    None
}

// ──────────────────────────────────────────────────────────────────────────────
// Top-level normalisation pass
// ──────────────────────────────────────────────────────────────────────────────

/// Normalise all casts in `expr` using the rules in `db` and the options in `cfg`.
///
/// The algorithm:
/// 1. (optionally) squash nested same-type casts,
/// 2. (optionally) push remaining casts inward using the rule database,
/// 3. (optionally) normalise numeric literals.
/// 4. Repeat until a fixed point is reached (up to a step limit).
pub fn normalize_casts(expr: &str, rules: &CastRuleDB, cfg: &NormCastConfig) -> CastNormResult {
    const MAX_STEPS: u32 = 1000;

    let mut current = expr.to_string();
    let mut total_steps = 0u32;

    loop {
        if total_steps >= MAX_STEPS {
            break;
        }

        let prev = current.clone();

        // Step 1 – squash nested casts.
        if cfg.squash_casts {
            let squashed = squash_nested_casts(&current);
            if squashed != current {
                total_steps += 1;
                current = squashed;
                continue;
            }
        }

        // Step 2 – push casts inward using the rule database.
        if cfg.push_casts_inward {
            let mut pushed = false;
            for rule in rules.all_rules() {
                if let Some(new_expr) = push_cast_inward(&current, rule) {
                    total_steps += 1;
                    current = new_expr;
                    pushed = true;
                    break;
                }
            }
            if pushed {
                continue;
            }
        }

        // Step 3 – normalise numeric literals embedded in casts.
        if cfg.normalize_numerals {
            let normalised = normalize_numeral_casts(&current);
            if normalised != current {
                total_steps += 1;
                current = normalised;
                continue;
            }
        }

        // Fixed point reached.
        if current == prev {
            break;
        }
    }

    CastNormResult {
        changed: current != expr,
        num_steps: total_steps,
        result_expr: current,
    }
}

/// Rewrite `cast_fn(N)` where `N` is a decimal integer literal into just `N`.
/// This is the normalisation of numeric literals in cast position.
fn normalize_numeral_casts(expr: &str) -> String {
    let sites = identify_casts(expr);
    let mut result = expr.to_string();
    // Process in reverse so offsets stay valid.
    for &(start, end, ref cast_fn) in sites.iter().rev() {
        let inner_str = &expr[start + cast_fn.len() + 1..end - 1];
        let trimmed = inner_str.trim();
        if trimmed.parse::<i64>().is_ok() {
            // Replace cast_fn(N) → N.
            result.replace_range(start..end, trimmed);
        }
    }
    result
}

// ──────────────────────────────────────────────────────────────────────────────
// Goal + hypothesis application
// ──────────────────────────────────────────────────────────────────────────────

/// Apply `norm_cast` to a proof goal and its hypotheses.
///
/// Each hypothesis is a `(name, type_expr)` pair.  The function normalises the
/// goal expression and every hypothesis type, then returns the combined result.
pub fn apply_norm_cast(
    goal: &str,
    hyps: &[(String, String)],
    cfg: &NormCastConfig,
) -> NormCastResult {
    // Build an empty rule database – callers that want rule-driven normalisation
    // should call `normalize_casts` directly with a populated `CastRuleDB`.
    let db = CastRuleDB::new();

    let goal_result = normalize_casts(goal, &db, cfg);
    let mut total_steps = goal_result.num_steps;
    let mut changed = goal_result.changed;

    let normalised_hyps: Vec<(String, String)> = hyps
        .iter()
        .map(|(name, ty)| {
            let r = normalize_casts(ty, &db, cfg);
            total_steps += r.num_steps;
            if r.changed {
                changed = true;
            }
            (name.clone(), r.result_expr)
        })
        .collect();

    NormCastResult {
        goal: goal_result.result_expr,
        hypotheses: normalised_hyps,
        changed,
        total_steps,
    }
}

/// Apply `norm_cast` using a full pipeline: state-tracking version.
///
/// This wrapper constructs a `NormCastTacticState`, runs the normalisation, and
/// records all rewrite steps into the state.
pub fn run_norm_cast_on_state(
    initial_goal: &str,
    rules: &CastRuleDB,
    cfg: &NormCastConfig,
) -> NormCastTacticState {
    let mut state = NormCastTacticState::new(initial_goal);

    // We do a step-by-step pass so we can record individual rewrites.
    const MAX_STEPS: u32 = 1000;
    let mut step_count = 0u32;

    loop {
        if step_count >= MAX_STEPS {
            break;
        }

        let current = state.goal_expr.clone();

        // Try squash.
        if cfg.squash_casts {
            let squashed = squash_nested_casts(&current);
            if squashed != current {
                let dummy_rule = CastRule {
                    from_type: String::new(),
                    to_type: String::new(),
                    cast_fn: "squash".to_string(),
                    priority: 0,
                };
                state.record_step(CastRewriteStep {
                    rule: dummy_rule,
                    position: 0,
                    before: current,
                    after: squashed,
                });
                step_count += 1;
                continue;
            }
        }

        // Try push inward.
        if cfg.push_casts_inward {
            let mut pushed = false;
            for rule in rules.all_rules() {
                if let Some(new_expr) = push_cast_inward(&current, rule) {
                    state.record_step(CastRewriteStep {
                        rule: rule.clone(),
                        position: 0,
                        before: current.clone(),
                        after: new_expr,
                    });
                    step_count += 1;
                    pushed = true;
                    break;
                }
            }
            if pushed {
                continue;
            }
        }

        // Try numeral normalisation.
        if cfg.normalize_numerals {
            let normalised = normalize_numeral_casts(&current);
            if normalised != current {
                let dummy_rule = CastRule {
                    from_type: String::new(),
                    to_type: String::new(),
                    cast_fn: "normalize_numeral".to_string(),
                    priority: 0,
                };
                state.record_step(CastRewriteStep {
                    rule: dummy_rule,
                    position: 0,
                    before: current,
                    after: normalised,
                });
                step_count += 1;
                continue;
            }
        }

        break;
    }

    state
}

// ──────────────────────────────────────────────────────────────────────────────
// Tests
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── CastRuleDB ────────────────────────────────────────────────────────────

    #[test]
    fn test_cast_rule_db_empty() {
        let db = CastRuleDB::new();
        assert!(db.find_rules("Nat", "Int").is_empty());
        assert!(db.all_rules().is_empty());
    }

    #[test]
    fn test_cast_rule_db_add_single() {
        let mut db = CastRuleDB::new();
        let rule = CastRule {
            from_type: "Nat".into(),
            to_type: "Int".into(),
            cast_fn: "Int.ofNat".into(),
            priority: 10,
        };
        db.add_rule(rule.clone());
        let found = db.find_rules("Nat", "Int");
        assert_eq!(found.len(), 1);
        assert_eq!(found[0], rule);
    }

    #[test]
    fn test_cast_rule_db_priority_ordering() {
        let mut db = CastRuleDB::new();
        db.add_rule(CastRule {
            from_type: "Nat".into(),
            to_type: "Int".into(),
            cast_fn: "low_prio".into(),
            priority: 5,
        });
        db.add_rule(CastRule {
            from_type: "Nat".into(),
            to_type: "Int".into(),
            cast_fn: "high_prio".into(),
            priority: 100,
        });
        db.add_rule(CastRule {
            from_type: "Nat".into(),
            to_type: "Int".into(),
            cast_fn: "mid_prio".into(),
            priority: 50,
        });
        let rules = db.find_rules("Nat", "Int");
        assert_eq!(rules[0].cast_fn, "high_prio");
        assert_eq!(rules[1].cast_fn, "mid_prio");
        assert_eq!(rules[2].cast_fn, "low_prio");
    }

    #[test]
    fn test_cast_rule_db_no_cross_type_match() {
        let mut db = CastRuleDB::new();
        db.add_rule(CastRule {
            from_type: "Nat".into(),
            to_type: "Int".into(),
            cast_fn: "Int.ofNat".into(),
            priority: 10,
        });
        assert!(db.find_rules("Int", "Nat").is_empty());
        assert!(db.find_rules("Nat", "Real").is_empty());
    }

    // ── identify_casts ────────────────────────────────────────────────────────

    #[test]
    fn test_identify_casts_simple() {
        let sites = identify_casts("Int.ofNat(42)");
        assert_eq!(sites.len(), 1);
        assert_eq!(sites[0].2, "Int.ofNat");
    }

    #[test]
    fn test_identify_casts_none() {
        let sites = identify_casts("a + b");
        assert!(sites.is_empty());
    }

    #[test]
    fn test_identify_casts_multiple() {
        let expr = "Int.ofNat(x) + Int.ofNat(y)";
        let sites = identify_casts(expr);
        assert_eq!(sites.len(), 2);
    }

    #[test]
    fn test_identify_casts_nested() {
        let sites = identify_casts("to_real(to_int(n))");
        // Outer: to_real, inner: to_int
        assert!(!sites.is_empty());
    }

    // ── is_redundant_cast ─────────────────────────────────────────────────────

    #[test]
    fn test_redundant_cast_same_type() {
        assert!(is_redundant_cast("Nat", "Nat"));
    }

    #[test]
    fn test_redundant_cast_different_types() {
        assert!(!is_redundant_cast("Nat", "Int"));
    }

    #[test]
    fn test_redundant_cast_trimming() {
        assert!(is_redundant_cast("  Nat  ", "Nat"));
    }

    // ── squash_nested_casts ───────────────────────────────────────────────────

    #[test]
    fn test_squash_no_nesting() {
        let expr = "Int.ofNat(x)";
        assert_eq!(squash_nested_casts(expr), expr);
    }

    #[test]
    fn test_squash_same_function_nesting() {
        let result = squash_nested_casts("Int.ofNat(Int.ofNat(x))");
        // Should be collapsed to Int.ofNat(x).
        assert_eq!(result, "Int.ofNat(x)");
    }

    #[test]
    fn test_squash_different_functions_no_squash() {
        let expr = "to_real(to_int(x))";
        // Different cast functions — should NOT be squashed.
        let result = squash_nested_casts(expr);
        assert_eq!(result, expr);
    }

    // ── push_cast_inward ──────────────────────────────────────────────────────

    #[test]
    fn test_push_cast_distributes_add() {
        let rule = CastRule {
            from_type: "Nat".into(),
            to_type: "Int".into(),
            cast_fn: "Int.ofNat".into(),
            priority: 10,
        };
        let result = push_cast_inward("Int.ofNat(a + b)", &rule);
        assert!(result.is_some());
        let r = result.unwrap();
        assert!(r.contains("Int.ofNat(a)") && r.contains("Int.ofNat(b)"));
    }

    #[test]
    fn test_push_cast_literal_normalisation() {
        let rule = CastRule {
            from_type: "Nat".into(),
            to_type: "Int".into(),
            cast_fn: "Int.ofNat".into(),
            priority: 10,
        };
        let result = push_cast_inward("Int.ofNat(42)", &rule);
        assert!(result.is_some());
        let r = result.unwrap();
        assert_eq!(r, "42");
    }

    #[test]
    fn test_push_cast_no_match() {
        let rule = CastRule {
            from_type: "Nat".into(),
            to_type: "Int".into(),
            cast_fn: "Int.ofNat".into(),
            priority: 10,
        };
        let result = push_cast_inward("a + b", &rule);
        assert!(result.is_none());
    }

    // ── normalize_casts ───────────────────────────────────────────────────────

    #[test]
    fn test_normalize_casts_numeral() {
        let mut db = CastRuleDB::new();
        db.add_rule(CastRule {
            from_type: "Nat".into(),
            to_type: "Int".into(),
            cast_fn: "Int.ofNat".into(),
            priority: 10,
        });
        let cfg = NormCastConfig::default();
        let result = normalize_casts("Int.ofNat(5)", &db, &cfg);
        assert!(result.changed);
        assert_eq!(result.result_expr, "5");
    }

    #[test]
    fn test_normalize_casts_unchanged() {
        let db = CastRuleDB::new();
        let cfg = NormCastConfig::default();
        let result = normalize_casts("a + b", &db, &cfg);
        assert!(!result.changed);
        assert_eq!(result.result_expr, "a + b");
    }

    #[test]
    fn test_normalize_casts_steps_counted() {
        let mut db = CastRuleDB::new();
        db.add_rule(CastRule {
            from_type: "Nat".into(),
            to_type: "Int".into(),
            cast_fn: "Int.ofNat".into(),
            priority: 10,
        });
        let cfg = NormCastConfig::default();
        // Int.ofNat(a + b) → Int.ofNat(a) + Int.ofNat(b)
        // → a + b  (two numeral-literal normalisations, but a/b aren't literals)
        let result = normalize_casts("Int.ofNat(a + b)", &db, &cfg);
        assert!(result.num_steps > 0);
    }

    // ── apply_norm_cast ───────────────────────────────────────────────────────

    #[test]
    fn test_apply_norm_cast_goal_only() {
        let cfg = NormCastConfig::default();
        let result = apply_norm_cast("Int.ofNat(3)", &[], &cfg);
        assert!(result.changed);
        assert_eq!(result.goal, "3");
    }

    #[test]
    fn test_apply_norm_cast_with_hyps() {
        let cfg = NormCastConfig::default();
        let hyps = vec![
            ("h1".into(), "Int.ofNat(0)".into()),
            ("h2".into(), "x + y".into()),
        ];
        let result = apply_norm_cast("a", &hyps, &cfg);
        // h1 should be normalised to "0".
        assert_eq!(result.hypotheses[0].1, "0");
        // h2 has no cast, should be unchanged.
        assert_eq!(result.hypotheses[1].1, "x + y");
    }

    #[test]
    fn test_apply_norm_cast_unchanged_goal() {
        let cfg = NormCastConfig::default();
        let result = apply_norm_cast("x = y", &[], &cfg);
        assert!(!result.changed);
    }

    #[test]
    fn test_apply_norm_cast_total_steps() {
        let cfg = NormCastConfig::default();
        let hyps = vec![("h".into(), "Int.ofNat(10)".into())];
        let result = apply_norm_cast("Int.ofNat(5)", &hyps, &cfg);
        assert!(result.total_steps >= 2);
    }

    // ── NormCastTacticState ───────────────────────────────────────────────────

    #[test]
    fn test_tactic_state_record_step() {
        let mut state = NormCastTacticState::new("Int.ofNat(x)");
        assert_eq!(state.num_steps(), 0);
        let rule = CastRule {
            from_type: "Nat".into(),
            to_type: "Int".into(),
            cast_fn: "Int.ofNat".into(),
            priority: 10,
        };
        state.record_step(super::super::types::CastRewriteStep {
            rule,
            position: 0,
            before: "Int.ofNat(x)".into(),
            after: "x".into(),
        });
        assert_eq!(state.num_steps(), 1);
        assert_eq!(state.goal_expr, "x");
    }

    #[test]
    fn test_run_norm_cast_on_state() {
        let mut db = CastRuleDB::new();
        db.add_rule(CastRule {
            from_type: "Nat".into(),
            to_type: "Int".into(),
            cast_fn: "Int.ofNat".into(),
            priority: 10,
        });
        let cfg = NormCastConfig::default();
        let state = run_norm_cast_on_state("Int.ofNat(7)", &db, &cfg);
        assert_eq!(state.goal_expr, "7");
        assert!(state.num_steps() > 0);
    }

    // ── NormCastConfig default ────────────────────────────────────────────────

    #[test]
    fn test_norm_cast_config_default() {
        let cfg = NormCastConfig::default();
        assert!(cfg.push_casts_inward);
        assert!(cfg.squash_casts);
        assert!(cfg.normalize_numerals);
    }

    // ── Edge cases ────────────────────────────────────────────────────────────

    #[test]
    fn test_identify_casts_empty() {
        assert!(identify_casts("").is_empty());
    }

    #[test]
    fn test_normalize_casts_empty_expr() {
        let db = CastRuleDB::new();
        let cfg = NormCastConfig::default();
        let result = normalize_casts("", &db, &cfg);
        assert!(!result.changed);
        assert_eq!(result.result_expr, "");
    }

    #[test]
    fn test_squash_triple_nesting() {
        // cast(cast(cast(x))) should fully collapse to cast(x)
        let result = squash_nested_casts("to_int(to_int(to_int(x)))");
        assert_eq!(result, "to_int(x)");
    }

    #[test]
    fn test_push_cast_distributes_multiply() {
        let rule = CastRule {
            from_type: "Nat".into(),
            to_type: "Int".into(),
            cast_fn: "Int.ofNat".into(),
            priority: 10,
        };
        let result = push_cast_inward("Int.ofNat(a * b)", &rule);
        assert!(result.is_some());
        let r = result.unwrap();
        assert!(r.contains("*"));
    }

    #[test]
    fn test_push_cast_distributes_sub() {
        let rule = CastRule {
            from_type: "Int".into(),
            to_type: "Real".into(),
            cast_fn: "Real.ofInt".into(),
            priority: 10,
        };
        let result = push_cast_inward("Real.ofInt(a - b)", &rule);
        assert!(result.is_some());
        let r = result.unwrap();
        assert!(r.contains("Real.ofInt(a)") && r.contains("Real.ofInt(b)"));
    }

    #[test]
    fn test_norm_cast_config_selective_disable() {
        let cfg = NormCastConfig {
            push_casts_inward: false,
            squash_casts: false,
            normalize_numerals: true,
        };
        let db = CastRuleDB::new();
        // Only numeral normalisation should fire.
        let result = normalize_casts("Int.ofNat(99)", &db, &cfg);
        assert!(result.changed);
        assert_eq!(result.result_expr, "99");
    }
}
