//! Types for the LCNF IR serialization module.

use std::collections::HashMap;

/// A fully serialized Oxilean LCNF module ready for storage or transmission.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SerializedIr {
    /// Format version used when this snapshot was written.
    pub version: u32,
    /// Fully-qualified name of the module.
    pub module_name: String,
    /// Every declaration exported by the module.
    pub declarations: Vec<SerialDecl>,
    /// Arbitrary key-value metadata (e.g. compiler flags, timestamps).
    pub metadata: HashMap<String, String>,
}

/// A single serialized declaration inside a [`SerializedIr`] module.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SerialDecl {
    /// The fully-qualified name of the declaration.
    pub name: String,
    /// The flavour of this declaration.
    pub kind: DeclKind,
    /// The type signature as a human-readable string.
    pub type_: String,
    /// The body (term) as a human-readable string, if present.
    pub body: Option<String>,
    /// Formal parameter names in order.
    pub params: Vec<String>,
}

/// Possible kinds of LCNF declarations.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclKind {
    /// An ordinary definition (`def`).
    Def,
    /// A proved theorem (`theorem`).
    Theorem,
    /// An axiom (unproved assumption).
    Axiom,
    /// An inductive type declaration.
    Inductive,
    /// A constructor of an inductive type.
    Constructor,
    /// A recursor / eliminator for an inductive type.
    Recursor,
}

impl std::fmt::Display for DeclKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DeclKind::Def => "def",
            DeclKind::Theorem => "theorem",
            DeclKind::Axiom => "axiom",
            DeclKind::Inductive => "inductive",
            DeclKind::Constructor => "constructor",
            DeclKind::Recursor => "recursor",
        };
        f.write_str(s)
    }
}

/// The serialization format to use when writing an IR snapshot.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrFormat {
    /// Human-readable plain-text format.
    Text,
    /// A compact binary encoding (length-prefixed fields).
    Binary,
    /// JSON encoding (UTF-8).
    Json,
}

/// Configuration for the IR serializer.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IrSerializeConfig {
    /// Which wire format to emit.
    pub format: IrFormat,
    /// Whether to pretty-print (indent) the output when supported.
    pub pretty: bool,
    /// Include type signatures in the output.
    pub include_types: bool,
    /// Include proof bodies in the output.
    pub include_proofs: bool,
}

impl Default for IrSerializeConfig {
    fn default() -> Self {
        IrSerializeConfig {
            format: IrFormat::Text,
            pretty: true,
            include_types: true,
            include_proofs: true,
        }
    }
}

/// The result of attempting to deserialize an IR snapshot.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrDeserializeResult {
    /// Deserialization succeeded.
    Ok(SerializedIr),
    /// The snapshot was written with a different format version.
    VersionMismatch {
        /// The version the deserializer expects.
        expected: u32,
        /// The version found in the data.
        found: u32,
    },
    /// The input could not be parsed.
    ParseError(String),
    /// The requested format is not supported by this deserializer.
    Unsupported(String),
}
