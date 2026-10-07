//! `merge_modified`: pairing deleted and inserted edits of one name and kind.

use super::*;

fn edit(
    kind: EditKind,
    name: &str,
    body_hash: u64,
    old: Option<usize>,
    new: Option<usize>,
) -> DeclEdit {
    DeclEdit {
        kind,
        old_idx: old,
        new_idx: new,
        fingerprint: DeclFingerprint {
            name: name.to_string(),
            kind: DeclKind::Definition,
            body_hash,
        },
    }
}

type Shape = (EditKind, String, u64, Option<usize>, Option<usize>);

fn shape(edits: &[DeclEdit]) -> Vec<Shape> {
    edits
        .iter()
        .map(|e| {
            (
                e.kind.clone(),
                e.fingerprint.name.clone(),
                e.fingerprint.body_hash,
                e.old_idx,
                e.new_idx,
            )
        })
        .collect()
}

#[test]
fn one_deleted_and_one_inserted_with_different_bodies_become_modified() {
    let raw = vec![
        edit(EditKind::Deleted, "f", 1, Some(0), None),
        edit(EditKind::Inserted, "f", 2, None, Some(0)),
    ];
    assert_eq!(
        shape(&merge_modified(raw)),
        vec![(EditKind::Modified, "f".to_string(), 2, Some(0), Some(0))]
    );
}

#[test]
fn equal_bodies_are_not_merged() {
    let raw = vec![
        edit(EditKind::Deleted, "f", 7, Some(0), None),
        edit(EditKind::Inserted, "f", 7, None, Some(0)),
    ];
    assert_eq!(
        shape(&merge_modified(raw)),
        vec![
            (EditKind::Deleted, "f".to_string(), 7, Some(0), None),
            (EditKind::Inserted, "f".to_string(), 7, None, Some(0)),
        ]
    );
}

#[test]
fn a_deleted_edit_without_an_inserted_partner_stays_deleted() {
    let raw = vec![
        edit(EditKind::Deleted, "f", 1, Some(0), None),
        edit(EditKind::Deleted, "f", 2, Some(1), None),
        edit(EditKind::Inserted, "f", 3, None, Some(0)),
    ];
    assert_eq!(
        shape(&merge_modified(raw)),
        vec![
            (EditKind::Modified, "f".to_string(), 3, Some(0), Some(0)),
            (EditKind::Deleted, "f".to_string(), 2, Some(1), None),
        ]
    );
}

#[test]
fn an_inserted_edit_without_a_deleted_partner_stays_inserted() {
    let raw = vec![
        edit(EditKind::Deleted, "f", 1, Some(0), None),
        edit(EditKind::Inserted, "f", 2, None, Some(0)),
        edit(EditKind::Inserted, "f", 3, None, Some(1)),
    ];
    assert_eq!(
        shape(&merge_modified(raw)),
        vec![
            (EditKind::Modified, "f".to_string(), 2, Some(0), Some(0)),
            (EditKind::Inserted, "f".to_string(), 3, None, Some(1)),
        ]
    );
}

#[test]
fn names_are_paired_independently_and_order_is_kept() {
    let raw = vec![
        edit(EditKind::Unchanged, "keep", 5, Some(0), Some(0)),
        edit(EditKind::Deleted, "g", 1, Some(1), None),
        edit(EditKind::Deleted, "h", 1, Some(2), None),
        edit(EditKind::Inserted, "h", 2, None, Some(1)),
        edit(EditKind::Inserted, "g", 2, None, Some(2)),
    ];
    assert_eq!(
        shape(&merge_modified(raw)),
        vec![
            (EditKind::Unchanged, "keep".to_string(), 5, Some(0), Some(0)),
            (EditKind::Modified, "g".to_string(), 2, Some(1), Some(2)),
            (EditKind::Modified, "h".to_string(), 2, Some(2), Some(1)),
        ]
    );
}

#[test]
fn several_pairs_of_one_name_are_matched_in_order() {
    let raw = vec![
        edit(EditKind::Deleted, "f", 1, Some(0), None),
        edit(EditKind::Deleted, "f", 2, Some(1), None),
        edit(EditKind::Inserted, "f", 3, None, Some(0)),
        edit(EditKind::Inserted, "f", 4, None, Some(1)),
    ];
    assert_eq!(
        shape(&merge_modified(raw)),
        vec![
            (EditKind::Modified, "f".to_string(), 3, Some(0), Some(0)),
            (EditKind::Modified, "f".to_string(), 4, Some(1), Some(1)),
        ]
    );
}
