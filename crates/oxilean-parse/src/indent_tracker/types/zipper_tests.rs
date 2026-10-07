//! `IndentZipper::move_up` / `move_down`.

use super::*;

fn zipper() -> IndentZipper {
    match IndentZipper::from_source("a\n  b\n    c\n") {
        Some(z) => z,
        None => panic!("a source with lines has a zipper"),
    }
}

#[test]
fn move_up_at_the_top_does_nothing() {
    let mut z = zipper();
    assert!(!z.move_up());
    assert_eq!(z.position(), 0);
    assert_eq!(z.current_indent(), 0);
}

#[test]
fn move_down_then_up_returns_to_the_same_line() {
    let mut z = zipper();
    assert!(z.move_down());
    assert!(z.move_down());
    assert_eq!(z.position(), 2);
    assert_eq!(z.current_indent(), 4);
    assert_eq!(z.current_content(), "    c");
    assert!(!z.move_down());
    assert!(z.move_up());
    assert_eq!(z.position(), 1);
    assert_eq!(z.current_content(), "  b");
    assert!(z.move_up());
    assert_eq!(z.position(), 0);
    assert_eq!(z.current_content(), "a");
    assert!(!z.move_up());
}
