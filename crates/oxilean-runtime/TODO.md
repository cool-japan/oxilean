# oxilean-runtime — TODO

> Task list for the runtime system crate.
> Last updated: 2026-11-12

## ✅ Completed

**Status**: COMPLETE — ~35,716 SLOC implemented across 278 source files

### Runtime System Features
- [x] Runtime primitives
- [x] Memory management
- [x] Evaluation support
- [x] Reference counting
- [x] Garbage collection integration
- [x] Object representation
- [x] Primitive operations

### Execution Support
- [x] Value representation
- [x] Closure representation
- [x] Thunk evaluation
- [x] Lazy evaluation support
- [x] Stack management
- [x] Exception handling

### Built-in Functions
- [x] I/O primitives
- [x] String operations
- [x] Arithmetic operations
- [x] Comparison operations
- [x] Container operations

---

## 🐛 Known Issues

Observations recorded by the 0.1.4 hygiene pass (commit 930c328), left unchanged in the code:

- [ ] `ArrayOps::pop` panics "already borrowed" on any non-empty array (an empty array or a scalar returns `None`).
- [ ] `PoolAllocator::reset` zero-fills the pool without dropping the values stored in it.
- [ ] A non-power-of-two `align` misaligns.
- [ ] `SharedThunk::force` deadlocks on self-reference.
- [ ] The runtime does not build for wasm32 (a literal out of range for `usize`).

---

## ✅ Completed: Runtime Profiling

- [x] Performance profiling support — `profiler.rs` (Profiler, ProfilingEvent, ProfileReport, 6 tests)
- [x] Memory profiling — `profiler.rs` (MemoryProfile, heap tracking)
- [x] Alternative GC strategies — `gc_strategies.rs` (MarkSweep, Semispace, Generational GC, 8 tests)
- [x] WASM runtime integration — `wasm_runtime.rs` (WasmModule, WasmMemory, WasmRuntime, 8 tests)

## v0.1.4 (released 2026-11-12)

- [x] Hygiene pass (commit 930c328): `.unwrap()` / `.expect()` outside tests 15 -> 5; the pool and arena allocators' `unsafe` sites gained `SAFETY:` comments; no public signature changed except one `pub(super)` fn added inside the arena module.
- [x] `ThunkVec::get` no longer stores `mem::zeroed::<T>()` as a placeholder: it keeps a `Forcing` state and answers `None` on re-entry or after a panicked thunk (`force_all` skips such an element).
- [x] `ArenaAllocator::alloc_bytes` no longer overflows `size + align - 1`; it returns the `None` its doc promised.
- [x] `PoolAllocator::deallocate`'s `# Safety` contract now excludes `reset` and `clear` (allocate, clear, deallocate was a use-after-free under Miri).
- [x] `RtArc`'s two `unsafe impl Send / Sync` removed; the auto traits give the same bounds, and a test asserts it.
- [x] `AtomicRefCounter::dec` spells out its compare-and-swap loop instead of calling `fetch_update`, so it builds without deprecation warnings on every toolchain from the MSRV (1.89) up.
- [x] MSRV 1.89 clippy suggestions applied (`div_ceil`, `is_multiple_of`, `slice::fill`).
- [x] New tests pin each of the above (arena bounds, placement, thunk states, `make_closure`, `load_u32`, concurrent `AtomicRefCounter`).
