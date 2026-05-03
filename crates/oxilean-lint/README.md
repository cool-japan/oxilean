# oxilean-lint

[![Crates.io](https://img.shields.io/crates/v/oxilean-lint.svg)](https://crates.io/crates/oxilean-lint)
[![Docs.rs](https://docs.rs/oxilean-lint/badge.svg)](https://docs.rs/oxilean-lint)

> **Static Analysis and Lint Rules for OxiLean**

`oxilean-lint` provides a pluggable lint engine and a comprehensive set of built-in lint rules for analyzing OxiLean source code. It catches potential issues -- dead code, unused hypotheses, style violations, deprecated API usage, missing documentation -- early in the development cycle, before elaboration and type checking.

Because the linter operates on the surface AST produced by `oxilean-parse`, lint diagnostics are advisory: they can never affect kernel soundness. The engine supports auto-fixes, IDE integration, and a plugin system for user-defined rules.

17,600 SLOC -- fully implemented lint engine with built-in rules (121 source files, 685 tests passing).

Part of the [OxiLean](https://github.com/cool-japan/oxilean) project -- a Lean-compatible theorem prover implemented in pure Rust.

## Overview

### Module Reference

| Module | Description |
|--------|-------------|
| `framework` | Core engine: `LintEngine`, `LintRegistry`, `LintRule`, `LintContext`, `LintDiagnostic` |
| `rules` | Built-in rule implementations |
| `autofix` | Automatic source-code fix generation |
| `plugin` | Plugin API for user-defined lint rules |
| `ide_integration` | LSP-compatible diagnostic output |

### Built-in Lint Rules

| Rule | Category | Description |
|------|----------|-------------|
| `DeadCodeRule` | Redundancy | Detects unreachable definitions |
| `UnreachableCodeRule` | Redundancy | Detects code after unconditional returns |
| `UnusedImportRule` | Redundancy | Flags imports that are never referenced |
| `UnusedVariableRule` | Redundancy | Flags variables that are bound but never used |
| `UnusedHypothesisRule` | Redundancy | Flags hypotheses in proof contexts not used |
| `RedundantAssumptionRule` | Redundancy | Detects duplicate or trivially true hypotheses |
| `RedundantPatternRule` | Redundancy | Detects unreachable match arms |
| `SimplifiableExprRule` | Complexity | Suggests simpler equivalents for complex expressions |
| `LongProofRule` | Complexity | Warns when a proof exceeds a configurable line threshold |
| `NamingConventionRule` | Naming | Enforces `camelCase` / `snake_case` naming conventions |
| `StyleRule` | Style | General formatting and style guidelines |
| `MissingDocRule` | Documentation | Flags public declarations without doc comments |
| `MissingDocstringRule` | Documentation | Flags theorem statements without module docstrings |
| `DeprecatedApiRule` | Deprecation | Warns on use of deprecated definitions |
| `DeprecatedTacticRule` | Deprecation | Warns on deprecated or superseded tactics |

### Lint Categories

```rust
pub enum LintCategory {
    Correctness,    // potential bugs
    Style,          // cosmetic / formatting issues
    Performance,    // potential inefficiencies
    Complexity,     // overly complex code
    Deprecation,    // use of deprecated APIs
    Documentation,  // missing or incorrect docs
    Naming,         // naming convention violations
    Redundancy,     // redundant or dead code
}
```

### Severity Levels

Each diagnostic carries a `Severity`:

```rust
pub enum Severity {
    Error,    // must be fixed (fails the build)
    Warning,  // should be addressed
    Info,     // informational hint
    Hint,     // IDE-level suggestion
}
```

### Lint Passes

Rules are grouped into *passes* that run together:

```rust
use oxilean_lint::{LintPass, LintEngine};

let pass = LintPass::new("redundancy")
    .with_lint("dead-code")
    .with_lint("unused-import")
    .with_fixes();
```

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
oxilean-lint = "0.1.2"
```

### Running the Engine

```rust,ignore
use oxilean_lint::{LintEngine, LintConfig, LintRegistry};

let mut registry = LintRegistry::default();
registry.register_builtins();

let engine = LintEngine::new(registry, LintConfig::default());
let diagnostics = engine.lint(&parsed_module)?;

for diag in &diagnostics {
    eprintln!("[{}] {}", diag.severity, diag.message);
}
```

## Dependencies

- `oxilean-kernel` -- core expression types
- `oxilean-parse` -- surface AST types for rule traversal

## Testing

```bash
cargo test -p oxilean-lint
cargo test -p oxilean-lint -- --nocapture
```

## License

Copyright COOLJAPAN OU (Team Kitasan). Apache-2.0 -- See [LICENSE](../../LICENSE) for details.
