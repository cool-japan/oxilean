//! # PrologGoalBuilder - number_codes_group Methods
//!
//! This module contains method implementations for `PrologGoalBuilder`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PrologTerm;

use super::functions::*;
use super::prologgoalbuilder_type::PrologGoalBuilder;

impl PrologGoalBuilder {
    /// Add `number_codes(N, Codes)`.
    #[allow(dead_code)]
    pub fn number_codes(self, n: PrologTerm, codes: PrologTerm) -> Self {
        self.goal(compound("number_codes", vec![n, codes]))
    }
    /// Add `atom_string(Atom, String)`.
    #[allow(dead_code)]
    pub fn atom_string(self, a: PrologTerm, s: PrologTerm) -> Self {
        self.goal(compound("atom_string", vec![a, s]))
    }
    /// Add `string_concat(A, B, C)`.
    #[allow(dead_code)]
    pub fn string_concat(self, a: PrologTerm, b: PrologTerm, c: PrologTerm) -> Self {
        self.goal(compound("string_concat", vec![a, b, c]))
    }
    /// Add `read_term(Term, Options)`.
    #[allow(dead_code)]
    pub fn read_term(self, term: PrologTerm, opts: PrologTerm) -> Self {
        self.goal(compound("read_term", vec![term, opts]))
    }
    /// Add `functor(Term, Name, Arity)`.
    #[allow(dead_code)]
    pub fn functor(self, term: PrologTerm, name: PrologTerm, arity: PrologTerm) -> Self {
        self.goal(compound("functor", vec![term, name, arity]))
    }
    /// Add `arg(N, Term, Arg)`.
    #[allow(dead_code)]
    pub fn arg(self, n: PrologTerm, term: PrologTerm, arg: PrologTerm) -> Self {
        self.goal(compound("arg", vec![n, term, arg]))
    }
    /// Add `copy_term(Term, Copy)`.
    #[allow(dead_code)]
    pub fn copy_term(self, term: PrologTerm, copy: PrologTerm) -> Self {
        self.goal(compound("copy_term", vec![term, copy]))
    }
}
