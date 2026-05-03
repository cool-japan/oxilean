//! The simp tactic: simplification by rewriting.
//!
//! Implements the main simplification algorithm used by simp,
//! simp only, simp_all, and related tactics.
//!
//! Module Layout:
//!
//! - `types`: Core types: SimpLemma, SimpConfig, SimpTheorems, SimpResult
//! - `main`: The main simplification loop: bottom-up rewriting with lemma matching
//! - `discharge`: Side-goal discharge for conditional rewrites
//! - `context`: Per-invocation state: SimpStats, SimpContext, SimpReport, SimpLemmaDatabase
//! - `extensions`: Extension types: SimpLemmaFilter, SimpTrace, SimpNormalForm, SimpConfigExt,
//!   SimpLemmaCache, SimpScheduler, SimpBudget
//! - `utilities`: Utility and analysis infrastructure types

#![allow(dead_code)]
#![allow(missing_docs)]

pub mod context;
pub mod discharge;
pub mod extensions;
pub mod main;
pub mod simp_rw;
pub mod types;
pub mod utilities;

// Re-export existing public API from submodules
pub use discharge::discharge_side_goal;
pub use main::simp;
pub use simp_rw::{apply_rw_rules, tac_simp_rw, tac_simp_rw_with_iters, RwRule};
pub use types::{default_simp_lemmas, SimpConfig, SimpLemma, SimpResult, SimpTheorems};

// Re-export from context
pub use context::{SimpContext, SimpLemmaDatabase, SimpReport, SimpStats};

// Re-export from extensions
pub use extensions::{
    SimpBudget, SimpConfigExt, SimpLemmaCache, SimpLemmaFilter, SimpNormalForm, SimpScheduler,
    SimpTrace,
};

// Re-export from utilities
pub use utilities::{
    ModExtConfig800, ModExtConfigVal800, ModExtDiag800, ModExtDiff800, ModExtPass800,
    ModExtPipeline800, ModExtResult800, SimpModBuilder, SimpModCounterMap, SimpModExt,
    SimpModExtMap, SimpModExtUtil, SimpModState, SimpModStateMachine, SimpModWindow,
    SimpModWorkQueue, TacticSimpModAnalysisPass, TacticSimpModConfig, TacticSimpModConfigValue,
    TacticSimpModDiagnostics, TacticSimpModDiff, TacticSimpModPipeline, TacticSimpModResult,
};
