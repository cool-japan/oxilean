//! Auto-generated module structure

pub mod bytecodecompiler_traits;
pub mod framedinterpreter_traits;
pub mod functions;
pub mod interpreter_traits;
pub mod peepholeoptimizer_traits;
pub mod profilinginterpreter_traits;
pub mod stackvalue_traits;
pub mod types;

// Re-export all types
pub use bytecodecompiler_traits::*;
pub use framedinterpreter_traits::*;
pub use functions::*;
pub use interpreter_traits::*;
pub use peepholeoptimizer_traits::*;
pub use profilinginterpreter_traits::*;
pub use stackvalue_traits::*;
pub use types::*;
