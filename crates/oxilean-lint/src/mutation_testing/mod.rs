//! Mutation testing framework for oxilean-lint.
//!
//! This module implements a text-based mutation testing engine that can
//! introduce small syntactic changes to source files and report on whether
//! the existing test suite detects each change.
//!
//! # Overview
//!
//! 1. **Discover mutations** — `find_mutations` scans a source string and
//!    produces a list of [`Mutation`] values, one per location where a
//!    mutation operator can be applied.
//!
//! 2. **Apply a mutation** — `apply_mutation` substitutes one mutation into
//!    the source text, producing a new string ready for compilation.
//!
//! 3. **Report** — `format_mutation_report`, `score_mutation_report`, and
//!    related helpers turn a completed [`MutationReport`] into human-readable
//!    output or numeric summaries.

pub mod functions;
pub mod types;

pub use functions::{
    apply_mutation, filter_mutations, find_mutations, format_mutation_report,
    generate_mutations_for_file, killed_mutations, mutations_by_operator, score_mutation_report,
    summarize_mutation_report, survived_mutations,
};
pub use types::{
    Mutation, MutationConfig, MutationConfigBuilder, MutationFilter, MutationOperator,
    MutationReport, MutationResult, MutationScanContext, MutationStats,
};
