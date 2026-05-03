//! Peephole optimisation pass for the Oxilean code generation pipeline.
//!
//! Peephole optimisation replaces small instruction sequences with cheaper
//! equivalents.  [`standard_rules`] provides the built-in rule set; callers
//! can compose their own rules by building [`PeepRule`] values and passing
//! them to [`run_peephole`].

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
