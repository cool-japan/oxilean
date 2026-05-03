//! Types for formal epistemology: propositions, belief states, Kripke frames.

/// A proposition in epistemic / propositional logic.
#[derive(Debug, Clone, PartialEq)]
pub enum Proposition {
    /// Atomic proposition identified by a string name.
    Atomic(String),
    /// Negation.
    Not(Box<Proposition>),
    /// Conjunction.
    And(Box<Proposition>, Box<Proposition>),
    /// Disjunction.
    Or(Box<Proposition>, Box<Proposition>),
    /// Material implication.
    Implies(Box<Proposition>, Box<Proposition>),
    /// Biconditional (if and only if).
    Iff(Box<Proposition>, Box<Proposition>),
}

impl Proposition {
    /// Convenience constructor for atomic propositions.
    pub fn atom(s: impl Into<String>) -> Self {
        Proposition::Atomic(s.into())
    }

    /// Negate a proposition.
    pub fn not(self) -> Self {
        Proposition::Not(Box::new(self))
    }

    /// Conjoin with another proposition.
    pub fn and(self, other: Proposition) -> Self {
        Proposition::And(Box::new(self), Box::new(other))
    }

    /// Disjoin with another proposition.
    pub fn or(self, other: Proposition) -> Self {
        Proposition::Or(Box::new(self), Box::new(other))
    }

    /// Form material implication self → other.
    pub fn implies(self, other: Proposition) -> Self {
        Proposition::Implies(Box::new(self), Box::new(other))
    }

    /// Form biconditional self ↔ other.
    pub fn iff(self, other: Proposition) -> Self {
        Proposition::Iff(Box::new(self), Box::new(other))
    }
}

/// An agent's belief state: a set of believed propositions with associated confidence levels.
#[derive(Debug, Clone)]
pub struct BeliefState {
    /// The propositions held in the belief set.
    pub beliefs: Vec<Proposition>,
    /// Confidence values in \[0,1\] corresponding to each belief.
    pub confidence: Vec<f64>,
}

impl BeliefState {
    /// Create a new belief state.
    pub fn new(beliefs: Vec<Proposition>, confidence: Vec<f64>) -> Self {
        BeliefState {
            beliefs,
            confidence,
        }
    }

    /// Create an empty belief state.
    pub fn empty() -> Self {
        BeliefState {
            beliefs: vec![],
            confidence: vec![],
        }
    }

    /// Add a belief with a given confidence.
    pub fn add(&mut self, prop: Proposition, conf: f64) {
        self.beliefs.push(prop);
        self.confidence.push(conf);
    }
}

/// Epistemic operators used in epistemic logic formulae.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EpistemicOperator {
    /// Agent knows φ (K_i φ).
    Knows,
    /// Agent believes φ (B_i φ).
    Believes,
    /// Common knowledge among a group (C_G φ).
    CommonKnowledge,
    /// Distributed knowledge (D_G φ).
    Distributed,
}

/// AGM belief revision operators.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RevisionOperator {
    /// Expansion: add a proposition without removing anything.
    Expansion,
    /// Contraction: remove a proposition.
    Contraction,
    /// Revision: consistently incorporate a new proposition (Levi identity).
    Revision,
}

/// A Kripke frame for modal and epistemic logic.
///
/// Worlds are identified by their index into `worlds`.
/// `accessibility` encodes the accessibility relation as (from_world, to_world) pairs.
/// `valuation` records which atomic propositions are true at each world.
#[derive(Debug, Clone)]
pub struct KripkeFrame {
    /// Names of possible worlds (index = world id).
    pub worlds: Vec<String>,
    /// Accessibility relation: (w1, w2) means w2 is accessible from w1.
    pub accessibility: Vec<(usize, usize)>,
    /// Valuation: (world_idx, atom_name) means atom is true at that world.
    pub valuation: Vec<(usize, String)>,
}

impl KripkeFrame {
    /// Create a new Kripke frame.
    pub fn new(
        worlds: Vec<String>,
        accessibility: Vec<(usize, usize)>,
        valuation: Vec<(usize, String)>,
    ) -> Self {
        KripkeFrame {
            worlds,
            accessibility,
            valuation,
        }
    }

    /// Return the set of worlds accessible from `world`.
    pub fn accessible_from(&self, world: usize) -> Vec<usize> {
        self.accessibility
            .iter()
            .filter_map(|&(from, to)| if from == world { Some(to) } else { None })
            .collect()
    }

    /// Check if atomic proposition `atom` is true at `world`.
    pub fn atom_true_at(&self, world: usize, atom: &str) -> bool {
        self.valuation.iter().any(|(w, a)| *w == world && a == atom)
    }
}

/// An agent's epistemic attitude at a particular world.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AgentBelief {
    /// The agent's identifier.
    pub agent_id: usize,
    /// The world at which the attitude is evaluated.
    pub world: usize,
    /// The proposition the attitude is about.
    pub proposition: Proposition,
    /// The epistemic operator (knows, believes, etc.).
    pub operator: EpistemicOperator,
}

impl AgentBelief {
    #[allow(dead_code)]
    /// Create a new agent belief.
    pub fn new(
        agent_id: usize,
        world: usize,
        proposition: Proposition,
        operator: EpistemicOperator,
    ) -> Self {
        AgentBelief {
            agent_id,
            world,
            proposition,
            operator,
        }
    }
}
