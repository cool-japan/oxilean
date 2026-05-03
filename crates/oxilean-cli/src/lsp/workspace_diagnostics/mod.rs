//! Workspace-level diagnostics management.
//!
//! Provides [`WorkspaceDiagnosticsManager`] for tracking per-file diagnostics
//! and generating `textDocument/publishDiagnostics` LSP notifications.

pub mod types;

pub use types::*;
