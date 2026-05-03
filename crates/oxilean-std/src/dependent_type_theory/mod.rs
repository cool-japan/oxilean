//! Dependent type theory formalizations.
//!
//! This module provides formalizations of dependent type theory concepts,
//! including Pure Type Systems (PTS), the Calculus of Constructions (CoC),
//! System F, System Fω, Church encodings, and the Barendregt λ-cube.

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
