//! # PrologGoalBuilder - goal_group Methods
//!
//! This module contains method implementations for `PrologGoalBuilder`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PrologTerm;

use super::functions::*;
use super::prologgoalbuilder_type::PrologGoalBuilder;

impl PrologGoalBuilder {
    /// Add a raw goal term.
    #[allow(dead_code)]
    pub fn goal(mut self, t: PrologTerm) -> Self {
        self.goals.push(t);
        self
    }
    /// Add a `call(F)` goal.
    #[allow(dead_code)]
    pub fn call(self, f: PrologTerm) -> Self {
        self.goal(compound("call", vec![f]))
    }
    /// Add a `call(F, Arg)` goal.
    #[allow(dead_code)]
    pub fn call1(self, f: PrologTerm, arg: PrologTerm) -> Self {
        self.goal(compound("call", vec![f, arg]))
    }
    /// Add a `call(F, A, B)` goal.
    #[allow(dead_code)]
    pub fn call2(self, f: PrologTerm, a: PrologTerm, b: PrologTerm) -> Self {
        self.goal(compound("call", vec![f, a, b]))
    }
    /// Add a `write(X)` goal.
    #[allow(dead_code)]
    pub fn write(self, x: PrologTerm) -> Self {
        self.goal(compound("write", vec![x]))
    }
    /// Add a `writeln(X)` goal.
    #[allow(dead_code)]
    pub fn writeln(self, x: PrologTerm) -> Self {
        self.goal(compound("writeln", vec![x]))
    }
    /// Add a `nl` goal (newline).
    #[allow(dead_code)]
    pub fn nl(self) -> Self {
        self.goal(atom("nl"))
    }
    /// Add a `fail` goal.
    #[allow(dead_code)]
    pub fn fail(self) -> Self {
        self.goal(atom("fail"))
    }
    /// Add a `true` goal.
    #[allow(dead_code)]
    pub fn true_goal(self) -> Self {
        self.goal(atom("true"))
    }
    /// Add a cut `!`.
    #[allow(dead_code)]
    pub fn cut(self) -> Self {
        self.goal(PrologTerm::Cut)
    }
    /// Add `X is Expr`.
    #[allow(dead_code)]
    pub fn is(self, x: PrologTerm, expr: PrologTerm) -> Self {
        self.goal(is_eval(x, expr))
    }
    /// Add `X = Y`.
    #[allow(dead_code)]
    pub fn unify(self, x: PrologTerm, y: PrologTerm) -> Self {
        self.goal(unify(x, y))
    }
    /// Add `assert(Clause)`.
    #[allow(dead_code)]
    pub fn assert(self, clause: PrologTerm) -> Self {
        self.goal(compound("assert", vec![clause]))
    }
    /// Add `asserta(Clause)`.
    #[allow(dead_code)]
    pub fn asserta(self, clause: PrologTerm) -> Self {
        self.goal(compound("asserta", vec![clause]))
    }
    /// Add `assertz(Clause)`.
    #[allow(dead_code)]
    pub fn assertz(self, clause: PrologTerm) -> Self {
        self.goal(compound("assertz", vec![clause]))
    }
    /// Add `retract(Clause)`.
    #[allow(dead_code)]
    pub fn retract(self, clause: PrologTerm) -> Self {
        self.goal(compound("retract", vec![clause]))
    }
    /// Add a `between(Low, High, X)` goal.
    #[allow(dead_code)]
    pub fn between(self, lo: PrologTerm, hi: PrologTerm, x: PrologTerm) -> Self {
        self.goal(compound("between", vec![lo, hi, x]))
    }
    /// Add `msort(List, Sorted)`.
    #[allow(dead_code)]
    pub fn msort(self, list: PrologTerm, sorted: PrologTerm) -> Self {
        self.goal(compound("msort", vec![list, sorted]))
    }
    /// Add `sort(List, Sorted)`.
    #[allow(dead_code)]
    pub fn sort(self, list: PrologTerm, sorted: PrologTerm) -> Self {
        self.goal(compound("sort", vec![list, sorted]))
    }
    /// Add `length(List, N)`.
    #[allow(dead_code)]
    pub fn length(self, list: PrologTerm, n: PrologTerm) -> Self {
        self.goal(compound("length", vec![list, n]))
    }
    /// Add `append(A, B, C)`.
    #[allow(dead_code)]
    pub fn append(self, a: PrologTerm, b: PrologTerm, c: PrologTerm) -> Self {
        self.goal(compound("append", vec![a, b, c]))
    }
    /// Add `member(X, List)`.
    #[allow(dead_code)]
    pub fn member(self, x: PrologTerm, list: PrologTerm) -> Self {
        self.goal(compound("member", vec![x, list]))
    }
    /// Add `nth0(N, List, Elem)`.
    #[allow(dead_code)]
    pub fn nth0(self, n: PrologTerm, list: PrologTerm, elem: PrologTerm) -> Self {
        self.goal(compound("nth0", vec![n, list, elem]))
    }
    /// Add `nth1(N, List, Elem)`.
    #[allow(dead_code)]
    pub fn nth1(self, n: PrologTerm, list: PrologTerm, elem: PrologTerm) -> Self {
        self.goal(compound("nth1", vec![n, list, elem]))
    }
    /// Add `last(List, Elem)`.
    #[allow(dead_code)]
    pub fn last(self, list: PrologTerm, elem: PrologTerm) -> Self {
        self.goal(compound("last", vec![list, elem]))
    }
    /// Add `reverse(List, Rev)`.
    #[allow(dead_code)]
    pub fn reverse(self, list: PrologTerm, rev: PrologTerm) -> Self {
        self.goal(compound("reverse", vec![list, rev]))
    }
    /// Add `maplist(Goal, List)`.
    #[allow(dead_code)]
    pub fn maplist1(self, goal: PrologTerm, list: PrologTerm) -> Self {
        self.goal(compound("maplist", vec![goal, list]))
    }
    /// Add `maplist(Goal, List, Result)`.
    #[allow(dead_code)]
    pub fn maplist2(self, goal: PrologTerm, list: PrologTerm, result: PrologTerm) -> Self {
        self.goal(compound("maplist", vec![goal, list, result]))
    }
    /// Add `include(Goal, List, Result)`.
    #[allow(dead_code)]
    pub fn include(self, goal: PrologTerm, list: PrologTerm, result: PrologTerm) -> Self {
        self.goal(compound("include", vec![goal, list, result]))
    }
    /// Add `exclude(Goal, List, Result)`.
    #[allow(dead_code)]
    pub fn exclude(self, goal: PrologTerm, list: PrologTerm, result: PrologTerm) -> Self {
        self.goal(compound("exclude", vec![goal, list, result]))
    }
    /// Add `foldl(Goal, List, V0, V)`.
    #[allow(dead_code)]
    pub fn foldl(self, goal: PrologTerm, list: PrologTerm, v0: PrologTerm, v: PrologTerm) -> Self {
        self.goal(compound("foldl", vec![goal, list, v0, v]))
    }
    /// Add `aggregate_all(count, Goal, Count)`.
    #[allow(dead_code)]
    pub fn aggregate_count(self, goal: PrologTerm, count: PrologTerm) -> Self {
        self.goal(compound("aggregate_all", vec![atom("count"), goal, count]))
    }
    /// Add `format(Fmt, Args)`.
    #[allow(dead_code)]
    pub fn format(self, fmt: PrologTerm, args: PrologTerm) -> Self {
        self.goal(compound("format", vec![fmt, args]))
    }
}
