# oxilean-parse

[![Crates.io](https://img.shields.io/crates/v/oxilean-parse.svg)](https://crates.io/crates/oxilean-parse)
[![Docs.rs](https://docs.rs/oxilean-parse/badge.svg)](https://docs.rs/oxilean-parse)

> **Parser for the OxiLean theorem prover**

Converts OxiLean source text (`.oxilean` files) into an abstract syntax tree (AST). This crate is **untrusted** -- bugs here can cause parse failures but never soundness issues, because the kernel independently type-checks all elaborated terms.

62,293 SLOC -- fully implemented parser with lexer, AST, and error recovery (335 source files, 2,153 tests passing).

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
| `token.rs` | Implemented | Token definitions (keywords, symbols, literals, identifiers) |
| `lexer.rs` | Implemented | Hand-written character-by-character lexer |
| `ast.rs` | Implemented | Surface syntax AST types |
| `parser.rs` | Implemented | Recursive descent / Pratt parser |
| `error.rs` | Implemented | Parse error type with `Display` and `Error` |

## Features

### Lexer
- UTF-8 identifier support (alpha, beta, Pi, lambda, arrow, turnstile, and other mathematical symbols)
- Line comments (`--`) and nested block comments (`/- ... -/`)
- Indentation tracking for `where` blocks
- Source span (`Span`) for every token (enables precise error reporting)

### Token System
- **Keywords**: `def`, `theorem`, `axiom`, `inductive`, `where`, `match`, `with`, `let`, `in`, `by`, `import`, `universe`, `namespace`, `end`, `if`, `then`, `else`
- **Symbols**: arrow, `=>`, `:`, `;`, `,`, `.`, `|`, `(`, `)`, `{`, `}`, `[`, `]`, `:=`, `_`, `@`, `#`
- **Literals**: `NatLit(u64)`, `StrLit(String)`
- **Identifiers**: `Ident(String)`

### Surface AST

```rust
// Expressions (what users write)
pub enum SurfaceExpr {
    Var(String),                    // unresolved name
    App(Box<SE>, Box<SE>),          // f a
    Lam(Vec<Binder>, Box<SE>),      // fun x => body
    Pi(Vec<Binder>, Box<SE>),       // (x : A) -> B
    Arrow(Box<SE>, Box<SE>),        // A -> B (non-dependent)
    Let(String, Option<Box<SE>>, Box<SE>, Box<SE>),
    Match(Box<SE>, Vec<MatchArm>),  // match e with | ...
    ByTactic(Vec<Tactic>),          // by { tac1; tac2; ... }
    Lit(Literal),                   // 42, "hello"
    Hole,                           // _
    Parens(Box<SE>),
    Proj(Box<SE>, String),          // e.field
}

// Declarations (top-level items)
pub enum SurfaceDecl {
    Def { name, params, ret_ty, body },
    Theorem { name, params, ty, proof },
    Axiom { name, params, ty },
    Inductive { name, params, ty, ctors },
    Import(String),
    Universe(Vec<String>),
}
```

### Parser
- **Pratt parsing** for expression operators (no external dependencies)
- Operator precedence: arrows (right-assoc, prec 25), application (left-assoc, prec 1024)
- Error recovery: synchronize on `def`, `theorem`, `axiom`, `inductive`, `EOF`
- Multi-error reporting (continues parsing after an error)

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

Copyright COOLJAPAN OU (Team Kitasan). Apache-2.0 -- See [LICENSE](../../LICENSE) for details.
