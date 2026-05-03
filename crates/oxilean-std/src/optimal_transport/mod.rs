//! Optimal transport module.
//!
//! Provides Wasserstein distances, Earth Mover's Distance, Sinkhorn algorithm,
//! sliced Wasserstein, Fréchet mean of measures, and related tools.

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
