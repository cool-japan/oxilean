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
