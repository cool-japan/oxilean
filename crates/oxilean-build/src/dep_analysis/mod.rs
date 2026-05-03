//! Dependency graph analysis for the OxiLean build system.
//!
//! Provides Tarjan SCC cycle detection, Kahn's topological ordering,
//! transitive closure, reachability BFS, impact analysis, and DOT rendering.

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
