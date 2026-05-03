//! Type definitions for bvdecide

use super::super::functions::*;
use oxilean_kernel::Expr;
use std::collections::{HashMap, HashSet, VecDeque};

/// The main CDCL SAT solver.
pub struct CdclSolver {
    /// The clause database.
    pub(crate) clause_db: ClauseDb,
    /// Current assignment.
    pub(crate) assignment: Assignment,
    /// VSIDS branching heuristic.
    pub(crate) vsids: VsidsScorer,
    /// Watch lists: for each literal, list of clause indices watching it.
    pub(crate) watch_lists: HashMap<i32, Vec<usize>>,
    /// Watched literal info per clause.
    pub(crate) watched: HashMap<usize, WatchedInfo>,
    /// Number of variables.
    pub(crate) num_vars: usize,
    /// Statistics.
    pub(crate) stats: CdclStats,
    /// Configuration.
    pub(crate) config: CdclConfig,
    /// Resolution chain for UNSAT proof.
    pub(crate) proof_chain: Vec<ResolutionStep>,
}

/// Configuration for the `bv_decide` tactic.
#[derive(Clone, Debug)]
pub struct BvDecideConfig {
    /// Maximum number of SAT variables before aborting.
    pub max_vars: u32,
    /// Maximum number of clauses before aborting.
    pub max_clauses: u64,
    /// Timeout in milliseconds (0 = no timeout).
    pub timeout_ms: u64,
    /// Enable preprocessing (constant propagation, simplification).
    pub preprocessing: bool,
    /// Enable CDCL (vs. basic DPLL without learning).
    pub enable_cdcl: bool,
    /// VSIDS decay factor.
    pub vsids_decay: f64,
    /// Maximum conflicts before restart.
    pub restart_limit: u64,
    /// Verbose output (for debugging).
    pub verbose: bool,
}

/// Analyzes kernel `Expr` goals and extracts BV operations.
pub struct GoalAnalyzer {
    /// Map from kernel FVar IDs to BV variable names and widths.
    pub(crate) var_map: HashMap<u64, (String, BitWidth)>,
    /// Next variable name counter.
    pub(crate) next_var: u32,
    /// Known bit-vector type widths by name.
    pub(crate) known_bv_types: HashMap<String, BitWidth>,
}

/// Tracks variable assignments, decision levels, and the trail.
#[derive(Clone, Debug)]
pub struct Assignment {
    /// Value assigned to each variable: None = unassigned.
    pub(crate) values: Vec<Option<bool>>,
    /// Decision level at which each variable was assigned.
    pub(crate) levels: Vec<Option<u32>>,
    /// The reason clause for each propagated assignment (None for decisions).
    pub(crate) reasons: Vec<Option<usize>>,
    /// Trail: sequence of assigned literals in chronological order.
    pub(crate) trail: Vec<Literal>,
    /// Indices into trail marking the start of each decision level.
    pub(crate) trail_lim: Vec<usize>,
    /// Current decision level.
    pub(crate) current_level: u32,
    /// Number of variables.
    pub(crate) num_vars: usize,
}

/// Result of a SAT solver run.
#[derive(Clone, Debug)]
pub enum SatResult {
    /// The formula is satisfiable; `Model` maps variables to their values.
    Sat(Model),
    /// The formula is unsatisfiable; includes a proof (resolution chain).
    Unsat(UnsatProof),
    /// Could not determine (resource limit reached).
    Unknown(String),
}

/// Encoder that converts bit-vector expressions to CNF using Tseitin transformation.
#[derive(Default)]
pub struct BvEncoder {
    /// The CNF formula being constructed.
    pub(crate) formula: CnfFormula,
    /// Cache from BvExpr to the SAT variables representing its bits.
    pub(crate) expr_cache: HashMap<u64, Vec<SatVar>>,
    /// Next expression ID for caching.
    pub(crate) next_expr_id: u64,
    /// Named BV variable map: variable name -> SAT variable bits.
    pub(crate) named_vars: HashMap<String, Vec<SatVar>>,
}

/// Statistics for the CDCL solver.
#[derive(Clone, Debug, Default)]
pub struct CdclStats {
    /// Number of decisions made.
    pub decisions: u64,
    /// Number of propagations.
    pub propagations: u64,
    /// Number of conflicts.
    pub conflicts: u64,
    /// Number of learned clauses.
    pub learned_clauses: u64,
    /// Number of restarts.
    pub restarts: u64,
    /// Number of clause deletions.
    pub deletions: u64,
}

/// A satisfying assignment.
#[derive(Clone, Debug)]
pub struct Model {
    /// Value of each variable.
    pub values: Vec<bool>,
}

/// A bit-vector expression AST for analysis.
#[derive(Clone, Debug)]
pub enum BvExpr {
    /// Named variable with width.
    Var(String, BitWidth),
    /// Constant value.
    Const(BitVec),
    /// Addition.
    Add(Box<BvExpr>, Box<BvExpr>),
    /// Subtraction.
    Sub(Box<BvExpr>, Box<BvExpr>),
    /// Multiplication.
    Mul(Box<BvExpr>, Box<BvExpr>),
    /// Bitwise AND.
    And(Box<BvExpr>, Box<BvExpr>),
    /// Bitwise OR.
    Or(Box<BvExpr>, Box<BvExpr>),
    /// Bitwise XOR.
    Xor(Box<BvExpr>, Box<BvExpr>),
    /// Bitwise NOT.
    Not(Box<BvExpr>),
    /// Left shift.
    Shl(Box<BvExpr>, Box<BvExpr>),
    /// Right shift.
    Shr(Box<BvExpr>, Box<BvExpr>),
    /// Equality comparison (returns 1-bit BV).
    Eq(Box<BvExpr>, Box<BvExpr>),
    /// Unsigned less-than (returns 1-bit BV).
    Ult(Box<BvExpr>, Box<BvExpr>),
    /// Signed less-than (returns 1-bit BV).
    Slt(Box<BvExpr>, Box<BvExpr>),
    /// Bit extraction \[high:low\].
    Extract(Box<BvExpr>, u32, u32),
    /// Concatenation (high ++ low).
    Concat(Box<BvExpr>, Box<BvExpr>),
    /// If-then-else (condition is 1-bit).
    Ite(Box<BvExpr>, Box<BvExpr>, Box<BvExpr>),
}

/// Statistics gathered during a `bv_decide` run.
#[derive(Clone, Debug, Default)]
pub struct BvDecideStats {
    /// Number of SAT variables created.
    pub vars: u32,
    /// Number of CNF clauses created.
    pub clauses: u64,
    /// Number of decisions in the SAT solver.
    pub decisions: u64,
    /// Number of unit propagations.
    pub propagations: u64,
    /// Number of conflicts encountered.
    pub conflicts: u64,
    /// Number of clauses learned via CDCL.
    pub learned_clauses: u64,
    /// Number of restarts.
    pub restarts: u64,
    /// Total time in milliseconds.
    pub time_ms: u64,
    /// Number of BV expression nodes analyzed.
    pub bv_nodes: usize,
    /// Encoding phase time in milliseconds.
    pub encoding_time_ms: u64,
    /// Solving phase time in milliseconds.
    pub solving_time_ms: u64,
}

/// Clause database with add, remove, and garbage collection.
#[derive(Clone, Debug)]
pub struct ClauseDb {
    /// All clauses stored by index.
    pub(crate) clauses: Vec<Option<Clause>>,
    /// Indices of original (non-learned) clauses.
    pub(crate) original_indices: Vec<usize>,
    /// Indices of learned clauses.
    pub(crate) learned_indices: Vec<usize>,
    /// Number of active clauses.
    pub(crate) num_active: usize,
    /// Clause activity bump amount.
    pub(crate) clause_bump: f64,
    /// Clause activity decay factor.
    pub(crate) clause_decay: f64,
}

/// An unsatisfiability proof (resolution chain).
#[derive(Clone, Debug)]
pub struct UnsatProof {
    /// Chain of resolution steps leading to the empty clause.
    pub resolution_chain: Vec<ResolutionStep>,
}

/// A single resolution step in an unsatisfiability proof.
#[derive(Clone, Debug)]
pub struct ResolutionStep {
    /// Index of the first clause.
    pub clause_a: usize,
    /// Index of the second clause.
    pub clause_b: usize,
    /// Variable resolved upon.
    pub pivot: SatVar,
    /// The resulting resolvent clause index.
    pub result: usize,
}

/// A clause is a disjunction of literals.
#[derive(Clone, Debug)]
pub struct Clause {
    /// The literals in this clause.
    pub lits: Vec<Literal>,
    /// Whether this clause was learned during CDCL.
    pub learned: bool,
    /// Activity score for clause deletion heuristics.
    pub activity: f64,
    /// LBD (Literal Block Distance) for learned clause quality.
    pub lbd: u32,
}

/// Watched-literal data for a clause.
#[derive(Clone, Debug)]
pub(crate) struct WatchedInfo {
    /// Index of the first watched literal in the clause.
    pub(crate) watch1: usize,
    /// Index of the second watched literal in the clause.
    pub(crate) watch2: usize,
}

/// A literal is a variable with a polarity (positive or negative).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Literal {
    /// The underlying variable.
    pub var: SatVar,
    /// If true, the literal is positive (x); if false, negative (not x).
    pub polarity: bool,
}

/// Configuration for the CDCL solver.
#[derive(Clone, Debug)]
pub struct CdclConfig {
    /// Maximum number of conflicts before giving up.
    pub max_conflicts: u64,
    /// Restart interval (Luby sequence base).
    pub restart_base: u64,
    /// Clause database GC interval (number of conflicts).
    pub gc_interval: u64,
    /// Fraction of learned clauses to keep during GC.
    pub gc_keep_fraction: f64,
    /// VSIDS decay factor.
    pub vsids_decay: f64,
}

/// A proof term for the BV decision procedure.
#[derive(Clone, Debug)]
pub enum BvProofTerm {
    /// The goal was proved by showing the negation is unsatisfiable.
    UnsatRefutation {
        /// The original goal expression.
        goal: Expr,
        /// Proof of unsatisfiability (empty clause derivation).
        unsat_proof: UnsatProof,
        /// The reconstructed kernel proof term.
        kernel_proof: Expr,
    },
    /// The goal was disproved: found a satisfying counterexample.
    SatCounterexample {
        /// The original goal expression.
        goal: Expr,
        /// The counterexample model.
        model: Model,
    },
    /// Trivially true (no BV variables).
    Trivial(Expr),
}

/// Tagged bit-vector value used during evaluation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BitVecValue {
    /// Concrete known value.
    Concrete(BitVec),
    /// Symbolic variable (index into variable table).
    Symbolic(u32, BitWidth),
    /// Unknown / not yet evaluated.
    Unknown(BitWidth),
}

/// A concrete bit-vector value with fixed width.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BitVec {
    /// Width of this bit-vector.
    pub width: BitWidth,
    /// Bits stored LSB-first: bits\[0\] is the least-significant bit.
    pub bits: Vec<bool>,
}

/// VSIDS (Variable State Independent Decaying Sum) heuristic for branching.
#[derive(Clone, Debug)]
pub struct VsidsScorer {
    /// Activity score for each variable.
    pub(crate) activity: Vec<f64>,
    /// Amount to bump on conflict participation.
    pub(crate) bump_amount: f64,
    /// Decay factor applied after each conflict.
    pub(crate) decay_factor: f64,
    /// Priority queue ordering cache: sorted variable indices (lazily rebuilt).
    pub(crate) order_dirty: bool,
    /// Sorted variable order (highest activity first).
    pub(crate) order: Vec<SatVar>,
}

/// A CNF formula: conjunction of clauses over numbered variables.
#[derive(Clone, Debug, Default)]
pub struct CnfFormula {
    /// Each clause is a disjunction of literals.
    pub clauses: Vec<Vec<Literal>>,
    /// Total number of variables (variables are 0..num_vars-1).
    pub num_vars: u32,
}

/// A SAT variable, identified by a non-negative integer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SatVar(pub u32);

/// Fixed bit-width for a bit-vector type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BitWidth(pub u32);
