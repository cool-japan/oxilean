//! Auto-generated module structure

pub mod api;
pub mod ast_diff;
pub mod atomicversion_traits;
pub mod changedetector_traits;
pub mod decldependencytracker_traits;
pub mod dependencygraph_traits;
pub mod fiberpool_traits;
pub mod functions;
pub mod incrementalerrormap_traits;
pub mod incrementallexer_traits;
pub mod incrscopestack_traits;
pub mod linediff_traits;
pub mod noderangecache_traits;
pub mod offsettotokenmap_traits;
pub mod parseversion_traits;
pub mod persistentvec_traits;
pub mod reparsequeue_traits;
pub mod simplerope_traits;
pub mod tokenreachability_traits;
pub mod tokenvalidity_traits;
pub mod transaction_traits;
pub mod types;

// Re-export all types
pub use api::{parse_incremental_change, IncrementalChangeResult};
pub use ast_diff::{diff_modules, DeclEdit, DeclFingerprint, DeclKind, EditKind};
pub use functions::*;
pub use types::*;
