//! Zero-knowledge proofs module.
//!
//! Provides ZK proof systems: Pedersen commitments, Schnorr-like discrete-log
//! proofs (Sigma protocols), and bit-decomposition range proofs over small toy
//! prime fields.

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
