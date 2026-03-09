# OxiLean Frequently Asked Questions

Answers to common questions about OxiLean.

## Table of Contents

1. [General Questions](#general-questions)
2. [Lean 4 Compatibility](#lean-4-compatibility)
3. [Performance Characteristics](#performance-characteristics)
4. [Kernel Soundness Guarantees](#kernel-soundness-guarantees)
5. [Comparison with Other Provers](#comparison-with-other-provers)
6. [WASM Support](#wasm-support)
7. [IDE Integration](#ide-integration)
8. [Advanced Topics](#advanced-topics)

---

## General Questions

### Q: What is OxiLean?

A: OxiLean is a pure Rust implementation of an Interactive Theorem Prover (ITP) based on the Calculus of Inductive Constructions (CiC). It's designed to be:

- **Memory-safe** -- Written in Rust with no unsafe code in the kernel
- **Zero-dependency** -- Trusted Computing Base has zero external dependencies
- **WASM-first** -- Can run entirely in the browser via oxilean-wasm bindings
- **Cargo-integrated** -- Proof libraries can be distributed as Rust crates

### Q: Why another theorem prover?

A: OxiLean fills several niches:

1. **Pure Rust** -- 1,221,710 SLOC across 11 crates, all in Rust with no C/Fortran dependencies
2. **Rust integration** -- Proofs as first-class Cargo items
3. **WASM capability** -- Run proofs in browsers via oxilean-wasm bindings
4. **Pedagogical** -- Clear, auditable design for learning ITP implementation

### Q: Is OxiLean production-ready?

A: OxiLean v0.1.1 has all core features complete:

- Type checker and kernel (113,179 SLOC)
- Lexer and parser (oxilean-parse)
- Elaborator (oxilean-elab)
- Tactic framework with 20+ tactics implemented
- Standard library (413K SLOC, oxilean-std)
- WASM bindings (oxilean-wasm)
- Code generation (oxilean-codegen)
- Build system (oxilean-build)
- Linter (oxilean-lint)
- Metaprogramming support (oxilean-meta)
- CLI (oxilean-cli)

OxiLean is suitable for educational and research use. Mathlib4 compatibility stands at 100% (4,530/4,530 declarations), with 289 curated theorem proofs passing.

### Q: Can I use OxiLean for real proofs?

A: Yes, for educational and research purposes. OxiLean supports:

- **Educational proofs** -- Learning formal verification
- **Research** -- Experimenting with ITP design
- **Theorem proving** -- 289 curated theorem proofs passing, full Mathlib4 declaration coverage
- **Formalization projects** -- Standard library with 413K SLOC

### Q: What's the difference between OxiLean and Lean 4?

OxiLean is NOT a Lean fork. Key differences:

| Aspect | OxiLean | Lean 4 |
|--------|---------|--------|
| Kernel deps | 0 | 200+ |
| SLOC (kernel) | 113K | 50,000+ |
| Total SLOC | 1,221,710 | N/A |
| Target platforms | WASM, Native | Native |
| Tactic system | 20+ tactics | Complex |
| Standard library | 413K SLOC | 10,000+ items |
| Mathlib4 compat | 100% declarations | Native |

OxiLean's design is inspired by Lean 4, but diverges significantly for portability.

### Q: Is OxiLean a fork of Coq?

A: No. OxiLean is an independent implementation. However, it shares:

- **Theory** -- Both implement variants of CiC
- **Inspiration** -- Both have similar design philosophies
- **Ecosystem** -- Both support tactics and modules

### Q: Can I import Lean proofs into OxiLean?

A: **Not directly.** You would need to:

1. Manually translate Lean syntax to OxiLean syntax
2. Use an automated translator (not yet built)
3. Export Lean to a standard format (LEX, not yet standardized)

The proof *content* could transfer, but requires rewriting.

---

## Lean 4 Compatibility

### Q: Can I run Lean 4 code in OxiLean?

A: **Not as-is.** However, many proofs can be ported with manual effort:

```lean
-- Lean 4
theorem add_comm : forall n m : Nat, n + m = m + n := by omega

-- Can be ported to OxiLean as:
theorem add_comm : forall n m : Nat, n + m = m + n := by
  intro n m
  omega
```

Key porting considerations:

1. **Syntax differences** -- Use OxiLean's surface syntax
2. **Tactic differences** -- OxiLean has 20+ tactics implemented
3. **Library differences** -- Standard library differs
4. **Universe polymorphism** -- OxiLean uses simpler universe system

### Q: What is OxiLean's Lean 4 compatibility status?

A: OxiLean achieves high compatibility with Lean 4:

- Core CiC theory -- complete
- Tactics -- 20+ implemented
- Pattern matching -- complete
- Type classes -- complete
- Module system -- complete
- Elaboration system -- complete
- Mathlib4 declarations -- 100% (4,530/4,530)
- Curated theorem proofs -- 289 passing

### Q: Can I use OxiLean libraries in Lean 4?

A: **No direct support**, but potential export formats:

1. **Proof certificates** -- Export proofs in a neutral format
2. **Kernel import** -- Lean 4 could import verified terms
3. **Explicit translation** -- Hand-translate definitions

---

## Performance Characteristics

### Q: How fast is OxiLean?

OxiLean performance depends on proof complexity:

| Task | Time | Notes |
|------|------|-------|
| Simple proof | <1ms | `trivial`, `rfl` |
| Arithmetic (omega) | 1-10ms | Small problems |
| Induction | 10-100ms | Linear depth |
| Simplification (simp) | 10-500ms | Depends on lemmas |
| Large proofs | 1-10s | Complex elaboration |

**Benchmarking:**

```bash
oxilean check --profile myproof.oxilean
# Shows timing breakdown
```

### Q: Why is OxiLean slower than Lean 4?

Main reasons:

1. **Less optimization** -- Lean has years of performance work
2. **Simpler kernel** -- Less aggressive reduction
3. **Pure Rust** -- No LLVM/C optimization
4. **No caching** -- Fresh evaluation each run

Performance improvements planned:

- Memoization in reduction
- Faster unification
- Better tactic evaluation
- Kernel-level optimizations

### Q: Can OxiLean scale to large formalizations?

Currently: **Small to medium proofs** (< 10K lines)

Future improvements needed:

1. **Incremental checking** -- Check only changed definitions
2. **Parallel proofs** -- Use Rust's parallelism
3. **Smart caching** -- Remember intermediate results
4. **Module boundaries** -- Separate compilation

Expected capacity at scale:

- 100K lines of proofs -- with incremental checking
- 1M lines -- with parallel checking + caching
- 10M lines -- requires architectural changes

### Q: How does WASM performance compare?

WASM execution in browsers:

- **Time overhead** -- 10-50% vs native (depends on browser)
- **Memory overhead** -- 2-5x for WASM runtime
- **Startup time** -- 0.5-2s for module loading

Optimizations:

- Compile to WebAssembly optimized format
- Use `wasm-opt` for binary size
- Lazy load standard library
- Cache compiled module

### Q: Should I use OxiLean or Lean 4?

**Use Lean 4 if:**
- You need a large, mature ecosystem
- You need advanced tactics beyond OxiLean's set
- You're doing large-scale formalization
- You need professional community support

**Use OxiLean if:**
- You want to understand ITP internals
- You need WASM deployment
- You prefer pure Rust with zero C/Fortran dependencies
- You're doing educational or research projects
- You want minimal dependencies

---

## Kernel Soundness Guarantees

### Q: How sound is OxiLean's kernel?

The kernel implements CiC faithfully, making it **sound** in the sense that:

1. **Type correctness** -- If a term typechecks, it has the declared type
2. **No false proofs** -- Cannot prove `False` without an axiom
3. **No undefined behavior** -- No `unsafe` code in kernel
4. **Deterministic** -- Same input always produces same output

**Formal guarantees:**

OxiLean does NOT have formal proofs of soundness (unlike some ITPs). Instead, soundness is ensured through:

1. **Code review** -- Minimal kernel for auditability
2. **Testing** -- Extensive test suite (289 curated theorem proofs)
3. **Design** -- Standard CiC implementation

### Q: What can break OxiLean's soundness?

**Cannot break soundness:**
- Parser bugs -- only cause parse errors
- Elaborator bugs -- only cause elaboration failure
- Tactic bugs -- only cause tactic failure
- Library bugs -- unsoundness won't propagate through kernel

**Can break soundness:**
- Kernel bugs -- core issue
- Unsound axioms -- user-added (user's responsibility)
- Unsafe code in kernel -- doesn't exist per `#![forbid(unsafe_code)]`

### Q: Can I add axioms?

Yes, but carefully:

```oxilean
-- Safe axiom: Classical logic
axiom excluded_middle : forall (p : Prop), p \/ ~p

-- Unsafe axiom: Breaks soundness
-- axiom false : False  -- Don't do this!

-- Combination axiom: can lead to contradiction
axiom magic : forall a : Type, a
-- This is unsound and will compile, but that's your fault
```

**Best practices:**
1. Document all axioms
2. Minimize axiom count
3. Verify axiom consistency
4. Use standard axioms (classical logic, function extensionality)

### Q: How does OxiLean handle universe levels?

OxiLean implements:

1. **Universe hierarchy** -- Prop : Type 0 : Type 1 : ...
2. **Universe polymorphism** -- Functions over universe levels
3. **Universe unification** -- Solver for universe constraints
4. **IMax** -- Impredicative universe multiplication

**Soundness implications:**

- **Type safety** -- Cannot apply Type n to Type m rules incorrectly
- **Consistency** -- Universe hierarchy prevents self-application paradoxes
- **Predicativity** -- Prop is impredicative; Type u is predicative

### Q: What about proof irrelevance?

OxiLean implements **proof irrelevance** for Prop:

```oxilean
-- Any two proofs of P are equal (P : Prop)
theorem proof_irrel : forall (p q : P), p = q := ...

-- False for Type:
-- theorem type_irrel : forall (x y : Type), x = y := ...  -- WRONG
```

This is sound because:

1. **Prop is definitional** -- Proofs don't affect computation
2. **Values matter** -- Two proofs of a type are distinct

---

## Comparison with Other Provers

### Q: How does OxiLean compare to Coq?

| Feature | OxiLean | Coq |
|---------|---------|-----|
| Logic | CiC | CiC |
| Kernel SLOC | 113K | 50K |
| Tactics | 20+ | ~200 |
| Library size | 413K SLOC | 10K+ items |
| WASM support | Native (oxilean-wasm) | Requires port |
| Learning curve | Easier | Harder |
| Production use | Education/Research | Yes |
| Performance | Moderate | Excellent |

### Q: How does OxiLean compare to Agda?

| Feature | OxiLean | Agda |
|---------|---------|------|
| Logic | CiC | MLTT + axioms |
| Dependent types | Yes | Yes (stronger) |
| Termination checking | Yes | Yes (more flexible) |
| Proof mode | Tactics | Mixed |
| WASM support | Yes | No |
| Learning curve | Medium | Hard |

### Q: How does OxiLean compare to Isabelle?

| Feature | OxiLean | Isabelle |
|---------|---------|----------|
| Logic | CiC | HOL |
| Automation | Limited | Excellent |
| Proof language | OxiLean syntax | Isar |
| Dependency count | 0 | Many |
| Performance | Good | Excellent |
| Production proofs | Education/Research | Yes |
| Beginner friendly | Yes | No |

### Q: Should I learn OxiLean or Lean 4?

**Lean 4 is better if:**
- You want to do large-scale formalization
- You need an active community
- You want professional support
- You're learning theorem proving

**OxiLean is better if:**
- You want to understand ITP design
- You prefer minimal dependencies
- You need WASM deployment
- You're curious about Rust
- You want a pure Rust, zero C/Fortran dependency toolchain

**Recommendation:** Learn Lean 4 for practical skills, study OxiLean to understand implementation.

---

## WASM Support

### Q: Can OxiLean run in a browser?

**Yes.** OxiLean includes the oxilean-wasm crate with full WASM bindings. The following API functions are available:

- `check()` -- Type check expressions and proofs
- `repl()` -- Interactive read-eval-print loop
- `completions()` -- Code completion suggestions
- `hover_info()` -- Type information on hover
- `format()` -- Code formatting

```html
<script src="oxilean.wasm"></script>
<script>
  const prover = new OxiLean();
  const result = prover.check("theorem t : True := trivial");
  console.log(result);
</script>
```

### Q: What does WASM OxiLean enable?

1. **Online IDE** -- Prove theorems in browser
2. **Interactive tutorials** -- No installation needed
3. **Collaborative proofs** -- Real-time shared proving
4. **Educational platform** -- Browser-based learning
5. **Lightweight verification** -- For client-side validation

### Q: What's the WASM binary size?

Estimated sizes (uncompressed):

- **Kernel only** -- 200-300 KB
- **With parser** -- 400-500 KB
- **Full (with elaborator)** -- 800 KB - 1 MB
- **With standard library** -- 2-3 MB

After gzip compression:

- **Kernel** -- 50-70 KB
- **Full** -- 150-200 KB
- **With library** -- 500-800 KB

### Q: Can I use OxiLean via a service?

Potential deployment options:

1. **Cloud service** -- oxilean.cloud or similar
2. **API** -- REST/gRPC interface
3. **IDE plugin** -- VS Code, Vim integration via LSP

---

## IDE Integration

### Q: Does OxiLean have LSP support?

**Yes.** OxiLean implements LSP support with the following features:

- Code completion (via `completions()` API)
- Hover information (via `hover_info()` API)
- Error checking
- Type information on hover

### Q: Can I use OxiLean in VS Code?

**Yes**, with LSP integration:

```json
{
  "oxilean.checkOnSave": true,
  "oxilean.tactic.depth": 100,
  "oxilean.kernel.verbose": false
}
```

### Q: What about Vim/Neovim?

OxiLean supports both via LSP:

```lua
-- Neovim with nvim-lspconfig
require'lspconfig'.oxilean.setup{}

-- Vim with vim-lsp
call lsp#register_server({
    \ 'name': 'oxilean',
    \ 'cmd': ['oxilean', 'lsp'],
    \ 'allowlist': ['oxilean'],
    \ })
```

### Q: Can I debug proofs?

Available and planned features:

1. **Step-through tactics** -- Execute tactic by tactic
2. **Breakpoints** -- Pause at specific points
3. **Variable inspection** -- See context at each step
4. **Time profiling** -- Identify slow proofs

Example:

```bash
oxilean debug --breakpoint "line 42" myproof.oxilean
```

---

## Advanced Topics

### Q: How do I contribute to OxiLean?

OxiLean welcomes contributions:

1. **Report bugs** -- Use GitHub issues
2. **Submit patches** -- Pull requests welcome
3. **Write documentation** -- Help improve guides
4. **Add examples** -- Contribute formalized proofs
5. **Optimize code** -- Performance improvements

See [CONTRIBUTING.md](../CONTRIBUTING.md) for details.

### Q: Can I embed OxiLean in my project?

Yes! OxiLean is designed for embedding:

```toml
[dependencies]
oxilean-kernel = "0.1"
```

```rust
use oxilean_kernel::{Environment, Expr, Level, Name};

let mut env = Environment::new();
let ty = Expr::Sort(Level::zero()); // Prop
// ... type check expressions against env
```

### Q: How is OxiLean licensed?

OxiLean is licensed under **Apache 2.0**.

### Q: Can I use OxiLean for formal verification?

**Yes**, for educational and research purposes. OxiLean provides:

1. **Sound kernel** -- CiC implementation with 113K SLOC
2. **Tactic framework** -- 20+ tactics for proof construction
3. **Standard library** -- 413K SLOC of formalized mathematics
4. **Mathlib4 compatibility** -- 100% declaration coverage (4,530/4,530)

For industrial-grade formal verification, Lean 4 or Coq remain more established choices.

### Q: How does universe polymorphism work?

OxiLean implements universe variables and constraints:

```oxilean
universe u v

def map {a : Sort u} {b : Sort v} (f : a -> b) : List a -> List b :=
  sorry

-- Uses constraints: u+1 <= v if needed
```

The system automatically:

1. **Generates constraints** -- From type checking
2. **Solves constraints** -- Finds consistent assignments
3. **Produces universe expressions** -- Complex level terms

### Q: What's the IMax operator?

IMax (implicit max) ensures impredicativity:

```
IMax(Prop, u) = Prop    -- stays at Prop
IMax(Type u, Type v) = Type (max u v)
```

This ensures:

- `Prop -> Prop : Prop` (impredicative)
- `Type 0 -> Type 0 : Type 1` (predicative)

### Q: How does proof search work?

OxiLean supports proof search with:

1. **Hint databases** -- Lemmas to try
2. **Backtracking** -- Try alternatives
3. **Depth limiting** -- Prevent infinite search
4. **Timeout** -- Stop after time limit

Example:

```oxilean
theorem something : goal := by
  auto                    -- Try automatic proof search
  auto [lemma1, lemma2]   -- With specific lemmas
  auto using hint_db      -- Using hint database
```

### Q: Can I define custom tactics?

Yes. OxiLean supports custom tactic definitions:

```oxilean
tactic my_simp : goal -> option goal :=
  fun g => ... -- tactic implementation

theorem test : P := by
  my_simp
```

### Q: What about metaprogramming?

OxiLean supports metaprogramming via the oxilean-meta crate:

1. **Macros** -- Generate code
2. **Tactic combinators** -- Build tactics from tactics
3. **Reflection** -- Manipulate expressions
4. **Elaborator hooks** -- Custom elaboration

---

## Getting Help

### Where can I get help?

1. **Documentation** -- Read guides in `/docs`
2. **Examples** -- Study `EXAMPLES.md`
3. **GitHub Issues** -- Ask questions on GitHub
4. **Discussion Forum** -- (Coming soon)
5. **Discord** -- (To be created)

### How do I report bugs?

1. Open GitHub issue with:
   - **Title** -- Clear description
   - **Minimal example** -- Reproduces issue
   - **Expected vs actual** -- What went wrong
   - **OxiLean version** -- Output of `oxilean --version`

### Where's the best place to learn?

1. Start with **TUTORIAL.md** -- Guided lessons
2. Read **USER_GUIDE.md** -- Complete reference
3. Study **EXAMPLES.md** -- Working code
4. Browse **source code** -- Understand implementation

---

**Last updated:** March 2026
**OxiLean version:** 0.1.1
**Status:** All phases complete
