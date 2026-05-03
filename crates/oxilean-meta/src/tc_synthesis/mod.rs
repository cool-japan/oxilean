//! Type class instance synthesis module.
//!
//! Provides recursive instance resolution, coherence checking, superclass
//! traversal, and a standard database of Lean4-like type classes.
//!
//! # Quick Start
//!
//! ```rust
//! use oxilean_meta::tc_synthesis::{standard_tc_db, synthesize_instance, SynthGoal, SynthConfig};
//!
//! let db = standard_tc_db();
//! let goal = SynthGoal::new("Add", vec!["Nat".to_string()]);
//! let (result, _trace) = synthesize_instance(&db, &goal, &SynthConfig::default());
//! ```

pub mod functions;
pub mod types;

pub use functions::{
    check_coherence, instance_to_string, standard_tc_db, superclass_chain, synthesize_instance,
};
pub use types::{SynthConfig, SynthGoal, SynthResult, SynthTrace, TcClass, TcDB, TcInstance};
