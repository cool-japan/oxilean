//! Implementation of the `abel` tactic — abelian group normalization.
//!
//! Normalizes both sides of an equality goal as elements of a free abelian
//! group, then checks whether the normal forms are equal.  This decides
//! equalities in any abelian group (e.g. integers, vectors, modules) as
//! long as the group operations appear syntactically as `+`, `-`, and
//! scalar multiplication.

#![allow(dead_code)]

use super::types::{AbelConfig, AbelNormalForm, AbelTerm};
use crate::basic::MetaContext;
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{Expr, Literal, Name};

// ---------------------------------------------------------------------------
// Helper utilities (local copies to avoid cross-module pub(super) issues)
// ---------------------------------------------------------------------------

/// Extract the constant name from an expression.
fn const_name(expr: &Expr) -> Option<String> {
    if let Expr::Const(name, _) = expr {
        Some(name.to_string())
    } else {
        None
    }
}

/// Check whether a constant name is the equality combinator.
fn is_eq_const(expr: &Expr) -> bool {
    matches!(expr, Expr::Const(name, _) if {
        let s = name.to_string();
        s == "Eq" || s == "eq"
    })
}

/// Extract `(lhs, rhs)` from an equality expression, or return `None`.
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

/// Produce a canonical string key for an `Expr`, used for sorting atoms.
fn atom_key(expr: &Expr) -> String {
    match expr {
        Expr::Const(name, _) => name.to_string(),
        Expr::FVar(id) => format!("fvar_{}", id.0),
        Expr::Lit(Literal::Nat(n)) => format!("lit_{}", n),
        Expr::Lit(Literal::Str(s)) => format!("str_{}", s),
        Expr::App(_, _) => {
            // Use a stable hash-like representation.
            format!("app_{}", expr.size())
        }
        _ => format!("other_{}", expr.size()),
    }
}

// ---------------------------------------------------------------------------
// AbelTerm construction
// ---------------------------------------------------------------------------

/// Parse a kernel `Expr` into an `AbelTerm`.
///
/// Recognises:
/// - `0` / `Nat.zero` / `Int.zero` → `Zero`
/// - `HAdd.hAdd a b` / `Add.add a b` → `Sum([to_abel(a), to_abel(b)])`
/// - `HSub.hSub a b` / `Sub.sub a b` → `Sum([to_abel(a), Neg(to_abel(b))])`
/// - `HMul.hMul (n : literal) b` → `SMul(n, to_abel(b))`
/// - `Neg.neg a` / `Int.neg a` → `Neg(to_abel(a))`
/// - Anything else → `Atom(expr)`
pub fn expr_to_abel(expr: &Expr) -> AbelTerm {
    match expr {
        Expr::Lit(Literal::Nat(0)) => AbelTerm::Zero,
        Expr::Const(name, _)
            if matches!(name.to_string().as_str(), "Nat.zero" | "Int.zero" | "zero") =>
        {
            AbelTerm::Zero
        }
        Expr::App(func, arg) => {
            // Unary ops: Neg.neg, Int.neg, Neg.
            if let Some(name) = const_name(func) {
                match name.as_str() {
                    "Neg.neg" | "Int.neg" | "neg" => {
                        return AbelTerm::negate(expr_to_abel(arg));
                    }
                    _ => {}
                }
            }
            // Binary ops: App(App(op, lhs), rhs)
            if let Expr::App(op_expr, lhs) = func.as_ref() {
                if let Some(op_name) = const_name(op_expr) {
                    let abel_lhs = expr_to_abel(lhs);
                    let abel_rhs = expr_to_abel(arg);
                    match op_name.as_str() {
                        "HAdd.hAdd" | "Add.add" | "add" => {
                            return AbelTerm::Sum(vec![abel_lhs, abel_rhs]);
                        }
                        "HSub.hSub" | "Sub.sub" | "sub" => {
                            return AbelTerm::Sum(vec![abel_lhs, AbelTerm::negate(abel_rhs)]);
                        }
                        "HMul.hMul" | "Mul.mul" | "mul" => {
                            // Detect `k * expr` where `k` is a literal integer.
                            if let AbelTerm::Atom(Expr::Lit(Literal::Nat(n))) = &abel_lhs {
                                let k = *n as i64;
                                return AbelTerm::SMul(k, Box::new(abel_rhs));
                            }
                            // Otherwise treat as an opaque atom.
                            return AbelTerm::Atom(expr.clone());
                        }
                        _ => {}
                    }
                }
                // Three-arg form: App(App(App(op, _ty), lhs), rhs)
                if let Expr::App(op_expr2, _ty) = op_expr.as_ref() {
                    if let Some(op_name) = const_name(op_expr2) {
                        let abel_lhs = expr_to_abel(lhs);
                        let abel_rhs = expr_to_abel(arg);
                        match op_name.as_str() {
                            "HAdd.hAdd" | "Add.add" => {
                                return AbelTerm::Sum(vec![abel_lhs, abel_rhs]);
                            }
                            "HSub.hSub" | "Sub.sub" => {
                                return AbelTerm::Sum(vec![abel_lhs, AbelTerm::negate(abel_rhs)]);
                            }
                            _ => {}
                        }
                    }
                }
            }
            AbelTerm::Atom(expr.clone())
        }
        _ => AbelTerm::Atom(expr.clone()),
    }
}

// ---------------------------------------------------------------------------
// Normalization
// ---------------------------------------------------------------------------

/// Normalize an `AbelTerm` into a sorted `AbelNormalForm`.
///
/// Steps:
/// 1. Flatten all `Sum` and `SMul` recursively into `(coefficient, atom)` pairs.
/// 2. Merge pairs sharing the same atom key.
/// 3. Drop entries with zero coefficient.
/// 4. Sort by atom key for a canonical order.
pub fn normalize_abel_term(term: AbelTerm) -> AbelNormalForm {
    let mut pairs: Vec<(i64, Expr)> = Vec::new();
    collect_pairs(term, 1, &mut pairs);

    // Merge: group by atom key, sum coefficients.
    let mut map: std::collections::BTreeMap<String, (i64, Expr)> =
        std::collections::BTreeMap::new();
    for (coeff, atom) in pairs {
        let key = atom_key(&atom);
        let entry = map.entry(key).or_insert((0, atom.clone()));
        entry.0 += coeff;
    }

    // Collect non-zero entries, already sorted by BTreeMap key order.
    let terms: Vec<(i64, Expr)> = map.into_values().filter(|(coeff, _)| *coeff != 0).collect();

    AbelNormalForm { terms }
}

/// Recursively collect `(coefficient, atom_expr)` pairs from an `AbelTerm`.
fn collect_pairs(term: AbelTerm, factor: i64, out: &mut Vec<(i64, Expr)>) {
    match term {
        AbelTerm::Zero => {}
        AbelTerm::Atom(expr) => {
            out.push((factor, expr));
        }
        AbelTerm::Neg(inner) => {
            collect_pairs(*inner, -factor, out);
        }
        AbelTerm::SMul(k, inner) => {
            collect_pairs(*inner, factor * k, out);
        }
        AbelTerm::Sum(terms) => {
            for t in terms {
                collect_pairs(t, factor, out);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Reconstruction
// ---------------------------------------------------------------------------

/// Convert an `AbelNormalForm` back to an `Expr`.
///
/// The result is a left-associated sum of `coeff * atom` terms.
/// An empty normal form returns `0` (as `Nat.zero`).
pub fn abel_to_expr(nf: &AbelNormalForm) -> Expr {
    if nf.terms.is_empty() {
        return Expr::Const(Name::str("Nat.zero"), vec![]);
    }
    let add_const = Expr::Const(Name::str("HAdd.hAdd"), vec![]);
    let mut it = nf.terms.iter();
    let first = it.next().expect("non-empty checked above");
    let mut acc = coeff_atom_to_expr(first.0, &first.1);
    for (coeff, atom) in it {
        let rhs = coeff_atom_to_expr(*coeff, atom);
        acc = Expr::App(
            Box::new(Expr::App(Box::new(add_const.clone()), Box::new(acc))),
            Box::new(rhs),
        );
    }
    acc
}

/// Build `coeff * atom` or `-coeff * atom` or just `atom`.
fn coeff_atom_to_expr(coeff: i64, atom: &Expr) -> Expr {
    match coeff {
        1 => atom.clone(),
        -1 => {
            let neg = Expr::Const(Name::str("Neg.neg"), vec![]);
            Expr::App(Box::new(neg), Box::new(atom.clone()))
        }
        k => {
            let lit = Expr::Lit(oxilean_kernel::Literal::Nat(k.unsigned_abs()));
            let mul = Expr::Const(Name::str("HMul.hMul"), vec![]);
            let scaled = Expr::App(
                Box::new(Expr::App(Box::new(mul), Box::new(lit))),
                Box::new(atom.clone()),
            );
            if k < 0 {
                let neg = Expr::Const(Name::str("Neg.neg"), vec![]);
                Expr::App(Box::new(neg), Box::new(scaled))
            } else {
                scaled
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Equality comparison
// ---------------------------------------------------------------------------

/// Check whether two `AbelNormalForm`s are equal.
///
/// Two normal forms are equal if and only if they have the same set of
/// `(coefficient, atom_key)` entries.
pub fn abel_forms_equal(a: &AbelNormalForm, b: &AbelNormalForm) -> bool {
    if a.terms.len() != b.terms.len() {
        return false;
    }
    // Build maps from atom key → coefficient and compare.
    let map_a: std::collections::HashMap<String, i64> =
        a.terms.iter().map(|(c, e)| (atom_key(e), *c)).collect();
    let map_b: std::collections::HashMap<String, i64> =
        b.terms.iter().map(|(c, e)| (atom_key(e), *c)).collect();
    map_a == map_b
}

// ---------------------------------------------------------------------------
// Tactic entry point
// ---------------------------------------------------------------------------

/// The `abel` tactic: decide equalities in abelian groups.
///
/// Requires the current goal to be an equality `a = b`.  Converts both
/// sides into `AbelNormalForm` and checks whether they coincide.  If they
/// do, the goal is closed with `rfl`.
pub fn tac_abel(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<()> {
    tac_abel_with_config(state, ctx, &AbelConfig::default())
}

/// `abel` with a custom `AbelConfig`.
pub fn tac_abel_with_config(
    state: &mut TacticState,
    ctx: &mut MetaContext,
    config: &AbelConfig,
) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("abel: goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);

    let (lhs, rhs) = extract_eq_sides(&target)
        .ok_or_else(|| TacticError::GoalMismatch("abel requires an equality goal".into()))?;

    let nf_lhs = normalize_abel_term(expr_to_abel(&lhs));
    let nf_rhs = normalize_abel_term(expr_to_abel(&rhs));

    if nf_lhs.atom_count() > config.max_atoms || nf_rhs.atom_count() > config.max_atoms {
        return Err(TacticError::Failed(format!(
            "abel: normal form exceeds max_atoms={} limit",
            config.max_atoms
        )));
    }

    if abel_forms_equal(&nf_lhs, &nf_rhs) {
        let rfl = Expr::Const(Name::str("rfl"), vec![]);
        state.close_goal(rfl, ctx)?;
        Ok(())
    } else {
        Err(TacticError::Failed(format!(
            "abel: left side '{:?}' differs from right side '{:?}'",
            abel_to_expr(&nf_lhs),
            abel_to_expr(&nf_rhs)
        )))
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::{Expr, Literal, Name};

    fn var(name: &str) -> Expr {
        Expr::Const(Name::str(name), vec![])
    }

    fn nat_lit(n: u64) -> Expr {
        Expr::Lit(Literal::Nat(n))
    }

    fn add_expr(a: Expr, b: Expr) -> Expr {
        let add = Expr::Const(Name::str("HAdd.hAdd"), vec![]);
        Expr::App(Box::new(Expr::App(Box::new(add), Box::new(a))), Box::new(b))
    }

    fn neg_expr(a: Expr) -> Expr {
        let neg = Expr::Const(Name::str("Neg.neg"), vec![]);
        Expr::App(Box::new(neg), Box::new(a))
    }

    fn sub_expr(a: Expr, b: Expr) -> Expr {
        let sub = Expr::Const(Name::str("HSub.hSub"), vec![]);
        Expr::App(Box::new(Expr::App(Box::new(sub), Box::new(a))), Box::new(b))
    }

    #[test]
    fn test_expr_to_abel_zero() {
        let z = nat_lit(0);
        let term = expr_to_abel(&z);
        assert_eq!(term, AbelTerm::Zero);
    }

    #[test]
    fn test_normalize_single_atom() {
        let x = var("x");
        let term = expr_to_abel(&x);
        let nf = normalize_abel_term(term);
        assert_eq!(nf.terms.len(), 1);
        assert_eq!(nf.terms[0].0, 1);
    }

    #[test]
    fn test_normalize_commutativity() {
        // `x + y` and `y + x` should normalize identically.
        let x = var("x");
        let y = var("y");
        let lhs_expr = add_expr(x.clone(), y.clone());
        let rhs_expr = add_expr(y, x);
        let nf_lhs = normalize_abel_term(expr_to_abel(&lhs_expr));
        let nf_rhs = normalize_abel_term(expr_to_abel(&rhs_expr));
        assert!(abel_forms_equal(&nf_lhs, &nf_rhs));
    }

    #[test]
    fn test_normalize_cancellation() {
        // `x + (-x)` should normalize to zero.
        let x = var("x");
        let expr = add_expr(x.clone(), neg_expr(x));
        let nf = normalize_abel_term(expr_to_abel(&expr));
        assert!(nf.is_zero(), "Expected zero normal form, got {:?}", nf);
    }

    #[test]
    fn test_abel_forms_not_equal() {
        // `x` and `y` should have different normal forms.
        let nf_x = normalize_abel_term(expr_to_abel(&var("x")));
        let nf_y = normalize_abel_term(expr_to_abel(&var("y")));
        assert!(!abel_forms_equal(&nf_x, &nf_y));
    }

    #[test]
    fn test_normalize_subtraction() {
        // `x - x` should normalize to zero.
        let x = var("x");
        let expr = sub_expr(x.clone(), x);
        let nf = normalize_abel_term(expr_to_abel(&expr));
        assert!(nf.is_zero(), "x - x should be zero, got {:?}", nf);
    }

    #[test]
    fn test_abel_to_expr_empty() {
        let nf = AbelNormalForm::zero();
        let e = abel_to_expr(&nf);
        assert!(matches!(e, Expr::Const(name, _) if name.to_string() == "Nat.zero"));
    }
}
