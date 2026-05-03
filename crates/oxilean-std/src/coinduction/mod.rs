//! Coinductive proofs and greatest fixed points.
//!
//! Provides types and algorithms for coinductive reasoning:
//! - `LazyStream<T>` — eventually-periodic infinite stream
//! - `BisimulationRelation<S>` — candidate bisimulation as a set of state pairs
//! - `CoinductiveProof<S>` — proof certificate for bisimilarity
//! - `CoalgebraMap<S, O>` — explicit transition table for LTS
//! - `GreibachNormalForm` — GNF grammar for coinductive context-free languages
//! - `greatest_fixed_point_approx` — iterative GFP computation
//! - `prove_bisimilar` — search for a bisimulation containing a given pair
//! - `fibonacci_stream` — canonical coinductive example

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
