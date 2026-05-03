//! LSP `$/progress` notification support.
//!
//! Provides [`ProgressReporter`] for sending begin/report/end progress
//! notifications to an LSP client during long-running server operations.

pub mod types;

pub use types::*;
