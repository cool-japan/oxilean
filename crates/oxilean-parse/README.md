# oxilean-parse

[![Crates.io](https://img.shields.io/crates/v/oxilean-parse.svg)](https://crates.io/crates/oxilean-parse)
[![Docs.rs](https://docs.rs/oxilean-parse/badge.svg)](https://docs.rs/oxilean-parse)

> **Parser for the OxiLean theorem prover**

Converts OxiLean source text (`.oxilean` files) into an abstract syntax tree (AST). This crate is **untrusted** -- bugs here can cause parse failures but never soundness issues, because the kernel independently type-checks all elaborated terms.

67,822 SLOC -- fully implemented parser with lexer, AST, and error recovery (385 source files, 2,388 tests passing).

## Architecture

```
Source text (.oxilean)
    |
    v
+----------------+
|    Lexer       |  -> Token stream with Span positions
|  (lexer.rs)    |
+-------+--------+
        v
+----------------+
|    Parser      |  -> Surface AST (SurfaceExpr, SurfaceDecl)
| (parser.rs)    |
+----------------+
```

## Module Overview

| Module | Status | Description |
|--------|--------|-------------|
| `tokens` | Implemented | `Token`, `TokenKind`, `Span`, `StringPart` |
| `lexer` | Implemented | Hand-written character-by-character `Lexer` |
| `ast_impl` | Implemented | Surface syntax AST (`SurfaceExpr`, `Decl`, `Located`, `Binder`, ...) |
| `parser_impl` | Implemented | Recursive descent / Pratt `Parser` |
| `error_impl`, `error_recovery`, `diagnostic` | Implemented | `ParseError`, `ParseErrorKind`, recovery strategies, `Diagnostic` |
| `command`, `tactic_parser`, `pattern`, `macro_parser`, `notation_system` | Implemented | Commands, tactic blocks, pattern compilation, macros, notations |
| `module_system`, `module` | Implemented | Import parsing, resolution and graph analysis |
| `prettyprint`, `roundtrip` | Implemented | Pretty printer (`print_decl`, `print_expr`) and round-trip checking |
| `incremental`, `expr_cache`, `indent_tracker`, `wasm_source_map`, `sourcemap` | Implemented | Incremental re-parsing, parse cache, indentation tracking, source maps |

## Features

### Lexer
- UTF-8 identifier support (alpha, beta, Pi, lambda, arrow, turnstile, and other mathematical symbols)
- Line comments (`--`) and nested block comments (`/- ... -/`)
- Indentation tracking for `where` blocks
- Source span (`Span`) for every token (enables precise error reporting)

### Token System
- **Keywords**: `def`, `theorem`, `axiom`, `inductive`, `where`, `match`, `with`, `let`, `in`, `by`, `import`, `universe`, `namespace`, `end`, `if`, `then`, `else`
- **Symbols**: arrow, `=>`, `:`, `;`, `,`, `.`, `|`, `(`, `)`, `{`, `}`, `[`, `]`, `:=`, `_`, `@`, `#`
- **Literals**: `Nat(u64)`, `Float(f64)`, `String(String)`, `Char(char)`
- **Identifiers**: `Ident(String)`

### Surface AST

Every node is wrapped in `Located<T>` (a value plus its `Span`).

```rust,ignore
// Expressions (what users write) -- abridged
pub enum SurfaceExpr {
    Sort(SortKind),
    Var(String),                                   // unresolved name
    App(Box<Located<SurfaceExpr>>, Box<Located<SurfaceExpr>>),
    Lam(Vec<Binder>, Box<Located<SurfaceExpr>>),   // fun x => body
    Pi(Vec<Binder>, Box<Located<SurfaceExpr>>),    // (x : A) -> B
    Let(String, Option<Box<Located<SurfaceExpr>>>, Box<Located<SurfaceExpr>>, Box<Located<SurfaceExpr>>),
    Lit(Literal),                                  // 42, "hello"
    Hole,                                          // _
    Proj(Box<Located<SurfaceExpr>>, String),       // e.field
    Match(Box<Located<SurfaceExpr>>, Vec<MatchArm>),
    ByTactic(Vec<Located<TacticRef>>),             // by tac1; tac2
    // ... also Ann, If, Do, Have, Show, Calc, ListLit, Tuple, ...
}

// Declarations (top-level items) -- abridged
pub enum Decl {
    Axiom { .. },
    Definition { .. },
    Theorem { .. },
    Inductive { .. },
    Import { .. },
    Namespace { .. },
    Structure { .. },
    // ... also ClassDecl, InstanceDecl, SectionDecl, Open, Mutual, ...
}
```

### Parser
- **Pratt parsing** for expression operators (no external dependencies)
- Error recovery: the parser skips to the next declaration keyword (`axiom`, `def`/`definition`, `theorem`, `lemma`) or separator

### Using the parser

```rust,ignore
use oxilean_parse::{Lexer, Parser};

let tokens = Lexer::new("def id (n : Nat) : Nat := n").tokenize();
let mut parser = Parser::new(tokens);
let decl = parser.parse_decl()?; // Located<Decl>
```

## Dependencies

- `oxilean-kernel` -- for `Name`, `Level`, `Literal` types

## Example (Target Syntax)

```oxilean
-- Natural number addition
def Nat.add (n m : Nat) : Nat :=
  match n with
  | Nat.zero   => m
  | Nat.succ k => Nat.succ (Nat.add k m)

-- Commutativity of addition (proof)
theorem Nat.add_comm (n m : Nat) : Nat.add n m = Nat.add m n := by
  induction n with
  | zero => simp [Nat.add]
  | succ k ih => simp [Nat.add, ih]
```

## Testing

```bash
cargo test -p oxilean-parse
```

## License

Copyright COOLJAPAN OU (Team KitaSan). Apache-2.0 -- See [LICENSE](../../LICENSE) for details.
