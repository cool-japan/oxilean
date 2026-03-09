//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    AddrRange, AllocConfig, AllocStats, AtomicVersion, Budget, BuildInfo, BumpAlloc, ByteOrder,
    CapabilitySet, CfgSnapshot, CodeSection, CompatLayer, CompatMatrix, CompatibilityReport,
    CompileFlags, ConditionalInit, CounterCell, CrossPlatformTimer, ErrorCode, FeatureFlags,
    InstructionBuffer, LibraryManifest, LinearScanAllocator, MemoryLayout, ObjectFile, OsResource,
    PageMap, PlatformCaps, PlatformInfo, RelocEntry, RuntimeVersion, ScratchBuffer, ShimRegistry,
    SpinFlag, StackGuard, StaticStr, StdCompat, SymbolTable, VersionConstraint, WasmFeatureSet,
    WasmMemRegion, WasmMemTable,
};

/// Macro that documents what would change in a `no_std` build.
///
/// In std mode this expands to nothing. In a hypothetical `no_std` build,
/// the `$no_std_block` would be active instead of `$std_block`.
#[macro_export]
macro_rules! cfg_if_std {
    (if_std { $($std_item:item)* } else { $($no_std_item:item)* }) => {
        $($std_item)*
    };
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_std_compat_alloc_types() {
        let types = StdCompat::required_alloc_types();
        assert!(types.contains(&"Vec"));
        assert!(types.contains(&"HashMap"));
        assert!(types.contains(&"Box"));
        assert!(types.contains(&"String"));
    }
    #[test]
    fn test_std_compat_required_features() {
        let features = StdCompat::required_features();
        assert!(features.contains(&"alloc"));
        assert!(features.contains(&"core"));
    }
    #[test]
    fn test_std_compat_wasm_limitations() {
        let limitations = StdCompat::wasm_limitations();
        assert!(limitations.contains(&"No file I/O"));
        assert!(limitations.contains(&"No threads"));
        assert!(limitations.contains(&"No env vars"));
    }
    #[test]
    fn test_alloc_config_default() {
        let cfg = AllocConfig::default();
        assert!(cfg.use_global_allocator);
        assert!(cfg.max_heap_bytes.is_none());
    }
    #[test]
    fn test_alloc_config_for_wasm() {
        let cfg = AllocConfig::for_wasm();
        assert!(cfg.use_global_allocator);
        assert_eq!(cfg.max_heap_bytes, Some(256 * 1024 * 1024));
        let desc = cfg.describe();
        assert!(desc.contains("256"));
    }
    #[test]
    fn test_platform_info() {
        assert!(!PlatformInfo::is_no_std());
        let ps = PlatformInfo::pointer_size();
        assert!(ps == 4 || ps == 8);
        assert!(!PlatformInfo::platform_name().is_empty());
    }
    #[test]
    fn test_wasm_feature_set() {
        let mut set = WasmFeatureSet::new();
        assert!(!set.has("bulk-memory"));
        set.require("bulk-memory");
        assert!(set.has("bulk-memory"));
        set.require("bulk-memory");
        assert_eq!(set.features.len(), 1);
        let standard = WasmFeatureSet::standard_wasm_features();
        assert!(standard.has("mutable-globals"));
        assert!(standard.has("bulk-memory"));
        assert!(standard.has("reference-types"));
    }
    #[test]
    fn test_compatibility_report() {
        let report = CompatibilityReport::generate();
        assert!(!report.platform.is_empty());
        assert!(report.std_available);
        assert!(report.alloc_available);
        if !PlatformInfo::is_wasm() {
            assert!(report.is_compatible());
        }
        let summary = report.report_string();
        assert!(summary.contains("Platform:"));
        assert!(summary.contains("std available:"));
    }
}
#[cfg(test)]
mod tests_compat_ext {
    use super::*;
    #[test]
    fn test_feature_flags() {
        let _ = FeatureFlags::parallel_enabled();
        let _ = FeatureFlags::serde_enabled();
        let feats = FeatureFlags::known_features();
        assert!(!feats.is_empty());
    }
    #[test]
    fn test_conditional_init() {
        let mut slot: ConditionalInit<u32> = ConditionalInit::uninit();
        assert!(!slot.is_init());
        slot.init(42);
        assert!(slot.is_init());
        assert_eq!(*slot.get(), 42);
    }
    #[test]
    #[should_panic]
    fn test_conditional_init_double_init() {
        let mut slot: ConditionalInit<u32> = ConditionalInit::uninit();
        slot.init(1);
        slot.init(2);
    }
    #[test]
    fn test_platform_caps() {
        let caps = PlatformCaps::detect();
        assert!(caps.heap);
    }
    #[test]
    fn test_byte_order_round_trip() {
        let val = 0xDEAD_BEEF_CAFE_BABEu64;
        let le_bytes = ByteOrder::LittleEndian.u64_to_bytes(val);
        let back = ByteOrder::LittleEndian.u64_from_bytes(le_bytes);
        assert_eq!(back, val);
        let be_bytes = ByteOrder::BigEndian.u64_to_bytes(val);
        let back_be = ByteOrder::BigEndian.u64_from_bytes(be_bytes);
        assert_eq!(back_be, val);
    }
    #[test]
    fn test_static_str() {
        let s = StaticStr::from_static("hello");
        assert_eq!(s.as_str(), "hello");
        assert!(!s.is_empty());
        assert_eq!(s.len(), 5);
        let owned = StaticStr::from_owned("world".to_string());
        assert_eq!(owned.as_str(), "world");
        assert_eq!(format!("{owned}"), "world");
    }
    #[test]
    fn test_alloc_stats() {
        let mut stats = AllocStats::new();
        stats.record_alloc(100);
        stats.record_alloc(200);
        assert_eq!(stats.alloc_count, 2);
        assert_eq!(stats.current_bytes, 300);
        assert_eq!(stats.peak_bytes, 300);
        stats.record_dealloc(100);
        assert_eq!(stats.live_allocs(), 1);
        assert_eq!(stats.current_bytes, 200);
    }
    #[test]
    fn test_scratch_buffer() {
        let mut buf = ScratchBuffer::with_capacity(64);
        assert!(buf.capacity() >= 64);
        {
            let v = buf.fresh();
            v.push(1);
            v.push(2);
        }
        assert_eq!(buf.len(), 2);
        buf.fresh();
        assert_eq!(buf.len(), 0);
    }
    #[test]
    fn test_spin_flag() {
        let flag = SpinFlag::new(false);
        assert!(!flag.get());
        flag.set();
        assert!(flag.get());
        let old = flag.test_and_set();
        assert!(old);
        flag.clear();
        assert!(!flag.get());
    }
    #[test]
    fn test_counter_cell() {
        let c = CounterCell::zero();
        assert_eq!(c.get(), 0);
        c.inc();
        c.inc();
        assert_eq!(c.get(), 2);
        c.add(10);
        assert_eq!(c.get(), 12);
        c.reset();
        assert_eq!(c.get(), 0);
    }
    #[test]
    fn test_os_resource() {
        let r = OsResource::FileDescriptor(3);
        assert!(r.is_available());
        assert_eq!(r.fd(), Some(3));
        let u = OsResource::Unavailable;
        assert!(!u.is_available());
        assert_eq!(u.fd(), None);
    }
    #[test]
    fn test_runtime_version() {
        let v = RuntimeVersion::CURRENT;
        let s = v.as_str();
        assert!(s.starts_with('0'));
        let other = RuntimeVersion {
            major: 0,
            minor: 99,
            patch: 0,
            pre: None,
        };
        assert!(v.is_compatible_with(&other));
        let incompat = RuntimeVersion {
            major: 1,
            minor: 0,
            patch: 0,
            pre: None,
        };
        assert!(!v.is_compatible_with(&incompat));
    }
    #[test]
    fn test_build_info() {
        assert!(!BuildInfo::version().is_empty());
        assert!(!BuildInfo::package_name().is_empty());
        assert!(!BuildInfo::summary().is_empty());
    }
    #[test]
    fn test_memory_layout() {
        let _align = MemoryLayout::simd_alignment();
        let _cl = MemoryLayout::cache_line_size();
        let ncpus = MemoryLayout::num_cpus();
        assert!(ncpus >= 1);
    }
    #[test]
    fn test_compat_layer() {
        let diag = CompatLayer::diagnostics();
        assert!(!diag.is_empty());
    }
}
/// A `Result` type parameterised over `ErrorCode` for platform operations.
#[allow(dead_code)]
pub type PlatformResult<T> = Result<T, ErrorCode>;
/// Converts a boolean success flag into a `PlatformResult<()>`.
#[allow(dead_code)]
pub fn bool_to_platform_result(ok: bool) -> PlatformResult<()> {
    if ok {
        Ok(())
    } else {
        Err(ErrorCode::Fail)
    }
}
/// Converts a `&str` to an OS path-like byte sequence.
///
/// On Unix, this is a no-op (UTF-8 bytes). On Windows, this would convert to
/// WTF-16 in a full implementation; here we return the UTF-8 bytes unchanged.
#[allow(dead_code)]
pub fn str_to_os_bytes(s: &str) -> Vec<u8> {
    s.as_bytes().to_vec()
}
/// Converts OS path bytes back to a `String`, replacing invalid UTF-8.
#[allow(dead_code)]
pub fn os_bytes_to_string(b: &[u8]) -> String {
    String::from_utf8_lossy(b).into_owned()
}
#[cfg(test)]
mod tests_compat_ext2 {
    use super::*;
    #[test]
    fn test_error_code() {
        assert!(ErrorCode::Ok.is_ok());
        assert!(!ErrorCode::Fail.is_ok());
        assert!(!ErrorCode::Oom.description().is_empty());
        assert_eq!(ErrorCode::Ok.as_i32(), 0);
    }
    #[test]
    fn test_bool_to_platform_result() {
        assert!(bool_to_platform_result(true).is_ok());
        assert!(bool_to_platform_result(false).is_err());
    }
    #[test]
    fn test_stack_guard() {
        let mut depth = 0usize;
        {
            let g = StackGuard::new(&mut depth, 10);
            assert_eq!(g.current_depth(), 1);
        }
        assert_eq!(depth, 0);
    }
    #[test]
    fn test_budget() {
        let mut b = Budget::new(10);
        assert!(b.consume(5).is_ok());
        assert_eq!(b.remaining(), 5);
        assert!(b.consume(6).is_err());
        b.refuel(20);
        assert!(b.has_remaining());
    }
    #[test]
    fn test_compat_matrix() {
        let mut m = CompatMatrix::new();
        let a = m.add_component("alpha");
        let b = m.add_component("beta");
        assert!(m.is_compatible(a, a));
        assert!(!m.is_compatible(a, b));
        m.mark_compatible(a, b);
        assert!(m.is_compatible(a, b));
        assert!(m.is_compatible(b, a));
    }
    #[test]
    fn test_version_constraint() {
        let c = VersionConstraint::major_minor(0, 1);
        let v_ok = RuntimeVersion {
            major: 0,
            minor: 1,
            patch: 0,
            pre: None,
        };
        let v_bad = RuntimeVersion {
            major: 1,
            minor: 0,
            patch: 0,
            pre: None,
        };
        assert!(c.satisfied_by(&v_ok));
        assert!(!c.satisfied_by(&v_bad));
    }
    #[test]
    fn test_atomic_version() {
        let av = AtomicVersion::new();
        assert_eq!(av.current(), 0);
        assert_eq!(av.bump(), 1);
        assert_eq!(av.bump(), 2);
    }
    #[test]
    fn test_capability_set() {
        let mut caps = CapabilitySet::empty();
        caps.add(CapabilitySet::THREADS);
        assert!(caps.has(CapabilitySet::THREADS));
        assert!(!caps.has(CapabilitySet::NETWORK));
        caps.remove(CapabilitySet::THREADS);
        assert!(!caps.has(CapabilitySet::THREADS));
    }
    #[test]
    fn test_shim_registry() {
        let mut reg = ShimRegistry::new();
        let i0 = reg.register("file_shim", true);
        let i1 = reg.register("net_shim", false);
        assert!(reg.is_enabled(i0));
        assert!(!reg.is_enabled(i1));
        assert_eq!(reg.name(i0), Some("file_shim"));
        assert_eq!(reg.count(), 2);
        assert_eq!(reg.enabled_count(), 1);
    }
    #[test]
    fn test_compile_flags() {
        let s = CompileFlags::summary();
        assert!(s.contains("debug="));
        assert!(s.contains("edition=2021"));
    }
    #[test]
    fn test_library_manifest() {
        let lib = LibraryManifest::required("core", "1.0.0");
        assert!(!lib.optional);
        let desc = lib.describe();
        assert!(desc.contains("core"));
        let opt = LibraryManifest::optional("tracing", "0.1.1");
        assert!(opt.optional);
    }
    #[test]
    fn test_cfg_snapshot() {
        let snap = CfgSnapshot::capture();
        assert!(!snap.arch.is_empty());
        assert!(!snap.os.is_empty());
        let s = snap.to_string();
        assert!(s.contains("arch="));
    }
    #[test]
    fn test_cross_platform_timer() {
        let timer = CrossPlatformTimer::start();
        let micros = timer.elapsed_micros();
        assert!(micros < 10_000_000);
    }
    #[test]
    fn test_os_bytes_round_trip() {
        let s = "hello/world";
        let b = str_to_os_bytes(s);
        let s2 = os_bytes_to_string(&b);
        assert_eq!(s, s2);
    }
}
#[cfg(test)]
mod tests_compat_ext3 {
    use super::*;
    #[test]
    fn test_wasm_mem_region() {
        let r = WasmMemRegion::new(0, 1024, 4096, "heap");
        assert_eq!(r.end(), 5120);
        assert!(r.contains(1024));
        assert!(r.contains(5119));
        assert!(!r.contains(5120));
    }
    #[test]
    fn test_wasm_mem_table() {
        let mut tbl = WasmMemTable::new();
        tbl.add(WasmMemRegion::new(0, 0, 1024, "stack"));
        tbl.add(WasmMemRegion::new(1, 1024, 4096, "heap"));
        assert_eq!(tbl.len(), 2);
        let found = tbl.find(2000);
        assert!(found.is_some());
        assert_eq!(found.expect("found should be valid").label, "heap");
        assert!(tbl.find(5200).is_none());
    }
    #[test]
    fn test_bump_alloc() {
        let mut alloc = BumpAlloc::new(0, 1024);
        let a = alloc.alloc(64, 8).expect("a should be present");
        assert_eq!(a, 0);
        let b = alloc.alloc(64, 8).expect("b should be present");
        assert_eq!(b, 64);
        assert_eq!(alloc.used(), 128);
        alloc.reset();
        assert_eq!(alloc.used(), 0);
        assert_eq!(alloc.remaining(), 1024);
    }
    #[test]
    fn test_bump_alloc_oom() {
        let mut alloc = BumpAlloc::new(0, 16);
        assert!(alloc.alloc(17, 1).is_none());
    }
    #[test]
    fn test_addr_range() {
        let r = AddrRange::new(100, 200);
        assert_eq!(r.size(), 100);
        assert!(r.contains(100));
        assert!(r.contains(199));
        assert!(!r.contains(200));
        let r2 = AddrRange::new(150, 250);
        assert!(r.overlaps(&r2));
        let r3 = AddrRange::new(200, 300);
        assert!(!r.overlaps(&r3));
    }
}
#[cfg(test)]
mod tests_compat_final {
    use super::*;
    #[test]
    fn test_page_map() {
        let mut pm = PageMap::new(4096);
        pm.map_page(0, "null_page");
        pm.map_page(4096, "code");
        pm.map_page(8192, "data");
        assert_eq!(pm.page_count(), 3);
        assert_eq!(pm.label_for(0), Some("null_page"));
        assert_eq!(pm.label_for(4096), Some("code"));
        assert_eq!(pm.label_for(100), Some("null_page"));
        assert_eq!(pm.label_for(99999), None);
    }
    #[test]
    fn test_linear_scan_allocator() {
        let mut alloc = LinearScanAllocator::new(3);
        alloc.add_interval(0, 5);
        alloc.add_interval(2, 8);
        alloc.add_interval(6, 10);
        alloc.allocate();
        assert_eq!(alloc.allocated_count(), 3);
    }
    #[test]
    fn test_instruction_buffer() {
        let mut buf = InstructionBuffer::new();
        assert!(buf.is_empty());
        buf.emit(0xDEAD_BEEF);
        buf.emit(0x1234_5678);
        assert_eq!(buf.len(), 2);
        assert_eq!(buf.get(0), Some(0xDEAD_BEEF));
        buf.patch(0, 0x0000_0001);
        assert_eq!(buf.get(0), Some(0x0000_0001));
        let s = buf.as_slice();
        assert_eq!(s.len(), 2);
    }
}
/// Standard ELF-like section layout for a kernel binary.
#[allow(dead_code)]
pub const STANDARD_SECTIONS: &[CodeSection] = &[
    CodeSection::new(".text", 0x0001_0000, 0x0010_0000, true, false),
    CodeSection::new(".rodata", 0x0011_0000, 0x0004_0000, false, false),
    CodeSection::new(".data", 0x0015_0000, 0x0002_0000, false, true),
    CodeSection::new(".bss", 0x0017_0000, 0x0001_0000, false, true),
];
#[cfg(test)]
mod tests_compat_final2 {
    use super::*;
    #[test]
    fn test_code_section() {
        let sec = CodeSection::new(".text", 0x1000, 0x2000, true, false);
        assert_eq!(sec.vend(), 0x3000);
        assert!(sec.contains(0x1000));
        assert!(sec.contains(0x2FFF));
        assert!(!sec.contains(0x3000));
    }
    #[test]
    fn test_standard_sections() {
        assert!(!STANDARD_SECTIONS.is_empty());
        let text = STANDARD_SECTIONS.iter().find(|s| s.name == ".text");
        assert!(text.is_some());
        assert!(text.expect("text should be valid").executable);
    }
    #[test]
    fn test_symbol_table() {
        let mut tbl = SymbolTable::new();
        tbl.add("main", 0x1000);
        tbl.add("start", 0x0FFF);
        assert_eq!(tbl.lookup("main"), Some(0x1000));
        assert_eq!(tbl.lookup("missing"), None);
        assert_eq!(tbl.nearest(0x1004), Some("main"));
        assert_eq!(tbl.len(), 2);
    }
    #[test]
    fn test_reloc_entry() {
        let r = RelocEntry::new(0x10, "foo", 8);
        let patched = r.apply(0x2000);
        assert_eq!(patched, 0x2008);
    }
    #[test]
    fn test_object_file() {
        let mut obj = ObjectFile::new();
        obj.add_section(".text", vec![0x90u8; 16]);
        obj.add_symbol("foo", 0x0);
        obj.add_reloc(RelocEntry::new(4, "bar", 0));
        assert_eq!(obj.num_sections(), 1);
        assert_eq!(obj.num_relocs(), 1);
        assert_eq!(obj.section_size(".text"), Some(16));
        assert_eq!(obj.section_size(".data"), None);
    }
}
