//! `PrologTerm::needs_quoting` / `fmt_atom`.

use super::*;

#[test]
fn empty_atoms_are_quoted() {
    assert!(PrologTerm::needs_quoting(""));
    assert_eq!(PrologTerm::fmt_atom(""), "''");
}

#[test]
fn plain_lowercase_atoms_are_not_quoted() {
    assert!(!PrologTerm::needs_quoting("foo"));
    assert!(!PrologTerm::needs_quoting("foo_bar1"));
    assert!(!PrologTerm::needs_quoting("_hidden"));
    assert_eq!(PrologTerm::fmt_atom("foo"), "foo");
}

#[test]
fn symbolic_atoms_are_not_quoted() {
    assert!(!PrologTerm::needs_quoting("+"));
    assert!(!PrologTerm::needs_quoting("=<"));
    assert!(!PrologTerm::needs_quoting("->"));
}

#[test]
fn uppercase_start_digits_spaces_and_punctuation_are_quoted() {
    assert!(PrologTerm::needs_quoting("Foo"));
    assert!(PrologTerm::needs_quoting("1abc"));
    assert!(PrologTerm::needs_quoting("a b"));
    assert!(PrologTerm::needs_quoting("a-b"));
    assert_eq!(PrologTerm::fmt_atom("Foo"), "'Foo'");
    assert_eq!(PrologTerm::fmt_atom("it's"), "'it\\'s'");
}
