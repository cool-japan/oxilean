//! Coercion system — automatic type coercion (casting) infrastructure.
//!
//! When Lean4 cannot directly unify two types `A` and `B`, it searches the
//! coercion database for a registered function `coe : A → B` (or a chain of
//! such functions) and inserts the call automatically.
//!
//! # Quick Start
//!
//! ```rust
//! use oxilean_meta::coercion_system::{standard_coercions, apply_coercion};
//!
//! let db = standard_coercions();
//! let path_result = db.find_path("Nat", "Real", 10);
//! ```

pub mod functions;
pub mod types;

pub use functions::{apply_coercion, coercion_to_string, detect_cycles, standard_coercions};
pub use types::{
    CoercedExpr, CoercionDB, CoercionDef, CoercionError, CoercionPath, CoercionResult,
};
