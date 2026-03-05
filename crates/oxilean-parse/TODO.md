# oxilean-parse — TODO

> Task list for the parser crate.
> Last updated: 2026-03-05

## ✅ Completed

- [x] Module structure (`token`, `lexer`, `ast`, `parser`, `error`)
- [x] `ParseError` type with `Display` and `Error` trait
- [x] Re-exports in `lib.rs`

---

## ✅ Completed (Phase 2): Parser Implementation

### Token Definitions (`token_impl.rs` — 498 lines)
- [x] `Token` enum with 60+ variants (all keywords, symbols, literals)
- [x] Keyword set: `def`, `theorem`, `axiom`, `inductive`, `structure`, `class`, `instance`, `where`, `match`, `with`, `let`, `in`, `by`, `import`, `universe`, `namespace`, `end`, `if`, `then`, `else`, `do`, `have`, `suffices`, `show`, `forall`, `exists`, `fun`, `return`, `opaque`, `section`, `open`, `export`, `attribute`, `variable`, `set_option`, etc.
- [x] Symbol tokens: `→`/`->`, `=>`, `:`, `;`, `,`, `.`, `..`, `|`, `@`, `#`, `+`, `-`, `*`, `/`, `%`, `^`, `:=`, `&&`, `||`, `!`, `<-`, `?`, parens, braces, brackets, angle brackets `⟨⟩`
- [x] Literal tokens: `NatLit`, `FloatLit`, `StrLit`, `CharLit`, `BoolLit`, `InterpStrLit`
- [x] Identifier token: `Ident(String)`
- [x] `Eof` token
- [x] `Span` type for source positions (start, end, line, column)
- [x] `TokenInfo` struct (token + span + trivia)

### Lexer (`lexer_impl.rs` — 1363 lines)
- [x] Character-by-character lexer struct
- [x] UTF-8 identifier scanning (α, β, Π, λ, →, ⊢, subscripts, etc.)
- [x] Number literal scanning (decimal, `0x` hex, `0b` binary, `0o` octal, separators)
- [x] Float literal scanning (`3.14`, `1.0e10`, `1.5e-3`)
- [x] String literal scanning (with escape sequences: `\n`, `\t`, `\x41`, `\u{...}`, `\0`)
- [x] Character literal scanning
- [x] String interpolation (`s!"..."` — nested token parsing)
- [x] Line comments: `--` to end of line
- [x] Nested block comments: `/- ... -/`
- [x] Whitespace and newline handling
- [x] Unicode symbol scanning: `→`, `⇒`, `∧`, `∨`, `¬`, `↔`, `∀`, `∃`, `λ`, `⟨`, `⟩`, `≤`, `≥`, `≠`
- [x] `Span` attachment to every token
- [x] `tokenize()` — full input tokenization
- [x] 30+ unit tests

### Surface AST (`ast_impl.rs` — 1551 lines)
- [x] `SurfaceExpr` enum — 27 variants (Var, App, Lam, Pi, Arrow, Let, Match, ByTactic, Lit, Hole, Parens, Proj, If, Do, Have, Suffices, Show, Return, AnonymousCtor, NamedArg, ListLit, Tuple, StringInterp, Range, Calc, etc.)
- [x] `Command` enum — 16 variants (Def, Theorem, Axiom, Inductive, Structure, Class, Instance, Import, Namespace, Section, Open, Universe, Variable, Attribute, HashCmd, SetOption, etc.)
- [x] `Binder` type (name, optional type, binder info: explicit/implicit/strict-implicit/inst-implicit)
- [x] `MatchArm` type (patterns, guard, body)
- [x] `Tactic` enum — 40+ variants (Intro, Apply, Exact, Rewrite, Simp, Cases, Induction, Have, Let, Constructor, Exists, Exfalso, Calc, Conv, etc.)
- [x] `Pattern` enum (Wildcard, Var, Constructor, Literal, As, Or, Inaccessible, Array)
- [x] `Constructor` type (name, type)
- [x] All types annotated with `Spanned<T>` wrapper
- [x] `Display` for all types

### Parser (`parser_impl.rs` — 3641 lines)
- [x] `Parser` struct holding token stream and position
- [x] `parse_decl()` → `Command` (dispatches to 17 declaration kinds)
  - [x] `def` declarations
  - [x] `theorem` / `lemma` declarations
  - [x] `axiom` declarations
  - [x] `inductive` declarations
  - [x] `structure` declarations
  - [x] `class` declarations
  - [x] `instance` declarations
  - [x] `import` statements
  - [x] `universe` declarations
  - [x] `namespace` / `section` / `end`
  - [x] `open` / `export`
  - [x] `variable` / `parameter`
  - [x] `attribute` (with `@[...]` syntax)
  - [x] `set_option`
  - [x] `#check` / `#eval` / `#print` hash commands
- [x] `parse_expr()` — Pratt parser for expressions
  - [x] Prefix: atoms, parenthesized expressions, lambda, pi, let, match, if, hole, negation, not
  - [x] Infix: application (100), arrows (1), iff (5), or (8), and (12), comparisons (20), add/sub (30), mul/div (40), power (50)
  - [x] Operator precedence handling (priority climbing)
- [x] `parse_primary()` — identifiers, literals, parenthesized, holes, anonymous constructors, list literals
- [x] `parse_binders()` — `(x : A)`, `{x : A}`, `⦃x : A⦄`, `[x : A]`, bare identifiers
- [x] `parse_match()` — match expression with arms and patterns
- [x] `parse_tactic_block()` — `by { tac₁; tac₂ }` and tactic combinators
- [x] `parse_do_block()` — do notation
- [x] `parse_if()`, `parse_let()`, `parse_lambda()`, `parse_forall()`
- [x] Dot projection (`e.field`)
- [x] Named arguments

### Tactic Parser (`tactic_parser.rs` — 2657 lines)
- [x] 40+ tactic variants fully parseable
- [x] Tactic combinators: `;` sequence, `<|>` alternative, `<;>` all-goals
- [x] Structured tactics: `intro`, `apply`, `exact`, `rewrite`/`rw`, `simp`, `cases`, `induction`, `have`, `let`, `suffices`, `conv`, `calc`
- [x] `by` block parsing

### Command Parser (`command_parser.rs` — 2608 lines)
- [x] Full command parsing (20+ command types)
- [x] Attribute parsing, binder collection, dot name parsing

### Error Recovery & Diagnostics (`error_impl.rs` — 1044 lines)
- [x] `ParseError` enum with 8 error kinds
- [x] `ErrorCode` — 15+ error codes
- [x] `CodeFix` — fix suggestions (span + replacement)
- [x] `SyncPoint` — synchronization on `def`, `theorem`, `axiom`, `inductive`, `EOF`
- [x] `ErrorReporter` — multi-error collection, filtering, sorting, merge, summary
- [x] Rustc-style formatted error output
- [x] Skip-to-sync on parse error

### Pattern Match Compiler (`pattern_compiler.rs` — 2013 lines)
- [x] Pattern matrix → decision tree compilation
- [x] Constructor specialization
- [x] Default matrix extraction
- [x] Best column selection heuristic
- [x] Exhaustiveness checking
- [x] Redundancy checking

### Macro System (`macro_parser.rs` — 1419 lines)
- [x] Macro hygiene
- [x] Syntax pattern → template matching
- [x] `syntax` and `macro_rules` declaration parsing
- [x] Pattern matching + template expansion
- [x] Depth-limited recursive expansion

### Notation System (`notation.rs` — 1295 lines)
- [x] Operator kinds (Prefix, Infixl, Infixr, Postfix)
- [x] Scoped notation registration
- [x] Priority comparison
- [x] Lookup methods

### Module System (`module.rs` — 2068 lines)
- [x] Module definition (name, imports, exports, visibility)
- [x] Namespace scope (nesting, aliases)
- [x] Multi-module management
- [x] Dependency graph with cycle detection (DFS) and topological sort (Kahn)

### Pretty Printer (`pretty_printer.rs` — 1695 lines)
- [x] Full expression and declaration pretty printing (all 27 `SurfaceExpr` variants)
- [x] Unicode/ASCII toggle (`→`/`->`, `∀`/`forall`, `λ`/`fun`)
- [x] Multi-line formatting, auto line breaks

### Source Map (`source_map.rs` — 1081 lines)
- [x] 11 semantic highlight kinds (Definition, Reference, TypeAnnotation, Binder, etc.)
- [x] Source map construction and query
- [x] Byte offset ↔ line/column conversion
- [x] IDE hover info, semantic tokens (LSP-compatible)

### REPL Parser (`repl_parser.rs` — 197 lines)
- [x] REPL command parsing (`:quit`, `:type`, `:load`, `:check`, `:env`, `:clear`)
- [x] Multi-line continuation detection (bracket balancing)

### Testing
- [x] Lexer unit tests: keywords, symbols, identifiers, literals, comments (30+ tests)
- [x] Lexer Unicode tests: mathematical symbols, subscripts
- [x] Parser unit tests: each declaration type
- [x] Parser expression tests: applications, arrows, lambdas, holes
- [x] Error recovery tests: malformed input with multiple errors
- [x] Pattern compiler tests
- [x] Tactic parser tests
- [x] Pretty printer tests
- [x] Module system tests

---

## ⚠️ Partially Implemented

- [x] Indentation tracking for `where` blocks
- [x] Guard expressions in match arms (elaborated via `ite` desugaring in `elaborate.rs`)
- [x] `parse_file()` top-level wrapper (implemented in `lib.rs:589`)
- [x] Round-trip tests: parse → pretty-print → parse = same AST
- [x] Golden file tests: `.oxilean` files with expected AST output

---

## ⚪ Future Enhancements

- [x] Incremental re-lexing / re-parsing for LSP — `incremental.rs` (IncrementalParser, TextChange, VersionedSource, 10 tests)
- [x] Expression cache — `expr_cache.rs` (StringInterner, ParseCache LRU, DeclHash, 8 tests)
- [x] Source map for WASM target — `wasm_source_map.rs` (VLQ encoding, SourceMap, WasmSourceMapBuilder, 8 tests)
- [x] Indentation tracking for `where` blocks — `indent_tracker.rs` (IndentStack, WhereBlockTracker, 8 tests)
- [x] Round-trip tests — `roundtrip.rs` (RoundTripChecker, GoldenTestSuite, 8 tests)

---

## 📝 Note

The original stub files (`token.rs`, `lexer.rs`, `ast.rs`, `parser.rs`, `error.rs`) remain as empty compatibility shims. All real implementations are in the `_impl` suffixed files and additional modules (`tactic_parser.rs`, `command_parser.rs`, `pattern_compiler.rs`, `macro_parser.rs`, `notation.rs`, `module.rs`, `pretty_printer.rs`, `source_map.rs`, `repl_parser.rs`).
