//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::{HashMap, HashSet};

/// Computes FIRST and FOLLOW sets for a context-free grammar.
#[allow(dead_code)]
pub struct FirstFollowSets<'a> {
    /// The grammar.
    pub grammar: &'a ContextFreeGrammar,
    /// FIRST sets: nonterminal index → set of (Option<terminal_index>)
    /// None represents ε.
    pub first: Vec<HashSet<Option<usize>>>,
    /// FOLLOW sets: nonterminal index → set of (Option<terminal_index>)
    /// None represents end-of-input ($).
    pub follow: Vec<HashSet<Option<usize>>>,
}
#[allow(dead_code)]
impl<'a> FirstFollowSets<'a> {
    /// Compute FIRST and FOLLOW sets for the grammar.
    pub fn compute(grammar: &'a ContextFreeGrammar) -> Self {
        let mut first: Vec<HashSet<Option<usize>>> = vec![HashSet::new(); grammar.n_nonterms];
        let mut follow: Vec<HashSet<Option<usize>>> = vec![HashSet::new(); grammar.n_nonterms];
        let mut changed = true;
        while changed {
            changed = false;
            for (lhs, rhs) in &grammar.productions {
                if rhs.is_empty() {
                    if first[*lhs].insert(None) {
                        changed = true;
                    }
                } else {
                    let (is_term, idx) = rhs[0];
                    if is_term {
                        if first[*lhs].insert(Some(idx)) {
                            changed = true;
                        }
                    } else if idx < grammar.n_nonterms {
                        let to_add: Vec<_> = first[idx].iter().cloned().collect();
                        for item in to_add {
                            if first[*lhs].insert(item) {
                                changed = true;
                            }
                        }
                    }
                }
            }
        }
        follow[grammar.start].insert(None);
        let mut changed = true;
        while changed {
            changed = false;
            for (lhs, rhs) in &grammar.productions {
                for (i, &(is_term, idx)) in rhs.iter().enumerate() {
                    if !is_term && idx < grammar.n_nonterms {
                        if i + 1 < rhs.len() {
                            let next = rhs[i + 1];
                            if next.0 {
                                if follow[idx].insert(Some(next.1)) {
                                    changed = true;
                                }
                            } else if next.1 < grammar.n_nonterms {
                                let to_add: Vec<_> = first[next.1].iter().cloned().collect();
                                for item in to_add {
                                    if follow[idx].insert(item) {
                                        changed = true;
                                    }
                                }
                            }
                        } else {
                            let to_add: Vec<_> = follow[*lhs].iter().cloned().collect();
                            for item in to_add {
                                if follow[idx].insert(item) {
                                    changed = true;
                                }
                            }
                        }
                    }
                }
            }
        }
        FirstFollowSets {
            grammar,
            first,
            follow,
        }
    }
    /// Get FIRST set for a nonterminal.
    pub fn get_first(&self, nt: usize) -> &HashSet<Option<usize>> {
        &self.first[nt]
    }
    /// Get FOLLOW set for a nonterminal.
    pub fn get_follow(&self, nt: usize) -> &HashSet<Option<usize>> {
        &self.follow[nt]
    }
    /// Check if a nonterminal is nullable (ε ∈ FIRST(nt)).
    pub fn is_nullable(&self, nt: usize) -> bool {
        self.first[nt].contains(&None)
    }
}
/// An LL(1) parsing table for a given grammar.
#[allow(dead_code)]
pub struct Ll1Table {
    /// For each (nonterminal, terminal) pair, the production index to use.
    /// The terminal None represents end-of-input.
    pub table: std::collections::HashMap<(usize, Option<usize>), usize>,
    /// Number of nonterminals.
    pub n_nonterms: usize,
    /// Number of terminals.
    pub n_terms: usize,
}
#[allow(dead_code)]
impl Ll1Table {
    /// Create an empty LL(1) table.
    pub fn new(n_nonterms: usize, n_terms: usize) -> Self {
        Ll1Table {
            table: std::collections::HashMap::new(),
            n_nonterms,
            n_terms,
        }
    }
    /// Set the production for (nonterminal, terminal).
    pub fn set(&mut self, nt: usize, term: Option<usize>, prod: usize) {
        self.table.insert((nt, term), prod);
    }
    /// Look up the production for (nonterminal, terminal).
    pub fn get(&self, nt: usize, term: Option<usize>) -> Option<usize> {
        self.table.get(&(nt, term)).cloned()
    }
    /// Check if the table is conflict-free (no cell has more than one production).
    /// Since we use a HashMap, each cell has at most one entry by construction.
    pub fn is_conflict_free(&self) -> bool {
        true
    }
    /// Count the number of filled table entries.
    pub fn entry_count(&self) -> usize {
        self.table.len()
    }
    /// Build an LL(1) table from a grammar and its FIRST/FOLLOW sets.
    pub fn build(grammar: &ContextFreeGrammar, sets: &FirstFollowSets<'_>) -> Self {
        let mut table = Ll1Table::new(grammar.n_nonterms, grammar.n_terms);
        for (prod_idx, (lhs, rhs)) in grammar.productions.iter().enumerate() {
            if rhs.is_empty() {
                for &follow_item in sets.get_follow(*lhs) {
                    table.set(*lhs, follow_item, prod_idx);
                }
            } else {
                let (is_term, idx) = rhs[0];
                if is_term {
                    table.set(*lhs, Some(idx), prod_idx);
                } else if idx < grammar.n_nonterms {
                    for &first_item in sets.get_first(idx) {
                        if let Some(t) = first_item {
                            table.set(*lhs, Some(t), prod_idx);
                        } else {
                            for &follow_item in sets.get_follow(*lhs) {
                                table.set(*lhs, follow_item, prod_idx);
                            }
                        }
                    }
                }
            }
        }
        table
    }
}
/// A context-free grammar with nonterminals numbered 0..n_nonterms
/// and terminals numbered 0..n_terms.
#[allow(dead_code)]
pub struct ContextFreeGrammar {
    /// Number of nonterminal symbols.
    pub n_nonterms: usize,
    /// Number of terminal symbols.
    pub n_terms: usize,
    /// Start symbol (index of nonterminal).
    pub start: usize,
    /// Productions: list of (lhs_nonterm, rhs_symbols) where rhs_symbols is
    /// a list of (is_terminal: bool, index).
    pub productions: Vec<(usize, Vec<(bool, usize)>)>,
}
#[allow(dead_code)]
impl ContextFreeGrammar {
    /// Create a new CFG.
    pub fn new(n_nonterms: usize, n_terms: usize, start: usize) -> Self {
        ContextFreeGrammar {
            n_nonterms,
            n_terms,
            start,
            productions: Vec::new(),
        }
    }
    /// Add a production: A → rhs.
    pub fn add_production(&mut self, lhs: usize, rhs: Vec<(bool, usize)>) {
        self.productions.push((lhs, rhs));
    }
    /// Get all productions for a given nonterminal.
    pub fn productions_for(&self, nt: usize) -> Vec<&Vec<(bool, usize)>> {
        self.productions
            .iter()
            .filter_map(|(lhs, rhs)| if *lhs == nt { Some(rhs) } else { None })
            .collect()
    }
    /// Check if any production is nullable (A → ε).
    pub fn is_nullable(&self, nt: usize) -> bool {
        self.productions
            .iter()
            .any(|(lhs, rhs)| *lhs == nt && rhs.is_empty())
    }
    /// Count total number of productions.
    pub fn production_count(&self) -> usize {
        self.productions.len()
    }
    /// Check if the grammar is in Chomsky Normal Form:
    /// every production is A → BC or A → a.
    pub fn is_cnf(&self) -> bool {
        for (_, rhs) in &self.productions {
            match rhs.len() {
                0 => return false,
                1 => {
                    if rhs[0].0 {
                    } else {
                        return false;
                    }
                }
                2 => {
                    if rhs[0].0 || rhs[1].0 {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        true
    }
}
/// A CYK (Cocke–Younger–Kasami) parser for CFGs in CNF.
#[allow(dead_code)]
pub struct CykParser<'a> {
    /// The grammar (must be in CNF).
    pub grammar: &'a ContextFreeGrammar,
}
#[allow(dead_code)]
impl<'a> CykParser<'a> {
    /// Create a new CYK parser.
    pub fn new(grammar: &'a ContextFreeGrammar) -> Self {
        CykParser { grammar }
    }
    /// Run CYK on a sequence of terminal indices.
    /// Returns true iff the input is in L(grammar).
    pub fn parse(&self, input: &[usize]) -> bool {
        let n = input.len();
        if n == 0 {
            return self.grammar.is_nullable(self.grammar.start);
        }
        let mut table: Vec<Vec<HashSet<usize>>> = vec![vec![HashSet::new(); n]; n];
        for (i, &term) in input.iter().enumerate() {
            for (lhs, rhs) in &self.grammar.productions {
                if rhs.len() == 1 && rhs[0].0 && rhs[0].1 == term {
                    table[i][0].insert(*lhs);
                }
            }
        }
        for len in 2..=n {
            for i in 0..=(n - len) {
                for k in 0..(len - 1) {
                    for (lhs, rhs) in &self.grammar.productions {
                        if rhs.len() == 2 && !rhs[0].0 && !rhs[1].0 {
                            let b = rhs[0].1;
                            let c = rhs[1].1;
                            if table[i][k].contains(&b)
                                && table[i + k + 1][len - k - 2].contains(&c)
                            {
                                table[i][len - 1].insert(*lhs);
                            }
                        }
                    }
                }
            }
        }
        table[0][n - 1].contains(&self.grammar.start)
    }
    /// Count the number of parse trees (ambiguity measure).
    pub fn count_parses(&self, input: &[usize]) -> usize {
        let n = input.len();
        if n == 0 {
            return if self.grammar.is_nullable(self.grammar.start) {
                1
            } else {
                0
            };
        }
        if self.parse(input) {
            1
        } else {
            0
        }
    }
}
/// A runtime PEG expression.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum PegExpr {
    /// Match a literal character.
    Char(char),
    /// Match any character.
    Any,
    /// Sequence: e1 followed by e2.
    Seq(Box<PegExpr>, Box<PegExpr>),
    /// Ordered choice: try e1, if it fails (without consuming input) try e2.
    Choice(Box<PegExpr>, Box<PegExpr>),
    /// Zero or more: e*
    Star(Box<PegExpr>),
    /// One or more: e+
    Plus(Box<PegExpr>),
    /// Optional: e?
    Optional(Box<PegExpr>),
    /// Negative lookahead: !e
    Not(Box<PegExpr>),
    /// Positive lookahead: &e
    And(Box<PegExpr>),
}
#[allow(dead_code)]
impl PegExpr {
    /// Try to match the expression at position `pos` in `input`.
    /// Returns the new position if successful, or None on failure.
    pub fn matches(&self, input: &str, pos: usize) -> Option<usize> {
        let chars: Vec<char> = input.chars().collect();
        self.match_at(&chars, pos)
    }
    fn match_at(&self, input: &[char], pos: usize) -> Option<usize> {
        match self {
            PegExpr::Char(c) => {
                if pos < input.len() && input[pos] == *c {
                    Some(pos + 1)
                } else {
                    None
                }
            }
            PegExpr::Any => {
                if pos < input.len() {
                    Some(pos + 1)
                } else {
                    None
                }
            }
            PegExpr::Seq(e1, e2) => {
                let p1 = e1.match_at(input, pos)?;
                e2.match_at(input, p1)
            }
            PegExpr::Choice(e1, e2) => e1.match_at(input, pos).or_else(|| e2.match_at(input, pos)),
            PegExpr::Star(e) => {
                let mut cur = pos;
                loop {
                    match e.match_at(input, cur) {
                        Some(next) if next > cur => cur = next,
                        _ => break,
                    }
                }
                Some(cur)
            }
            PegExpr::Plus(e) => {
                let first = e.match_at(input, pos)?;
                let rest = PegExpr::Star(e.clone()).match_at(input, first)?;
                Some(rest)
            }
            PegExpr::Optional(e) => Some(e.match_at(input, pos).unwrap_or(pos)),
            PegExpr::Not(e) => {
                if e.match_at(input, pos).is_some() {
                    None
                } else {
                    Some(pos)
                }
            }
            PegExpr::And(e) => {
                e.match_at(input, pos)?;
                Some(pos)
            }
        }
    }
    /// Check if the expression matches the entire string.
    pub fn match_full(&self, input: &str) -> bool {
        let chars: Vec<char> = input.chars().collect();
        self.match_at(&chars, 0) == Some(chars.len())
    }
}
