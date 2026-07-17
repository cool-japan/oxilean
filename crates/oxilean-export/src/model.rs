//! The record model: the typed, kernel-facing declarations surfaced by the
//! reader, plus the metadata record.
//!
//! Primitive records (names, levels, expressions) are *not* surfaced as public
//! values — they are interned directly into the reader's index tables and
//! converted to `oxilean_kernel` types. What a consumer sees is the sequence of
//! [`ExportDecl`] declarations (each carrying kernel-typed payloads) plus the
//! [`Meta`] header.

use oxilean_kernel::{
    AxiomVal, ConstructorVal, DefinitionVal, InductiveVal, OpaqueVal, QuotVal, RecursorVal,
    TheoremVal,
};

/// The metadata header (first line of every export file).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Meta {
    /// Exporter tool name (e.g. `"lean4export"`).
    pub exporter_name: String,
    /// Exporter tool version (e.g. `"3.1.0"`).
    pub exporter_version: String,
    /// Lean toolchain git hash the export was produced with.
    pub lean_githash: String,
    /// Lean toolchain version (e.g. `"4.32.0-rc1"`).
    pub lean_version: String,
    /// The NDJSON format version string (e.g. `"3.1.0"`).
    pub format_version: String,
}

impl Meta {
    /// The declared major version, parsed from the leading component of
    /// [`Meta::format_version`]. Returns `None` if it is not a leading integer.
    #[must_use]
    pub fn format_major(&self) -> Option<u32> {
        let head = self.format_version.split('.').next()?;
        head.parse::<u32>().ok()
    }
}

/// A fully-parsed, kernel-typed declaration surfaced by the reader.
///
/// Each variant wraps the corresponding `oxilean_kernel` value directly, so a
/// consumer can hand it straight to the kernel checking API. The `Inductive`
/// variant bundles a whole mutual group (types + constructors + recursors), as
/// the export format emits it in a single record.
#[derive(Debug, Clone)]
pub enum ExportDecl {
    /// An `axiom` declaration.
    Axiom(AxiomVal),
    /// A `def` declaration.
    Definition(DefinitionVal),
    /// A `thm` (theorem) declaration.
    Theorem(TheoremVal),
    /// An `opaque` declaration.
    Opaque(OpaqueVal),
    /// A `quot` primitive (one of `Quot`, `Quot.mk`, `Quot.lift`, `Quot.ind`).
    Quot(QuotVal),
    /// An `inductive` bundle: a mutual group of types with their constructors
    /// and (exporter-derived) recursors.
    Inductive(InductiveBundle),
    /// A declaration the reader refused to materialize because its kernel
    /// trees would exceed the PER-DECLARATION materialization budget
    /// (`Limits::decl_materialize_budget` — C22: a single Lean-core theorem
    /// can legally DAG-share into a tree of hundreds of millions of nodes,
    /// which would OOM the process before the kernel ever sees it).
    ///
    /// Carries every name the declaration would have introduced (read from
    /// the interned name table WITHOUT materializing any expression), so the
    /// replayer can defer them and cascade their dependents exactly like any
    /// other named-unsupported declaration. No term is ever fabricated and
    /// nothing downstream can turn this into a wrong verdict: dependents see
    /// an unknown-but-deferred constant, never a bogus type.
    Oversized(OversizedVal),
}

/// Payload of [`ExportDecl::Oversized`]: the names a skipped declaration
/// would have introduced, plus the named unsupported feature.
#[derive(Debug, Clone)]
pub struct OversizedVal {
    /// Every name the declaration record would have introduced (for a
    /// def/thm/opaque group: each member; for an inductive bundle: all
    /// types, constructors and recursors).
    pub names: Vec<Name>,
    /// The stable named feature (see `reader::DECL_BUDGET_FEATURE`).
    pub feature: &'static str,
}

/// A mutual inductive group as emitted in a single `inductive` record.
#[derive(Debug, Clone)]
pub struct InductiveBundle {
    /// The inductive types in the mutual group.
    pub types: Vec<InductiveVal>,
    /// The constructors of all types in the group.
    pub ctors: Vec<ConstructorVal>,
    /// The exporter-derived recursors. Per the spec these are redundant and a
    /// full checker re-derives them; the reader surfaces them for cross-checks.
    pub recs: Vec<RecursorVal>,
}

impl ExportDecl {
    /// The primary name of this declaration, for reporting.
    ///
    /// For an inductive bundle this is the first type's name; a (structurally
    /// impossible) empty bundle yields the anonymous name.
    #[must_use]
    pub fn primary_name(&self) -> oxilean_kernel::Name {
        match self {
            ExportDecl::Axiom(v) => v.common.name.clone(),
            ExportDecl::Definition(v) => v.common.name.clone(),
            ExportDecl::Theorem(v) => v.common.name.clone(),
            ExportDecl::Opaque(v) => v.common.name.clone(),
            ExportDecl::Quot(v) => v.common.name.clone(),
            ExportDecl::Inductive(b) => b
                .types
                .first()
                .map_or_else(Name::anonymous, |t| t.common.name.clone()),
            ExportDecl::Oversized(v) => v.names.first().map_or_else(Name::anonymous, Clone::clone),
        }
    }

    /// A short, stable label for the declaration kind (for reports).
    #[must_use]
    pub fn kind_label(&self) -> &'static str {
        match self {
            ExportDecl::Axiom(_) => "axiom",
            ExportDecl::Definition(_) => "def",
            ExportDecl::Theorem(_) => "thm",
            ExportDecl::Opaque(_) => "opaque",
            ExportDecl::Quot(_) => "quot",
            ExportDecl::Inductive(_) => "inductive",
            ExportDecl::Oversized(_) => "oversized",
        }
    }
}

use oxilean_kernel::Name;
