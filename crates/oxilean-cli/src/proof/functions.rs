//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{collect_const_names, Declaration, Environment, Expr, Name, TypeChecker};

use super::types::{
    AnnotatedProof, AnnotatedTrace, AnnotationKind, BatchChecker, DisplayGoal, ExportFormat,
    GoalShape, ProofAnnotation, ProofAttemptLog, ProofChecker, ProofComplexityMetrics,
    ProofDepGraph, ProofGoalHistory, ProofHint, ProofOptions, ProofProgress, ProofRecord,
    ProofReplayResult, ProofSearchState, ProofSession, ProofSessionV2, ProofStatus, ProofStatusV2,
    ProofStep, ProofSummary, ProofTrace, ProofValidationError, TacticSuggestion,
};

/// Pretty-print an expression as a proof term.
pub fn display_proof_term(expr: &Expr) -> String {
    format!("{:?}", expr)
}
/// Format a proof record for CLI output.
pub fn format_proof_record(record: &ProofRecord, use_color: bool) -> String {
    let status_str = if use_color {
        match &record.status {
            ProofStatus::Verified => "\x1b[32mverified\x1b[0m".to_string(),
            ProofStatus::Partial => "\x1b[33mpartial\x1b[0m".to_string(),
            ProofStatus::Failed(msg) => format!("\x1b[31mfailed: {}\x1b[0m", msg),
            ProofStatus::Unchecked => "\x1b[90munchecked\x1b[0m".to_string(),
        }
    } else {
        record.status.to_string()
    };
    format!("{}: {}", record.name, status_str)
}
/// Export a `ProofSummary` in the given format.
pub fn export_summary(summary: &ProofSummary, format: ExportFormat) -> String {
    match format {
        ExportFormat::Text => export_text(summary),
        ExportFormat::Json => export_json(summary),
        ExportFormat::Lean4 => export_lean4(summary),
        ExportFormat::Markdown => export_markdown(summary),
    }
}
fn export_text(summary: &ProofSummary) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Total: {} | Verified: {} | Failed: {} | Partial: {} | Unchecked: {}\n",
        summary.total(),
        summary.verified_count(),
        summary.failed_count(),
        summary.partial_count(),
        summary.unchecked_count()
    ));
    for record in summary.iter() {
        out.push_str(&format!("  {}\n", record));
    }
    out
}
fn export_json(summary: &ProofSummary) -> String {
    let mut entries = Vec::new();
    for record in summary.iter() {
        let status = match &record.status {
            ProofStatus::Verified => "\"verified\"".to_string(),
            ProofStatus::Partial => "\"partial\"".to_string(),
            ProofStatus::Failed(msg) => {
                format!("\"failed: {}\"", msg.replace('"', "\\\""))
            }
            ProofStatus::Unchecked => "\"unchecked\"".to_string(),
        };
        entries.push(format!(
            "{{\"name\":\"{}\",\"status\":{}}}",
            record.name, status
        ));
    }
    format!(
        "{{\"total\":{},\"verified\":{},\"failed\":{},\"records\":[{}]}}",
        summary.total(),
        summary.verified_count(),
        summary.failed_count(),
        entries.join(",")
    )
}
fn export_lean4(summary: &ProofSummary) -> String {
    let mut out = String::from("-- Proof status report\n");
    for record in summary.iter() {
        let comment = match &record.status {
            ProofStatus::Verified => "-- verified".to_string(),
            ProofStatus::Partial => "-- partial (sorry)".to_string(),
            ProofStatus::Failed(msg) => format!("-- FAILED: {}", msg),
            ProofStatus::Unchecked => "-- unchecked".to_string(),
        };
        out.push_str(&format!("{} {}\n", comment, record.name));
    }
    out
}
fn export_markdown(summary: &ProofSummary) -> String {
    let mut out = String::from("# Proof Status Report\n\n");
    out.push_str(
        &format!(
            "| Metric | Value |\n|--------|-------|\n| Total | {} |\n| Verified | {} |\n| Failed | {} |\n\n",
            summary.total(), summary.verified_count(), summary.failed_count()
        ),
    );
    out.push_str("## Details\n\n| Name | Status |\n|------|--------|\n");
    for record in summary.iter() {
        out.push_str(&format!("| `{}` | {} |\n", record.name, record.status));
    }
    out
}
#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::Literal;
    #[test]
    fn test_create_checker() {
        let env = Environment::new();
        let checker = ProofChecker::new(&env);
        assert!(checker.env().get(&Name::str("nonexistent")).is_none());
    }
    #[test]
    fn test_check_proof_not_found() {
        let env = Environment::new();
        let checker = ProofChecker::new(&env);
        let proof = Expr::Lit(Literal::Nat(42));
        let result = checker.check_proof(&Name::str("nonexistent"), &proof);
        assert!(result.is_err());
    }
    #[test]
    fn test_verify_all_empty() {
        let env = Environment::new();
        let checker = ProofChecker::new(&env);
        let result = checker.verify_all();
        assert!(result.is_ok());
        assert_eq!(result.expect("test operation should succeed").len(), 0);
    }
    #[test]
    fn test_proof_status_display() {
        assert_eq!(ProofStatus::Verified.to_string(), "verified");
        assert_eq!(ProofStatus::Partial.to_string(), "partial (sorry)");
        assert!(ProofStatus::Failed("bad".to_string())
            .to_string()
            .contains("bad"));
        assert_eq!(ProofStatus::Unchecked.to_string(), "unchecked");
    }
    #[test]
    fn test_proof_status_predicates() {
        assert!(ProofStatus::Verified.is_verified());
        assert!(!ProofStatus::Verified.is_failed());
        assert!(ProofStatus::Partial.is_partial());
        assert!(ProofStatus::Failed("x".to_string()).is_failed());
        assert!(ProofStatus::Unchecked.is_unchecked());
    }
    #[test]
    fn test_proof_record_transitions() {
        let r = ProofRecord::new(Name::str("myThm"));
        assert!(r.status.is_unchecked());
        let r2 = r.mark_verified();
        assert!(r2.status.is_verified());
        let r3 = ProofRecord::new(Name::str("myThm2")).mark_failed("oops");
        assert!(r3.status.is_failed());
        let r4 = ProofRecord::new(Name::str("myThm3")).mark_partial();
        assert!(r4.status.is_partial());
    }
    #[test]
    fn test_proof_record_with_detail() {
        let r = ProofRecord::new(Name::str("foo"))
            .mark_verified()
            .with_detail("took 12ms");
        assert_eq!(r.detail.as_deref(), Some("took 12ms"));
        assert!(r.to_string().contains("took 12ms"));
    }
    #[test]
    fn test_proof_summary_counts() {
        let mut summary = ProofSummary::new();
        summary.add(ProofRecord::new(Name::str("a")).mark_verified());
        summary.add(ProofRecord::new(Name::str("b")).mark_verified());
        summary.add(ProofRecord::new(Name::str("c")).mark_failed("err"));
        summary.add(ProofRecord::new(Name::str("d")).mark_partial());
        summary.add(ProofRecord::new(Name::str("e")));
        assert_eq!(summary.total(), 5);
        assert_eq!(summary.verified_count(), 2);
        assert_eq!(summary.failed_count(), 1);
        assert_eq!(summary.partial_count(), 1);
        assert_eq!(summary.unchecked_count(), 1);
        assert!(!summary.all_verified());
    }
    #[test]
    fn test_proof_summary_all_verified() {
        let mut summary = ProofSummary::new();
        summary.add(ProofRecord::new(Name::str("a")).mark_verified());
        summary.add(ProofRecord::new(Name::str("b")).mark_verified());
        assert!(summary.all_verified());
    }
    #[test]
    fn test_proof_summary_failed_names() {
        let mut summary = ProofSummary::new();
        summary.add(ProofRecord::new(Name::str("ok")).mark_verified());
        summary.add(ProofRecord::new(Name::str("bad")).mark_failed("err"));
        let failed = summary.failed_names();
        assert_eq!(failed.len(), 1);
        assert_eq!(failed[0].to_string(), "bad");
    }
    #[test]
    fn test_export_text() {
        let mut summary = ProofSummary::new();
        summary.add(ProofRecord::new(Name::str("a")).mark_verified());
        let s = export_summary(&summary, ExportFormat::Text);
        assert!(s.contains("Total:"));
        assert!(s.contains("verified"));
    }
    #[test]
    fn test_export_json() {
        let mut summary = ProofSummary::new();
        summary.add(ProofRecord::new(Name::str("a")).mark_verified());
        let s = export_summary(&summary, ExportFormat::Json);
        assert!(s.contains("total"));
        assert!(s.contains("verified"));
    }
    #[test]
    fn test_export_markdown() {
        let mut summary = ProofSummary::new();
        summary.add(ProofRecord::new(Name::str("a")).mark_verified());
        let s = export_summary(&summary, ExportFormat::Markdown);
        assert!(s.contains("# Proof Status Report"));
    }
    #[test]
    fn test_export_lean4() {
        let mut summary = ProofSummary::new();
        summary.add(ProofRecord::new(Name::str("myThm")).mark_verified());
        let s = export_summary(&summary, ExportFormat::Lean4);
        assert!(s.contains("-- verified"));
        assert!(s.contains("myThm"));
    }
    #[test]
    fn test_export_format_from_extension() {
        assert_eq!(
            ExportFormat::from_extension("json"),
            Some(ExportFormat::Json)
        );
        assert_eq!(
            ExportFormat::from_extension("lean"),
            Some(ExportFormat::Lean4)
        );
        assert_eq!(
            ExportFormat::from_extension("md"),
            Some(ExportFormat::Markdown)
        );
        assert_eq!(ExportFormat::from_extension("xyz"), None);
    }
    #[test]
    fn test_proof_step() {
        let step = ProofStep::new(1, "intro h", 2, 1, true);
        let s = step.to_string();
        assert!(s.contains("intro h"));
        assert!(s.contains("OK"));
        assert_eq!(step.goals_eliminated(), 1);
        assert!(!step.closed_all());
        let step2 = ProofStep::new(2, "exact h", 1, 0, true);
        assert!(step2.closed_all());
    }
    #[test]
    fn test_proof_trace_complete() {
        let mut trace = ProofTrace::new();
        assert!(trace.is_empty());
        trace.push(ProofStep::new(1, "intro", 1, 0, true));
        assert!(trace.is_complete());
        assert_eq!(trace.successful_steps(), 1);
        assert_eq!(trace.failed_steps(), 0);
    }
    #[test]
    fn test_proof_trace_incomplete() {
        let mut trace = ProofTrace::new();
        trace.push(ProofStep::new(1, "intro", 2, 1, true));
        assert!(!trace.is_complete());
    }
    #[test]
    fn test_display_goal_format() {
        let mut goal = DisplayGoal::new(1, "P and Q");
        goal.add_hyp("h1", "P");
        goal.add_hyp("h2", "Q");
        let s = goal.to_string();
        assert!(s.contains("Goal 1:"));
        assert!(s.contains("h1 : P"));
    }
    #[test]
    fn test_proof_options_defaults() {
        let opts = ProofOptions::default();
        assert!(!opts.sorry_is_error);
        assert_eq!(opts.max_depth, 1024);
    }
    #[test]
    fn test_proof_options_strict() {
        let opts = ProofOptions::strict();
        assert!(opts.sorry_is_error);
    }
    #[test]
    fn test_checker_declaration_count() {
        let env = Environment::new();
        let checker = ProofChecker::new(&env);
        assert_eq!(checker.declaration_count(), 0);
    }
    #[test]
    fn test_checker_is_declared() {
        let env = Environment::new();
        let checker = ProofChecker::new(&env);
        assert!(!checker.is_declared(&Name::str("nonexistent")));
    }
    #[test]
    fn test_batch_checker_empty() {
        let env = Environment::new();
        let checker = BatchChecker::new(&env);
        let summary = checker.check_names(&[]);
        assert_eq!(summary.total(), 0);
    }
    #[test]
    fn test_format_proof_record_no_color() {
        let record = ProofRecord::new(Name::str("myThm")).mark_verified();
        let s = format_proof_record(&record, false);
        assert!(s.contains("myThm"));
        assert!(s.contains("verified"));
    }
    #[test]
    fn test_format_proof_record_with_color() {
        let record = ProofRecord::new(Name::str("myThm")).mark_failed("bad");
        let s = format_proof_record(&record, true);
        assert!(s.contains("\x1b["));
    }
}
/// Export a `ProofTrace` as a simple text log.
pub fn trace_to_text(trace: &ProofTrace) -> String {
    let mut out = String::new();
    for step in trace.iter() {
        out.push_str(&format!("{}\n", step));
    }
    if trace.is_complete() {
        out.push_str("QED\n");
    } else {
        out.push_str("(incomplete)\n");
    }
    out
}
/// Export a `ProofTrace` as a Lean 4 `by` block.
///
/// Successful tactic steps are emitted verbatim, indented by two spaces.
/// Failed steps are commented out with a `-- FAILED:` prefix so that the
/// output remains syntactically valid Lean 4 while still preserving the
/// full trace for debugging.
/// If the proof is complete (all goals closed), a trailing `-- QED` comment
/// is appended; otherwise a `-- proof incomplete` comment is added.
pub fn trace_to_lean4(trace: &ProofTrace) -> String {
    let mut out = String::from(
        "by
",
    );
    for step in trace.iter() {
        if step.success {
            out.push_str(&format!(
                "  {}
",
                step.tactic
            ));
        } else {
            out.push_str(&format!(
                "  -- FAILED: {}
",
                step.tactic
            ));
        }
    }
    if trace.is_complete() {
        out.push_str(
            "  -- QED
",
        );
    } else {
        out.push_str(
            "  -- proof incomplete
",
        );
    }
    out
}
#[cfg(test)]
mod new_proof_tests {
    use super::*;
    #[test]
    fn test_proof_progress_empty() {
        let p = ProofProgress::new();
        assert!(p.is_empty());
        assert!(p.initial_goals().is_none());
        assert!(!p.is_complete());
    }
    #[test]
    fn test_proof_progress_record() {
        let mut p = ProofProgress::new();
        p.record(3);
        p.record(1);
        p.record(0);
        assert_eq!(p.initial_goals(), Some(3));
        assert_eq!(p.final_goals(), Some(0));
        assert!(p.is_complete());
        assert_eq!(p.goals_eliminated(), 3);
    }
    #[test]
    fn test_proof_progress_not_complete() {
        let mut p = ProofProgress::new();
        p.record(2);
        p.record(1);
        assert!(!p.is_complete());
    }
    #[test]
    fn test_proof_annotation_display() {
        let ann = ProofAnnotation::note(1, "This is a note");
        let s = format!("{}", ann);
        assert!(s.contains("note"));
        assert!(s.contains("This is a note"));
    }
    #[test]
    fn test_annotation_kind_display() {
        assert_eq!(AnnotationKind::Note.to_string(), "note");
        assert_eq!(AnnotationKind::Warning.to_string(), "warning");
        assert_eq!(AnnotationKind::Reference.to_string(), "reference");
    }
    #[test]
    fn test_annotated_trace_basic() {
        let mut at = AnnotatedTrace::new();
        at.push(ProofStep::new(1, "intro", 1, 0, true));
        at.annotate(ProofAnnotation::note(1, "introduce h"));
        assert!(at.is_complete());
        assert_eq!(at.annotation_count(), 1);
        let anns = at.annotations_for(1);
        assert_eq!(anns.len(), 1);
    }
    #[test]
    fn test_proof_session_steps() {
        let mut sess = ProofSession::new(Name::str("myThm"), 2);
        assert_eq!(sess.current_goals(), 2);
        sess.apply_step("intro h", 1, true);
        sess.apply_step("exact h", 0, true);
        assert!(sess.is_complete());
        assert_eq!(sess.num_steps(), 2);
        assert!(sess.status.is_verified());
    }
    #[test]
    fn test_proof_session_to_record() {
        let mut sess = ProofSession::new(Name::str("foo"), 1);
        sess.apply_step("exact rfl", 0, true);
        let rec = sess.to_record();
        assert!(rec.detail.is_some());
        assert!(rec
            .detail
            .as_deref()
            .expect("type conversion should succeed")
            .contains("steps"));
    }
    #[test]
    fn test_trace_to_text() {
        let mut trace = ProofTrace::new();
        trace.push(ProofStep::new(1, "intro", 1, 0, true));
        let text = trace_to_text(&trace);
        assert!(text.contains("intro"));
        assert!(text.contains("QED"));
    }
    #[test]
    fn test_trace_to_lean4() {
        let mut trace = ProofTrace::new();
        trace.push(ProofStep::new(1, "simp", 1, 0, true));
        let lean4 = trace_to_lean4(&trace);
        assert!(lean4.starts_with("by\n"));
        assert!(lean4.contains("simp"));
    }
    #[test]
    fn test_proof_progress_snapshots() {
        let mut p = ProofProgress::new();
        p.record(5);
        p.record(3);
        p.record(1);
        assert_eq!(p.snapshots().len(), 3);
        assert_eq!(p.len(), 3);
    }
}
#[allow(dead_code)]
pub fn compress_proof_steps(steps: &[ProofStep]) -> Vec<&ProofStep> {
    let mut seen_tactics = std::collections::HashSet::new();
    let mut compressed = Vec::new();
    for step in steps {
        let key = format!("{}-{}", step.tactic, step.goals_after);
        if seen_tactics.insert(key) {
            compressed.push(step);
        }
    }
    compressed
}
#[allow(dead_code)]
pub fn replay_proof_trace(trace: &ProofTrace) -> ProofReplayResult {
    let mut succeeded = true;
    let mut failed_at = None;
    for step in trace.steps() {
        if !step.success {
            succeeded = false;
            failed_at = Some(step.step);
            break;
        }
    }
    ProofReplayResult {
        replayed_steps: if succeeded {
            trace.steps().len()
        } else {
            failed_at.unwrap_or(0)
        },
        failed_at,
        succeeded,
    }
}
#[allow(dead_code)]
pub fn merge_proof_traces(a: &ProofTrace, b: &ProofTrace) -> ProofTrace {
    let mut merged = ProofTrace::new();
    let mut offset = 0usize;
    for step in a.steps() {
        merged.push(ProofStep::new(
            step.step + offset,
            &step.tactic,
            step.goals_before,
            step.goals_after,
            step.success,
        ));
        offset = offset.max(step.step + 1);
    }
    for step in b.steps() {
        merged.push(ProofStep::new(
            step.step + offset,
            &step.tactic,
            step.goals_before,
            step.goals_after,
            step.success,
        ));
    }
    merged
}
#[allow(dead_code)]
pub fn format_proof_as_tactic_block(trace: &ProofTrace) -> String {
    let mut out = String::from("by\n");
    for step in trace.steps() {
        out.push_str(&format!("  {}\n", step.tactic));
    }
    out
}
#[allow(dead_code)]
pub fn format_proof_as_table(trace: &ProofTrace) -> String {
    let mut out = format!(
        "{:<5} {:<20} {:<8} {:<8}\n",
        "Step", "Tactic", "Before", "After"
    );
    out.push_str(&"-".repeat(45));
    out.push('\n');
    for step in trace.steps() {
        out.push_str(&format!(
            "{:<5} {:<20} {:<8} {:<8}\n",
            step.step, step.tactic, step.goals_before, step.goals_after
        ));
    }
    out
}
#[allow(dead_code)]
pub fn format_proof_as_json(trace: &ProofTrace) -> String {
    let steps: Vec<String> = trace
        .steps()
        .iter()
        .map(|s| {
            format!(
                "{{\"step\":{},\"tactic\":\"{}\",\"goals_before\":{},\"goals_after\":{},\"ok\":{}}}",
                s.step, s.tactic, s.goals_before, s.goals_after, s.success
            )
        })
        .collect();
    format!("{{\"steps\":[{}]}}", steps.join(","))
}
#[cfg(test)]
mod proof_extended_tests {
    use super::*;
    fn make_step(id: usize, tactic: &str, before: usize, after: usize) -> ProofStep {
        ProofStep::new(id, tactic, before, after, true)
    }
    #[test]
    fn test_annotated_proof_markdown() {
        let steps = vec![make_step(0, "intro h", 1, 1), make_step(1, "exact h", 1, 0)];
        let mut ap = AnnotatedProof::new(steps);
        ap.annotate(0, "note", "Introduce hypothesis");
        let md = ap.to_markdown();
        assert!(md.contains("**Step 0**"));
        assert!(md.contains("Introduce hypothesis"));
    }
    #[test]
    fn test_annotations_for_step() {
        let steps = vec![make_step(0, "intro", 1, 1)];
        let mut ap = AnnotatedProof::new(steps);
        ap.annotate(0, "key", "value");
        ap.annotate(0, "key2", "value2");
        let anns = ap.annotations_for(0);
        assert_eq!(anns.len(), 2);
    }
    #[test]
    fn test_dep_graph_independent() {
        let mut g = ProofDepGraph::new();
        g.add_dep(1, 0);
        assert!(!g.is_independent(1));
        assert!(g.is_independent(0));
    }
    #[test]
    fn test_compress_proof_steps() {
        let steps = vec![
            make_step(0, "intro", 2, 1),
            make_step(1, "intro", 2, 1),
            make_step(2, "exact", 1, 0),
        ];
        let compressed = compress_proof_steps(&steps);
        assert_eq!(compressed.len(), 2);
    }
    #[test]
    fn test_replay_succeeded() {
        let mut trace = ProofTrace::new();
        trace.push(make_step(0, "intro", 1, 1));
        trace.push(make_step(1, "exact h", 1, 0));
        let result = replay_proof_trace(&trace);
        assert!(result.succeeded);
        assert_eq!(result.replayed_steps, 2);
    }
    #[test]
    fn test_replay_fails_on_bad_step() {
        let mut trace = ProofTrace::new();
        trace.push(ProofStep::new(0, "intro", 1, 1, true));
        trace.push(ProofStep::new(1, "bad_tactic", 1, 1, false));
        let result = replay_proof_trace(&trace);
        assert!(!result.succeeded);
        assert_eq!(result.failed_at, Some(1));
    }
    #[test]
    fn test_merge_traces() {
        let mut t1 = ProofTrace::new();
        t1.push(make_step(0, "intro", 1, 1));
        let mut t2 = ProofTrace::new();
        t2.push(make_step(0, "exact", 1, 0));
        let merged = merge_proof_traces(&t1, &t2);
        assert_eq!(merged.steps().len(), 2);
    }
    #[test]
    fn test_goal_history_trend() {
        let mut hist = ProofGoalHistory::new();
        hist.record(0, 3, "start");
        hist.record(1, 2, "after intro");
        hist.record(2, 0, "done");
        let trend = hist.trend();
        assert_eq!(trend.len(), 3);
        assert_eq!(hist.min_goals(), 0);
        assert_eq!(hist.max_goals(), 3);
    }
    #[test]
    fn test_format_proof_as_tactic_block() {
        let mut trace = ProofTrace::new();
        trace.push(make_step(0, "intro h", 1, 1));
        trace.push(make_step(1, "exact h", 1, 0));
        let block = format_proof_as_tactic_block(&trace);
        assert!(block.starts_with("by\n"));
        assert!(block.contains("intro h"));
        assert!(block.contains("exact h"));
    }
    #[test]
    fn test_format_proof_as_json() {
        let mut trace = ProofTrace::new();
        trace.push(make_step(0, "rfl", 1, 0));
        let json = format_proof_as_json(&trace);
        assert!(json.contains("\"steps\""));
        assert!(json.contains("rfl"));
    }
    #[test]
    fn test_format_proof_as_table() {
        let mut trace = ProofTrace::new();
        trace.push(make_step(0, "exact", 1, 0));
        let table = format_proof_as_table(&trace);
        assert!(table.contains("Step"));
        assert!(table.contains("exact"));
    }
}
#[allow(dead_code)]
pub fn estimate_proof_complexity(trace: &ProofTrace) -> ProofComplexityMetrics {
    let steps = trace.steps();
    let mut unique_tactics = std::collections::HashSet::new();
    let mut max_depth = 0usize;
    let mut has_sorry = false;
    let mut total_chars = 0usize;
    let mut branch_increases = 0usize;
    for step in steps {
        unique_tactics.insert(step.tactic.as_str());
        if step.goals_after > step.goals_before {
            branch_increases += 1;
        }
        if step.goals_before > max_depth {
            max_depth = step.goals_before;
        }
        if step.tactic.contains("sorry") {
            has_sorry = true;
        }
        total_chars += step.tactic.len();
    }
    ProofComplexityMetrics {
        num_steps: steps.len(),
        max_goal_depth: max_depth,
        branching_factor: if steps.is_empty() {
            0.0
        } else {
            branch_increases as f64 / steps.len() as f64
        },
        unique_tactics: unique_tactics.len(),
        has_sorry,
        proof_length_chars: total_chars,
    }
}
#[allow(dead_code)]
pub fn suggest_next_tactics(
    goals_before: usize,
    last_tactic: Option<&str>,
) -> Vec<TacticSuggestion> {
    let mut suggestions = Vec::new();
    if goals_before == 0 {
        return suggestions;
    }
    if let Some(last) = last_tactic {
        if last.contains("intro") {
            suggestions.push(TacticSuggestion::new(
                "assumption",
                "hypothesis may now match goal",
                0.8,
            ));
            suggestions.push(TacticSuggestion::new(
                "exact",
                "use introduced hypothesis",
                0.7,
            ));
        } else if last.contains("apply") {
            suggestions.push(TacticSuggestion::new(
                "intro",
                "apply created subgoal needing binder",
                0.75,
            ));
            suggestions.push(TacticSuggestion::new(
                "assumption",
                "check if hypothesis closes new goal",
                0.6,
            ));
        } else if last.contains("cases") || last.contains("induction") {
            suggestions.push(TacticSuggestion::new(
                "simp",
                "simplify after case split",
                0.65,
            ));
            suggestions.push(TacticSuggestion::new(
                "exact",
                "goal may be trivial after split",
                0.5,
            ));
        }
    }
    suggestions.push(TacticSuggestion::new("rfl", "goal may be reflexivity", 0.4));
    suggestions.push(TacticSuggestion::new(
        "simp",
        "simplification often helps",
        0.35,
    ));
    suggestions.push(TacticSuggestion::new(
        "sorry",
        "placeholder to continue",
        0.1,
    ));
    suggestions
}
#[allow(dead_code)]
pub fn greedy_proof_search(
    initial_goals: usize,
    max_depth: usize,
    tactics_to_try: &[(&str, usize)],
) -> Option<ProofTrace> {
    let mut state = ProofSearchState::initial(initial_goals);
    for _ in 0..max_depth {
        if state.is_complete() {
            return Some(state.trace);
        }
        let best = tactics_to_try
            .iter()
            .filter(|(_, after)| *after < state.goals_remaining)
            .min_by_key(|(_, after)| *after);
        match best {
            Some((tactic, after)) => {
                state = state.apply_tactic(tactic, *after);
            }
            None => break,
        }
    }
    if state.is_complete() {
        Some(state.trace)
    } else {
        None
    }
}
#[allow(dead_code)]
pub fn validate_proof_trace(trace: &ProofTrace) -> Vec<ProofValidationError> {
    let mut errors = Vec::new();
    let steps = trace.steps();
    for (i, step) in steps.iter().enumerate() {
        if i == 0 && step.goals_before == 0 {
            errors.push(ProofValidationError {
                step_id: step.step,
                message: "First step should have at least one goal".to_string(),
            });
        }
        if step.tactic.contains("sorry") {
            errors.push(ProofValidationError {
                step_id: step.step,
                message: format!("Step {} uses sorry — proof is incomplete", step.step),
            });
        }
        if !step.success && step.goals_after > 0 {
            errors.push(ProofValidationError {
                step_id: step.step,
                message: format!(
                    "Step {} failed but reports {} remaining goals",
                    step.step, step.goals_after
                ),
            });
        }
    }
    if let Some(last) = steps.last() {
        if last.success && last.goals_after != 0 {
            errors.push(ProofValidationError {
                step_id: last.step,
                message: "Proof trace ends with unclosed goals".to_string(),
            });
        }
    }
    errors
}
#[allow(dead_code)]
pub fn export_proof_to_coq(trace: &ProofTrace, theorem_name: &str) -> String {
    let mut out = format!("Theorem {} : (* type here *).\nProof.\n", theorem_name);
    for step in trace.steps() {
        let coq_tactic = match step.tactic.as_str() {
            "intro" | "intros" => "intros",
            "exact" => "exact",
            "apply" => "apply",
            "simp" => "simpl",
            "rfl" => "reflexivity",
            "sorry" => "admit",
            t => t,
        };
        out.push_str(&format!("  {}.\n", coq_tactic));
    }
    out.push_str("Qed.\n");
    out
}
#[allow(dead_code)]
pub fn export_proof_to_isabelle(trace: &ProofTrace, theorem_name: &str) -> String {
    let mut out = format!("lemma {}: (* statement here *)\n", theorem_name);
    if trace.steps().is_empty() {
        out.push_str("  by simp\n");
    } else {
        out.push_str("proof -\n");
        for step in trace.steps() {
            out.push_str(&format!("  (* {} *)\n", step.tactic));
        }
        out.push_str("qed\n");
    }
    out
}
#[cfg(test)]
mod proof_extended2_tests {
    use super::*;
    fn make_step(id: usize, tactic: &str, before: usize, after: usize) -> ProofStep {
        ProofStep::new(id, tactic, before, after, true)
    }
    #[test]
    fn test_complexity_metrics_no_sorry() {
        let mut trace = ProofTrace::new();
        trace.push(make_step(0, "intro h", 1, 1));
        trace.push(make_step(1, "exact h", 1, 0));
        let metrics = estimate_proof_complexity(&trace);
        assert_eq!(metrics.num_steps, 2);
        assert!(!metrics.has_sorry);
        assert_eq!(metrics.unique_tactics, 2);
    }
    #[test]
    fn test_complexity_metrics_with_sorry() {
        let mut trace = ProofTrace::new();
        trace.push(ProofStep::new(0, "sorry", 1, 0, true));
        let metrics = estimate_proof_complexity(&trace);
        assert!(metrics.has_sorry);
    }
    #[test]
    fn test_suggest_after_intro() {
        let suggestions = suggest_next_tactics(1, Some("intro h"));
        assert!(!suggestions.is_empty());
        assert!(suggestions.iter().any(|s| s.tactic == "assumption"));
    }
    #[test]
    fn test_suggest_no_goals() {
        let suggestions = suggest_next_tactics(0, None);
        assert!(suggestions.is_empty());
    }
    #[test]
    fn test_proof_search_simple() {
        let tactics = &[("intro", 1), ("exact", 0)];
        let result = greedy_proof_search(2, 10, tactics);
        assert!(result.is_some());
    }
    #[test]
    fn test_proof_search_impossible() {
        let tactics = &[("intro", 1)];
        let result = greedy_proof_search(1, 3, tactics);
        assert!(result.is_none());
    }
    #[test]
    fn test_validate_proof_with_sorry() {
        let mut trace = ProofTrace::new();
        trace.push(ProofStep::new(0, "sorry", 1, 0, true));
        let errors = validate_proof_trace(&trace);
        assert!(errors.iter().any(|e| e.message.contains("sorry")));
    }
    #[test]
    fn test_validate_unclosed_goals() {
        let mut trace = ProofTrace::new();
        trace.push(make_step(0, "intro", 2, 1));
        let errors = validate_proof_trace(&trace);
        assert!(errors.iter().any(|e| e.message.contains("unclosed goals")));
    }
    #[test]
    fn test_export_coq() {
        let mut trace = ProofTrace::new();
        trace.push(make_step(0, "intro", 1, 1));
        trace.push(make_step(1, "exact", 1, 0));
        let coq = export_proof_to_coq(&trace, "my_thm");
        assert!(coq.contains("Theorem my_thm"));
        assert!(coq.contains("Qed."));
        assert!(coq.contains("intros."));
    }
    #[test]
    fn test_export_isabelle_empty() {
        let trace = ProofTrace::new();
        let isa = export_proof_to_isabelle(&trace, "my_lem");
        assert!(isa.contains("lemma my_lem"));
        assert!(isa.contains("by simp"));
    }
    #[test]
    fn test_search_state_apply_tactic() {
        let state = ProofSearchState::initial(2);
        let next = state.apply_tactic("cases", 1);
        assert_eq!(next.goals_remaining, 1);
        assert_eq!(next.depth, 1);
    }
}
#[allow(dead_code)]
pub fn classify_goal_shape(goal_str: &str) -> GoalShape {
    let s = goal_str.trim();
    if s.contains(" = ") && !s.contains("->") {
        GoalShape::Equality
    } else if s.contains("->") || s.contains("→") {
        GoalShape::Implication
    } else if s.contains("∧") || s.contains("And") {
        GoalShape::Conjunction
    } else if s.contains("∨") || s.contains("Or") {
        GoalShape::Disjunction
    } else if s.contains("∀") || s.starts_with("forall") {
        GoalShape::Universal
    } else if s.contains("∃") || s.starts_with("exists") {
        GoalShape::Existential
    } else if s.contains("¬") || s.starts_with("Not") {
        GoalShape::Negation
    } else if !s.contains(' ') {
        GoalShape::Atomic
    } else {
        GoalShape::Unknown
    }
}
#[allow(dead_code)]
pub fn heuristic_tactic_for_shape(shape: &GoalShape) -> Vec<&'static str> {
    match shape {
        GoalShape::Equality => vec!["rfl", "simp", "ring", "linarith"],
        GoalShape::Implication => vec!["intro", "intros", "exact"],
        GoalShape::Conjunction => vec!["constructor", "split", "exact"],
        GoalShape::Disjunction => vec!["left", "right", "cases"],
        GoalShape::Universal => vec!["intro", "intros"],
        GoalShape::Existential => vec!["exists", "use"],
        GoalShape::Negation => vec!["intro", "by_contra", "push_neg"],
        GoalShape::Atomic => vec!["assumption", "exact", "rfl"],
        GoalShape::Unknown => vec!["simp", "assumption", "sorry"],
    }
}
#[cfg(test)]
mod proof_heuristic_and_session_tests {
    use super::*;
    #[test]
    fn test_classify_equality() {
        assert_eq!(classify_goal_shape("a = b"), GoalShape::Equality);
    }
    #[test]
    fn test_classify_implication() {
        assert_eq!(classify_goal_shape("P -> Q"), GoalShape::Implication);
    }
    #[test]
    fn test_classify_conjunction() {
        assert_eq!(classify_goal_shape("P ∧ Q"), GoalShape::Conjunction);
    }
    #[test]
    fn test_classify_universal() {
        assert_eq!(classify_goal_shape("∀ x, P x"), GoalShape::Universal);
    }
    #[test]
    fn test_classify_existential() {
        assert_eq!(classify_goal_shape("∃ x, P x"), GoalShape::Existential);
    }
    #[test]
    fn test_heuristic_for_equality() {
        let tactics = heuristic_tactic_for_shape(&GoalShape::Equality);
        assert!(tactics.contains(&"rfl"));
        assert!(tactics.contains(&"ring"));
    }
    #[test]
    fn test_heuristic_for_implication() {
        let tactics = heuristic_tactic_for_shape(&GoalShape::Implication);
        assert!(tactics.contains(&"intro"));
    }
    #[test]
    fn test_proof_session_apply_and_complete() {
        let mut sess = ProofSessionV2::start("my_thm", 1);
        assert!(!sess.session_complete);
        sess.apply("exact h", 0);
        assert!(sess.session_complete);
        assert_eq!(sess.trace.steps().len(), 1);
    }
    #[test]
    fn test_proof_session_summary_contains_name() {
        let sess = ProofSessionV2::start("foo_lemma", 2);
        let summary = sess.summary();
        assert!(summary.contains("foo_lemma"));
        assert!(summary.contains("Goals: 2"));
    }
    #[test]
    fn test_classify_negation() {
        assert_eq!(classify_goal_shape("¬ P"), GoalShape::Negation);
    }
    #[test]
    fn test_heuristic_for_existential() {
        let tactics = heuristic_tactic_for_shape(&GoalShape::Existential);
        assert!(tactics.contains(&"exists"));
    }
}
#[cfg(test)]
mod proof_attempt_tests {
    use super::*;
    #[test]
    fn test_proof_status_terminal() {
        assert!(ProofStatusV2::Complete.is_terminal());
        assert!(ProofStatusV2::Failed("err".to_string()).is_terminal());
        assert!(!ProofStatusV2::InProgress.is_terminal());
    }
    #[test]
    fn test_attempt_log() {
        let mut log = ProofAttemptLog::new();
        let id = log.start_attempt(1000);
        if let Some(a) = log.get_attempt_mut(id) {
            a.add_tactic("intro h");
            a.add_tactic("exact h");
            a.status = ProofStatusV2::Complete;
        }
        assert_eq!(log.successful_attempts().len(), 1);
    }
}
#[allow(dead_code)]
pub fn generate_basic_hints(goal_type: &str) -> Vec<ProofHint> {
    let mut hints = Vec::new();
    if goal_type.contains("forall") || goal_type.contains("∀") {
        hints.push(ProofHint::new(
            "intro",
            "goal is a universal statement",
            0.9,
        ));
    }
    if goal_type.contains("Exists") || goal_type.contains("∃") {
        hints.push(ProofHint::new(
            "use",
            "goal is existential; provide a witness",
            0.85,
        ));
    }
    if goal_type.contains("And") || goal_type.contains("∧") {
        hints.push(ProofHint::new("constructor", "goal is a conjunction", 0.8));
    }
    if goal_type.contains("Or") || goal_type.contains("∨") {
        hints.push(ProofHint::new(
            "left",
            "goal is a disjunction; try left branch",
            0.5,
        ));
    }
    if goal_type.starts_with("True") {
        hints.push(ProofHint::new("trivial", "goal is trivially true", 0.99));
    }
    hints
}
#[cfg(test)]
mod proof_hint_tests {
    use super::*;
    #[test]
    fn test_hint_display() {
        let h = ProofHint::new("intro", "universal", 0.9);
        assert!(h.display().contains("intro"));
        assert!(h.display().contains("90%"));
    }
    #[test]
    fn test_generate_hints_forall() {
        let hints = generate_basic_hints("forall x, P x");
        assert!(hints.iter().any(|h| h.tactic == "intro"));
    }
    #[test]
    fn test_generate_hints_and() {
        let hints = generate_basic_hints("And P Q");
        assert!(hints.iter().any(|h| h.tactic == "constructor"));
    }
}
