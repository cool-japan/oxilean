# oxilean-cli — TODO

> Task list for the CLI crate.
> Last updated: 2026-05-03

## ✅ Completed

- [x] Argument parsing (`--version`, `--help`, `check`, `repl`)
- [x] Version display
- [x] Help / usage display
- [x] Subcommand dispatch
- [x] **~64,848 SLOC implemented across 256 source files**

---

## ✅ Completed: `check` Command (COMPLETE)

### File Checking (`check <file>`)
- [x] Read `.oxilean` source file
- [x] Invoke lexer → token stream
- [x] Invoke parser → surface AST
- [x] Invoke elaborator → kernel expressions
- [x] Invoke kernel type checker → verified declarations
- [x] Report success message with declaration count
- [x] Report errors with source positions (file:line:col)
- [x] Exit code 0 on success, 1 on failure
- [x] Support multiple files

### Error Reporting
- [x] Colorized terminal output (errors in red, warnings in yellow)
- [x] Source span highlighting (show the relevant code)
- [x] Caret (`^`) underline for error position
- [x] Suggestion hints for common mistakes
- [x] Multi-error reporting

### Diagnostics
- [x] `--verbose` flag for detailed output
- [x] `--timing` flag for performance measurements
- [x] `--trace` flag for kernel reduction tracing

---

## ✅ Completed: `repl` Command (COMPLETE)

**Implementation**: `repl.rs` (~92,828 lines)

### Interactive REPL
- [x] Read-eval-print loop
- [x] Line editing (history, cursor movement)
- [x] Multi-line input (detect incomplete expressions)
- [x] Goal display formatting
- [x] Tactic input and execution
- [x] `Ctrl+C` to cancel current proof, `Ctrl+D` to exit
- [x] Undo / backtrack support

### REPL Commands
- [x] `#check <expr>` — infer and display type
- [x] `#eval <expr>` — evaluate and display result
- [x] `#print <name>` — print definition / type of a constant
- [x] `#help` — display REPL help
- [x] `#quit` / `#exit` — exit REPL

---

## ✅ Additional Features Implemented (Beyond Original Scope)

### Commands Module (`commands.rs` — 47,418 lines)
- [x] Command routing and orchestration
- [x] Common command utilities

### Build System (`build.rs` — 40,508 lines)
- [x] Multi-file compilation
- [x] Dependency resolution
- [x] Incremental build support

### Project Management (`project.rs` — 104,631 lines)
- [x] Project scaffolding (`new` command)
- [x] Project manifest handling
- [x] Dependency management
- [x] Build configuration

### Interactive Mode (`interactive.rs` — 75,663 lines)
- [x] Advanced interactive features
- [x] Enhanced goal visualization
- [x] Interactive tactic application

### Format Command (`format.rs` — 63,889 lines)
- [x] Source code formatting
- [x] Style enforcement
- [x] Configurable formatting options

### Documentation Generation (`docgen.rs` — 70,528 lines)
- [x] Documentation extraction
- [x] HTML documentation generation
- [x] Cross-reference generation

### LSP Server (`lsp/` directory — multiple files)
- [x] Language Server Protocol implementation
- [x] Completion support (`completion.rs`, `completion_adv.rs`)
- [x] Hover information (`hover.rs`, `hover_adv.rs`)
- [x] Diagnostics (`diagnostics.rs`, `diagnostics_adv.rs`)
- [x] Semantic tokens (`semantic_tokens.rs`)
- [x] Server implementation (`server.rs`)
- [x] UI widgets (`widgets.rs`)

### Export Features
- [x] JSON export (`json_export.rs` — 19,641 lines)
- [x] LaTeX export (`latex_export.rs` — 27,519 lines)

### File Watcher (`watcher.rs` — 24,816 lines)
- [x] Watch mode: re-check on file changes
- [x] Incremental re-compilation

### Benchmarking (`bench.rs` — 23,460 lines)
- [x] Performance benchmarking
- [x] Regression detection

### Proof Management (`proof.rs` — 2,792 lines)
- [x] Proof state management
- [x] Proof serialization

### Configuration (`config.rs` — 7,129 lines)
- [x] Configuration file handling
- [x] User preferences

---

## ✅ Completed: Extended CLI Features

- [x] Shell completions (bash, zsh, fish, PowerShell, Elvish) — `completions.rs` (6 tests)
- [x] Progress bar for large file checking (optional enhancement)
- [x] Additional export formats (if needed)

## v0.1.3 LSP + Tool Improvements

> Last updated: 2026-05-29 (Ring 1 in-progress items added)

### Ring 0

- [x] Add LSP server integration test suite (in-process mock transport)
  - **Goal:** Verify the existing 109-file LSP implementation actually processes a file end-to-end and emits correct `publishDiagnostics`
  - **Design:** Add `run_with_transport<R: Read, W: Write>(r: R, w: W)` entry point to LSP server (or detect if one already exists); write `#[cfg(test)]` integration test that pipes JSON-RPC messages in via in-memory channel/cursor; send `initialize` → `initialized` → `textDocument/didOpen` (with fixture containing a type error) → `shutdown` → `exit`; assert `publishDiagnostics` notification contains expected error
  - **Files:** `src/lsp/lsp_server/functions.rs` (add transport entry point), `src/lsp/tests/lsp_integration.rs` (new test file)
  - **Prerequisites:** None — LSP infrastructure complete
  - **Tests:** JSON-RPC round-trip; diagnostics on error fixture; clean shutdown sequence
  - **Risk:** Server may use blocking stdio reads — need to refactor into generic `Read`/`Write` transport to enable in-process testing

### Ring 1

- [x] VS Code extension skeleton under `src/lsp/editor/vscode/`
- [ ] `oxilean playground` subcommand that opens the wasm playground in browser
- [x] Shell completion improvements (dynamic completion from project symbols)
- [x] Wire `register_cc_helper` + `register_polyrith_helper` into production envs (cycle 10) (2026-05-30)
  - **Goal:** Add calls to `register_cc_helper(&mut env)` and `register_polyrith_helper(&mut env)` alongside `register_omega_helper` in `repl/types.rs` (lines ~70, ~104) and `commands/functions.rs` (line ~37). Both are guarded/idempotent. This ensures cc and polyrith proof terms kernel-verify in the live CLI/REPL.
  - **Files:** `src/repl/types.rs`, `src/commands/functions.rs`

### Ring 1 (in progress — this cycle)

- [x] LSP: Add `textDocument/semanticTokens/range` handler and advertise semanticTokensProvider (2026-05-29)
  - **Goal:** semanticTokens/full handler exists at `src/lsp/semantic_tokens/functions.rs:18` and is wired at `server/types.rs:268`, but the provider is not advertised in ServerCapabilities (`lsp_types/mod.rs:710`). Add the `/range` handler and advertise the full provider.
  - **Design:** Add handler fn `handle_semantic_tokens_range(doc, range) -> SemanticTokensRangeResult` in `semantic_tokens/functions.rs` — call the existing tokenizer, filter to range, return subset. Add arm to `RequestDispatcher::dispatch` in `server/types.rs`. In `lsp_types/mod.rs:710` (`oxilean_defaults()`), add `semanticTokensProvider: SemanticTokensServerCapabilities { legend: build_semantic_tokens_legend(), full: Some(true), range: Some(true) }`.
  - **Files:** `src/lsp/semantic_tokens/functions.rs`, `src/lsp/server/types.rs`, `src/lsp/lsp_types/mod.rs`
  - **Tests:** Round-trip test: send `textDocument/semanticTokens/range` with a sub-range; verify tokens returned are subset of full-doc tokens; verify advertised capability JSON contains `semanticTokensProvider`.
  - **Risk:** Range bound-check (byte offsets vs UTF-16 code units) — verify offset conversion with the existing tokenizer's conventions.

- [x] LSP: Wire oxilean-lint diagnostics into `textDocument/codeAction` responses (2026-05-29)
  - **Goal:** `handle_code_action` at `lsp_server/mod.rs:792` currently has standalone code actions (add-sorry, rename, etc.) but is NOT wired to oxilean-lint. Add oxilean-lint as a dep and surface its auto-fix suggestions.
  - **Design:** Add `oxilean-lint = { workspace = true }` to `Cargo.toml`. Implement `lint_to_code_action(lint_diag: &LintDiagnostic, uri: &str) -> Option<CodeAction>` — if the lint diagnostic has a fix suggestion, convert to `CodeAction { kind: "quickfix", edit: WorkspaceEdit { changes: ... } }`. Wire into `handle_code_action`: run linter on the document source, collect lint diagnostics that intersect the requested range, convert to CodeActions. If oxilean-lint lacks a fix-suggestion field on its diagnostic type, add one first (IMPLEMENT POLICY).
  - **Files:** `Cargo.toml`, `src/lsp/lsp_server/mod.rs`, `src/lsp/code_actions/functions.rs`
  - **Tests:** Test document with a lint error that has a fix (e.g. unused import); codeAction response includes a quickfix with the correct text edit.
  - **Risk:** oxilean-lint fix-suggestion API may not exist — build it inline in lint crate if missing.

- [x] LSP: Harden and advertise incremental text sync (text_document_sync 1 → 2) (2026-05-29)
  - **Goal:** `apply_incremental_change` (`server/types.rs:399`) is wired into didChange but capabilities advertise `text_document_sync: 1` (Full). Before flipping to `2` (Incremental), add a rigorous edge-case test battery.
  - **Design:** (a) Add integration tests for `apply_incremental_change`: UTF-16 code-unit offsets (multi-byte CJK chars — 3 bytes in UTF-8 = 1 code unit in UTF-16), emoji (4 bytes = 2 code units surrogate pair), multi-line insert/delete/replace, document boundary conditions, empty range insertions, assertions that incremental result == full-doc sync result for all cases. (b) Fix any bug found. (c) Only after tests pass, change `text_document_sync: 1` → `text_document_sync: 2` in `lsp_types/mod.rs:710`.
  - **Files:** `src/lsp/server/types.rs` (tests), `src/lsp/lsp_types/mod.rs` (capability flip)
  - **Tests:** ≥8 edge-case tests for apply_incremental_change covering the UTF-16/multibyte scenarios above.
  - **Risk:** This is the highest-risk item — a client-facing behavior change. Failing to handle UTF-16 offsets correctly will break all editors. The test battery is mandatory before the flip.

- [x] Incremental `didChange` sync (range-based partial parse) (2026-05-29)
  - **Goal:** Wire `oxilean_parse::parse_incremental_change` into the LSP `didChange` handler so the lex step is incremental (splice tokens) instead of full re-tokenize.
  - **Files:** `src/lsp/server/types.rs`, `src/lsp/document/mod.rs`, `src/lsp/analysis/mod.rs`.
  - **Tests:** `test_incremental_relex_single_token`, `test_incremental_relex_equals_full`, `test_incremental_relex_fallback_when_cache_empty`, `test_analyze_and_cache_updates_tokens`, `test_position_to_char_offset_*`.

- [x] Semantic highlighting token type specification (2026-05-29)
  - **Goal:** Define the canonical `SEMANTIC_TOKEN_LEGEND` constant with documented index mapping; ensure `semanticTokens/full` and `/range` use it.
  - **Files:** `src/lsp/semantic_tokens/functions.rs`.
  - **Tests:** `test_semantic_token_legend_stable`, `test_semantic_token_modifier_legend_stable`, `test_build_semantic_tokens_legend_matches_constants`, `test_oxi_token_type_index_matches_legend`.

- [x] `oxilean playground` subcommand — std::net static-file server (2026-05-30)
  - **Goal:** New `playground` subcommand serving `crates/oxilean-wasm/playground/dist/` over a minimal `std::net::TcpListener` handler (pure std, no external HTTP crate). Correct MIME types, path-traversal guard, best-effort browser open.
  - **Files:** `crates/oxilean-cli/src/main/functions.rs`, `crates/oxilean-cli/src/commands/playground.rs` (new).
  - **Tests:** MIME table; traversal rejection; request-line parsing; fixture file round-trip.
- [x] LSP fully-incremental analysis via `diff_modules` + `cached_decls` (2026-05-30)
  - **Goal:** Wire `oxilean_parse::diff_modules` into `analyze_document`; add `cached_decls: Vec<LocatedDecl>` to `Document`; re-analyze only from the first changed decl onward (tail-invalidation strategy for correctness).
  - **Files:** `crates/oxilean-cli/src/lsp/analysis/mod.rs`, `crates/oxilean-cli/src/lsp/document/mod.rs`.
  - **Tests:** append/edit/replace scenarios; full vs incremental identical diagnostics.
