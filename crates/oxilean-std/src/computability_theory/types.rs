//! Types for computability and decidability theory.

use std::collections::HashMap;

/// Movement direction for a Turing machine head.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    /// Move the head one cell to the left.
    Left,
    /// Move the head one cell to the right.
    Right,
    /// Keep the head in place.
    Stay,
}

/// The result of running a Turing machine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TmResult {
    /// The machine reached an accept state; contains the final tape contents.
    Accept(Vec<char>),
    /// The machine reached a reject state; contains the final tape contents.
    Reject(Vec<char>),
    /// The machine did not halt within the allowed steps.
    Loop {
        /// Number of steps actually executed before giving up.
        steps_taken: usize,
    },
    /// The per-step limit was exhausted (same as Loop but explicitly labelled).
    StepLimit,
}

/// A deterministic single-tape Turing machine.
#[derive(Debug, Clone)]
pub struct TuringMachine {
    /// All states in the machine (including start, accept, reject).
    pub states: Vec<String>,
    /// The tape alphabet (must include a blank symbol, conventionally `'_'`).
    pub alphabet: Vec<char>,
    /// The current contents of the tape.
    pub tape: Vec<char>,
    /// The current head position (index into `tape`).
    pub head: usize,
    /// The current state.
    pub current_state: String,
    /// States in which the machine accepts.
    pub accept_states: Vec<String>,
    /// States in which the machine rejects.
    pub reject_states: Vec<String>,
    /// Transition function: (state, symbol) -> (new_state, write_symbol, direction).
    pub transitions: HashMap<(String, char), (String, char, Direction)>,
}

impl TuringMachine {
    /// Creates a new Turing machine.
    pub fn new(
        states: Vec<String>,
        alphabet: Vec<char>,
        transitions: HashMap<(String, char), (String, char, Direction)>,
        start: String,
        accept_states: Vec<String>,
        reject_states: Vec<String>,
    ) -> Self {
        TuringMachine {
            states,
            alphabet,
            tape: vec!['_'],
            head: 0,
            current_state: start,
            accept_states,
            reject_states,
            transitions,
        }
    }

    /// Loads a string onto the tape and resets the head to position 0.
    pub fn load_input(&mut self, input: &[char]) {
        self.tape = input.to_vec();
        if self.tape.is_empty() {
            self.tape.push('_');
        }
        self.head = 0;
    }

    /// Returns the symbol currently under the head.
    pub fn read_symbol(&self) -> char {
        self.tape.get(self.head).copied().unwrap_or('_')
    }

    /// Checks if the machine is in an accept state.
    pub fn is_accepting(&self) -> bool {
        self.accept_states.contains(&self.current_state)
    }

    /// Checks if the machine is in a reject state.
    pub fn is_rejecting(&self) -> bool {
        self.reject_states.contains(&self.current_state)
    }

    /// Checks if the machine is in a halting state (accept or reject).
    pub fn is_halted(&self) -> bool {
        self.is_accepting() || self.is_rejecting()
    }
}

/// An instruction in a Register Machine (RM).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RmInstruction {
    /// Increment register `r`.
    Inc(usize),
    /// Decrement register `r` if non-zero; otherwise jump to `jump_addr`.
    Dec(usize, usize),
    /// If register `r` is zero, jump to `jump_addr`.
    JumpIfZero(usize, usize),
    /// Halt the machine.
    Halt,
}

/// A Register Machine (Minsky machine / counter machine).
#[derive(Debug, Clone)]
pub struct RegisterMachine {
    /// The register values (indices are register numbers).
    pub registers: Vec<i64>,
    /// The program: a sequence of instructions.
    pub program: Vec<RmInstruction>,
    /// The program counter.
    pub pc: usize,
}

impl RegisterMachine {
    /// Creates a new register machine with `num_registers` registers (all zero).
    pub fn new(num_registers: usize, program: Vec<RmInstruction>) -> Self {
        RegisterMachine {
            registers: vec![0; num_registers],
            program,
            pc: 0,
        }
    }

    /// Sets the value of register `r`.
    pub fn set_register(&mut self, r: usize, value: i64) {
        if r < self.registers.len() {
            self.registers[r] = value;
        }
    }

    /// Gets the value of register `r` (0 if out of bounds).
    pub fn get_register(&self, r: usize) -> i64 {
        self.registers.get(r).copied().unwrap_or(0)
    }
}

/// Decidability classification for a language or problem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecidabilityResult {
    /// The problem is decidable (there exists a TM that always halts with yes/no).
    Decidable,
    /// The problem is semi-decidable (recursively enumerable but not decidable).
    SemiDecidable,
    /// The problem is undecidable (no algorithm can solve it).
    Undecidable,
    /// The decidability status is unknown or not established.
    Unknown,
}

/// Standard complexity classes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComplexityClass {
    /// L: logarithmic space.
    Logspace,
    /// P: polynomial time.
    P,
    /// NP: nondeterministic polynomial time.
    NP,
    /// PSPACE: polynomial space.
    PSpace,
    /// EXPTIME: exponential time.
    ExpTime,
    /// Primitive recursive functions.
    Primitive,
    /// General recursive (Turing-computable) functions.
    General,
}

impl std::fmt::Display for ComplexityClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ComplexityClass::Logspace => "L",
            ComplexityClass::P => "P",
            ComplexityClass::NP => "NP",
            ComplexityClass::PSpace => "PSPACE",
            ComplexityClass::ExpTime => "EXPTIME",
            ComplexityClass::Primitive => "PRIMITIVE_RECURSIVE",
            ComplexityClass::General => "GENERAL_RECURSIVE",
        };
        write!(f, "{s}")
    }
}

impl std::fmt::Display for DecidabilityResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DecidabilityResult::Decidable => "Decidable",
            DecidabilityResult::SemiDecidable => "Semi-decidable",
            DecidabilityResult::Undecidable => "Undecidable",
            DecidabilityResult::Unknown => "Unknown",
        };
        write!(f, "{s}")
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Left => write!(f, "L"),
            Direction::Right => write!(f, "R"),
            Direction::Stay => write!(f, "S"),
        }
    }
}

impl std::fmt::Display for TmResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TmResult::Accept(tape) => write!(f, "Accept({tape:?})"),
            TmResult::Reject(tape) => write!(f, "Reject({tape:?})"),
            TmResult::Loop { steps_taken } => write!(f, "Loop(steps={steps_taken})"),
            TmResult::StepLimit => write!(f, "StepLimit"),
        }
    }
}
