//! Convex analysis module.
//!
//! Provides convex sets, convex functions, subgradients, projections,
//! Fenchel conjugates, proximal operators, and related optimisation tools.

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
