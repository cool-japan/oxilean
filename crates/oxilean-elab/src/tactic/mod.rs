//! Auto-generated module structure

pub mod functions;
pub mod functions_2;
pub mod functions_3;
pub mod tacticerror_traits;
pub mod tacticregistry_traits;
pub mod tacticstate_traits;
pub mod types;

/// Enhanced `decide` tactic — decides propositions via arithmetic, SAT, etc.
pub mod decide_enhanced;
/// `norm_cast` tactic — normalises coercions in goals and hypotheses.
pub mod norm_cast;

// Re-export all types
pub use functions::*;
pub use functions_2::*;
pub use functions_3::*;
pub use tacticerror_traits::*;
pub use tacticregistry_traits::*;
pub use tacticstate_traits::*;
pub use types::*;
