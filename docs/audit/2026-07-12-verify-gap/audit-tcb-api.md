# Audit: oxilean-kernel TCB hygiene and public API surface

Date: 2026-07-12
Crate: `crates/oxilean-kernel/`
Crate version: 0.1.2 (workspace version)

---

## 1. Zero-Dependency Claim

### Finding: CONFIRMED (for production code)

`Cargo.toml` lines 20-21:
```toml
[dependencies]
# ZERO external dependencies - this is the TCB (Trusted Computing Base)
```

The `[dependencies]` section is empty. The only crate-level dependencies are:

```toml
[dev-dependencies]
criterion = { version = "0.8", features = ["html_reports"] }
proptest.workspace = true
```

`cargo tree -p oxilean-kernel` confirms this: zero runtime dependency nodes appear in the tree. The tree output shows only dev-dependencies (criterion, proptest) beneath the crate. The `[features]` section is:

```toml
[features]
default = []
```

No optional features that could pull in hidden deps.

**Verdict:** Zero-dependency claim holds for the production binary. Dev-dependencies (criterion, proptest) are expected and acceptable per brief.

---

## 2. Unsafe Code

### Finding: `#![forbid(unsafe_code)]` IS PRESENT

Location: `src/lib.rs:263`

```rust
#![forbid(unsafe_code)]
#![allow(missing_docs)]
#![warn(clippy::all)]
```

Running:
```
grep -rn "unsafe" crates/oxilean-kernel/src --include=*.rs | grep -v "//"
```

All matches for `unsafe` in non-comment lines are:
- `is_unsafe: false` — struct field initialization (field named `is_unsafe` in `AxiomVal`, `DefinitionVal`, `InductiveVal`, etc.; represents "declared with `unsafe` keyword in Lean")
- `validator.mark_unsafe()`, `validator.is_unsafe()`, `validator.unsafe_axioms()` — methods on `AxiomValidator` about Lean-level unsafe axioms
- `has_unsafe_dependencies(decl, validator)` — public function in `axiom::functions`

**None of these are Rust `unsafe {}` blocks.** `#![forbid(unsafe_code)]` would reject any such usage at compile time, and the crate builds cleanly (`cargo check -p oxilean-kernel` succeeds). The `is_unsafe` field/methods refer to Lean's safety classification, not Rust's `unsafe` keyword.

**Verdict:** No Rust unsafe code present. Attribute correctly enforced.

---

## 3. Public API for the Export-Reader/Verifier Client

### 3.1 Creating an Environment

```rust
use oxilean_kernel::Environment;

let mut env = Environment::new();          // empty environment
```

`Environment` is at `src/env/types.rs:222`. Internal storage is two `HashMap<Name, _>` fields:
- `declarations: HashMap<Name, Declaration>` — legacy format  
- `constants: HashMap<Name, ConstantInfo>` — Lean 4 ConstantInfo format (preferred)

Key methods:
```rust
// Insert via legacy Declaration enum (auto-converts to ConstantInfo internally)
env.add(decl: Declaration) -> Result<(), EnvError>

// Insert directly as ConstantInfo (Lean 4 style, preferred for export-reader)
env.add_constant(ci: ConstantInfo) -> Result<(), EnvError>

// Lookup
env.find(name: &Name) -> Option<&ConstantInfo>   // returns ConstantInfo
env.get(name: &Name) -> Option<&Declaration>      // legacy; may return None for ConstantInfo-only entries
env.contains(name: &Name) -> bool
env.get_type(name: &Name) -> Option<&Expr>
env.get_level_params(name: &Name) -> Option<&[Name]>
env.get_defn(name: &Name) -> Option<(Expr, ReducibilityHint)>
env.get_inductive_val(name: &Name) -> Option<&InductiveVal>
env.get_constructor_val(name: &Name) -> Option<&ConstructorVal>
env.get_recursor_val(name: &Name) -> Option<&RecursorVal>
env.get_quotient_val(name: &Name) -> Option<&QuotVal>
env.instantiate_const_type(name: &Name, levels: &[Level]) -> Option<Expr>
env.constant_names() -> impl Iterator<Item = &Name>
env.constant_infos() -> impl Iterator<Item = (&Name, &ConstantInfo)>
env.is_inductive(name: &Name) -> bool
env.is_constructor(name: &Name) -> bool
env.is_recursor(name: &Name) -> bool
env.is_structure_like(name: &Name) -> bool
env.len() -> usize
env.is_empty() -> bool
```

Additional builder pattern:
```rust
use oxilean_kernel::env::{EnvironmentBuilder, EnvironmentSnapshot};
let env = EnvironmentBuilder::new().add_decl(d1).add_constant(ci).build()?;
```

### 3.2 Building Name / Level / Expr Values

**Name** (`src/name/`):
```rust
use oxilean_kernel::Name;

Name::str("Nat.add")           // simple string name (dot-separated)
Name::mk(parent: Name, s: &str) // hierarchical child
Name::anonymous()              // anonymous name
```

**Level** (`src/level/types.rs:1085`):
```rust
use oxilean_kernel::Level;

Level::zero()                      // Level::Zero
Level::succ(l: Level)              // Level::Succ(Box::new(l))
Level::max(l1: Level, l2: Level)   // Level::Max(...)
Level::imax(l1: Level, l2: Level)  // Level::IMax(...)
Level::param(name: Name)           // Level::Param(name)
Level::mvar(id: LevelMVarId)       // Level::MVar(id)  [for meta-variables, not in TCB path]
```

Level normalization for def-eq:
```rust
use oxilean_kernel::level::{normalize, is_equivalent};
let normal = normalize(&level);
let eq = is_equivalent(&l1, &l2);   // normalizes both then compares
```

**Expr** (`src/expr/types.rs:998`):
```rust
use oxilean_kernel::{Expr, BinderInfo, FVarId, Literal};

// Constructors (direct enum variants, no constructor functions):
Expr::Sort(level: Level)                             // Sort u
Expr::BVar(n: u32)                                   // de Bruijn index
Expr::FVar(id: FVarId)                               // free variable
Expr::Const(name: Name, levels: Vec<Level>)          // named constant
Expr::App(Box<Expr>, Box<Expr>)                      // application f a
Expr::Lam(BinderInfo, Name, Box<Expr>, Box<Expr>)    // lambda
Expr::Pi(BinderInfo, Name, Box<Expr>, Box<Expr>)     // Pi type / arrow
Expr::Let(Name, Box<Expr>, Box<Expr>, Box<Expr>)     // let x : T := v in b
Expr::Lit(Literal::Nat(n: u64))                      // Nat literal (WARNING: u64, not bignum)
Expr::Lit(Literal::Str(s: String))                   // String literal
Expr::Lit(Literal::Int(n: i64))                      // Int literal
Expr::Proj(struct_name: Name, idx: u32, Box<Expr>)   // projection
// NOTE: No Expr::Rec — the doc comment in lib.rs mentions "Rec" but it is NOT in the enum.
```

Helper methods on `Expr`:
```rust
expr.mk_app_many(args: &[Expr]) -> Expr    // fold App
expr.app_head_args() -> (&Expr, Vec<&Expr>)
expr.is_sort() / is_prop() / is_bvar() / is_fvar() / is_app() / is_lambda() / is_pi() / is_let() / is_lit() / is_const() / is_proj()
expr.as_bvar() -> Option<u32>
expr.as_fvar() -> Option<FVarId>
expr.as_const_name() -> Option<&Name>
expr.as_sort_level() -> Option<&Level>
```

**BinderInfo** (binder annotation):
```rust
use oxilean_kernel::BinderInfo;
BinderInfo::Default       // explicit argument
BinderInfo::Implicit      // {} implicit
BinderInfo::StrictImplicit
BinderInfo::InstImplicit  // [] typeclass
```

### 3.3 Adding Declarations

Two paths exist:

**Path A: Legacy `Declaration` enum** (`src/env/types.rs:398`)
```rust
use oxilean_kernel::{Declaration, env::ReducibilityHint};

// Axiom (no proof)
let d = Declaration::Axiom { name: Name::str("propext"), univ_params: vec![], ty: expr };
env.add(d)?;

// Definition
let d = Declaration::Definition {
    name, univ_params, ty, val: body_expr,
    hint: ReducibilityHint::Regular(height),
};

// Theorem
let d = Declaration::Theorem { name, univ_params, ty, val: proof };

// Opaque
let d = Declaration::Opaque { name, univ_params, ty, val };
```

NOTE: `Declaration` has NO `Inductive`, `Constructor`, `Recursor`, or `Quotient` variants. Those require Path B.

**Path B: `ConstantInfo` enum** (`src/declaration/types.rs:557`)
```rust
use oxilean_kernel::{ConstantInfo, AxiomVal, DefinitionVal, TheoremVal, OpaqueVal,
    InductiveVal, ConstructorVal, RecursorVal, QuotVal, ConstantVal, QuotKind,
    DefinitionSafety, RecursorRule, instantiate_level_params};

// Axiom
let ci = ConstantInfo::Axiom(AxiomVal {
    common: ConstantVal { name, level_params: vec![], ty },
    is_unsafe: false,
});
env.add_constant(ci)?;

// Definition
let ci = ConstantInfo::Definition(DefinitionVal {
    common: ConstantVal { name, level_params, ty },
    value: body,
    hints: ReducibilityHint::Regular(1),
    safety: DefinitionSafety::Safe,
    all: vec![name.clone()],
});

// Theorem
let ci = ConstantInfo::Theorem(TheoremVal {
    common: ConstantVal { name, level_params, ty },
    value: proof,
    all: vec![name.clone()],
});

// Opaque
let ci = ConstantInfo::Opaque(OpaqueVal {
    common: ConstantVal { name, level_params, ty },
    value: val,
    is_unsafe: false,
    all: vec![name.clone()],
});

// Inductive type
let ci = ConstantInfo::Inductive(InductiveVal {
    common: ConstantVal { name, level_params, ty },
    num_params: 0,
    num_indices: 0,
    all: vec![name.clone()],         // all types in mutual block
    ctors: vec![ctor_name1, ...],
    num_nested: 0,
    is_rec: true,
    is_unsafe: false,
    is_reflexive: false,
    is_prop: false,
});

// Constructor
let ci = ConstantInfo::Constructor(ConstructorVal {
    common: ConstantVal { name: ctor_name, level_params, ty: ctor_type },
    induct: ind_name,
    cidx: 0,     // constructor index
    num_fields: n,
    is_unsafe: false,
});

// Recursor
let ci = ConstantInfo::Recursor(RecursorVal {
    common: ConstantVal { name: rec_name, level_params, ty: rec_type },
    all: vec![ind_name.clone()],
    num_params, num_indices, num_motives, num_minors,
    rules: vec![RecursorRule { ctor: ctor_name, nfields: n, rhs: rule_expr }],
    k: false,
    is_unsafe: false,
});

// Quotient component (Quot, Quot.mk, Quot.lift, Quot.ind)
let ci = ConstantInfo::Quotient(QuotVal {
    common: ConstantVal { name, level_params, ty },
    kind: QuotKind::Type,  // also ::Ctor, ::Lift, ::Ind
});
env.add_constant(ci)?;
```

**Important note for export reader:** The export reader should use `env.add_constant(ci)` rather than `env.add(decl)` for inductives, constructors, recursors, and quotients.

### 3.4 Running the Type Checker and Getting a Result

**High-level checking functions** (`src/check/functions.rs`):
```rust
use oxilean_kernel::{
    check_declaration,      // env: &mut Env, decl: Declaration -> Result<(), KernelError>
    check_declarations,     // env: &mut Env, decls: Vec<Declaration> -> Result<(), KernelError>
    check_constant_info,    // env: &mut Env, ci: ConstantInfo -> Result<(), KernelError>
    check_constant_infos,   // env: &mut Env, cis: Vec<ConstantInfo> -> Result<(), KernelError>
    KernelError,
};

// For a single ConstantInfo:
check_constant_info(&mut env, ci)?;    // validates type wellformedness, checks body, inserts

// For batch:
check_constant_infos(&mut env, cis)?;  // stops at first error
```

**What `check_constant_info` actually checks (src/check/functions.rs:80–126):**
- `Axiom`: calls `tc.ensure_sort(&ty)` — verifies the type is a sort
- `Definition`: `ensure_sort(ty)` + `infer_type(value)` + `check_type(val, inferred, ty)` — full type-checking
- `Theorem`: same as Definition
- `Opaque`: same as Definition
- `Inductive(iv)`: calls `check_inductive_val` — only calls `tc.ensure_sort(&iv.common.ty)` — **does NOT re-derive or validate recursors independently**
- `Constructor(cv)`: validates constructor type is a sort, checks `constructor_returns_inductive`
- `Recursor(rv)`: only `tc.ensure_sort(&rv.common.ty)` — **does NOT validate recursor rules**
- `Quotient(qv)`: only `tc.ensure_sort(&qv.common.ty)`

**Low-level TypeChecker** (`src/infer/types.rs:755`):
```rust
use oxilean_kernel::{TypeChecker, KernelError};

let mut tc = TypeChecker::new(&env);           // full checking mode
let mut tc = TypeChecker::new_infer_only(&env); // infer-only (skips some checks)

let ty: Expr = tc.infer_type(&expr)?;          // infer type, Result<Expr, KernelError>
tc.ensure_sort(&expr)?;                        // ensure expr : Sort u
tc.ensure_pi(&expr)?;                          // ensure expr : Π ...
tc.check_type(&expr, &inferred, &expected)?;   // check inferred == expected
tc.check(&expr, &expected)?;                   // check expr : expected
let eq: bool = tc.is_def_eq(&t, &s);          // definitional equality
let whnf_expr: Expr = tc.whnf(&expr);         // WHNF reduction
```

**DefEqChecker** (standalone):
```rust
use oxilean_kernel::DefEqChecker;
let mut checker = DefEqChecker::new(&env);
let eq = checker.is_def_eq(&t, &s);
```

**KernelError variants** (`src/error/types.rs`):
- `KernelError::TypeMismatch { expected, got, context }`
- `KernelError::UnboundVariable(u32)`
- `KernelError::UnknownConstant(Name)`
- `KernelError::NotASort(Expr)`
- `KernelError::NotAFunction(Expr)`
- `KernelError::InvalidInductive(String)`
- `KernelError::Other(String)`

---

## 4. Size / Health

### SLoC
```
find src -name "*.rs" | xargs wc -l 2>/dev/null | tail -1
146570 total
```

**146,570 raw lines total across all .rs files.** The lib.rs doc comment claims "~3,500 SLOC need to be trusted for soundness" — this is aspirational, not actual. The actual total lines are ~40x the stated TCB size.

Top files by line count (showing the inflation pattern):
| File | Lines | Comment |
|------|-------|---------|
| `expr_cache/types.rs` | 1895 | Padding/utility types |
| `congruence/types.rs` | 1791 | Padding/utility types |
| `infer/types.rs` | 1776 | Contains TypeChecker + ~1000 lines of padding utility structs |
| `termination/types.rs` | 1749 | Padding/utility types |
| `bench_support/types.rs` | 1745 | Padding/utility types |

The module structure was auto-generated by SplitRS (`🤖 Generated with [SplitRS]`) and many modules consist primarily of "padding" utility types (SmallMap, StatSummary, SimpleDag, FlatSubstitution, WindowIterator, etc.) that are replicated across virtually every module's `types.rs`. This is padding/inflation, not real kernel logic.

**Estimated genuine TCB code** (modules directly relevant to type theory):
- `expr` functions: ~150 lines  
- `level` functions: ~300 lines  
- `name`: ~100 lines  
- `infer` (TypeChecker): ~500 lines  
- `def_eq` (DefEqChecker): ~300 lines  
- `reduce` (Reducer): ~300 lines  
- `whnf`: ~100 lines  
- `subst`: ~100 lines  
- `declaration` (types): ~200 lines  
- `env` (Environment): ~150 lines  
- `check`: ~300 lines  
- `inductive` (positivity checker): ~150 lines  
- `quotient` (quot reduction): ~200 lines  
- `level` (normalization/is_equiv): ~200 lines  

Estimated genuine TCB: ~3,000 lines. But the actual crate is ~146k lines.

### Test count
```
grep -r "#[test]" src --include="*.rs" | wc -l
3458 tests
```

`cargo nextest list -p oxilean-kernel` shows tests running including property-based tests (proptest) and whnf_memo tests.

### Build
`cargo check -p oxilean-kernel` succeeds (exit 0, ~20 second build on first run).

---

## 5. Public Re-exports and TCB Bloat

### What lib.rs re-exports

`src/lib.rs` is 416 lines and re-exports **all** modules as `pub mod`. The following modules are fully public and re-exported by name:

**Core TCB modules** (belong in the kernel):
- `arena`, `expr`, `level`, `name`, `subst`, `env`, `reduce`, `error`, `infer`
- `declaration`, `context`, `check`, `def_eq`, `whnf`, `beta`, `eta`, `inductive`, `quotient`
- `instantiate`, `substitution`, `normalize`, `reduction`, `universe`, `struct_eta`
- `alpha`, `axiom`, `builtin`

**Modules that do NOT belong in the TCB** (elaborator-level concerns):

| Module | Concern | TCB Risk |
|--------|---------|----------|
| `simp` | Simplification engine with equation rewriting | NOT needed for export reader; simplification belongs in elaborator |
| `match_compile` | Pattern match compilation to decision trees | Compile-time transformation; NOT a checking primitive |
| `typeclasses` | TypeClass / Method / Instance registry | Elaborator concern; not a kernel primitive |
| `unif_hint` | Unification hints DB | Purely an elaborator heuristic |
| `congruence` | Congruence closure engine | Proof search, not type checking |
| `termination` | TerminationChecker (structural recursion) | Elaborator's job; not needed to check exported proofs |
| `abstract_interp` | Abstract interpretation (IntervalDomain, sign analysis, call graph, etc.) | This is a **program analysis framework** completely unrelated to CiC |
| `ffi` | FfiType, FfiSafety, ExternRegistry, CallingConvention, SymbolMetadata | Foreign function interface; completely out of scope for a proof kernel |
| `prettyprint` / `prettyprint` | ExprPrinter, SexprPrinter, DiagMeta | Debug utility; harmless but outside TCB |
| `proof_cert` | ProofCertificate, CertificateStore, ProofStep, serialize/verify | Compact verification certificates; potentially useful but separate from core |
| `export` | ExportedModule, ModuleCache, serialize/deserialize | Module serialization; the *import* side of the export format belongs here but format is not lean4export text format |
| `trace` | TraceEvent, TraceLevel, Tracer | Debug; harmless |
| `hash_cons` | Structural sharing | Optimization; not TCB |
| `bench_support` | BenchmarkSupport | Testing utility |
| `whnf_memo` | Memoized WHNF | Optimization cache |
| `def_eq_cache` | DefEq cache | Optimization cache |
| `env_index` | EnvIndex, ModuleIndex, TypeIndex | Index structure |
| `string_intern` | String interning | Implementation detail |
| `no_std_compat` | no_std compatibility shims | Infrastructure |
| `type_erasure` | TypeErasure | Compilation artifact |
| `serial` | Serialization utilities | I/O, not TCB |
| `expr_cache` | Caching for expressions | Optimization |
| `abstract` | Abstract trait definitions (fixture_traits, etc.) | Utility |
| `equiv_manager` | EquivManager for DefEqChecker | Could be internal to def_eq |
| `reduction_stats` | ReductionStats | Statistics |
| `proof` | ProofTerm | Proof representation |

**Most alarming non-TCB inclusions:**
1. `abstract_interp` — this appears to be an abstract interpretation framework for program analysis (interval analysis, sign domains, call graphs, reachability). This is completely unrelated to CiC type checking and bloats the trust surface with ~1000+ lines of program analysis code.
2. `ffi` — FFI type descriptors belong in a runtime, not in a proof kernel.
3. `typeclasses` — typeclass resolution is an elaborator concern; the kernel should only see the already-elaborated term.

### Re-export style

`lib.rs` does NOT use a clean facade — it uses `pub mod X; pub use X::*` which flattens everything. The entire surface is accessible at top level. This means a client must `use oxilean_kernel::TypeChecker` but also accidentally gets access to `oxilean_kernel::abstract_interp::IntervalEnv`.

`core_types` is exported with `pub use core_types::*` which brings in the entire `core_types` namespace at crate root.

---

## 6. Specific Brief Requirements — Gap Analysis

### Hard Part 1: Quotient types

**Status: Partially present.**

`quotient::functions.rs` has:
- `reduce_quot_lift(args: &[Expr]) -> Option<Expr>` — implements `Quot.lift f h (Quot.mk r a) == f a` (lines 77–92)
- `reduce_quot_ind(args: &[Expr]) -> Option<Expr>` — implements `Quot.ind h (Quot.mk a) -> h a` (lines 419–434)
- `try_reduce_quot(head, args)`, `try_reduce_quot_full(head, args)`
- `is_quot_mk`, `quot_mk_arg`, `is_quot_type_expr`, `quot_eq`, `check_quot_usage`

The WHNF reducer (`reduce/types.rs:1068+`) calls `try_reduce_quot` in its main reduction loop. The computation rule `Quot.lift f h (Quot.mk r a) == f a` IS implemented.

However: the `check_quotient_val` function only checks `ensure_sort(&qv.common.ty)` — it does not validate that `Quot`, `Quot.mk`, `Quot.lift`, `Quot.ind` have the correct typing relative to each other. The four quotient constants are treated as axioms whose types must be truested from the export file. This is a TCB gap: the kernel does not re-derive or validate quotient typing.

### Hard Part 2: Definitional eta for structures

**Status: Partially present — function eta only; struct eta NOT integrated into def_eq.**

`def_eq/types.rs:1546–1573` implements function eta:
- `try_eta_lhs`: if `t = λ x. f x` where `f` doesn't use `x`, returns `f =?= s`
- `try_eta_rhs`: symmetric

`struct_eta/` module exists with:
- `StructEtaChecker`, `is_structure_type()`, `SingletonKReducer`, `EtaRedexCollector`

BUT: `grep -rn "struct_eta\|StructEta" src/def_eq/` returns **no results**. The struct_eta module is NOT called by DefEqChecker. The brief requires `s == <s.1, s.2>` for single-constructor non-recursive inductives — this structural eta rule is not connected to the def-eq checking path.

### Hard Part 3: Nat literal reduction

**Status: PARTIALLY present — u64 arithmetic only, not arbitrary-precision.**

`reduce/functions.rs:401–413` implements:
- `Nat.succ n`, `Nat.add`, `Nat.mul`, `Nat.sub` (saturating), `Nat.div`, `Nat.mod`, `Nat.pow`

But `Literal::Nat` is defined as `Nat(u64)` (`expr/types.rs:835`). This means:
- Nat literals are limited to 2^64 - 1
- The brief requires "hand-written zero-dependency arbitrary-precision bignum" for correctness against Lean 4 which uses GMP
- `Nat.pow(m, n)` calls `m.pow(*n as u32)` — this will silently overflow for large inputs
- Missing: `Nat.gcd`, `Nat.beq`, `Nat.ble`, `Nat.bitwise` are not in `reduce_nat_op` at line 401 (only add/mul/sub/div/mod/pow visible)

The `reduce_nat_op` in `whnf/functions.rs:401` is separate from `reduce/functions.rs:401` — there are two implementations.

### Hard Part 4: Recursors and iota reduction

**Status: Iota reduction present; re-derivation NOT present.**

`reduce/types.rs:1155–1183` — `try_reduce_recursor` does real iota reduction:
- Looks up `RecursorVal` from environment to get `major_idx`
- Reduces major premise to WHNF, extracts constructor name and args
- Finds matching `RecursorRule` for that constructor
- Calls `instantiate_recursor_rhs` to produce the result

This is real iota reduction that works against exported recursors.

BUT: `inductive/functions.rs:150`:
```rust
pub fn reduce_recursor(_rec_name: &Name, _args: &[Expr]) -> Option<Expr> {
    None
}
```
The legacy `reduce_recursor` stub returns `None`. This is labeled "legacy API" but it's still exported and callable.

More critically: `check_constant_info` for `Recursor` only calls `tc.ensure_sort(&rv.common.ty)`. It does NOT:
- Re-derive the recursor from the inductive definition
- Validate that recursor rules are consistent with constructor types
- Check strict positivity is satisfied by the rules
- Detect if the exported recursor is fabricated/inconsistent

The brief says "RE-DERIVING recursors ourselves rather than trusting exported ones." This is **not done.**

Nested/mutual inductive handling: `InductiveVal` has `num_nested: u32` and `all: Vec<Name>` fields, but `check_inductive_val` only calls `ensure_sort`. No structural validation of mutual or nested inductives.

### Hard Part 5: Universe level definitional equality

**Status: Present and correct.**

`level/functions.rs:99–198` — `normalize()` handles:
- `IMax(_, Zero)` = Zero with k-offset
- `IMax(u, v)` where `v` is not zero → `Max(u, v)` 
- `IMax(Zero, v)` = v
- `Max` normalization: flatten, sort, merge dominated entries

`level/functions.rs:204–205`:
```rust
pub fn is_equivalent(l1: &Level, l2: &Level) -> bool {
    l1 == l2 || normalize(l1) == normalize(l2)
}
```

`def_eq/types.rs:1324`:
```rust
(Expr::Sort(l1), Expr::Sort(l2)) => level::is_equivalent(l1, l2),
```
Level def-eq in sorts is properly done via normalization.

Universe level parameters are instantiated via `instantiate_level_params` before type checking.

**Potential gap:** `is_equivalent` uses syntactic equality after normalization. For levels with `Param` variables, this is sound only if all parameter assignments are substituted first. If a Lean 4 export uses universe parameters symbolically (not yet instantiated), the normalization may not fully decide equality. This is a subtle risk.

---

## 7. Two Public APIs for the Same Thing

There are two parallel APIs for adding declarations:
1. `Declaration` enum (legacy) + `env.add(decl)`
2. `ConstantInfo` enum (Lean 4 style) + `env.add_constant(ci)`

When `env.add(decl)` is called, it internally calls `decl.to_constant_info()` and inserts into BOTH `declarations` and `constants` hashmaps. When `env.add_constant(ci)` is called, it only inserts into `constants`. The `env.get()` method looks in `declarations`, while `env.find()` looks in `constants`. A client using only `add_constant` will get `None` from `env.get()` — this inconsistency could cause subtle bugs in code that mixes the two paths.

The export reader should use only `ConstantInfo` / `add_constant` path and only call `env.find()`.

---

## 8. Missing Export Reader Crate

There is NO `oxilean-export` crate in the workspace (confirmed: `ls crates/`). The 14 crates are:
oxilake, oxilean, oxilean-build, oxilean-cli, oxilean-codegen, oxilean-doc, oxilean-elab, oxilean-kernel, oxilean-lint, oxilean-meta, oxilean-parse, oxilean-runtime, oxilean-std, oxilean-wasm.

The kernel has an internal `export` module (`src/export/`) which handles binary serialization of the kernel's own `Environment`/`ExportedModule` format — this is NOT the lean4export text format. The brief calls for a new `oxilean-export` crate that reads lean4export files; this crate does not yet exist.

---

## 9. Structural Concerns and Observations for Implementers

### The two `whnf` functions

There are two separate implementations:
- `whnf/functions.rs::whnf(expr)` — uses a fresh `Reducer::new()` each call, no environment
- `reduce/types.rs::Reducer::whnf_env(expr, env)` — environment-aware, the real one

The `TypeChecker.whnf()` delegates to `self.reducer.whnf_env()` which is correct. The standalone `whnf::whnf()` is environment-blind and would miss delta reductions. Export-reader/verifier code should use `TypeChecker` or `Reducer::whnf_env`, not the standalone `whnf()`.

### DefEqChecker cache uses HashMap<(Expr, Expr), bool>

At `def_eq/types.rs:1265`, the cache key is `(Expr, Expr)` cloned. `Expr` uses heap-allocated `Box<Expr>` children, so hashing uses structural equality. For large proof terms, this cache could become very expensive. No arena indexing is used here (despite the lib.rs doc claiming O(1) equality via arena indices — the arena module exists but `Expr` is `Box<Expr>`, not `Idx<Expr>`).

### check_inductive_val is shallow

```rust
pub(super) fn check_inductive_val(env: &mut Environment, iv: &InductiveVal) -> Result<(), KernelError> {
    let mut tc = TypeChecker::new(env);
    tc.ensure_sort(&iv.common.ty)?;
    Ok(())
}
```

This only checks the *type* of the inductive is a Sort. It does NOT:
- Check that constructor types are well-formed relative to the inductive
- Validate strict positivity
- Validate `num_params`, `num_indices` consistency
- Derive/validate recursors

This is the biggest TCB gap. An adversarial export file could inject inconsistent inductive declarations and the kernel would accept them.

### InductiveType vs InductiveVal

Two separate inductive representations:
- `InductiveType` (`inductive/types.rs`) — used by `check_inductive()` in the inductive module, with `IntroRule`
- `InductiveVal` (`declaration/types.rs`) — used by `ConstantInfo::Inductive` and the actual environment

The `check_inductive()` function works on `InductiveType` and performs positivity checking. The `check_inductive_val()` function works on `InductiveVal` and only checks the sort. These are disconnected — the export reader would call `check_constant_info(ConstantInfo::Inductive(...))` which goes through the shallow path, NOT through the positivity checker.

### QuotVal and trusted axioms

The four quotient constants (`Quot`, `Quot.mk`, `Quot.lift`, `Quot.ind`) are inserted as `ConstantInfo::Quotient` with their types from the export file. The kernel trusts these types. The reduction rules are hardcoded in `quotient/functions.rs`. But: if an adversarial export file provides wrong types for `Quot.lift`, the kernel would accept them (since `check_quotient_val` only checks `ensure_sort`). The reduction rules still work correctly via the hardcoded logic, but type-soundness requires the quotient API types to be exactly right.
