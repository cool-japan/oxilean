//! `oxilean-doc` — documentation generator for OxiLean (`.lean`) source files.
//!
//! Two modes of operation:
//!
//! ## Single-file mode (default)
//!
//! Reads a single `.lean` file, extracts all named declarations along with
//! their doc-comments, and emits a self-contained single-page HTML file.
//!
//! ```text
//! oxilean-doc Foo.lean                    # prints HTML to stdout
//! oxilean-doc Foo.lean -o Foo.html        # writes to file
//! oxilean-doc Foo.lean --title "Foo API"  # override page title
//! ```
//!
//! ## Multi-file mode
//!
//! Reads one or more `.lean` source files, groups their declarations by module
//! namespace, resolves cross-references in doc-comments, and writes one HTML
//! page per module plus an `index.html` to an output directory.
//!
//! ```text
//! oxilean-doc multi --output-dir docs/ Foo.lean Bar.lean
//! ```

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod cross_ref;
mod error;
mod extractor;
mod markdown;
mod multifile;
mod renderer;
mod symbol_index;
mod walker;

// ---------------------------------------------------------------------------
// CLI definition
// ---------------------------------------------------------------------------

#[derive(Parser)]
#[command(
    name = "oxilean-doc",
    about = "Generate HTML documentation for OxiLean (.lean) source files",
    long_about = None,
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,

    /// Input `.lean` source file (single-file mode, used when no subcommand given).
    #[arg(value_name = "INPUT", required_unless_present = "command")]
    input: Option<PathBuf>,

    /// Output HTML file (single-file mode).  Defaults to stdout when omitted.
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Page title (single-file mode).  Defaults to the input file's stem.
    #[arg(long, value_name = "TITLE")]
    title: Option<String>,
}

#[derive(Subcommand)]
enum Command {
    /// Generate multi-file HTML docs from one or more .lean source files.
    Multi {
        /// One or more `.lean` source files to document.
        #[arg(value_name = "INPUT", required = true)]
        inputs: Vec<PathBuf>,

        /// Output directory for the generated HTML files.
        #[arg(short = 'd', long, value_name = "DIR", default_value = "docs")]
        output_dir: PathBuf,
    },
}

// ---------------------------------------------------------------------------
// Entry points
// ---------------------------------------------------------------------------

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Multi { inputs, output_dir }) => run_multi(inputs, &output_dir),

        None => {
            // Single-file mode — `input` is guaranteed by `required_unless_present`.
            let input = cli.input.expect("input required in single-file mode");
            run_single(input, cli.output, cli.title)
        }
    }
}

// ---------------------------------------------------------------------------
// Single-file mode
// ---------------------------------------------------------------------------

fn run_single(
    input: PathBuf,
    output: Option<PathBuf>,
    title_override: Option<String>,
) -> Result<()> {
    let source =
        std::fs::read_to_string(&input).with_context(|| format!("reading {}", input.display()))?;

    let title = title_override.unwrap_or_else(|| {
        input
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Documentation")
            .to_string()
    });

    let items = extractor::extract(&source)
        .with_context(|| format!("extracting declarations from {}", input.display()))?;

    let html = renderer::render_html(&title, &items);

    match output {
        Some(ref path) => std::fs::write(path, &html)
            .with_context(|| format!("writing HTML to {}", path.display()))?,
        None => print!("{html}"),
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Multi-file mode
// ---------------------------------------------------------------------------

fn run_multi(inputs: Vec<PathBuf>, output_dir: &std::path::Path) -> Result<()> {
    let mut all_items = Vec::new();

    for path in &inputs {
        let source =
            std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        let items = extractor::extract(&source)
            .with_context(|| format!("extracting declarations from {}", path.display()))?;
        all_items.extend(items);
    }

    multifile::generate_multi_file(&all_items, output_dir).map_err(|e| anyhow::anyhow!("{e}"))?;

    eprintln!(
        "Generated docs for {} file(s) in {}",
        inputs.len(),
        output_dir.display()
    );

    Ok(())
}

// ---------------------------------------------------------------------------
// Integration tests (temp-file based)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::io::Write;

    // Use the simplified Lean subset that `oxilean_parse::Parser` supports:
    // no binder parameters in def/theorem, simple types and values.
    const LEAN_FIXTURE: &str = r#"
/-- A zero constant. -/
def zero_nat : Nat := 0

/-- A simple axiom. -/
axiom nat_axiom : Nat
"#;

    #[test]
    fn test_roundtrip_via_temp_file() {
        // Write fixture to a temp file.
        let mut tmp = tempfile::Builder::new()
            .suffix(".lean")
            .tempfile()
            .expect("create temp file");
        tmp.write_all(LEAN_FIXTURE.as_bytes())
            .expect("write fixture");
        tmp.flush().expect("flush fixture");

        // Parse & render.
        let source = std::fs::read_to_string(tmp.path()).expect("read temp file");
        let items = crate::extractor::extract(&source).expect("extract");
        let html = crate::renderer::render_html("FixtureModule", &items);

        // Basic HTML sanity checks.
        assert!(html.contains("<!DOCTYPE html>"), "missing DOCTYPE");
        assert!(
            html.contains("<title>FixtureModule</title>"),
            "missing title"
        );
        assert!(html.contains("zero_nat"), "expected zero_nat in output");
    }

    #[test]
    fn test_extract_and_render_empty_file() {
        let items = crate::extractor::extract("").expect("empty extract");
        let html = crate::renderer::render_html("Empty", &items);
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("No declarations found."));
    }

    #[test]
    fn test_multi_file_roundtrip_via_temp_files() {
        // Write two fixture files, generate multi-file docs, verify output.
        let dir = std::env::temp_dir().join("oxilean_doc_main_test_multi");
        std::fs::remove_dir_all(&dir).ok();

        let mut tmp_a = tempfile::Builder::new()
            .suffix(".lean")
            .tempfile()
            .expect("create temp file A");
        tmp_a
            .write_all(b"/-- Adds two numbers. -/\ndef add_two : Nat := 2")
            .expect("write A");
        tmp_a.flush().expect("flush A");

        let items =
            crate::extractor::extract(&std::fs::read_to_string(tmp_a.path()).expect("read A"))
                .expect("extract A");

        crate::multifile::generate_multi_file(&items, &dir).expect("multi-file generation");

        assert!(
            dir.join("index.html").exists(),
            "index.html should be generated"
        );
    }
}
