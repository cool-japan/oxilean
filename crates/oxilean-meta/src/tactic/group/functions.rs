//! Implementation of the `group` tactic — group word normalization.
//!
//! Parses kernel expressions into `GroupWord`s and performs free-group
//! reduction (cancelling adjacent inverse pairs).  The tactic closes a
//! goal `a = b` if both sides reduce to the same word.

#![allow(dead_code)]

use super::types::{GroupConfig, GroupLetter, GroupWord};
use crate::basic::MetaContext;
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{Expr, Name};

// ---------------------------------------------------------------------------
// Helper utilities
// ---------------------------------------------------------------------------

fn const_name(expr: &Expr) -> Option<String> {
    if let Expr::Const(name, _) = expr {
        Some(name.to_string())
    } else {
        None
    }
}

fn is_eq_const(expr: &Expr) -> bool {
    matches!(expr, Expr::Const(name, _) if {
        let s = name.to_string();
        s == "Eq" || s == "eq"
    })
}

fn extract_eq_sides(expr: &Expr) -> Option<(Expr, Expr)> {
    if let Expr::App(func, rhs) = expr {
        if let Expr::App(func2, lhs) = func.as_ref() {
            if let Expr::App(eq_expr, _ty) = func2.as_ref() {
                if is_eq_const(eq_expr) {
                    return Some((*lhs.clone(), *rhs.clone()));
                }
            }
            if is_eq_const(func2) {
                return Some((*lhs.clone(), *rhs.clone()));
            }
        }
    }
    None
}

/// Produce a stable string key for a group atom.
fn atom_key(expr: &Expr) -> String {
    match expr {
        Expr::Const(name, _) => name.to_string(),
        Expr::FVar(id) => format!("fvar_{}", id.0),
        _ => format!("atom_{}", expr.size()),
    }
}

/// Check structural equality of two expressions (used for letter cancellation).
fn exprs_equal(a: &Expr, b: &Expr) -> bool {
    match (a, b) {
        (Expr::Const(n1, ls1), Expr::Const(n2, ls2)) => n1 == n2 && ls1 == ls2,
        (Expr::FVar(id1), Expr::FVar(id2)) => id1 == id2,
        (Expr::BVar(i), Expr::BVar(j)) => i == j,
        (Expr::App(f1, a1), Expr::App(f2, a2)) => exprs_equal(f1, f2) && exprs_equal(a1, a2),
        (Expr::Lit(l1), Expr::Lit(l2)) => l1 == l2,
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Parsing
// ---------------------------------------------------------------------------

/// Parse a kernel `Expr` into a (possibly unreduced) `GroupWord`.
///
/// Recognises:
/// - `HMul.hMul a b` / `Mul.mul a b` → word concatenation `[...a...] ++ [...b...]`
/// - `Inv.inv a` / `GroupInv.inv a` → invert the word of `a`
/// - `1` / `one` / `Group.one` → identity word `[]`
/// - Anything else → a single positive letter `[expr]`
pub fn expr_to_group_word(expr: &Expr) -> GroupWord {
    match expr {
        // Identity constants.
        Expr::Const(name, _)
            if matches!(
                name.to_string().as_str(),
                "1" | "one" | "One.one" | "Group.one" | "Monoid.one"
            ) =>
        {
            GroupWord::identity()
        }
        Expr::App(func, arg) => {
            // Unary inverse.
            if let Some(name) = const_name(func) {
                if matches!(name.as_str(), "Inv.inv" | "GroupInv.inv" | "inv") {
                    let inner_word = expr_to_group_word(arg);
                    return invert_word(inner_word);
                }
            }
            // Binary multiplication.
            if let Expr::App(op_expr, lhs) = func.as_ref() {
                if let Some(op_name) = const_name(op_expr) {
                    if matches!(op_name.as_str(), "HMul.hMul" | "Mul.mul" | "mul") {
                        let lhs_word = expr_to_group_word(lhs);
                        let rhs_word = expr_to_group_word(arg);
                        return lhs_word.concat(rhs_word);
                    }
                }
                // Three-arg form: App(App(App(op, ty), lhs), rhs)
                if let Expr::App(op_expr2, _ty) = op_expr.as_ref() {
                    if let Some(op_name) = const_name(op_expr2) {
                        if matches!(op_name.as_str(), "HMul.hMul" | "Mul.mul") {
                            let lhs_word = expr_to_group_word(lhs);
                            let rhs_word = expr_to_group_word(arg);
                            return lhs_word.concat(rhs_word);
                        }
                    }
                }
            }
            // Opaque application → single atom.
            GroupWord::atom(expr.clone())
        }
        _ => GroupWord::atom(expr.clone()),
    }
}

/// Invert a word: reverse the letter order and flip each letter.
///
/// `(a · b · c)⁻¹ = c⁻¹ · b⁻¹ · a⁻¹`
pub fn invert_word(word: GroupWord) -> GroupWord {
    let letters = word.letters.into_iter().rev().map(|l| l.invert()).collect();
    GroupWord { letters }
}

// ---------------------------------------------------------------------------
// Reduction
// ---------------------------------------------------------------------------

/// Reduce a `GroupWord` to its free-group normal form.
///
/// Repeatedly scans for adjacent pairs `x · x⁻¹` or `x⁻¹ · x` and removes
/// them, until no more cancellations are possible.
///
/// This is O(n²) in the worst case but correct for all inputs up to
/// `config.max_steps`.
pub fn reduce_word(word: GroupWord) -> GroupWord {
    let config = GroupConfig::default();
    reduce_word_with_config(word, &config)
}

/// Reduce with a custom `GroupConfig`.
pub fn reduce_word_with_config(word: GroupWord, config: &GroupConfig) -> GroupWord {
    let mut letters = word.letters;
    let mut steps = 0;
    loop {
        if steps >= config.max_steps || letters.len() > config.max_length {
            break;
        }
        let cancel_idx = find_cancellable_pair(&letters);
        match cancel_idx {
            None => break,
            Some(i) => {
                letters.remove(i + 1);
                letters.remove(i);
                steps += 1;
            }
        }
    }
    GroupWord { letters }
}

/// Find the index `i` such that `letters[i]` and `letters[i+1]` form a
/// cancellable pair (`x · x⁻¹` or `x⁻¹ · x`).
fn find_cancellable_pair(letters: &[GroupLetter]) -> Option<usize> {
    for i in 0..letters.len().saturating_sub(1) {
        let a = &letters[i];
        let b = &letters[i + 1];
        if exprs_equal(&a.atom, &b.atom) && a.inverse != b.inverse {
            return Some(i);
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Equality
// ---------------------------------------------------------------------------

/// Check whether two reduced `GroupWord`s are equal.
///
/// Two reduced words are equal iff they have the same sequence of letters.
pub fn words_equal(a: &GroupWord, b: &GroupWord) -> bool {
    if a.letters.len() != b.letters.len() {
        return false;
    }
    a.letters
        .iter()
        .zip(b.letters.iter())
        .all(|(la, lb)| la.inverse == lb.inverse && exprs_equal(&la.atom, &lb.atom))
}

// ---------------------------------------------------------------------------
// Tactic entry point
// ---------------------------------------------------------------------------

/// The `group` tactic: decide equalities in groups.
///
/// Requires the current goal to be an equality `a = b`.  Converts both sides
/// to group words, reduces each to normal form, and closes the goal with `rfl`
/// if the words coincide.
pub fn tac_group(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<()> {
    tac_group_with_config(state, ctx, &GroupConfig::default())
}

/// `group` with a custom `GroupConfig`.
pub fn tac_group_with_config(
    state: &mut TacticState,
    ctx: &mut MetaContext,
    config: &GroupConfig,
) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("group: goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);

    let (lhs, rhs) = extract_eq_sides(&target)
        .ok_or_else(|| TacticError::GoalMismatch("group requires an equality goal".into()))?;

    let word_lhs = reduce_word_with_config(expr_to_group_word(&lhs), config);
    let word_rhs = reduce_word_with_config(expr_to_group_word(&rhs), config);

    if word_lhs.len() > config.max_length || word_rhs.len() > config.max_length {
        return Err(TacticError::Failed(format!(
            "group: reduced word exceeds max_length={}",
            config.max_length
        )));
    }

    if words_equal(&word_lhs, &word_rhs) {
        let rfl = Expr::Const(Name::str("rfl"), vec![]);
        state.close_goal(rfl, ctx)?;
        Ok(())
    } else {
        Err(TacticError::Failed(format!(
            "group: words are not equal ({} letters vs {} letters)",
            word_lhs.len(),
            word_rhs.len()
        )))
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::{Expr, Name};

    fn var(name: &str) -> Expr {
        Expr::Const(Name::str(name), vec![])
    }

    fn mul_expr(a: Expr, b: Expr) -> Expr {
        let mul = Expr::Const(Name::str("HMul.hMul"), vec![]);
        Expr::App(Box::new(Expr::App(Box::new(mul), Box::new(a))), Box::new(b))
    }

    fn inv_expr(a: Expr) -> Expr {
        let inv = Expr::Const(Name::str("Inv.inv"), vec![]);
        Expr::App(Box::new(inv), Box::new(a))
    }

    #[test]
    fn test_identity_word_empty() {
        let one = Expr::Const(Name::str("One.one"), vec![]);
        let word = expr_to_group_word(&one);
        assert!(word.is_empty(), "identity should produce empty word");
    }

    #[test]
    fn test_reduce_cancels_inverse() {
        // `x * x⁻¹` should reduce to identity.
        let x = var("x");
        let expr = mul_expr(x.clone(), inv_expr(x));
        let word = expr_to_group_word(&expr);
        let reduced = reduce_word(word);
        assert!(
            reduced.is_empty(),
            "x * x⁻¹ should reduce to identity, got {:?}",
            reduced
        );
    }

    #[test]
    fn test_reduce_no_cancellation() {
        // `x * y` where x ≠ y — cannot cancel.
        let x = var("x");
        let y = var("y");
        let expr = mul_expr(x, y);
        let word = expr_to_group_word(&expr);
        let reduced = reduce_word(word);
        assert_eq!(reduced.len(), 2, "x * y should stay as two letters");
    }

    #[test]
    fn test_words_equal_reflexive() {
        let x = var("x");
        let expr = mul_expr(x.clone(), inv_expr(x.clone()));
        let w1 = reduce_word(expr_to_group_word(&expr));
        let w2 = GroupWord::identity();
        assert!(words_equal(&w1, &w2));
    }

    #[test]
    fn test_reduce_nested_cancellation() {
        // `x * y * y⁻¹ * x⁻¹` → identity.
        let x = var("x");
        let y = var("y");
        let inner = mul_expr(y.clone(), inv_expr(y));
        let outer = mul_expr(mul_expr(x.clone(), inner), inv_expr(x));
        let word = expr_to_group_word(&outer);
        let reduced = reduce_word(word);
        assert!(
            reduced.is_empty(),
            "x * y * y⁻¹ * x⁻¹ should be identity, got {:?}",
            reduced
        );
    }
}
