# oxilean-wasm TODO

> Status: v0.1.2 WASM bindings complete. v0.1.3 adds browser playground.
> Last updated: 2026-05-29

## v0.1.2 (Complete)

- [x] WebAssembly bindings (wasm_api.rs) for parse/check/elaborate pipeline
- [x] wasm-pack build for bundler/web/nodejs targets
- [x] getrandom wasm_js feature fix for wasm32-unknown-unknown target
- [x] npm-publish.yml CI workflow for @cooljapan/oxilean

## v0.1.3 Playground

### Ring 0

- [x] Create `playground/` static-site browser proof assistant
  - **Goal:** Ship a browser-deliverable interactive proof assistant: CodeMirror 6 editor + live type-check via existing WASM API + result/diagnostics panel; `build.sh` produces deployable `dist/`
  - **Design:**
    - `playground/index.html` — two-panel layout (editor left, results right); loads `main.js` as ES module
    - `playground/main.js` — import CodeMirror 6 from pinned ESM CDN (`https://esm.sh/codemirror@6.0.1`); import WASM module from `../pkg-bundler/oxilean.js`; set up CM6 editor with debounced (300ms) onChange handler calling `check(source)`; render diagnostics (line/col + message) in result panel; handle WASM init promise
    - `playground/style.css` — minimal: flexbox two-panel, monospace editor font, error highlighting
    - `playground/build.sh` — `set -euo pipefail`; wasm-pack build `--target bundler --features wasm` producing `pkg-bundler/`; `mkdir -p dist`; `cp playground/{index.html,main.js,style.css} dist/`; `cp pkg-bundler/*.wasm pkg-bundler/*.js dist/`; echo success
  - **Files:** `crates/oxilean-wasm/playground/index.html`, `playground/main.js`, `playground/style.css`, `playground/build.sh`
  - **Prerequisites:** `src/wasm_api.rs` (already exists with `check()` export)
  - **Tests:** `bash playground/build.sh` exits 0; `dist/index.html` exists; `dist/*.wasm` exists
  - **Risk:** ESM CDN URL for CodeMirror 6 must be pinned; WASM init is async — `main.js` must await before calling `check()`

### Ring 1

- [x] Persistent storage via browser IndexedDB (completed 2026-05-29)
  - **Goal:** Save editor content to IndexedDB on edit; restore on page load. Pure JS in playground HTML — no Rust changes needed.
  - **Design:** On editor change events, write `{content: <string>}` to IndexedDB key `"oxilean_editor"`. On page load, read key and populate editor. Use the IndexedDB async API with Promises; wrap in a small `storage.js` helper.
  - **Files:** `playground/main.js` (openDB/saveContent/loadContent helpers; called from editor updateListener and main startup)
  - **Prerequisites:** None
  - **Tests:** Automated: set content, reload, verify content restored (Jest or manual test).
- [x] Share-via-URL (URL fragment = OxiARC-compressed base64 source) (completed 2026-05-29)
  - **Goal:** Add wasm-bindgen exports `compress_share(source: &str) -> String` (→ base64url) and `decompress_share(encoded: &str) -> Result<String, JsValue>`; playground JS writes the hash fragment on "Share" click and reads+decodes it on page load.
  - **Design:** `compress_share`: OxiARC `deflate(source.as_bytes(), 6)` → base64url encode (no padding, URL-safe charset); `decompress_share`: base64url decode → OxiARC `inflate` → UTF-8 string. Added `oxiarc-deflate = "0.3"` to workspace Cargo.toml and crate deps.
  - **Files:** `src/share.rs` (new), `src/lib.rs` (pub mod share), `playground/main.js` (Share button + fragment load on startup), `playground/index.html` (Share button, examples select), `/Cargo.toml` (workspace dep)
  - **Tests:** 7 new tests in `share::tests` — all 55 oxilean-wasm tests pass.
- [x] Bundled example library (10–15 proofs shipped in-page as dropdown) (completed 2026-05-29)
  - **Goal:** In-page `<select>` dropdown with 10-15 curated Lean4/oxilean proof examples; selecting one loads its code into the editor.
  - **Design:** 15 `<option>` entries hard-coded in `index.html` `<select id="examples-select">`; `initExamplesDropdown()` in `main.js` wires up change event to set editor content and reset selector to placeholder.
  - **Files:** `playground/index.html` (15 examples in select), `playground/main.js` (initExamplesDropdown)
  - **Tests:** Dropdown has 15 entries (≥10); selecting any example populates the editor.
- [x] Multi-file workspace UI (tab bar + file tree) (completed 2026-05-29)
- [ ] GitHub Pages deploy (separate user approval required for new workflow yaml)
- [x] Performance: incremental WASM check (diff-based partial reparse) (completed 2026-05-30)
  - **Goal:** Replace the line-based heuristic in `incremental/functions.rs` with `diff_modules`-driven AST diffing. Cache keys use `DeclFingerprint.body_hash` for stability.
  - **Design:** `parse_source_decls` (real parser) + `diff_modules` Myers LCS diff replaces `extract_declarations`. `IncrementalCache.prev_decls: Vec<Located<Decl>>` stores prior AST for diffing. Tail-invalidation: all decls after the first changed one are re-checked. `EditKind::{Inserted,Modified}` → recheck; `Unchanged` before first change → cache hit.
  - **Files:** `crates/oxilean-wasm/src/incremental/functions.rs`, `crates/oxilean-wasm/src/incremental/types.rs`.
  - **Tests:** 4 new tests: `test_incremental_check_full_equals_incremental`, `test_incremental_edit_middle_decl`, `test_incremental_append_decl`, `test_incremental_delete_decl`.
