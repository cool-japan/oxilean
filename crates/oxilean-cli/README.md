# oxilean-cli

[![Crates.io](https://img.shields.io/crates/v/oxilean-cli.svg)](https://crates.io/crates/oxilean-cli)
[![Docs.rs](https://docs.rs/oxilean-cli/badge.svg)](https://docs.rs/oxilean-cli)

> **Command-line interface for the OxiLean theorem prover**

The CLI provides the user-facing entry point for checking proofs, running the REPL, and managing OxiLean projects.

73,609 SLOC -- fully implemented CLI with argument parsing, subcommand dispatch, file checking, and interactive REPL (298 source files, 2,205 tests passing).

## Usage

```bash
# Show version (also: oxilean --version, oxilean -V)
oxilean version

# Show help (also: oxilean --help, oxilean -h)
oxilean help

# Check a file
oxilean check file.oxilean

# Start interactive REPL (also the default with no arguments)
oxilean repl

# Start the LSP server on stdio
oxilean lsp

# Serve the wasm playground
oxilean playground
```

`--help` / `-h` and `--version` / `-V` are handled by the binary and exit 0. `-v` is intentionally not an alias for version (it means `--verbose`). Any other first argument is treated as a file name and checked, so `oxilean file.oxilean` is the same as `oxilean check file.oxilean`.

## Commands

| Command | Status | Description |
|---------|--------|-------------|
| `check <file>` | Implemented | Type-check an `.oxilean` source file |
| `repl` | Implemented | Interactive proof REPL (default with no arguments) |
| `lsp` / `serve` | Implemented | Language server on stdio |
| `playground` | Implemented | Static-file server for the wasm playground (pure `std::net`) |
| `version` / `--version` / `-V` | Implemented | Print version information |
| `help` / `--help` / `-h` | Implemented | Print usage information |

The crate also contains build, bench, doc, format, export, watch and project-init modules, with a `CliCommand` parser and help table for them; these are not yet dispatched by the `oxilean` binary's entry point (`src/main/functions.rs`).

## Architecture

```
CLI Argument Parsing
    |
    +-- "check" <file>
    |       |
    |       +-- Read file
    |       +-- Lex -> Token stream       (oxilean-parse)
    |       +-- Parse -> Surface AST       (oxilean-parse)
    |       +-- Elaborate -> Kernel Exprs  (oxilean-elab)
    |       +-- Type-check via Kernel      (oxilean-kernel)
    |       +-- Report success / errors
    |
    +-- "repl"
    |       |
    |       +-- Interactive loop:
    |           +-- Read input
    |           +-- Parse + Elaborate
    |           +-- Display goals / results
    |           +-- Accept tactic commands
    |
    +-- "lsp" | "serve"  -> LSP server on stdio
    |
    +-- "playground"     -> static-file server for the wasm playground
    |
    +-- "version" | "--version" | "-V"
    |       +-- Print version string
    |
    +-- "help" | "--help" | "-h"
            +-- Print usage information
```

## Dependencies

- `oxilean-kernel` -- core type checking
- `oxilean-parse` -- lexing and parsing
- `oxilean-elab` -- elaboration and tactics

## Building

```bash
# Build the CLI binary
cargo build -p oxilean-cli

# Build in release mode
cargo build -p oxilean-cli --release

# Run directly
cargo run --bin oxilean -- version
```

## Testing

```bash
cargo test -p oxilean-cli
```

## License

Copyright COOLJAPAN OU (Team KitaSan). Apache-2.0 -- See [LICENSE](../../LICENSE) for details.
