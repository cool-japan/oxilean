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

# Multi-file crate-wide documentation with cross-references
oxilean-doc --crate . -o docs/
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
| `main.rs` | CLI entry point via `clap` |

## Crate details

```toml
[dependencies]
oxilean-doc = "0.1.4"
```

- License: Apache-2.0
- Edition: 2021
- Rust version: 1.70+
- Depends on: `oxilean-parse` (for source parsing)

Copyright 2026 COOLJAPAN OU (Team Kitasan). Apache-2.0.
