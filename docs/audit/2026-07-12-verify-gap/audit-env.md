# Audit: Environment and Toolchain Availability
## Repository: oxilean
## Date: 2026-07-12
## Area: Environment and toolchain availability (this machine)

---

## 1. Rust Toolchain

| Tool | Status | Version |
|------|--------|---------|
| `rustc` | PRESENT | 1.95.0 (59807616e 2026-04-14) |
| `cargo` | PRESENT | 1.95.0 (f2d3ce0bd 2026-03-21) |
| `cargo-nextest` | PRESENT | 0.9.136 (1d5bf1ec9 2026-05-16) |
| `cargo-fuzz` | **MISSING** | not installed |
| `cargo-miri` / miri | **MISSING** | binary absent (miri not installed) |

**Rustup home:** `~/.rustup`
**Default toolchain:** `stable-x86_64-unknown-linux-gnu`
**Nightly toolchain:** **NOT installed** (needed for cargo-fuzz)
**Stable update available:** 1.95.0 → 1.97.0 (2026-07-07)

### Installed Rust targets:
- `wasm32-unknown-unknown` — PRESENT (critical for WASM build)
- `x86_64-unknown-linux-gnu` — PRESENT

**Implication:** `cargo-fuzz` requires nightly. The brief mandates cargo-fuzz on the export reader in CI. Both `rustup toolchain install nightly` and `cargo install cargo-fuzz` are needed before CI fuzzing works.

---

## 2. WASM Tooling

| Tool | Status | Notes |
|------|--------|-------|
| `wasm-pack` | **MISSING** | Critical for building the WASM target |
| `wasm-objdump` (wabt) | **MISSING** | `dpkg -l wabt` → not installed |
| `wasm-tools` | **MISSING** | Not in PATH, not in apt installed |
| `wasm-opt` (binaryen) | **MISSING** | `dpkg -l binaryen` → not installed |
| `node` / `npm` | **MISSING** | `dpkg -l nodejs` → not installed |

**Impact:** The build-wasm.sh script at `crates/oxilean-wasm/build-wasm.sh` calls `wasm-pack` directly. Without wasm-pack, the WASM build cannot run. The 400KB gzip gate and WASM export-count gate mentioned in the brief are currently unenforceable. The script also does `node -e "..."` to patch package.json; node is missing. However, since wasm-pack is the primary need, node is secondary.

**Note on wasm32-unknown-unknown:** The Rust target itself is installed, so `cargo build --target wasm32-unknown-unknown` would work at the Rust level, but wasm-pack is the build driver for the full WASM package + JS bindings.

---

## 3. Lean / Elan Ecosystem

| Tool | Status | Notes |
|------|--------|-------|
| `elan` | **MISSING** | Not in PATH, `~/.elan` does not exist |
| `lean` | **MISSING** | Not in PATH |
| `lake` | **MISSING** | Not in PATH |

**apt package `elan`:** Available at version `4.1.2-3.1ubuntu1` (from Ubuntu universe). This IS the real Lean version manager (`https://github.com/leanprover/elan`), not a coincidental package name. It has native deps: libc6, libcurl4, libgcc-s1, liblzma5, libzstd1, sensible-utils. Could be installed with `sudo apt install elan`.

**Lean4 version required:** `lean4export` (the tool that generates .ndjson corpus files) currently requires **`leanprover/lean4:v4.32.0-rc1`** (as per lean-toolchain file in `leanprover/lean4export` master branch, fetched 2026-07-12). This RC was released on 2026-06-17 and its Linux x86_64 tarball (`lean-4.32.0-rc1-linux.tar.zst`) is **537 MB compressed**.

**git-lfs:** **MISSING** — not in PATH.

---

## 4. Supplementary Tools

| Tool | Status | Notes |
|------|--------|-------|
| `python3` | PRESENT | 3.14.4 at `/usr/bin/python3` |
| `git-lfs` | **MISSING** | |
| `just` | PRESENT | Found in `~/.cargo/bin/just` |
| `rust-analyzer` | PRESENT | In cargo bin |
| `splitrs` | PRESENT | COOLJAPAN tool in cargo bin |

---

## 5. System Resources

**Disk space:**
- `/` (nvme): 471G total, 123G used, **324G free** (28% used) — ample for lean toolchain (537MB + expanded ~2-3GB)
- `/run/media/<user>/512G`: 477G, 298G used, **180G free** — also available
- `/run/media/<user>/Windows7_OS`: 451G, 262G used, **190G free**

**CPU:** **16 cores** (`nproc` → 16)

---

## 6. Existing lean4export Files on Disk

**Result: None found.** 

- `find / -maxdepth 4 -name "*.export"` → no results (searched the whole filesystem to depth 4)
- `find ~/work -name "*.ndjson"` → no results
- No `.export` or `.ndjson` files anywhere in `oxilean`

---

## 7. lean4lean / trepplein Checkouts

**`ls ~/work` shows:** `cargoclean.sh`, `ecosystem`, `noffi`, `numrs`, `oxilean`, `scirs`

- **No lean4lean checkout** (neither `lean4lean` nor `lean4lean` under any org)
- **No trepplein checkout**
- **No lean4export checkout**

`~/work/ecosystem/` appears to be a different COOLJAPAN project (has scirs2-related content).

---

## 8. Network Access

**GitHub:** REACHABLE. `curl -sI https://github.com --max-time 5` → `HTTP/2 200`

**Verified reachable repos:**
- `https://github.com/leanprover/lean4export` → HTTP/2 200
- `https://github.com/gebner/trepplein` → HTTP/2 200
- `https://github.com/digama0/lean4lean` → HTTP/2 200
- `https://github.com/leanprover/lean4` (releases) → accessible via API

**Not reachable (404):**
- `https://github.com/lean4lean/lean4lean` (wrong org)
- `https://github.com/0art0/lean4export`

---

## 9. lean4export Format Details (for implementer)

The `leanprover/lean4export` repo exports in **NDJSON format v3.0.0** (current format_ndjson.md says v3.1.0). Each line is one JSON object. The file begins with a `meta` object, then a stream of:

- **Name interning:** `{"in": <id>, "str": {"pre": <parent_id>, "str": "..."}}` or `{"in": <id>, "num": {"pre": <parent_id>, "i": <n>}}`
- **Level interning:** `{"il": <id>, "succ": <inner_id>}`, `{"il": <id>, "max": [l1, l2]}`, `{"il": <id>, "imax": [l1, l2]}`, `{"il": <id>, "param": <name_id>}` (Level.zero is always index 0)
- **Expr interning:** `{"ie": <id>, "bvar": <n>}`, `{"ie": <id>, "sort": <level_id>}`, `{"ie": <id>, "const": {"name": <name_id>, "us": [<level_id>...]}}`, `{"ie": <id>, "app": {"fn": <ie>, "arg": <ie>}}`, `{"ie": <id>, "lam": {"name": <n>, "binderInfo": "<bi>", "type": <ie>, "body": <ie>}}`, `{"ie": <id>, "forallE": {...}}`, `{"ie": <id>, "letE": {"name":..., "type":..., "value":..., "body":..., "nondep": bool}}`, `{"ie": <id>, "proj": {"typeName": <n>, "idx": <i>, "struct": <ie>}}`, `{"ie": <id>, "natVal": "<bignum_string>"}`, `{"ie": <id>, "strVal": "..."}`, `{"ie": <id>, "mdata": {"expr": <ie>, "data": {...}}}`
- **Declarations:**
  - `{"axiom": {"name": <n>, "levelParams": [...], "type": <ie>, "isUnsafe": bool}}`
  - `{"def": {"name": <n>, "levelParams": [...], "type": <ie>, "value": <ie>, "hints": ..., "safety": ..., "all": [...]}}`
  - `{"thm": {"name": <n>, "levelParams": [...], "type": <ie>, "value": <ie>, "all": [...]}}`
  - `{"opaque": {"name": <n>, "levelParams": [...], "type": <ie>, "value": <ie>, "isUnsafe": bool, "all": [...]}}`
  - `{"quot": {"name": <n>, "levelParams": [...], "type": <ie>, "kind": "type"|"ctor"|"lift"|"ind"}}`
  - `{"inductive": {"types": [InductiveVal...], "ctors": [ConstructorVal...], "recs": [RecursorVal...]}}`

**BinderInfo values:** `"default"`, `"implicit"`, `"strictImplicit"`, `"instImplicit"`
**ReducibilityHints:** `"opaque"`, `"abbrev"`, `{"regular": integer}`

**Example file available:** `https://raw.githubusercontent.com/leanprover/lean4export/refs/heads/master/examples/Nat.add_succ.ndjson` (572 lines, ~32KB, fetchable NOW via curl)

---

## 10. Missing Crate: oxilean-export

The workspace at `Cargo.toml` lists 14 crates but **does not include `oxilean-export`** (the brief's new reader crate with ≤3000 SLoC, zero deps, fuzzed). This is the critical missing deliverable. The workspace has no `crates/oxilean-export` directory.

The existing `crates/oxilean-kernel/src/export/` module appears to be an internal "export" for trait objects (it has files like `focusstack_traits.rs`, `functions.rs`, `modulecache_traits.rs` — these are internal kernel trait exports, NOT a lean4export reader).

---

## 11. CI Status

**Active workflow:** Only `npm-publish.yml` in `.github/workflows/`. Other workflows are in `.github/workflows.disabled/`.

**No CI gate for:**
- WASM export count
- WASM 400KB gzip size
- cargo-fuzz on export reader
- Native-vs-WASM determinism check

---

## 12. Path to a Real .export Corpus (feasibility analysis)

### Option A: Install elan + Lean 4.32.0-rc1 + lean4export + export Lean core

**Steps:**
1. `sudo apt install elan` (4.1.2-3.1ubuntu1, 1.6MB package, deps already on system)
2. elan will auto-download lean4 toolchain when configured
3. Clone `leanprover/lean4export` (small Lean package)
4. `lake build` inside lean4export → downloads Lean 4.32.0-rc1 (~537MB zst → ~2-3GB extracted)
5. `lake exe lean4export -- Init.Prelude` or similar to export lean core
6. Produces .ndjson corpus files

**Disk required:** ~3-4GB for Lean toolchain + Lean core build artifacts. 324GB available = feasible.
**Time estimate:** ~30-60 minutes for download + first build of Lean core.
**Risk:** lean4export requires v4.32.0-rc1 which is an RC; elan 4.1.2 from apt should handle this. The nightly nature of RC toolchains means future breakage is possible but current state is stable.

### Option B: Fetch prebuilt .ndjson fixtures

**Available NOW without any installation:**
- `https://raw.githubusercontent.com/leanprover/lean4export/refs/heads/master/examples/Nat.add_succ.ndjson` — 572 lines, ~32KB, covers Nat.add_succ proof with full dependency chain
  
**From lean4lean or trepplein:** No prebuilt `.export` or `.ndjson` corpus artifacts in releases (lean4lean has 0 releases; trepplein is Scala and targets the old Lean 3 text format, not the new ndjson v3 format).

**Assessment:** Option B gives a minimal smoke-test fixture immediately. For meaningful fuzzing and kernel verification, Option A is required.

### Option C: Hand-craft small .ndjson test vectors

For fuzz corpus seeding and unit tests, the format is now fully documented. Small hand-written vectors can be created without any Lean installation. The `Nat.add_succ.ndjson` example provides a complete reference.

---

## 13. oxilean Workspace Relevant Details

**Kernel crate:** `crates/oxilean-kernel/Cargo.toml`
- `[dependencies]` is empty (zero external deps — CORRECT per brief)
- `#![forbid(unsafe_code)]` at line 263 of `src/lib.rs` — CORRECT
- `src/lib.rs` is 416 lines; total kernel SLoC is 146,570 lines across 950 `.rs` files — **far exceeds the brief's "~3500 SLOC" TCB claim** (likely most are stub/skeleton files)

**WASM crate:** `crates/oxilean-wasm/Cargo.toml`
- Has `wasm-bindgen`, `serde`, `serde_json`, `serde-wasm-bindgen`, `js-sys` as optional features under the `wasm` feature flag
- `getrandom` with `wasm_js` feature for wasm32 target
- Does NOT currently depend on oxilean-export (which doesn't exist yet)

**No oxilean-verify crate** exists in the workspace.
**No oxilean-export crate** exists in the workspace.

---

## 14. Summary of What Must Be Installed

To implement oxilean-verify per the brief, the following installations are needed (in priority order):

1. **`wasm-pack`** — required for WASM build (`cargo install wasm-pack` or via script)
2. **`rustup toolchain install nightly`** — required for cargo-fuzz
3. **`cargo install cargo-fuzz`** (after nightly) — required for CI fuzz gate
4. **`sudo apt install elan`** — required to get Lean 4 toolchain for .ndjson corpus generation
5. **Lean 4.32.0-rc1 toolchain** — downloaded automatically by elan from lean4export's lean-toolchain
6. **`node`/`npm`** — needed for wasm-pack's JS packaging step and build-wasm.sh patching (lower priority)
7. **`wasm-opt` (binaryen)** — needed for 400KB gate enforcement (`sudo apt install binaryen` or `cargo install wasm-opt`)
8. **`wabt`** — needed for wasm-objdump inspection (`sudo apt install wabt`)

**Immediate bootstrap without any installs:** Can fetch `Nat.add_succ.ndjson` from lean4export GitHub to start implementing the oxilean-export reader and writing the kernel interface. The ndjson format is fully documented.
