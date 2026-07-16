# Audit: lean4export Reader Status

**Date:** 2026-07-12
**Repo:** oxilean (branch 0.1.3)
**Auditor task:** Confirm absence, map what building it requires.

---

## 1. Confirmed Absence of lean4export Parser

### Grep for lean4export line-command strings

```
grep -rn "#QUOT|#IND|#DEF|#AX|#ES|#EA|#EL|#NS|#NI|#US|#UM|#UIM|#UP" . --include="*.rs"
```
**Result: zero hits.** No file in the workspace contains these command prefixes.

### Grep for "lean4export" and related terms

```
grep -rn "lean4export|lean4lean|trepplein|export_reader|export_format" --include="*.rs"
```
**Hits:** Only one hit in the CLI crate was for `test_export_format_from_extension` — a test about file extension detection, not the lean4export text format.

Additional grep for `lean4export` across all file types:
```
grep -rn "lean4export|lean4lean|trepplein" --include="*.rs" --include="*.toml" --include="*.md"
```
**Result:** zero hits for `lean4export`, `lean4lean`, `trepplein`.

### Verification that the kernel's src/export/ is unrelated

The kernel has `crates/oxilean-kernel/src/export/` — but this is **OxiLean's own binary serialization format** (OXLN), not the lean4export text format:

- File: `crates/oxilean-kernel/src/export/functions.rs:54`
- `pub const MAGIC_NUMBER: u32 = 0x4F584C4E;` — that decodes to ASCII "OXLN"
- The serialize/deserialize functions use custom binary encoding (little-endian u8/u32/u64 tags), NOT the text-based lean4export format.
- The exported types (`ExportedModule`, `ModuleCache`) are OxiLean's internal inter-crate module format.

**Conclusion: ZERO lean4export parsing exists anywhere in the repository.**

---

## 2. lean4export Format Documentation (from knowledge; uncertain parts flagged)

The lean4export format is a line-oriented text format used by Lean 4's `lean4export` tool and consumed by checkers like lean4lean and trepplein. Each line represents one entity in the environment. Entities are referenced by integer indices assigned in order of appearance.

### Format Version Header (UNCERTAIN)

- Line 1 is typically a version line like `export_format_version:1` or just an integer.
- **UNCERTAIN:** The exact syntax of the version header. Another agent fetching the authoritative spec should clarify. Some implementations also accept no header (implicit version 1).

### Name Lines (NS / NI)

Names are hierarchical identifiers built up component-by-component:

```
NS <idx> <parent-idx> <string-component>
NI <idx> <parent-idx> <nat-component>
```

- `NS <i> <p> <s>`: name `i` = parent name `p` with string suffix `s`
- `NI <i> <p> <n>`: name `i` = parent name `p` with numeric suffix `n`
- Index 0 is implicitly the anonymous (root) name `""`.

### Universe Level Lines (US / UM / UIM / UP)

```
US <idx> <inner-idx>          -- Succ: u+1
UM <idx> <left-idx> <right-idx>  -- Max
UIM <idx> <left-idx> <right-idx> -- IMax
UP <idx> <name-idx>           -- Param(name)
```

- Index 0 is implicitly Level::Zero.
- No explicit line for Zero.

### Expression Lines (EV / ES / EC / EA / EL / EP / EZ / EJ / ELN / ELS)

```
EV <idx> <de-bruijn-index>           -- BVar (bound variable)
ES <idx> <level-idx>                 -- Sort (universe)
EC <idx> <name-idx> [<level-idx>...] -- Const (global constant + universe args)
EA <idx> <func-idx> <arg-idx>        -- App (function application)
EL <idx> <binder-info> <name-idx> <type-idx> <body-idx>  -- Lam
EP <idx> <binder-info> <name-idx> <type-idx> <body-idx>  -- Pi
EZ <idx> <name-idx> <type-idx> <val-idx> <body-idx>      -- Let
EJ <idx> <major-name-idx> <proj-idx> <struct-idx>        -- Proj (projection)
ELN <idx> <nat-literal-value>        -- Nat literal (arbitrary precision)
ELS <idx> <string-literal>           -- String literal
```

**Binder info values (UNCERTAIN: exact integer encoding):**
- 0 = Default (explicit)
- 1 = Implicit `{}`
- 2 = StrictImplicit `⦃⦄`
- 3 = InstImplicit `[]`

**ELN nat literal format (UNCERTAIN):** Could be decimal, hex, or a big-endian hex blob for large naturals. For numbers fitting in u64 it's decimal. For bignum, the format is uncertain — needs spec.

**UNCERTAIN:** Whether `EV` or a different code is used for bound variables. Some implementations use `#EV` prefix; others just `EV` (no hash). The `#` prefix may only apply to declaration lines. Need to check lean4export source.

### Declaration Lines

**UNCERTAIN:** Whether declarations use `#` prefix (like `#DEF`) or no prefix.

Based on lean4lean/trepplein analysis:

```
#DEF <name-idx> <num-level-params> <type-idx> <value-idx> [<hint>]
     -- Definition; hint = "O" (opaque), "A" (abbrev), or integer height
#AX <name-idx> <num-level-params> <type-idx>
     -- Axiom (no body)
#OPAQ <name-idx> <num-level-params> <type-idx> <value-idx>
     -- Opaque definition (UNCERTAIN: may just be #DEF with hint=O)
#THM <name-idx> <num-level-params> <type-idx> <value-idx>
     -- Theorem (UNCERTAIN: may be same as #DEF)
#IND <name-idx> <num-level-params> <num-params> <num-indices> <num-ctors> [<ctor-name-idx> <ctor-type-idx>...] <type-idx>
     -- Inductive type (exact ordering of fields UNCERTAIN)
#QUOT
     -- Declares the quotient type block (Quot, Quot.mk, Quot.lift, Quot.ind all at once)
```

**HIGHLY UNCERTAIN:** The exact field ordering and counts in `#IND`, `#CTOR`, `#REC` lines. lean4export may emit inductive types as: one `#IND` line for the type + one `#CTOR` line per constructor + one `#REC` line for the recursor. Or it may bundle them. The authoritative source is `lean4export/Lean/Export.lean` in the Lean 4 repo.

**UNCERTAIN:** Whether constructors and recursors get their own lines (`#CTOR`, `#REC`) or are embedded in `#IND`.

**Level params:** These are declared by `<num-level-params>` and then named by consecutive name indices starting from a known base. **UNCERTAIN:** How exactly universe param names are threaded — may require reading a specific range of name indices.

---

## 3. oxilean-kernel Public API for Building Terms

The reader must construct the kernel's data types. Here are the relevant types and their public constructors, all from `crates/oxilean-kernel/src/`:

### Name (`src/name/types.rs:711`)

```rust
pub enum Name {
    Anonymous,
    Str(Box<Name>, String),
    Num(Box<Name>, u64),
}
impl Name {
    pub fn str(s: impl Into<String>) -> Self;           // Anonymous.s
    pub fn from_str(s: &str) -> Self;                   // dot-split
    pub fn append_str(self, s: impl Into<String>) -> Self;
    pub fn append_num(self, n: u64) -> Self;
}
```

The lean4export reader needs a `Vec<Name>` indexed table. NS/NI lines map directly:
- NS i p s → `name_table[i] = name_table[p].clone().append_str(s)`
- NI i p n → `name_table[i] = name_table[p].clone().append_num(n)`

### Level (`src/level/types.rs:1085`)

```rust
pub enum Level {
    Zero,
    Succ(Box<Level>),
    Max(Box<Level>, Box<Level>),
    IMax(Box<Level>, Box<Level>),
    Param(Name),
    MVar(LevelMVarId),  // not used in export reader — metavariables don't appear
}
impl Level {
    pub fn zero() -> Self;
    pub fn succ(l: Level) -> Self;
    pub fn max(l1: Level, l2: Level) -> Self;
    pub fn imax(l1: Level, l2: Level) -> Self;
    pub fn param(name: Name) -> Self;
}
```

The reader maintains a `Vec<Level>` table. Level index 0 = `Level::Zero`.

### Literal (`src/expr/types.rs:833`)

```rust
pub enum Literal {
    Nat(u64),   // WARNING: only fits u64 — bignum is missing!
    Int(i64),
    Str(String),
}
```

**CRITICAL GAP:** `Literal::Nat(u64)` can only hold naturals up to 2^64-1. The lean4export format supports arbitrary-precision naturals (ELN). Lean's standard library has many large nat literals (e.g. in cryptography). The kernel's `Literal` type is insufficient for full lean4export compatibility. The brief calls out a hand-written zero-dependency bignum as a requirement for section 4 part (3).

### Expr (`src/expr/types.rs:998`)

```rust
pub enum Expr {
    Sort(Level),
    BVar(u32),
    FVar(FVarId),
    Const(Name, Vec<Level>),
    App(Box<Expr>, Box<Expr>),
    Lam(BinderInfo, Name, Box<Expr>, Box<Expr>),
    Pi(BinderInfo, Name, Box<Expr>, Box<Expr>),
    Let(Name, Box<Expr>, Box<Expr>, Box<Expr>),   // name, type, val, body
    Lit(Literal),
    Proj(Name, u32, Box<Expr>),
}
```

All constructors are public. The reader can build `Expr` values directly. The reader maintains a `Vec<Expr>` indexed table.

**Note:** `FVar` is used only internally by the type checker. It should NOT appear in export files. The reader should reject any `EV`-like line that produces an FVar.

**Missing variant:** lean4export `EV` = bound variable = `Expr::BVar(u32)`. Mapping is direct.

### BinderInfo (`src/expr/types.rs:423`)

```rust
pub enum BinderInfo {
    Default,
    Implicit,
    StrictImplicit,
    InstImplicit,
}
```

### Declaration types

The reader needs to build `ConstantInfo` variants (not `Declaration`, which is the legacy format). `ConstantInfo` is richer and handles inductives, constructors, recursors, and quotients:

```rust
// src/declaration/types.rs
pub enum ConstantInfo {
    Axiom(AxiomVal),
    Definition(DefinitionVal),
    Theorem(TheoremVal),
    Opaque(OpaqueVal),
    Inductive(InductiveVal),
    Constructor(ConstructorVal),
    Recursor(RecursorVal),
    Quotient(QuotVal),
}

pub struct ConstantVal {         // common base
    pub name: Name,
    pub level_params: Vec<Name>,
    pub ty: Expr,
}

pub struct AxiomVal { pub common: ConstantVal, pub is_unsafe: bool }

pub struct DefinitionVal {
    pub common: ConstantVal,
    pub value: Expr,
    pub hints: ReducibilityHint,  // Opaque | Abbrev | Regular(u32)
    pub safety: DefinitionSafety, // Safe | Unsafe | Partial
    pub all: Vec<Name>,           // mutual group
}

pub struct TheoremVal {
    pub common: ConstantVal,
    pub value: Expr,
    pub all: Vec<Name>,
}

pub struct OpaqueVal {
    pub common: ConstantVal,
    pub value: Expr,
    pub is_unsafe: bool,
    pub all: Vec<Name>,
}

pub struct InductiveVal {
    pub common: ConstantVal,
    pub num_params: u32,
    pub num_indices: u32,
    pub all: Vec<Name>,           // mutual group of inductives
    pub ctors: Vec<Name>,
    pub num_nested: u32,
    pub is_rec: bool,
    pub is_unsafe: bool,
    pub is_reflexive: bool,
    pub is_prop: bool,
}

pub struct ConstructorVal {
    pub common: ConstantVal,
    pub induct: Name,
    pub cidx: u32,
    pub num_params: u32,
    pub num_fields: u32,
    pub is_unsafe: bool,
}

pub struct RecursorVal {
    pub common: ConstantVal,
    pub all: Vec<Name>,
    pub num_params: u32,
    pub num_indices: u32,
    pub num_motives: u32,
    pub num_minors: u32,
    pub rules: Vec<RecursorRule>,
    pub k: bool,
    pub is_unsafe: bool,
}

pub struct RecursorRule {
    pub ctor: Name,
    pub nfields: u32,
    pub rhs: Expr,
}

pub struct QuotVal {
    pub common: ConstantVal,
    pub kind: QuotKind,  // Type | Mk | Lift | Ind
}

pub enum ReducibilityHint { Opaque, Abbrev, Regular(u32) }
pub enum DefinitionSafety  { Safe, Unsafe, Partial }
```

### Environment API (`src/env/types.rs:222`)

```rust
pub struct Environment { ... }
impl Environment {
    pub fn new() -> Self;
    pub fn add(&mut self, decl: Declaration) -> Result<(), EnvError>;
    pub fn add_constant(&mut self, ci: ConstantInfo) -> Result<(), EnvError>;
    pub fn find(&self, name: &Name) -> Option<&ConstantInfo>;
    pub fn contains(&self, name: &Name) -> bool;
    pub fn constant_infos(&self) -> impl Iterator<Item = (&Name, &ConstantInfo)>;
    // ... more query methods
}
```

**The reader should use `add_constant` (for ConstantInfo), not `add` (legacy Declaration).**

### Kernel check API (`src/check/functions.rs`)

```rust
// Check and add one ConstantInfo:
pub fn check_constant_info(env: &mut Environment, ci: ConstantInfo) -> Result<(), KernelError>;

// Check multiple in sequence:
pub fn check_constant_infos(env: &mut Environment, cis: Vec<ConstantInfo>) -> Result<(), KernelError>;
```

`KernelError` is in `src/error/` — public type, implementers should inspect it.

---

## 4. Recommended Crate Design

### Workspace additions

Add two new crates to `Cargo.toml` workspace members:
```
"crates/oxilean-export"
"crates/oxilean-verify"
```

Also add to `[workspace.dependencies]`:
```toml
oxilean-export = { path = "crates/oxilean-export", version = "0.1.3" }
oxilean-verify = { path = "crates/oxilean-verify", version = "0.1.3" }
```

### Crate: `crates/oxilean-export`

**Purpose:** Parse the lean4export text format into an `ExportFile` struct; zero external deps; fuzzable.

**`Cargo.toml`:**
```toml
[package]
name = "oxilean-export"
# ...workspace package fields...

[dependencies]
# ZERO external dependencies — this is part of the TCB

[dev-dependencies]
proptest.workspace = true

[[test]]
name = "fuzz_like_tests"
path = "tests/fuzz_like_tests.rs"
```

**`src/lib.rs` attributes:**
```rust
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
```

**Module layout:**

```
crates/oxilean-export/src/
  lib.rs          -- pub use; re-exports; module declaration
  error.rs        -- ExportError enum: MalformedInput(String), UnsupportedConstruct(String)
  format.rs       -- ExportFile, EntityTable, NameTable, LevelTable, ExprTable, DeclEntry
  parser.rs       -- ExportParser: streaming line-by-line parser; parse_export()
  name_table.rs   -- NameTable: Vec<Option<Name>> indexed by entity index
  level_table.rs  -- LevelTable: Vec<Option<Level>> indexed by entity index
  expr_table.rs   -- ExprTable: Vec<Option<Expr>> indexed by entity index
  decl_list.rs    -- DeclEntry enum (wraps ConstantInfo) + metadata
  binder_info.rs  -- parse_binder_info(u8) -> Result<BinderInfo, ExportError>
  bignum.rs       -- Bignum: hand-written arbitrary-precision natural (see below)
```

**Key types:**

```rust
// error.rs
#[derive(Debug, Clone)]
pub enum ExportError {
    /// Malformed input: line could not be parsed.
    MalformedInput { line: usize, message: String },
    /// Input is valid lean4export but uses a construct this reader does not support.
    UnsupportedConstruct { line: usize, construct: String },
}

// format.rs
pub struct ExportFile {
    /// Declaration entries in order (each contains a ConstantInfo ready for the kernel).
    pub declarations: Vec<DeclEntry>,
    /// The format version (e.g. 1).
    pub version: u32,
}

pub struct DeclEntry {
    pub ci: oxilean_kernel::ConstantInfo,
}
```

**Main public API:**

```rust
// lib.rs
pub use error::ExportError;
pub use format::ExportFile;

/// Parse a lean4export file from a byte reader.
///
/// Reads line-by-line without loading the entire file into memory (streaming).
/// Returns an `ExportFile` that can be passed to `replay`.
pub fn parse_export<R: std::io::BufRead>(reader: R) -> Result<ExportFile, ExportError>;

/// Parse from a string slice (convenience for tests).
pub fn parse_export_str(s: &str) -> Result<ExportFile, ExportError> {
    parse_export(std::io::Cursor::new(s))
}
```

**Parser internals (`parser.rs`):**

```rust
struct ExportParser {
    names: Vec<Option<Name>>,     // indexed by export name-index
    levels: Vec<Option<Level>>,   // indexed by export level-index (0 = Zero)
    exprs: Vec<Option<Expr>>,     // indexed by export expr-index
    decls: Vec<DeclEntry>,
    version: Option<u32>,
    line_no: usize,
}

impl ExportParser {
    fn parse_line(&mut self, line: &str) -> Result<(), ExportError>;
    fn parse_name_line(&mut self, tag: &str, fields: &[&str]) -> Result<(), ExportError>;
    fn parse_level_line(&mut self, tag: &str, fields: &[&str]) -> Result<(), ExportError>;
    fn parse_expr_line(&mut self, tag: &str, fields: &[&str]) -> Result<(), ExportError>;
    fn parse_decl_line(&mut self, tag: &str, fields: &[&str]) -> Result<(), ExportError>;
    fn get_name(&self, idx: usize) -> Result<Name, ExportError>;
    fn get_level(&self, idx: usize) -> Result<Level, ExportError>;
    fn get_expr(&self, idx: usize) -> Result<Expr, ExportError>;
}
```

**Streaming strategy:** Process one line at a time, growing `names`/`levels`/`exprs` vecs on demand. This avoids loading large Mathlib export files into memory.

**Bignum for ELN (`bignum.rs`):**

The kernel's `Literal::Nat(u64)` is insufficient. The export reader needs a bignum type to handle large nat literals during parsing. Options:
1. Add `Bignum` in `oxilean-export` and convert to a string-keyed special form passed to the kernel.
2. Propose extending `Literal::Nat` in the kernel to use `Vec<u64>` (breaks TCB size constraint).

**Recommended:** The export reader should define a `Bignum(Vec<u32>)` (little-endian limbs) in `bignum.rs`. For nat literals small enough to fit in `u64`, emit `Expr::Lit(Literal::Nat(n))`. For large naturals, the reader should emit `UnsupportedConstruct("bignum Nat literal")` until the kernel is extended. This ensures safety while tracking the gap explicitly.

**Alternatively**, the kernel's `Literal::Nat` could be changed to `Nat(Box<[u32]>)` or a newtype — but that change is in the TCB and requires careful review.

### Crate: `crates/oxilean-verify` (CLI + WASM)

**Purpose:** Standalone binary that reads a lean4export file, replays declarations through the kernel, and reports verdicts. Also compiled to WASM for browser use.

**`Cargo.toml`:**
```toml
[package]
name = "oxilean-verify"

[[bin]]
name = "oxilean-verify"
path = "src/main.rs"

[lib]
name = "oxilean_verify"
path = "src/lib.rs"
crate-type = ["cdylib", "rlib"]

[dependencies]
oxilean-kernel = { workspace = true }
oxilean-export = { path = "../oxilean-export" }
wasm-bindgen = { version = "0.2", optional = true }

[features]
default = []
wasm = ["wasm-bindgen"]
```

**Module layout:**

```
crates/oxilean-verify/src/
  lib.rs      -- pub use; replay(); Verdict enum
  verify.rs   -- replay(env, file) -> Vec<(Name, Verdict)>
  verdict.rs  -- Verdict enum
  main.rs     -- CLI entrypoint: arg parsing, read file, call replay, exit code
  wasm.rs     -- #[wasm_bindgen] wrapper (feature-gated)
```

**Key types:**

```rust
// verdict.rs
#[derive(Debug, Clone)]
pub enum Verdict {
    /// Kernel accepted the declaration.
    Verified,
    /// A named feature is not yet supported; cannot check.
    Unsupported(String),
    /// Kernel rejected the declaration (type error or other kernel error).
    Rejected(String),  // human-readable reason
}

// lib.rs
/// Replay all declarations in `file` against a fresh kernel environment.
/// Returns one verdict per declaration in the file's order.
pub fn replay(file: &ExportFile) -> Vec<(oxilean_kernel::Name, Verdict)>;

/// Parse and replay in one step (convenience).
pub fn verify_export<R: std::io::BufRead>(reader: R) -> Result<Vec<(oxilean_kernel::Name, Verdict)>, ExportError>;
```

**CLI (`main.rs`):**

```rust
fn main() {
    let path = std::env::args().nth(1).expect("usage: oxilean-verify <file.export>");
    let file = std::fs::File::open(&path).expect("cannot open file");
    let reader = std::io::BufReader::new(file);
    let export = match oxilean_export::parse_export(reader) {
        Ok(e) => e,
        Err(e) => { eprintln!("parse error: {e:?}"); std::process::exit(2); }
    };
    let verdicts = oxilean_verify::replay(&export);
    let mut rejected = false;
    for (name, verdict) in &verdicts {
        match verdict {
            Verdict::Verified => println!("ok  {name}"),
            Verdict::Unsupported(f) => println!("?   {name}  [unsupported: {f}]"),
            Verdict::Rejected(r)    => { println!("ERR {name}  {r}"); rejected = true; }
        }
    }
    if rejected { std::process::exit(1); }
}
```

**WASM (`wasm.rs`):**

```rust
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn verify_export_str(content: &str) -> js_sys::Array {
    let export = match oxilean_export::parse_export_str(content) {
        Ok(e) => e,
        Err(e) => { /* return error object */ ... }
    };
    let verdicts = replay(&export);
    // serialize to JS array of {name, verdict, detail} objects
    ...
}
```

### WASM Build Hygiene

The `oxilean-verify` crate (with WASM feature) has exactly two production deps: `oxilean-kernel` + `oxilean-export` + `wasm-bindgen`. This satisfies the dependency allow-list constraint. The `oxilean-export` crate has zero external deps; `oxilean-kernel` has zero external deps.

For the 400KB gzip budget: the kernel has ~3500 SLOC and the export reader target is <=3000 SLOC. A stripped WASM build of these two crates + wasm-bindgen glue should be well under 400KB gzip. The current wasm crate (`oxilean-wasm`) pulls in parser + elaborator; the verify WASM product must NOT depend on those.

### cargo-fuzz Setup

Add a `fuzz/` directory under `crates/oxilean-export/`:
```
crates/oxilean-export/fuzz/
  Cargo.toml
  fuzz_targets/
    fuzz_parse.rs   -- libFuzzer harness calling parse_export(input)
```

The fuzzer should confirm: parse_export never panics on arbitrary byte input, only returns errors.

---

## 5. Important API Gaps / Mismatches

### Gap 1: `Literal::Nat(u64)` — cannot represent bignums

**Location:** `crates/oxilean-kernel/src/expr/types.rs:835`
**Impact:** lean4export ELN lines with values > 2^64-1 cannot be represented.
**Required work:** Either extend `Literal::Nat` in the kernel (TCB change, requires audit) or add an unsupported-construct verdict path in the reader for large literals.

### Gap 2: `Declaration` vs `ConstantInfo` — two parallel type systems

**Location:** `crates/oxilean-kernel/src/env/types.rs:398` and `src/declaration/types.rs:557`
The environment has both a legacy `Declaration` enum (Axiom/Definition/Theorem/Opaque only) and a richer `ConstantInfo` enum (adds Inductive/Constructor/Recursor/Quotient). The reader MUST use `ConstantInfo` and `env.add_constant()` — the `Declaration` type cannot represent inductives.

### Gap 3: No `check_inductive_with_rederiving` API

**Location:** `crates/oxilean-kernel/src/check/functions.rs`
The brief requires that recursors are RE-DERIVED by the kernel rather than trusted from the export file (section 4 part 4). The current `check_constant_info` for `ConstantInfo::Recursor` accepts a `RecursorVal` at face value and calls `check_recursor_val`. There is no API to: (a) ignore exported recursor rules, (b) re-derive them from the InductiveVal. The reader will need to either strip recursor lines and let the kernel synthesize them, or the kernel needs a new entry point `check_and_rederive_recursor(env, recursor_name)`.

### Gap 4: No `instantiate_level_params` exposed on Expr level for reader use

**Location:** `crates/oxilean-kernel/src/declaration/functions.rs`
`pub fn instantiate_level_params(e: &Expr, params: &[Name], levels: &[Level]) -> Expr` is exported from `crate::declaration`. The reader needs this when replaying `ConstantInfo` that has universe parameters. This function IS publicly re-exported from `oxilean_kernel` (line 298 in lib.rs: `pub use declaration::{instantiate_level_params, ...}`), so no gap here — just needs documentation.

### Gap 5: `RecursorVal.rules` — exported rhs Exprs use de Bruijn indices from the exporter

The rhs in `RecursorRule` must use the kernel's locally-nameless BVar convention. If the reader re-derives recursors itself (rather than reading from the export), this is moot. If trusting the export, the Expr indices must be correct. This is a subtle correctness risk.

### Gap 6: No verify CLI subcommand in `oxilean-cli`

The current CLI has: check, build, repl, format, doc, lint, serve, clean, test, completions, help, version. No `verify` command exists.
**Location:** `crates/oxilean-cli/src/main/functions.rs:924` (`builtin_subcommands`)

---

## 6. File/Line Reference Summary

| What | File | Lines |
|------|------|-------|
| Kernel export/ (OXLN binary, NOT lean4export) | `crates/oxilean-kernel/src/export/functions.rs` | 54 (magic), 509-588 |
| `Expr` enum (10 variants, no bignum) | `crates/oxilean-kernel/src/expr/types.rs` | 998-1035 |
| `Literal` enum (Nat u64 only) | `crates/oxilean-kernel/src/expr/types.rs` | 833-839 |
| `Level` enum | `crates/oxilean-kernel/src/level/types.rs` | 1085-1101 |
| `Name` enum | `crates/oxilean-kernel/src/name/types.rs` | 711-719 |
| `ConstantInfo` enum | `crates/oxilean-kernel/src/declaration/types.rs` | 557-573 |
| `InductiveVal` struct | `crates/oxilean-kernel/src/declaration/types.rs` | 1020-1041 |
| `RecursorVal` struct | `crates/oxilean-kernel/src/declaration/types.rs` | 197-233 |
| `ConstructorVal` struct | `crates/oxilean-kernel/src/declaration/types.rs` | 133-146 |
| `QuotVal` struct | `crates/oxilean-kernel/src/declaration/types.rs` | 1302-1307 |
| `DefinitionVal` struct | `crates/oxilean-kernel/src/declaration/types.rs` | 1288-1299 |
| `RecursorRule` struct | `crates/oxilean-kernel/src/declaration/types.rs` | 497-504 |
| `DefinitionSafety` enum | `crates/oxilean-kernel/src/declaration/types.rs` | 828-835 |
| `ReducibilityHint` enum | `crates/oxilean-kernel/src/reduce/types.rs` | 1467-1474 |
| `Environment::add_constant` | `crates/oxilean-kernel/src/env/types.rs` | 250-257 |
| `check_constant_info` | `crates/oxilean-kernel/src/check/functions.rs` | 80-126 |
| No lean4export grep | (confirmed zero hits) | — |
| No oxilean-verify crate | Cargo.toml workspace members | — |
| No oxilean-export crate | Cargo.toml workspace members | — |
| CLI has no verify subcommand | `crates/oxilean-cli/src/main/functions.rs` | 924-986 |

---

## 7. Summary of What the implementer needs to do

1. **Create `crates/oxilean-export`** with `parse_export()` returning `ExportFile`.
2. **Implement NS/NI/US/UM/UIM/UP/EV/ES/EC/EA/EL/EP/EZ/EJ/ELN/ELS line parsers** with Vec-based index tables.
3. **Handle `#AX/#DEF/#OPAQ/#THM/#IND/#QUOT` lines**, mapping to `ConstantInfo` variants.
4. **Write `bignum.rs`** for ELN; return `UnsupportedConstruct` for values > u64::MAX until kernel is extended.
5. **Create `crates/oxilean-verify`** with `replay()` calling `check_constant_info` per declaration, building the three-bucket `Verdict`.
6. **Add cargo-fuzz harness** under `crates/oxilean-export/fuzz/`.
7. **Add `verify` subcommand** to `oxilean-cli` for integration.
8. **WASM build** using `oxilean-verify --features wasm`; verify gzip size ≤ 400KB.

The authoritative lean4export format spec should be obtained from `lean4export/Lean/Export.lean` in the Lean4 repository. Several fields marked UNCERTAIN above must be validated against that source before implementation.
