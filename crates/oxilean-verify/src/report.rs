//! The machine-readable JSON report: an accumulator that consumes streamed
//! [`DeclEvent`]s and a hand-rolled, deterministic writer.
//!
//! The report shape is fixed and documented in `docs/VERIFY.md`. Field order is
//! stable; the same run always produces byte-identical bytes. See [`crate::jsonw`]
//! for the writer.

use crate::engine::{DeclEvent, Summary, Verdict};
use crate::jsonw::JsonWriter;
use crate::pins::EnvironmentPins;

/// How many declaration names to list per unsupported feature before capping.
/// The cap keeps the report bounded on large corpora; the total count is always
/// exact and the cap is noted in the JSON (`decls_capped: true`).
pub const UNSUPPORTED_DECL_CAP: usize = 32;

/// A per-declaration record, retained only when `--json-full` is requested.
#[derive(Debug, Clone)]
pub struct DeclRecord {
    /// Fully-qualified declaration name.
    pub name: String,
    /// Declaration kind label.
    pub kind: &'static str,
    /// The verdict bucket label.
    pub verdict: &'static str,
    /// Microseconds spent checking (only meaningful for verified).
    pub micros: u64,
    /// The named feature (unsupported) or rejection reason, if any.
    pub detail: Option<String>,
}

/// One rejected declaration.
#[derive(Debug, Clone)]
pub struct Rejection {
    /// The declaration name.
    pub name: String,
    /// The rejection reason (rendered kernel error).
    pub reason: String,
}

/// One unsupported feature group with its count and a capped sample of decls.
#[derive(Debug, Clone)]
pub struct UnsupportedGroup {
    /// The named feature.
    pub feature: String,
    /// Total number of declarations deferred for this feature (exact).
    pub count: usize,
    /// Up to [`UNSUPPORTED_DECL_CAP`] declaration names.
    pub decls: Vec<String>,
    /// `true` if `count` exceeded the cap and `decls` was truncated.
    pub decls_capped: bool,
}

/// Accumulates the data a JSON report needs from a stream of [`DeclEvent`]s.
///
/// Feeding is O(1) amortised and memory stays bounded even on big corpora
/// (unsupported groups cap their sample lists; full per-decl records are only
/// kept when explicitly enabled).
#[derive(Debug, Clone, Default)]
pub struct ReportAccumulator {
    unsupported: Vec<UnsupportedGroup>,
    rejected: Vec<Rejection>,
    /// Populated only when `keep_decls` is set.
    decls: Vec<DeclRecord>,
    keep_decls: bool,
}

impl ReportAccumulator {
    /// A fresh accumulator. Set `keep_decls` to retain a full per-declaration
    /// list (for `--json-full`).
    #[must_use]
    pub fn new(keep_decls: bool) -> Self {
        Self {
            keep_decls,
            ..Self::default()
        }
    }

    /// Feed one streamed declaration event.
    pub fn record(&mut self, event: &DeclEvent) {
        match &event.verdict {
            Verdict::Verified { micros } => {
                if self.keep_decls {
                    self.decls.push(DeclRecord {
                        name: event.name.clone(),
                        kind: event.kind,
                        verdict: "verified",
                        micros: *micros,
                        detail: None,
                    });
                }
            }
            Verdict::Unsupported { feature } => {
                self.push_unsupported(feature, &event.name);
                if self.keep_decls {
                    self.decls.push(DeclRecord {
                        name: event.name.clone(),
                        kind: event.kind,
                        verdict: "unsupported",
                        micros: 0,
                        detail: Some((*feature).to_string()),
                    });
                }
            }
            Verdict::Rejected { reason } => {
                self.rejected.push(Rejection {
                    name: event.name.clone(),
                    reason: reason.clone(),
                });
                if self.keep_decls {
                    self.decls.push(DeclRecord {
                        name: event.name.clone(),
                        kind: event.kind,
                        verdict: "rejected",
                        micros: 0,
                        detail: Some(reason.clone()),
                    });
                }
            }
        }
    }

    fn push_unsupported(&mut self, feature: &str, name: &str) {
        if let Some(group) = self.unsupported.iter_mut().find(|g| g.feature == feature) {
            group.count += 1;
            if group.decls.len() < UNSUPPORTED_DECL_CAP {
                group.decls.push(name.to_string());
            } else {
                group.decls_capped = true;
            }
        } else {
            self.unsupported.push(UnsupportedGroup {
                feature: feature.to_string(),
                count: 1,
                decls: vec![name.to_string()],
                decls_capped: false,
            });
        }
    }

    /// The accumulated unsupported groups (in first-seen order).
    #[must_use]
    pub fn unsupported_groups(&self) -> &[UnsupportedGroup] {
        &self.unsupported
    }

    /// The accumulated rejections (in file order).
    #[must_use]
    pub fn rejections(&self) -> &[Rejection] {
        &self.rejected
    }
}

/// Input-file metadata for the report's `input` section.
#[derive(Debug, Clone)]
pub struct InputMeta {
    /// The file path as given on the command line.
    pub path: String,
    /// Lowercase hex SHA-256 of the file bytes, if computed.
    pub sha256: Option<String>,
    /// File size in bytes, if known.
    pub size_bytes: Option<u64>,
}

/// Render the full JSON report deterministically.
///
/// Field order (stable, documented in `docs/VERIFY.md`):
/// `tool`, `pins`, `input`, `totals`, `unsupported_features`, `rejected`,
/// and — only when the accumulator kept them — `decls`.
#[must_use]
pub fn render_report(
    pins: &EnvironmentPins,
    input: &InputMeta,
    summary: &Summary,
    acc: &ReportAccumulator,
) -> String {
    let mut w = JsonWriter::new();
    {
        let mut root = w.object();

        // tool
        {
            let mut tool = root.key("tool").object();
            tool.str_field("name", pins.tool_name);
            tool.str_field("version", pins.tool_version);
        }

        // pins
        {
            let mut p = root.key("pins").object();
            p.str_field("lean4export_commit", pins.lean4export_commit);
            p.str_field("lean_toolchain", pins.lean_toolchain);
            p.str_field("reader_format_version", pins.reader_format_version);
            match &pins.file_format_version {
                Some(v) => p.str_field("file_format_version", v),
                None => p.null_field("file_format_version"),
            }
            match &pins.file_lean_version {
                Some(v) => p.str_field("file_lean_version", v),
                None => p.null_field("file_lean_version"),
            }
            match &pins.file_lean_githash {
                Some(v) => p.str_field("file_lean_githash", v),
                None => p.null_field("file_lean_githash"),
            }
        }

        // input
        {
            let mut inp = root.key("input").object();
            inp.str_field("path", &input.path);
            match &input.sha256 {
                Some(s) => inp.str_field("sha256", s),
                None => inp.null_field("sha256"),
            }
            match input.size_bytes {
                Some(n) => inp.uint_field("size_bytes", n),
                None => inp.null_field("size_bytes"),
            }
            match &pins.file_format_version {
                Some(v) => inp.str_field("format_version", v),
                None => inp.null_field("format_version"),
            }
        }

        // totals
        {
            let mut t = root.key("totals").object();
            t.uint_field("verified", summary.verified as u64);
            t.uint_field("unsupported", summary.unsupported as u64);
            t.uint_field("rejected", summary.rejected as u64);
            t.uint_field("wall_ms", summary.wall_ms());
        }

        // unsupported_features
        {
            let mut arr = root.key("unsupported_features").array();
            for group in &acc.unsupported {
                let mut g = arr.element().object();
                g.str_field("feature", &group.feature);
                g.uint_field("count", group.count as u64);
                g.bool_field("decls_capped", group.decls_capped);
                {
                    let mut decls = g.key("decls").array();
                    for name in &group.decls {
                        decls.element().string_value(name);
                    }
                }
            }
        }

        // rejected
        {
            let mut arr = root.key("rejected").array();
            for r in &acc.rejected {
                let mut obj = arr.element().object();
                obj.str_field("name", &r.name);
                obj.str_field("reason", &r.reason);
            }
        }

        // decls (optional)
        if acc.keep_decls {
            let mut arr = root.key("decls").array();
            for d in &acc.decls {
                let mut obj = arr.element().object();
                obj.str_field("name", &d.name);
                obj.str_field("kind", d.kind);
                obj.str_field("verdict", d.verdict);
                obj.uint_field("micros", d.micros);
                match &d.detail {
                    Some(detail) => obj.str_field("detail", detail),
                    None => obj.null_field("detail"),
                }
            }
        }
    }
    w.into_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{DeclEvent, Summary, Verdict};
    use crate::pins::EnvironmentPins;

    fn ev(name: &str, kind: &'static str, verdict: Verdict, index: usize) -> DeclEvent {
        DeclEvent {
            name: name.to_string(),
            kind,
            verdict,
            index,
        }
    }

    fn sample_pins() -> EnvironmentPins {
        EnvironmentPins {
            tool_name: "oxilean-verify",
            tool_version: "9.9.9",
            lean4export_commit: "COMMIT",
            lean_toolchain: "TOOLCHAIN",
            reader_format_version: "3.1.0",
            file_format_version: Some("3.1.0".to_string()),
            file_lean_version: Some("4.32.0-rc1".to_string()),
            file_lean_githash: Some("GITHASH".to_string()),
        }
    }

    #[test]
    fn accumulator_groups_unsupported_and_lists_rejections() {
        let mut acc = ReportAccumulator::new(false);
        acc.record(&ev("A", "def", Verdict::Verified { micros: 12 }, 0));
        acc.record(&ev(
            "B",
            "thm",
            Verdict::Unsupported { feature: "feat-x" },
            1,
        ));
        acc.record(&ev(
            "C",
            "thm",
            Verdict::Unsupported { feature: "feat-x" },
            2,
        ));
        acc.record(&ev(
            "D",
            "def",
            Verdict::Rejected {
                reason: "boom".to_string(),
            },
            3,
        ));

        let groups = acc.unsupported_groups();
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].feature, "feat-x");
        assert_eq!(groups[0].count, 2);
        assert_eq!(groups[0].decls, vec!["B".to_string(), "C".to_string()]);
        assert!(!groups[0].decls_capped);

        let rej = acc.rejections();
        assert_eq!(rej.len(), 1);
        assert_eq!(rej[0].name, "D");
        assert_eq!(rej[0].reason, "boom");
    }

    #[test]
    fn unsupported_decl_list_is_capped_but_count_is_exact() {
        let mut acc = ReportAccumulator::new(false);
        let total = UNSUPPORTED_DECL_CAP + 10;
        for i in 0..total {
            acc.record(&ev(
                &format!("d{i}"),
                "thm",
                Verdict::Unsupported { feature: "f" },
                i,
            ));
        }
        let g = &acc.unsupported_groups()[0];
        assert_eq!(g.count, total, "count is exact");
        assert_eq!(g.decls.len(), UNSUPPORTED_DECL_CAP, "sample list is capped");
        assert!(g.decls_capped, "cap flag is set");
    }

    /// Golden: the exact bytes of a report. Pins field order and the whole
    /// shape. If this changes, `docs/VERIFY.md` must change with it.
    #[test]
    fn render_report_golden() {
        let pins = sample_pins();
        let input = InputMeta {
            path: "corpus/simple.ndjson".to_string(),
            sha256: Some("abc123".to_string()),
            size_bytes: Some(1024),
        };
        let summary = Summary {
            verified: 2,
            unsupported: 1,
            rejected: 1,
            total: 4,
            wall_micros: 3_400,
        };
        let mut acc = ReportAccumulator::new(false);
        acc.record(&ev("A", "def", Verdict::Verified { micros: 10 }, 0));
        acc.record(&ev("B", "thm", Verdict::Verified { micros: 20 }, 1));
        acc.record(&ev(
            "C",
            "thm",
            Verdict::Unsupported {
                feature: "quotient replay",
            },
            2,
        ));
        acc.record(&ev(
            "D",
            "def",
            Verdict::Rejected {
                reason: "type mismatch".to_string(),
            },
            3,
        ));

        let out = render_report(&pins, &input, &summary, &acc);
        let expected = "{\n  \"tool\": {\n    \"name\": \"oxilean-verify\",\n    \"version\": \"9.9.9\"\n  },\n  \"pins\": {\n    \"lean4export_commit\": \"COMMIT\",\n    \"lean_toolchain\": \"TOOLCHAIN\",\n    \"reader_format_version\": \"3.1.0\",\n    \"file_format_version\": \"3.1.0\",\n    \"file_lean_version\": \"4.32.0-rc1\",\n    \"file_lean_githash\": \"GITHASH\"\n  },\n  \"input\": {\n    \"path\": \"corpus/simple.ndjson\",\n    \"sha256\": \"abc123\",\n    \"size_bytes\": 1024,\n    \"format_version\": \"3.1.0\"\n  },\n  \"totals\": {\n    \"verified\": 2,\n    \"unsupported\": 1,\n    \"rejected\": 1,\n    \"wall_ms\": 3\n  },\n  \"unsupported_features\": [\n    {\n      \"feature\": \"quotient replay\",\n      \"count\": 1,\n      \"decls_capped\": false,\n      \"decls\": [\n        \"C\"\n      ]\n    }\n  ],\n  \"rejected\": [\n    {\n      \"name\": \"D\",\n      \"reason\": \"type mismatch\"\n    }\n  ]\n}\n";
        assert_eq!(out, expected);
    }

    #[test]
    fn render_report_without_meta_emits_nulls() {
        let pins = EnvironmentPins::without_meta("1.0.0");
        let input = InputMeta {
            path: "x".to_string(),
            sha256: None,
            size_bytes: None,
        };
        let summary = Summary::default();
        let acc = ReportAccumulator::new(false);
        let out = render_report(&pins, &input, &summary, &acc);
        assert!(out.contains("\"file_format_version\": null"));
        assert!(out.contains("\"sha256\": null"));
        assert!(out.contains("\"size_bytes\": null"));
        assert!(out.contains("\"unsupported_features\": []"));
        assert!(out.contains("\"rejected\": []"));
    }
}
