//! Hybrid dynamical systems theory.
//!
//! Combines discrete transition systems (automata) with continuous-time
//! ODE dynamics in each discrete mode.  Provides simulation, safety
//! checking, Zeno detection, reachability analysis, and composition.

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
