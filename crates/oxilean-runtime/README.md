# oxilean-runtime

[![Crates.io](https://img.shields.io/crates/v/oxilean-runtime.svg)](https://crates.io/crates/oxilean-runtime)
[![Docs.rs](https://docs.rs/oxilean-runtime/badge.svg)](https://docs.rs/oxilean-runtime)

> **Runtime System for the OxiLean Theorem Prover**

`oxilean-runtime` implements the execution substrate for compiled OxiLean programs and proofs. It provides everything needed to evaluate terms that have already been type-checked by the kernel: memory management, closure representation, lazy evaluation, I/O, and parallel task scheduling.

Unlike the kernel -- which is the Trusted Computing Base (TCB) and must be auditable -- the runtime is **untrusted**: bugs here can cause incorrect evaluation results or crashes but never logical unsoundness (all terms are checked before reaching the runtime).

31,676 SLOC -- fully implemented runtime system (253 source files, 969 tests passing).

Part of the [OxiLean](https://github.com/cool-japan/oxilean) project -- a Lean-compatible theorem prover implemented in pure Rust.

## Overview

### Module Reference

| Module | Description |
|--------|-------------|
| `arena` | Typed and generational arena allocators (`BumpArena`, `GenerationalArena`, `RegionManager`) |
| `object` | Tagged-pointer runtime value representation (`RtObject`, `TypeTag`, `TypeRegistry`) |
| `rc` | Reference-counted smart pointers with elision analysis (`Rc`, `RtArc`, `CowBox`) |
| `closure` | Flat closure representation and partial application (`Closure`, `Pap`, `FunctionTable`) |
| `lazy_eval` | Call-by-need thunks with memoization (`Thunk`, `SharedThunk`, `MemoFn`) |
| `tco` | Trampoline-based tail-call optimization (`trampoline`, `TailCallDetector`) |
| `scheduler` | Work-stealing parallel task scheduler (`Scheduler`, `Worker`, `WorkStealingDeque`) |
| `io_runtime` | File, console, and string I/O (`IoRuntime`, `IoExecutor`, `StringFormatter`) |
| `bytecode_interp` | Bytecode interpreter for evaluation |
| `gc_strategies` | Pluggable garbage collection strategies |
| `memory_pool` | Pool-based allocator for fixed-size objects |
| `profiler` | Runtime profiling and statistics |
| `string_pool` | Interned string storage |
| `wasm_runtime` | WebAssembly-specific runtime adaptations |

### Global Configuration

```rust
use oxilean_runtime::RuntimeConfig;

let config = RuntimeConfig {
    max_heap_bytes: 512 * 1024 * 1024, // 512 MiB limit
    initial_arena_bytes: 64 * 1024,    // 64 KiB initial arena
    worker_threads: 4,                  // parallel evaluation threads
    rc_elision: true,                   // enable RC elision optimisation
    lazy_eval: true,                    // enable call-by-need thunks
    stack_limit: 8192,                  // recursion depth limit
    gc_stats: false,                    // GC statistics collection
};
```

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
oxilean-runtime = "0.1.1"
```

### Arena Allocation

```rust
use oxilean_runtime::{BumpArena, TypedArena};

let mut arena = BumpArena::new(64 * 1024);
let idx = arena.alloc(42u64);
```

### Closures and Partial Application

```rust
use oxilean_runtime::{Closure, Pap, FunctionTable};

// Build a partial application of a 2-argument function
let pap = Pap::new(fn_ptr, vec![arg0]);
let result = pap.apply(arg1);
```

### Lazy Evaluation

```rust
use oxilean_runtime::{Thunk, SharedThunk};

// Wrap a computation in a lazily evaluated thunk
let thunk = SharedThunk::new(|| expensive_computation());
let value = thunk.force(); // computed only once, memoized thereafter
```

## Dependencies

- `oxilean-kernel` -- expression types and environment definitions

## Testing

```bash
cargo test -p oxilean-runtime
cargo test -p oxilean-runtime -- --nocapture
```

## License

Copyright COOLJAPAN OU (Team Kitasan). Apache-2.0 -- See [LICENSE](../../LICENSE) for details.
