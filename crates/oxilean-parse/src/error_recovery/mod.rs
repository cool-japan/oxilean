//! Parser error recovery strategies.
//!
//! This module provides types and functions for recovering from parse errors
//! in OxiLean source code.  Recovery is based on a small set of strategies
//! (skip-to-sync, insert, delete, replace, panic) and can produce a
//! corrected source string together with a list of the errors that were
//! encountered.

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
