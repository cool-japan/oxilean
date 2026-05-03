//! Prolog-style logic programming engine: SLD resolution, unification, databases.
//!
//! This module provides:
//! - [`LpTerm`] — Prolog terms (atoms, variables, compounds, integers, lists)
//! - [`LpClause`] / [`LpDatabase`] — Horn clauses and clause databases
//! - [`Substitution`] — variable bindings
//! - [`Query`] / [`SolveConfig`] — query and solver configuration
//! - [`resolve`] / [`solve_one`] — SLD resolution engines
//! - [`unify`] / [`apply_subst`] / [`occurs_check`] — unification primitives
//! - [`parse_term`] / [`parse_clause`] / [`term_to_string`] — term I/O
//! - [`load_standard_predicates`] — classic Prolog predicates (member, append, reverse, …)

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
