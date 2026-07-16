# oxilean-doc TODO

> Status: v0.1.3 Ring 0 in progress
> Last updated: 2026-05-29

## v0.1.3 (Ring 0 — complete)

- [x] Crate scaffold: main.rs, extractor.rs, renderer.rs
- [x] Parse .lean source via oxilean-parse
- [x] Extract declarations + docstrings from AST (token-span correlation)
- [x] Render single-page HTML output with embedded CSS
- [x] Table of contents with kind badges
- [x] Integration tests using tempfile

## Ring 1

- [x] Multi-file crate-wide doc generation with cross-references (implemented 2026-05-29)
  - **Goal:** Walk all modules in a crate, build a symbol→page/anchor index, resolve intra-crate references in signatures and doc-comments to relative links, emit one HTML page per module plus an index page. Hosting-portable relative-link output (no absolute paths).
  - **Design:** (a) Module walker: recursively enumerate modules from crate root; (b) Symbol index: `HashMap<Name, (page_path, anchor_id)>` built in first pass; (c) Reference resolver: scan rendered doc-comment text for `[Name]`-style refs, replace with `<a href="relative">` using the symbol index; (d) Emitter: one `.html` per module in output dir + `index.html` listing all modules. Reuse the existing single-file generator for the per-module body.
  - **Files:** `src/` (read current structure first), `src/walker.rs` (new), `src/symbol_index.rs` (new), `src/cross_ref.rs` (new)
  - **Prerequisites:** Existing single-file generator (Ring 0, already done)
  - **Tests:** 2-module fixture: module A defines `foo`, module B's doc-comment `[foo]` resolves to a relative link pointing to module A's page; `index.html` lists both modules.
  - **Risk:** Module structure may require reading from a parsed AST rather than filesystem; adapt accordingly.
- [x] Client-side search index (JSON + vanilla JS search box in multi-file output)
  - `to_search_json()` on `SymbolIndex` emits `[{"name":…,"kind":"decl","page":…,"anchor":…,"signature":""}]`
  - `search-index.json` written alongside every `generate_multi_file` call
  - Vanilla-JS search box (`#search-input` / `#search-results`) embedded in `index.html` and all module pages
  - URL-depth-aware `fetch` base-path detection so search works from any page depth
- [x] Theming + hosting-ready relative-link output (implemented 2026-05-29)
  - **Goal:** Light/dark theming via CSS custom properties: convert hardcoded palettes in `renderer.rs` STYLE and `multifile.rs` MULTI_STYLE to `:root` + `[data-theme="dark"]` variables, default via `@media (prefers-color-scheme: dark)`, add a localStorage-persisted vanilla-JS toggle button (reuse the existing `<script>` injection pattern from search). Keep relative-link output hosting-portable.
  - **Files:** `crates/oxilean-doc/src/renderer.rs`, `crates/oxilean-doc/src/multifile.rs`.
  - **Tests:** output contains light+dark variables + toggle element; relative links resolve.
- [x] DocIR integration with oxilean-codegen (2026-05-30)
  - **Goal:** Bridge `oxilean_codegen::DocIR` into `oxilean-doc`'s `DocItem` structure so renderers can accept codegen-produced IRs without re-parsing source.
  - **Files:** `crates/oxilean-doc/src/extractor.rs` — added `doc_items_from_ir(ir: &DocIR) -> Vec<DocItem>`; `crates/oxilean-doc/Cargo.toml` — added `oxilean-codegen` dependency.
  - **Tests:** 4 new tests: empty IR, "def" kind mapping, "axiom" kind mapping, deprecated flag preservation.
- [x] Markdown rendering in doc comments (pure-Rust mini-renderer: bold, italic, inline code, fenced code blocks, links)
  - `crates/oxilean-doc/src/markdown.rs` — `render_markdown()` and `html_escape()`
  - Wired into `multifile.rs`'s `render_module_item` — doc comments now pass through `render_markdown`
  - `html_escape` from `markdown` module replaces the renderer-local `escape_html` in multifile output
  - 30 tests in `markdown::tests` cover all inline/block elements and edge cases
- [x] `#[deprecated]` / `@[deprecated]` attribute surfacing (implemented 2026-05-29)
  - **Goal:** Surface deprecated declarations in generated docs. The extractor currently treats `Decl::Attribute { decl, .. }` as a transparent wrapper and discards the attribute (`extractor.rs`); instead inspect it for a `deprecated` annotation, add `deprecated: bool` (+ optional message) to `DocItem`, and render a badge / `<del>`-styled entry in both renderers.
  - **Files:** `crates/oxilean-doc/src/extractor.rs`, `crates/oxilean-doc/src/renderer.rs`, `crates/oxilean-doc/src/multifile.rs`.
  - **Tests:** fixture `@[deprecated] def foo` → `DocItem.deprecated == true` + badge rendered; non-deprecated unaffected.
  - **Risk:** verify the `AnnotationKind::Deprecated` AST shape before threading; no cross-crate change.
