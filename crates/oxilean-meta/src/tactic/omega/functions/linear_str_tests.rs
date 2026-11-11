//! `parse_linear_str` on `+` / `-` expressions.

use super::*;

fn tokens(parts: &[&str]) -> Vec<String> {
    parts.iter().map(|s| s.to_string()).collect()
}

fn x() -> LinearExpr {
    LinearExpr::var(Name::str("x"))
}

fn y() -> LinearExpr {
    LinearExpr::var(Name::str("y"))
}

fn z() -> LinearExpr {
    LinearExpr::var(Name::str("z"))
}

#[test]
fn plus_splits_into_a_sum() {
    assert_eq!(
        parse_linear_str(&tokens(&["x", "+", "y"])),
        Some(x().add(&y()))
    );
}

#[test]
fn minus_splits_into_a_difference() {
    assert_eq!(
        parse_linear_str(&tokens(&["x", "-", "y"])),
        Some(x().sub(&y()))
    );
}

#[test]
fn the_last_operator_at_depth_zero_splits_first() {
    assert_eq!(
        parse_linear_str(&tokens(&["x", "-", "y", "+", "z"])),
        Some(x().sub(&y()).add(&z()))
    );
    assert_eq!(
        parse_linear_str(&tokens(&["x", "+", "y", "-", "z"])),
        Some(x().add(&y()).sub(&z()))
    );
}

#[test]
fn operators_inside_parentheses_do_not_split() {
    assert_eq!(
        parse_linear_str(&tokens(&["x", "-", "(", "y", "+", "z", ")"])),
        Some(x().sub(&y().add(&z())))
    );
    assert_eq!(
        parse_linear_str(&tokens(&["(", "x", "+", "y", ")"])),
        Some(x().add(&y()))
    );
}

#[test]
fn constants_and_scaling_combine_with_the_operators() {
    assert_eq!(
        parse_linear_str(&tokens(&["x", "+", "2"])),
        Some(x().add(&LinearExpr::constant(2)))
    );
    assert_eq!(
        parse_linear_str(&tokens(&["x", "-", "2"])),
        Some(x().sub(&LinearExpr::constant(2)))
    );
}

#[test]
fn a_leading_operator_is_not_a_split_point() {
    assert_eq!(parse_linear_str(&tokens(&["-", "x"])), None);
    assert_eq!(parse_linear_str(&tokens(&["+"])), None);
    assert_eq!(parse_linear_str(&tokens(&[])), None);
}
