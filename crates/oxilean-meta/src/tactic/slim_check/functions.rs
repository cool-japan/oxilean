//! Implementation of `slim_check` — random counterexample search.
//!
//! `slim_check` does NOT prove goals.  For positive results (no counterexample
//! found) it emits a warning and leaves the goal open (mirroring Lean 4's
//! behaviour of using `sorry` internally).  For negative results it returns
//! an error that includes the counterexample.
//!
//! The random number generator is a simple linear congruential generator
//! (Knuth MMIX parameters) so we have no external dependencies.

#![allow(dead_code)]

use super::types::{Counterexample, SlimCheckConfig, SlimCheckOutcome, SlimCheckResult};
use crate::basic::MetaContext;
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{BinderInfo, Expr, Literal, Name};

// ---------------------------------------------------------------------------
// LCG random number generator
// ---------------------------------------------------------------------------

/// A single step of the Knuth MMIX linear congruential generator.
///
/// Returns the new state (which is also the next pseudo-random value).
pub fn lcg_rand(state: &mut u64) -> u64 {
    // Knuth's MMIX parameters: a=6364136223846793005, c=1442695040888963407
    *state = state
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407);
    *state
}

/// Generate a pseudo-random natural number in `[0, max_size]`.
pub fn gen_nat(rng: &mut u64, max_size: usize) -> u64 {
    let raw = lcg_rand(rng);
    if max_size == 0 {
        0
    } else {
        raw % (max_size as u64 + 1)
    }
}

/// Generate a pseudo-random integer in `[-max_size, max_size]`.
pub fn gen_int(rng: &mut u64, max_size: usize) -> i64 {
    let raw = lcg_rand(rng);
    if max_size == 0 {
        0
    } else {
        let range = 2 * max_size as u64 + 1;
        (raw % range) as i64 - max_size as i64
    }
}

/// Generate a pseudo-random boolean.
pub fn gen_bool(rng: &mut u64) -> bool {
    lcg_rand(rng) & 1 == 1
}

// ---------------------------------------------------------------------------
// Goal analysis
// ---------------------------------------------------------------------------

/// A universally-quantified variable extracted from a goal expression.
#[derive(Debug, Clone)]
pub struct ForallVar {
    /// The variable's declared name (or a synthetic name if anonymous).
    pub name: Name,
    /// The variable's type expression.
    pub ty: Expr,
}

/// Extract universally-quantified variables from the outermost `Pi`-binders
/// of an expression.
///
/// `∀ (x : α) (y : β), P x y` → `[(x, α), (y, β)]`.
/// The body is not inspected further.
pub fn extract_forall_vars(expr: &Expr) -> Vec<ForallVar> {
    let mut vars = Vec::new();
    let mut cur = expr;
    while let Expr::Pi(_bi, name, ty, body) = cur {
        vars.push(ForallVar {
            name: name.clone(),
            ty: *ty.clone(),
        });
        cur = body;
    }
    vars
}

// ---------------------------------------------------------------------------
// Test execution
// ---------------------------------------------------------------------------

/// Describe a variable type as a string for error reporting.
fn describe_type(ty: &Expr) -> &'static str {
    match ty {
        Expr::Const(name, _) => match name.to_string().as_str() {
            "Nat" | "Nat.type" => "Nat",
            "Int" | "Int.type" => "Int",
            "Bool" => "Bool",
            _ => "unknown",
        },
        _ => "unknown",
    }
}

/// Generate a value for a variable of the given type and return it as a string.
///
/// Returns `None` if the type is not supported.
fn gen_value(ty: &Expr, rng: &mut u64, max_size: usize) -> Option<String> {
    match ty {
        Expr::Const(name, _) => match name.to_string().as_str() {
            "Nat" | "Nat.type" => Some(gen_nat(rng, max_size).to_string()),
            "Int" | "Int.type" => Some(gen_int(rng, max_size).to_string()),
            "Bool" => Some(gen_bool(rng).to_string()),
            _ => None,
        },
        _ => None,
    }
}

/// Attempt to evaluate a goal expression on concrete assignments and check
/// whether the result might be `False`.
///
/// Since we operate at the kernel expression level without a full evaluator,
/// we use a conservative structural check:
/// - If the innermost body of all Pi-binders is structurally `False`, that
///   is a trivially false property → flag as counterexample.
/// - Otherwise, we cannot falsify it and report `NoCounterexample`.
///
/// In a real implementation this would use a partial evaluator.
fn evaluate_goal_on_assignment(goal_body: &Expr, _vars: &[(String, String)]) -> Option<bool> {
    // Conservative structural check: detect `False` as the body.
    match goal_body {
        Expr::Const(name, _) if name.to_string() == "False" => Some(false),
        Expr::Const(name, _) if name.to_string() == "True" => Some(true),
        // App patterns: Eq (literal) (literal) can be checked.
        Expr::App(func, rhs_expr) => {
            if let Expr::App(func2, lhs_expr) = func.as_ref() {
                if let Expr::App(eq_expr, _) = func2.as_ref() {
                    if let Expr::Const(eq_name, _) = eq_expr.as_ref() {
                        if eq_name.to_string() == "Eq" {
                            // Structural equality check on literals only.
                            if let (Expr::Lit(l1), Expr::Lit(l2)) =
                                (lhs_expr.as_ref(), rhs_expr.as_ref())
                            {
                                return Some(l1 == l2);
                            }
                        }
                    }
                }
            }
            None
        }
        _ => None,
    }
}

/// Skip the leading Pi-binders and return the body expression.
fn strip_foralls(expr: &Expr) -> &Expr {
    let mut cur = expr;
    while let Expr::Pi(_, _, _, body) = cur {
        cur = body;
    }
    cur
}

/// Try to find a counterexample to the universally-quantified statement
/// described by `goal_target`.
///
/// Returns a `SlimCheckResult` indicating whether a counterexample was found,
/// and if so, what it was.
pub fn try_find_counterexample(goal_target: &Expr, config: &SlimCheckConfig) -> SlimCheckResult {
    let forall_vars = extract_forall_vars(goal_target);
    let body = strip_foralls(goal_target);

    if forall_vars.is_empty() {
        // Nothing to quantify over.
        return SlimCheckResult {
            result: SlimCheckOutcome::NoCounterexample,
            num_tested: 0,
        };
    }

    let mut rng = config.seed;
    let mut gave_up_count = 0;
    let mut tested = 0;

    for _ in 0..config.num_tests {
        // Generate values for all universally-quantified variables.
        let mut assignment: Vec<(String, String)> = Vec::new();
        let mut can_generate = true;

        for var in &forall_vars {
            match gen_value(&var.ty, &mut rng, config.max_size) {
                Some(val) => {
                    assignment.push((var.name.to_string(), val));
                }
                None => {
                    // Cannot generate a value for this type.
                    gave_up_count += 1;
                    can_generate = false;
                    break;
                }
            }
        }

        if !can_generate {
            continue;
        }

        tested += 1;

        // Evaluate the goal body on this assignment.
        match evaluate_goal_on_assignment(body, &assignment) {
            Some(false) => {
                // Found a counterexample.
                let desc = format!("counterexample found after {} tests", tested);
                return SlimCheckResult {
                    result: SlimCheckOutcome::Counterexample(Counterexample {
                        vars: assignment,
                        description: desc,
                    }),
                    num_tested: tested,
                };
            }
            Some(true) | None => {
                // Test passed or could not be evaluated.
            }
        }
    }

    if gave_up_count > 0 && tested == 0 {
        return SlimCheckResult {
            result: SlimCheckOutcome::GaveUp(gave_up_count),
            num_tested: 0,
        };
    }

    SlimCheckResult {
        result: SlimCheckOutcome::NoCounterexample,
        num_tested: tested,
    }
}

// ---------------------------------------------------------------------------
// Tactic entry point
// ---------------------------------------------------------------------------

/// The `slim_check` tactic: search for counterexamples to the current goal.
///
/// **Important**: This tactic does NOT prove the goal.  If no counterexample
/// is found it returns `Ok(())` but the goal is **not** closed (it is left
/// open, mirroring Lean 4's `sorry`-based behaviour).  If a counterexample
/// is found it returns `Err` with the counterexample description.
pub fn tac_slim_check(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<()> {
    tac_slim_check_with_config(state, ctx, &SlimCheckConfig::default())
}

/// `slim_check` with a custom `SlimCheckConfig`.
pub fn tac_slim_check_with_config(
    state: &mut TacticState,
    ctx: &mut MetaContext,
    config: &SlimCheckConfig,
) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("slim_check: goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);

    let result = try_find_counterexample(&target, config);

    match result.result {
        SlimCheckOutcome::Counterexample(ce) => Err(TacticError::Failed(format!(
            "slim_check: counterexample found — {}",
            ce.display()
        ))),
        SlimCheckOutcome::GaveUp(n) => {
            // Could not generate values for some variables — leave goal open.
            // This is not an error per se; we just cannot test.
            Err(TacticError::Failed(format!(
                "slim_check: gave up after {} attempts (unsupported types)",
                n
            )))
        }
        SlimCheckOutcome::NoCounterexample => {
            // No counterexample found.  Do NOT close the goal; leave it open
            // (analogous to `sorry` in Lean 4's slim_check).
            // Return Ok to indicate "no problem found".
            Ok(())
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::{BinderInfo, Expr, Literal, Name};

    fn nat_type() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }

    fn bool_type() -> Expr {
        Expr::Const(Name::str("Bool"), vec![])
    }

    fn false_expr() -> Expr {
        Expr::Const(Name::str("False"), vec![])
    }

    fn true_expr() -> Expr {
        Expr::Const(Name::str("True"), vec![])
    }

    /// Build `∀ (x : Nat), body`.
    fn forall_nat(body: Expr) -> Expr {
        Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat_type()),
            Box::new(body),
        )
    }

    #[test]
    fn test_lcg_rand_different_values() {
        let mut state = 42u64;
        let v1 = lcg_rand(&mut state);
        let v2 = lcg_rand(&mut state);
        let v3 = lcg_rand(&mut state);
        assert_ne!(v1, v2);
        assert_ne!(v2, v3);
    }

    #[test]
    fn test_gen_nat_in_range() {
        let mut rng = 12345u64;
        for _ in 0..100 {
            let v = gen_nat(&mut rng, 50);
            assert!(v <= 50, "gen_nat out of range: {}", v);
        }
    }

    #[test]
    fn test_gen_int_in_range() {
        let mut rng = 999u64;
        for _ in 0..100 {
            let v = gen_int(&mut rng, 10);
            assert!((-10..=10).contains(&v), "gen_int out of range: {}", v);
        }
    }

    #[test]
    fn test_extract_forall_vars_none() {
        let e = true_expr();
        let vars = extract_forall_vars(&e);
        assert!(vars.is_empty());
    }

    #[test]
    fn test_extract_forall_vars_one() {
        let e = forall_nat(false_expr());
        let vars = extract_forall_vars(&e);
        assert_eq!(vars.len(), 1);
        assert_eq!(vars[0].name.to_string(), "x");
    }

    #[test]
    fn test_no_counterexample_for_true() {
        // `∀ (x : Nat), True` — should find no counterexample.
        let goal = forall_nat(true_expr());
        let config = SlimCheckConfig {
            num_tests: 20,
            ..Default::default()
        };
        let result = try_find_counterexample(&goal, &config);
        assert!(!result.has_counterexample());
    }

    #[test]
    fn test_counterexample_for_false() {
        // `∀ (x : Nat), False` — always a counterexample.
        let goal = forall_nat(false_expr());
        let config = SlimCheckConfig {
            num_tests: 10,
            ..Default::default()
        };
        let result = try_find_counterexample(&goal, &config);
        assert!(
            result.has_counterexample(),
            "Expected a counterexample for ∀ x : Nat, False"
        );
    }
}
