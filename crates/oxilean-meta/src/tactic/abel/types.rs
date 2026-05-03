//! Types for the `abel` tactic — abelian group normalization.

use oxilean_kernel::Expr;

/// Configuration for the `abel` tactic.
#[derive(Debug, Clone)]
pub struct AbelConfig {
    /// Maximum number of atoms (distinct sub-expressions) permitted in the
    /// normal form.  Prevents exponential blowup on pathological inputs.
    pub max_atoms: usize,
}

impl Default for AbelConfig {
    fn default() -> Self {
        Self { max_atoms: 512 }
    }
}

/// A symbolic term in an abelian group, constructed by `expr_to_abel`.
///
/// This is an intermediate algebraic representation before normalization.
/// After normalization the canonical form is `AbelNormalForm`.
#[derive(Debug, Clone, PartialEq)]
pub enum AbelTerm {
    /// The additive identity.
    Zero,
    /// An irreducible atom (leaf expression from the kernel).
    Atom(Expr),
    /// Scalar multiple: `coefficient * term`.
    SMul(i64, Box<AbelTerm>),
    /// Negation: `-term`.
    Neg(Box<AbelTerm>),
    /// Sum of a list of terms.
    Sum(Vec<AbelTerm>),
}

impl AbelTerm {
    /// Convenience constructor: negate a term (wraps the `Neg` variant).
    pub fn negate(t: AbelTerm) -> Self {
        AbelTerm::Neg(Box::new(t))
    }

    /// Convenience constructor: scalar multiplication.
    pub fn smul(k: i64, t: AbelTerm) -> Self {
        AbelTerm::SMul(k, Box::new(t))
    }

    /// Convenience constructor: two-term sum (wraps the `Sum` variant).
    pub fn sum_two(a: AbelTerm, b: AbelTerm) -> Self {
        AbelTerm::Sum(vec![a, b])
    }
}

/// The normalized form of an abelian group expression.
///
/// Represented as a sorted list of `(coefficient, atom)` pairs.
/// Atoms are identified by their `Display` string for ordering purposes.
/// Zero coefficient terms are omitted.
#[derive(Debug, Clone, PartialEq)]
pub struct AbelNormalForm {
    /// Each entry is `(coefficient, atom_expr)`.  Sorted by atom key, no
    /// zero coefficients, at most one entry per distinct atom.
    pub terms: Vec<(i64, Expr)>,
}

impl AbelNormalForm {
    /// The additive identity: no terms.
    pub fn zero() -> Self {
        Self { terms: Vec::new() }
    }

    /// A single atom with coefficient 1.
    pub fn atom(expr: Expr) -> Self {
        Self {
            terms: vec![(1, expr)],
        }
    }

    /// Whether this normal form represents zero.
    pub fn is_zero(&self) -> bool {
        self.terms.is_empty()
    }

    /// The number of distinct atoms.
    pub fn atom_count(&self) -> usize {
        self.terms.len()
    }
}
