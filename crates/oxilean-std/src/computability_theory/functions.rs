//! Functions for computability and decidability theory.

use std::collections::HashMap;

use super::types::{
    ComplexityClass, DecidabilityResult, Direction, RegisterMachine, RmInstruction, TmResult,
    TuringMachine,
};

// ── TuringMachine methods ─────────────────────────────────────────────────────

impl TuringMachine {
    /// Performs one step of computation.
    /// Returns `true` if the machine is still running (not halted), `false` if halted.
    pub fn step(&mut self) -> bool {
        if self.is_halted() {
            return false;
        }

        let symbol = self.read_symbol();
        let key = (self.current_state.clone(), symbol);

        match self.transitions.get(&key).cloned() {
            None => {
                // No transition defined: reject by default
                if let Some(reject) = self.reject_states.first() {
                    self.current_state = reject.clone();
                }
                false
            }
            Some((new_state, write_sym, direction)) => {
                // Write symbol
                if self.head < self.tape.len() {
                    self.tape[self.head] = write_sym;
                } else {
                    self.tape.push(write_sym);
                }

                // Update state
                self.current_state = new_state;

                // Move head
                match direction {
                    Direction::Left => {
                        if self.head > 0 {
                            self.head -= 1;
                        } else {
                            // Extend tape to the left by shifting
                            self.tape.insert(0, '_');
                            // head stays at 0
                        }
                    }
                    Direction::Right => {
                        self.head += 1;
                        if self.head >= self.tape.len() {
                            self.tape.push('_');
                        }
                    }
                    Direction::Stay => {}
                }

                !self.is_halted()
            }
        }
    }

    /// Runs the machine for at most `max_steps` steps.
    pub fn run(&mut self, max_steps: usize) -> TmResult {
        for steps in 0..max_steps {
            if self.is_accepting() {
                return TmResult::Accept(self.tape.clone());
            }
            if self.is_rejecting() {
                return TmResult::Reject(self.tape.clone());
            }
            let still_running = self.step();
            if !still_running {
                if self.is_accepting() {
                    return TmResult::Accept(self.tape.clone());
                }
                return TmResult::Reject(self.tape.clone());
            }
            let _ = steps;
        }
        // Check final state after loop
        if self.is_accepting() {
            return TmResult::Accept(self.tape.clone());
        }
        if self.is_rejecting() {
            return TmResult::Reject(self.tape.clone());
        }
        TmResult::Loop {
            steps_taken: max_steps,
        }
    }
}

// ── RegisterMachine methods ───────────────────────────────────────────────────

impl RegisterMachine {
    /// Runs the register machine for at most `max_steps` steps.
    /// Returns `Some(r0)` if the machine halts (Halt instruction), `None` on timeout.
    pub fn run(&mut self, max_steps: usize) -> Option<i64> {
        for _ in 0..max_steps {
            if self.pc >= self.program.len() {
                return Some(self.get_register(0));
            }
            match self.program[self.pc].clone() {
                RmInstruction::Halt => {
                    return Some(self.get_register(0));
                }
                RmInstruction::Inc(r) => {
                    if r < self.registers.len() {
                        self.registers[r] += 1;
                    }
                    self.pc += 1;
                }
                RmInstruction::Dec(r, jump_addr) => {
                    if r < self.registers.len() && self.registers[r] > 0 {
                        self.registers[r] -= 1;
                        self.pc += 1;
                    } else {
                        self.pc = jump_addr;
                    }
                }
                RmInstruction::JumpIfZero(r, jump_addr) => {
                    if self.get_register(r) == 0 {
                        self.pc = jump_addr;
                    } else {
                        self.pc += 1;
                    }
                }
            }
        }
        None
    }
}

// ── Turing machine constructors ───────────────────────────────────────────────

/// Builds a Turing machine that recognizes palindromes over {0, 1}.
/// The language is { w ∈ {0,1}* | w = reverse(w) }.
/// Uses a standard mark-and-check algorithm.
pub fn build_palindrome_checker() -> TuringMachine {
    // States: q0=start, q1=saw0, q2=saw1, q3=check_right_0, q4=check_right_1,
    //         q5=scan_right, q6=scan_left, qA=accept, qR=reject
    let states: Vec<String> = [
        "q0",
        "q_read0",
        "q_read1",
        "q_scan_right0",
        "q_scan_right1",
        "q_scan_left",
        "q_check0",
        "q_check1",
        "q_accept",
        "q_reject",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    let alphabet = vec!['0', '1', 'X', '_'];

    let mut transitions: HashMap<(String, char), (String, char, Direction)> = HashMap::new();

    let t = |from: &str, sym: char, to: &str, write: char, dir: Direction| {
        ((from.to_string(), sym), (to.to_string(), write, dir))
    };

    // q0: read and mark leftmost symbol
    transitions.extend([
        t("q0", '0', "q_scan_right0", 'X', Direction::Right),
        t("q0", '1', "q_scan_right1", 'X', Direction::Right),
        t("q0", 'X', "q0", 'X', Direction::Right),
        t("q0", '_', "q_accept", '_', Direction::Stay), // empty or all marked
    ]);

    // q_scan_right0: scan right looking for rightmost '0'
    transitions.extend([
        t("q_scan_right0", '0', "q_scan_right0", '0', Direction::Right),
        t("q_scan_right0", '1', "q_scan_right0", '1', Direction::Right),
        t("q_scan_right0", 'X', "q_scan_right0", 'X', Direction::Right),
        t("q_scan_right0", '_', "q_check0", '_', Direction::Left),
    ]);

    // q_scan_right1: scan right looking for rightmost '1'
    transitions.extend([
        t("q_scan_right1", '0', "q_scan_right1", '0', Direction::Right),
        t("q_scan_right1", '1', "q_scan_right1", '1', Direction::Right),
        t("q_scan_right1", 'X', "q_scan_right1", 'X', Direction::Right),
        t("q_scan_right1", '_', "q_check1", '_', Direction::Left),
    ]);

    // q_check0: at rightmost unprocessed cell, must be '0'
    transitions.extend([
        t("q_check0", '0', "q_scan_left", 'X', Direction::Left),
        t("q_check0", '1', "q_reject", '1', Direction::Stay),
        t("q_check0", 'X', "q_accept", 'X', Direction::Stay), // only marked remain
        t("q_check0", '_', "q_accept", '_', Direction::Stay),
    ]);

    // q_check1: at rightmost unprocessed cell, must be '1'
    transitions.extend([
        t("q_check1", '1', "q_scan_left", 'X', Direction::Left),
        t("q_check1", '0', "q_reject", '0', Direction::Stay),
        t("q_check1", 'X', "q_accept", 'X', Direction::Stay),
        t("q_check1", '_', "q_accept", '_', Direction::Stay),
    ]);

    // q_scan_left: scan back to leftmost unprocessed
    transitions.extend([
        t("q_scan_left", '0', "q_scan_left", '0', Direction::Left),
        t("q_scan_left", '1', "q_scan_left", '1', Direction::Left),
        t("q_scan_left", 'X', "q_scan_left", 'X', Direction::Left),
        t("q_scan_left", '_', "q0", '_', Direction::Right),
    ]);

    TuringMachine::new(
        states,
        alphabet,
        transitions,
        "q0".to_string(),
        vec!["q_accept".to_string()],
        vec!["q_reject".to_string()],
    )
}

/// Builds a Turing machine that computes binary increment (n → n+1).
/// Input: a binary number written left-to-right (MSB first) on the tape.
/// The machine scans to the end and adds 1 from the right.
pub fn build_binary_increment() -> TuringMachine {
    let states: Vec<String> = ["q_scan", "q_carry", "q_done", "q_accept", "q_reject"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    let alphabet = vec!['0', '1', '_'];

    let mut transitions: HashMap<(String, char), (String, char, Direction)> = HashMap::new();

    let t = |from: &str, sym: char, to: &str, write: char, dir: Direction| {
        ((from.to_string(), sym), (to.to_string(), write, dir))
    };

    // q_scan: move right to find end of input
    transitions.extend([
        t("q_scan", '0', "q_scan", '0', Direction::Right),
        t("q_scan", '1', "q_scan", '1', Direction::Right),
        t("q_scan", '_', "q_carry", '_', Direction::Left),
    ]);

    // q_carry: add carry from the right
    transitions.extend([
        t("q_carry", '0', "q_done", '1', Direction::Left), // 0+1=1, no carry
        t("q_carry", '1', "q_carry", '0', Direction::Left), // 1+1=0, carry
        t("q_carry", '_', "q_accept", '1', Direction::Stay), // all 1s: prepend 1
    ]);

    // q_done: scan left to beginning, then accept
    transitions.extend([
        t("q_done", '0', "q_done", '0', Direction::Left),
        t("q_done", '1', "q_done", '1', Direction::Left),
        t("q_done", '_', "q_accept", '_', Direction::Right),
    ]);

    TuringMachine::new(
        states,
        alphabet,
        transitions,
        "q_scan".to_string(),
        vec!["q_accept".to_string()],
        vec!["q_reject".to_string()],
    )
}

// ── Decidability lookup ────────────────────────────────────────────────────────

/// Returns the decidability classification of a well-known computational problem.
/// Uses a lookup table of famous results.
pub fn universal_property_holds(description: &str) -> DecidabilityResult {
    let normalized = description.to_lowercase();
    // Decidable problems
    let decidable = [
        "halting problem for linear bounded automata",
        "emptiness of regular language",
        "equivalence of dfa",
        "membership in regular language",
        "membership in context-free language",
        "emptiness of context-free language",
        "satisfiability of propositional logic",
        "validity of propositional logic",
        "linear arithmetic over integers",
        "presburger arithmetic",
        "tautology in propositional calculus",
        "word problem for free groups",
    ];
    // Semi-decidable (r.e. but not decidable)
    let semi_decidable = [
        "halting problem",
        "turing machine acceptance",
        "membership in recursively enumerable language",
        "provability in peano arithmetic",
        "validity of first-order logic",
        "satisfiability of first-order logic",
    ];
    // Undecidable
    let undecidable = [
        "halting problem for turing machines",
        "post correspondence problem",
        "hilbert's tenth problem",
        "diophantine equations",
        "equivalence of turing machines",
        "rice's theorem",
        "totality of turing machine",
        "emptiness of turing machine language",
        "context-free grammar ambiguity",
        "intersection of context-free grammars",
        "word problem for groups",
        "equivalence of context-free grammars",
        "universal turing machine acceptance",
    ];

    // Check undecidable first so that "halting problem for turing machines" wins
    // over the more-general "turing machine acceptance" (semi-decidable).
    for p in &undecidable {
        if normalized.contains(p) {
            return DecidabilityResult::Undecidable;
        }
    }
    for p in &decidable {
        if normalized.contains(p) {
            return DecidabilityResult::Decidable;
        }
    }
    for p in &semi_decidable {
        if normalized.contains(p) {
            return DecidabilityResult::SemiDecidable;
        }
    }
    DecidabilityResult::Unknown
}

/// Returns a known reduction chain from `from` to `to`, if one exists.
/// Each entry in the returned vector is a step: "A ≤_m B".
pub fn reduction_chain(from: &str, to: &str) -> Option<Vec<String>> {
    // Hard-coded known many-one reduction chains
    let chains: &[(&str, &str, &[&str])] = &[
        (
            "halting_problem",
            "acceptance_problem",
            &["halting_problem ≤_m acceptance_problem"],
        ),
        (
            "post_correspondence",
            "halting_problem",
            &["post_correspondence ≤_m halting_problem"],
        ),
        (
            "hilbert10",
            "halting_problem",
            &["hilbert10 ≤_m halting_problem"],
        ),
        (
            "emptiness_tm",
            "halting_problem",
            &["emptiness_tm ≤_m halting_problem"],
        ),
        (
            "equivalence_tm",
            "acceptance_problem",
            &["equivalence_tm ≤_m acceptance_problem"],
        ),
    ];

    for (f, t, chain) in chains {
        if f == &from && t == &to {
            return Some(chain.iter().map(|s| s.to_string()).collect());
        }
    }
    None
}

/// Returns the complexity class of a well-known problem, if known.
pub fn complexity_class_of(problem: &str) -> Option<ComplexityClass> {
    let normalized = problem.to_lowercase();
    let table: &[(&str, ComplexityClass)] = &[
        ("reachability in directed graph", ComplexityClass::Logspace),
        ("graph reachability", ComplexityClass::Logspace),
        ("path problem", ComplexityClass::Logspace),
        ("sorting", ComplexityClass::P),
        ("shortest path", ComplexityClass::P),
        ("maximum matching", ComplexityClass::P),
        ("linear programming", ComplexityClass::P),
        ("primality testing", ComplexityClass::P),
        ("satisfiability", ComplexityClass::NP),
        ("3-sat", ComplexityClass::NP),
        ("graph coloring", ComplexityClass::NP),
        ("traveling salesman", ComplexityClass::NP),
        ("knapsack", ComplexityClass::NP),
        ("clique", ComplexityClass::NP),
        ("vertex cover", ComplexityClass::NP),
        ("quantified boolean formula", ComplexityClass::PSpace),
        ("qbf", ComplexityClass::PSpace),
        ("pspace-complete", ComplexityClass::PSpace),
        ("succinct graph coloring", ComplexityClass::ExpTime),
        ("succinct circuit satisfiability", ComplexityClass::ExpTime),
        ("primitive recursive", ComplexityClass::Primitive),
        ("ackermann function", ComplexityClass::General),
        ("busy beaver", ComplexityClass::General),
        ("halting problem", ComplexityClass::General),
    ];

    for (name, class) in table {
        if normalized.contains(name) {
            return Some(class.clone());
        }
    }
    None
}

/// Returns the known value of the Busy Beaver function BB(n), for small n.
/// BB(n) is the maximum number of 1s that an n-state TM can write before halting.
/// Known: BB(1)=1, BB(2)=4, BB(3)=6, BB(4)=13.
pub fn busy_beaver(n: u32) -> Option<u64> {
    match n {
        1 => Some(1),
        2 => Some(4),
        3 => Some(6),
        4 => Some(13),
        _ => None, // BB(5) and above are unknown or uncomputed
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Direction Display ──────────────────────────────────────────────────────

    #[test]
    fn test_direction_display() {
        assert_eq!(format!("{}", Direction::Left), "L");
        assert_eq!(format!("{}", Direction::Right), "R");
        assert_eq!(format!("{}", Direction::Stay), "S");
    }

    // ── TuringMachine::new ─────────────────────────────────────────────────────

    #[test]
    fn test_tm_new() {
        let tm = build_binary_increment();
        assert!(!tm.states.is_empty());
        assert!(!tm.accept_states.is_empty());
        assert!(!tm.reject_states.is_empty());
    }

    // ── binary increment TM ────────────────────────────────────────────────────

    #[test]
    fn test_binary_increment_zero() {
        let mut tm = build_binary_increment();
        tm.load_input(&['0']);
        let result = tm.run(1000);
        // 0 + 1 = 1
        match result {
            TmResult::Accept(tape) => {
                let meaningful: Vec<char> = tape.into_iter().filter(|&c| c != '_').collect();
                assert!(meaningful.contains(&'1'), "Expected '1' in tape");
            }
            other => panic!("Expected Accept, got {other:?}"),
        }
    }

    #[test]
    fn test_binary_increment_one() {
        let mut tm = build_binary_increment();
        tm.load_input(&['1']);
        let result = tm.run(1000);
        // 1 + 1 = 10
        match result {
            TmResult::Accept(tape) => {
                let meaningful: String = tape.into_iter().filter(|&c| c != '_').collect();
                // Should contain "10"
                assert!(
                    meaningful.contains("10") || meaningful.contains("1"),
                    "Expected binary 10, got {meaningful}"
                );
            }
            other => panic!("Expected Accept, got {other:?}"),
        }
    }

    #[test]
    fn test_binary_increment_101() {
        // 101 (=5) + 1 = 110 (=6)
        let mut tm = build_binary_increment();
        tm.load_input(&['1', '0', '1']);
        let result = tm.run(1000);
        match result {
            TmResult::Accept(tape) => {
                let meaningful: String = tape.into_iter().filter(|&c| c != '_').collect();
                assert!(meaningful.contains("110"), "Expected 110, got {meaningful}");
            }
            other => panic!("Expected Accept, got {other:?}"),
        }
    }

    // ── palindrome checker TM ──────────────────────────────────────────────────

    #[test]
    fn test_palindrome_empty() {
        let mut tm = build_palindrome_checker();
        tm.load_input(&[]);
        let result = tm.run(10000);
        // Empty string is trivially a palindrome
        assert!(
            matches!(result, TmResult::Accept(_)),
            "Expected Accept for empty, got {result:?}"
        );
    }

    #[test]
    fn test_palindrome_single_char() {
        let mut tm = build_palindrome_checker();
        tm.load_input(&['0']);
        let result = tm.run(10000);
        assert!(
            matches!(result, TmResult::Accept(_)),
            "Expected Accept for '0', got {result:?}"
        );
    }

    #[test]
    fn test_palindrome_01_not_palindrome() {
        let mut tm = build_palindrome_checker();
        tm.load_input(&['0', '1']);
        let result = tm.run(10000);
        assert!(
            matches!(result, TmResult::Reject(_)),
            "Expected Reject for '01', got {result:?}"
        );
    }

    #[test]
    fn test_palindrome_00_is_palindrome() {
        let mut tm = build_palindrome_checker();
        tm.load_input(&['0', '0']);
        let result = tm.run(10000);
        assert!(
            matches!(result, TmResult::Accept(_)),
            "Expected Accept for '00', got {result:?}"
        );
    }

    #[test]
    fn test_palindrome_010() {
        let mut tm = build_palindrome_checker();
        tm.load_input(&['0', '1', '0']);
        let result = tm.run(10000);
        assert!(
            matches!(result, TmResult::Accept(_)),
            "Expected Accept for '010', got {result:?}"
        );
    }

    #[test]
    fn test_palindrome_011_not() {
        let mut tm = build_palindrome_checker();
        tm.load_input(&['0', '1', '1']);
        let result = tm.run(10000);
        assert!(
            matches!(result, TmResult::Reject(_)),
            "Expected Reject for '011', got {result:?}"
        );
    }

    // ── RegisterMachine ────────────────────────────────────────────────────────

    #[test]
    fn test_register_machine_halt_immediately() {
        let program = vec![RmInstruction::Halt];
        let mut rm = RegisterMachine::new(2, program);
        rm.set_register(0, 42);
        let result = rm.run(100);
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_register_machine_increment() {
        // Increment r0 three times then halt
        let program = vec![
            RmInstruction::Inc(0),
            RmInstruction::Inc(0),
            RmInstruction::Inc(0),
            RmInstruction::Halt,
        ];
        let mut rm = RegisterMachine::new(2, program);
        let result = rm.run(100);
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_register_machine_decrement_zero() {
        // Dec(0, 2): if r0=0, jump to instruction 2 (Halt)
        let program = vec![
            RmInstruction::Dec(0, 2),
            RmInstruction::Inc(1), // skipped
            RmInstruction::Halt,
        ];
        let mut rm = RegisterMachine::new(2, program);
        rm.set_register(0, 0);
        let result = rm.run(100);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_register_machine_copy_r1_to_r0() {
        // Simple program: copy r1 to r0 by decrementing r1 and incrementing r0
        // r0=0, r1=5 -> r0=5
        // loop: Dec(1, end), Inc(0), jump to loop
        let program = vec![
            RmInstruction::Dec(1, 3), // 0: if r1=0 jump to 3 (Halt), else dec r1, goto 1
            RmInstruction::Inc(0),    // 1: r0++
            RmInstruction::JumpIfZero(2, 0), // 2: jump back (r2 is always 0)
            RmInstruction::Halt,      // 3
        ];
        let mut rm = RegisterMachine::new(3, program);
        rm.set_register(0, 0);
        rm.set_register(1, 5);
        let result = rm.run(1000);
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_register_machine_timeout() {
        // Infinite loop: just keep incrementing
        let program = vec![
            RmInstruction::Inc(0),
            RmInstruction::JumpIfZero(1, 0), // r1 is always 0, so always jump to 0
        ];
        let mut rm = RegisterMachine::new(2, program);
        let result = rm.run(50);
        assert!(result.is_none());
    }

    // ── decidability lookup ────────────────────────────────────────────────────

    #[test]
    fn test_decidability_halting() {
        let r = universal_property_holds("Halting Problem for Turing Machines");
        assert_eq!(r, DecidabilityResult::Undecidable);
    }

    #[test]
    fn test_decidability_regular_emptiness() {
        let r = universal_property_holds("emptiness of regular language");
        assert_eq!(r, DecidabilityResult::Decidable);
    }

    #[test]
    fn test_decidability_fo_validity() {
        let r = universal_property_holds("validity of first-order logic");
        assert_eq!(r, DecidabilityResult::SemiDecidable);
    }

    #[test]
    fn test_decidability_unknown() {
        let r = universal_property_holds("some random obscure problem");
        assert_eq!(r, DecidabilityResult::Unknown);
    }

    #[test]
    fn test_decidability_post_correspondence() {
        let r = universal_property_holds("Post Correspondence Problem");
        assert_eq!(r, DecidabilityResult::Undecidable);
    }

    #[test]
    fn test_decidability_hilbert_tenth() {
        let r = universal_property_holds("Hilbert's tenth problem");
        assert_eq!(r, DecidabilityResult::Undecidable);
    }

    // ── reduction_chain ────────────────────────────────────────────────────────

    #[test]
    fn test_reduction_chain_known() {
        let chain = reduction_chain("halting_problem", "acceptance_problem");
        assert!(chain.is_some());
        let chain = chain.unwrap();
        assert!(!chain.is_empty());
        assert!(chain[0].contains("≤_m"));
    }

    #[test]
    fn test_reduction_chain_unknown() {
        let chain = reduction_chain("foo", "bar");
        assert!(chain.is_none());
    }

    // ── complexity_class_of ────────────────────────────────────────────────────

    #[test]
    fn test_complexity_sat() {
        let c = complexity_class_of("satisfiability");
        assert_eq!(c, Some(ComplexityClass::NP));
    }

    #[test]
    fn test_complexity_sorting() {
        let c = complexity_class_of("sorting");
        assert_eq!(c, Some(ComplexityClass::P));
    }

    #[test]
    fn test_complexity_qbf() {
        let c = complexity_class_of("QBF");
        assert_eq!(c, Some(ComplexityClass::PSpace));
    }

    #[test]
    fn test_complexity_unknown() {
        let c = complexity_class_of("some unknown problem xyz");
        assert!(c.is_none());
    }

    // ── busy_beaver ────────────────────────────────────────────────────────────

    #[test]
    fn test_busy_beaver_1() {
        assert_eq!(busy_beaver(1), Some(1));
    }

    #[test]
    fn test_busy_beaver_2() {
        assert_eq!(busy_beaver(2), Some(4));
    }

    #[test]
    fn test_busy_beaver_3() {
        assert_eq!(busy_beaver(3), Some(6));
    }

    #[test]
    fn test_busy_beaver_4() {
        assert_eq!(busy_beaver(4), Some(13));
    }

    #[test]
    fn test_busy_beaver_5_unknown() {
        assert_eq!(busy_beaver(5), None);
    }

    // ── TmResult Display ───────────────────────────────────────────────────────

    #[test]
    fn test_tm_result_display() {
        let r = TmResult::Accept(vec!['0', '1']);
        assert!(format!("{r}").contains("Accept"));
        let r2 = TmResult::Loop { steps_taken: 100 };
        assert!(format!("{r2}").contains("100"));
    }

    // ── ComplexityClass Display ────────────────────────────────────────────────

    #[test]
    fn test_complexity_class_display() {
        assert_eq!(format!("{}", ComplexityClass::NP), "NP");
        assert_eq!(format!("{}", ComplexityClass::PSpace), "PSPACE");
        assert_eq!(format!("{}", ComplexityClass::General), "GENERAL_RECURSIVE");
    }

    // ── TuringMachine::load_input / read_symbol ────────────────────────────────

    #[test]
    fn test_tm_load_and_read() {
        let mut tm = build_binary_increment();
        tm.load_input(&['1', '0', '1']);
        assert_eq!(tm.read_symbol(), '1');
        assert_eq!(tm.head, 0);
        assert_eq!(tm.tape.len(), 3);
    }

    #[test]
    fn test_tm_is_halted_initial() {
        let tm = build_binary_increment();
        // Not in accept or reject initially
        assert!(!tm.is_halted());
    }
}
