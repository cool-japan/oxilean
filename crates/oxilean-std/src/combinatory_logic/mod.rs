//! Combinatory logic and SKI calculus.
//!
//! This module provides a complete implementation of the SKI combinator calculus,
//! including reduction strategies, bracket abstraction algorithms for converting
//! lambda terms to combinators, Church numeral encoding, and structural utilities.

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
