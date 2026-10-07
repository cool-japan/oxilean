//! `option_get_or_insert` on both of its cases.

use super::option_get_or_insert;

#[test]
fn returns_the_stored_value_or_stores_the_given_one() {
    let mut empty: Option<String> = None;
    assert_eq!(option_get_or_insert(&mut empty, "new".to_string()), "new");
    assert_eq!(empty.as_deref(), Some("new"));
    let mut full = Some("old".to_string());
    assert_eq!(option_get_or_insert(&mut full, "new".to_string()), "old");
    assert_eq!(full.as_deref(), Some("old"));
}
