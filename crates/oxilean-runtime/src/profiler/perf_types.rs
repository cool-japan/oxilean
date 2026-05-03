//! Performance profiler types: `ProfilerEvent`, `EventKind`, `CallRecord`,
//! `AllocationRecord`, `ProfileReport2`, `Profiler2`, `ProfilerConfig2`,
//! and `FlameGraphNode`.
//!
//! These types provide a comprehensive, config-driven profiling API distinct
//! from the lightweight event-log API in `types.rs`.

use std::collections::HashMap;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

// ---------------------------------------------------------------------------
// EventKind
// ---------------------------------------------------------------------------

/// The kind of runtime profiling event.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventKind {
    /// A function was entered.
    FunctionEnter,
    /// A function was exited.
    FunctionExit,
    /// A heap allocation occurred.
    Allocation {
        /// Number of bytes allocated.
        size_bytes: usize,
    },
    /// A GC pause occurred.
    GcPause {
        /// Duration of the GC pause in nanoseconds.
        duration_ns: u64,
    },
    /// A tail call was optimised away.
    TailCall,
    /// A lazy thunk was forced.
    ThunkForced,
    /// A closure was created.
    ClosureCreated,
    /// A closure was applied to an argument.
    ClosureApplied,
    /// A runtime error occurred.
    RuntimeError,
}

impl EventKind {
    /// Return the static variant name used for serialisation / display.
    pub fn variant_name(&self) -> &'static str {
        match self {
            EventKind::FunctionEnter => "FunctionEnter",
            EventKind::FunctionExit => "FunctionExit",
            EventKind::Allocation { .. } => "Allocation",
            EventKind::GcPause { .. } => "GcPause",
            EventKind::TailCall => "TailCall",
            EventKind::ThunkForced => "ThunkForced",
            EventKind::ClosureCreated => "ClosureCreated",
            EventKind::ClosureApplied => "ClosureApplied",
            EventKind::RuntimeError => "RuntimeError",
        }
    }
}

impl std::fmt::Display for EventKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.variant_name())
    }
}

// ---------------------------------------------------------------------------
// ProfilerEvent
// ---------------------------------------------------------------------------

/// A single timestamped profiling event.
#[derive(Clone, Debug)]
pub struct ProfilerEvent {
    /// Absolute timestamp from the epoch, in nanoseconds.
    pub timestamp_ns: u64,
    /// The kind of event.
    pub kind: EventKind,
    /// Name associated with the event (function name, type name, etc.).
    pub name: String,
    /// Optional auxiliary data (e.g. allocation size, duration).
    pub data: Option<u64>,
}

impl ProfilerEvent {
    /// Construct a new profiler event stamped with the current wall-clock time.
    pub fn new(kind: EventKind, name: impl Into<String>, data: Option<u64>) -> Self {
        Self {
            timestamp_ns: wall_clock_ns(),
            kind,
            name: name.into(),
            data,
        }
    }

    /// Construct a profiler event with an explicit timestamp.
    pub fn with_timestamp(
        timestamp_ns: u64,
        kind: EventKind,
        name: impl Into<String>,
        data: Option<u64>,
    ) -> Self {
        Self {
            timestamp_ns,
            kind,
            name: name.into(),
            data,
        }
    }
}

// ---------------------------------------------------------------------------
// CallRecord
// ---------------------------------------------------------------------------

/// Aggregated timing information for a single named call site.
#[derive(Clone, Debug)]
pub struct CallRecord {
    /// Function (or call site) name.
    pub name: String,
    /// Total number of times this function was called.
    pub calls: u64,
    /// Cumulative inclusive wall time in nanoseconds (enter → exit).
    pub total_ns: u64,
    /// Exclusive (self) time — `total_ns` minus time in callees.
    pub self_ns: u64,
    /// Maximum single-call duration in nanoseconds.
    pub max_ns: u64,
    /// Minimum single-call duration in nanoseconds.
    pub min_ns: u64,
}

impl CallRecord {
    /// Create an empty record for `name`.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            calls: 0,
            total_ns: 0,
            self_ns: 0,
            max_ns: 0,
            min_ns: u64::MAX,
        }
    }

    /// Mean inclusive time per call, in nanoseconds.
    pub fn mean_ns(&self) -> f64 {
        if self.calls == 0 {
            0.0
        } else {
            self.total_ns as f64 / self.calls as f64
        }
    }
}

// ---------------------------------------------------------------------------
// AllocationRecord
// ---------------------------------------------------------------------------

/// Aggregated allocation statistics for a type or allocation site.
#[derive(Clone, Debug)]
pub struct AllocationRecord {
    /// Type name or allocation tag.
    pub type_name: String,
    /// Number of allocations recorded.
    pub count: u64,
    /// Total bytes allocated.
    pub total_bytes: u64,
}

impl AllocationRecord {
    /// Create an empty record for `type_name`.
    pub fn new(type_name: impl Into<String>) -> Self {
        Self {
            type_name: type_name.into(),
            count: 0,
            total_bytes: 0,
        }
    }

    /// Average allocation size in bytes.
    pub fn avg_bytes(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            self.total_bytes as f64 / self.count as f64
        }
    }
}

// ---------------------------------------------------------------------------
// ProfileReport2
// ---------------------------------------------------------------------------

/// A comprehensive report produced by [`Profiler2`].
#[derive(Clone, Debug)]
pub struct ProfileReport2 {
    /// Per-function aggregated call records.
    pub call_records: Vec<CallRecord>,
    /// Per-type aggregated allocation records.
    pub alloc_records: Vec<AllocationRecord>,
    /// Total number of events stored in the profiler buffer.
    pub total_events: u64,
    /// Total wall time covered by this profiling session, in nanoseconds.
    pub wall_time_ns: u64,
    /// Duration of each GC pause, in nanoseconds.
    pub gc_pauses: Vec<u64>,
}

impl ProfileReport2 {
    /// Create an empty report.
    pub fn empty() -> Self {
        Self {
            call_records: Vec::new(),
            alloc_records: Vec::new(),
            total_events: 0,
            wall_time_ns: 0,
            gc_pauses: Vec::new(),
        }
    }

    /// Total GC pause time in nanoseconds.
    pub fn total_gc_pause_ns(&self) -> u64 {
        self.gc_pauses.iter().sum()
    }

    /// Number of GC pauses recorded.
    pub fn gc_pause_count(&self) -> usize {
        self.gc_pauses.len()
    }

    /// Mean GC pause duration in nanoseconds (0.0 if no pauses).
    pub fn mean_gc_pause_ns(&self) -> f64 {
        if self.gc_pauses.is_empty() {
            0.0
        } else {
            self.total_gc_pause_ns() as f64 / self.gc_pauses.len() as f64
        }
    }
}

// ---------------------------------------------------------------------------
// ProfilerConfig2
// ---------------------------------------------------------------------------

/// Configuration for [`Profiler2`].
#[derive(Clone, Debug)]
pub struct ProfilerConfig2 {
    /// Maximum number of raw events to keep in the ring buffer.
    pub max_events: usize,
    /// Sampling rate — 1 means record every event, N means record 1 in N.
    pub sample_rate: u32,
    /// Whether to track allocation events.
    pub track_allocations: bool,
}

impl ProfilerConfig2 {
    /// Construct a default configuration.
    pub fn new() -> Self {
        Self {
            max_events: 1_000_000,
            sample_rate: 1,
            track_allocations: true,
        }
    }

    /// Builder: set the maximum number of events.
    pub fn with_max_events(mut self, max_events: usize) -> Self {
        self.max_events = max_events;
        self
    }

    /// Builder: set the sampling rate (1 = every event).
    pub fn with_sample_rate(mut self, sample_rate: u32) -> Self {
        self.sample_rate = sample_rate.max(1);
        self
    }

    /// Builder: enable or disable allocation tracking.
    pub fn with_track_allocations(mut self, track_allocations: bool) -> Self {
        self.track_allocations = track_allocations;
        self
    }
}

impl Default for ProfilerConfig2 {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Internal call-stack frame
// ---------------------------------------------------------------------------

/// An in-flight call-stack entry used internally by [`Profiler2`].
#[derive(Clone, Debug)]
pub(super) struct InFlightFrame {
    /// Function name.
    pub(super) name: String,
    /// `Instant` captured at `enter()` time.
    pub(super) enter_instant: Instant,
    /// Cumulative time spent in direct callees (subtracted to get self-time).
    pub(super) callee_ns: u64,
}

impl InFlightFrame {
    pub(super) fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            enter_instant: Instant::now(),
            callee_ns: 0,
        }
    }
}

// ---------------------------------------------------------------------------
// Profiler2
// ---------------------------------------------------------------------------

/// A comprehensive, config-driven profiler.
///
/// Create one with [`Profiler2::new`], call [`Profiler2::enter`] /
/// [`Profiler2::exit`] around code regions, then call
/// [`Profiler2::generate_report`] to obtain a [`ProfileReport2`].
pub struct Profiler2 {
    /// Whether profiling is active.
    pub enabled: bool,
    /// Raw event buffer (ring buffer capped at `config.max_events`).
    pub(super) events: Vec<ProfilerEvent>,
    /// In-flight call stack.
    pub(super) call_stack: Vec<InFlightFrame>,
    /// Accumulated per-function call records.
    pub(super) call_records: HashMap<String, CallRecord>,
    /// Accumulated per-type allocation records.
    pub(super) alloc_records: HashMap<String, AllocationRecord>,
    /// GC pause durations (nanoseconds).
    pub(super) gc_pauses: Vec<u64>,
    /// Wall-clock instant when profiling was first enabled.
    pub(super) start_instant: Option<Instant>,
    /// Configuration.
    pub(super) config: ProfilerConfig2,
    /// Monotonically-incrementing event counter for sampling decisions.
    pub(super) event_counter: u64,
}

impl Profiler2 {
    /// Construct a new profiler with the given configuration.
    ///
    /// The profiler starts **disabled**; call [`Profiler2::enable`] or
    /// create via [`Profiler2::new_enabled`] to begin recording.
    pub fn new(config: ProfilerConfig2) -> Self {
        Self {
            enabled: false,
            events: Vec::new(),
            call_stack: Vec::new(),
            call_records: HashMap::new(),
            alloc_records: HashMap::new(),
            gc_pauses: Vec::new(),
            start_instant: None,
            config,
            event_counter: 0,
        }
    }

    /// Construct a new profiler that is immediately enabled.
    pub fn new_enabled(config: ProfilerConfig2) -> Self {
        let mut p = Self::new(config);
        p.enable();
        p
    }

    /// Enable profiling.
    pub fn enable(&mut self) {
        if !self.enabled {
            self.enabled = true;
            self.start_instant = Some(Instant::now());
        }
    }

    /// Disable profiling.
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Returns `true` when the event should be sampled (respects `sample_rate`).
    pub(super) fn should_sample(&mut self) -> bool {
        self.event_counter = self.event_counter.wrapping_add(1);
        let rate = self.config.sample_rate as u64;
        (self.event_counter % rate) == 0
    }

    /// Push a raw event onto the ring buffer, evicting the oldest entry when
    /// the buffer is full.
    pub(super) fn push_event(&mut self, event: ProfilerEvent) {
        if self.events.len() >= self.config.max_events {
            self.events.remove(0);
        }
        self.events.push(event);
    }
}

// ---------------------------------------------------------------------------
// FlameGraphNode
// ---------------------------------------------------------------------------

/// A node in the flame-graph call tree.
#[derive(Clone, Debug)]
pub struct FlameGraphNode {
    /// Function (call site) name.
    pub name: String,
    /// Exclusive (self) time in nanoseconds.
    pub self_time: u64,
    /// Inclusive (total) time in nanoseconds.
    pub total_time: u64,
    /// Children in the call tree (callees).
    pub children: Vec<FlameGraphNode>,
}

impl FlameGraphNode {
    /// Construct a leaf node with zero times.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            self_time: 0,
            total_time: 0,
            children: Vec::new(),
        }
    }

    /// Find or create a direct child with the given name and return a mutable
    /// reference to it.
    pub fn get_or_create_child(&mut self, name: &str) -> &mut FlameGraphNode {
        if let Some(pos) = self.children.iter().position(|c| c.name == name) {
            &mut self.children[pos]
        } else {
            self.children.push(FlameGraphNode::new(name));
            self.children
                .last_mut()
                .expect("just pushed; cannot be empty")
        }
    }

    /// Total number of nodes in this sub-tree (including `self`).
    pub fn node_count(&self) -> usize {
        1 + self.children.iter().map(|c| c.node_count()).sum::<usize>()
    }
}

// ---------------------------------------------------------------------------
// Helper: wall-clock nanoseconds
// ---------------------------------------------------------------------------

/// Return the number of nanoseconds since the Unix epoch.
pub(super) fn wall_clock_ns() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .as_ref()
        .map(Duration::as_nanos)
        .unwrap_or(0) as u64
}
