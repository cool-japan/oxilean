//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet, VecDeque};

use super::functions::*;

/// Deterministic Finite Automaton.
///
/// States are indices `0..states`.  `delta[q][c]` gives the next state
/// when the automaton is in state `q` and reads symbol index `c`.
/// `alphabet[c]` maps symbol index to `char`.
pub struct Dfa {
    /// Number of states.
    pub states: usize,
    /// Ordered alphabet.
    pub alphabet: Vec<char>,
    /// Transition function: `delta[state][symbol_index] = next_state`.
    pub delta: Vec<Vec<usize>>,
    /// Start state index.
    pub start: usize,
    /// `accept[q]` is true iff state `q` is accepting.
    pub accept: Vec<bool>,
}
impl Dfa {
    /// Returns `true` iff the DFA accepts `input`.
    pub fn accepts(&self, input: &str) -> bool {
        let mut state = self.start;
        for ch in input.chars() {
            if let Some(idx) = self.alphabet.iter().position(|&c| c == ch) {
                state = self.delta[state][idx];
            } else {
                return false;
            }
        }
        self.accept[state]
    }
}
/// CYK (Cocke-Younger-Kasami) parser for context-free grammars in CNF.
///
/// Given a CFG in Chomsky Normal Form, `CykParser` decides whether a given
/// string belongs to the language in O(n³ |G|) time using dynamic programming.
///
/// Rules must be in CNF:
/// - `A → B C`  (binary non-terminal rules), or
/// - `A → a`    (unit terminal rules), or
/// - `S → ε`    (the start symbol may derive ε).
pub struct CykParser {
    /// Start variable name.
    pub start: String,
    /// Binary rules: `(A, B, C)` means `A → B C`.
    pub binary: Vec<(String, String, String)>,
    /// Unit rules: `(A, c)` means `A → "c"` (single terminal character).
    pub unit: Vec<(String, char)>,
    /// Whether the start symbol derives ε.
    pub derives_empty: bool,
}
impl CykParser {
    /// Returns `true` iff the grammar derives `input`.
    ///
    /// Uses the standard CYK DP table: `table[i][j]` is the set of variables
    /// that derive `input[i..=j]`.
    pub fn accepts(&self, input: &str) -> bool {
        let chars: Vec<char> = input.chars().collect();
        let n = chars.len();
        if n == 0 {
            return self.derives_empty;
        }
        let mut table: Vec<Vec<HashSet<String>>> = vec![vec![HashSet::new(); n]; n];
        for (i, &ch) in chars.iter().enumerate() {
            for (lhs, terminal) in &self.unit {
                if *terminal == ch {
                    table[i][i].insert(lhs.clone());
                }
            }
        }
        for len in 2..=n {
            for i in 0..=(n - len) {
                let j = i + len - 1;
                for k in i..j {
                    for (a, b, c) in &self.binary {
                        if table[i][k].contains(b) && table[k + 1][j].contains(c) {
                            table[i][j].insert(a.clone());
                        }
                    }
                }
            }
        }
        table[0][n - 1].contains(&self.start)
    }
}
/// A Turing machine tape direction.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TapeDir {
    /// Move left.
    Left,
    /// Move right.
    Right,
    /// Stay.
    Stay,
}
/// Direction for the TM head movement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TmDirection {
    Left,
    Right,
    Stay,
}
/// Regular expression AST.
pub enum Regex {
    /// The empty language (matches nothing).
    Empty,
    /// The empty string ε.
    Epsilon,
    /// A single character literal.
    Char(char),
    /// Alternation: `r | s`.
    Alt(Box<Regex>, Box<Regex>),
    /// Concatenation: `r · s`.
    Concat(Box<Regex>, Box<Regex>),
    /// Kleene star: `r*`.
    Star(Box<Regex>),
}
impl Regex {
    /// Returns `true` iff this regex matches the string `s` (backtracking).
    pub fn matches(&self, s: &str) -> bool {
        self.match_prefix(s.as_bytes()) == Some(s.len())
    }
    /// Returns `Some(consumed)` if the regex can match the first `consumed`
    /// bytes of `input`, or `None` if it cannot start a match here.
    /// For `Star` and `Alt` this uses greedy-then-backtrack semantics.
    fn match_prefix(&self, input: &[u8]) -> Option<usize> {
        match self {
            Regex::Empty => None,
            Regex::Epsilon => Some(0),
            Regex::Char(c) => {
                let ch_bytes = c.to_string();
                let cb = ch_bytes.as_bytes();
                if input.starts_with(cb) {
                    Some(cb.len())
                } else {
                    None
                }
            }
            Regex::Alt(r, s) => r.match_prefix(input).or_else(|| s.match_prefix(input)),
            Regex::Concat(r, s) => {
                let n = r.match_prefix(input)?;
                let m = s.match_prefix(&input[n..])?;
                Some(n + m)
            }
            Regex::Star(r) => {
                let mut pos = 0;
                loop {
                    match r.match_prefix(&input[pos..]) {
                        Some(0) | None => break,
                        Some(n) => pos += n,
                    }
                }
                Some(pos)
            }
        }
    }
}
/// A simple Turing machine definition.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TuringMachine {
    /// States.
    pub states: Vec<String>,
    /// Tape alphabet.
    pub tape_alphabet: Vec<char>,
    /// Input alphabet (subset of tape alphabet).
    pub input_alphabet: Vec<char>,
    /// Initial state.
    pub initial_state: String,
    /// Accept state.
    pub accept_state: String,
    /// Reject state.
    pub reject_state: String,
    /// Blank symbol.
    pub blank: char,
    /// Transitions.
    pub transitions: Vec<TuringTransition>,
}
#[allow(dead_code)]
impl TuringMachine {
    /// Creates a Turing machine.
    pub fn new(initial: &str, accept: &str, reject: &str, blank: char) -> Self {
        TuringMachine {
            states: vec![initial.to_string(), accept.to_string(), reject.to_string()],
            tape_alphabet: vec![blank],
            input_alphabet: Vec::new(),
            initial_state: initial.to_string(),
            accept_state: accept.to_string(),
            reject_state: reject.to_string(),
            blank,
            transitions: Vec::new(),
        }
    }
    /// Adds a transition.
    pub fn add_transition(
        &mut self,
        state: &str,
        read: char,
        new_state: &str,
        write: char,
        dir: TapeDir,
    ) {
        self.transitions.push(TuringTransition {
            state: state.to_string(),
            read_symbol: read,
            new_state: new_state.to_string(),
            write_symbol: write,
            direction: dir,
        });
    }
    /// Returns the number of transitions.
    pub fn num_transitions(&self) -> usize {
        self.transitions.len()
    }
    /// Returns the transition function for (state, symbol), if defined.
    pub fn get_transition(&self, state: &str, symbol: char) -> Option<&TuringTransition> {
        self.transitions
            .iter()
            .find(|t| t.state == state && t.read_symbol == symbol)
    }
    /// Checks if the TM halts on empty tape by simulating (up to max_steps).
    pub fn halts_on_empty(&self, max_steps: usize) -> Option<bool> {
        let mut tape: Vec<char> = vec![self.blank; 10];
        let mut head: usize = 5;
        let mut state = self.initial_state.clone();
        for _ in 0..max_steps {
            if state == self.accept_state {
                return Some(true);
            }
            if state == self.reject_state {
                return Some(false);
            }
            let sym = tape[head];
            match self.get_transition(&state, sym) {
                None => return Some(false),
                Some(t) => {
                    tape[head] = t.write_symbol;
                    state = t.new_state.clone();
                    match t.direction {
                        TapeDir::Left => {
                            if head > 0 {
                                head -= 1;
                            }
                        }
                        TapeDir::Right => {
                            head += 1;
                            if head >= tape.len() {
                                tape.push(self.blank);
                            }
                        }
                        TapeDir::Stay => {}
                    }
                }
            }
        }
        None
    }
}
/// Nondeterministic Finite Automaton.
///
/// `delta[q][c]` is the set of states reachable from `q` on symbol index `c`.
/// Epsilon transitions are not modelled; use subset construction via
/// [`nfa_to_dfa`] for conversion.
pub struct Nfa {
    /// Number of states.
    pub states: usize,
    /// Ordered alphabet.
    pub alphabet: Vec<char>,
    /// Transition relation: `delta[state][symbol_index]` = list of next states.
    pub delta: Vec<Vec<Vec<usize>>>,
    /// Start state index.
    pub start: usize,
    /// `accept[q]` is true iff state `q` is accepting.
    pub accept: Vec<bool>,
}
impl Nfa {
    /// Returns `true` iff the NFA accepts `input` (subset construction on the fly).
    pub fn accepts(&self, input: &str) -> bool {
        let mut current: HashSet<usize> = HashSet::new();
        current.insert(self.start);
        for ch in input.chars() {
            let idx = match self.alphabet.iter().position(|&c| c == ch) {
                Some(i) => i,
                None => return false,
            };
            let mut next: HashSet<usize> = HashSet::new();
            for &q in &current {
                for &r in &self.delta[q][idx] {
                    next.insert(r);
                }
            }
            current = next;
        }
        current.iter().any(|&q| self.accept[q])
    }
}
/// Represents a context-free grammar.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ContextFreeGrammar {
    /// Start symbol.
    pub start: String,
    /// Non-terminals.
    pub nonterminals: Vec<String>,
    /// Terminals.
    pub terminals: Vec<String>,
    /// Productions: (lhs, rhs_symbols).
    pub productions: Vec<(String, Vec<String>)>,
}
#[allow(dead_code)]
impl ContextFreeGrammar {
    /// Creates a CFG.
    pub fn new(start: &str) -> Self {
        ContextFreeGrammar {
            start: start.to_string(),
            nonterminals: vec![start.to_string()],
            terminals: Vec::new(),
            productions: Vec::new(),
        }
    }
    /// Adds a non-terminal.
    pub fn add_nonterminal(&mut self, nt: &str) {
        if !self.nonterminals.contains(&nt.to_string()) {
            self.nonterminals.push(nt.to_string());
        }
    }
    /// Adds a terminal.
    pub fn add_terminal(&mut self, t: &str) {
        if !self.terminals.contains(&t.to_string()) {
            self.terminals.push(t.to_string());
        }
    }
    /// Adds a production A → α.
    pub fn add_production(&mut self, lhs: &str, rhs: Vec<&str>) {
        let rhs_owned: Vec<String> = rhs.iter().map(|s| s.to_string()).collect();
        self.productions.push((lhs.to_string(), rhs_owned));
    }
    /// Returns the number of productions.
    pub fn num_productions(&self) -> usize {
        self.productions.len()
    }
    /// Checks if the grammar is in Chomsky Normal Form (each rule is A→BC or A→a).
    pub fn is_cnf(&self) -> bool {
        self.productions.iter().all(|(_, rhs)| {
            rhs.len() == 2 && rhs.iter().all(|s| self.nonterminals.contains(s))
                || rhs.len() == 1 && rhs.iter().all(|s| self.terminals.contains(s))
        })
    }
    /// Checks if the grammar generates the empty string (has S → ε production).
    pub fn generates_empty(&self) -> bool {
        self.productions
            .iter()
            .any(|(lhs, rhs)| lhs == &self.start && rhs.is_empty())
    }
    /// Pumping lemma constant: 2^|G| (exponential in grammar size).
    pub fn pumping_length_bound(&self) -> usize {
        2usize.saturating_pow(self.productions.len() as u32)
    }
}
/// A Turing machine transition.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TuringTransition {
    /// Current state.
    pub state: String,
    /// Current symbol read.
    pub read_symbol: char,
    /// New state.
    pub new_state: String,
    /// Symbol to write.
    pub write_symbol: char,
    /// Direction to move.
    pub direction: TapeDir,
}
/// Represents a pushdown automaton (PDA).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PushdownAutomaton {
    /// States.
    pub states: Vec<String>,
    /// Input alphabet.
    pub input_alphabet: Vec<char>,
    /// Stack alphabet.
    pub stack_alphabet: Vec<char>,
    /// Initial state.
    pub initial_state: String,
    /// Initial stack symbol.
    pub initial_stack: char,
    /// Accept states.
    pub accept_states: Vec<String>,
    /// Transitions: (state, input, stack_top) → (new_state, push_symbols).
    pub transitions: Vec<(String, Option<char>, char, String, Vec<char>)>,
}
#[allow(dead_code)]
impl PushdownAutomaton {
    /// Creates a PDA.
    pub fn new(initial: &str, init_stack: char) -> Self {
        PushdownAutomaton {
            states: vec![initial.to_string()],
            input_alphabet: Vec::new(),
            stack_alphabet: vec![init_stack],
            initial_state: initial.to_string(),
            initial_stack: init_stack,
            accept_states: Vec::new(),
            transitions: Vec::new(),
        }
    }
    /// Adds a state.
    pub fn add_state(&mut self, state: &str) {
        if !self.states.contains(&state.to_string()) {
            self.states.push(state.to_string());
        }
    }
    /// Adds an accept state.
    pub fn add_accept_state(&mut self, state: &str) {
        self.add_state(state);
        if !self.accept_states.contains(&state.to_string()) {
            self.accept_states.push(state.to_string());
        }
    }
    /// Adds a transition.
    pub fn add_transition(
        &mut self,
        from: &str,
        input: Option<char>,
        stack_top: char,
        to: &str,
        push: Vec<char>,
    ) {
        self.transitions
            .push((from.to_string(), input, stack_top, to.to_string(), push));
    }
    /// Returns the number of states.
    pub fn num_states(&self) -> usize {
        self.states.len()
    }
    /// Checks if PDA is deterministic (each (state, input, stack) has at most one transition).
    pub fn is_deterministic(&self) -> bool {
        for t in &self.transitions {
            let count = self
                .transitions
                .iter()
                .filter(|t2| t2.0 == t.0 && t2.1 == t.1 && t2.2 == t.2)
                .count();
            if count > 1 {
                return false;
            }
        }
        true
    }
}
/// Represents a regular expression (simplified, structural).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum RegexExt {
    /// Empty language.
    Empty,
    /// Epsilon (empty string).
    Epsilon,
    /// Single character.
    Char(char),
    /// Concatenation.
    Concat(Box<RegexExt>, Box<RegexExt>),
    /// Union (alternation).
    Union(Box<RegexExt>, Box<RegexExt>),
    /// Kleene star.
    Star(Box<RegexExt>),
}
#[allow(dead_code)]
impl RegexExt {
    /// Creates a single-character regex.
    pub fn ch(c: char) -> Self {
        RegexExt::Char(c)
    }
    /// Creates a concatenation.
    pub fn concat(r1: RegexExt, r2: RegexExt) -> Self {
        RegexExt::Concat(Box::new(r1), Box::new(r2))
    }
    /// Creates a union.
    pub fn union(r1: RegexExt, r2: RegexExt) -> Self {
        RegexExt::Union(Box::new(r1), Box::new(r2))
    }
    /// Creates Kleene star.
    pub fn star(r: RegexExt) -> Self {
        RegexExt::Star(Box::new(r))
    }
    /// Checks if the regex accepts the empty string.
    pub fn accepts_empty(&self) -> bool {
        match self {
            RegexExt::Empty => false,
            RegexExt::Epsilon => true,
            RegexExt::Char(_) => false,
            RegexExt::Concat(r1, r2) => r1.accepts_empty() && r2.accepts_empty(),
            RegexExt::Union(r1, r2) => r1.accepts_empty() || r2.accepts_empty(),
            RegexExt::Star(_) => true,
        }
    }
    /// Checks if the regex is the empty language.
    pub fn is_empty_language(&self) -> bool {
        matches!(self, RegexExt::Empty)
    }
    /// Computes the Brzozowski derivative with respect to character c.
    pub fn derivative(&self, c: char) -> RegexExt {
        match self {
            RegexExt::Empty | RegexExt::Epsilon => RegexExt::Empty,
            RegexExt::Char(d) => {
                if *d == c {
                    RegexExt::Epsilon
                } else {
                    RegexExt::Empty
                }
            }
            RegexExt::Concat(r1, r2) => {
                let d1 = RegexExt::concat(r1.derivative(c), *r2.clone());
                if r1.accepts_empty() {
                    RegexExt::union(d1, r2.derivative(c))
                } else {
                    d1
                }
            }
            RegexExt::Union(r1, r2) => RegexExt::union(r1.derivative(c), r2.derivative(c)),
            RegexExt::Star(r) => RegexExt::concat(r.derivative(c), RegexExt::Star(r.clone())),
        }
    }
    /// Checks if a string is accepted by the regex using Brzozowski derivatives.
    pub fn accepts(&self, s: &str) -> bool {
        let mut current = self.clone();
        for c in s.chars() {
            current = current.derivative(c);
        }
        current.accepts_empty()
    }
}
/// Nondeterministic Pushdown Automaton.
///
/// Accepts by empty stack.  Transitions are of the form:
/// `(state, input_symbol_opt, stack_top_opt) → [(next_state, push_symbols)]`
///
/// - `input_symbol_opt = None` denotes an epsilon transition on input.
/// - `stack_top_opt = None` means the top of stack is not checked / consumed.
/// - `push_symbols` is what is pushed onto the stack (bottom-first in the vec).
///   An empty `push_symbols` means the top is popped and nothing is pushed.
///
/// Stack alphabet indices are separate from input alphabet indices.
pub struct PdaSimulator {
    /// Number of states.
    pub states: usize,
    /// Input alphabet.
    pub alphabet: Vec<char>,
    /// Stack alphabet (symbol 0 is typically the initial stack symbol).
    pub stack_alphabet: Vec<String>,
    /// Transitions: `(from_state, input_opt, stack_top_opt, to_state, push_vec)`.
    pub transitions: Vec<(usize, Option<usize>, Option<usize>, usize, Vec<usize>)>,
    /// Start state.
    pub start: usize,
    /// Initial stack symbol index pushed at the start.
    pub initial_stack: usize,
    /// Accept states (used for acceptance by final state).
    pub accept: Vec<bool>,
}
impl PdaSimulator {
    /// Returns `true` iff the PDA accepts `input` (by final state, BFS).
    pub fn accepts(&self, input: &str) -> bool {
        let input_chars: Vec<usize> = input
            .chars()
            .map(|ch| self.alphabet.iter().position(|&c| c == ch))
            .collect::<Option<Vec<_>>>()
            .unwrap_or_default();
        let init = PdaConfig {
            state: self.start,
            stack: vec![self.initial_stack],
        };
        let mut frontier: VecDeque<(usize, PdaConfig)> = VecDeque::new();
        frontier.push_back((0, init));
        let mut visited: HashSet<(usize, usize, Vec<usize>)> = HashSet::new();
        while let Some((pos, cfg)) = frontier.pop_front() {
            let key = (pos, cfg.state, cfg.stack.clone());
            if !visited.insert(key) {
                continue;
            }
            if pos == input_chars.len() && self.accept[cfg.state] {
                return true;
            }
            for &(from, inp_opt, stk_opt, to, ref push) in &self.transitions {
                if from != cfg.state {
                    continue;
                }
                let new_pos = match inp_opt {
                    Some(sym) => {
                        if pos < input_chars.len() && input_chars[pos] == sym {
                            pos + 1
                        } else {
                            continue;
                        }
                    }
                    None => pos,
                };
                let mut new_stack = cfg.stack.clone();
                if let Some(top_sym) = stk_opt {
                    match new_stack.last() {
                        Some(&t) if t == top_sym => {
                            new_stack.pop();
                        }
                        _ => continue,
                    }
                }
                for &sym in push.iter().rev() {
                    new_stack.push(sym);
                }
                frontier.push_back((
                    new_pos,
                    PdaConfig {
                        state: to,
                        stack: new_stack,
                    },
                ));
            }
        }
        false
    }
}
/// A configuration of a nondeterministic pushdown automaton (NPDA).
#[derive(Clone, Debug)]
struct PdaConfig {
    state: usize,
    stack: Vec<usize>,
}
/// Context-Free Grammar in the form (V, Σ, R, S).
///
/// Each rule is a pair `(lhs_variable, rhs_symbols)` where each symbol in
/// `rhs_symbols` is either a variable name or a terminal written as a
/// single-character string.
pub struct Cfg {
    /// Non-terminal variables.
    pub variables: Vec<String>,
    /// Terminal symbols.
    pub terminals: Vec<char>,
    /// Production rules: `(lhs, rhs)`.
    pub rules: Vec<(String, Vec<String>)>,
    /// Start variable.
    pub start: String,
}
impl Cfg {
    /// Returns `true` iff every production is in Chomsky Normal Form:
    ///
    /// - `A → B C` (two variables), or
    /// - `A → a` (one terminal), or
    /// - `S → ε` (start symbol produces empty string, only allowed for start).
    pub fn is_in_cnf(&self) -> bool {
        let var_set: HashSet<&str> = self.variables.iter().map(String::as_str).collect();
        let term_set: HashSet<char> = self.terminals.iter().copied().collect();
        for (lhs, rhs) in &self.rules {
            match rhs.as_slice() {
                [] => {
                    if lhs != &self.start {
                        return false;
                    }
                }
                [a] => {
                    if a.chars().count() != 1
                        || !term_set.contains(
                            &a.chars().next().expect(
                                "a has exactly one char: just checked by chars().count() == 1",
                            ),
                        )
                    {
                        return false;
                    }
                }
                [b, c] => {
                    if !var_set.contains(b.as_str()) || !var_set.contains(c.as_str()) {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        true
    }
}
/// A single-tape deterministic Turing machine.
///
/// The tape is modelled as a `Vec<usize>` (symbol indices) that grows
/// automatically; the blank symbol has index 0.
///
/// A transition `(state, read_sym) → (next_state, write_sym, direction)`
/// is stored in the `delta` map.
///
/// The machine halts when no transition matches the current `(state, sym)` pair.
/// It accepts if the halting state is an accept state.
pub struct TuringMachineSimulator {
    /// Number of states (0..states).
    pub states: usize,
    /// Tape alphabet (index 0 = blank symbol).
    pub tape_alphabet: Vec<String>,
    /// Transition function: `(state, tape_sym) → (next_state, write_sym, direction)`.
    pub delta: HashMap<(usize, usize), (usize, usize, TmDirection)>,
    /// Start state index.
    pub start: usize,
    /// Accept state indices.
    pub accept: HashSet<usize>,
    /// Reject state indices.
    pub reject: HashSet<usize>,
    /// Maximum steps before giving up (0 = unlimited, beware infinite loops).
    pub step_limit: usize,
}
impl TuringMachineSimulator {
    /// Run the TM on `input`, returning `true` for accept, `false` for reject or
    /// step-limit exceeded.
    ///
    /// Input symbols are looked up in `tape_alphabet[1..]` (index 0 is blank).
    pub fn run(&self, input: &str) -> bool {
        let mut tape: Vec<usize> = input
            .chars()
            .map(|ch| {
                self.tape_alphabet
                    .iter()
                    .position(|s| s.len() == 1 && s.starts_with(ch))
                    .unwrap_or(0)
            })
            .collect();
        if tape.is_empty() {
            tape.push(0);
        }
        let mut head: usize = 0;
        let mut state = self.start;
        let mut steps = 0usize;
        loop {
            if self.accept.contains(&state) {
                return true;
            }
            if self.reject.contains(&state) {
                return false;
            }
            if self.step_limit > 0 && steps >= self.step_limit {
                return false;
            }
            let sym = if head < tape.len() { tape[head] } else { 0 };
            let (next_state, write_sym, dir) = match self.delta.get(&(state, sym)) {
                Some(&t) => t,
                None => return false,
            };
            if head >= tape.len() {
                tape.resize(head + 1, 0);
            }
            tape[head] = write_sym;
            match dir {
                TmDirection::Left => {
                    if head > 0 {
                        head -= 1;
                    }
                }
                TmDirection::Right => {
                    head += 1;
                    if head >= tape.len() {
                        tape.push(0);
                    }
                }
                TmDirection::Stay => {}
            }
            state = next_state;
            steps += 1;
        }
    }
    /// Returns the tape contents as a string after running, up to the first blank.
    pub fn run_and_read(&self, input: &str) -> (bool, String) {
        let accepted = self.run(input);
        let mut tape: Vec<usize> = input
            .chars()
            .map(|ch| {
                self.tape_alphabet
                    .iter()
                    .position(|s| s.len() == 1 && s.starts_with(ch))
                    .unwrap_or(0)
            })
            .collect();
        if tape.is_empty() {
            tape.push(0);
        }
        let mut head = 0usize;
        let mut state = self.start;
        let mut steps = 0usize;
        loop {
            if self.accept.contains(&state) || self.reject.contains(&state) {
                break;
            }
            if self.step_limit > 0 && steps >= self.step_limit {
                break;
            }
            let sym = if head < tape.len() { tape[head] } else { 0 };
            let (next_state, write_sym, dir) = match self.delta.get(&(state, sym)) {
                Some(&t) => t,
                None => break,
            };
            if head >= tape.len() {
                tape.resize(head + 1, 0);
            }
            tape[head] = write_sym;
            match dir {
                TmDirection::Left => {
                    if head > 0 {
                        head -= 1;
                    }
                }
                TmDirection::Right => {
                    head += 1;
                    if head >= tape.len() {
                        tape.push(0);
                    }
                }
                TmDirection::Stay => {}
            }
            state = next_state;
            steps += 1;
        }
        let last_nonblank = tape
            .iter()
            .rposition(|&s| s != 0)
            .map(|i| i + 1)
            .unwrap_or(0);
        let output: String = tape[..last_nonblank]
            .iter()
            .map(|&s| {
                self.tape_alphabet
                    .get(s)
                    .map(|name| {
                        if name.len() == 1 {
                            name.chars()
                                .next()
                                .expect("name has length 1: just checked")
                        } else {
                            '?'
                        }
                    })
                    .unwrap_or('?')
            })
            .collect();
        (accepted, output)
    }
}
/// Büchi automaton over infinite words.
///
/// An infinite run over `word[0], word[1], word[2], ...` (represented as a
/// finite `lasso` = `prefix + cycle`) is accepting iff it visits some
/// accepting state infinitely often.
///
/// For simulation purposes we accept a finite representation of an infinite
/// word as `(prefix, cycle)` where the cycle repeats forever.
pub struct BuchiAutomaton {
    /// Number of states.
    pub states: usize,
    /// Input alphabet.
    pub alphabet: Vec<char>,
    /// Transition relation: `delta[state][symbol_index]` = list of next states.
    pub delta: Vec<Vec<Vec<usize>>>,
    /// Start state.
    pub start: usize,
    /// `accept[q]` is true iff state `q` is an accepting (Büchi) state.
    pub accept: Vec<bool>,
}
impl BuchiAutomaton {
    /// Returns `true` iff the Büchi automaton accepts the lasso word
    /// `prefix ++ cycle^ω` (cycle repeated infinitely).
    ///
    /// Uses the standard algorithm:
    /// 1. Compute all states reachable after reading `prefix + cycle^k` (k ≥ 0) to
    ///    convergence — these are the candidate cycle-entry states.
    /// 2. For each candidate entry state `q`, check whether reading `cycle` from
    ///    `q` can return to `q` while passing through an accepting state.
    pub fn accepts_lasso(&self, prefix: &str, cycle: &str) -> bool {
        if cycle.is_empty() {
            return false;
        }
        let to_sym = |s: &str| -> Option<Vec<usize>> {
            s.chars()
                .map(|ch| self.alphabet.iter().position(|&c| c == ch))
                .collect()
        };
        let pre_syms = match to_sym(prefix) {
            Some(v) => v,
            None => return false,
        };
        let cyc_syms = match to_sym(cycle) {
            Some(v) => v,
            None => return false,
        };
        let advance = |states: &HashSet<usize>, syms: &[usize]| -> HashSet<usize> {
            let mut cur = states.clone();
            for &sym in syms {
                let mut nxt: HashSet<usize> = HashSet::new();
                for &q in &cur {
                    for &r in &self.delta[q][sym] {
                        nxt.insert(r);
                    }
                }
                cur = nxt;
            }
            cur
        };
        let mut after_pre: HashSet<usize> = HashSet::new();
        after_pre.insert(self.start);
        after_pre = advance(&after_pre, &pre_syms);
        let mut cycle_entries = after_pre.clone();
        loop {
            let extended = advance(&cycle_entries, &cyc_syms);
            let old_size = cycle_entries.len();
            cycle_entries.extend(extended);
            if cycle_entries.len() == old_size {
                break;
            }
        }
        for &entry in &cycle_entries {
            let mut states_with_accept: HashSet<usize> = HashSet::new();
            let mut states_without_accept: HashSet<usize> = HashSet::new();
            if self.accept[entry] {
                states_with_accept.insert(entry);
            } else {
                states_without_accept.insert(entry);
            }
            for &sym in &cyc_syms {
                let mut nwa: HashSet<usize> = HashSet::new();
                let mut nwoa: HashSet<usize> = HashSet::new();
                for &q in states_with_accept
                    .iter()
                    .chain(states_without_accept.iter())
                {
                    let saw = states_with_accept.contains(&q);
                    for &r in &self.delta[q][sym] {
                        if saw || self.accept[r] {
                            nwa.insert(r);
                        } else {
                            nwoa.insert(r);
                        }
                    }
                }
                states_with_accept = nwa;
                states_without_accept = nwoa;
            }
            if states_with_accept.contains(&entry) {
                return true;
            }
        }
        false
    }
}
