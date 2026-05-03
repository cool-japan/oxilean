//! Proof search and automation for Oxilean.
//!
//! Provides BFS, A\*, DFS, and IDDFS proof search over a configurable set of
//! `AutoTactic` variants.  The engines are self-contained: they operate on a
//! *goal-manipulation model* (a `Vec<MVarId>` representing open goals) and
//! produce a [`ProofSearchResult`] that includes a placeholder proof term and
//! a human-readable tactic trace.  The caller is responsible for replaying the
//! trace against a live `MetaContext` to obtain the real kernel-checked proof.
//!
//! # Quick start
//!
//! ```
//! use oxilean_meta::tactic::proof_search::{
//!     BfsProofSearch, ProofSearchEngine, AutoTactic, ProofSearchConfig,
//! };
//! use oxilean_meta::basic::MVarId;
//!
//! let engine = BfsProofSearch::new();
//! let goals = vec![MVarId(0)];
//! match engine.search(goals, 100) {
//!     Ok(result) => println!("Proof found in {} steps: {:?}", result.depth, result.tactics_used),
//!     Err(e)     => println!("Search failed: {e}"),
//! }
//! ```

pub mod functions;
pub mod types;

// Re-export everything so callers can do `use oxilean_meta::tactic::proof_search::*`.
pub use functions::search_with_config;
pub use types::{
    AstarHeap, AstarNode, AstarProofSearch, AutoTactic, BfsProofSearch, DfsProofSearch,
    IddfsProofSearch, ProofSearchConfig, ProofSearchEngine, ProofSearchError, ProofSearchResult,
    SearchNode, SearchStats, SearchStrategy, TacticApplicationResult,
};
