//! `format_expected` for every length of list.

use super::*;

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| s.to_string()).collect()
}

#[test]
fn formats_by_length() {
    assert_eq!(format_expected(&strings(&[])), "something");
    assert_eq!(format_expected(&strings(&["a"])), "a");
    assert_eq!(format_expected(&strings(&["a", "b"])), "a or b");
    assert_eq!(format_expected(&strings(&["a", "b", "c"])), "a, b, or c");
    assert_eq!(
        format_expected(&strings(&["a", "b", "c", "d"])),
        "a, b, c, or d"
    );
}
