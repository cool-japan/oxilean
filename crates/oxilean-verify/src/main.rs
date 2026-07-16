//! The `oxilean-verify` binary: drive the streaming verify engine over one or
//! more lean4export NDJSON files, print per-declaration lines and a summary, and
//! exit with the product's three-way exit code.
//!
//! See [`oxilean_verify::cli`] for the exit-code contract and the CLI surface.
//! This entry point is deliberately thin: parse, then delegate to the engine,
//! and translate outcomes into I/O and an exit code.

#![forbid(unsafe_code)]

use std::io::{BufReader, Read, Write};
use std::process::ExitCode;

use oxilean_verify::cli::{
    help_text, parse_args, render_decl_line, render_summary_line, version_text, Command, RunConfig,
    UsageError,
};
use oxilean_verify::engine::{verify_stream, VerifyError, VerifyOptions};
use oxilean_verify::report::{render_report, InputMeta, ReportAccumulator};
use oxilean_verify::sha256::Sha256;
use oxilean_verify::{Summary, VERSION};

use oxilean_export::Limits;

/// Exit codes, per the engineering brief §7.
mod exit {
    /// Ran to completion, zero rejected.
    pub const OK: u8 = 0;
    /// At least one declaration rejected (the alarm).
    pub const REJECTED: u8 = 1;
    /// Usage error, I/O error, or malformed export input.
    pub const USAGE_OR_IO: u8 = 2;
}

fn main() -> ExitCode {
    let raw: Vec<String> = std::env::args().skip(1).collect();

    // Colour is on by default only when stdout is a terminal and NO_COLOR is
    // unset. We do not link a tty crate (zero-dep product), so we approximate
    // "is a tty" conservatively: honour NO_COLOR, otherwise default to colour
    // on. The `--no-color` flag always wins. (An explicit env override keeps
    // piped output clean for anyone who sets NO_COLOR.)
    let no_color_env = std::env::var_os("NO_COLOR").is_some();
    let color_default = !no_color_env;

    let command = match parse_args(&raw, color_default) {
        Ok(c) => c,
        Err(e) => return usage_error(&e),
    };

    match command {
        Command::Help => {
            print!("{}", help_text());
            ExitCode::from(exit::OK)
        }
        Command::Version => {
            print!("{}", version_text());
            ExitCode::from(exit::OK)
        }
        Command::Run(cfg) => run_on_verify_thread(cfg),
    }
}

/// Run the verification on a dedicated thread with a large, configurable
/// stack (C17). Deep Lean-core reduction chains legitimately recurse far
/// beyond the 8 MiB OS default; reserving (not committing) a large stack via
/// `std::thread::Builder::stack_size` removes the need for any `ulimit -s`
/// incantation. Spawn/join failures map to exit code 2 (an environment
/// problem, never a verdict).
fn run_on_verify_thread(cfg: RunConfig) -> ExitCode {
    let stack_size_mib = cfg.stack_size_mib;
    let stack_bytes = stack_size_mib
        .saturating_mul(1024 * 1024)
        .min(usize::MAX as u64) as usize;
    let builder = std::thread::Builder::new()
        .name("oxilean-verify".to_string())
        .stack_size(stack_bytes);
    match builder.spawn(move || run(cfg)) {
        Ok(handle) => match handle.join() {
            Ok(code) => code,
            Err(_) => {
                eprintln!("oxilean-verify: internal error: verification thread panicked");
                ExitCode::from(exit::USAGE_OR_IO)
            }
        },
        Err(e) => {
            eprintln!(
                "oxilean-verify: cannot spawn the verification thread \
                 (stack size {stack_size_mib} MiB): {e}"
            );
            ExitCode::from(exit::USAGE_OR_IO)
        }
    }
}

/// Print a usage error to stderr and return exit code 2.
fn usage_error(e: &UsageError) -> ExitCode {
    eprintln!("oxilean-verify: {}", e.message);
    eprintln!("try 'oxilean-verify --help'");
    ExitCode::from(exit::USAGE_OR_IO)
}

/// The main run path over one or more files.
fn run(cfg: RunConfig) -> ExitCode {
    let stdout = std::io::stdout();
    let stderr = std::io::stderr();

    let limits = Limits {
        materialize_budget: cfg.limits.budget(),
        decl_materialize_budget: cfg.limits.decl_budget(),
    };

    // When the JSON report is written to stdout (`--json -`), stdout must be
    // pure JSON, so all human-readable lines (per-decl stream, summary,
    // multi-file headers) are routed to stderr instead. Otherwise they go to
    // stdout as usual.
    let human_to_stderr = cfg.json_path.as_deref() == Some("-");

    // Aggregate the exit-relevant state across all files.
    let mut any_rejected = false;
    // One report per invocation: when `--json` is given we accumulate the
    // three-bucket totals and the unsupported/rejected lists across all input
    // files into a single report. The `input` fingerprint (path/sha256/size) is
    // exact for the common single-file case and summarised for many files.
    let single_file = cfg.files.len() == 1;
    let mut json_accumulator = ReportAccumulator::new(cfg.json_full);
    let mut json_summary = Summary::default();
    let mut json_input: Option<InputMeta> = None;
    let mut json_pins: Option<oxilean_verify::EnvironmentPins> = None;

    for path in &cfg.files {
        // Fingerprint the file and feed the engine WITHOUT holding it in memory:
        // a whole-corpus export (Mathlib is ~6 GiB) must not sit in RAM
        // alongside the growing environment. The sha256 is computed in a first
        // streaming pass; the engine then streams the file a second time. Both
        // passes use a bounded buffer, so CLI memory is O(1) in the file size —
        // only the engine's environment (structurally shared) grows.
        let size_bytes = match std::fs::metadata(path) {
            Ok(m) => m.len(),
            Err(e) => {
                let mut err = stderr.lock();
                let _ = writeln!(err, "oxilean-verify: cannot read '{path}': {e}");
                return ExitCode::from(exit::USAGE_OR_IO);
            }
        };
        let sha = {
            let file = match std::fs::File::open(path) {
                Ok(f) => f,
                Err(e) => {
                    let mut err = stderr.lock();
                    let _ = writeln!(err, "oxilean-verify: cannot read '{path}': {e}");
                    return ExitCode::from(exit::USAGE_OR_IO);
                }
            };
            let mut reader = BufReader::new(file);
            let mut h = Sha256::new();
            let mut buf = [0u8; 1 << 16];
            loop {
                match reader.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => h.update(&buf[..n]),
                    Err(e) => {
                        let mut err = stderr.lock();
                        let _ = writeln!(err, "oxilean-verify: read error '{path}': {e}");
                        return ExitCode::from(exit::USAGE_OR_IO);
                    }
                }
            }
            h.finalize_hex()
        };

        if cfg.files.len() > 1 && !cfg.quiet {
            let mut h = human_writer(&stdout, &stderr, human_to_stderr);
            let _ = writeln!(h, "== {path} ==");
        }

        // Feed the engine by streaming the file directly (a second pass); the
        // per-decl callback streams a line live (true streaming: each verdict is
        // written as the kernel produces it) and feeds the JSON accumulator.
        let engine_file = match std::fs::File::open(path) {
            Ok(f) => f,
            Err(e) => {
                let mut err = stderr.lock();
                let _ = writeln!(err, "oxilean-verify: cannot read '{path}': {e}");
                return ExitCode::from(exit::USAGE_OR_IO);
            }
        };
        let reader = BufReader::new(engine_file);
        let need_json = cfg.json_path.is_some();
        let stream_lines = !cfg.quiet;
        let color = cfg.color;
        let result = {
            let mut human = human_writer(&stdout, &stderr, human_to_stderr);
            verify_stream(
                reader,
                VERSION,
                VerifyOptions {
                    limits,
                    fail_fast: cfg.fail_fast,
                    per_decl_fuel: cfg.limits.decl_fuel(),
                    per_decl_time_budget: cfg.limits.decl_time_budget(),
                },
                |event| {
                    if need_json {
                        json_accumulator.record(event);
                    }
                    if stream_lines {
                        let _ = writeln!(human, "{}", render_decl_line(event, color));
                    }
                },
            )
        };

        let report = match result {
            Ok(r) => r,
            Err(e) => {
                // Malformed / reader-unsupported / IO: exit 2, with a message
                // that distinguishes "this file is broken" from a rejection.
                let mut err = stderr.lock();
                match &e {
                    VerifyError::Malformed { .. } => {
                        let _ = writeln!(
                            err,
                            "oxilean-verify: {path}: {e} (this is a broken file, not a rejected proof)"
                        );
                    }
                    _ => {
                        let _ = writeln!(err, "oxilean-verify: {path}: {e}");
                    }
                }
                return ExitCode::from(exit::USAGE_OR_IO);
            }
        };

        // The per-file summary line is printed even in --quiet mode: quiet
        // suppresses the per-declaration stream, not the summary.
        {
            let mut h = human_writer(&stdout, &stderr, human_to_stderr);
            let _ = writeln!(h, "{}", render_summary_line(&report.summary, cfg.color));
        }

        if report.summary.rejected > 0 {
            any_rejected = true;
        }

        // Fold into the run-wide JSON state.
        if cfg.json_path.is_some() {
            json_summary.verified += report.summary.verified;
            json_summary.unsupported += report.summary.unsupported;
            json_summary.rejected += report.summary.rejected;
            json_summary.total += report.summary.total;
            json_summary.wall_micros += report.summary.wall_micros;
            if json_input.is_none() {
                json_input = Some(InputMeta {
                    path: if single_file {
                        path.clone()
                    } else {
                        format!("{path} (+{} more)", cfg.files.len() - 1)
                    },
                    sha256: if single_file { Some(sha) } else { None },
                    size_bytes: if single_file { Some(size_bytes) } else { None },
                });
                json_pins = Some(report.pins.clone());
            }
        }

        // Under fail-fast, a rejection in this file stops the whole run.
        if cfg.fail_fast && report.summary.rejected > 0 {
            break;
        }
    }

    // Write the JSON report, if requested.
    if let Some(json_path) = &cfg.json_path {
        if let (Some(input), Some(pins)) = (&json_input, &json_pins) {
            let text = render_report(pins, input, &json_summary, &json_accumulator);
            if json_path == "-" {
                let mut out = stdout.lock();
                if out.write_all(text.as_bytes()).is_err() {
                    return ExitCode::from(exit::USAGE_OR_IO);
                }
            } else if let Err(e) = std::fs::write(json_path, text.as_bytes()) {
                eprintln!("oxilean-verify: cannot write JSON report '{json_path}': {e}");
                return ExitCode::from(exit::USAGE_OR_IO);
            }
        }
    }

    if any_rejected {
        ExitCode::from(exit::REJECTED)
    } else {
        ExitCode::from(exit::OK)
    }
}

/// Lock and return the writer for human-readable output: stderr when the JSON
/// report occupies stdout (`--json -`), stdout otherwise. Boxed so both branches
/// share one type at the call site.
fn human_writer<'a>(
    stdout: &'a std::io::Stdout,
    stderr: &'a std::io::Stderr,
    to_stderr: bool,
) -> Box<dyn Write + 'a> {
    if to_stderr {
        Box::new(stderr.lock())
    } else {
        Box::new(stdout.lock())
    }
}
