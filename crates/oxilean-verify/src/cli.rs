//! Hand-rolled command-line parsing and output rendering for the
//! `oxilean-verify` binary.
//!
//! No `clap`: the whole product depends only on the kernel and the reader, so
//! the argument parser is a small, explicit state machine here, unit-tested
//! flag-by-flag. This module is UI logic only — the actual verification runs
//! through [`crate::engine`].
//!
//! ## Exit-code contract (engineering brief §7)
//!
//! * `0` — the run completed and **zero** declarations were rejected.
//! * `1` — the run completed and **at least one** declaration was rejected.
//!   `rejected` is an alarm; any non-zero count trips it.
//! * `2` — a usage error, an I/O error, or malformed export input. Malformed
//!   input is *not* a rejection: it is "this file is broken", reported here and
//!   kept off the exit-1 alarm channel.

use crate::engine::{DeclEvent, Summary, Verdict};

/// Streaming glyphs (the demo transcript, brief §6).
const GLYPH_VERIFIED: char = '\u{2713}'; // ✓
const GLYPH_UNSUPPORTED: char = '\u{2298}'; // ⊘
const GLYPH_REJECTED: char = '\u{2717}'; // ✗
/// The middle dot separating the three summary counts.
const SUMMARY_DOT: char = '\u{00b7}'; // ·

/// Which materialization-budget preset to use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LimitsPreset {
    /// The reader's small default budget (the untrusted-input bound).
    Default,
    /// A large budget suitable for trusted whole-corpus exports (Lean core,
    /// Mathlib), where indexed sharing legitimately expands to hundreds of
    /// millions of nodes.
    Corpus,
}

impl LimitsPreset {
    /// The materialization budget for this preset.
    #[must_use]
    pub fn budget(self) -> u64 {
        match self {
            // The reader's audited default (2^26 nodes).
            LimitsPreset::Default => oxilean_export::DEFAULT_MATERIALIZE_BUDGET,
            // Large but still finite — never `u64::MAX`, so an adversarial
            // trusted-mode file still errors rather than OOMs. This clears the
            // ~908M nodes observed for Lean core's `Init.ndjson`.
            LimitsPreset::Corpus => 1 << 31,
        }
    }

    /// The PER-DECLARATION materialization budget for this preset (C22): a
    /// single declaration whose kernel trees would exceed this many nodes is
    /// surfaced as a named *unsupported* declaration
    /// ([`oxilean_export::DECL_BUDGET_FEATURE`]) instead of expanding —
    /// never in `rejected`, and never an OOM.
    #[must_use]
    pub fn decl_budget(self) -> u64 {
        match self {
            LimitsPreset::Default => oxilean_export::DEFAULT_MATERIALIZE_BUDGET,
            LimitsPreset::Corpus => oxilean_export::CORPUS_DECL_MATERIALIZE_BUDGET,
        }
    }

    /// The deterministic per-declaration resource budget (kernel `Expr`
    /// nodes cloned) for this preset. A declaration exceeding it lands in
    /// the *unsupported* bucket with the named
    /// [`oxilean_export::RESOURCE_LIMIT`] feature — never in `rejected`,
    /// and never an OOM.
    #[must_use]
    pub fn decl_fuel(self) -> Option<u64> {
        match self {
            LimitsPreset::Default => Some(oxilean_export::DEFAULT_DECL_FUEL),
            LimitsPreset::Corpus => Some(oxilean_export::CORPUS_DECL_FUEL),
        }
    }

    /// The wall-clock per-declaration deadline for this preset (`None` = none).
    ///
    /// A backstop for whole-corpus reads: some Mathlib `CategoryTheory`
    /// declarations drive a reduction/def-eq loop that never constructs nodes,
    /// so the deterministic [`decl_fuel`](Self::decl_fuel) never catches them
    /// and they would hang the run. Over-time declarations are degraded like
    /// fuel-exhausted ones and reported as
    /// [`oxilean_export::RESOURCE_LIMIT`]. The `Default` (untrusted-input)
    /// preset arms no deadline: its small node budgets already bound the work,
    /// and a wall-clock limit would make small verdicts machine-dependent.
    #[must_use]
    pub fn decl_time_budget(self) -> Option<std::time::Duration> {
        match self {
            LimitsPreset::Default => None,
            LimitsPreset::Corpus => Some(std::time::Duration::from_secs(30)),
        }
    }
}

/// The default stack size for the verification thread, in MiB (C17): deep
/// Lean-core reduction chains legitimately need far more than the 8 MiB
/// default thread stack; 512 MiB of *reserved* (not committed) stack removes
/// the need for any `ulimit -s` incantation.
pub const DEFAULT_STACK_SIZE_MIB: u64 = 512;

/// A fully parsed, validated command line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunConfig {
    /// Input files, in the order given. Never empty (validated at parse time).
    pub files: Vec<String>,
    /// Path for the JSON report, if `--json <PATH>` was given.
    pub json_path: Option<String>,
    /// Whether `--json-full` was given (retain a per-declaration list).
    pub json_full: bool,
    /// Suppress per-declaration streaming lines (summary only).
    pub quiet: bool,
    /// Disable ANSI colour in streaming output.
    pub color: bool,
    /// The materialization-budget preset.
    pub limits: LimitsPreset,
    /// Stop at the first rejection.
    pub fail_fast: bool,
    /// Stack size for the verification thread, in MiB (`--stack-size`).
    pub stack_size_mib: u64,
}

/// The outcome of parsing a command line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    /// Run the verifier with this configuration.
    Run(RunConfig),
    /// Print help and exit 0.
    Help,
    /// Print the version banner and exit 0.
    Version,
}

/// A usage error. Always maps to exit code 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsageError {
    /// The human-readable message (without a trailing newline).
    pub message: String,
}

impl UsageError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

/// Parse `args` (the arguments *after* the program name) into a [`Command`].
///
/// `color_default` is the colour setting to use when neither `--no-color` nor
/// the `NO_COLOR` environment variable applies; the binary passes
/// `stdout_is_tty && NO_COLOR unset`.
///
/// # Errors
/// Returns a [`UsageError`] (exit 2) for unknown flags, missing option values,
/// bad `--limits` values, or no input files.
pub fn parse_args(args: &[String], color_default: bool) -> Result<Command, UsageError> {
    let mut files = Vec::new();
    let mut json_path = None;
    let mut json_full = false;
    let mut quiet = false;
    let mut no_color = false;
    let mut limits = LimitsPreset::Default;
    let mut fail_fast = false;
    let mut stack_size_mib = DEFAULT_STACK_SIZE_MIB;
    let mut positional_only = false;

    fn parse_stack_size(v: &str) -> Result<u64, UsageError> {
        match v.parse::<u64>() {
            Ok(n) if n >= 1 => Ok(n),
            _ => Err(UsageError::new(format!(
                "--stack-size expects a positive number of MiB, got '{v}'"
            ))),
        }
    }

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if positional_only {
            files.push(arg.clone());
            i += 1;
            continue;
        }
        match arg.as_str() {
            "-h" | "--help" => return Ok(Command::Help),
            "-V" | "--version" => return Ok(Command::Version),
            "--" => {
                positional_only = true;
            }
            "--quiet" | "-q" => quiet = true,
            "--no-color" => no_color = true,
            "--fail-fast" => fail_fast = true,
            "--json-full" => json_full = true,
            "--json" => {
                i += 1;
                match args.get(i) {
                    Some(p) => json_path = Some(p.clone()),
                    None => return Err(UsageError::new("--json requires a <PATH> argument")),
                }
            }
            "--limits" => {
                i += 1;
                match args.get(i).map(String::as_str) {
                    Some("default") => limits = LimitsPreset::Default,
                    Some("corpus") => limits = LimitsPreset::Corpus,
                    Some(other) => {
                        return Err(UsageError::new(format!(
                            "--limits expects 'default' or 'corpus', got '{other}'"
                        )))
                    }
                    None => {
                        return Err(UsageError::new(
                            "--limits requires a value: 'default' or 'corpus'",
                        ))
                    }
                }
            }
            "--stack-size" => {
                i += 1;
                match args.get(i) {
                    Some(v) => stack_size_mib = parse_stack_size(v)?,
                    None => {
                        return Err(UsageError::new(
                            "--stack-size requires a value in MiB (e.g. 512)",
                        ))
                    }
                }
            }
            // Attached forms: --json=PATH, --limits=corpus, --stack-size=N.
            s if s.starts_with("--stack-size=") => {
                stack_size_mib = parse_stack_size(&s["--stack-size=".len()..])?;
            }
            s if s.starts_with("--json=") => {
                json_path = Some(s["--json=".len()..].to_string());
            }
            s if s.starts_with("--limits=") => {
                let v = &s["--limits=".len()..];
                match v {
                    "default" => limits = LimitsPreset::Default,
                    "corpus" => limits = LimitsPreset::Corpus,
                    other => {
                        return Err(UsageError::new(format!(
                            "--limits expects 'default' or 'corpus', got '{other}'"
                        )))
                    }
                }
            }
            s if s.starts_with('-') && s != "-" => {
                return Err(UsageError::new(format!("unknown option: {s}")));
            }
            _ => files.push(arg.clone()),
        }
        i += 1;
    }

    if files.is_empty() {
        return Err(UsageError::new(
            "no input files given; usage: oxilean-verify [OPTIONS] <FILE.ndjson>...",
        ));
    }

    let color = color_default && !no_color;

    Ok(Command::Run(RunConfig {
        files,
        json_path,
        json_full,
        quiet,
        color,
        limits,
        fail_fast,
        stack_size_mib,
    }))
}

/// The `--help` text.
#[must_use]
pub fn help_text() -> String {
    format!(
        "oxilean-verify {version} — an independent Lean 4 proof checker\n\
\n\
USAGE:\n    \
oxilean-verify [OPTIONS] <FILE.ndjson>...\n\
\n\
ARGS:\n    \
<FILE.ndjson>...   one or more lean4export NDJSON files to verify\n\
\n\
OPTIONS:\n    \
--json <PATH>      write a machine-readable JSON report to PATH\n    \
--json-full        include a full per-declaration list in the JSON report\n    \
--quiet, -q        suppress per-declaration lines; print only the summary\n    \
--no-color         disable ANSI colour (also honours the NO_COLOR env var)\n    \
--limits <PRESET>  materialization budget: 'default' (small, untrusted) or\n                       \
'corpus' (large, for trusted whole-corpus exports); also\n                       \
sets the per-declaration resource budget (a declaration\n                       \
exceeding it is reported as unsupported, never rejected)\n    \
--stack-size <MiB> stack size of the verification thread in MiB\n                       \
(default 512; deep reduction chains in real corpora need\n                       \
far more than the 8 MiB OS default — no ulimit needed)\n    \
--fail-fast        stop at the first rejected declaration\n    \
--version, -V      print version and pinned toolchain, then exit\n    \
--help, -h         print this help and exit\n\
\n\
EXIT CODES:\n    \
0   ran to completion, zero declarations rejected\n    \
1   at least one declaration REJECTED (alarm — a proof did not check)\n    \
2   usage error, I/O error, or malformed export input (a broken file,\n        \
which is NOT a rejection)\n",
        version = crate::VERSION
    )
}

/// The `--version` banner, naming the pinned reader toolchain.
#[must_use]
pub fn version_text() -> String {
    format!(
        "oxilean-verify {version}\n\
lean4export commit: {commit}\n\
Lean toolchain:     {toolchain}\n\
NDJSON format:      {format}\n",
        version = crate::VERSION,
        commit = crate::LEAN4EXPORT_COMMIT,
        toolchain = crate::LEAN_TOOLCHAIN,
        format = crate::NDJSON_FORMAT_VERSION,
    )
}

/// ANSI colour codes, gated on the `color` flag.
struct Palette {
    green: &'static str,
    yellow: &'static str,
    red: &'static str,
    dim: &'static str,
    reset: &'static str,
}

impl Palette {
    fn new(color: bool) -> Self {
        if color {
            Self {
                green: "\u{1b}[32m",
                yellow: "\u{1b}[33m",
                red: "\u{1b}[31m",
                dim: "\u{1b}[2m",
                reset: "\u{1b}[0m",
            }
        } else {
            Self {
                green: "",
                yellow: "",
                red: "",
                dim: "",
                reset: "",
            }
        }
    }
}

/// The column width at which the declaration name is padded before the timing
/// or reason column. Chosen to match the brief's demo transcript spacing.
const NAME_COLUMN: usize = 44;

/// Render one streamed declaration line, mirroring the brief's demo transcript:
///
/// ```text
/// ✓  Real.exp_log                              0.9 ms
/// ⊘  Real.exp_approx        unsupported: Nat literal reduction
/// ✗  Foo.bad                rejected: type mismatch ...
/// ```
#[must_use]
pub fn render_decl_line(event: &DeclEvent, color: bool) -> String {
    let pal = Palette::new(color);
    let name = &event.name;
    let padded_name = pad_name(name, NAME_COLUMN);
    match &event.verdict {
        Verdict::Verified { micros } => {
            format!(
                "{g}{glyph}{reset}  {name}{dim}{ms:>10}{reset}",
                g = pal.green,
                glyph = GLYPH_VERIFIED,
                reset = pal.reset,
                name = padded_name,
                dim = pal.dim,
                ms = format_ms(*micros),
            )
        }
        Verdict::Unsupported { feature } => {
            format!(
                "{y}{glyph}{reset}  {name}unsupported: {feature}",
                y = pal.yellow,
                glyph = GLYPH_UNSUPPORTED,
                reset = pal.reset,
                name = padded_name,
                feature = feature,
            )
        }
        Verdict::Rejected { reason } => {
            format!(
                "{r}{glyph}  {name}rejected: {reason}{reset}",
                r = pal.red,
                glyph = GLYPH_REJECTED,
                reset = pal.reset,
                name = padded_name,
                reason = one_line(reason),
            )
        }
    }
}

/// Render the final summary line: `N verified · U unsupported · R rejected`.
#[must_use]
pub fn render_summary_line(summary: &Summary, color: bool) -> String {
    let pal = Palette::new(color);
    format!(
        "{g}{v} verified{reset} {dot} {y}{u} unsupported{reset} {dot} {rc}{r} rejected{reset}",
        g = pal.green,
        v = summary.verified,
        reset = pal.reset,
        dot = SUMMARY_DOT,
        y = pal.yellow,
        u = summary.unsupported,
        rc = if summary.rejected > 0 {
            pal.red
        } else {
            pal.dim
        },
        r = summary.rejected,
    )
}

/// Pad a declaration name to at least `width` columns (by `char` count), with a
/// trailing space. Over-long names are not truncated — correctness of the name
/// beats column alignment.
fn pad_name(name: &str, width: usize) -> String {
    let len = name.chars().count();
    if len >= width {
        format!("{name} ")
    } else {
        let pad = width - len;
        let mut s = String::with_capacity(name.len() + pad);
        s.push_str(name);
        for _ in 0..pad {
            s.push(' ');
        }
        s
    }
}

/// Format microseconds as a millisecond string with one decimal place, e.g.
/// `1234` µs → `"1.2 ms"`. Deterministic and allocation-cheap.
fn format_ms(micros: u64) -> String {
    let tenths = (micros + 50) / 100; // round to 0.1 ms
    let whole = tenths / 10;
    let frac = tenths % 10;
    format!("{whole}.{frac} ms")
}

/// Collapse a possibly multi-line reason into a single line for stream output.
fn one_line(reason: &str) -> String {
    reason.replace(['\n', '\r'], " ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| (*s).to_string()).collect()
    }

    #[test]
    fn help_flag() {
        assert_eq!(parse_args(&args(&["--help"]), true), Ok(Command::Help));
        assert_eq!(parse_args(&args(&["-h"]), true), Ok(Command::Help));
        // Help wins even alongside other args.
        assert_eq!(
            parse_args(&args(&["file.ndjson", "--help"]), true),
            Ok(Command::Help)
        );
    }

    #[test]
    fn version_flag() {
        assert_eq!(
            parse_args(&args(&["--version"]), true),
            Ok(Command::Version)
        );
        assert_eq!(parse_args(&args(&["-V"]), true), Ok(Command::Version));
    }

    #[test]
    fn no_files_is_usage_error() {
        let err = parse_args(&args(&["--quiet"]), true).unwrap_err();
        assert!(err.message.contains("no input files"));
    }

    #[test]
    fn unknown_flag_is_usage_error() {
        let err = parse_args(&args(&["--bogus", "f.ndjson"]), true).unwrap_err();
        assert!(err.message.contains("unknown option: --bogus"));
    }

    #[test]
    fn single_file_defaults() {
        let cmd = parse_args(&args(&["f.ndjson"]), true).unwrap();
        let Command::Run(cfg) = cmd else {
            panic!("expected Run");
        };
        assert_eq!(cfg.files, vec!["f.ndjson".to_string()]);
        assert_eq!(cfg.json_path, None);
        assert!(!cfg.json_full);
        assert!(!cfg.quiet);
        assert!(cfg.color); // color_default = true, no --no-color
        assert_eq!(cfg.limits, LimitsPreset::Default);
        assert!(!cfg.fail_fast);
    }

    #[test]
    fn all_flags_together() {
        let cmd = parse_args(
            &args(&[
                "--json",
                "out.json",
                "--json-full",
                "--quiet",
                "--no-color",
                "--limits",
                "corpus",
                "--fail-fast",
                "a.ndjson",
                "b.ndjson",
            ]),
            true,
        )
        .unwrap();
        let Command::Run(cfg) = cmd else {
            panic!("expected Run");
        };
        assert_eq!(
            cfg.files,
            vec!["a.ndjson".to_string(), "b.ndjson".to_string()]
        );
        assert_eq!(cfg.json_path, Some("out.json".to_string()));
        assert!(cfg.json_full);
        assert!(cfg.quiet);
        assert!(!cfg.color); // --no-color overrides
        assert_eq!(cfg.limits, LimitsPreset::Corpus);
        assert!(cfg.fail_fast);
    }

    #[test]
    fn attached_option_forms() {
        let cmd = parse_args(
            &args(&["--json=r.json", "--limits=corpus", "f.ndjson"]),
            false,
        )
        .unwrap();
        let Command::Run(cfg) = cmd else {
            panic!("expected Run");
        };
        assert_eq!(cfg.json_path, Some("r.json".to_string()));
        assert_eq!(cfg.limits, LimitsPreset::Corpus);
        assert!(!cfg.color); // color_default = false
    }

    #[test]
    fn json_requires_value() {
        let err = parse_args(&args(&["--json"]), true).unwrap_err();
        assert!(err.message.contains("--json requires"));
    }

    #[test]
    fn limits_bad_value() {
        let err = parse_args(&args(&["--limits", "huge", "f.ndjson"]), true).unwrap_err();
        assert!(err.message.contains("default") && err.message.contains("corpus"));
        let err2 = parse_args(&args(&["--limits"]), true).unwrap_err();
        assert!(err2.message.contains("--limits requires"));
    }

    #[test]
    fn double_dash_forces_positional() {
        // A file literally named "--json" after `--`.
        let cmd = parse_args(&args(&["--", "--json", "f.ndjson"]), true).unwrap();
        let Command::Run(cfg) = cmd else {
            panic!("expected Run");
        };
        assert_eq!(
            cfg.files,
            vec!["--json".to_string(), "f.ndjson".to_string()]
        );
        assert_eq!(cfg.json_path, None);
    }

    #[test]
    fn no_color_env_via_color_default() {
        // The binary folds NO_COLOR into color_default; simulate it as false.
        let cmd = parse_args(&args(&["f.ndjson"]), false).unwrap();
        let Command::Run(cfg) = cmd else {
            panic!("expected Run");
        };
        assert!(!cfg.color);
    }

    #[test]
    fn stack_size_flag() {
        // Default when absent.
        let Command::Run(cfg) = parse_args(&args(&["f.ndjson"]), true).unwrap() else {
            panic!("expected Run");
        };
        assert_eq!(cfg.stack_size_mib, DEFAULT_STACK_SIZE_MIB);
        // Separate and attached forms.
        let Command::Run(cfg) =
            parse_args(&args(&["--stack-size", "64", "f.ndjson"]), true).unwrap()
        else {
            panic!("expected Run");
        };
        assert_eq!(cfg.stack_size_mib, 64);
        let Command::Run(cfg) =
            parse_args(&args(&["--stack-size=1024", "f.ndjson"]), true).unwrap()
        else {
            panic!("expected Run");
        };
        assert_eq!(cfg.stack_size_mib, 1024);
        // Bad values are usage errors (exit 2), never a default fallback.
        assert!(parse_args(&args(&["--stack-size", "0", "f.ndjson"]), true).is_err());
        assert!(parse_args(&args(&["--stack-size", "huge", "f.ndjson"]), true).is_err());
        assert!(parse_args(&args(&["--stack-size"]), true).is_err());
    }

    #[test]
    fn limits_presets_carry_decl_fuel() {
        assert_eq!(
            LimitsPreset::Default.decl_fuel(),
            Some(oxilean_export::DEFAULT_DECL_FUEL)
        );
        assert_eq!(
            LimitsPreset::Corpus.decl_fuel(),
            Some(oxilean_export::CORPUS_DECL_FUEL)
        );
    }

    #[test]
    fn format_ms_rounding() {
        assert_eq!(format_ms(0), "0.0 ms");
        assert_eq!(format_ms(50), "0.1 ms");
        assert_eq!(format_ms(949), "0.9 ms");
        assert_eq!(format_ms(1200), "1.2 ms");
        assert_eq!(format_ms(4750), "4.8 ms");
    }

    #[test]
    fn summary_line_uses_middle_dot_and_words() {
        let s = Summary {
            verified: 1204,
            unsupported: 3,
            rejected: 0,
            total: 1207,
            wall_micros: 5,
        };
        let line = render_summary_line(&s, false);
        assert_eq!(
            line,
            "1204 verified \u{00b7} 3 unsupported \u{00b7} 0 rejected"
        );
    }

    #[test]
    fn decl_line_glyphs_no_color() {
        let verified = DeclEvent {
            name: "Real.exp_log".to_string(),
            kind: "thm",
            verdict: Verdict::Verified { micros: 900 },
            index: 0,
        };
        let line = render_decl_line(&verified, false);
        assert!(line.starts_with('\u{2713}'));
        assert!(line.contains("Real.exp_log"));
        assert!(line.contains("0.9 ms"));

        let unsupported = DeclEvent {
            name: "Real.exp_approx".to_string(),
            kind: "thm",
            verdict: Verdict::Unsupported {
                feature: "Nat literal reduction",
            },
            index: 1,
        };
        let line = render_decl_line(&unsupported, false);
        assert!(line.starts_with('\u{2298}'));
        assert!(line.contains("unsupported: Nat literal reduction"));

        let rejected = DeclEvent {
            name: "Foo.bad".to_string(),
            kind: "thm",
            verdict: Verdict::Rejected {
                reason: "type mismatch\nsecond line".to_string(),
            },
            index: 2,
        };
        let line = render_decl_line(&rejected, false);
        assert!(line.starts_with('\u{2717}'));
        assert!(line.contains("rejected: type mismatch second line"));
        // Reason was collapsed to a single line.
        assert!(!line.contains('\n'));
    }

    #[test]
    fn color_wraps_glyph_in_ansi() {
        let verified = DeclEvent {
            name: "x".to_string(),
            kind: "def",
            verdict: Verdict::Verified { micros: 100 },
            index: 0,
        };
        let line = render_decl_line(&verified, true);
        assert!(line.contains("\u{1b}[32m"));
        assert!(line.contains("\u{1b}[0m"));
    }
}
