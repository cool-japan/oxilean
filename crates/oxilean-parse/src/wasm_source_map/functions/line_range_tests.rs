//! `annotation_line_range`.

use super::*;

fn table_with_lines(lines: &[u32]) -> WasmAnnotationTable {
    let mut table = WasmAnnotationTable::new();
    for (i, &line) in lines.iter().enumerate() {
        table.add(WasmAnnotation::new(i as u32, 0, line, 0));
    }
    table
}

#[test]
fn an_empty_table_has_no_range() {
    assert_eq!(annotation_line_range(&table_with_lines(&[])), None);
}

#[test]
fn a_single_annotation_gives_its_own_line_twice() {
    assert_eq!(annotation_line_range(&table_with_lines(&[7])), Some((7, 7)));
}

#[test]
fn the_range_is_the_minimum_and_maximum_line_in_any_order() {
    assert_eq!(
        annotation_line_range(&table_with_lines(&[5, 1, 9, 3])),
        Some((1, 9))
    );
    assert_eq!(
        annotation_line_range(&table_with_lines(&[9, 8, 7])),
        Some((7, 9))
    );
    assert_eq!(
        annotation_line_range(&table_with_lines(&[2, 2, 2])),
        Some((2, 2))
    );
}
