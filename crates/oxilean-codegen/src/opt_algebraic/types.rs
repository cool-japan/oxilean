//! Types for the Algebraic Simplification optimisation pass.

use std::collections::HashMap;

/// An algebraic expression tree.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlgExpr {
    /// An integer constant.
    Const(i64),
    /// A named variable.
    Var(String),
    /// Addition.
    Add(Box<AlgExpr>, Box<AlgExpr>),
    /// Subtraction.
    Sub(Box<AlgExpr>, Box<AlgExpr>),
    /// Multiplication.
    Mul(Box<AlgExpr>, Box<AlgExpr>),
    /// Integer division.
    Div(Box<AlgExpr>, Box<AlgExpr>),
    /// Negation.
    Neg(Box<AlgExpr>),
    /// Exponentiation.
    Pow(Box<AlgExpr>, Box<AlgExpr>),
    /// Integer modulo.
    Mod(Box<AlgExpr>, Box<AlgExpr>),
}

/// An algebraic identity / rewrite rule (stored as human-readable strings for
/// reporting purposes; actual application is performed programmatically).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SimplRule {
    /// Short name identifying the rule (e.g. `"add_zero"`).
    pub name: String,
    /// The pattern this rule matches (e.g. `"x + 0"`).
    pub pattern: String,
    /// The replacement this rule produces (e.g. `"x"`).
    pub replacement: String,
}

impl SimplRule {
    /// Construct a new rule with the given name, pattern, and replacement.
    pub fn new(
        name: impl Into<String>,
        pattern: impl Into<String>,
        replacement: impl Into<String>,
    ) -> Self {
        SimplRule {
            name: name.into(),
            pattern: pattern.into(),
            replacement: replacement.into(),
        }
    }
}

/// The result of simplifying an expression.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SimplResult {
    /// The simplified expression.
    pub expr: AlgExpr,
    /// Human-readable description of each simplification step applied.
    pub steps: Vec<String>,
    /// `true` if at least one simplification was applied.
    pub reduced: bool,
}

/// Configuration for the algebraic simplification pass.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AlgSimplConfig {
    /// Maximum number of simplification passes to perform.
    pub max_passes: usize,
    /// Whether to evaluate constant sub-expressions eagerly.
    pub fold_constants: bool,
    /// Whether to expand products (e.g. `(a+b)*c = a*c + b*c`).
    pub expand: bool,
    /// Whether to factorise expressions (e.g. `a*c + b*c = (a+b)*c`).
    pub factor: bool,
}

impl Default for AlgSimplConfig {
    fn default() -> Self {
        AlgSimplConfig {
            max_passes: 20,
            fold_constants: true,
            expand: false,
            factor: false,
        }
    }
}

/// Statistics gathered by the simplification pass.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct SimplStats {
    /// Total number of identity rules applied.
    pub rules_applied: usize,
    /// Number of full simplification passes completed.
    pub passes_completed: usize,
    /// Node count of the expression before simplification.
    pub size_before: usize,
    /// Node count of the expression after simplification.
    pub size_after: usize,
}

/// All built-in algebraic rewrite rules.
pub fn builtin_rules() -> Vec<SimplRule> {
    vec![
        SimplRule::new("add_zero_right", "x + 0", "x"),
        SimplRule::new("add_zero_left", "0 + x", "x"),
        SimplRule::new("mul_one_right", "x * 1", "x"),
        SimplRule::new("mul_one_left", "1 * x", "x"),
        SimplRule::new("mul_zero_right", "x * 0", "0"),
        SimplRule::new("mul_zero_left", "0 * x", "0"),
        SimplRule::new("sub_zero", "x - 0", "x"),
        SimplRule::new("add_self", "x + x", "2*x"),
        SimplRule::new("sub_self", "x - x", "0"),
        SimplRule::new("div_self", "x / x", "1"),
        SimplRule::new("neg_zero", "0 - x", "-x"),
        SimplRule::new("double_neg", "-(-x)", "x"),
        SimplRule::new("pow_zero", "x ^ 0", "1"),
        SimplRule::new("pow_one", "x ^ 1", "x"),
        SimplRule::new("zero_pow", "0 ^ x", "0"),
    ]
}
