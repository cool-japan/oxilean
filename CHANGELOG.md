# Changelog

All notable changes to OxiLean will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Copyright (c) COOLJAPAN OU (Team Kitasan)

---

## [0.1.4] — 2026-11-12

**Lean core `Init` now verifies in full, in 34 seconds.** Nested inductives are supported, the kernel works on terms as they are shared (and charges its per-declaration budget the same way), and terms carry their hash: over all 57,277 declarations of Lean 4 core's `Init` export, **57,277 verified · 0 unsupported · 0 rejected in 34 s** (0.1.3: 35,424 · 21,853 · 0 in 8 m 28 s, same machine). A 28,832-declaration Mathlib slice reports 27,523 verified · 1,309 unsupported · 0 rejected. Three kernel fixes ship with it, for bugs that are all present in the published 0.1.3; two wrong accepts (the K-like one and an open-term accept through the Rust API) make **upgrading recommended — see Security.** The MSRV is now an honest 1.89, and a hygiene pass removes `unwrap`/`expect` outside tests across the workspace. Workspace tests: 33,733 passed, 746 skipped (cargo nextest, all features, macOS); the tree before the release-prep fixes (33,727 tests) also passed on Linux.

### Added

#### Nested inductives
- Nested-to-mutual inductive elaboration (C10): a nested inductive such as `Lean.Syntax` (nested through `Array`/`List`) is specialised into an auxiliary mutual family, recursors are derived and restored over the real container, and every exported recursor is re-verified by def-eq, never trusted. On `Init`, verified rises 35,424 → 47,119 with 0 rejected (`b3c17f8`, `b3adf3f`)

#### Resource limits for small stacks (off by default)
- `oxilean_kernel::fuel::set_reduction_nesting_limit` / `reduction_nesting_limit` and `fuel::set_term_depth_limit` / `term_depth_limit` (plus `term_depth_limit_hit`): cap nested reductions and term depth for a caller with a small stack; a declaration over either limit is `unsupported` by name (new `resource limit exceeded: term depth`), never a verdict
- New `term-depth` Cargo feature on `oxilean-kernel`, `oxilean-export` and `oxilean-verify`, so the term-depth limit also covers terms built during checking
- `oxilean_export::TERM_DEPTH_LIMIT`; `oxilean-verify-wasm` sets both limits and exposes `reduction_nesting_limit()` / `term_depth_limit()` (a browser gives a module about a megabyte of stack)
- `expr::REBUILD_COST_MAX`: `rebuild_cost` saturates at 2^29 − 1, the word being shared with three flags (free variable, universe parameter, universe metavariable)

#### Test fixtures (`tests/fixtures/lean4export/`)
- Positive fixtures `CongruenceFirst`, `DeepTerm`, `NestedReduction`, `SharedSubterms`, `QuotPartialApp`, `ReductionChain` and `Syntax` (nested inductive), each exercising one of the fixes and performance changes below
- Negative fixture `KGuardCollision`: a declaration Lean's own kernel rejects; expected split 3 verified · 0 unsupported · 1 rejected (exit 1)

#### Release tooling
- `deny.toml` at the workspace root with the COOLJAPAN banned-crate list; `cargo deny check bans` passes

### Changed

#### Breaking changes
- BREAKING: `Node::rc()` is removed, replaced by `Node::as_ptr()` and `Node::ptr_eq()`. A `Node` no longer holds an `Rc<Expr>`; the cached facts (loose-bound-variable range, rebuild cost, hash) moved into the shared allocation. What `Expr` writes to a `Hasher` differs from 0.1.3; `Debug` output does not
- BREAKING: `StackCalc::{add, sub, mul}` return `Option<i64>` instead of panicking on a stack of fewer than two values, leaving the stack unchanged on failure (34 compiled copies; reachable as `oxilean_kernel::StackCalc`) (`4f99762`)
- BREAKING: the per-declaration budget constants keep their values (2^24 default, 2^26 corpus) but change unit: a unit is now one subterm a term builder reaches (shared or rebuilt), not a cloned tree node
- BREAKING: `check_constant_info` and the inductive-family entry points reject a type, value or recursor rule that is not a closed term (free variable or loose bound variable), as Lean's kernel does

#### Performance (two passes, 2026-11-11)
- `Init` goes from 31 m 51 s to 3 m 58 s after the first pass and to **34 s** after the second; the Mathlib slice from 5 m 50 s to 43 s and then to 11 s (verified 25,690 → 27,523)
- Terms carry their hash: `Name`, `Level` and every shared subterm store a structural hash computed once at construction, so cache lookups no longer walk their keys (55% of a profiled Mathlib replay)
- Pairs of terms are ordered by hash instead of by formatting both as text (27% of the same profile)
- A local's type is dropped when its binder closes, instead of being kept for every binder opened while checking a declaration
- Terms are walked as they are shared: substitution, lifting, abstraction and universe instantiation build a shared subterm once per walk, the reduction cache keeps terms of any size, and type inference remembers what it has typed. The 632 `Init` declarations that ran out of budget, and the 9,512 behind them, now verify
- `f a ≡ f b` from `a ≡ b`: two applications of one definition are compared argument by argument before the definition is unfolded, as in Lean's lazy delta reduction
- Memory: peak live heap on `Init` 2,723 MB → 2,458 MB → 995 MB (Mathlib slice 1,349 MB → 419 MB); `Expr` is 40 bytes where it was 64
- Differential vs lean4lean (`779c51f`) on `Init.Prelude`: **0 disagreements** across 1,976 joined names; net of start-up, oxilean-verify takes 1.1× lean4lean's time (it was 4.3×)
- Kernel in a Tab artifact: 168,775 B gzip / 427,912 B raw (0.1.3: 147,336 B gzip); in node 26 it replays `Init` and the Mathlib slice with the native verdicts, declaration for declaration

#### Structural sharing, Stage F
- `Name`/`Level` become Rc-newtypes with interning (`e1a11c8`): whole-value and prefix sharing collapse per-`Const` name duplication. Memory-only; no verdict moved

#### Hygiene
- `cargo formal check --layer hygiene` findings: kernel 297 → 19, workspace 346 → 68 (`930c328`, `4f99762`); `unwrap`/`expect` outside tests are removed by construction (let-else, slice patterns, invariants moved into types), and 14 `unsafe` sites gain a `SAFETY:` comment while 9 go away
- The hygiene commits add 126 + 212 tests and remove none (the kernel's suite went 3,512 → 3,655 in that commit; it has 3,742 at release)

#### Toolchain and dependencies
- MSRV bumped from 1.70 to 1.89 (`rust-version` in `[workspace.package]`): 1.70 never actually built, since the locked dependencies already required 1.89. The 202 clippy suggestions unlocked at the new MSRV (`is_multiple_of`, `div_ceil`, `repeat_n`, `is_none_or`, …) are applied
- `oxiz-solver` / `oxiz-math` 0.2.3 → 0.3.4, `oxiarc-deflate` 0.3.6 → 0.4.2, `oxicode` 0.2.4 → 0.2.6

#### Release preparation
- Workspace dependency policy: `clap`, `serde`, `serde-wasm-bindgen`, `anyhow`, `tempfile`, `criterion` and `getrandom` (0.3 and 0.4, the wasm32 `wasm_js` shims) are declared once in `[workspace.dependencies]`; minimum requirements are raised to the versions already resolved (`wasm-bindgen` 0.2.130, `js-sys` 0.3.107, `proptest` 1.12.0). The resolved dependency graph is unchanged (checked with `cargo metadata`)
- Unused dependencies removed: `oxilean-kernel` from `oxilean-build` and `oxilean-lint`; `lazy_static` from `oxilean-elab`; `serde_json` and `js-sys` from `oxilean-wasm` (and from its `wasm` feature); dev-dependencies `proptest` (`oxilean-export`) and `tempfile` (`oxilake`). `oxilean-elab`'s dev-dependency on `oxilean-std` is now path-only
- `oxilean-verify-wasm` is marked `publish = false`: it was never published to crates.io and is the wasm-pack artifact behind `web/verify-demo`
- `web/verify-demo/pkg` rebuilt for 0.1.4: 168,785 B gzip, 427,914 B raw, 41 exports (badge 165 KB)
- `oxilean-verify --help` now states that `--limits corpus` also arms the 30 s per-declaration wall-clock deadline
- Gate and build scripts run on macOS: `web/scripts/size-gate.sh`, `scripts/gate-demo-smoke.sh`, `scripts/regen-corpus.sh` and `web/verify-demo/build.sh` no longer use GNU `stat -c`; `scripts/bench-verify.sh` falls back to `sysctl -n hw.ncpu`
- User docs (`docs/USER_GUIDE.md`, `docs/FAQ.md`, `docs/TUTORIAL.md`) brought up to 0.1.4: version strings, sizes, Mathlib figures aligned with the README, and the environment-variable section corrected to what the binary reads

### Fixed
- `oxilean --help`, `-h`, `--version` and `-V` were treated as file names (`Failed to read file --help`, exit 1); they now print the help or the version and exit 0 (`-v` stays `--verbose`). `oxilean version` no longer prints the stale `Kernel SLOC: ~2,600` line; it prints `Kernel: zero external dependencies, #![forbid(unsafe_code)]`
- Wrong accept: K-like reduction checked its side condition in a nested checker that reused free-variable ids already handed out, so a bound variable could be identified with an outer local; a theorem Lean's kernel rejects was reported verified. It is now rejected and kept as the negative fixture `KGuardCollision`
- Wrong reject on point-free quotient maps: a head that reduced to a partial application lost its outer arguments, so `Quot.lift` (or a recursor) on such a major premise looked stuck (Mathlib's `Quot.map` / `Quotient.map'`, e.g. `AddCon.addGroup`, `Con.group`, `FreeGroup.instGroup`)
- Wrong reject on long reductions: a head-reduction chain was cut off at 10,000 steps and the unfinished term compared as a normal form; a declaration failing after the cap fired is now checked again with the cap lifted, under the per-declaration budget. Any other truncated reduction is reported `unsupported` (`resource limit exceeded: reduction depth or step limit`), never `rejected`. On the Mathlib slice, rejected goes 27 → 0
- `oxilean-doc` panicked at startup in every debug build (clap named a subcommand in `required_unless_present`) and the release build rejected `multi`; it now uses `subcommand_negates_reqs` with a required input (`930c328`)
- `SimpleLruCache::put` (five kernel copies) is unwind-safe: a panicking `Clone`, `Hash`, `Eq` or `Drop` can no longer leave the order list shorter than the keys (`4f99762`)

### Security
- **Wrong accept in the published 0.1.3**: K-like reduction could report as verified a theorem that Lean's kernel and `leanchecker` both reject (see Fixed). Upgrading is recommended
- **Open-term accept in the published 0.1.3**: through the Rust API, `fun (x : F) => FVar 1` was accepted as a proof of `F → F` and stored with the free variable in it; declarations must now be closed terms. `oxilean-verify` was never exposed (no export can spell a free variable), but a caller building declarations through the API was. Upgrading is recommended
- Unsound paths reachable from safe code are closed (`930c328`): `oxilean-elab` `validate_locals` (a transmuted `&str` of a per-iteration `String` used as a `HashMap` key; a use-after-free under Miri), `oxilean-runtime` `ThunkVec::get` (a `mem::zeroed::<T>()` placeholder), and `ArenaAllocator::alloc_bytes` (`size + align - 1` overflow)
- The `# Safety` contract of `PoolAllocator::deallocate` is tightened to exclude `reset` and `clear` (allocate → clear → deallocate was a use-after-free under Miri)

## [0.1.3] — 2026-07-16

Independent Lean 4 proof checker ("Kernel in a Tab"): oxilean-export NDJSON v3.1.0 reader, oxilean-verify three-bucket streaming CLI, oxilean-verify-wasm (144 KB gzip) + web/verify-demo static demo, BigNat arbitrary-precision literals, quotient and struct-eta soundness overhaul, re-derived recursors, complete universe-level definitional equality, CI/TCB gates, and fuzz infrastructure.

**First full-corpus result** — over **all 57,277 declarations** of Lean 4 core's `Init` export (v4.32.0-rc1, 345 MB NDJSON): **35,223 verified · 22,054 unsupported (named buckets) · 0 rejected** (exit 0). Every declaration got exactly one verdict; nothing was skipped silently. The unsupported bucket is dominated by a single root cause — nested inductives (`Lean.Syntax`), whose dependency cascade accounts for 98.9% of it. Overall throughput 11.8 decls/s (1 h 20 m 44 s wall, 96% CPU, peak RSS 9.98 GiB inside a 12 GiB memory cage on a 14 GiB machine) — far outside the 5×-of-lean4lean budget; root cause (unshared kernel `Expr`) and its fix (structural sharing) shipped in this same release — see **Changed → Structural sharing** below, which brings the same corpus to 11 m 28 s and 35,424 verified. Full report: `docs/reports/2026-07-15-lean-core-init.md`.

**Differential harness vs lean4lean** — on Lean core `Init.Prelude`, the harness first surfaced 20 real disagreements (all false rejections of multi-constructor `*.noConfusion`, one systematic kernel bug); after the C15 fix the re-run joins at **0 disagreements** across 1,987 declarations. See `verify/differential/RESULTS-2026-07-12.md`.

### Added

#### oxilean-export — lean4export NDJSON v3.1.0 reader
- New `oxilean-export` crate: zero-external-dependency, `#![forbid(unsafe_code)]` NDJSON v3 reader and replay engine for lean4export files (`dfe640c`, `b8d9bfd`, `ac81989`)
- Budgeted index-node reader with memory limits; supports inductive families, quotients, recursor verification, `Limits` corpus preset
- Fuzz target for the reader (`G9` partial — wired in Wave 3b)

#### oxilean-verify — streaming three-bucket proof checker CLI
- New `oxilean-verify` crate and binary: independent streaming checker emitting exactly `verified` / `unsupported (named feature)` / `rejected` per declaration; exits non-zero on any `rejected` verdict (the alarm) (`dbad39b`, `4ff55fa`)
- Hand-rolled JSON report output — no serde in the TCB; `--limits` flag for corpus-scale runs
- `docs/VERIFY.md`: three-bucket schema, exit codes, JSON report format, lean4export pins, unsupported-feature list

#### oxilean-verify-wasm — Kernel in a Tab (144 KB gzip)
- New `oxilean-verify-wasm` cdylib: kernel + export + verify + wasm-bindgen + js-sys ONLY; `wasm-bindgen` non-optional (DCE-regression class structurally impossible) (`bab8552`)
- Streaming JS API: `VerifySession::new(LimitsPreset)` / `push_chunk` / `finish(on_decl, path)` → `VerifySummary`; client-side `sha256` getter ("0 bytes uploaded")
- Artifact: raw 363 KB / **gzip 144 KB** (147,132 B; was 138 KB before the Wave-4 kernel completeness fixes) — well under 400 KB budget; 39 exports; `wasm-tools validate` VALID

#### web/verify-demo — static demo ("Kernel in a Tab")
- Static page: drop-zone for `.ndjson` export, live streaming verdicts (✓/⊘/✗), three-bucket summary, badge: "kernel: 144 KB wasm · 0 external dependencies · 0 unsafe · 0 bytes uploaded" (`bab8552`)
- Serves under `python3 -m http.server` (no COOP/COEP/SharedArrayBuffer needed); no CDN/framework/worker

#### oxilean-kernel — soundness fixes and BigNat
- Arbitrary-precision `BigNat` (`schoolbook + Karatsuba mul, Knuth-D div`): replaces `Literal::Nat(u64)` fast-path; `MAX_RESULT_BITS` guard leaves stuck terms instead of wrong ones (`654e110`, `Wave 1`)
- `Literal::Int(i64)` variant and wrapping evaluators removed from the kernel TCB (`S10`)
- Complete universe-level definitional equality: `level/order.rs` with smart constructors + full param-case-split leq (trepplein/nanoda algorithm); `imax(u,u) → u` and numeral subsumption (`a3febca`, C3)
- Definitional eta for structures (`try_eta_struct`, both orientations) + `is_def_eq_unit_like` wired into `DefEqChecker` (`d9d3674`, C1/C2)
- Quotient-type overhaul: `check_quotient_val` validates against kernel-built canonical types; `add_quot` installs four canonical QuotVals; `Quot.ind` reduction rule (`Quot.ind h (Quot.mk r a) ≡ h a`) with re-application of over-args (`2987a28`, S2/S3/S9)
- Re-derived recursors (`inductive/derive.rs`): builds IHs, correct binders, level params propagated; validates RecursorVals from exports against kernel derivation; builtin inductives go through checked `add_inductive_family` (`8049397`, S3–S6)
- `infer_const` enforces level-arity (empty list for polymorphic constants rejected, S7); `check_univ_param_hygiene_ci` gates duplicate/undeclared params/MVars at top of `check_constant_info` for all 8 variants (S8, `c328c78`)
- NatLit/StrLit ↔ constructor bridge in `DefEqChecker` (`c328c78`, C6)
- Three completeness fixes from Init.ndjson corpus replay (`0017bc3`)
- `oxilean_kernel::wall_clock::Instant`: transparent newtype over `std::time::Instant` off wasm; monotonic `AtomicU64` counter on wasm (fixes `Instant::now()` wasm panic that collapsed kernel to 22 KB stub)

#### CI and TCB gates
- CI re-enabled: `check` / `nextest` / `clippy -D warnings` / `fmt --check` / TCB-gates jobs (`d884cf4`, G1)
- `scripts/gate-zero-deps.sh`: zero-external-dep invariant for kernel, export, verify (`77a9ba0`, G2)
- `verify/allowed-deps.txt` + `scripts/gate-allow-list.sh`: full transitive runtime closure validation (`98f0863`, G3)
- `#![forbid(unsafe_code)]` gate over kernel, export, verify, verify-wasm (G4)
- WASM export-count gate: `web/scripts/export-gate.sh` (G5); size-budget gate: `web/scripts/size-gate.sh` (G6)
- Native determinism gate: `scripts/gate-determinism.sh` (G7 partial — wasm half deferred)
- Demo smoke test: `scripts/gate-demo-smoke.sh` (G8 partial)
- Root-level `[[test]]` registration for `tests/cli_test.rs` (68 tests) and `tests/perf_test.rs` (26 tests) (G10)

#### M4 (Mathlib-scale) hardening
- **File-streaming CLI.** `oxilean-verify` streams the input file (sha256 in a bounded-buffer pass + `BufReader<File>` for the engine) instead of reading it all into RAM, so a ~6 GB whole-corpus export verifies with the CLI holding O(1) memory — only the shared environment grows (`b92faf7`).
- **Per-declaration wall-clock deadline.** A backstop (`oxilean_kernel::deadline`) for reduction/def-eq loops that make progress without constructing nodes and so never exhaust the deterministic fuel — including an exponential `imax` case-split in the universe `≤` procedure (`is_leq_core`). An over-time declaration is degraded exactly like a fuel-exhausted one (a named resource-limit `unsupported`), never a hang, never a wrong verdict; the wall-clock basis is machine-dependent and disclosed (`db509da`, `b904017`).

### Fixed
- **u64 literal overflow soundness hole** (S1): `2^32 * 2^32` would silently wrap to 0 in release builds; replaced with BigNat (`654e110`)
- **Quot.ind off-by-one** (S2): `try_reduce_quot` discarded `args[mk_pos+1..]` (over-application) — now re-applies; same fix in recursor iota (`2987a28`)
- **de Bruijn capture in `instantiate`**: off-by-one with params/indices in recursor derivation (`8049397`, S5)
- **`is_prop` flag trusted from export** (S4): kernel now decides large-elimination from normalized levels; tampered `is_prop` flags rejected
- **Wrong builtin `Quot` type** (S9): `{α : Sort u} → Prop → Sort u` removed; replaced by four canonical QuotVals
- **`infer_proj_field_type` fabricated a type on telescope mismatch** (S11) — now returns typed `Err`
- **wasm DCE regression + `Instant::now` wasm panic** (V3): `std::time::Instant::now()` on `wasm32-unknown-unknown` collapsed the entire kernel path to a 22 KB stub; fixed with `wall_clock::Instant`
- **`imax(u,u)` not normalized** (C3): `def Endo.{u} : Sort u → Sort u` was spuriously REJECTED; fixed by smart constructors and complete param-case-split leq

### Changed
- `Literal::Int(i64)` removed from `Literal` enum — BREAKING: serialization tag 2 is now a hard read error; kernel is a Lean4 verifier and Lean4 has no `Int` literal in its kernel

#### Structural sharing of the kernel `Expr` (wave5)
- **`Box<Expr>` → `Rc<Expr>` → cached-header `Node` edges.** Every recursive `Expr` field is now a `Node { range: u32, cost: u32, rc: Rc<Expr> }` that caches the subterm's `looseBVarRange` and no-skip rebuild cost. Clone is an O(1) refcount bump; `Node` transparently `Deref`s to `Expr`, so match arms are untouched (`bc0bb50`, `4996a12`).
- **Materialise-once export reader.** The reader memoises each export DAG id to a single shared `Node`, so Init's 908,550,041 tree nodes are 552,915 distinct exprs (~1,643× sharing). Sharing-bomb inputs materialise as small DAGs; the per-declaration/file budgets are counted in distinct nodes (`6b75000`).
- **O(1) untouched-subtree substitution skip, fuel-exact.** `instantiate`/`lift` return a shared clone when `range <= depth`, charging the *exact* fuel a rebuild would — so per-declaration fuel, and therefore every verdict, is byte-identical to the non-sharing baseline. Pinned by three differential property tests (skip result / skip fuel / cached metadata vs recomputed) (`bb59431`).
- **Full-`Init` result:** wall **1 h 20 m 44 s → 11 m 28 s (≈7×)**, peak RSS **9.98 GiB + ~6.7 GiB swap-thrash → ≈6 GiB, no swap**; verdicts a strict superset — **35,424 verified · 21,853 unsupported · 0 rejected** (+201 verified vs 35,223, no regressions).

### Security
- TCB hardening: kernel, export, and verify each have zero external runtime dependencies, `#![forbid(unsafe_code)]`, and are independently validated by CI gates on every push
- `unsafe` block removed from `oxilean-parse` (`retain`-based eviction replaces `ptr::read`); `#![forbid(unsafe_code)]` propagated to parse, build, codegen, lint, umbrella

---

## [0.1.2] — 2026-05-03

Codegen compilation fixes and real WASM pipeline integration: 59 test compilation errors resolved, oxilean-wasm now uses the full kernel/parse/elab pipeline.

### Added
- `EvmBackend::compute_selector` and `SolidityFunction::selector` now use real keccak256 (via `tiny-keccak`) — EVM/Solidity ABI 4-byte selectors are spec-correct and interoperate with real chains
- `GroebnerBasis::reduce` — full multivariate polynomial division (Cox–Little–O'Shea §2.3); `contains` is now meaningful for ideal membership testing in `polyrith` tactic
- `SmtContext::check_sat` and `run_smt_tactic` — real SMT solving via `oxiz-solver 0.2.1` (OxiZ); returns `Sat`/`Unsat`/`Unknown` from live solver
- `WasmModule::call_function` — full WebAssembly bytecode interpreter: all 157 `WasmInstruction` variants wired, structured control flow (Block/Loop/If/Else/End), branch instructions (Br/BrIf/BrTable/Return), frame-stack call dispatch (Call/CallIndirect with locals and return-PC bookkeeping)
- `InliningPass::run_all` / `run_with_context` in `opt_copy_prop` — real fixed-point function inliner with variable-ID freshening and configurable cost threshold
- 1,041 rustdoc `broken_intra_doc_links` / HTML tag errors fixed across `oxilean-std` (math bracket escaping), `oxilean-elab`, `oxilean-parse`, `oxilean-wasm`

### Changed
- `rand` upgraded 0.9.2 → 0.9.4 (resolves RUSTSEC-2026-0097)
- `wasm-bindgen` upgraded 0.2.118 → 0.2.120; `js-sys` 0.3.95 → 0.3.97; `fastrand` 2.4.0 → 2.4.1 (was yanked)
- 5 files proactively split via `splitrs` (all were 1,945–1,985 lines, now ≤1,004 lines)
- SLOC grown to ~1,347,650 across 5,978 files
- Test suite grown to **33,091 passing** (was ~29,831 at v0.1.1)

### Fixed

- 59 test compilation errors in `oxilean-codegen` caused by private method/field visibility after splitrs refactoring, affecting modules: `core_types`, `matlab_backend`, `wasm_backend`, `elixir_backend`, `evm_backend`, `kotlin_backend`, `lua_backend`, `opt_cse`, `opt_regalloc`, `x86_64_backend`

### Changed (pipeline)

- `oxilean-wasm` now integrates with the real `oxilean-kernel` / `oxilean-parse` / `oxilean-elab` pipeline instead of mock/stub implementations

---

## [0.1.1] — 2026-03-09

Mathlib4 compatibility leap: from 4,530 to 181,890 declarations tested, achieving 99.7% parse compatibility across the entire Mathlib4 codebase.

### Added

#### Mathlib4 Compatibility Test Suite
- Expanded from 566 files / 47 categories to **7,759 files / 280+ categories**
- Expanded from 4,530 declarations to **181,890 declarations** (99.7% parse rate)
- Multi-line declaration extraction: joins indented continuation lines, strips line comments
- Summary test scanning 31 top-level recursive directories + Archive + Counterexamples
- Diagnostic test infrastructure for categorizing remaining parse failures
- 769 tests total (19 basic + 750 category/summary tests), zero warnings

#### Normalization Pipeline (normalize.rs, ~6,000 lines)
- 280+ Unicode operator replacements (category theory, analysis, algebra, set theory, etc.)
- `normalize_head_binders`: moves theorem head binders into `forall` type
- `normalize_exists_quantifier`: desugars multi-binder `exists` into nested `Exists(fun ...)`
- `normalize_psigma_binder`: wraps `PSigma x : T, body` into `PSigma (fun (x : T) -> body)`
- `normalize_bounded_quantifiers`: `ISup k < n, body` into `ISup (fun k -> body)`
- `normalize_big_prod_sum`: `BigProd` / `BigSum` with set membership
- `normalize_set_literals`, `normalize_set_builder_notation`, `normalize_singleton_sets`
- `normalize_subtype_sets`: `{ x : T // P }` into `Subtype T (fun x -> P)`
- `normalize_exists_unique`: `exists! x, P` into `ExistsUnique (fun x -> P)`
- `normalize_if_then_else_in_type`, `normalize_match_in_type`
- `normalize_dot_anonymous_fn`: `(. < .)` into `(fun x y -> x < y)`
- `normalize_star_type_suffix`: `beta*` into `beta_Star`
- `normalize_finsum_finprod`: handles `finprod`/`finsum` with `U+1DA0` marker
- `strip_universe_annotations`, `strip_attributes`, `strip_where_block`
- `replace_proof_with_sorry`: `by <tactics>` into `sorry`
- `parenthesize_dot_exprs`: `ident.field` into `(ident.field)`
- `parenthesize_bare_forall_binders`: `forall h:` into `forall (h:)`
- `strip_quantifier_binder_groups`, `strip_prop_condition_binders`
- `fix_trailing_operator_before_sorry`, `fix_forall_no_body_before_assign`
- `strip_orphan_close_brackets`, `strip_orphan_close_parens`
- `balance_parens_before_sorry`, `fix_truncated_decl`
- `^*` (fixed-points/pullback/dual) normalization

### Changed
- Test file structure reorganized: `normalize.rs`, `normalize_2.rs`, `normalize_3.rs`, `test_infra.rs`, `tests_basic.rs`, `tests_categories.rs`, `tests_summary.rs`, `types.rs`

---

## [0.1.0] — 2026-03-05

First release of OxiLean: a Lean4-inspired proof assistant kernel and toolchain
implemented in pure Rust. 1,221,710 SLOC across 11 crates and 5,380 files.

### Added

#### `oxilean-kernel` (113,179 SLOC)
- `Arena<T>` typed arena allocator with `Idx<T>` indexing
- `Name` hierarchical names (`Anonymous`, `Str`, `Num`) with `name!` macro
- `Level` universe levels (`Zero`, `Succ`, `Max`, `IMax`, `Param`)
- `Expr` core expression type with all variants (`BVar`, `FVar`, `Sort`, `Const`, `App`, `Lam`, `Pi`, `Let`, `Lit`, `Proj`)
- `BinderInfo` binder annotations (Default, Implicit, StrictImplicit, InstImplicit)
- `Literal` native literals (Nat, String)
- `FVarId` unique free variable identifiers
- Substitution engine: `instantiate`, `abstract`, `lift_bvars`
- WHNF reduction with full strategy support: beta, delta, zeta, iota, projection, and quotient reduction
- Type inference for all `Expr` variants
- Definitional equality checker with proof irrelevance
- Declaration checking for Axiom, Definition, Theorem, and Opaque declarations
- Inductive type declarations with strict positivity checking
- Recursor generation and iota-reduction rules
- Quotient types: `Quot.mk`, `Quot.lift`, `Quot.sound`
- Bootstrap types: Bool, Unit, Empty, Nat, String
- Zero external dependencies enforced
- `#![forbid(unsafe_code)]` in kernel

#### `oxilean-meta` (150,298 SLOC)
- Metavar-aware weak head normal form (WHNF) computation
- Higher-order unification engine
- Type class synthesis and instance resolution
- Tactic infrastructure and metaprogramming framework
- AST manipulation and transformation utilities

#### `oxilean-parse` (61,225 SLOC)
- UTF-8 lexer with full Unicode identifier support
- 60+ token types with precise source spans
- Pratt parser for operator precedence handling
- 27 `SurfaceExpr` variants covering the full surface syntax
- 16 `Command` variants for top-level declarations and directives
- Tactic parser supporting 40+ tactic forms
- Macro system with hygienic expansion
- Notation system for user-defined syntax extensions
- Module system with dependency graph resolution
- Pattern compiler for match expressions
- Pretty printer with configurable formatting
- Source map for diagnostic reporting
- Error recovery for resilient parsing

#### `oxilean-elab` (91,008 SLOC)
- `MetaContext` with metavar creation, assignment, and zonking
- Constraint-based unification solver
- Full expression elaboration: name resolution, implicit argument insertion, universe polymorphism
- Pattern match compilation with exhaustiveness and redundancy checking
- Declaration elaboration for def, theorem, inductive, structure, class, and instance
- Attribute system for declaration metadata
- Coercion system for automatic type conversions
- Derive system for automatic instance generation
- Parallel elaboration for independent declarations
- Termination checking for recursive definitions
- Tactic framework with core tactics: intro, apply, exact, simp, omega, ring, cases, induction, constructor, rewrite, have, let, suffices, show, assumption, contradiction, exfalso, trivial, decide, norm_num, linarith, field_simp, ring_nf, push_neg, by_contra, by_cases, ext, funext, congr, calc, rfl, symm, trans, and more

#### `oxilean-cli` (64,163 SLOC)
- Interactive REPL with line editing and history
- Multi-line input support with continuation detection
- Colorized output for types, terms, errors, and diagnostics
- File checking mode for `.oxilean` and `.lean` files
- `#check` command for type inference display
- `#eval` command for expression evaluation
- `#print` command for definition inspection

#### `oxilean-std` (413,202 SLOC)
- Core data structures: Nat, Bool, List, Option, Result
- Mathematical definitions and proof library
- Algebraic hierarchy: Semigroup, Monoid, Group, Ring, Field
- Type classes: Eq, Ord, Functor, Monad, Applicative, Decidable
- Decision procedures and certified algorithms

#### `oxilean-codegen` (240,840 SLOC)
- LCNF (lambda-lifted closure-free normal form) intermediate representation
- LCNF-based compilation pipeline with optimization passes
- Rust code generation backend

#### `oxilean-runtime` (31,115 SLOC)
- Runtime memory management and object representation
- Closure allocation and application
- I/O monad implementation
- Task scheduling and concurrency primitives

#### `oxilean-build` (25,194 SLOC)
- Multi-file project compilation
- Dependency resolution and topological ordering
- Incremental build support

#### `oxilean-lint` (17,061 SLOC)
- Static analysis passes for common errors
- Style enforcement and naming conventions
- Best practice recommendations and suggestions

#### `oxilean-wasm` (381 SLOC)
- WebAssembly bindings via `wasm-bindgen`
- `check` function for type checking expressions
- `repl` function for interactive evaluation
- `completions` function for editor integration
- `hover` function for type-on-hover information
- `format` function for source code formatting

#### Project Infrastructure
- Cargo workspace with 11 crates
- License: Apache-2.0
- Pure Rust with zero C/Fortran dependencies
- `cargo clippy` clean with zero warnings

---

[Unreleased]: https://github.com/cool-japan/oxilean/compare/v0.1.4...HEAD
[0.1.4]: https://github.com/cool-japan/oxilean/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/cool-japan/oxilean/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/cool-japan/oxilean/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/cool-japan/oxilean/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/cool-japan/oxilean/releases/tag/v0.1.0
