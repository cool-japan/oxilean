//! Serialization and deserialization of the LCNF intermediate representation.
//!
//! Supports three wire formats: [`IrFormat::Text`] (human-readable), [`IrFormat::Json`]
//! (machine-readable UTF-8 JSON), and [`IrFormat::Binary`] (compact binary; write-only
//! in this version).

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
