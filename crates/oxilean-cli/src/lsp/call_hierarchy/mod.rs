//! Call hierarchy module for OxiLean LSP.
//!
//! Implements `textDocument/prepareCallHierarchy`, `callHierarchy/incomingCalls`,
//! and `callHierarchy/outgoingCalls` for Lean4-like source files.
//!
//! The analysis is purely textual and does not require the type-checker to be
//! fully operational: declarations are identified by keyword patterns and
//! references are found by identifier-boundary scanning.

pub mod functions;
pub mod types;

// Re-export public surface
pub use functions::*;
pub use types::*;
