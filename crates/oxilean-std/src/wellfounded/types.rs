//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// Sigma-type measure: `f : α → Nat` witnesses well-foundedness of `(<) ∘ f`.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SizeMeasure {
    /// Name of the measure function.
    pub name: String,
    /// Expected domain type name.
    pub domain: String,
}
impl SizeMeasure {
    /// Create a measure.
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, domain: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            domain: domain.into(),
        }
    }
    /// Standard measure for lists: the length function.
    #[allow(dead_code)]
    pub fn list_length() -> Self {
        Self::new("List.length", "List")
    }
    /// Standard measure for natural numbers: identity.
    #[allow(dead_code)]
    pub fn nat_id() -> Self {
        Self::new("id", "Nat")
    }
    /// Standard measure for trees: size function.
    #[allow(dead_code)]
    pub fn tree_size() -> Self {
        Self::new("Tree.size", "Tree")
    }
    /// Produce a declaration body for the induced well-founded relation.
    #[allow(dead_code)]
    pub fn induced_relation_name(&self) -> String {
        format!("InvImage.lt_{}", self.name.replace('.', "_"))
    }
}
/// A ranked proof tree for tracking structural recursion depth.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum RecursionTree {
    /// A base case (leaf).
    Base { label: String },
    /// A recursive case with a child proof tree.
    Rec {
        label: String,
        decreasing_arg: String,
        child: Box<RecursionTree>,
    },
    /// A branching point (e.g., `match` with multiple arms).
    Branch {
        label: String,
        arms: Vec<RecursionTree>,
    },
}
impl RecursionTree {
    /// Return the maximum depth of the recursion tree.
    #[allow(dead_code)]
    pub fn depth(&self) -> usize {
        match self {
            Self::Base { .. } => 0,
            Self::Rec { child, .. } => 1 + child.depth(),
            Self::Branch { arms, .. } => arms.iter().map(|a| a.depth()).max().unwrap_or(0),
        }
    }
    /// Return the number of recursive calls.
    #[allow(dead_code)]
    pub fn recursive_calls(&self) -> usize {
        match self {
            Self::Base { .. } => 0,
            Self::Rec { child, .. } => 1 + child.recursive_calls(),
            Self::Branch { arms, .. } => arms.iter().map(|a| a.recursive_calls()).sum(),
        }
    }
    /// Return `true` if the tree is simply a base case.
    #[allow(dead_code)]
    pub fn is_base(&self) -> bool {
        matches!(self, Self::Base { .. })
    }
}
/// Well-founded relation builder: constructs `InvImage r f` and `Prod.Lex`.
#[allow(dead_code)]
pub struct WfRelBuilder;
impl WfRelBuilder {
    /// Build the expression for `InvImage r f`:
    /// a well-founded relation on `α` induced by `f : α → β` and `r` on `β`.
    #[allow(dead_code)]
    pub fn inv_image(r: Expr, f: Expr) -> Expr {
        Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("InvImage"), vec![])),
                Box::new(r),
            )),
            Box::new(f),
        )
    }
    /// Build `Prod.Lex r s`: lexicographic product of relations.
    #[allow(dead_code)]
    pub fn prod_lex(r: Expr, s: Expr) -> Expr {
        Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Prod.Lex"), vec![])),
                Box::new(r),
            )),
            Box::new(s),
        )
    }
    /// The standard `Nat.lt` well-founded relation.
    #[allow(dead_code)]
    pub fn nat_lt() -> Expr {
        Expr::Const(Name::str("Nat.lt"), vec![])
    }
    /// The measure-induced relation `InvImage Nat.lt f`.
    #[allow(dead_code)]
    pub fn measure(f: Expr) -> Expr {
        Self::inv_image(Self::nat_lt(), f)
    }
}
/// Heuristic for choosing a termination proof strategy.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminationStrategy {
    /// Recurse on structurally smaller arguments.
    Structural,
    /// Use an explicit numeric measure function.
    Measure,
    /// Use a lexicographic tuple of measures.
    Lexicographic,
    /// Use a well-founded relation other than `<`.
    WellFounded,
    /// Multiple-argument mutual recursion.
    Mutual,
}
impl TerminationStrategy {
    /// Return a description of the strategy.
    #[allow(dead_code)]
    pub fn description(self) -> &'static str {
        match self {
            Self::Structural => "Structural recursion on an inductive argument",
            Self::Measure => "Explicit numeric measure function",
            Self::Lexicographic => "Lexicographic ordering of multiple arguments",
            Self::WellFounded => "Custom well-founded relation",
            Self::Mutual => "Mutual recursion with shared termination argument",
        }
    }
    /// Return all available strategies.
    #[allow(dead_code)]
    pub fn all() -> &'static [TerminationStrategy] {
        &[
            Self::Structural,
            Self::Measure,
            Self::Lexicographic,
            Self::WellFounded,
            Self::Mutual,
        ]
    }
}
/// Proof-relevant termination certificates.
///
/// A `TerminationCert` pairs a measure function with evidence that the
/// measure strictly decreases on each recursive call.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TerminationCert {
    /// Name of the function being certified.
    pub fn_name: String,
    /// Name of the measure function (`measure : domain → Nat`).
    pub measure_fn: String,
    /// Human-readable justification.
    pub justification: String,
    /// Whether automated verification succeeded.
    pub verified: bool,
}
impl TerminationCert {
    /// Construct a certificate.
    #[allow(dead_code)]
    pub fn new(
        fn_name: impl Into<String>,
        measure_fn: impl Into<String>,
        justification: impl Into<String>,
        verified: bool,
    ) -> Self {
        Self {
            fn_name: fn_name.into(),
            measure_fn: measure_fn.into(),
            justification: justification.into(),
            verified,
        }
    }
    /// Returns a structural recursion certificate (always verified).
    #[allow(dead_code)]
    pub fn structural(fn_name: impl Into<String>) -> Self {
        let nm = fn_name.into();
        Self {
            measure_fn: format!("structural_measure_{}", nm),
            justification: "Structural recursion on an inductive argument".to_owned(),
            fn_name: nm,
            verified: true,
        }
    }
    /// Returns a well-founded recursion certificate.
    #[allow(dead_code)]
    pub fn well_founded(fn_name: impl Into<String>, measure_fn: impl Into<String>) -> Self {
        Self::new(fn_name, measure_fn, "Well-founded recursion", true)
    }
    /// Returns a certificate backed by lexicographic ordering.
    #[allow(dead_code)]
    pub fn lexicographic(fn_name: impl Into<String>, components: Vec<String>) -> Self {
        let nm = fn_name.into();
        let just = format!("Lexicographic order on ({}).", components.join(", "));
        Self::new(nm.clone(), format!("lex_measure_{}", nm), just, true)
    }
    /// Returns `true` if the certificate is valid.
    #[allow(dead_code)]
    pub fn is_valid(&self) -> bool {
        self.verified && !self.fn_name.is_empty() && !self.measure_fn.is_empty()
    }
    /// Produce a summary string.
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        format!(
            "TerminationCert{{ fn='{}', measure='{}', ok={}, justification='{}' }}",
            self.fn_name, self.measure_fn, self.verified, self.justification
        )
    }
}
/// A registry of termination certificates indexed by function name.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct TerminationRegistry {
    certs: Vec<TerminationCert>,
}
impl TerminationRegistry {
    /// Create an empty registry.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Register a certificate.
    #[allow(dead_code)]
    pub fn register(&mut self, cert: TerminationCert) {
        self.certs.push(cert);
    }
    /// Look up the certificate for a function.
    #[allow(dead_code)]
    pub fn lookup(&self, fn_name: &str) -> Option<&TerminationCert> {
        self.certs.iter().find(|c| c.fn_name == fn_name)
    }
    /// Return all verified certificates.
    #[allow(dead_code)]
    pub fn verified_certs(&self) -> Vec<&TerminationCert> {
        self.certs.iter().filter(|c| c.verified).collect()
    }
    /// Return the total number of registered certificates.
    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        self.certs.len()
    }
}
/// A combinator for building lexicographic well-founded orderings.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LexOrder {
    /// Component measure names in decreasing priority.
    pub components: Vec<String>,
}
impl LexOrder {
    /// Construct from a list of measure component names.
    #[allow(dead_code)]
    pub fn new(components: Vec<impl Into<String>>) -> Self {
        Self {
            components: components.into_iter().map(Into::into).collect(),
        }
    }
    /// Add an additional (lowest-priority) component.
    #[allow(dead_code)]
    pub fn push(mut self, component: impl Into<String>) -> Self {
        self.components.push(component.into());
        self
    }
    /// Return the number of components.
    #[allow(dead_code)]
    pub fn arity(&self) -> usize {
        self.components.len()
    }
    /// Describe the lex order as a tuple type string.
    #[allow(dead_code)]
    pub fn type_string(&self) -> String {
        format!("({})", self.components.join(" × "))
    }
    /// Verify that a concrete sequence of `(before, after)` pairs is
    /// lexicographically decreasing using the given values for each component.
    ///
    /// `values_before[i]` and `values_after[i]` are `u64` values of component `i`.
    #[allow(dead_code)]
    pub fn check_decrease(&self, values_before: &[u64], values_after: &[u64]) -> bool {
        let n = self
            .components
            .len()
            .min(values_before.len())
            .min(values_after.len());
        for i in 0..n {
            if values_before[i] < values_after[i] {
                return false;
            }
            if values_before[i] > values_after[i] {
                return true;
            }
        }
        false
    }
}
