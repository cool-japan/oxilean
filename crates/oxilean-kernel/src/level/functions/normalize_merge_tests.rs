//! `normalize` where the `max` arguments merge to one argument and to
//! several, and over every level of a small grammar: idempotent and
//! equivalent to its input.

use super::*;
use crate::{Level, LevelView, Name};

fn param(name: &str) -> Level {
    Level::param(Name::str(name))
}

#[test]
fn merged_arguments_rebuild_one_level_or_a_right_nested_max() {
    let (u, v) = (param("u"), param("v"));
    let one = Level::succ(Level::zero());
    // One argument left after merging.
    assert_eq!(normalize(&Level::max(u.clone(), u.clone())), u);
    assert_eq!(normalize(&Level::max(Level::zero(), u.clone())), u);
    assert_eq!(
        normalize(&Level::max(one.clone(), Level::succ(u.clone()))),
        Level::succ(u.clone())
    );
    assert_eq!(normalize(&Level::max(one.clone(), one.clone())), one);
    // Several: a `max` whose arguments are the merged ones, nested to the right.
    let two = normalize(&Level::max(v.clone(), u.clone()));
    assert!(matches!(two.view(), LevelView::Max(_, _)), "{two:?}");
    assert_eq!(normalize(&Level::max(u.clone(), v.clone())), two);
    let three = normalize(&Level::max(Level::max(u.clone(), param("w")), v.clone()));
    assert!(
        matches!(three.view(), LevelView::Max(_, rest) if matches!(rest.view(), LevelView::Max(_, _))),
        "{three:?}"
    );
}

#[test]
fn normal_forms_are_idempotent_and_equivalent_over_a_small_grammar() {
    let atoms = [
        Level::zero(),
        Level::succ(Level::zero()),
        param("u"),
        param("v"),
        Level::succ(param("u")),
    ];
    let mut levels: Vec<Level> = atoms.to_vec();
    for x in &atoms {
        levels.push(Level::succ(x.clone()));
        for y in &atoms {
            levels.push(Level::max(x.clone(), y.clone()));
            levels.push(Level::imax(x.clone(), y.clone()));
            for z in &atoms {
                levels.push(Level::max(Level::max(x.clone(), y.clone()), z.clone()));
                levels.push(Level::max(x.clone(), Level::imax(y.clone(), z.clone())));
            }
        }
    }
    for l in &levels {
        let n = normalize(l);
        assert_eq!(normalize(&n), n, "not idempotent on {l:?}");
        assert!(is_equivalent(l, &n), "{l:?} normalised to {n:?}");
    }
}
