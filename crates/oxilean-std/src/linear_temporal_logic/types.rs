//! Types for Linear Temporal Logic (LTL) model checking.

use std::collections::HashSet;

/// An LTL formula over atomic propositions (strings).
///
/// Supports the full classical LTL connectives:
/// - Boolean: True, False, Atom, Neg, And, Or, Implies
/// - Temporal: Next (X), Until (U), Release (R), Globally (G), Finally (F), WeakUntil (W)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LtlFormula {
    /// The logical constant true.
    True,
    /// The logical constant false.
    False,
    /// An atomic proposition by name.
    Atom(String),
    /// Negation: ¬φ
    Neg(Box<LtlFormula>),
    /// Conjunction: φ ∧ ψ
    And(Box<LtlFormula>, Box<LtlFormula>),
    /// Disjunction: φ ∨ ψ
    Or(Box<LtlFormula>, Box<LtlFormula>),
    /// Implication: φ → ψ
    Implies(Box<LtlFormula>, Box<LtlFormula>),
    /// Next: X φ — holds in the next step
    Next(Box<LtlFormula>),
    /// Until: φ U ψ — φ holds until ψ holds (ψ must eventually hold)
    Until(Box<LtlFormula>, Box<LtlFormula>),
    /// Release: φ R ψ — ψ holds until and including when φ holds; if φ never holds, ψ holds forever
    Release(Box<LtlFormula>, Box<LtlFormula>),
    /// Globally: G φ — φ holds at every future step
    Globally(Box<LtlFormula>),
    /// Finally: F φ — φ holds at some future step
    Finally(Box<LtlFormula>),
    /// Weak Until: φ W ψ — like Until but ψ need not hold if φ holds forever
    WeakUntil(Box<LtlFormula>, Box<LtlFormula>),
}

/// An infinite trace represented as a lasso (finite prefix + repeating cycle).
///
/// The trace is: prefix\[0\], prefix\[1\], ..., prefix[n-1], cycle\[0\], cycle\[1\], ..., cycle[m-1], cycle\[0\], ...
/// `loop_start` records the index into `states` where the loop begins.
#[derive(Debug, Clone)]
pub struct LtlTrace {
    /// All states in the trace (prefix + one unrolled cycle).
    pub states: Vec<HashSet<String>>,
    /// Index into `states` where the cycle begins (None = finite, no loop).
    pub loop_start: Option<usize>,
}

impl LtlTrace {
    /// Construct a lasso trace from a prefix and a non-empty cycle.
    pub fn new_lasso(prefix: Vec<HashSet<String>>, cycle: Vec<HashSet<String>>) -> Self {
        let loop_start = prefix.len();
        let mut states = prefix;
        states.extend(cycle);
        LtlTrace {
            states,
            loop_start: Some(loop_start),
        }
    }

    /// Construct a finite trace (no loop).
    pub fn new_finite(states: Vec<HashSet<String>>) -> Self {
        LtlTrace {
            states,
            loop_start: None,
        }
    }

    /// Length of the trace (prefix + one copy of cycle).
    pub fn len(&self) -> usize {
        self.states.len()
    }

    /// Whether the trace has no states.
    pub fn is_empty(&self) -> bool {
        self.states.is_empty()
    }

    /// Get the state at logical position `pos` (wraps around the cycle if looped).
    pub fn state_at(&self, pos: usize) -> Option<&HashSet<String>> {
        if pos < self.states.len() {
            Some(&self.states[pos])
        } else if let Some(lstart) = self.loop_start {
            let cycle_len = self.states.len() - lstart;
            if cycle_len == 0 {
                None
            } else {
                let offset = (pos - lstart) % cycle_len;
                self.states.get(lstart + offset)
            }
        } else {
            None
        }
    }
}

/// A Kripke structure (transition system) used as the model.
#[derive(Debug, Clone)]
pub struct LtlModel {
    /// Names for each state (index = state id).
    pub states: Vec<String>,
    /// Directed transitions as (from, to) pairs (state indices).
    pub transitions: Vec<(usize, usize)>,
    /// Atomic propositions true in each state.
    pub labels: Vec<HashSet<String>>,
    /// Initial states (by index).
    pub initial: Vec<usize>,
}

impl LtlModel {
    /// Create an empty model.
    pub fn new() -> Self {
        LtlModel {
            states: Vec::new(),
            transitions: Vec::new(),
            labels: Vec::new(),
            initial: Vec::new(),
        }
    }

    /// Add a state with a given name and atomic propositions.
    pub fn add_state(&mut self, name: impl Into<String>, props: HashSet<String>) -> usize {
        let id = self.states.len();
        self.states.push(name.into());
        self.labels.push(props);
        id
    }

    /// Add a transition.
    pub fn add_transition(&mut self, from: usize, to: usize) {
        self.transitions.push((from, to));
    }

    /// Mark a state as initial.
    pub fn set_initial(&mut self, state: usize) {
        if !self.initial.contains(&state) {
            self.initial.push(state);
        }
    }

    /// Successors of a given state.
    pub fn successors(&self, state: usize) -> Vec<usize> {
        self.transitions
            .iter()
            .filter_map(|&(from, to)| if from == state { Some(to) } else { None })
            .collect()
    }
}

impl Default for LtlModel {
    fn default() -> Self {
        Self::new()
    }
}

/// A Büchi automaton for LTL model checking.
#[derive(Debug, Clone)]
pub struct BuchiAutomaton {
    /// Names for each state.
    pub states: Vec<String>,
    /// Transitions as (from, label, to); label is an atomic proposition or "".
    pub transitions: Vec<(usize, String, usize)>,
    /// Initial states.
    pub initial: Vec<usize>,
    /// Accepting (final) states — must be visited infinitely often.
    pub accepting: Vec<usize>,
}

impl BuchiAutomaton {
    /// Create an empty Büchi automaton.
    pub fn new() -> Self {
        BuchiAutomaton {
            states: Vec::new(),
            transitions: Vec::new(),
            initial: Vec::new(),
            accepting: Vec::new(),
        }
    }
}

impl Default for BuchiAutomaton {
    fn default() -> Self {
        Self::new()
    }
}

/// The result of LTL model checking.
#[derive(Debug, Clone)]
pub enum ModelCheckResult {
    /// The formula is satisfied by the model.
    Satisfied,
    /// The formula is violated; a counterexample trace is provided.
    Violated { counterexample: LtlTrace },
    /// The result could not be determined (e.g., infinite-state model).
    Unknown,
}
