//! Geometric topology: 3-manifolds, knot surgery, JSJ decomposition, gluing matrices.
//!
//! This module provides:
//! - [`Manifold3`] — a 3-manifold with basic topological invariants
//! - [`HeegaardSplitting`] — Heegaard splitting data
//! - [`DehnSurgery`] — (p/q)-Dehn surgery specification
//! - [`JSJDecomposition`] — JSJ decomposition into geometric pieces
//! - [`Matrix2x2`] — 2×2 integer gluing matrices in SL(2,Z)
//! - Functions for computing Euler characteristics, Heegaard genus, Thurston geometry, etc.

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
