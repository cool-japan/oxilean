//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::functions::*;
use std::collections::HashMap;

/// A simple linear arithmetic expression over integer variables.
///
/// Used by the `omega` tactic to represent linear constraints.
#[allow(dead_code)]
pub struct OmegaLinearExpr {
    /// Coefficients for each variable (index → coefficient).
    pub coeffs: Vec<i64>,
    /// Constant term.
    pub constant: i64,
}
#[allow(dead_code)]
impl OmegaLinearExpr {
    /// Create a new linear expression with the given coefficients and constant.
    pub fn new(coeffs: Vec<i64>, constant: i64) -> Self {
        Self { coeffs, constant }
    }
    /// Evaluate the expression at the given integer assignment.
    pub fn eval(&self, vars: &[i64]) -> i64 {
        self.coeffs
            .iter()
            .zip(vars.iter())
            .map(|(c, v)| c * v)
            .sum::<i64>()
            + self.constant
    }
    /// Check if the constraint `self ≤ 0` is trivially satisfied (constant ≤ 0, all coeffs 0).
    pub fn is_trivially_nonpositive(&self) -> bool {
        self.constant <= 0 && self.coeffs.iter().all(|&c| c == 0)
    }
    /// Negate this linear expression.
    pub fn negate(&self) -> Self {
        Self {
            coeffs: self.coeffs.iter().map(|&c| -c).collect(),
            constant: -self.constant,
        }
    }
    /// Add two linear expressions.
    pub fn add(&self, other: &Self) -> Self {
        let len = self.coeffs.len().max(other.coeffs.len());
        let mut coeffs = vec![0i64; len];
        for (i, &c) in self.coeffs.iter().enumerate() {
            coeffs[i] += c;
        }
        for (i, &c) in other.coeffs.iter().enumerate() {
            coeffs[i] += c;
        }
        Self {
            coeffs,
            constant: self.constant + other.constant,
        }
    }
}
/// A certificate for the `linarith` tactic: a non-negative linear combination of hypotheses
/// that witnesses a contradiction.
#[allow(dead_code)]
pub struct LinarithCertificate {
    /// Multipliers for each hypothesis.
    pub multipliers: Vec<u64>,
    /// The hypotheses as linear expressions.
    pub hypotheses: Vec<OmegaLinearExpr>,
}
#[allow(dead_code)]
impl LinarithCertificate {
    /// Create a new certificate.
    pub fn new(multipliers: Vec<u64>, hypotheses: Vec<OmegaLinearExpr>) -> Self {
        Self {
            multipliers,
            hypotheses,
        }
    }
    /// Check validity: the weighted sum of hypotheses evaluates to a contradiction.
    /// Returns true if the combination yields constant > 0 with all-zero variable part.
    pub fn is_valid_contradiction(&self, vars: &[i64]) -> bool {
        if self.multipliers.len() != self.hypotheses.len() {
            return false;
        }
        let combined_val: i64 = self
            .multipliers
            .iter()
            .zip(self.hypotheses.iter())
            .map(|(&m, hyp)| m as i64 * hyp.eval(vars))
            .sum();
        combined_val > 0
    }
    /// Check structural validity (all multipliers non-negative, lengths match).
    pub fn is_structurally_valid(&self) -> bool {
        self.multipliers.len() == self.hypotheses.len()
    }
}
/// A ring normalization context for the `ring_nf` / `ring` tactic.
///
/// Tracks a list of ring axioms used in a normalization proof.
#[allow(dead_code)]
pub struct RingNfContext {
    /// Names of ring axioms applied during normalization.
    pub applied_axioms: Vec<String>,
    /// The carrier type name (e.g. "Int", "Real").
    pub carrier: String,
}
#[allow(dead_code)]
impl RingNfContext {
    /// Create a new ring normalization context.
    pub fn new(carrier: &str) -> Self {
        Self {
            applied_axioms: Vec::new(),
            carrier: carrier.to_string(),
        }
    }
    /// Record that an axiom was applied.
    pub fn apply_axiom(&mut self, axiom: &str) {
        self.applied_axioms.push(axiom.to_string());
    }
    /// Check whether the context has applied any axioms.
    pub fn is_nontrivial(&self) -> bool {
        !self.applied_axioms.is_empty()
    }
    /// Get the number of axiom applications.
    pub fn step_count(&self) -> usize {
        self.applied_axioms.len()
    }
    /// Reset the normalization context.
    pub fn reset(&mut self) {
        self.applied_axioms.clear();
    }
}
/// A simp set extended with custom lemmas and priorities.
#[allow(dead_code)]
pub struct ExtendedSimpSet {
    /// Lemma names and priorities in this simp set.
    pub lemmas: Vec<(String, u64)>,
    /// Erased lemma names.
    pub erased: Vec<String>,
}
#[allow(dead_code)]
impl ExtendedSimpSet {
    /// Create an empty simp set.
    pub fn new() -> Self {
        Self {
            lemmas: Vec::new(),
            erased: Vec::new(),
        }
    }
    /// Add a lemma with a given priority.
    pub fn add_lemma(&mut self, name: &str, priority: u64) {
        self.lemmas.push((name.to_string(), priority));
    }
    /// Erase a lemma by name.
    pub fn erase_lemma(&mut self, name: &str) {
        self.erased.push(name.to_string());
    }
    /// Check if a lemma name is active (added but not erased).
    pub fn is_active(&self, name: &str) -> bool {
        let added = self.lemmas.iter().any(|(n, _)| n == name);
        let erased = self.erased.iter().any(|n| n == name);
        added && !erased
    }
    /// Sort lemmas by descending priority.
    pub fn sorted_lemmas(&self) -> Vec<(String, u64)> {
        let mut result: Vec<_> = self
            .lemmas
            .iter()
            .filter(|(n, _)| !self.erased.contains(n))
            .cloned()
            .collect();
        result.sort_by_key(|b| std::cmp::Reverse(b.1));
        result
    }
    /// Total number of active lemmas.
    pub fn active_count(&self) -> usize {
        self.lemmas
            .iter()
            .filter(|(n, _)| !self.erased.contains(n))
            .count()
    }
}
/// A positivity checker that tracks the sign of sub-expressions.
#[allow(dead_code)]
pub struct PositivityChecker {
    /// Map from expression names to sign: -1 (negative), 0 (zero), 1 (positive), 2 (nonneg).
    pub signs: std::collections::HashMap<String, i8>,
}
#[allow(dead_code)]
impl PositivityChecker {
    /// Create a new positivity checker.
    pub fn new() -> Self {
        Self {
            signs: std::collections::HashMap::new(),
        }
    }
    /// Record the sign of an expression.
    pub fn record_sign(&mut self, expr: &str, sign: i8) {
        self.signs.insert(expr.to_string(), sign);
    }
    /// Check whether an expression is known nonnegative.
    pub fn is_nonneg(&self, expr: &str) -> bool {
        match self.signs.get(expr) {
            Some(&s) => s >= 0,
            None => false,
        }
    }
    /// Check whether the product of two expressions is nonnegative.
    pub fn product_nonneg(&self, a: &str, b: &str) -> bool {
        let sa = self.signs.get(a).copied().unwrap_or(0);
        let sb = self.signs.get(b).copied().unwrap_or(0);
        (sa >= 0 && sb >= 0) || (sa <= 0 && sb <= 0)
    }
    /// Infer sign of sum.
    pub fn sum_sign(&self, a: &str, b: &str) -> Option<i8> {
        let sa = self.signs.get(a).copied()?;
        let sb = self.signs.get(b).copied()?;
        if sa >= 0 && sb >= 0 {
            Some(1)
        } else if sa < 0 && sb < 0 {
            Some(-1)
        } else {
            None
        }
    }
}
/// A simp lemma entry: name, priority, and proof expression.
///
/// Represents `SimpLemmaEntry.mk : Name → Nat → Expr → SimpLemmaEntry`
/// in the Lean 4 simp infrastructure.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SimpLemmaEntry {
    /// Name of the lemma.
    pub name: Name,
    /// Priority (higher = applied first).
    pub priority: u64,
    /// The proof term for the simp lemma.
    pub proof: Expr,
}
impl SimpLemmaEntry {
    /// Create a new simp lemma entry.
    #[allow(dead_code)]
    pub fn mk(name: Name, priority: u64, proof: Expr) -> Self {
        SimpLemmaEntry {
            name,
            priority,
            proof,
        }
    }
}
/// A collection of simp lemmas (simp set).
///
/// Represents `SimpTheorems : Type` in the Lean 4 simp infrastructure.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct SimpTheorems {
    /// The lemmas in this set, ordered by priority (highest first).
    pub lemmas: Vec<SimpLemmaEntry>,
}
impl SimpTheorems {
    /// Create an empty simp theorem set.
    #[allow(dead_code)]
    pub fn empty() -> Self {
        SimpTheorems { lemmas: vec![] }
    }
    /// Add a simp lemma entry to the set.
    #[allow(dead_code)]
    pub fn add(&mut self, entry: SimpLemmaEntry) {
        let pos = self
            .lemmas
            .iter()
            .position(|e| e.priority < entry.priority)
            .unwrap_or(self.lemmas.len());
        self.lemmas.insert(pos, entry);
    }
    /// Erase a lemma by name.
    #[allow(dead_code)]
    pub fn erase(&mut self, name: &Name) {
        self.lemmas.retain(|e| &e.name != name);
    }
    /// Get the number of lemmas.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.lemmas.len()
    }
    /// Check if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.lemmas.is_empty()
    }
    /// Look up a lemma by name.
    #[allow(dead_code)]
    pub fn get(&self, name: &Name) -> Option<&SimpLemmaEntry> {
        self.lemmas.iter().find(|e| &e.name == name)
    }
}
