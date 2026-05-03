//! First-order model theory: signatures, structures, terms, formulas, and evaluation types.
//!
//! These types provide a concrete computational representation of first-order logic,
//! suitable for model checking, satisfiability testing, and elementary embedding verification.

use std::collections::HashMap;

/// A first-order signature: function symbols (with arity), relation symbols (with arity),
/// and constant symbols.
///
/// The arity of a function symbol f is the number of arguments it takes.
/// Constants are 0-ary function symbols but are listed separately for clarity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoSignature {
    /// Function symbols with their arities: (name, arity).
    pub functions: Vec<(String, usize)>,
    /// Relation symbols with their arities: (name, arity).
    pub relations: Vec<(String, usize)>,
    /// Constant symbols (0-ary functions).
    pub constants: Vec<String>,
}

impl FoSignature {
    /// Construct an empty signature.
    pub fn new() -> Self {
        FoSignature {
            functions: Vec::new(),
            relations: Vec::new(),
            constants: Vec::new(),
        }
    }

    /// Add a function symbol with the given arity.
    pub fn add_function(&mut self, name: &str, arity: usize) {
        self.functions.push((name.to_string(), arity));
    }

    /// Add a relation symbol with the given arity.
    pub fn add_relation(&mut self, name: &str, arity: usize) {
        self.relations.push((name.to_string(), arity));
    }

    /// Add a constant symbol.
    pub fn add_constant(&mut self, name: &str) {
        self.constants.push(name.to_string());
    }

    /// Look up the arity of a function symbol, returning None if not found.
    pub fn function_arity(&self, name: &str) -> Option<usize> {
        self.functions
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, a)| *a)
    }

    /// Look up the arity of a relation symbol, returning None if not found.
    pub fn relation_arity(&self, name: &str) -> Option<usize> {
        self.relations
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, a)| *a)
    }
}

impl Default for FoSignature {
    fn default() -> Self {
        Self::new()
    }
}

/// Interpretation of a single symbol in a first-order structure.
///
/// - `Function`: maps argument tuples to result domain elements.
/// - `Relation`: lists which tuples of domain elements satisfy the relation.
/// - `Constant`: a single domain element.
#[derive(Debug, Clone)]
pub enum StructureInterp {
    /// Function interpretation: list of (argument tuple, result) pairs.
    Function(Vec<(Vec<String>, String)>),
    /// Relation interpretation: list of tuples in the relation.
    Relation(Vec<Vec<String>>),
    /// Constant interpretation: single domain element.
    Constant(String),
}

/// A first-order structure (model): a domain plus interpretations of all symbols.
///
/// A structure M = (|M|, σ^M) consists of:
/// - A non-empty domain |M| (the "universe").
/// - For each symbol s in the signature σ, an interpretation s^M.
#[derive(Debug, Clone)]
pub struct FoStructure {
    /// The domain (universe) — a list of element names.
    pub domain: Vec<String>,
    /// Symbol interpretations: symbol name → interpretation.
    pub interpretations: HashMap<String, StructureInterp>,
}

impl FoStructure {
    /// Construct a new structure with the given domain.
    pub fn new(domain: Vec<String>) -> Self {
        FoStructure {
            domain,
            interpretations: HashMap::new(),
        }
    }

    /// Add an interpretation for a symbol.
    pub fn add_interp(&mut self, symbol: &str, interp: StructureInterp) {
        self.interpretations.insert(symbol.to_string(), interp);
    }

    /// Look up the interpretation of a symbol.
    pub fn interp(&self, symbol: &str) -> Option<&StructureInterp> {
        self.interpretations.get(symbol)
    }

    /// Check if an element is in the domain.
    pub fn contains(&self, elem: &str) -> bool {
        self.domain.iter().any(|e| e == elem)
    }

    /// Size of the domain.
    pub fn domain_size(&self) -> usize {
        self.domain.len()
    }
}

/// A first-order term (over a signature σ).
///
/// Terms are built from:
/// - Variables `x, y, z, …`
/// - Constants (0-ary function symbols) `c, d, …`
/// - Function applications `f(t_1, …, t_n)` for n-ary function f
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    /// A variable symbol.
    Var(String),
    /// A constant symbol.
    Const(String),
    /// A function application: function name and argument terms.
    App(String, Vec<Term>),
}

impl Term {
    /// Collect all free variable names appearing in this term.
    pub fn free_vars(&self) -> Vec<String> {
        match self {
            Term::Var(x) => vec![x.clone()],
            Term::Const(_) => vec![],
            Term::App(_, args) => {
                let mut vars = Vec::new();
                for arg in args {
                    for v in arg.free_vars() {
                        if !vars.contains(&v) {
                            vars.push(v);
                        }
                    }
                }
                vars
            }
        }
    }
}

/// A first-order formula (over a signature σ).
///
/// Formulas are built from:
/// - Atomic formulas: relation applications and equality
/// - Propositional connectives: ¬, ∧, ∨, →
/// - Quantifiers: ∀x, ∃x
#[derive(Debug, Clone, PartialEq)]
pub enum Formula {
    /// Atomic formula: relation name applied to terms.
    Atom(String, Vec<Term>),
    /// Equality of two terms.
    Equal(Term, Term),
    /// Negation ¬φ.
    Neg(Box<Formula>),
    /// Conjunction φ ∧ ψ.
    And(Box<Formula>, Box<Formula>),
    /// Disjunction φ ∨ ψ.
    Or(Box<Formula>, Box<Formula>),
    /// Implication φ → ψ.
    Implies(Box<Formula>, Box<Formula>),
    /// Universal quantification ∀x. φ.
    Forall(String, Box<Formula>),
    /// Existential quantification ∃x. φ.
    Exists(String, Box<Formula>),
}

impl Formula {
    /// Compute the set of free variables in this formula.
    pub fn free_vars(&self) -> Vec<String> {
        match self {
            Formula::Atom(_, terms) => {
                let mut vars = Vec::new();
                for t in terms {
                    for v in t.free_vars() {
                        if !vars.contains(&v) {
                            vars.push(v);
                        }
                    }
                }
                vars
            }
            Formula::Equal(t1, t2) => {
                let mut vars = t1.free_vars();
                for v in t2.free_vars() {
                    if !vars.contains(&v) {
                        vars.push(v);
                    }
                }
                vars
            }
            Formula::Neg(f) => f.free_vars(),
            Formula::And(f1, f2) | Formula::Or(f1, f2) | Formula::Implies(f1, f2) => {
                let mut vars = f1.free_vars();
                for v in f2.free_vars() {
                    if !vars.contains(&v) {
                        vars.push(v);
                    }
                }
                vars
            }
            Formula::Forall(x, f) | Formula::Exists(x, f) => {
                let mut vars = f.free_vars();
                vars.retain(|v| v != x);
                vars
            }
        }
    }

    /// Returns true if the formula is a sentence (no free variables).
    pub fn is_sentence(&self) -> bool {
        self.free_vars().is_empty()
    }

    /// Construct ¬φ.
    pub fn neg(phi: Formula) -> Formula {
        Formula::Neg(Box::new(phi))
    }

    /// Construct φ ∧ ψ.
    pub fn and(phi: Formula, psi: Formula) -> Formula {
        Formula::And(Box::new(phi), Box::new(psi))
    }

    /// Construct φ ∨ ψ.
    pub fn or(phi: Formula, psi: Formula) -> Formula {
        Formula::Or(Box::new(phi), Box::new(psi))
    }

    /// Construct φ → ψ.
    pub fn implies(phi: Formula, psi: Formula) -> Formula {
        Formula::Implies(Box::new(phi), Box::new(psi))
    }

    /// Construct ∀x. φ.
    pub fn forall(x: &str, phi: Formula) -> Formula {
        Formula::Forall(x.to_string(), Box::new(phi))
    }

    /// Construct ∃x. φ.
    pub fn exists(x: &str, phi: Formula) -> Formula {
        Formula::Exists(x.to_string(), Box::new(phi))
    }
}

/// A variable assignment: maps variable names to domain elements (strings).
///
/// Used during formula evaluation to track the current binding of each variable.
#[derive(Debug, Clone, Default)]
pub struct FoAssignment {
    /// The variable-to-domain-element mapping.
    pub map: HashMap<String, String>,
}

impl FoAssignment {
    /// Construct an empty assignment.
    pub fn new() -> Self {
        FoAssignment {
            map: HashMap::new(),
        }
    }

    /// Extend the assignment by binding `var` to `val`, returning a new assignment.
    pub fn extend(&self, var: &str, val: &str) -> Self {
        let mut new_map = self.map.clone();
        new_map.insert(var.to_string(), val.to_string());
        FoAssignment { map: new_map }
    }

    /// Look up the domain element bound to `var`.
    pub fn get(&self, var: &str) -> Option<&String> {
        self.map.get(var)
    }

    /// Set a binding directly (mutating).
    pub fn set(&mut self, var: &str, val: &str) {
        self.map.insert(var.to_string(), val.to_string());
    }
}

/// A first-order theory: a set of axioms (sentences) over a signature.
///
/// A structure M is a model of theory T if M satisfies all axioms of T.
#[derive(Debug, Clone)]
pub struct FoTheory {
    /// The axioms of the theory (should all be sentences).
    pub axioms: Vec<Formula>,
    /// The signature of the theory.
    pub signature: FoSignature,
}

impl FoTheory {
    /// Construct a theory from axioms and signature.
    pub fn new(axioms: Vec<Formula>, signature: FoSignature) -> Self {
        FoTheory { axioms, signature }
    }

    /// Add an axiom to the theory.
    pub fn add_axiom(&mut self, axiom: Formula) {
        self.axioms.push(axiom);
    }

    /// Number of axioms.
    pub fn num_axioms(&self) -> usize {
        self.axioms.len()
    }
}

/// An elementary embedding between two structures (indexed by position in a list).
///
/// An embedding j: M → N is elementary if for every formula φ and assignment a in M,
/// M ⊨ φ\[a\] iff N ⊨ φ[j(a)].
#[derive(Debug, Clone)]
pub struct ElementaryEmbedding {
    /// Index of the source structure (in some ambient list).
    pub source: usize,
    /// Index of the target structure (in some ambient list).
    pub target: usize,
}

impl ElementaryEmbedding {
    /// Construct an elementary embedding by indices.
    pub fn new(source: usize, target: usize) -> Self {
        ElementaryEmbedding { source, target }
    }
}

/// An ultrafilter on a finite index set I = {0, 1, …, index_set-1}.
///
/// An ultrafilter U on I is a family of "large" subsets of I satisfying:
/// 1. I ∈ U (the whole set is large)
/// 2. ∅ ∉ U (the empty set is not large)
/// 3. If A ∈ U and A ⊆ B, then B ∈ U (upward closed)
/// 4. For every A ⊆ I, exactly one of A, I\A is in U (ultrafilter property)
///
/// Ultrafilters are used in the ultraproduct construction (Łoś's theorem).
#[derive(Debug, Clone)]
pub struct FoUltrafilter {
    /// The "large" subsets of the index set.
    pub sets: Vec<Vec<usize>>,
    /// Size of the index set I.
    pub index_set: usize,
}

impl FoUltrafilter {
    /// Construct an ultrafilter on an index set of the given size.
    pub fn new(index_set: usize, sets: Vec<Vec<usize>>) -> Self {
        FoUltrafilter { sets, index_set }
    }

    /// Check if a given subset is "large" (belongs to the ultrafilter).
    pub fn is_large(&self, subset: &[usize]) -> bool {
        self.sets
            .iter()
            .any(|s| s.len() == subset.len() && subset.iter().all(|x| s.contains(x)))
    }

    /// The principal ultrafilter at point i: U = { A ⊆ I : i ∈ A }.
    pub fn principal(index_set: usize, point: usize) -> Self {
        let sets: Vec<Vec<usize>> = if point < index_set {
            // All subsets containing `point` (represented by generating family).
            (0..index_set)
                .filter(|&j| j == point)
                .map(|j| vec![j])
                .collect()
        } else {
            Vec::new()
        };
        FoUltrafilter { sets, index_set }
    }
}
