//! # oxilean-verify-wasm — *Kernel in a Tab*
//!
//! WebAssembly bindings for the independent OxiLean Lean 4 proof checker. This
//! crate exists **only** to run [`oxilean-verify`](../oxilean_verify/index.html)
//! in a browser tab: a page reads a dropped `.export`/`.ndjson` file in slices,
//! pushes each slice as a text chunk, and receives one verdict per declaration
//! through a JS callback — verified / unsupported / rejected, the three buckets
//! of the engineering brief (§7), kept strictly separate.
//!
//! ## The dependency closure *is* the product (brief §8.3)
//!
//! A reviewer auditing this shim audits its whole runtime closure, and that
//! closure is exactly:
//!
//! * `oxilean-kernel` — the trusted computing base (zero external deps, no `unsafe`),
//! * `oxilean-export` — the lean4export NDJSON reader + replay,
//! * `oxilean-verify` — the reusable streaming engine that drives the two above,
//! * `wasm-bindgen` + `js-sys` — JS glue codegen and the callback bridge.
//!
//! Nothing else. `wasm-bindgen` is **not** feature-gated here: this crate has no
//! non-wasm build, so dead-code elimination can never strip the `#[wasm_bindgen]`
//! exports — the 0.1.2 "DCE ate the crate" regression class is structurally
//! impossible for this artifact.
//!
//! ## The engine path is identical to native
//!
//! [`VerifySession::finish`] drives [`oxilean_verify::verify_stream`] over a
//! `Cursor` of the accumulated file bytes exactly the way the native CLI drives
//! it over a file, so a declaration's verdict is byte-for-byte the same on both
//! (timing aside). Nothing is uploaded: the bytes never leave the WebAssembly
//! linear memory in the page.

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![warn(clippy::all)]

use std::io::Cursor;

use js_sys::Function;
use wasm_bindgen::prelude::*;

use oxilean_export::Limits;
use oxilean_verify::{
    render_report, verify_stream, DeclEvent, EnvironmentPins, InputMeta, ReportAccumulator,
    Summary, Verdict, VerifyOptions,
};

/// The build-time size of the checker itself, in kilobytes of gzipped wasm.
///
/// This is filled in at **copy time** by the demo's build script (it reads the
/// gzipped `.wasm` and writes the number into the page badge). The constant here
/// is the source of truth the JS badge falls back to when a fresher value is not
/// injected; it exists so the "kernel: N KB wasm" line is never invented.
///
/// It is deliberately *not* baked from the wasm itself (a module cannot know its
/// own compressed size before it exists); the demo overrides it. See
/// [`kernel_kb_placeholder`].
const KERNEL_KB_PLACEHOLDER: u32 = 0;

/// Which resource-limit preset a [`VerifySession`] runs with.
///
/// Presets map to [`oxilean_export::Limits`]. The default is the small,
/// untrusted-input budget — correct for a browser eating a dropped file. The
/// corpus preset raises the materialization budget for large, trusted exports.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LimitsPreset {
    /// The small untrusted-input budget ([`Limits::default`]): the right default
    /// for a file a stranger dropped into the page.
    Default = 0,
    /// The whole-corpus budget ([`Limits::corpus`]) for large trusted exports.
    Corpus = 1,
}

impl LimitsPreset {
    fn to_limits(self) -> Limits {
        match self {
            LimitsPreset::Default => Limits::default(),
            LimitsPreset::Corpus => Limits::corpus(),
        }
    }

    /// The deterministic per-declaration resource budget (kernel `Expr`
    /// nodes cloned) for this preset — a declaration exceeding it lands in
    /// the *unsupported* bucket with the named
    /// [`oxilean_export::RESOURCE_LIMIT`] feature, so one declaration can
    /// never OOM the tab.
    fn decl_fuel(self) -> Option<u64> {
        match self {
            LimitsPreset::Default => Some(oxilean_export::DEFAULT_DECL_FUEL),
            LimitsPreset::Corpus => Some(oxilean_export::CORPUS_DECL_FUEL),
        }
    }
}

/// A streaming verify session.
///
/// Lifecycle, matching how the page reads a `File`:
///
/// 1. [`VerifySession::new`] with a [`LimitsPreset`].
/// 2. [`VerifySession::push_chunk`] repeatedly — one call per slice the page
///    reads from the dropped file. Chunks are appended in order; the engine is
///    not run yet (it needs the whole stream to resolve back-references in the
///    lean4export index tables).
/// 3. [`VerifySession::finish`] with a per-declaration JS callback. This runs the
///    real [`oxilean_verify::verify_stream`] engine over the accumulated bytes,
///    invoking the callback once per declaration in file order, then returns the
///    summary + pins as a JSON string.
///
/// A session is single-shot: after [`finish`](VerifySession::finish) it should be
/// dropped. Pushing more chunks after finishing simply appends to bytes that are
/// no longer consulted.
#[wasm_bindgen]
pub struct VerifySession {
    bytes: Vec<u8>,
    limits: Limits,
    per_decl_fuel: Option<u64>,
}

/// A single declaration verdict handed to JS.
///
/// This is the shape the per-decl callback receives (as a plain object via
/// wasm-bindgen). The three buckets are kept distinct: `verdict` is exactly one
/// of `"verified"`, `"unsupported"`, or `"rejected"`, and `rejected` is the
/// alarm — never summed with `unsupported`.
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct DeclVerdict {
    name: String,
    kind: String,
    verdict: String,
    detail: String,
    micros: f64,
    index: usize,
}

#[wasm_bindgen]
impl DeclVerdict {
    /// The declaration's fully-qualified name.
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// The declaration kind label (`"axiom"`, `"def"`, `"thm"`, ...).
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn kind(&self) -> String {
        self.kind.clone()
    }

    /// The verdict bucket: `"verified"`, `"unsupported"`, or `"rejected"`.
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn verdict(&self) -> String {
        self.verdict.clone()
    }

    /// For `unsupported`, the named missing feature; for `rejected`, the reason;
    /// empty for `verified`.
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn detail(&self) -> String {
        self.detail.clone()
    }

    /// Microseconds spent checking this declaration (only meaningful for
    /// `verified`; `0` otherwise). Returned as `f64` because JS numbers are
    /// doubles — the value fits exactly for any realistic per-decl time.
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn micros(&self) -> f64 {
        self.micros
    }

    /// Milliseconds spent checking this declaration, as a float (convenience for
    /// the demo's right-aligned `N.N ms` column).
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn ms(&self) -> f64 {
        self.micros / 1000.0
    }

    /// The 0-based index of this declaration in file order.
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn index(&self) -> usize {
        self.index
    }
}

/// The final three-bucket summary returned by [`VerifySession::finish`].
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct VerifySummary {
    verified: usize,
    unsupported: usize,
    rejected: usize,
    total: usize,
    wall_ms: f64,
    pins_json: String,
}

#[wasm_bindgen]
impl VerifySummary {
    /// Declarations verified (checked and a proof).
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn verified(&self) -> usize {
        self.verified
    }

    /// Declarations deferred as unsupported (a named feature is missing).
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn unsupported(&self) -> usize {
        self.unsupported
    }

    /// Declarations rejected — the alarm bucket. Never summed with
    /// `unsupported`.
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn rejected(&self) -> usize {
        self.rejected
    }

    /// Total declarations seen.
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn total(&self) -> usize {
        self.total
    }

    /// Wall time for the whole file, in milliseconds (float).
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn wall_ms(&self) -> f64 {
        self.wall_ms
    }

    /// The deterministic JSON report as a string: `tool`, `pins`, `input`,
    /// `totals`, `unsupported_features`, `rejected`. Identical in shape to the
    /// native CLI's `--json` output (timing masked to whole ms). The demo can
    /// display or offer this for download — it is produced entirely in the page,
    /// so nothing is uploaded.
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn pins_json(&self) -> String {
        self.pins_json.clone()
    }
}

#[wasm_bindgen]
impl VerifySession {
    /// Create a new session with the given resource-limit preset.
    #[wasm_bindgen(constructor)]
    #[must_use]
    pub fn new(preset: LimitsPreset) -> VerifySession {
        VerifySession {
            bytes: Vec::new(),
            limits: preset.to_limits(),
            per_decl_fuel: preset.decl_fuel(),
        }
    }

    /// Append one text chunk (a slice of the dropped file) to the session.
    ///
    /// The page reads the `File` in slices and calls this once per slice, in
    /// order. Chunks are concatenated verbatim; verification runs only on
    /// [`finish`](VerifySession::finish).
    pub fn push_chunk(&mut self, chunk: &str) {
        self.bytes.extend_from_slice(chunk.as_bytes());
    }

    /// Append one raw byte chunk (a `Uint8Array` slice of the file) to the
    /// session. Preferred over [`push_chunk`](VerifySession::push_chunk) when the
    /// page reads the file as bytes rather than decoded text, so the exact file
    /// fingerprint is preserved.
    pub fn push_bytes(&mut self, chunk: &[u8]) {
        self.bytes.extend_from_slice(chunk);
    }

    /// The number of bytes pushed so far.
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn byte_len(&self) -> usize {
        self.bytes.len()
    }

    /// The lowercase-hex SHA-256 fingerprint of everything pushed so far.
    ///
    /// Computed client-side (in the page's WebAssembly memory) so the demo can
    /// show the exact file fingerprint under the "0 bytes uploaded" claim — the
    /// bytes never leave the machine.
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn sha256(&self) -> String {
        oxilean_verify::sha256::hex_digest(&self.bytes)
    }

    /// Run the verifier over everything pushed, streaming one verdict per
    /// declaration to `on_decl`, and return the summary + pins JSON.
    ///
    /// `on_decl` is a JS function called once per declaration, in file order,
    /// with a single [`DeclVerdict`] argument. It is invoked synchronously from
    /// inside the engine; to keep the UI live the page should let the whole
    /// `finish` run inside a worker, or slice the work — but the engine itself
    /// runs to completion here.
    ///
    /// # Errors
    /// Returns a JS error string (via `Err`) when the file cannot be read to
    /// completion: broken JSON, a lean4export spec violation, or the
    /// materialization budget being exceeded. This is **"the file is broken"**,
    /// distinct from any per-declaration `rejected` verdict — do not show it in
    /// the rejected bucket.
    #[wasm_bindgen]
    pub fn finish(&self, on_decl: &Function, path: &str) -> Result<VerifySummary, JsError> {
        let mut acc = ReportAccumulator::new(false);
        let this = JsValue::NULL;

        let options = VerifyOptions {
            limits: self.limits,
            fail_fast: false,
            per_decl_fuel: self.per_decl_fuel,
            // No wall-clock deadline in the browser: on wasm32 the monotonic
            // clock is a meaningless tick counter (see kernel `wall_clock`), so
            // the deterministic node budgets are the real per-declaration bound.
            per_decl_time_budget: None,
        };

        let cursor = Cursor::new(self.bytes.clone());

        // The callback records into the accumulator (for the JSON report) and
        // hands a plain DeclVerdict object to JS for live rendering. A callback
        // error from JS cannot abort the engine (its signature is infallible);
        // we swallow the JS-side call result deliberately — a rendering failure
        // in the page must not corrupt the verdict stream.
        let mut on_event = |event: &DeclEvent| {
            acc.record(event);
            let verdict = decl_verdict_from_event(event);
            let _ = on_decl.call1(&this, &JsValue::from(verdict));
        };

        let report = verify_stream(cursor, oxilean_verify::VERSION, options, &mut on_event)
            .map_err(|e| JsError::new(&format!("{e}")))?;

        let summary = report.summary;
        let pins_json = render_pins_json(&report.pins, path, &self.bytes, &summary, &acc);

        Ok(VerifySummary {
            verified: summary.verified,
            unsupported: summary.unsupported,
            rejected: summary.rejected,
            total: summary.total,
            wall_ms: summary.wall_micros as f64 / 1000.0,
            pins_json,
        })
    }
}

/// Build the deterministic JSON report string from the file report + input.
fn render_pins_json(
    pins: &EnvironmentPins,
    path: &str,
    bytes: &[u8],
    summary: &Summary,
    acc: &ReportAccumulator,
) -> String {
    let input = InputMeta {
        path: path.to_string(),
        sha256: Some(oxilean_verify::sha256::hex_digest(bytes)),
        size_bytes: Some(bytes.len() as u64),
    };
    render_report(pins, &input, summary, acc)
}

/// Map a streamed [`DeclEvent`] to the JS-facing [`DeclVerdict`].
fn decl_verdict_from_event(event: &DeclEvent) -> DeclVerdict {
    let (verdict, detail, micros) = match &event.verdict {
        Verdict::Verified { micros } => ("verified", String::new(), *micros as f64),
        Verdict::Unsupported { feature } => ("unsupported", (*feature).to_string(), 0.0),
        Verdict::Rejected { reason } => ("rejected", reason.clone(), 0.0),
    };
    DeclVerdict {
        name: event.name.clone(),
        kind: event.kind.to_string(),
        verdict: verdict.to_string(),
        detail,
        micros,
        index: event.index,
    }
}

/// The provenance pins as a JSON string, without needing a file: the build-time
/// pins only (tool name/version, lean4export commit, Lean toolchain, reader
/// format version). Lets the demo show the pins in the badge/footer before any
/// file is dropped.
#[wasm_bindgen]
#[must_use]
pub fn build_pins_json() -> String {
    let pins = EnvironmentPins::without_meta(oxilean_verify::VERSION);
    let mut out = String::new();
    out.push('{');
    push_json_field(&mut out, "tool_name", pins.tool_name, true);
    push_json_field(&mut out, "tool_version", pins.tool_version, false);
    push_json_field(
        &mut out,
        "lean4export_commit",
        pins.lean4export_commit,
        false,
    );
    push_json_field(&mut out, "lean_toolchain", pins.lean_toolchain, false);
    push_json_field(
        &mut out,
        "reader_format_version",
        pins.reader_format_version,
        false,
    );
    out.push('}');
    out
}

fn push_json_field(out: &mut String, key: &str, value: &str, first: bool) {
    if !first {
        out.push(',');
    }
    out.push('"');
    out.push_str(key);
    out.push_str("\":\"");
    // Values here are build-time constants (no control chars / quotes), but
    // escape defensively so the output is always valid JSON.
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            _ => out.push(ch),
        }
    }
    out.push('"');
}

/// This tool's version string (`CARGO_PKG_VERSION` of `oxilean-verify`).
#[wasm_bindgen]
#[must_use]
pub fn verify_version() -> String {
    oxilean_verify::VERSION.to_string()
}

/// The `lean4export` git commit this checker's reader is pinned to.
#[wasm_bindgen]
#[must_use]
pub fn lean4export_commit() -> String {
    oxilean_verify::LEAN4EXPORT_COMMIT.to_string()
}

/// The Lean toolchain version this checker's reader is pinned to.
#[wasm_bindgen]
#[must_use]
pub fn lean_toolchain() -> String {
    oxilean_verify::LEAN_TOOLCHAIN.to_string()
}

/// The NDJSON format version this checker's reader targets.
#[wasm_bindgen]
#[must_use]
pub fn ndjson_format_version() -> String {
    oxilean_verify::NDJSON_FORMAT_VERSION.to_string()
}

/// The built-in placeholder for the badge's "kernel: N KB wasm" figure. The demo
/// overrides this with the real gzipped size measured at copy time; this exists
/// so the number is never invented in the JS.
#[wasm_bindgen]
#[must_use]
pub fn kernel_kb_placeholder() -> u32 {
    KERNEL_KB_PLACEHOLDER
}

#[cfg(test)]
mod tests {
    //! Native tests of the *boundary* logic — the verdict→JS mapping and the
    //! JSON builders. The wasm-bindgen ABI itself cannot run natively here (no
    //! JS runtime in CI), but everything between the `verify_stream` engine and
    //! the JS surface is plain Rust and is exercised here. The engine path proper
    //! (`verify_stream` + `render_report`) has its own coverage in `oxilean-verify`
    //! and is proven byte-identical to this crate's use of it by
    //! `scripts/gate-determinism.sh`.

    use super::*;

    #[test]
    fn verdict_mapping_covers_all_three_buckets() {
        let verified = DeclEvent {
            name: "A".to_string(),
            kind: "def",
            verdict: Verdict::Verified { micros: 1234 },
            index: 0,
        };
        let v = decl_verdict_from_event(&verified);
        assert_eq!(v.verdict, "verified");
        assert_eq!(v.detail, "");
        assert!((v.micros - 1234.0).abs() < f64::EPSILON);
        assert!((v.ms() - 1.234).abs() < 1e-9);

        let unsupported = DeclEvent {
            name: "B".to_string(),
            kind: "thm",
            verdict: Verdict::Unsupported {
                feature: "quotient replay",
            },
            index: 1,
        };
        let u = decl_verdict_from_event(&unsupported);
        assert_eq!(u.verdict, "unsupported");
        assert_eq!(u.detail, "quotient replay");
        assert_eq!(u.micros, 0.0);

        let rejected = DeclEvent {
            name: "C".to_string(),
            kind: "def",
            verdict: Verdict::Rejected {
                reason: "type mismatch".to_string(),
            },
            index: 2,
        };
        let r = decl_verdict_from_event(&rejected);
        assert_eq!(r.verdict, "rejected");
        assert_eq!(r.detail, "type mismatch");
        assert_eq!(r.micros, 0.0);
    }

    #[test]
    fn build_pins_json_is_valid_and_carries_the_pins() {
        let json = build_pins_json();
        assert!(json.starts_with('{') && json.ends_with('}'));
        assert!(json.contains("\"tool_name\":\"oxilean-verify\""));
        assert!(json.contains("\"lean4export_commit\":\""));
        assert!(json.contains("\"lean_toolchain\":\""));
        assert!(json.contains("\"reader_format_version\":\""));
    }

    #[test]
    fn render_pins_json_includes_input_fingerprint_and_totals() {
        let pins = EnvironmentPins::without_meta(oxilean_verify::VERSION);
        let summary = Summary {
            verified: 2,
            unsupported: 1,
            rejected: 0,
            total: 3,
            wall_micros: 4_200,
        };
        let mut acc = ReportAccumulator::new(false);
        acc.record(&DeclEvent {
            name: "Only".to_string(),
            kind: "thm",
            verdict: Verdict::Unsupported { feature: "nested" },
            index: 0,
        });
        let bytes = b"some export bytes";
        let json = render_pins_json(&pins, "sample.ndjson", bytes, &summary, &acc);
        // The report carries the client-side fingerprint (proves "0 bytes uploaded"
        // is computed locally) and the exact byte length.
        let expected_sha = oxilean_verify::sha256::hex_digest(bytes);
        assert!(json.contains(&expected_sha));
        assert!(json.contains("\"size_bytes\": 17"));
        assert!(json.contains("\"verified\": 2"));
        assert!(json.contains("\"unsupported\": 1"));
        assert!(json.contains("\"rejected\": 0"));
        assert!(json.contains("\"path\": \"sample.ndjson\""));
    }

    #[test]
    fn limits_preset_maps_to_the_reader_budgets() {
        assert_eq!(
            LimitsPreset::Default.to_limits().materialize_budget,
            Limits::default().materialize_budget
        );
        assert_eq!(
            LimitsPreset::Corpus.to_limits().materialize_budget,
            Limits::corpus().materialize_budget
        );
    }
}
