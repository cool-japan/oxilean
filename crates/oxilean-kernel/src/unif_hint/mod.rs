//! Unification Hint System for the OxiLean kernel.
//!
//! Unification hints are user-declared conditional equations that guide the
//! definitional equality checker when structural comparison is stuck.  They
//! are analogous to Lean 4's `@[unif_hint]` attribute.
//!
//! # Overview
//!
//! A [`UnifHint`] is an equation
//! ```text
//! hypotheses ⊢ lhs ≡ rhs
//! ```
//! stored in a [`UnifHintDB`].  When `DefEqChecker::is_def_eq` cannot decide
//! whether two WHNF terms are definitionally equal through the standard rules
//! (structural match, eta, lazy delta), it calls [`try_unif_hints`] which:
//!
//! 1. Queries the DB for every hint whose `(lhs, rhs)` or `(rhs, lhs)` can
//!    be pattern-matched against the stuck pair `(t, s)`.
//! 2. For each candidate, verifies each hypothesis `(lhs_h, rhs_h)` by
//!    applying the matched substitution and recursively calling `is_def_eq`.
//! 3. Returns `true` if any hint fires (all hypotheses satisfied).
//!
//! # Pattern Variables
//!
//! Inside hint patterns, any `Const` node whose name starts with `?` is
//! treated as a *pattern variable* and may match any expression.  The same
//! pattern variable may appear multiple times; it must bind to the same
//! expression everywhere (linear consistency).
//!
//! # Example
//!
//! ```ignore
//! use oxilean_kernel::unif_hint::{UnifHint, UnifHintDB};
//! use oxilean_kernel::{Expr, Literal, Name};
//!
//! let mut db = UnifHintDB::new();
//!
//! // Register: add 0 ?n ≡ ?n
//! let add = Expr::Const(Name::str("add"), vec![]);
//! let zero = Expr::Lit(Literal::Nat(0));
//! let n_pat = Expr::Const(Name::str("?n"), vec![]);
//! let lhs = Expr::App(
//!     Box::new(Expr::App(Box::new(add), Box::new(zero))),
//!     Box::new(n_pat.clone()),
//! );
//! db.add_hint(UnifHint::new(lhs, n_pat));
//! ```

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
