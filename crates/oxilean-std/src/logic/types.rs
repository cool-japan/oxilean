//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// Represents a proof system and its complexity measures.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProofComplexitySystem {
    /// Name of the proof system.
    pub name: String,
    /// Whether the system has polynomial-size proofs for tautologies (p-simulation).
    pub has_p_simulations: bool,
    /// Whether the system can simulate resolution.
    pub simulates_resolution: bool,
    /// The Cook-Reckhow conjectured lower bound on proof size.
    pub super_polynomial_lower_bound: bool,
}
#[allow(dead_code)]
impl ProofComplexitySystem {
    /// Create a new proof complexity system descriptor.
    pub fn new(name: &str) -> Self {
        ProofComplexitySystem {
            name: name.to_string(),
            has_p_simulations: false,
            simulates_resolution: false,
            super_polynomial_lower_bound: false,
        }
    }
    /// Resolution proof system.
    pub fn resolution() -> Self {
        let mut sys = ProofComplexitySystem::new("Resolution");
        sys.simulates_resolution = true;
        sys
    }
    /// Extended Frege system.
    pub fn extended_frege() -> Self {
        let mut sys = ProofComplexitySystem::new("Extended Frege");
        sys.has_p_simulations = true;
        sys.simulates_resolution = true;
        sys
    }
    /// Frege system (Hilbert-style).
    pub fn frege() -> Self {
        let mut sys = ProofComplexitySystem::new("Frege");
        sys.simulates_resolution = true;
        sys
    }
    /// Cook-Reckhow theorem: if NP ≠ coNP, no proof system is polynomially bounded.
    pub fn cook_reckhow_separation(&self) -> bool {
        true
    }
    /// Propositional pigeonhole principle: known to require exponential proofs in Resolution.
    pub fn php_complexity(&self) -> String {
        match self.name.as_str() {
            "Resolution" => "Exponential lower bound (Ben-Sasson-Wigderson 2001)".to_string(),
            "Extended Frege" => "Polynomial upper bound".to_string(),
            _ => "Unknown".to_string(),
        }
    }
}
/// A truth value in a many-valued logic.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ManyValuedTruth {
    /// True.
    True,
    /// False.
    False,
    /// Both true and false (glutted).
    Both,
    /// Neither true nor false (gapped).
    Neither,
    /// A numerical value in \[0, 1\] (fuzzy).
    Fuzzy(f64),
}
#[allow(dead_code)]
impl ManyValuedTruth {
    /// Is this value "designated" (true-like) in LP/FDE.
    pub fn is_designated(&self) -> bool {
        matches!(self, ManyValuedTruth::True | ManyValuedTruth::Both)
    }
    /// Kleene three-valued conjunction: A ∧ B.
    pub fn kleene_and(a: &ManyValuedTruth, b: &ManyValuedTruth) -> ManyValuedTruth {
        use ManyValuedTruth::*;
        match (a, b) {
            (True, True) => True,
            (False, _) | (_, False) => False,
            (Neither, _) | (_, Neither) => Neither,
            _ => Both,
        }
    }
    /// Kleene three-valued disjunction: A ∨ B.
    pub fn kleene_or(a: &ManyValuedTruth, b: &ManyValuedTruth) -> ManyValuedTruth {
        use ManyValuedTruth::*;
        match (a, b) {
            (False, False) => False,
            (True, _) | (_, True) => True,
            (Neither, _) | (_, Neither) => Neither,
            _ => Both,
        }
    }
    /// Kleene three-valued negation: ¬A.
    pub fn kleene_not(a: &ManyValuedTruth) -> ManyValuedTruth {
        use ManyValuedTruth::*;
        match a {
            True => False,
            False => True,
            Neither => Neither,
            Both => Both,
            Fuzzy(v) => Fuzzy(1.0 - v),
        }
    }
}
/// Describes a second-order logic system.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SecondOrderLogic {
    /// Whether comprehension axiom is full or restricted.
    pub full_comprehension: bool,
    /// Whether the system includes induction.
    pub with_induction: bool,
    /// The second-order system name.
    pub system_name: String,
}
#[allow(dead_code)]
impl SecondOrderLogic {
    /// Create a new second-order logic descriptor.
    pub fn new(system_name: &str, full_comprehension: bool, with_induction: bool) -> Self {
        SecondOrderLogic {
            full_comprehension,
            with_induction,
            system_name: system_name.to_string(),
        }
    }
    /// Second-order arithmetic Z_2 (full second-order arithmetic).
    pub fn z2() -> Self {
        SecondOrderLogic::new("Z_2", true, true)
    }
    /// ACA_0: Arithmetical Comprehension Axiom (base system of second-order arithmetic).
    pub fn aca0() -> Self {
        SecondOrderLogic::new("ACA_0", false, true)
    }
    /// Whether this system interprets first-order PA (Peano Arithmetic).
    pub fn interprets_pa(&self) -> bool {
        self.with_induction && !self.full_comprehension || self.full_comprehension
    }
    /// Categoricity: second-order PA is categorical (all models are isomorphic).
    pub fn is_categorical(&self) -> bool {
        self.full_comprehension
    }
    /// Incompleteness: by Gödel, Z_2 is incomplete if ω-consistent.
    pub fn is_complete(&self) -> bool {
        false
    }
}
/// Sequent calculus proof tree.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SequentProof {
    pub antecedents: Vec<String>,
    pub consequent: String,
    pub rule_name: String,
    pub premises: Vec<SequentProof>,
}
#[allow(dead_code)]
impl SequentProof {
    pub fn axiom(formula: &str) -> Self {
        SequentProof {
            antecedents: vec![formula.to_string()],
            consequent: formula.to_string(),
            rule_name: "Ax".to_string(),
            premises: Vec::new(),
        }
    }
    pub fn new(ants: Vec<&str>, cons: &str, rule: &str, prems: Vec<SequentProof>) -> Self {
        SequentProof {
            antecedents: ants.iter().map(|s| s.to_string()).collect(),
            consequent: cons.to_string(),
            rule_name: rule.to_string(),
            premises: prems,
        }
    }
    pub fn height(&self) -> usize {
        if self.premises.is_empty() {
            0
        } else {
            1 + self.premises.iter().map(|p| p.height()).max().unwrap_or(0)
        }
    }
    pub fn n_leaves(&self) -> usize {
        if self.premises.is_empty() {
            1
        } else {
            self.premises.iter().map(|p| p.n_leaves()).sum()
        }
    }
}
/// Linear temporal logic (LTL) formula.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LTLFormula {
    Atom(String),
    Not(Box<LTLFormula>),
    And(Box<LTLFormula>, Box<LTLFormula>),
    Or(Box<LTLFormula>, Box<LTLFormula>),
    Next(Box<LTLFormula>),
    Globally(Box<LTLFormula>),
    Finally(Box<LTLFormula>),
    Until(Box<LTLFormula>, Box<LTLFormula>),
}
#[allow(dead_code)]
impl LTLFormula {
    pub fn atom(s: &str) -> Self {
        LTLFormula::Atom(s.to_string())
    }
    pub fn safety(phi: LTLFormula) -> Self {
        LTLFormula::Globally(Box::new(phi))
    }
    pub fn liveness(phi: LTLFormula) -> Self {
        LTLFormula::Finally(Box::new(phi))
    }
    pub fn is_temporal(&self) -> bool {
        matches!(
            self,
            LTLFormula::Next(_)
                | LTLFormula::Globally(_)
                | LTLFormula::Finally(_)
                | LTLFormula::Until(_, _)
        )
    }
    pub fn depth(&self) -> usize {
        match self {
            LTLFormula::Atom(_) => 0,
            LTLFormula::Not(f) => 1 + f.depth(),
            LTLFormula::And(f, g) | LTLFormula::Or(f, g) | LTLFormula::Until(f, g) => {
                1 + f.depth().max(g.depth())
            }
            LTLFormula::Next(f) | LTLFormula::Globally(f) | LTLFormula::Finally(f) => 1 + f.depth(),
        }
    }
}
/// Modal logic Kripke frame.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KripkeFrame {
    pub worlds: Vec<String>,
    pub accessibility: Vec<(usize, usize)>,
}
#[allow(dead_code)]
impl KripkeFrame {
    pub fn new(worlds: Vec<&str>) -> Self {
        KripkeFrame {
            worlds: worlds.iter().map(|s| s.to_string()).collect(),
            accessibility: Vec::new(),
        }
    }
    pub fn add_access(&mut self, w: usize, v: usize) {
        self.accessibility.push((w, v));
    }
    pub fn is_reflexive(&self) -> bool {
        (0..self.worlds.len()).all(|w| self.accessibility.contains(&(w, w)))
    }
    pub fn is_transitive(&self) -> bool {
        for &(w, v) in &self.accessibility {
            for &(v2, u) in &self.accessibility {
                if v == v2 && !self.accessibility.contains(&(w, u)) {
                    return false;
                }
            }
        }
        true
    }
    pub fn is_symmetric(&self) -> bool {
        self.accessibility
            .iter()
            .all(|&(w, v)| self.accessibility.contains(&(v, w)))
    }
    pub fn modal_logic_name(&self) -> &'static str {
        match (
            self.is_reflexive(),
            self.is_transitive(),
            self.is_symmetric(),
        ) {
            (true, true, true) => "S5 (equivalence relation)",
            (true, true, false) => "S4",
            (true, false, false) => "T",
            (false, true, false) => "K4",
            _ => "K (basic modal logic)",
        }
    }
}
/// Represents a paraconsistent logic: a logic where contradictions do not
/// cause explosion (ex falso quodlibet fails).
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ParaconsistentSystem {
    /// LP (Logic of Paradox): allows true contradictions (dialetheism).
    LP,
    /// FDE (First Degree Entailment): four-valued logic.
    FDE,
    /// Relevant logic R.
    RelevantR,
    /// Priest's LP with modus ponens.
    PriestLP,
    /// Jaskowski's discussive logic D2.
    JaskowskiD2,
}
#[allow(dead_code)]
impl ParaconsistentSystem {
    /// Returns the number of truth values in the system.
    pub fn num_truth_values(&self) -> usize {
        match self {
            ParaconsistentSystem::LP => 3,
            ParaconsistentSystem::FDE => 4,
            ParaconsistentSystem::RelevantR => 2,
            ParaconsistentSystem::PriestLP => 3,
            ParaconsistentSystem::JaskowskiD2 => 2,
        }
    }
    /// Check if the law of explosion (⊢ A → (¬A → B)) holds.
    pub fn explosion_holds(&self) -> bool {
        match self {
            ParaconsistentSystem::LP => false,
            ParaconsistentSystem::FDE => false,
            ParaconsistentSystem::RelevantR => false,
            ParaconsistentSystem::PriestLP => false,
            ParaconsistentSystem::JaskowskiD2 => false,
        }
    }
    /// Whether this system handles the liar paradox consistently.
    pub fn handles_liar_paradox(&self) -> bool {
        match self {
            ParaconsistentSystem::LP => true,
            ParaconsistentSystem::FDE => true,
            _ => false,
        }
    }
}
