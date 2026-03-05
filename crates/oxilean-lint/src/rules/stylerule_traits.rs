//! # StyleRule - Trait Implementations
//!
//! This module contains trait implementations for `StyleRule`.
//!
//! ## Implemented Traits
//!
//! - `LintRule`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::framework::{
    collect_var_refs, is_camel_case, is_pascal_case, is_snake_case, lint_ids, to_snake_case,
    AutoFix, LintCategory, LintContext, LintDiagnostic, LintId, LintRule, Severity, SourceRange,
};

use super::types::StyleRule;

impl LintRule for StyleRule {
    fn id(&self) -> LintId {
        lint_ids::style()
    }
    fn name(&self) -> &str {
        "style"
    }
    fn description(&self) -> &str {
        "enforces code style rules (whitespace, line length, etc.)"
    }
    fn default_severity(&self) -> Severity {
        Severity::Hint
    }
    fn category(&self) -> LintCategory {
        LintCategory::Style
    }
    fn finalize(&self, ctx: &mut LintContext<'_>) {
        let source = ctx.source;
        let mut offset = 0;
        let mut blank_count = 0;
        for (line_num, line) in source.lines().enumerate() {
            let line_start = offset;
            if self.check_trailing_whitespace && line != line.trim_end() {
                let ws_start = line_start + line.trim_end().len();
                let ws_end = line_start + line.len();
                ctx.emit(
                    LintDiagnostic::new(
                        lint_ids::trailing_whitespace(),
                        Severity::Hint,
                        format!("trailing whitespace on line {}", line_num + 1),
                        SourceRange::new(ws_start, ws_end),
                    )
                    .with_fix(AutoFix::deletion(
                        "remove trailing whitespace",
                        SourceRange::new(ws_start, ws_end),
                    )),
                );
            }
            if line.len() > self.max_line_length {
                ctx.emit(LintDiagnostic::new(
                    lint_ids::long_line(),
                    Severity::Hint,
                    format!(
                        "line {} is {} characters long (max {})",
                        line_num + 1,
                        line.len(),
                        self.max_line_length
                    ),
                    SourceRange::new(line_start, line_start + line.len()),
                ));
            }
            if self.disallow_tabs && line.contains('\t') {
                ctx.emit(
                    LintDiagnostic::new(
                        lint_ids::inconsistent_indentation(),
                        Severity::Hint,
                        format!("tab character found on line {}", line_num + 1),
                        SourceRange::new(line_start, line_start + line.len()),
                    )
                    .with_note("use spaces instead of tabs"),
                );
            }
            if line.trim().is_empty() {
                blank_count += 1;
                if blank_count > self.max_blank_lines {
                    ctx.emit(LintDiagnostic::new(
                        self.id(),
                        Severity::Hint,
                        format!("more than {} consecutive blank lines", self.max_blank_lines),
                        SourceRange::new(line_start, line_start + line.len()),
                    ));
                }
            } else {
                blank_count = 0;
            }
            offset += line.len() + 1;
        }
        if self.require_final_newline && !source.is_empty() && !source.ends_with('\n') {
            ctx.emit(
                LintDiagnostic::new(
                    lint_ids::missing_final_newline(),
                    Severity::Hint,
                    "file does not end with a newline",
                    SourceRange::new(source.len(), source.len()),
                )
                .with_fix(AutoFix::insertion(
                    "add final newline",
                    source.len(),
                    "\n".to_string(),
                )),
            );
        }
    }
}
