//! Computability and decidability theory.
//!
//! This module provides:
//! - [`TuringMachine`]: a deterministic single-tape Turing machine simulator
//! - [`RegisterMachine`]: a Minsky/counter machine simulator
//! - [`DecidabilityResult`]: classification of computational problems
//! - [`ComplexityClass`]: standard complexity classes (P, NP, PSPACE, ...)
//! - Functions for simulating TMs, register machines, and looking up known decidability results

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
