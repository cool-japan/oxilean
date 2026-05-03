//! Types for hole inference (automatically filling `_` in terms).

/// A single hole (`_`) found in source code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hole {
    /// Unique identifier for this hole.
    pub id: usize,
    /// The expected type at this hole position.
    pub expected_type: String,
    /// Local context at the hole: list of (name, type) pairs.
    pub context: Vec<(String, String)>,
    /// Source span: (start_byte, end_byte), if known.
    pub span: Option<(usize, usize)>,
}

/// A proposed filling for a specific hole.
#[derive(Debug, Clone, PartialEq)]
pub struct HoleFilling {
    /// Which hole this filling is for.
    pub hole_id: usize,
    /// The term to fill the hole with.
    pub term: String,
    /// Confidence in this filling (0.0 to 1.0).
    pub confidence: f64,
}

/// Result of attempting to fill all holes in a source string.
#[derive(Debug, Clone, PartialEq)]
pub struct HoleInferenceResult {
    /// Successful fillings.
    pub fillings: Vec<HoleFilling>,
    /// Holes that could not be filled.
    pub remaining: Vec<Hole>,
    /// Statistics about the inference run.
    pub stats: InferenceStats,
}

/// Statistics about a hole inference run.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct InferenceStats {
    /// Total holes found in the source.
    pub holes_found: usize,
    /// Number of holes successfully filled.
    pub holes_filled: usize,
    /// Time taken in milliseconds.
    pub time_ms: u64,
}

/// What kind of `_` (hole) this is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HoleKind {
    /// A type-level hole (`_ : Type`).
    Type,
    /// A term-level hole.
    Term,
    /// A universe-level hole (`Sort _`).
    Universe,
    /// An instance (typeclass) hole.
    Instance,
}

/// Context surrounding a hole, with its expected type and kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HoleContext {
    /// Local declarations at the hole site.
    pub local_decls: Vec<(String, String)>,
    /// The expected type of this hole.
    pub expected: String,
    /// What kind of `_` this is.
    pub kind: HoleKind,
}
