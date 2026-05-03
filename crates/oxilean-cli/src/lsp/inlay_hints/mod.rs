//! Dedicated LSP inlay hints module.
//!
//! Provides:
//! - [`InlayHint`] — a single inlay hint with position, label, and kind
//! - [`InlayHintKind`] — `Type` (= 1) and `Parameter` (= 2) variants
//! - [`InlayHintHandler`] — handles `textDocument/inlayHint` requests

pub mod types;

pub use types::*;
