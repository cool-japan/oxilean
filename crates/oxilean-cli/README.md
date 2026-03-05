# oxilean-cli

[![Crates.io](https://img.shields.io/crates/v/oxilean-cli.svg)](https://crates.io/crates/oxilean-cli)
[![Docs.rs](https://docs.rs/oxilean-cli/badge.svg)](https://docs.rs/oxilean-cli)

> **Command-line interface for the OxiLean theorem prover**

The CLI provides the user-facing entry point for checking proofs, running the REPL, and managing OxiLean projects.

64,163 SLOC -- fully implemented CLI with argument parsing, subcommand dispatch, file checking, and interactive REPL.

## Usage

```bash
# Show version
oxilean --version
oxilean -v

# Show help
oxilean --help
oxilean -h

# Check a file
oxilean check file.oxilean

# Start interactive REPL
oxilean repl
```

## Commands

| Command | Status | Description |
|---------|--------|-------------|
| `check <file>` | Implemented | Type-check an `.oxilean` source file |
| `repl` | Implemented | Interactive proof REPL |
| `--version` / `-v` | Implemented | Print version information |
| `--help` / `-h` | Implemented | Print usage information |

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
    +-- "--version"
    |       +-- Print version string
    |
    +-- "--help"
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
cargo run --bin oxilean -- --version
```

## Testing

```bash
cargo test -p oxilean-cli
```

## License

Copyright COOLJAPAN OU (Team Kitasan). Apache-2.0 -- See [LICENSE](../../LICENSE) for details.
