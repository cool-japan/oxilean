//! Folding range module for OxiLean LSP.
//!
//! Implements `textDocument/foldingRange` for Lean4-like source files.
//! Detects five categories of foldable regions: comment blocks, import blocks,
//! declaration bodies, do-blocks, and merged adjacent single-line regions.

pub mod functions;
pub mod types;

// Re-export public surface
pub use functions::*;
pub use types::*;
