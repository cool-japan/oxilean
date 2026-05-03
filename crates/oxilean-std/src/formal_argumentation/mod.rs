//! Formal Argumentation — Dung's abstract argumentation frameworks.
//!
//! Implements:
//! - `ArgumentationFramework` (Args, Att) — the core Dung structure.
//! - Admissibility, conflict-freeness, and the characteristic function F_AF.
//! - Grounded extension (least fixed point), preferred extensions (maximal admissible),
//!   stable extensions, complete extensions, CF2 semantics.
//! - Credulous and skeptical acceptance queries.
//! - Classic examples: Nixon diamond, reinstatement, floating conclusions.

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
