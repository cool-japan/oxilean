//! Abstract Rewriting Systems (ARS) and term rewriting.
//!
//! This module provides:
//! - [`RewriteRule`]: conditional and unconditional rewrite rules over generic terms
//! - [`RewriteSystem`]: a collection of rules with a reduction strategy
//! - [`TermTree`]: a concrete tree-structured term representation
//! - Functions for normalization, confluence checking, and termination analysis

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
