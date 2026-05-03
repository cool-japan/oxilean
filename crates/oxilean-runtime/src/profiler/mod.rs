//! Profiling and performance analysis for the OxiLean runtime.
//!
//! This module provides two profiling APIs:
//!
//! 1. **Legacy event-log API** (`types` / `functions`): lightweight ring-buffer
//!    profiler suited for embedded use with minimal overhead.
//!
//! 2. **Comprehensive config-driven API** (`perf_types` / `perf_functions`):
//!    full-featured profiler with call-record aggregation, flame-graph
//!    generation, folded-stacks output, and report merging.

pub mod allocationtracker_traits;
pub mod annotatedtimeline_traits;
pub mod countingstep_traits;
pub mod eventfilter_traits;
pub mod flamegraph_traits;
pub mod functions;
pub mod gcprofiler_traits;
pub mod perf_functions;
pub mod perf_types;
pub mod profiler_traits;
pub mod profilerconfig_traits;
pub mod profilingmiddleware_traits;
pub mod tacticprofilelog_traits;
pub mod timelineview_traits;
pub mod types;

// Re-export legacy types
pub use allocationtracker_traits::*;
pub use annotatedtimeline_traits::*;
pub use countingstep_traits::*;
pub use eventfilter_traits::*;
pub use flamegraph_traits::*;
pub use functions::*;
pub use gcprofiler_traits::*;
pub use profiler_traits::*;
pub use profilerconfig_traits::*;
pub use profilingmiddleware_traits::*;
pub use tacticprofilelog_traits::*;
pub use timelineview_traits::*;
pub use types::*;

// Re-export comprehensive profiling API
pub use perf_functions::{format_flame_graph_dot, format_report2, merge_reports2};
pub use perf_types::{
    AllocationRecord, CallRecord, EventKind, FlameGraphNode, ProfileReport2, Profiler2,
    ProfilerConfig2, ProfilerEvent,
};
