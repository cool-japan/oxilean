# oxilean-doc

Documentation generator for OxiLean projects.

## Purpose

`oxilean-doc` extracts docstrings and signatures from parsed OxiLean/Lean4
source files and renders structured HTML documentation — analogous to `rustdoc`
for Lean4 source.

## Usage

```bash
# Single file → stdout
oxilean-doc src/Main.lean

# Single file → HTML output
oxilean-doc src/Main.lean -o docs/Main.html

# Multi-file documentation with cross-references: one page per module + index.html
oxilean-doc multi --output-dir docs/ src/Foo.lean src/Bar.lean
```

## Features

- Docstring extraction from `SurfaceDecl` nodes (docstring + name + type)
- Single-page HTML output (no template engine; pure-Rust `write!`)
- Multi-file crate-wide generation with intra-crate cross-reference links
- Client-side search index (`search-index.json`) with symbol/kind/anchor
- Pure-Rust mini-Markdown renderer: bold, italic, inline code, fenced code,
  links, paragraphs
- Light/dark theming via CSS custom properties (`prefers-color-scheme` default;
  `localStorage` JS toggle)
- Relative links throughout — output is hosting-portable

## Architecture

| Module | Role |
|---|---|
| `extractor.rs` | Walk `SurfaceDecl` nodes, extract docstring + name + type string |
| `renderer.rs` | Single-page HTML renderer; embedded CSS with light/dark theme |
| `symbol_index.rs` | Symbol table for cross-references and search index |
| `multifile.rs` | Multi-file mode: one page per module + index page |
| `markdown.rs` | Pure-Rust mini-Markdown renderer |
| `walker.rs` | Group declarations by module for multi-file output |
| `cross_ref.rs` | Resolve `[Name]` references in doc-comments to relative links |
| `error.rs` | Error type for the multi-file generation pipeline |
| `main.rs` | CLI entry point via `clap` (`multi` subcommand; single-file mode otherwise) |

## Crate details

```toml
[dependencies]
oxilean-doc = "0.1.4"
```

- License: Apache-2.0
- Edition: 2021
- Rust version: 1.89+
- Depends on: `oxilean-parse` (for source parsing)

Copyright 2026 COOLJAPAN OU (Team Kitasan). Apache-2.0.
