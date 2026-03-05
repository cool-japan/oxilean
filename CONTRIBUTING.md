# Contributing to OxiLean

Thank you for your interest in contributing to OxiLean! This document provides guidelines for contributing to our proof assistant (Lean4-like kernel) implemented in Rust. The project consists of 11 crates with over 1.2 million lines of Rust code across 5,380 files.

---

## Getting Started

### Prerequisites

- **Rust 1.70+** (stable toolchain)
- No external dependencies required for the kernel

### Setup

```bash
git clone https://github.com/cool-japan/oxilean
cd oxilean
cargo build --workspace
cargo test --workspace
```

---

## Development Workflow

### 1. Branch Strategy

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-description
```

### 2. Build & Test

```bash
# Build all crates
cargo build --workspace

# Run all tests
cargo test --workspace

# Run tests for a specific crate
cargo test -p oxilean-kernel

# Run with verbose output
cargo test --workspace -- --nocapture
```

### 3. Code Quality

Before submitting a PR, ensure:

```bash
# Format code
cargo fmt --all

# Lint (zero warnings policy)
cargo clippy --workspace -- -D warnings

# Build documentation
cargo doc --workspace --no-deps
```

---

## Code Style Guidelines

### General Rules

- **`cargo fmt`** — mandatory before every commit
- **`cargo clippy`** — zero warnings policy
- **No `unsafe`** in the kernel (`#![forbid(unsafe_code)]`)
- All public functions must have `///` doc comments
- All kernel functions return `Result<T, KernelError>` (no panics)

### Naming Conventions

| Item | Convention | Example |
|------|-----------|---------|
| Types/Structs | `PascalCase` | `TypeChecker`, `ExprKind` |
| Functions/Methods | `snake_case` | `infer_type`, `is_def_eq` |
| Constants | `SCREAMING_SNAKE` | `MAX_UNIVERSE_LEVEL` |
| Type parameters | Single uppercase | `T`, `E` |
| Lifetimes | Short lowercase | `'a`, `'env` |

### Commit Messages

Follow the format: `<scope>: <description>`

```
kernel: implement beta reduction in WHNF
parse: add UTF-8 identifier support to lexer
elab: fix implicit argument insertion for Pi types
cli: add --verbose flag to check command
test: add integration tests for Nat inductive
docs: update BLUEPRINT with quotient types section
```

### File Organization

- One module per logical concept
- Unit tests in the same file (`#[cfg(test)] mod tests`)
- Integration tests in the `tests/` directory

---

## Architecture Rules

### Trust Boundary

**Only `oxilean-kernel` is inside the trust boundary.** When contributing:

1. **Kernel changes** require extra scrutiny — any bug here could cause unsoundness
2. **Parser/Elaborator changes** are less critical — bugs cause proof failure, not unsoundness
3. **Keep the kernel minimal** — if a feature can be implemented outside the kernel, it should be

### Dependency Policy

| Crate | External Dependencies Allowed |
|-------|------------------------------|
| `oxilean-kernel` | **NONE** (zero external deps) |
| `oxilean-meta` | Minimal (metaprogramming primitives) |
| `oxilean-parse` | Minimal (prefer hand-written) |
| `oxilean-elab` | Reasonable (no heavy frameworks) |
| `oxilean-cli` | As needed for CLI ergonomics |
| `oxilean-std` | Minimal (standard library definitions) |
| `oxilean-codegen` | Reasonable (code generation backends) |
| `oxilean-runtime` | Reasonable (runtime evaluation support) |
| `oxilean-build` | As needed for build system integration |
| `oxilean-lint` | Minimal (lint rules and analysis) |
| `oxilean-wasm` | As needed for WASM compilation targets |

### Testing Requirements

- Every new function must have at least one unit test
- Kernel functions need both positive and negative tests
- Integration tests for end-to-end scenarios

---

## Areas for Contribution

### Good First Issues

- Writing additional unit tests for undercovered modules
- Improving error messages across all 11 crates
- Documentation improvements (rustdoc comments, examples)
- Adding `Display` implementations where missing
- Fixing clippy lints and code style issues

### Intermediate

- Performance optimization of existing algorithms (profiling-driven)
- Expanding the standard library (`oxilean-std`) with new definitions and lemmas
- Adding new tactics to the elaborator
- Improving WASM compilation output (`oxilean-wasm`)
- Enhancing lint rules and diagnostics (`oxilean-lint`)
- Improving code generation backends (`oxilean-codegen`)
- Build system improvements (`oxilean-build`)

### Advanced

- Runtime evaluation optimizations (`oxilean-runtime`)
- Metaprogramming framework extensions (`oxilean-meta`)
- Advanced tactic implementations (omega, simp, ring, etc.)
- Parallel type checking and elaboration
- Incremental compilation and caching strategies
- Cross-crate integration testing and end-to-end proof verification

---

## Reporting Issues

Please include:

1. **Description** of the issue
2. **Steps to reproduce** (if applicable)
3. **Expected behavior** vs **actual behavior**
4. **Rust version** (`rustc --version`)
5. **OS and platform**

---

## NO WARNINGS POLICY

OxiLean strictly enforces a **zero-warnings** policy. This is critical for maintaining code quality and catching potential bugs early.

### Policy Overview

1. **All Rust warnings must be fixed** before merging
2. **All rustdoc warnings must be fixed** (missing docs, broken links)
3. **All clippy warnings must be fixed** (style and potential bugs)
4. **No allowlists** (no `#[allow(...)]`) except in special cases (see below)

### Fixing Common Warnings

#### Missing documentation

Every public item needs `/// ` rustdoc comments:

```rust
// BAD
pub fn add(x: i32, y: i32) -> i32 { x + y }

// GOOD
/// Add two numbers.
///
/// # Examples
///
/// ```
/// # use oxilean::*;
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(x: i32, y: i32) -> i32 { x + y }
```

#### Dead code

For intentionally unused items:

```rust
// If it's intentional (e.g., for future use):
#[allow(dead_code)]
pub fn future_function() { }

// If it's a test helper:
#[cfg(test)]
mod tests {
    #![allow(dead_code)]  // Test helpers often unused

    fn helper() { }
}
```

#### Clippy warnings: Too many arguments

For functions with many parameters, use a struct:

```rust
// BAD (clippy complains)
fn process(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 { }

// GOOD
struct ProcessArgs {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    e: i32,
}

fn process(args: ProcessArgs) -> i32 { }

// OR use builder pattern
```

If unavoidable, suppress with:
```rust
#[allow(clippy::too_many_arguments)]
fn process(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 { }
```

#### Clippy warnings: Type complexity

Use type aliases:

```rust
// BAD
fn process(x: Result<Option<Vec<Box<dyn Trait>>>, Error>) { }

// GOOD
type ComplexType = Result<Option<Vec<Box<dyn Trait>>>, Error>;
fn process(x: ComplexType) { }
```

### Testing the NO WARNINGS Policy

Before every commit:

```bash
# Build and capture ALL output
cargo build --workspace 2>&1 | tee build.log

# Check for warnings
if grep -i warning build.log; then
    echo "FAILED: Warnings found"
    exit 1
fi

# Run clippy with warnings-as-errors
cargo clippy --workspace -- -D warnings

# Check tests
cargo test --workspace 2>&1 | tee test.log
if grep -i warning test.log; then
    echo "FAILED: Test warnings found"
    exit 1
fi

# Check doc tests
cargo test --doc --all 2>&1 | tee doc.log
if grep -i warning doc.log; then
    echo "FAILED: Doc test warnings found"
    exit 1
fi
```

### Special Cases for Allowlists

Only these patterns are acceptable:

1. **`#[allow(clippy::too_many_arguments)]`** — Complex functions that can't be refactored
2. **`#[allow(dead_code)]`** — Public API for future use, test helpers
3. **`#[allow(rustdoc::missing_docs)]`** — Private implementation details
4. **`#[allow(unsafe_code)]`** in `unsafe { }` blocks — Only in `unsafe` functions
5. **`#![allow(...)]` at module level** in `#[cfg(test)]` — For test utilities

All allowlists must have a comment explaining **why**:

```rust
// Only this tactic is rarely used; keeping for completeness
#[allow(dead_code)]
pub fn tactic_unused() { }

// WRONG: No explanation
#[allow(dead_code)]
pub fn foo() { }
```

---

## Testing Requirements

### Unit Tests

Every public function should have unit tests:

```rust
/// Add two numbers.
pub fn add(x: i32, y: i32) -> i32 { x + y }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_positive() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_add_zero() {
        assert_eq!(add(5, 0), 5);
    }
}
```

### Kernel Testing (Extra Rigor)

Kernel functions need **positive** and **negative** tests:

```rust
#[test]
fn test_type_check_valid() {
    // ✓ Valid proof should pass
    assert!(type_check_expr(valid_expr).is_ok());
}

#[test]
fn test_type_check_invalid() {
    // ✗ Invalid proof should fail
    assert!(type_check_expr(invalid_expr).is_err());
}
```

### Integration Tests

For end-to-end scenarios:

```bash
tests/
  ├─ integration_check.rs     # File checking
  ├─ integration_repl.rs      # REPL session
  └─ fixtures/
     ├─ valid_proof.oxilean
     └─ invalid_proof.oxilean
```

### Property-Based Testing

For mathematical properties:

```rust
use quickcheck::{quickcheck, TestResult};

#[test]
fn prop_add_comm() {
    fn prop(x: i32, y: i32) -> bool {
        add(x, y) == add(y, x)
    }
    quickcheck(prop as fn(i32, i32) -> bool);
}
```

### Test Coverage

Target: **80%+ coverage** for kernel, **60%+** for other crates

```bash
# Generate coverage report
cargo tarpaulin --out Html --workspace
```

---

## Kernel Validation Checklist

Before submitting a PR that modifies `oxilean-kernel`:

### Safety Checks

- [ ] **No `unsafe` blocks** (kernel forbids `unsafe_code`)
- [ ] **No external dependencies** (kernel must be self-contained)
- [ ] **All public functions documented** (100% doc coverage)
- [ ] **All error paths handled** (no unwrap/expect in production)

### Soundness Checks

- [ ] **Type inference changes**: Tested with at least 5 proof types
- [ ] **Definitional equality changes**: Verified with symmetric/transitive properties
- [ ] **Reduction rules**: No infinite loops (test with recursive definitions)
- [ ] **Inductive types**: Positivity checking still works
- [ ] **Universe constraints**: No cycles introduced

### Testing Checklist

- [ ] **Unit tests**: All new functions have tests
- [ ] **Kernel tests**: Both positive and negative cases
- [ ] **Integration**: End-to-end tests with elaborator
- [ ] **Regression**: No existing tests fail
- [ ] **Performance**: No unexpected slowdowns

### Code Review Checklist

- [ ] **Logic correctness**: Algorithm/proof logic is sound
- [ ] **Edge cases**: Handles empty/single-element cases
- [ ] **Error messages**: Clear and helpful
- [ ] **Variable names**: Meaningful (not `x`, `y`, `z` unless unavoidable)
- [ ] **Code size**: Reasonably sized functions (<100 SLOC per function)

### Documentation Checklist

- [ ] **Rustdoc comments**: All public items documented
- [ ] **Examples**: Complex functions have usage examples
- [ ] **Invariants**: Safety/soundness guarantees documented
- [ ] **Related modules**: Links to related code documented

---

## Debugging Techniques

### Debug Tracing

Enable tracing for execution flow:

```bash
OXILEAN_TRACE=1 cargo run -- check file.oxilean
```

Use the `trace` module:

```rust
use oxilean_kernel::trace::Tracer;

let mut tracer = Tracer::new(TraceLevel::Debug);
tracer.event("Starting type inference");
tracer.event(&format!("Checking expr: {:?}", expr));
```

### Memory Profiling

Check for memory leaks or excessive allocation:

```bash
# Valgrind (Linux)
valgrind --leak-check=full cargo run -- check file.oxilean

# Instruments (macOS)
cargo instruments -t "System Trace"
```

### Performance Analysis

Identify hot spots:

```bash
# CPU profiling with flamegraph
cargo install flamegraph
cargo flamegraph -- check file.oxilean
```

### Test Isolation

Run a single test for faster iteration:

```bash
# Run specific test
cargo test test_name -- --nocapture

# Run tests in kernel only
cargo test -p oxilean-kernel -- --nocapture

# Run with all output
cargo test --all -- --nocapture --test-threads=1
```

---

## Performance Optimization Guidelines

### When to Optimize

1. **Profiling first**: Measure before and after
2. **Significant impact**: Aim for >10% improvement
3. **No soundness trade-offs**: Never sacrifice correctness for speed
4. **No binary bloat**: Avoid code duplication for marginal gains

### Common Optimizations in OxiLean

#### 1. Memoization

```rust
// Store results of expensive operations
use std::collections::HashMap;

struct WithMemo {
    cache: HashMap<Expr, Expr>,
}

fn whnf_memoized(&mut self, e: &Expr) -> Result<Expr> {
    if let Some(cached) = self.cache.get(e) {
        return Ok(cached.clone());
    }
    let result = whnf(e)?;
    self.cache.insert(e.clone(), result.clone());
    Ok(result)
}
```

#### 2. Early Exit

```rust
// Stop as soon as answer found
fn find_instance(ctx: &Context, goal: &Expr) -> Result<Expr> {
    for instance in &ctx.instances {
        if is_unifiable(goal, &instance.ty)? {
            return Ok(instance.term.clone());  // Exit early
        }
    }
    Err(NotFound)
}
```

#### 3. Lazy Evaluation

```rust
// Defer expensive computation
fn whnf_lazy(e: &Expr) -> Lazy<Expr> {
    Lazy::new(move || {
        // Expensive computation only when needed
        compute_whnf(e)
    })
}

// Use: let result = whnf_lazy(e).force();
```

#### 4. Specialization

```rust
// Generate specialized code for common cases
fn unify_special_case(e1: &Expr, e2: &Expr) -> Result<bool> {
    // Fast path for obviously equal expressions
    if e1.as_ptr() == e2.as_ptr() {
        return Ok(true);  // Same object
    }

    // Fast path for literals
    match (e1, e2) {
        (Expr::Lit(l1), Expr::Lit(l2)) => return Ok(l1 == l2),
        _ => {}
    }

    // Slow path for complex cases
    full_unify(e1, e2)
}
```

---

## Release Process & Versioning

### Versioning Scheme

OxiLean uses **semantic versioning**: `MAJOR.MINOR.PATCH`

- **MAJOR** (0.x.y): Breaking changes or major features
- **MINOR** (x.1.y): New features, backward compatible
- **PATCH** (x.y.1): Bug fixes, no new features

### Release Checklist

1. **Update version** in all `Cargo.toml` files
2. **Update CHANGELOG.md** with notable changes
3. **Tag release**: `git tag v0.x.y`
4. **Build artifacts**: `cargo build --release --all`
5. **Run full test suite**: `cargo test --all --release`
6. **Check warnings**: Ensure zero warnings
7. **Push tag**: `git push origin v0.x.y`
8. **Create GitHub release** with notes

### Changelog Format

```markdown
## [0.2.0] — 2026-06-15

### Added
- New `omega` tactic for linear arithmetic
- Support for quotient types
- LSP hover information

### Changed
- Refactored unification algorithm (10% faster)
- Updated documentation

### Fixed
- Fixed universe level calculation bug
- Fixed termination checker false positives

### Deprecated
- Old `simp` configuration (use new API)

### Removed
- Legacy AST format (use new parser)
```

---

## Reporting Issues

Please include:

1. **Description** of the issue
2. **Steps to reproduce** (if applicable)
3. **Expected behavior** vs **actual behavior**
4. **Rust version** (`rustc --version`)
5. **OS and platform**
6. **Minimal reproducible example** (if possible)

### Issue Template

```markdown
### Description
[Clear description of issue]

### Steps to Reproduce
1. First step
2. Second step
3. Expected behavior
4. Actual behavior

### Environment
- Rust version: `rustc 1.xx.x`
- OS: Linux/macOS/Windows
- OxiLean version: 0.x.y (or commit hash)

### Minimal Example
[Minimal code that reproduces the issue]

### Related Issues
[Link to related issues if any]
```

---

## Code Quality Metrics

OxiLean targets:

| Metric | Target |
|--------|--------|
| Test coverage (kernel) | ≥85% |
| Test coverage (other) | ≥70% |
| Documentation coverage | 100% for public API |
| Compilation warnings | 0 |
| Clippy warnings | 0 |
| Security vulnerabilities | 0 |
| Performance regression | <5% per release |

---

## License

By contributing, you agree that your contributions will be licensed under **Apache-2.0**.
