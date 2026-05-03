//! Types for the `field_simp` tactic — denominator clearing and fraction normalization.

use oxilean_kernel::Expr;

/// Configuration for the `field_simp` tactic.
#[derive(Debug, Clone)]
pub struct FieldSimpConfig {
    /// Maximum simplification steps before giving up.
    pub max_steps: usize,
    /// Whether to multiply both sides by denominators to clear fractions.
    pub clear_denominators: bool,
    /// Whether to further normalize the resulting expression.
    pub normalize_result: bool,
}

impl Default for FieldSimpConfig {
    fn default() -> Self {
        Self {
            max_steps: 200,
            clear_denominators: true,
            normalize_result: true,
        }
    }
}

/// The result of running `field_simp` on a goal.
#[derive(Debug, Clone)]
pub struct FieldSimpResult {
    /// The simplified expression.
    pub simplified: Expr,
    /// How many simplification steps were taken.
    pub num_steps: usize,
    /// Whether any change was made.
    pub changed: bool,
}

/// A pattern identifying a division or inverse sub-expression.
///
/// Represents either `a / b` or `a⁻¹` found somewhere in an expression tree.
#[derive(Debug, Clone, PartialEq)]
pub enum DivisionPattern {
    /// Division: `numerator / denominator`.
    Div {
        /// The numerator expression.
        numerator: Expr,
        /// The denominator expression.
        denominator: Expr,
    },
    /// Multiplicative inverse: `expr⁻¹`.
    Inv {
        /// The expression being inverted.
        inner: Expr,
    },
}

impl DivisionPattern {
    /// Returns the denominator expression for this pattern.
    pub fn denominator(&self) -> &Expr {
        match self {
            DivisionPattern::Div { denominator, .. } => denominator,
            DivisionPattern::Inv { inner } => inner,
        }
    }

    /// Returns the numerator, if this is a `Div` pattern.
    pub fn numerator(&self) -> Option<&Expr> {
        match self {
            DivisionPattern::Div { numerator, .. } => Some(numerator),
            DivisionPattern::Inv { .. } => None,
        }
    }
}
