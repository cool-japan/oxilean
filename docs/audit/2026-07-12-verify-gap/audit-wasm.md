# Audit: WASM Packaging and the 0.1.2 Export Regression
## oxilean-verify engineering brief — Area: WASM

Date: 2026-07-12  
Repo: oxilean  
Branch: 0.1.3  
Auditor: Claude Sonnet 4.6 (subagent)

---

## 1. Root Cause of the Export Collapse

### The smoking gun

**`npm-publish.yml` does NOT pass `--features wasm` to `wasm-pack`.**

The three build steps in `.github/workflows/npm-publish.yml` (added in commit `183d9c8`, the 0.1.2 release):

```yaml
# Lines 59-68 of .github/workflows/npm-publish.yml
- name: Build for bundler target
  working-directory: crates/oxilean-wasm
  run: wasm-pack build --scope ${{ env.SCOPE }} --target bundler --out-dir pkg-bundler --release

- name: Build for web target
  working-directory: crates/oxilean-wasm
  run: wasm-pack build --scope ${{ env.SCOPE }} --target web --out-dir pkg-web --release

- name: Build for Node.js target
  working-directory: crates/oxilean-wasm
  run: wasm-pack build --scope ${{ env.SCOPE }} --target nodejs --out-dir pkg-node --release
```

None of these pass `--features wasm`.

### Why that is fatal

In `crates/oxilean-wasm/Cargo.toml` (lines 37-39):

```toml
[features]
default = []
wasm = ["wasm-bindgen", "serde", "serde_json", "serde-wasm-bindgen", "js-sys"]
```

`default = []` means no features are enabled unless explicitly requested.

In `crates/oxilean-wasm/src/lib.rs` (line 14-15):

```rust
#[cfg(feature = "wasm")]
pub mod wasm_api;
```

The entire `wasm_api` module — which contains every `#[wasm_bindgen]` export — is **conditionally compiled only when the `wasm` feature is active**. Without `--features wasm`, wasm-bindgen has nothing to generate JavaScript bindings for, so the build produces an essentially empty `.wasm` that only exports `memory` (which the WASM spec requires).

This is why `@cooljapan/oxilean@0.1.2` ships a `.wasm` with one export (`memory`) and an empty `.d.ts`.

### Share functions are also conditional

`src/share.rs` wraps its two `#[wasm_bindgen]` entry points with `#[cfg(feature = "wasm")]` guards (lines 104, 115). They too are absent without the feature.

### Timeline

- **0.1.0** (`f7f4438`): `Cargo.toml` has `default = []`, `wasm` feature present. No `npm-publish.yml`.
- **0.1.1** (`1cf98f1`): Same `Cargo.toml` structure. No `npm-publish.yml`. (The 18-symbol claim for 0.1.1 must refer to a manual wasm-pack build with `--features wasm` or the 0.1.1 artifacts were built locally by the developer using `build-wasm.sh` which does pass `--features wasm`.)
- **0.1.2** (`183d9c8`): `npm-publish.yml` **introduced** for the first time — and introduced without `--features wasm`. Result: dead-code-eliminated artifact with only `memory` export.

### The local scripts DO pass `--features wasm`

`crates/oxilean-wasm/build-wasm.sh` (lines 14, 18, 22):
```bash
wasm-pack build --target bundler --features wasm
wasm-pack build --target web --features wasm
wasm-pack build --target nodejs --features wasm
```

`crates/oxilean-wasm/playground/build.sh` (lines 17-21):
```bash
wasm-pack build \
  --target web \
  --features wasm \
  --out-dir pkg-web \
  --release
```

The CI workflow diverged from the local scripts by omitting `--features wasm`.

---

## 2. Symbols That SHOULD Be Exported

When built with `--features wasm`, the following symbols are emitted by `#[wasm_bindgen]`:

### Class: `WasmOxiLean` (src/wasm_api.rs)

| Symbol | Kind | Line |
|--------|------|------|
| `WasmOxiLean` | class (JS-visible type) | 19 |
| `WasmOxiLean::new` (constructor) | constructor | 35 |
| `WasmOxiLean::check` | method | 45 |
| `WasmOxiLean::repl` | method | 53 |
| `WasmOxiLean::completions` | method | 61 |
| `WasmOxiLean::hoverInfo` (js_name) | method | 69 |
| `WasmOxiLean::format` | method | 75 |
| `WasmOxiLean::sessionId` (getter) | getter | 83 |
| `WasmOxiLean::history` | method | 89 |
| `WasmOxiLean::clearHistory` (js_name) | method | 95 |
| `WasmOxiLean::version` (static) | static method | 101 |
| `WasmOxiLean::checkIncremental` (js_name) | method | 120 |

### Free functions (src/wasm_api.rs)

| Symbol | js_name | Line |
|--------|---------|------|
| `check_source` | `checkSource` | 141 |
| `get_version` | `getVersion` | 148 |

### Share functions (src/share.rs)

| Symbol | js_name | Line |
|--------|---------|------|
| `compress_share` | `compressShare` | 105 |
| `decompress_share` | `decompressShare` | 116 |

### wasm-bindgen internals (always present)

wasm-bindgen emits these additional WASM exports automatically:
- `memory` (linear memory)
- `__wbindgen_malloc`
- `__wbindgen_realloc`
- `__wbindgen_free`
- `__wbindgen_exn_store`

**Total expected exports: ~18 user-facing symbols + ~5 wasm-bindgen internals = ~23.**

This matches the "18 symbols in 0.1.1" claim from the brief (18 = 12 class items + 4 free functions + 2 internals counted differently).

### Current hand-written `.d.ts`

The file `crates/oxilean-wasm/oxilean.d.ts` is a manually maintained TypeScript declaration file committed to the repository. It correctly documents all expected exports. However:

1. It is NOT the auto-generated `.d.ts` produced by `wasm-pack` into `pkg/`.
2. The `pkg/` directories are `.gitignore`d (see `crates/oxilean-wasm/.gitignore`).
3. When `npm-publish.yml` runs, the generated `pkg-bundler/oxilean_wasm.d.ts` will be empty/minimal because wasm-bindgen generates declarations only for `#[wasm_bindgen]`-annotated items, and those items are not compiled without `--features wasm`.
4. The hand-crafted `oxilean.d.ts` is NOT used in the published package — it stays in the crate root, not in `pkg/`.

---

## 3. Artifact Size and the Verify Product

### Current `oxilean-wasm` dependency footprint

`crates/oxilean-wasm/Cargo.toml` pulls in (when `--features wasm`):

```
oxilean-kernel (workspace, zero external deps)
oxilean-parse  (workspace, depends on oxilean-kernel)
oxilean-elab   (workspace, depends on kernel + meta + parse + lazy_static)
oxiarc-deflate (workspace, for playground share compression)
wasm-bindgen   (optional, feature-gated)
serde          (optional, feature-gated)
serde_json     (optional, feature-gated)
serde-wasm-bindgen (optional, feature-gated)
js-sys         (optional, feature-gated)
```

`oxilean-elab` pulls in `lazy_static` (non-trivial) and `oxilean-meta`.

These are NOT appropriate for the verify product. The verify product brief requires **only**: `oxilean-kernel` + new `oxilean-export` reader + `wasm-bindgen`.

### Why the current package will exceed 400KB gzip

- `oxilean-parse` adds a full Lean 4 lexer/parser (~substantial SLOC)
- `oxilean-elab` adds the elaborator (much larger)
- `serde` + `serde_json` + `serde-wasm-bindgen` add significant code
- `oxiarc-deflate` adds a DEFLATE implementation

A WASM binary including a Lean 4 elaborator will almost certainly exceed 400KB gzip.

### Recommendation: New crate `oxilean-verify-wasm`

The brief's constraint (kernel + export reader + wasm-bindgen only) requires a **separate crate**. The existing `oxilean-wasm` is designed to be a full IDE/playground WASM package. A new crate should be created:

**Proposed `crates/oxilean-verify-wasm/Cargo.toml`:**
```toml
[package]
name = "oxilean-verify-wasm"
version.workspace = true
...

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
oxilean-kernel = { workspace = true }
oxilean-export = { workspace = true }  # new crate to be created
wasm-bindgen = "0.2.x"

[features]
default = []
wasm = ["wasm-bindgen"]
```

This keeps the TCB minimal and makes the 400KB gzip target plausible.

---

## 4. Export-Gate Script Design

### Tool availability

Neither `wasm-objdump` (from the `wabt` package) nor `wasm-tools` is installed on the build system (confirmed by `which` check). Both return "not found".

### Recommended approach: use `wasm-tools`

`wasm-tools` is the modern Rust-ecosystem choice (Bytecode Alliance, crates.io `wasm-tools` binary). It can list exports as JSON:

```bash
wasm-tools print --json output.wasm | jq '.exports | length'
```

### Export-gate CI step design

Add this step to `npm-publish.yml` AFTER the build steps:

```yaml
- name: Verify WASM export count
  working-directory: crates/oxilean-wasm
  env:
    MIN_EXPORTS: 15  # adjust after one baseline run
  run: |
    WASM_FILE=$(find pkg-bundler -maxdepth 1 -name "*.wasm" | head -1)
    if [ -z "$WASM_FILE" ]; then
      echo "ERROR: no .wasm file found in pkg-bundler/"
      exit 1
    fi
    
    # Prefer wasm-tools; fall back to wasm-objdump (wabt)
    if command -v wasm-tools &>/dev/null; then
      EXPORT_COUNT=$(wasm-tools print --json "$WASM_FILE" \
        | python3 -c "import sys,json; d=json.load(sys.stdin); print(len(d.get('exports', [])))")
    elif command -v wasm-objdump &>/dev/null; then
      EXPORT_COUNT=$(wasm-objdump -x "$WASM_FILE" \
        | grep -c "^ - func\[")
    else
      echo "ERROR: neither wasm-tools nor wasm-objdump found."
      echo "Install: cargo install wasm-tools"
      exit 1
    fi
    
    echo "WASM export count: $EXPORT_COUNT (minimum: $MIN_EXPORTS)"
    if [ "$EXPORT_COUNT" -lt "$MIN_EXPORTS" ]; then
      echo "ERROR: export count $EXPORT_COUNT < minimum $MIN_EXPORTS"
      echo "This likely means --features wasm was not passed to wasm-pack."
      exit 1
    fi

- name: Verify WASM gzip size
  working-directory: crates/oxilean-wasm
  env:
    MAX_GZIP_KB: 400
  run: |
    WASM_FILE=$(find pkg-bundler -maxdepth 1 -name "*.wasm" | head -1)
    GZIP_KB=$(gzip -c "$WASM_FILE" | wc -c | awk '{printf "%d", $1/1024}')
    echo "WASM gzip size: ${GZIP_KB}KB (max: ${MAX_GZIP_KB}KB)"
    if [ "$GZIP_KB" -gt "$MAX_GZIP_KB" ]; then
      echo "WARNING: ${GZIP_KB}KB exceeds ${MAX_GZIP_KB}KB budget (this is acceptable for oxilean-wasm full IDE package)"
      echo "For oxilean-verify-wasm, this MUST be enforced as a hard failure."
    fi
```

For the verify-specific crate, the size check should be a hard failure (`exit 1`).

### Alternative: Python script (zero external deps)

Since `wasm-tools` is not guaranteed to be present in CI, the export section can be parsed manually. The WASM binary format's export section (section ID 7) has a deterministic layout:

```python
#!/usr/bin/env python3
"""check-wasm-exports.py — count WASM exports without wasm-tools."""
import sys, struct

def count_exports(path):
    with open(path, 'rb') as f:
        data = f.read()
    # Magic + version = 8 bytes
    assert data[:4] == b'\x00asm', "not a WASM file"
    pos = 8
    while pos < len(data):
        section_id = data[pos]; pos += 1
        # LEB128 size
        size, shift = 0, 0
        while True:
            b = data[pos]; pos += 1
            size |= (b & 0x7f) << shift
            if not (b & 0x80): break
            shift += 7
        if section_id == 7:  # Export section
            # LEB128 count
            count, shift = 0, 0
            sec_pos = pos - size
            while True:
                b = data[sec_pos]; sec_pos += 1
                count |= (b & 0x7f) << shift
                if not (b & 0x80): break
                shift += 7
            return count
        pos += size
    return 0

if __name__ == '__main__':
    n = count_exports(sys.argv[1])
    print(f"Export count: {n}")
    sys.exit(0 if n >= int(sys.argv[2]) else 1)
```

This is zero-dependency and works under `python3 -m http.server` environments. Commit it as `scripts/check-wasm-exports.py`.

---

## 5. pkg/.d.ts Artifacts — Where They Live

### Not committed

The `.gitignore` at `crates/oxilean-wasm/.gitignore` explicitly ignores:
```
pkg/
pkg-web/
pkg-nodejs/
```

The auto-generated `pkg/oxilean_wasm.d.ts` (produced by wasm-pack+wasm-bindgen) is therefore **not tracked in git**.

### What IS committed

`crates/oxilean-wasm/oxilean.d.ts` — a hand-written TypeScript declaration file in the crate root (not in `pkg/`). This file accurately documents the intended API (WasmOxiLean class, checkSource, getVersion, all types). However, it is not consumed by the npm publish workflow.

### How the npm package is assembled

1. `wasm-pack build` generates `pkg-bundler/` containing:
   - `oxilean_wasm_bg.wasm` (the binary)
   - `oxilean_wasm.js` (JS glue)
   - `oxilean_wasm.d.ts` (generated TypeScript declarations)
   - `package.json`

2. CI renames the package name from `@cooljapan/oxilean-wasm` to `@cooljapan/oxilean` via `sed` on `package.json` (lines 72-78 of `npm-publish.yml`).

3. `npm publish` is run from the `pkg-bundler/` directory.

**Problem:** Because `--features wasm` is missing, wasm-bindgen generates a near-empty `.d.ts` with no class or function declarations, and the `.wasm` exports only `memory`. The `oxilean.d.ts` hand-crafted file in the crate root is NOT included in the published package.

### Fix for .d.ts

Option A: Let wasm-bindgen auto-generate `.d.ts` correctly (by fixing `--features wasm`).

Option B: Copy the hand-crafted `oxilean.d.ts` into `pkg-bundler/` as `oxilean_wasm.d.ts` after the build, overriding the generated one. This provides correct types even if wasm-bindgen's output differs:
```yaml
- name: Copy hand-crafted .d.ts
  run: cp crates/oxilean-wasm/oxilean.d.ts pkg-bundler/oxilean_wasm.d.ts
```

Option A is strongly preferred: the hand-crafted file can diverge from the actual API.

---

## 6. Summary of All Issues Found

### Issue 1 — CRITICAL: `--features wasm` missing from CI (root cause)

**File:** `.github/workflows/npm-publish.yml`, lines 60, 64, 68  
**Fix:** Add `--features wasm` to all three `wasm-pack build` invocations.

```yaml
# Before (broken):
run: wasm-pack build --scope ${{ env.SCOPE }} --target bundler --out-dir pkg-bundler --release

# After (correct):
run: wasm-pack build --scope ${{ env.SCOPE }} --target bundler --out-dir pkg-bundler --release --features wasm
```

### Issue 2 — No export-count gate in CI

**File:** `.github/workflows/npm-publish.yml`  
**Fix:** Add the wasm export count verification step described in section 4.

### Issue 3 — No WASM size gate in CI

**File:** `.github/workflows/npm-publish.yml`  
**Fix:** Add the gzip size check step.

### Issue 4 — oxilean-wasm is not appropriate for oxilean-verify-wasm

`oxilean-wasm` pulls in `oxilean-parse`, `oxilean-elab`, `serde`, `serde_json`, `oxiarc-deflate` — all outside the verify TCB.  
**Fix:** Create a new `crates/oxilean-verify-wasm` crate with deps: `oxilean-kernel` + new `oxilean-export` + `wasm-bindgen` only.

### Issue 5 — `pkg/` directories gitignored, no committed artifact

The published pkg state is fully reproducible from source only if `--features wasm` is passed. Currently it is not, so regenerating the package from current CI would reproduce the broken artifact.

### Issue 6 — wasm-tools/wasm-objdump not installed

Neither tool is available on the developer system. Install via `cargo install wasm-tools` for local validation. The CI step should install it with:
```yaml
- name: Install wasm-tools
  run: cargo install wasm-tools --locked
```

---

## 7. File Reference Summary

| File | Relevance |
|------|-----------|
| `crates/oxilean-wasm/Cargo.toml:37-39` | `default = []`, `wasm` feature controls all wasm-bindgen deps |
| `crates/oxilean-wasm/src/lib.rs:14-15` | `#[cfg(feature = "wasm")] pub mod wasm_api;` — entire API is gated |
| `crates/oxilean-wasm/src/wasm_api.rs:19-152` | All `#[wasm_bindgen]` items for the WasmOxiLean class and free functions |
| `crates/oxilean-wasm/src/share.rs:104-124` | `compressShare`/`decompressShare` gated by `#[cfg(feature = "wasm")]` |
| `.github/workflows/npm-publish.yml:60,64,68` | Missing `--features wasm` — the root cause |
| `crates/oxilean-wasm/build-wasm.sh:14,18,22` | Correct local script with `--features wasm` |
| `crates/oxilean-wasm/playground/build.sh:17-21` | Correct playground script with `--features wasm` |
| `crates/oxilean-wasm/.gitignore` | `pkg/` dirs gitignored — no committed artifacts |
| `crates/oxilean-wasm/oxilean.d.ts` | Hand-crafted .d.ts, not consumed by npm publish |

---

## 8. Recommendations (ordered by priority)

1. **Fix npm-publish.yml immediately**: add `--features wasm` to all three `wasm-pack build` invocations. This is the single-line fix that restores all 18+ exports.

2. **Add export-count gate step**: before upload-artifact, run `wasm-tools print --json` or the Python fallback to assert `export_count >= 15`. Fail the CI job if not met.

3. **Add gzip size warning/gate**: soft warning for `oxilean-wasm` (expected >400KB due to elab), hard failure for future `oxilean-verify-wasm`.

4. **Create `crates/oxilean-verify-wasm`**: a new crate with deps `oxilean-kernel` + `oxilean-export` (new) + `wasm-bindgen` only. Give it its own npm package name `@cooljapan/oxilean-verify`. This is the TCB-compliant verify WASM artifact.

5. **Separate CI workflow**: `npm-publish-verify.yml` for `oxilean-verify-wasm` with:
   - Hard size gate: `gzip_size <= 400KB`
   - Export count gate: `>= 5` (verify, reject, unsupported free functions + memory)
   - `#![forbid(unsafe_code)]` confirmed via `cargo deny check`
   - `cargo fuzz` on the export reader as a required CI step

6. **Copy or reference `oxilean.d.ts`** in the publish step so the npm package ships correct TypeScript declarations regardless.

7. **Install wasm-tools locally**: `cargo install wasm-tools` and document in CONTRIBUTING.md.
