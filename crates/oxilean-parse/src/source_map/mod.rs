//! Source mapping for macro-expanded and generated code.
//!
//! This module provides a [`SourceMap`] that tracks the origin of every byte
//! of source text, including code produced by macro expansions or code
//! generators.  It can translate between byte offsets and line/column
//! positions and can walk back through the chain of macro expansions to find
//! the original user-written source location.

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
