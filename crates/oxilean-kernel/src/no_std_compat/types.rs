//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::PlatformResult;

/// Marker type representing a future no_std–compatible hash map.
#[allow(dead_code)]
pub struct NoStdHashMap<K, V> {
    _phantom: std::marker::PhantomData<(K, V)>,
}
#[allow(dead_code)]
impl<K, V> NoStdHashMap<K, V> {
    /// Creates a placeholder instance.
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}
/// High-level capability flags for the current platform.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct PlatformCaps {
    /// Whether OS-level threading is available.
    pub threads: bool,
    /// Whether OS-level file I/O is available.
    pub file_io: bool,
    /// Whether environment variables are accessible.
    pub env_vars: bool,
    /// Whether heap allocation is available.
    pub heap: bool,
}
#[allow(dead_code)]
impl PlatformCaps {
    /// Returns capabilities for the current build target.
    pub fn detect() -> Self {
        let is_wasm = cfg!(target_arch = "wasm32");
        Self {
            threads: !is_wasm,
            file_io: !is_wasm,
            env_vars: !is_wasm,
            heap: true,
        }
    }
    /// Returns a minimal capability set (useful for testing).
    pub fn minimal() -> Self {
        Self {
            threads: false,
            file_io: false,
            env_vars: false,
            heap: true,
        }
    }
    /// Returns `true` if all capability flags are set.
    pub fn is_full() -> bool {
        let caps = Self::detect();
        caps.threads && caps.file_io && caps.env_vars && caps.heap
    }
}
/// A simple timer abstraction backed by `std::time::Instant` in std builds.
///
/// In a `no_std` build this would be backed by a hardware counter.
#[allow(dead_code)]
pub struct CrossPlatformTimer {
    start: std::time::Instant,
}
#[allow(dead_code)]
impl CrossPlatformTimer {
    /// Starts a new timer.
    pub fn start() -> Self {
        Self {
            start: std::time::Instant::now(),
        }
    }
    /// Returns elapsed time in microseconds.
    pub fn elapsed_micros(&self) -> u64 {
        self.start.elapsed().as_micros() as u64
    }
    /// Returns elapsed time in milliseconds.
    pub fn elapsed_millis(&self) -> u64 {
        self.start.elapsed().as_millis() as u64
    }
    /// Resets the timer.
    pub fn reset(&mut self) {
        self.start = std::time::Instant::now();
    }
}
/// Provides information about the current compilation target platform.
pub struct PlatformInfo;
impl PlatformInfo {
    /// Returns `true` when compiled for `wasm32` targets.
    pub fn is_wasm() -> bool {
        cfg!(target_arch = "wasm32")
    }
    /// Returns `true` when running without the standard library.
    ///
    /// This is always `false` in the current std build.
    pub fn is_no_std() -> bool {
        false
    }
    /// Returns a short name identifying the current platform.
    pub fn platform_name() -> &'static str {
        if cfg!(target_arch = "wasm32") {
            "wasm32"
        } else if cfg!(target_os = "linux") {
            "linux"
        } else if cfg!(target_os = "macos") {
            "macos"
        } else if cfg!(target_os = "windows") {
            "windows"
        } else {
            "unknown"
        }
    }
    /// Returns the size of a pointer in bytes on the current platform.
    pub fn pointer_size() -> usize {
        std::mem::size_of::<usize>()
    }
}
/// Tracks allocation statistics in builds where a custom allocator is used.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct AllocStats {
    /// Total number of allocations performed.
    pub alloc_count: usize,
    /// Total number of deallocations performed.
    pub dealloc_count: usize,
    /// Peak memory usage in bytes.
    pub peak_bytes: usize,
    /// Current memory usage in bytes.
    pub current_bytes: usize,
}
#[allow(dead_code)]
impl AllocStats {
    /// Creates a zeroed statistics record.
    pub fn new() -> Self {
        Self {
            alloc_count: 0,
            dealloc_count: 0,
            peak_bytes: 0,
            current_bytes: 0,
        }
    }
    /// Records an allocation of `bytes` bytes.
    pub fn record_alloc(&mut self, bytes: usize) {
        self.alloc_count += 1;
        self.current_bytes += bytes;
        if self.current_bytes > self.peak_bytes {
            self.peak_bytes = self.current_bytes;
        }
    }
    /// Records a deallocation of `bytes` bytes.
    pub fn record_dealloc(&mut self, bytes: usize) {
        self.dealloc_count += 1;
        self.current_bytes = self.current_bytes.saturating_sub(bytes);
    }
    /// Returns the number of live allocations.
    pub fn live_allocs(&self) -> usize {
        self.alloc_count.saturating_sub(self.dealloc_count)
    }
}
/// Static information about the build, available at runtime.
#[allow(dead_code)]
pub struct BuildInfo;
#[allow(dead_code)]
impl BuildInfo {
    /// Returns the version string from `Cargo.toml`.
    pub fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
    /// Returns the package name.
    pub fn package_name() -> &'static str {
        env!("CARGO_PKG_NAME")
    }
    /// Returns the target architecture string.
    pub fn target_arch() -> &'static str {
        "unknown"
    }
    /// Returns the target OS string.
    pub fn target_os() -> &'static str {
        "unknown"
    }
    /// Returns a one-line summary of package + version + target.
    pub fn summary() -> String {
        format!(
            "{} v{} ({}-{})",
            Self::package_name(),
            Self::version(),
            Self::target_arch(),
            Self::target_os(),
        )
    }
}
/// Byte order selection for cross-platform binary serialization.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ByteOrder {
    /// Least-significant byte first.
    LittleEndian,
    /// Most-significant byte first.
    BigEndian,
    /// Native byte order of the current target.
    Native,
}
#[allow(dead_code)]
impl ByteOrder {
    /// Returns the native byte order of the current compilation target.
    pub fn native() -> Self {
        #[cfg(target_endian = "little")]
        {
            ByteOrder::LittleEndian
        }
        #[cfg(target_endian = "big")]
        {
            ByteOrder::BigEndian
        }
    }
    /// Returns `true` if this byte order matches the native order.
    pub fn is_native(self) -> bool {
        match self {
            ByteOrder::Native => true,
            ByteOrder::LittleEndian => cfg!(target_endian = "little"),
            ByteOrder::BigEndian => cfg!(target_endian = "big"),
        }
    }
    /// Converts a `u64` to bytes in this byte order.
    pub fn u64_to_bytes(self, val: u64) -> [u8; 8] {
        match self {
            ByteOrder::LittleEndian | ByteOrder::Native => val.to_le_bytes(),
            ByteOrder::BigEndian => val.to_be_bytes(),
        }
    }
    /// Converts 8 bytes to a `u64` in this byte order.
    pub fn u64_from_bytes(self, b: [u8; 8]) -> u64 {
        match self {
            ByteOrder::LittleEndian | ByteOrder::Native => u64::from_le_bytes(b),
            ByteOrder::BigEndian => u64::from_be_bytes(b),
        }
    }
}
/// Runtime query for compile-time feature flags.
#[allow(dead_code)]
pub struct FeatureFlags;
#[allow(dead_code)]
#[allow(unexpected_cfgs)]
impl FeatureFlags {
    /// Returns `true` if the `parallel` feature is active.
    pub fn parallel_enabled() -> bool {
        cfg!(feature = "parallel")
    }
    /// Returns `true` if the `serde` feature is active.
    pub fn serde_enabled() -> bool {
        cfg!(feature = "serde")
    }
    /// Returns a sorted list of all known feature names (always returns the
    /// same set, regardless of whether each feature is active).
    pub fn known_features() -> Vec<&'static str> {
        let mut v = vec!["parallel", "serde", "tracing", "metrics", "ffi"];
        v.sort_unstable();
        v
    }
}
/// Records metadata about a linked library dependency.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct LibraryManifest {
    /// Library name.
    pub name: &'static str,
    /// Library version string.
    pub version: &'static str,
    /// Whether the library is optional.
    pub optional: bool,
}
#[allow(dead_code)]
impl LibraryManifest {
    /// Creates a required library entry.
    pub const fn required(name: &'static str, version: &'static str) -> Self {
        Self {
            name,
            version,
            optional: false,
        }
    }
    /// Creates an optional library entry.
    pub const fn optional(name: &'static str, version: &'static str) -> Self {
        Self {
            name,
            version,
            optional: true,
        }
    }
    /// Returns a human-readable description of the library entry.
    pub fn describe(&self) -> String {
        let req = if self.optional {
            "optional"
        } else {
            "required"
        };
        format!("{} v{} ({})", self.name, self.version, req)
    }
}
/// A simple page-aligned memory map abstraction.
#[allow(dead_code)]
pub struct PageMap {
    page_size: usize,
    entries: Vec<(usize, String)>,
}
#[allow(dead_code)]
impl PageMap {
    /// Creates a new page map with the given page size.
    pub fn new(page_size: usize) -> Self {
        assert!(
            page_size.is_power_of_two(),
            "page_size must be a power of two"
        );
        Self {
            page_size,
            entries: Vec::new(),
        }
    }
    /// Maps an address to a page index and optional label.
    pub fn map_page(&mut self, addr: usize, label: impl Into<String>) {
        let page = addr / self.page_size;
        if !self.entries.iter().any(|(p, _)| *p == page) {
            self.entries.push((page, label.into()));
        }
    }
    /// Returns the label for the page containing `addr`, if any.
    pub fn label_for(&self, addr: usize) -> Option<&str> {
        let page = addr / self.page_size;
        self.entries
            .iter()
            .find(|(p, _)| *p == page)
            .map(|(_, l)| l.as_str())
    }
    /// Returns the number of mapped pages.
    pub fn page_count(&self) -> usize {
        self.entries.len()
    }
    /// Returns the page size.
    pub fn page_size(&self) -> usize {
        self.page_size
    }
}
/// Runtime flags derived from environment variables (std builds only).
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct EnvFlags {
    /// Whether verbose diagnostic output is requested.
    pub verbose: bool,
    /// An optional log level override.
    pub log_level: Option<String>,
}
#[allow(dead_code)]
impl EnvFlags {
    /// Reads flags from environment variables.
    ///
    /// Falls back to defaults when variables are absent or unreadable.
    pub fn detect() -> Self {
        #[allow(unexpected_cfgs)]
        {
            #[cfg(feature = "std")]
            {
                let verbose = std::env::var("OXILEAN_VERBOSE")
                    .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                    .unwrap_or(false);
                let log_level = std::env::var("OXILEAN_LOG").ok();
                return Self { verbose, log_level };
            }
            Self {
                verbose: false,
                log_level: None,
            }
        }
    }
    /// Returns default flags (no verbose, no log level override).
    pub fn defaults() -> Self {
        Self {
            verbose: false,
            log_level: None,
        }
    }
}
/// An atomically-updated version counter, useful for cache invalidation.
#[allow(dead_code)]
pub struct AtomicVersion {
    inner: std::sync::atomic::AtomicU64,
}
#[allow(dead_code)]
impl AtomicVersion {
    /// Creates a new counter starting at zero.
    pub const fn new() -> Self {
        Self {
            inner: std::sync::atomic::AtomicU64::new(0),
        }
    }
    /// Bumps the version by one, returning the new value.
    pub fn bump(&self) -> u64 {
        self.inner.fetch_add(1, std::sync::atomic::Ordering::AcqRel) + 1
    }
    /// Reads the current version.
    pub fn current(&self) -> u64 {
        self.inner.load(std::sync::atomic::Ordering::Acquire)
    }
}
/// Describes what types and traits a `no_std` build of oxilean-kernel would require.
pub struct StdCompat;
impl StdCompat {
    /// Returns the alloc types required for a `no_std` build.
    ///
    /// These types come from the `alloc` crate when `std` is not available.
    pub fn required_alloc_types() -> Vec<&'static str> {
        vec!["Vec", "HashMap", "Box", "String"]
    }
    /// Returns the Rust features required for a `no_std` build.
    pub fn required_features() -> Vec<&'static str> {
        vec!["alloc", "core"]
    }
    /// Returns the limitations that apply in a WASM `no_std` environment.
    pub fn wasm_limitations() -> Vec<&'static str> {
        vec!["No file I/O", "No threads", "No env vars"]
    }
}
/// A table of named WASM linear memory regions.
#[allow(dead_code)]
pub struct WasmMemTable {
    regions: Vec<WasmMemRegion>,
}
#[allow(dead_code)]
impl WasmMemTable {
    /// Creates an empty table.
    pub fn new() -> Self {
        Self {
            regions: Vec::new(),
        }
    }
    /// Adds a region to the table.
    pub fn add(&mut self, region: WasmMemRegion) {
        self.regions.push(region);
    }
    /// Returns the region containing `offset`, or `None`.
    pub fn find(&self, offset: u32) -> Option<&WasmMemRegion> {
        self.regions.iter().find(|r| r.contains(offset))
    }
    /// Returns the total number of registered regions.
    pub fn len(&self) -> usize {
        self.regions.len()
    }
    /// Returns `true` if the table is empty.
    pub fn is_empty(&self) -> bool {
        self.regions.is_empty()
    }
}
/// A linear-scan register allocator stub (used by the erased code-gen backend).
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct LinearScanAllocator {
    /// Total number of available registers.
    pub num_regs: usize,
    /// For each live interval: (start, end, assigned_reg or None).
    pub intervals: Vec<(usize, usize, Option<usize>)>,
}
#[allow(dead_code)]
impl LinearScanAllocator {
    /// Creates a new allocator with `num_regs` available registers.
    pub fn new(num_regs: usize) -> Self {
        Self {
            num_regs,
            intervals: Vec::new(),
        }
    }
    /// Adds a live interval `[start, end)` and returns its interval index.
    pub fn add_interval(&mut self, start: usize, end: usize) -> usize {
        let idx = self.intervals.len();
        self.intervals.push((start, end, None));
        idx
    }
    /// Runs a simple linear-scan allocation.
    ///
    /// Each interval is greedily assigned the first free register.
    pub fn allocate(&mut self) {
        let mut free: Vec<bool> = vec![true; self.num_regs];
        self.intervals.sort_unstable_by_key(|&(s, _, _)| s);
        let mut active: Vec<(usize, usize)> = Vec::new();
        for i in 0..self.intervals.len() {
            let (start, end, _) = self.intervals[i];
            active.retain(|&(ae, ar)| {
                if ae <= start {
                    free[ar] = true;
                    false
                } else {
                    true
                }
            });
            if let Some(reg) = free.iter().position(|&f| f) {
                free[reg] = false;
                self.intervals[i].2 = Some(reg);
                active.push((end, reg));
            }
        }
    }
    /// Returns the number of intervals that were successfully allocated a register.
    pub fn allocated_count(&self) -> usize {
        self.intervals
            .iter()
            .filter(|(_, _, r)| r.is_some())
            .count()
    }
}
/// A growing buffer of abstract instructions (u32-encoded).
#[allow(dead_code)]
pub struct InstructionBuffer {
    buf: Vec<u32>,
}
#[allow(dead_code)]
impl InstructionBuffer {
    /// Creates a new empty instruction buffer.
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }
    /// Emits a 32-bit instruction word.
    pub fn emit(&mut self, word: u32) {
        self.buf.push(word);
    }
    /// Returns the number of instructions emitted.
    pub fn len(&self) -> usize {
        self.buf.len()
    }
    /// Returns `true` if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
    /// Returns the instruction at `idx`, or `None`.
    pub fn get(&self, idx: usize) -> Option<u32> {
        self.buf.get(idx).copied()
    }
    /// Patches the instruction at `idx` with `word`.
    pub fn patch(&mut self, idx: usize, word: u32) {
        if let Some(slot) = self.buf.get_mut(idx) {
            *slot = word;
        }
    }
    /// Returns a slice over all emitted instructions.
    pub fn as_slice(&self) -> &[u32] {
        &self.buf
    }
}
/// A minimal stub for an object file representation.
#[allow(dead_code)]
pub struct ObjectFile {
    /// Sections contained in this object.
    sections: Vec<(String, Vec<u8>)>,
    /// Relocation entries.
    relocs: Vec<RelocEntry>,
    /// Symbol table.
    symbols: SymbolTable,
}
#[allow(dead_code)]
impl ObjectFile {
    /// Creates an empty object file.
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
            relocs: Vec::new(),
            symbols: SymbolTable::new(),
        }
    }
    /// Adds a section with the given name and content.
    pub fn add_section(&mut self, name: impl Into<String>, data: Vec<u8>) {
        self.sections.push((name.into(), data));
    }
    /// Adds a relocation entry.
    pub fn add_reloc(&mut self, entry: RelocEntry) {
        self.relocs.push(entry);
    }
    /// Adds a symbol.
    pub fn add_symbol(&mut self, name: impl Into<String>, addr: usize) {
        self.symbols.add(name, addr);
    }
    /// Returns the number of sections.
    pub fn num_sections(&self) -> usize {
        self.sections.len()
    }
    /// Returns the number of relocation entries.
    pub fn num_relocs(&self) -> usize {
        self.relocs.len()
    }
    /// Returns the size of the named section, or `None`.
    pub fn section_size(&self, name: &str) -> Option<usize> {
        self.sections
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, data)| data.len())
    }
}
/// A RAII guard that increments a depth counter and panics if the limit is exceeded.
#[allow(dead_code)]
pub struct StackGuard<'a> {
    pub(super) depth: &'a mut usize,
    limit: usize,
}
#[allow(dead_code)]
impl<'a> StackGuard<'a> {
    /// Creates a new stack guard.  Panics if `depth >= limit`.
    pub fn new(depth: &'a mut usize, limit: usize) -> Self {
        *depth += 1;
        if *depth > limit {
            panic!(
                "StackGuard: recursion depth {} exceeds limit {}",
                *depth, limit
            );
        }
        Self { depth, limit }
    }
    /// Returns the current depth.
    pub fn current_depth(&self) -> usize {
        *self.depth
    }
}
/// Represents a handle to shared memory (stub for no_std builds).
#[allow(dead_code)]
pub struct SharedMemHandle {
    /// The raw pointer to the shared region.
    ptr: *mut u8,
    /// Size of the shared region in bytes.
    size: usize,
}
#[allow(dead_code)]
impl SharedMemHandle {
    /// Creates a null (invalid) handle.
    pub fn null() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
            size: 0,
        }
    }
    /// Returns `true` if the handle points to a valid region.
    pub fn is_valid(&self) -> bool {
        !self.ptr.is_null() && self.size > 0
    }
    /// Returns the size of the shared region.
    pub fn size(&self) -> usize {
        self.size
    }
}
/// A relocation entry in an object file.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct RelocEntry {
    /// Offset within the section where the relocation applies.
    pub offset: usize,
    /// Name of the symbol to relocate against.
    pub symbol: String,
    /// Addend to apply.
    pub addend: i64,
}
#[allow(dead_code)]
impl RelocEntry {
    /// Creates a new relocation entry.
    pub fn new(offset: usize, symbol: impl Into<String>, addend: i64) -> Self {
        Self {
            offset,
            symbol: symbol.into(),
            addend,
        }
    }
    /// Applies this relocation given the symbol's resolved address.
    ///
    /// Returns the patched address value.
    pub fn apply(&self, sym_addr: usize) -> usize {
        (sym_addr as i64 + self.addend) as usize
    }
}
/// A platform-independent error code.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    /// Operation succeeded.
    Ok = 0,
    /// Generic failure.
    Fail = 1,
    /// Allocation failure (out of memory).
    Oom = 2,
    /// Invalid argument supplied.
    InvalidArg = 3,
    /// Feature not supported on this platform.
    Unsupported = 4,
    /// Resource not found.
    NotFound = 5,
    /// Operation timed out.
    Timeout = 6,
    /// Permission denied.
    PermDenied = 7,
    /// Internal logic error (should not happen).
    InternalError = 8,
}
#[allow(dead_code)]
impl ErrorCode {
    /// Returns `true` if the code represents success.
    pub fn is_ok(self) -> bool {
        self == ErrorCode::Ok
    }
    /// Returns a short human-readable description.
    pub fn description(self) -> &'static str {
        match self {
            ErrorCode::Ok => "ok",
            ErrorCode::Fail => "generic failure",
            ErrorCode::Oom => "out of memory",
            ErrorCode::InvalidArg => "invalid argument",
            ErrorCode::Unsupported => "unsupported",
            ErrorCode::NotFound => "not found",
            ErrorCode::Timeout => "timeout",
            ErrorCode::PermDenied => "permission denied",
            ErrorCode::InternalError => "internal error",
        }
    }
    /// Converts the code to an integer.
    pub fn as_i32(self) -> i32 {
        self as i32
    }
}
/// Marker type representing a future no_std–compatible ring buffer.
#[allow(dead_code)]
pub struct NoStdRingBuf<T> {
    _phantom: std::marker::PhantomData<T>,
}
#[allow(dead_code)]
impl<T> NoStdRingBuf<T> {
    /// Creates a placeholder instance.
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}
/// A single entry point that summarises all compatibility-related queries.
#[allow(dead_code)]
pub struct CompatLayer;
#[allow(dead_code)]
impl CompatLayer {
    /// Returns a full diagnostic string describing the current build environment.
    pub fn diagnostics() -> String {
        let report = CompatibilityReport::generate();
        let triple = TargetTriple::current();
        let layout = MemoryLayout::num_cpus();
        let order = ByteOrder::native();
        let order_s = if order == ByteOrder::LittleEndian {
            "little-endian"
        } else {
            "big-endian"
        };
        format!(
            "{}\ntarget: {}\ncpus: {}\nbyte-order: {}",
            report.report_string(),
            triple.as_str(),
            layout,
            order_s,
        )
    }
    /// Returns `true` if the build is fully compatible with all known requirements.
    pub fn is_fully_compatible() -> bool {
        CompatibilityReport::generate().is_compatible()
    }
}
/// A half-open address range `[lo, hi)`.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AddrRange {
    /// Inclusive lower bound.
    pub lo: usize,
    /// Exclusive upper bound.
    pub hi: usize,
}
#[allow(dead_code)]
impl AddrRange {
    /// Creates a new address range.
    pub fn new(lo: usize, hi: usize) -> Self {
        assert!(lo <= hi, "AddrRange: lo > hi");
        Self { lo, hi }
    }
    /// Returns the size of the range in bytes.
    pub fn size(&self) -> usize {
        self.hi - self.lo
    }
    /// Returns `true` if the range is empty.
    pub fn is_empty(&self) -> bool {
        self.lo == self.hi
    }
    /// Returns `true` if `addr` is within the range.
    pub fn contains(&self, addr: usize) -> bool {
        addr >= self.lo && addr < self.hi
    }
    /// Returns `true` if the ranges overlap.
    pub fn overlaps(&self, other: &Self) -> bool {
        self.lo < other.hi && other.lo < self.hi
    }
}
/// Describes a named section within a compiled binary.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct CodeSection {
    /// Section name (e.g. `.text`, `.data`).
    pub name: &'static str,
    /// Virtual address of the section start.
    pub vaddr: usize,
    /// Size of the section in bytes.
    pub size: usize,
    /// Whether the section is executable.
    pub executable: bool,
    /// Whether the section is writable.
    pub writable: bool,
}
#[allow(dead_code)]
impl CodeSection {
    /// Creates a new code section descriptor.
    pub const fn new(
        name: &'static str,
        vaddr: usize,
        size: usize,
        exec: bool,
        write: bool,
    ) -> Self {
        Self {
            name,
            vaddr,
            size,
            executable: exec,
            writable: write,
        }
    }
    /// Returns the end virtual address (exclusive).
    pub fn vend(&self) -> usize {
        self.vaddr + self.size
    }
    /// Returns `true` if `addr` falls within this section.
    pub fn contains(&self, addr: usize) -> bool {
        addr >= self.vaddr && addr < self.vend()
    }
}
/// Represents a semver-like constraint of the form `major[.minor[.patch]]`.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct VersionConstraint {
    /// Required major version.
    pub major: u32,
    /// Optional required minor version.
    pub minor: Option<u32>,
    /// Optional required patch version.
    pub patch: Option<u32>,
}
#[allow(dead_code)]
impl VersionConstraint {
    /// Creates a constraint requiring exactly the given major version.
    pub fn major_only(major: u32) -> Self {
        Self {
            major,
            minor: None,
            patch: None,
        }
    }
    /// Creates a constraint requiring the given major and minor version.
    pub fn major_minor(major: u32, minor: u32) -> Self {
        Self {
            major,
            minor: Some(minor),
            patch: None,
        }
    }
    /// Returns `true` if `v` satisfies this constraint.
    pub fn satisfied_by(&self, v: &RuntimeVersion) -> bool {
        if v.major != self.major {
            return false;
        }
        if let Some(min) = self.minor {
            if v.minor < min {
                return false;
            }
        }
        if let Some(pat) = self.patch {
            if v.patch < pat {
                return false;
            }
        }
        true
    }
}
/// A snapshot of compile-time flags for introspection.
#[allow(dead_code)]
pub struct CompileFlags;
#[allow(dead_code)]
impl CompileFlags {
    /// Returns `true` if this is a debug build.
    pub fn is_debug() -> bool {
        cfg!(debug_assertions)
    }
    /// Returns `true` if this is a release build.
    pub fn is_release() -> bool {
        !cfg!(debug_assertions)
    }
    /// Returns `true` if overflow checks are enabled.
    pub fn overflow_checks() -> bool {
        cfg!(debug_assertions)
    }
    /// Returns the Rust edition as a string.
    pub fn rust_edition() -> &'static str {
        "2021"
    }
    /// Returns a summary of key compile flags.
    pub fn summary() -> String {
        format!(
            "debug={}, release={}, overflow_checks={}, edition={}",
            Self::is_debug(),
            Self::is_release(),
            Self::overflow_checks(),
            Self::rust_edition(),
        )
    }
}
/// A compile-time or runtime string that works in both std and `no_std`.
///
/// In std builds we can use `String`; in `no_std` we keep a `&'static str`.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum StaticStr {
    /// A string known at compile time.
    Static(&'static str),
    /// A heap-allocated string (std builds only).
    Owned(String),
}
#[allow(dead_code)]
impl StaticStr {
    /// Creates a `Static` variant.
    pub fn from_static(s: &'static str) -> Self {
        StaticStr::Static(s)
    }
    /// Creates an `Owned` variant from a `String`.
    pub fn from_owned(s: String) -> Self {
        StaticStr::Owned(s)
    }
    /// Returns the string as a `&str`.
    pub fn as_str(&self) -> &str {
        match self {
            StaticStr::Static(s) => s,
            StaticStr::Owned(s) => s.as_str(),
        }
    }
    /// Returns `true` if the string is empty.
    pub fn is_empty(&self) -> bool {
        self.as_str().is_empty()
    }
    /// Returns the length in bytes.
    pub fn len(&self) -> usize {
        self.as_str().len()
    }
}
/// Lazily initialises a value only in std builds.
///
/// In a hypothetical `no_std` build this would need to be replaced by a
/// spin-lock–based once cell from an external crate.
#[allow(dead_code)]
pub struct ConditionalInit<T> {
    inner: Option<T>,
}
#[allow(dead_code)]
impl<T> ConditionalInit<T> {
    /// Creates an uninitialised slot.
    pub fn uninit() -> Self {
        Self { inner: None }
    }
    /// Initialises the slot with `val`.  Panics if already initialised.
    pub fn init(&mut self, val: T) {
        assert!(self.inner.is_none(), "ConditionalInit: already initialised");
        self.inner = Some(val);
    }
    /// Returns a reference to the inner value.  Panics if not yet initialised.
    pub fn get(&self) -> &T {
        self.inner
            .as_ref()
            .expect("ConditionalInit: not yet initialised")
    }
    /// Returns `true` if the slot has been initialised.
    pub fn is_init(&self) -> bool {
        self.inner.is_some()
    }
}
/// A registry of named platform shims.
///
/// Shims are lightweight adapters that provide a unified API over
/// platform-specific functionality.
#[allow(dead_code)]
pub struct ShimRegistry {
    names: Vec<String>,
    enabled: Vec<bool>,
}
#[allow(dead_code)]
impl ShimRegistry {
    /// Creates an empty shim registry.
    pub fn new() -> Self {
        Self {
            names: Vec::new(),
            enabled: Vec::new(),
        }
    }
    /// Registers a shim, returning its index.
    pub fn register(&mut self, name: impl Into<String>, enabled: bool) -> usize {
        let idx = self.names.len();
        self.names.push(name.into());
        self.enabled.push(enabled);
        idx
    }
    /// Returns `true` if the shim at `idx` is enabled.
    pub fn is_enabled(&self, idx: usize) -> bool {
        self.enabled.get(idx).copied().unwrap_or(false)
    }
    /// Returns the name of the shim at `idx`.
    pub fn name(&self, idx: usize) -> Option<&str> {
        self.names.get(idx).map(|s| s.as_str())
    }
    /// Returns the total number of registered shims.
    pub fn count(&self) -> usize {
        self.names.len()
    }
    /// Returns the count of enabled shims.
    pub fn enabled_count(&self) -> usize {
        self.enabled.iter().filter(|&&e| e).count()
    }
}
/// An atomic counter suitable for use in both single- and multi-threaded builds.
#[allow(dead_code)]
pub struct CounterCell {
    inner: std::sync::atomic::AtomicU64,
}
#[allow(dead_code)]
impl CounterCell {
    /// Creates a new counter initialised to zero.
    pub const fn zero() -> Self {
        Self {
            inner: std::sync::atomic::AtomicU64::new(0),
        }
    }
    /// Increments the counter by one.
    pub fn inc(&self) {
        self.inner
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    /// Adds `n` to the counter.
    pub fn add(&self, n: u64) {
        self.inner
            .fetch_add(n, std::sync::atomic::Ordering::Relaxed);
    }
    /// Resets the counter to zero.
    pub fn reset(&self) {
        self.inner.store(0, std::sync::atomic::Ordering::Relaxed);
    }
    /// Reads the current counter value.
    pub fn get(&self) -> u64 {
        self.inner.load(std::sync::atomic::Ordering::Relaxed)
    }
}
/// Tracks which WASM features are required by a build configuration.
#[derive(Default)]
pub struct WasmFeatureSet {
    /// The set of required WASM feature strings.
    pub features: Vec<String>,
}
impl WasmFeatureSet {
    /// Creates an empty feature set.
    pub fn new() -> Self {
        Self::default()
    }
    /// Adds a required WASM feature if it is not already present.
    pub fn require(&mut self, feature: &str) {
        if !self.has(feature) {
            self.features.push(feature.to_string());
        }
    }
    /// Returns `true` if `feature` is in the required set.
    pub fn has(&self, feature: &str) -> bool {
        self.features.iter().any(|f| f == feature)
    }
    /// Returns a [`WasmFeatureSet`] pre-populated with common WASM features
    /// needed for a proof-checking kernel.
    pub fn standard_wasm_features() -> Self {
        let mut set = Self::new();
        set.require("mutable-globals");
        set.require("sign-extension");
        set.require("bulk-memory");
        set.require("reference-types");
        set.require("simd128");
        set
    }
}
/// A reusable byte buffer that avoids repeated heap allocations.
#[allow(dead_code)]
pub struct ScratchBuffer {
    data: Vec<u8>,
}
#[allow(dead_code)]
impl ScratchBuffer {
    /// Creates a new scratch buffer with the given initial capacity.
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            data: Vec::with_capacity(cap),
        }
    }
    /// Returns a mutable reference to the buffer, cleared to zero length.
    pub fn fresh(&mut self) -> &mut Vec<u8> {
        self.data.clear();
        &mut self.data
    }
    /// Returns the current capacity of the buffer.
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }
    /// Returns the current length of the buffer.
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// Returns `true` if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
/// A matrix that records pairwise compatibility between components.
#[allow(dead_code)]
pub struct CompatMatrix {
    /// Ordered list of component names.
    components: Vec<String>,
    /// `compatible[i][j]` = whether components `i` and `j` are compatible.
    compatible: Vec<Vec<bool>>,
}
#[allow(dead_code)]
impl CompatMatrix {
    /// Creates an empty compatibility matrix.
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            compatible: Vec::new(),
        }
    }
    /// Registers a component and returns its index.
    pub fn add_component(&mut self, name: impl Into<String>) -> usize {
        let idx = self.components.len();
        self.components.push(name.into());
        for row in self.compatible.iter_mut() {
            row.push(false);
        }
        let mut row = vec![false; idx + 1];
        row[idx] = true;
        self.compatible.push(row);
        idx
    }
    /// Marks components `a` and `b` as mutually compatible.
    pub fn mark_compatible(&mut self, a: usize, b: usize) {
        if a < self.compatible.len() && b < self.compatible[a].len() {
            self.compatible[a][b] = true;
        }
        if b < self.compatible.len() && a < self.compatible[b].len() {
            self.compatible[b][a] = true;
        }
    }
    /// Returns whether components `a` and `b` are compatible.
    pub fn is_compatible(&self, a: usize, b: usize) -> bool {
        self.compatible
            .get(a)
            .and_then(|row| row.get(b))
            .copied()
            .unwrap_or(false)
    }
    /// Returns the number of registered components.
    pub fn num_components(&self) -> usize {
        self.components.len()
    }
}
/// A snapshot of selected `cfg!()` values for diagnostics.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct CfgSnapshot {
    /// The target architecture string.
    pub arch: &'static str,
    /// The target operating system string.
    pub os: &'static str,
    /// Whether the build has debug assertions.
    pub debug: bool,
    /// Whether this is a 64-bit pointer platform.
    pub is_64bit: bool,
}
#[allow(dead_code)]
impl CfgSnapshot {
    /// Captures the current cfg snapshot.
    pub fn capture() -> Self {
        Self {
            arch: "unknown",
            os: "unknown",
            debug: cfg!(debug_assertions),
            is_64bit: std::mem::size_of::<usize>() == 8,
        }
    }
}
/// Configuration for the global allocator in constrained environments.
pub struct AllocConfig {
    /// Whether to use the global allocator (always true in std builds).
    pub use_global_allocator: bool,
    /// Maximum heap size in bytes, if constrained. `None` means unlimited.
    pub max_heap_bytes: Option<usize>,
}
impl AllocConfig {
    /// Returns the default allocator configuration for a standard build.
    pub fn new() -> Self {
        Self::default()
    }
    /// Returns allocator configuration suitable for `wasm32` targets.
    ///
    /// WASM linear memory starts small and grows on demand; a 256 MiB cap is
    /// a reasonable default for proof-checking workloads.
    pub fn for_wasm() -> Self {
        Self {
            use_global_allocator: true,
            max_heap_bytes: Some(256 * 1024 * 1024),
        }
    }
    /// Returns a human-readable description of this configuration.
    pub fn describe(&self) -> String {
        let allocator = if self.use_global_allocator {
            "global allocator"
        } else {
            "custom allocator"
        };
        match self.max_heap_bytes {
            Some(bytes) => {
                format!(
                    "{allocator}, max heap: {} MiB ({} bytes)",
                    bytes / (1024 * 1024),
                    bytes
                )
            }
            None => format!("{allocator}, max heap: unlimited"),
        }
    }
}
/// A report describing whether the current build environment is compatible
/// with the oxilean-kernel requirements.
pub struct CompatibilityReport {
    /// Name of the detected platform.
    pub platform: String,
    /// Whether the standard library is available.
    pub std_available: bool,
    /// Whether an allocator (`alloc` crate or `std`) is available.
    pub alloc_available: bool,
    /// List of detected compatibility issues (empty means fully compatible).
    pub issues: Vec<String>,
}
impl CompatibilityReport {
    /// Generates a compatibility report for the current compilation environment.
    pub fn generate() -> Self {
        let platform = PlatformInfo::platform_name().to_string();
        let std_available = !PlatformInfo::is_no_std();
        let alloc_available = true;
        let mut issues = Vec::new();
        if PlatformInfo::is_wasm() {
            issues.push("WASM: file I/O is unavailable".to_string());
            issues.push("WASM: multi-threading requires shared memory flag".to_string());
        }
        Self {
            platform,
            std_available,
            alloc_available,
            issues,
        }
    }
    /// Returns `true` if no compatibility issues were detected.
    pub fn is_compatible(&self) -> bool {
        self.issues.is_empty()
    }
    /// Returns a human-readable summary of the report.
    pub fn report_string(&self) -> String {
        let status = if self.is_compatible() {
            "COMPATIBLE"
        } else {
            "ISSUES DETECTED"
        };
        let mut lines = vec![
            format!("Platform:       {}", self.platform),
            format!("std available:  {}", self.std_available),
            format!("alloc available:{}", self.alloc_available),
            format!("Status:         {status}"),
        ];
        for issue in &self.issues {
            lines.push(format!("  - {issue}"));
        }
        lines.join("\n")
    }
}
/// A simple bump allocator simulation for WASM linear memory.
#[allow(dead_code)]
pub struct BumpAlloc {
    base: u32,
    limit: u32,
    top: u32,
}
#[allow(dead_code)]
impl BumpAlloc {
    /// Creates a bump allocator over `[base, base + size)`.
    pub fn new(base: u32, size: u32) -> Self {
        Self {
            base,
            limit: base.saturating_add(size),
            top: base,
        }
    }
    /// Allocates `bytes` bytes with `align` alignment.
    /// Returns the allocated base address, or `None` if out of memory.
    pub fn alloc(&mut self, bytes: u32, align: u32) -> Option<u32> {
        let aligned_top = (self.top.saturating_add(align - 1)) & !(align - 1);
        let new_top = aligned_top.checked_add(bytes)?;
        if new_top > self.limit {
            return None;
        }
        self.top = new_top;
        Some(aligned_top)
    }
    /// Resets the allocator, freeing all allocations.
    pub fn reset(&mut self) {
        self.top = self.base;
    }
    /// Returns the number of bytes allocated.
    pub fn used(&self) -> u32 {
        self.top - self.base
    }
    /// Returns the number of bytes remaining.
    pub fn remaining(&self) -> u32 {
        self.limit.saturating_sub(self.top)
    }
}
/// Abstracts an OS resource handle in a way that degrades gracefully in no_std.
#[allow(dead_code)]
#[derive(Debug)]
pub enum OsResource {
    /// A file descriptor (std builds only).
    FileDescriptor(i32),
    /// A placeholder for no_std builds where OS resources are unavailable.
    Unavailable,
}
#[allow(dead_code)]
impl OsResource {
    /// Returns `true` if the resource is available.
    pub fn is_available(&self) -> bool {
        matches!(self, OsResource::FileDescriptor(_))
    }
    /// Returns the underlying file descriptor, or `None`.
    pub fn fd(&self) -> Option<i32> {
        match self {
            OsResource::FileDescriptor(fd) => Some(*fd),
            OsResource::Unavailable => None,
        }
    }
}
/// Describes the memory layout characteristics of the current platform.
#[allow(dead_code)]
pub struct MemoryLayout;
#[allow(dead_code)]
impl MemoryLayout {
    /// Returns the alignment (in bytes) required for SIMD operations.
    pub fn simd_alignment() -> usize {
        if cfg!(target_arch = "wasm32") {
            16
        } else {
            32
        }
    }
    /// Returns the size of a cache line on the current platform.
    pub fn cache_line_size() -> usize {
        if cfg!(target_arch = "aarch64") {
            128
        } else {
            64
        }
    }
    /// Returns `true` if the platform supports unaligned memory accesses.
    pub fn supports_unaligned_access() -> bool {
        cfg!(any(
            target_arch = "x86",
            target_arch = "x86_64",
            target_arch = "aarch64",
            target_arch = "wasm32"
        ))
    }
    /// Returns the number of physical CPUs (always 1 for WASM).
    pub fn num_cpus() -> usize {
        if cfg!(target_arch = "wasm32") {
            1
        } else {
            std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1)
        }
    }
}
/// A simple flag backed by an `AtomicBool`, suitable for `no_std` with atomics.
#[allow(dead_code)]
pub struct SpinFlag {
    inner: std::sync::atomic::AtomicBool,
}
#[allow(dead_code)]
impl SpinFlag {
    /// Creates a new `SpinFlag` with the given initial value.
    pub const fn new(val: bool) -> Self {
        Self {
            inner: std::sync::atomic::AtomicBool::new(val),
        }
    }
    /// Sets the flag to `true`.
    pub fn set(&self) {
        self.inner.store(true, std::sync::atomic::Ordering::Release);
    }
    /// Clears the flag.
    pub fn clear(&self) {
        self.inner
            .store(false, std::sync::atomic::Ordering::Release);
    }
    /// Returns the current value.
    pub fn get(&self) -> bool {
        self.inner.load(std::sync::atomic::Ordering::Acquire)
    }
    /// Atomically sets the flag, returning the old value.
    pub fn test_and_set(&self) -> bool {
        self.inner.swap(true, std::sync::atomic::Ordering::AcqRel)
    }
}
/// Versioning information for the OxiLean runtime.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct RuntimeVersion {
    /// Major version component.
    pub major: u32,
    /// Minor version component.
    pub minor: u32,
    /// Patch version component.
    pub patch: u32,
    /// Optional pre-release label (e.g. `"alpha.1"`).
    pub pre: Option<&'static str>,
}
#[allow(dead_code)]
impl RuntimeVersion {
    /// The current runtime version.
    pub const CURRENT: Self = Self {
        major: 0,
        minor: 1,
        patch: 0,
        pre: Some("dev"),
    };
    /// Returns `true` if `other` is compatible with this version (same major).
    pub fn is_compatible_with(&self, other: &Self) -> bool {
        self.major == other.major
    }
    /// Returns a dotted version string.
    pub fn as_str(&self) -> String {
        match self.pre {
            Some(pre) => format!("{}.{}.{}-{}", self.major, self.minor, self.patch, pre),
            None => format!("{}.{}.{}", self.major, self.minor, self.patch),
        }
    }
}
/// A fuel/budget counter used to bound the work done by recursive algorithms.
#[allow(dead_code)]
pub struct Budget {
    remaining: usize,
}
#[allow(dead_code)]
impl Budget {
    /// Creates a budget with `n` units of fuel.
    pub fn new(n: usize) -> Self {
        Self { remaining: n }
    }
    /// Consumes `cost` units of fuel.  Returns `Ok(())` or `Err(ErrorCode::Timeout)`.
    pub fn consume(&mut self, cost: usize) -> PlatformResult<()> {
        if self.remaining >= cost {
            self.remaining -= cost;
            Ok(())
        } else {
            Err(ErrorCode::Timeout)
        }
    }
    /// Returns the remaining budget.
    pub fn remaining(&self) -> usize {
        self.remaining
    }
    /// Returns `true` if there is budget remaining.
    pub fn has_remaining(&self) -> bool {
        self.remaining > 0
    }
    /// Adds `n` additional units of fuel.
    pub fn refuel(&mut self, n: usize) {
        self.remaining = self.remaining.saturating_add(n);
    }
}
/// Abstracts over panic-on-error vs. return-error in constrained builds.
#[allow(dead_code)]
pub struct PanicSink;
#[allow(dead_code)]
impl PanicSink {
    /// In a std build, panics with `msg`.  In a `no_std` build this would
    /// write to a pre-registered error buffer instead.
    pub fn fail(msg: &str) -> ! {
        panic!("{}", msg);
    }
    /// Asserts `cond`, calling `fail` if it is `false`.
    pub fn assert(cond: bool, msg: &str) {
        if !cond {
            Self::fail(msg);
        }
    }
}
/// A bitset of capability flags.
#[allow(dead_code)]
pub struct CapabilitySet {
    bits: u64,
}
#[allow(dead_code)]
impl CapabilitySet {
    /// The empty capability set.
    pub const EMPTY: u64 = 0;
    /// Capability bit: threading support.
    pub const THREADS: u64 = 1 << 0;
    /// Capability bit: file I/O support.
    pub const FILE_IO: u64 = 1 << 1;
    /// Capability bit: network support.
    pub const NETWORK: u64 = 1 << 2;
    /// Capability bit: SIMD support.
    pub const SIMD: u64 = 1 << 3;
    /// Capability bit: memory-mapped files.
    pub const MMAP: u64 = 1 << 4;
    /// Creates an empty capability set.
    pub fn empty() -> Self {
        Self { bits: 0 }
    }
    /// Creates a capability set with all capabilities enabled.
    pub fn all() -> Self {
        Self { bits: u64::MAX }
    }
    /// Adds a capability bit.
    pub fn add(&mut self, bit: u64) {
        self.bits |= bit;
    }
    /// Removes a capability bit.
    pub fn remove(&mut self, bit: u64) {
        self.bits &= !bit;
    }
    /// Returns `true` if the capability bit is set.
    pub fn has(&self, bit: u64) -> bool {
        self.bits & bit != 0
    }
    /// Returns the underlying bit pattern.
    pub fn bits(&self) -> u64 {
        self.bits
    }
}
/// Represents a named memory region for WASM linear memory management.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct WasmMemRegion {
    /// Region identifier.
    pub id: u32,
    /// Base offset within linear memory.
    pub base: u32,
    /// Size in bytes.
    pub size: u32,
    /// Human-readable label.
    pub label: String,
}
#[allow(dead_code)]
impl WasmMemRegion {
    /// Creates a new memory region descriptor.
    pub fn new(id: u32, base: u32, size: u32, label: impl Into<String>) -> Self {
        Self {
            id,
            base,
            size,
            label: label.into(),
        }
    }
    /// Returns the end offset (exclusive) of the region.
    pub fn end(&self) -> u32 {
        self.base.saturating_add(self.size)
    }
    /// Returns `true` if `offset` falls within this region.
    pub fn contains(&self, offset: u32) -> bool {
        offset >= self.base && offset < self.end()
    }
}
/// A minimal target-triple descriptor for the current compilation.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct TargetTriple {
    /// The ISA component (e.g. `"x86_64"`, `"aarch64"`, `"wasm32"`).
    pub arch: &'static str,
    /// The vendor component (e.g. `"unknown"`, `"apple"`).
    pub vendor: &'static str,
    /// The OS component (e.g. `"linux"`, `"macos"`, `"none"`).
    pub os: &'static str,
}
#[allow(dead_code)]
impl TargetTriple {
    /// Returns the target triple for the current compilation.
    pub fn current() -> Self {
        Self {
            arch: "unknown",
            vendor: "unknown",
            os: "unknown",
        }
    }
    /// Returns a canonical `arch-vendor-os` string.
    pub fn as_str(&self) -> String {
        format!("{}-{}-{}", self.arch, self.vendor, self.os)
    }
}
/// A simple symbol table mapping names to addresses.
#[allow(dead_code)]
pub struct SymbolTable {
    symbols: Vec<(String, usize)>,
}
#[allow(dead_code)]
impl SymbolTable {
    /// Creates an empty symbol table.
    pub fn new() -> Self {
        Self {
            symbols: Vec::new(),
        }
    }
    /// Adds a symbol.
    pub fn add(&mut self, name: impl Into<String>, addr: usize) {
        self.symbols.push((name.into(), addr));
    }
    /// Looks up a symbol by name.
    pub fn lookup(&self, name: &str) -> Option<usize> {
        self.symbols
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, a)| *a)
    }
    /// Looks up the name of the symbol nearest to `addr`.
    pub fn nearest(&self, addr: usize) -> Option<&str> {
        self.symbols
            .iter()
            .filter(|(_, a)| *a <= addr)
            .min_by_key(|(_, a)| addr - a)
            .map(|(n, _)| n.as_str())
    }
    /// Returns the total number of symbols.
    pub fn len(&self) -> usize {
        self.symbols.len()
    }
    /// Returns `true` if the table is empty.
    pub fn is_empty(&self) -> bool {
        self.symbols.is_empty()
    }
}
