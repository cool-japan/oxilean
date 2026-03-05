//! # OxiLean Runtime -- Memory Management, Closures, I/O, and Scheduling
//!
//! This crate implements the runtime system for OxiLean.
//! It provides:
//!
//! - Tagged pointer object system (object) - Core runtime value representation
//! - Reference counting (rc) - Non-atomic and atomic RC with elision hints
//! - Arena allocators (arena) - Bump allocation and region-based memory
//! - Closure representation (closure) - Flat closures and partial application
//! - I/O operations (io_runtime) - File, console, and string I/O
//! - Task scheduler (scheduler) - Work-stealing parallel evaluation
//! - Lazy evaluation with memoization (lazy_eval) - Call-by-need thunks and memo caches
//! - Tail call optimization (tco) - Trampoline loop and TCO analysis
//! - Rich evaluation errors (eval_error) - Structured errors with source spans and hints

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::single_match)]
#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::approx_constant)]
#![allow(clippy::useless_format)]
#![allow(clippy::type_complexity)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::module_inception)]
#![allow(clippy::unnecessary_map_on_constructor)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::result_large_err)]
#![allow(clippy::write_with_newline)]
#![allow(clippy::unnecessary_map_or)]

pub mod arena;
pub mod bytecode_interp;
pub mod closure;
pub mod eval_error;
pub mod gc_strategies;
pub mod io_runtime;
pub mod lazy_eval;
pub mod memory_pool;
pub mod object;
pub mod profiler;
pub mod rc;
pub mod scheduler;
pub mod string_pool;
pub mod tco;
pub mod wasm_runtime;

// Re-exports for convenience
pub use arena::{
    ArenaIdx, ArenaOffset, ArenaPool, BumpArena, GenIdx, GenerationalArena, Region, RegionManager,
    TypedArena,
};
pub use closure::{
    CallConvention, CallStack, Closure, ClosureBuilder, FnPtr, FunctionEntry, FunctionTable,
    MutualRecGroup, Pap, PapResult, StackFrame,
};
pub use eval_error::{
    EvalError, EvalErrorBuilder, EvalErrorKind, EvalFrame, RuntimeError, SourceSpan,
};
pub use io_runtime::{
    ConsoleOps, FileOps, IoError, IoErrorKind, IoExecutor, IoResult, IoRuntime, IoValue,
    StringFormatter,
};
pub use lazy_eval::{LazyList, MemoFn, SharedThunk, Thunk, ThunkCache};
pub use object::{
    ArrayOps, BoxInto, FieldAccess, HeapObject, ObjectHeader, ObjectStore, ObjectTable, RtArith,
    RtObject, StringOps, ThunkOps, TypeInfo, TypeRegistry, TypeTag, UnboxFrom,
};
pub use rc::{
    ArcWeak, BorrowFlag, BorrowState, CowBox, Rc, RcElisionAnalysis, RcElisionHint, RcManager,
    RcPolicy, RcStats, RtArc, Weak,
};
pub use scheduler::{
    LoadBalanceStrategy, LoadBalancer, ParallelEval, Scheduler, SchedulerConfig, SharedState, Task,
    TaskId, TaskPriority, TaskState, WorkStealingDeque, Worker,
};
pub use tco::{
    run_tco_interpreter, trampoline, trampoline_instrumented, RecursiveStep, StepResult, TailCall,
    TailCallCounter, TailCallDetector, TailPosition,
};
