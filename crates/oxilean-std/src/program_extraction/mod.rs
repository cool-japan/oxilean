//! Program extraction from constructive proofs via the Curry-Howard correspondence.
//!
//! This module implements the computational content extraction from intuitionistic
//! proofs. Under the Curry-Howard isomorphism, propositions correspond to types
//! and proofs correspond to programs; this module makes that extraction explicit.

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
