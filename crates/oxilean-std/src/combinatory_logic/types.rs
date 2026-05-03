//! Types for combinatory logic (SKI calculus) and bracket abstraction.

/// A term in the SKI combinatory logic calculus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CombTerm {
    /// The S combinator: S x y z = x z (y z).
    S,
    /// The K combinator: K x y = x.
    K,
    /// The I combinator: I x = x.
    I,
    /// Application of one combinator term to another.
    App(Box<CombTerm>, Box<CombTerm>),
    /// A free variable (used during bracket abstraction).
    Var(String),
    /// A named constant (for user-defined combinators or atoms).
    Const(String),
}

/// A single step in a reduction sequence, recording the before/after terms
/// and which reduction rule was applied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReductionStep {
    /// The term before the reduction step.
    pub from: CombTerm,
    /// The term after the reduction step.
    pub to: CombTerm,
    /// The reduction rule that was applied.
    pub rule_applied: CombRule,
}

/// The reduction rules applicable in SKI combinatory logic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombRule {
    /// I x → x
    IRule,
    /// K x y → x
    KRule,
    /// S x y z → x z (y z)
    SRule,
    /// Beta reduction (for lambda-extended combinatory terms).
    Beta,
}

/// A combinator term that has been verified to be in normal form
/// (no more reduction rules apply).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalForm(pub CombTerm);

impl NormalForm {
    /// Extract the underlying term.
    pub fn into_inner(self) -> CombTerm {
        self.0
    }

    /// Get a reference to the underlying term.
    pub fn as_term(&self) -> &CombTerm {
        &self.0
    }
}

/// Algorithm for bracket abstraction when converting lambda terms to SKI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BracketMethod {
    /// Naive bracket abstraction: always uses S, K, I combinators directly.
    /// Simple but produces exponentially large terms.
    #[default]
    Naive,
    /// Optimized bracket abstraction: uses η-reduction when the variable
    /// does not appear free in the body, reducing term size.
    Optimized,
    /// Turner's optimized bracket abstraction: uses B, C, S', B', C' extensions
    /// to produce more compact translations (simulated within pure SKI here).
    TurnerOptimized,
}

/// A converter from lambda calculus to combinatory logic using bracket abstraction.
#[derive(Debug, Clone)]
pub struct LambdaToComb {
    /// The bracket abstraction method to use.
    pub bracket_abstraction: BracketMethod,
}

impl LambdaToComb {
    /// Create a new converter with the given bracket abstraction method.
    pub fn new(method: BracketMethod) -> Self {
        Self {
            bracket_abstraction: method,
        }
    }

    /// Create a converter using the naive bracket abstraction.
    pub fn naive() -> Self {
        Self::new(BracketMethod::Naive)
    }

    /// Create a converter using the optimized bracket abstraction.
    pub fn optimized() -> Self {
        Self::new(BracketMethod::Optimized)
    }

    /// Create a converter using Turner's optimized bracket abstraction.
    pub fn turner() -> Self {
        Self::new(BracketMethod::TurnerOptimized)
    }
}
