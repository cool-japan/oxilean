//! Document Symbols module for OxiLean LSP.
//!
//! Provides `textDocument/documentSymbol` support — an outline view of all
//! declarations in a Lean4-like source file.

pub mod functions;
pub mod types;

// Re-export all types and functions
pub use functions::*;
pub use types::*;
