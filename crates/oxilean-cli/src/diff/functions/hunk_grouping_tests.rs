//! Compares `group_into_hunks` with its previous form on every sequence of
//! up to eight line changes, for context widths 0 to 3, and checks the
//! tokenizer's handling of punctuation.

use super::super::types::{ChangeKind, DiffHunk, DiffLine, LineChange, OxiDiffToken};
use super::{group_into_hunks, oxi_tokenize_line};

fn previous_group_into_hunks(changes: &[LineChange], context: usize) -> Vec<DiffHunk> {
    if changes.is_empty() {
        return Vec::new();
    }
    let change_indices: Vec<usize> = changes
        .iter()
        .enumerate()
        .filter(|(_, c)| c.kind != ChangeKind::Unchanged)
        .map(|(i, _)| i)
        .collect();
    if change_indices.is_empty() {
        return Vec::new();
    }
    let mut groups: Vec<Vec<usize>> = Vec::new();
    let mut current_group: Vec<usize> = vec![change_indices[0]];
    for &idx in &change_indices[1..] {
        let prev = *current_group.last().expect("initialized with one element");
        if idx <= prev + 2 * context + 1 {
            current_group.push(idx);
        } else {
            groups.push(current_group);
            current_group = vec![idx];
        }
    }
    groups.push(current_group);
    let mut hunks = Vec::new();
    for group in &groups {
        let first = *group.first().expect("group is non-empty");
        let last = *group.last().expect("group is non-empty");
        let start = first.saturating_sub(context);
        let end = (last + context + 1).min(changes.len());
        let hunk_lc: Vec<LineChange> = changes[start..end].to_vec();
        let old_start = hunk_lc
            .iter()
            .filter(|l| l.old_lineno > 0)
            .map(|l| l.old_lineno)
            .min()
            .unwrap_or(1);
        let new_start = hunk_lc
            .iter()
            .filter(|l| l.new_lineno > 0)
            .map(|l| l.new_lineno)
            .min()
            .unwrap_or(1);
        let old_count = hunk_lc
            .iter()
            .filter(|l| l.kind != ChangeKind::Added)
            .count();
        let new_count = hunk_lc
            .iter()
            .filter(|l| l.kind != ChangeKind::Removed)
            .count();
        let hunk_lines: Vec<DiffLine> = hunk_lc.iter().map(|l| l.to_diff_line()).collect();
        hunks.push(DiffHunk {
            header: DiffHunk::make_header(old_start, old_count, new_start, new_count),
            old_start,
            old_count,
            new_start,
            new_count,
            lines: hunk_lines,
        });
    }
    hunks
}

/// The change sequence encoded by `code` in base 3, `len` changes long, with
/// consistent old and new line numbers.
fn changes_for(mut code: u32, len: usize) -> Vec<LineChange> {
    let (mut old_no, mut new_no) = (0, 0);
    let mut out = Vec::new();
    for i in 0..len {
        let kind = match code % 3 {
            0 => ChangeKind::Unchanged,
            1 => ChangeKind::Added,
            _ => ChangeKind::Removed,
        };
        code /= 3;
        let (o, n) = match kind {
            ChangeKind::Unchanged => {
                old_no += 1;
                new_no += 1;
                (old_no, new_no)
            }
            ChangeKind::Added => {
                new_no += 1;
                (0, new_no)
            }
            ChangeKind::Removed => {
                old_no += 1;
                (old_no, 0)
            }
        };
        out.push(LineChange::new(kind, format!("line {}", i), o, n));
    }
    out
}

#[test]
fn matches_the_previous_form_on_every_short_change_sequence() {
    for len in 0..=8usize {
        for code in 0..3u32.pow(len as u32) {
            let changes = changes_for(code, len);
            for context in 0..=3 {
                let current = format!("{:?}", group_into_hunks(&changes, context));
                let previous = format!("{:?}", previous_group_into_hunks(&changes, context));
                assert_eq!(current, previous, "len {len} code {code} context {context}");
            }
        }
    }
}

#[test]
fn tokenizer_splits_multibyte_punctuation_by_character() {
    let tokens = oxi_tokenize_line("x→y ∧ 12");
    assert_eq!(
        tokens,
        vec![
            OxiDiffToken::Ident("x".to_string()),
            OxiDiffToken::Punct("→".to_string()),
            OxiDiffToken::Ident("y".to_string()),
            OxiDiffToken::Whitespace(" ".to_string()),
            OxiDiffToken::Punct("∧".to_string()),
            OxiDiffToken::Whitespace(" ".to_string()),
            OxiDiffToken::Number("12".to_string()),
        ]
    );
    assert!(oxi_tokenize_line("").is_empty());
}
