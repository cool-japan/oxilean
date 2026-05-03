//! Code Actions module for OxiLean LSP.
//!
//! Provides `textDocument/codeAction` support — quick-fixes and refactoring options
//! for Lean4-like source files.

pub mod functions;
pub mod types;

// Re-export all types and functions
pub use functions::*;
pub use types::*;
