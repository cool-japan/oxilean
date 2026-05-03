//! Types for the peephole optimisation pass.

/// A single instruction in the peephole instruction model.
///
/// The instruction set is intentionally small: it covers the stack-machine
/// primitives that appear in common IR back-ends and for which standard
/// peephole identities are well-known.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PeepInstr {
    /// Push an immediate integer constant.
    Const(i64),
    /// Integer addition (pops two, pushes one).
    Add,
    /// Integer subtraction (pops two, pushes one).
    Sub,
    /// Integer multiplication (pops two, pushes one).
    Mul,
    /// Integer division (pops two, pushes one).
    Div,
    /// Arithmetic negation (pops one, pushes one).
    Neg,
    /// Load the value of a named variable.
    Load(String),
    /// Store the top-of-stack into a named variable.
    Store(String),
    /// Conditional branch to the named label.
    Branch(String),
    /// Unconditional jump to the named label.
    Jump(String),
    /// Return from the current function.
    Ret,
    /// Duplicate the top-of-stack.
    Dup,
    /// Discard the top-of-stack.
    Pop,
    /// Swap the top two stack values.
    Swap,
    /// No-operation (placeholder; removed by the dead-nop rule).
    Nop,
}

/// A pattern to be matched against a window of instructions.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PeepPattern {
    /// The sequence of instructions that must match.
    pub instrs: Vec<PeepInstr>,
    /// Human-readable name for this pattern (used in diagnostics).
    pub name: String,
}

/// The replacement sequence produced when a [`PeepPattern`] fires.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PeepReplacement {
    /// The sequence that replaces the matched window.
    pub instrs: Vec<PeepInstr>,
    /// Human-readable name for this replacement (used in diagnostics).
    pub name: String,
}

/// A complete peephole optimisation rule: pattern → replacement.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PeepRule {
    /// The pattern to look for.
    pub pattern: PeepPattern,
    /// The replacement to emit when the pattern fires.
    pub replacement: PeepReplacement,
    /// Higher priority rules are tried first.
    pub priority: i32,
}

/// The result returned by [`crate::opt_peephole::run_peephole`].
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PeepResult {
    /// The optimised instruction sequence.
    pub instructions: Vec<PeepInstr>,
    /// Names of the rules that were applied (in order of application).
    pub rules_applied: Vec<String>,
    /// The net number of instructions eliminated (original count minus final count).
    pub reduction: usize,
}
