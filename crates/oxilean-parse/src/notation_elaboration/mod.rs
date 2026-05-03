//! User-defined notation elaboration for OxiLean.
//!
//! This module manages a database of notation definitions (prefix, infix,
//! postfix, and mixfix) and provides a lightweight elaboration pass that
//! rewrites source text according to those definitions.
//!
//! # Quick example
//!
//! ```ignore
//! use oxilean_parse::notation_elaboration::{standard_notations, elaborate_notation};
//!
//! let db = standard_notations();
//! let result = elaborate_notation("P ∧ Q", &db);
//! assert!(result.elaborated.contains("And"));
//! ```

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
