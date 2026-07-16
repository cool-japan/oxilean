# oxilean-lint — TODO

> Task list for the linting system crate.
> Last updated: 2026-05-03

## ✅ Completed

**Status**: COMPLETE — ~17,600 SLOC implemented across 121 source files

### Linting Features
- [x] Code quality checks
- [x] Style enforcement
- [x] Best practices validation
- [x] Naming convention checks
- [x] Complexity analysis
- [x] Unused code detection
- [x] Dead code elimination warnings

### Lint Categories
- [x] Style lints (formatting, naming)
- [x] Correctness lints (type errors, logic errors)
- [x] Performance lints (inefficient patterns)
- [x] Security lints (unsafe patterns)
- [x] Maintainability lints (complexity)

### Lint Configuration
- [x] Enable/disable specific lints
- [x] Severity levels (error, warning, info)
- [x] Per-file configuration
- [x] Lint suppression annotations

---

## 🐛 Known Issues

None reported. All tests passing.

---

## ✅ Completed: Extended Lint Features

- [x] Additional lint rules — `rules.rs` (extended with complexity, style, security lints)
- [x] Custom lint plugin system — `plugin.rs` (LintPlugin trait, PluginRegistry, BuiltinPlugin, 4 tests)
- [x] Auto-fix suggestions
- [x] IDE integration for real-time linting

## v0.1.3 LSP Code Action Integration

> Last updated: 2026-05-29

### Ring 1

- [x] Wire oxilean-lint diagnostics into LSP `textDocument/codeAction` responses
  - **Goal:** When the LSP serves `codeAction` requests, include auto-fix suggestions from oxilean-lint lints that have auto-fix implementations
  - **Design:** Add `pub fn lint_to_code_action(diag: &LintDiagnostic) -> Option<CodeAction>` bridge function; wire into `crates/oxilean-cli/src/lsp/code_actions/` handler; actions should have `edit: WorkspaceEdit` computed from the lint's fix suggestion
  - **Files:** `src/` (add bridge fn), `crates/oxilean-cli/src/lsp/code_actions/` (wire in)
  - **Prerequisites:** LSP integration test (oxilean-cli Ring 0) — verifies the wiring works
  - **Tests:** Golden code action response for a lint with known auto-fix
  - **Risk:** LSP code action protocol requires specific `kind` strings — follow LSP 3.17 spec
  - **Note (2026-05-29):** Implementation tracked from oxilean-cli side. This cycle, add `fix_suggestion: Option<LintFixSuggestion>` to `LintDiagnostic` (or equivalent) if not already present — oxilean-cli needs this to convert lint diags to CodeAction text edits. See oxilean-cli/TODO.md for the full plan.
