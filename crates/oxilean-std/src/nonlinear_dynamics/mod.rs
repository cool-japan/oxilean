//! Nonlinear dynamics module.
//!
//! Provides types and algorithms for nonlinear dynamical systems, including
//! discrete maps (logistic, Hénon, tent), continuous systems (Lorenz, Rössler),
//! Lyapunov exponents, fractal dimensions, Poincaré sections, and bifurcation
//! analysis.

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
