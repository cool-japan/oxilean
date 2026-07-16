# Audit: CI, Gates, Fuzzing, Differential Harness
## Repository: oxilean  |  Branch: 0.1.3  |  Date: 2026-07-12

---

## 1. Current CI State

### What runs on push/PR today

**Only one active workflow:** `.github/workflows/npm-publish.yml`

Triggers:
- `push` on tags matching `v*`
- `workflow_dispatch` (manual, with `publish_target` choice: `dry-run` | `npm`)

This workflow:
- Builds WASM with `wasm-pack` for bundler / web / nodejs targets
- Reports `ls -lh pkg-bundler/*.wasm` (human-readable size display ONLY — no assertion)
- Publishes to npm under `@cooljapan/oxilean`

**No CI runs on `push` to any branch or `pull_request`.** There is no gate of any kind protecting the main branch.

### The disabled workflows

`.github/workflows.disabled/ci.yml` (added in commit `f7f4438` "Availability of 0.1.0"):
- Triggers: push to `main`/`dev`, PR to `main`
- Jobs: `test` (build + `cargo test --workspace` + clippy + fmt) and `build-release` (release binary upload) and `security-audit` (`rustsec/audit-check@v1`)
- Uses `dtolnay/rust-toolchain@stable` — no pinned toolchain

`.github/workflows.disabled/bench.yml`:
- Trigger: weekly cron + `workflow_dispatch`
- Runs `cargo test --workspace -- --test bench_ --nocapture`
- Uploads results to `/tmp/bench_results/`

**Why disabled:** Both were placed under `workflows.disabled/` from the very first commit (`f7f4438`). There is no git commit that moved them from `workflows/` — they were never active. The `npm-publish.yml` was added in `183d9c8` ("Availability of 0.1.2") as the only active workflow. The reason for disabling the CI is not documented; the most likely cause is that the workspace did not compile cleanly at the time of the initial push (stubs everywhere, unimplemented code).

---

## 2. Brief-Required Gates — Status

### (a) Zero-dep invariant gate for oxilean-kernel

**Status: MISSING**

The `[dependencies]` section of `crates/oxilean-kernel/Cargo.toml` is empty (comment reads "ZERO external dependencies - this is the TCB"). This is correct by inspection, but **there is no CI gate that enforces it**.

Concrete check needed:
```bash
# Fails if any non-dev dep is introduced
cargo tree -p oxilean-kernel --edges normal 2>&1 | grep -v "^oxilean-kernel" && { echo "FAIL: kernel has external deps"; exit 1; } || echo "OK: kernel is dep-free"
```

This should be a CI job step that exits non-zero if `cargo tree --edges normal` for oxilean-kernel lists any crate other than the kernel itself.

### (b) Dependency allow-list for verify product (verify/allowed-deps.txt)

**Status: MISSING**

There is no `allowed-deps.txt` or equivalent, no `deny.toml` (cargo-deny), no `cargo-deny` invocation anywhere. For the future `oxilean-verify` crate the brief requires only `{oxilean-kernel, oxilean-export, wasm-bindgen}` as non-dev dependencies.

Concrete gate needed (using cargo-deny or a shell script):
```toml
# deny.toml
[bans]
multiple-versions = "warn"
[[bans.deny]]
name = "libc"   # example: no libc in verify product

[licenses]
allow = ["Apache-2.0", "MIT"]
```

Or simpler shell check:
```bash
ALLOWED="oxilean-kernel oxilean-export wasm-bindgen"
cargo tree -p oxilean-verify --edges normal --prefix none \
  | awk '{print $1}' | sort -u > /tmp/actual_deps.txt
# diff against allow-list...
```

### (c) `#![forbid(unsafe_code)]` presence check

**Status: PARTIAL — kernel only, not enforced in CI**

- `crates/oxilean-kernel/src/lib.rs:263` — has `#![forbid(unsafe_code)]`
- `crates/oxilean-meta/src/lib.rs:316` — has `#![forbid(unsafe_code)]`
- `crates/oxilean-parse/src/lib.rs` — does NOT have it; furthermore `crates/oxilean-parse/src/expr_cache/types/impls.rs:274` contains an actual `unsafe { std::ptr::read(raw) }` block
- No other crates in the workspace have `#![forbid(unsafe_code)]`

For the brief's TCB (kernel + new export reader), both crates must carry `#![forbid(unsafe_code)]`. A CI gate should grep for its presence:
```bash
# Fail if kernel or export reader is missing the attribute
for crate in crates/oxilean-kernel crates/oxilean-export; do
  grep -r "#!\[forbid(unsafe_code)\]" "$crate/src/lib.rs" || { echo "FAIL: $crate missing forbid(unsafe_code)"; exit 1; }
done
```

Additionally, no CI step currently runs `cargo +nightly miri` or any sanitizer.

### (d) cargo-fuzz targets + CI job

**Status: COMPLETELY MISSING**

- No `fuzz/` directory exists anywhere in the repository
- No `cargo-fuzz` or `libfuzzer-sys` in any `Cargo.toml`
- No `proptest` fuzz target for the export reader (though `proptest` is in `[workspace.dependencies]`, it is used for unit-level property tests only, not as a persistent fuzz corpus)
- No `oxilean-export` crate exists yet (it is one of the crates to be created)

The brief requires cargo-fuzz on the export reader in CI. A skeleton:

```
crates/oxilean-export/fuzz/
  Cargo.toml            (with [[bin]] fuzz_export_parser + libfuzzer-sys dep)
  fuzz_targets/
    fuzz_export_parser.rs
```

`fuzz_export_parser.rs` skeleton:
```rust
#![no_main]
use libfuzzer_sys::fuzz_target;
use oxilean_export::parse_export;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = parse_export(s); // must not panic
    }
});
```

CI job (separate workflow, runs on schedule or PR):
```yaml
- name: Run fuzz (short, CI-safe)
  run: |
    cargo +nightly fuzz run fuzz_export_parser -- -max_total_time=60
  working-directory: crates/oxilean-export
```

### (e) WASM export-count gate

**Status: MISSING**

The `npm-publish.yml` only shows `ls -lh pkg-bundler/*.wasm` for human inspection; it does not count or assert on the number of `#[wasm_bindgen]` exports. No gate exists.

To implement: after `wasm-pack build`, use `wasm-objdump` or `wasm-nm` or parse the `.wasm` with a small script:

```bash
EXPORT_COUNT=$(wasm-objdump -x pkg-bundler/oxilean_wasm_bg.wasm \
  | grep -c "^  - func\[")
MAX_EXPORTS=50  # set as brief specifies
if [ "$EXPORT_COUNT" -gt "$MAX_EXPORTS" ]; then
  echo "FAIL: $EXPORT_COUNT exports exceeds budget of $MAX_EXPORTS"
  exit 1
fi
```

Alternatively with `wasm-pack`-generated JS: count exported symbols in the `.d.ts` file.

### (f) WASM size budget gate (<=400KB gzip)

**Status: MISSING**

The `npm-publish.yml` prints size (`ls -lh`) but has no assertion. No gate exists.

Concrete step to add to `npm-publish.yml` after the build:
```yaml
- name: Check WASM gzip size
  working-directory: crates/oxilean-wasm
  run: |
    WASM_FILE=$(find pkg-bundler -name "*.wasm" | head -1)
    GZIP_SIZE=$(gzip -c "$WASM_FILE" | wc -c)
    MAX_BYTES=$((400 * 1024))
    echo "WASM gzip size: ${GZIP_SIZE} bytes (budget: ${MAX_BYTES})"
    if [ "$GZIP_SIZE" -gt "$MAX_BYTES" ]; then
      echo "FAIL: WASM exceeds 400KB gzip budget"
      exit 1
    fi
```

### (g) Native-vs-WASM determinism gate

**Status: COMPLETELY MISSING**

No test or harness compares native and WASM outputs for the same input. No mechanism exists.

Proposed skeleton for a determinism harness:

```
tests/determinism/
  run_determinism_check.sh
  corpus/
    simple.export
    quot.export
    nat_arith.export
```

`run_determinism_check.sh`:
```bash
#!/usr/bin/env bash
set -euo pipefail
CORPUS_DIR="tests/determinism/corpus"

# Build native verifier
cargo build --release --bin oxilean-verify

# Build WASM and use Node.js to run it
wasm-pack build crates/oxilean-wasm --target nodejs --features wasm
node tests/determinism/run_wasm.js  # wrapper that feeds same .export files

for f in "$CORPUS_DIR"/*.export; do
  NATIVE=$(./target/release/oxilean-verify check "$f")
  WASM=$(node tests/determinism/run_wasm.js "$f")
  if [ "$NATIVE" != "$WASM" ]; then
    echo "DETERMINISM FAIL on $f: native='$NATIVE' wasm='$WASM'"
    exit 1
  fi
done
echo "All determinism checks passed"
```

This must be a required CI step because integer arithmetic differences (e.g. u64 vs wasm32 wrapping) can cause silent divergence in bignum implementations.

### (h) Demo-under-python3-http.server smoke test

**Status: PARTIAL / NOT IN CI**

The playground infrastructure exists:
- `crates/oxilean-wasm/playground/build.sh` — builds and assembles `dist/`
- `crates/oxilean-wasm/playground/index.html`, `main.js`, `style.css` — static demo UI
- `build.sh:85`: prints `python3 -m http.server 8080` as the serve command (instruction only)

However there is **no automated CI smoke test** that:
1. Builds the WASM + playground
2. Starts `python3 -m http.server` in the background
3. Hits the running server (e.g. `curl http://localhost:8080/`) and checks 200 OK
4. Verifies the JS actually loads and can call into WASM

CI job skeleton:
```yaml
- name: Build playground
  working-directory: crates/oxilean-wasm
  run: bash playground/build.sh

- name: Smoke-test demo (python3 http.server)
  working-directory: crates/oxilean-wasm/dist
  run: |
    python3 -m http.server 8080 &
    SERVER_PID=$!
    sleep 2
    curl -fsS http://localhost:8080/ | grep -q "OxiLean"
    curl -fsS http://localhost:8080/oxilean_wasm_bg.wasm | file - | grep -q "WebAssembly"
    kill $SERVER_PID
```

---

## 3. Differential Testing Harness vs lean4lean / trepplein

### Does anything exist?

**Result of grep for `lean4lean` and `trepplein`:** No results. Neither string appears anywhere in the repository (`.rs`, `.toml`, `.yml`, `.sh`, `.md`).

There is a `differential_test` module in `crates/oxilean-elab/src/differential_test/` but it is **entirely internal** — it tests OxiLean against OxiLean (the `Lean4DiffTester` struct just runs the OxiLean elaborator and checks whether it returns success). It does not invoke any external binary or compare against any other Lean 4 checker.

Key types in that module:
- `DiffTestRunner` — runs `DiffTestCase` through OxiLean's elaborator
- `Lean4DiffTester` — wraps the above for "Lean 4 surface syntax" test cases
- `DiffTestHarness`, `DiffTestSuite`, `DiffTestReport` — test infrastructure

None of these invoke lean4lean, trepplein, or any external process.

### No `.export` corpus

There are zero `.export` files in the repository. The `lean4export` file format (produced by `lean4 --export`) is the bridge between Lean 4 and external checkers. No corpus directory, no sample `.export` files.

### Design for the differential harness

```
tests/differential/
  Cargo.toml              (binary crate, dev-only)
  corpus/
    lean4/
      nat_add.export
      quot_lift.export
      struct_eta.export
      list_map.export
    README.md             (how to regenerate corpus)
  src/
    main.rs               (CLI: --corpus DIR, --checker1 PATH, --checker2 PATH)
    runner.rs             (run one checker on one .export file, capture verdict)
    report.rs             (tabular diff output)
```

`runner.rs` interface:
```rust
pub enum Verdict {
    Verified,
    Unsupported(String),  // named missing feature
    Rejected(String),     // with error message
    Timeout,
    InternalError(String),
}

pub fn run_checker(binary: &Path, export_file: &Path, timeout: Duration) -> Verdict;
```

For oxilean-verify it calls `oxilean-verify check <file>` and maps exit codes:
- 0 → Verified
- 2 → Unsupported (stdout contains "unsupported: ...")
- 1 → Rejected (stdout contains "rejected: ...")

For lean4lean (a Lean 4 checker) or trepplein (a Scala checker):
- Must be pre-built and on PATH or specified via `--checker2`

`main.rs` produces a Markdown table:

| File | oxilean-verify | lean4lean | Agreement |
|------|---------------|-----------|-----------|
| nat_add.export | Verified | Verified | OK |
| quot_lift.export | Unsupported(Quot) | Verified | DIVERGE |

CI job in `.github/workflows/differential.yml`:
```yaml
name: Differential Testing
on:
  schedule:
    - cron: '0 4 * * 1'
  workflow_dispatch:

jobs:
  differential:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Build oxilean-verify
        run: cargo build --release --bin oxilean-verify
      - name: Install lean4lean (if available)
        run: |
          # lean4lean requires Lean 4 installed — skip in CI if not available
          echo "lean4lean: not installed, comparison limited to self-consistency"
      - name: Run differential harness
        run: cargo run --release -p oxilean-differential -- \
          --corpus tests/differential/corpus \
          --checker1 target/release/oxilean-verify \
          --output /tmp/diff_report.md
      - name: Upload differential report
        uses: actions/upload-artifact@v4
        with:
          name: differential-report
          path: /tmp/diff_report.md
```

---

## 4. cargo-deny / cargo-audit / rust-toolchain / MSRV

### cargo-deny / cargo-audit

**No `deny.toml` exists anywhere in the repository.**

The disabled `ci.yml` references `rustsec/audit-check@v1` (a security audit via RustSec advisory database), but since ci.yml has never been active, cargo-audit has never run in CI.

No `cargo deny` or `deny.toml` exists. There is no license checking, ban list, or source control for dependencies.

### rust-toolchain file

**No `rust-toolchain.toml` or `rust-toolchain` file exists anywhere.**

The disabled `ci.yml` uses `dtolnay/rust-toolchain@stable` without pinning. The active `npm-publish.yml` also uses `dtolnay/rust-toolchain@stable` with `targets: wasm32-unknown-unknown`. Neither pins to a specific version.

This means:
1. Build reproducibility is broken — any nightly stable release could change behavior
2. MSRV cannot be verified automatically without a pinned toolchain

### MSRV policy: is `rust-version = "1.70"` honest?

**Likely NOT honest — MSRV is understated.**

Evidence:
- `std::sync::OnceLock` is used in `crates/oxilean-kernel/src/declaration/types.rs:1175-1176` and `crates/oxilean-codegen/src/opt_alias/types.rs:986`. `OnceLock` was stabilized in **Rust 1.70.0** — so 1.70 is technically the floor for these files.
- Workspace dependency inheritance (`version.workspace = true`, `edition.workspace = true`, etc.) was stabilized in **Rust 1.64**. So 1.64 is the minimum for workspace inheritance syntax.
- The workspace itself uses `edition = "2021"` which requires Rust 1.56+.
- **Key risk**: The workspace `Cargo.toml` uses `workspace.dependencies` with `dep:` feature syntax in `crates/oxilean/Cargo.toml` (`dep:oxilean-kernel` etc.). `dep:` in feature definitions requires **Rust 1.60+**.
- The installed Rust is 1.95.0. No MSRV test job exists to verify 1.70 actually compiles.
- `HashMap::from([...])` is in `oxilean-std/src/type_inference_algorithms/types.rs:864`. `HashMap::from` via the `From` impl was stabilized in Rust **1.56** via the `IntoIterator for arrays` feature — acceptable for 1.70.
- No evidence of let-chains (nightly-only until 1.88), `LazyLock` (1.80+), or other features that would push the floor above 1.70.

**Verdict**: 1.70 is probably the correct MSRV given `OnceLock` usage, but it has never been verified by CI. Add a MSRV check job:
```yaml
- name: Check MSRV
  uses: dtolnay/rust-toolchain@1.70
- name: Build on MSRV
  run: cargo build --workspace
```

---

## 5. publish.sh Analysis

File: `publish.sh`

**Tier ordering (dependency order):**
| Tier | Crates |
|------|--------|
| 1 | oxilean-kernel |
| 2 | oxilean-parse, oxilean-meta, oxilean-std, oxilean-codegen, oxilean-runtime |
| 3 | oxilean-elab, oxilean-build, oxilean-lint |
| 4 | oxilean-cli, oxilean-wasm |
| 5 | oxilean (umbrella) |

**Missing crates for oxilean-verify:**
- `oxilean-export` — not in any tier
- `oxilean-verify` — not in any tier

Both must be added before publishing `oxilean-verify`. Proposed placement:
- `oxilean-export` → Tier 2 (depends only on `oxilean-kernel`)
- `oxilean-verify` → Tier 4 alongside `oxilean-cli` (depends on `oxilean-kernel` + `oxilean-export`; the WASM variant of verify would also be in Tier 4)

The script uses `cargo publish --allow-dirty` in both dry-run and real modes. This is risky — `--allow-dirty` bypasses the check that ensures only committed files are published. For the real publish path, this should be removed (the script already checks `git diff --quiet` before running, so `--allow-dirty` is redundant in clean mode and dangerous in dirty-workspace mode).

The script correctly sleeps 30s between tiers for crates.io index propagation (only in real publish mode). Dry-run skips the sleep.

---

## 6. Summary of All Gates — Gap Table

| Gate | Required by Brief | Present | Quality |
|------|-----------------|---------|---------|
| Any CI on push/PR | Yes | NO | Critical gap |
| Zero-dep gate for oxilean-kernel | Yes | NO | Dep section empty but not gated |
| Dep allow-list for verify product | Yes | NO | No deny.toml, no allowed-deps.txt |
| `#![forbid(unsafe_code)]` gate | Yes | PARTIAL | kernel+meta only, no CI assertion |
| cargo-fuzz on export reader | Yes | NO | No fuzz/ dir, no oxilean-export crate |
| WASM export-count gate | Yes | NO | No wasm-objdump check |
| WASM size <=400KB gzip | Yes | NO | `ls -lh` printed but not asserted |
| Native-vs-WASM determinism | Yes | NO | No harness |
| Demo python3 http.server smoke | Yes | PARTIAL | build.sh exists, no CI test |
| Differential harness vs lean4lean | Yes | NO | No external checker invoked |
| Differential corpus (.export files) | Yes | NO | No .export files |
| cargo-deny (license/bans) | Recommended | NO | No deny.toml |
| cargo-audit (security) | In disabled CI | NO | audit-check referenced but disabled |
| rust-toolchain pinned | Recommended | NO | No rust-toolchain file |
| MSRV verification job | Recommended | NO | No CI job on 1.70 toolchain |

---

## 7. Recommended CI Workflow Structure (new files to create)

### `.github/workflows/ci.yml` (re-enable and extend)
```yaml
name: CI
on:
  push:
    branches: [main, dev, master]
  pull_request:
    branches: [main, master]
env:
  CARGO_TERM_COLOR: always
  RUSTFLAGS: "-D warnings"
jobs:
  # --- Basic hygiene ---
  test:
    name: Test Suite
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@1.70   # pin to MSRV
      - uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
      - run: cargo build --workspace
      - run: cargo test --workspace --no-fail-fast
      - run: cargo clippy --workspace --all-targets -- -D warnings
      - run: cargo fmt --all -- --check

  # --- TCB gates ---
  kernel-zero-deps:
    name: Kernel has zero external deps
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Check kernel has no runtime deps
        run: |
          OUTPUT=$(cargo tree -p oxilean-kernel --edges normal 2>&1)
          COUNT=$(echo "$OUTPUT" | grep -v "^oxilean-kernel" | grep -v "^\[" | grep -c "^" || true)
          if [ "$COUNT" -gt "0" ]; then
            echo "FAIL: oxilean-kernel has $COUNT external deps:"
            echo "$OUTPUT"
            exit 1
          fi
          echo "OK: oxilean-kernel is dep-free"

  forbid-unsafe:
    name: TCB crates forbid unsafe code
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Check forbid(unsafe_code) in TCB
        run: |
          for crate in crates/oxilean-kernel crates/oxilean-export; do
            if [ -f "$crate/src/lib.rs" ]; then
              grep -r "#!\[forbid(unsafe_code)\]" "$crate/src/lib.rs" \
                || { echo "FAIL: $crate missing #![forbid(unsafe_code)]"; exit 1; }
            fi
          done

  # --- WASM gates ---
  wasm-size-gate:
    name: WASM size <=400KB gzip
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown
      - name: Install wasm-pack
        run: cargo install wasm-pack --version 0.13.1 --locked || true
      - name: Build WASM
        working-directory: crates/oxilean-wasm
        run: wasm-pack build --target web --features wasm --release --out-dir pkg-web
      - name: Gate: gzip size
        working-directory: crates/oxilean-wasm
        run: |
          WASM_FILE=$(find pkg-web -name "*.wasm" | head -1)
          GZIP_SIZE=$(gzip -c "$WASM_FILE" | wc -c)
          MAX=$((400 * 1024))
          echo "WASM gzip: ${GZIP_SIZE}B / budget: ${MAX}B"
          test "$GZIP_SIZE" -le "$MAX" || { echo "FAIL: exceeds 400KB"; exit 1; }

  # --- Demo smoke test ---
  demo-smoke:
    name: Demo runs under python3 http.server
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown
      - name: Install wasm-pack
        run: cargo install wasm-pack --version 0.13.1 --locked || true
      - name: Build playground
        working-directory: crates/oxilean-wasm
        run: bash playground/build.sh
      - name: Smoke test with python3 http.server
        working-directory: crates/oxilean-wasm/dist
        run: |
          python3 -m http.server 8080 &
          SERVER_PID=$!
          sleep 2
          curl -fsS http://localhost:8080/ | grep -q "OxiLean"
          kill $SERVER_PID

  # --- Security ---
  security-audit:
    name: Security Audit
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: rustsec/audit-check@v1
        with:
          token: ${{ secrets.GITHUB_TOKEN }}
```

### `.github/workflows/fuzz.yml` (new — requires oxilean-export crate to exist)
```yaml
name: Fuzz Export Reader
on:
  schedule:
    - cron: '0 3 * * *'
  workflow_dispatch:
jobs:
  fuzz:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@nightly
      - name: Install cargo-fuzz
        run: cargo install cargo-fuzz
      - name: Run fuzz (60s CI budget)
        working-directory: crates/oxilean-export
        run: cargo fuzz run fuzz_export_parser -- -max_total_time=60
      - name: Upload corpus
        if: always()
        uses: actions/upload-artifact@v4
        with:
          name: fuzz-corpus
          path: crates/oxilean-export/fuzz/corpus/
```

### `.github/workflows/determinism.yml` (new)
```yaml
name: Native-vs-WASM Determinism
on:
  schedule:
    - cron: '0 5 * * 1'
  workflow_dispatch:
jobs:
  determinism:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown
      - name: Build native verifier
        run: cargo build --release --bin oxilean-verify
      - name: Build WASM (Node target)
        working-directory: crates/oxilean-wasm
        run: wasm-pack build --target nodejs --features wasm --out-dir pkg-node
      - uses: actions/setup-node@v4
        with:
          node-version: '22'
      - name: Run determinism check
        run: bash tests/determinism/run_determinism_check.sh
```

---

## 8. Key File References

| File | Relevance |
|------|-----------|
| `.github/workflows.disabled/ci.yml` | Disabled CI — needs resurrection + extension |
| `.github/workflows.disabled/bench.yml` | Disabled bench — weak (uses `--test bench_`) |
| `.github/workflows/npm-publish.yml` | Only active CI — tag-triggered WASM publish |
| `publish.sh` | Tier-ordered publish; missing oxilean-export + oxilean-verify |
| `Cargo.toml` (workspace root) | MSRV 1.70, edition 2021, no rust-toolchain file |
| `crates/oxilean-kernel/Cargo.toml` | Correctly empty `[dependencies]` |
| `crates/oxilean-kernel/src/lib.rs:263` | `#![forbid(unsafe_code)]` present |
| `crates/oxilean-meta/src/lib.rs:316` | `#![forbid(unsafe_code)]` present |
| `crates/oxilean-parse/src/expr_cache/types/impls.rs:274` | Real `unsafe { std::ptr::read }` — parse crate is NOT safe |
| `crates/oxilean-wasm/playground/build.sh` | Builds demo; mentions `python3 -m http.server` |
| `tests/` (workspace root) | integration, mathlib_compat, formal_proofs, etc. — no export-reader tests |
| `crates/oxilean-elab/src/differential_test/` | Internal diff harness, NOT vs lean4lean/trepplein |
