//! `GlslSwizzleValidator::validate`: length checks, the swizzle set chosen by
//! the first character, and the component limit.

use super::*;

#[test]
fn empty_and_overlong_masks_report_their_length() {
    assert_eq!(
        GlslSwizzleValidator::validate("", 4),
        Err("swizzle mask length 0 is not in [1,4]".to_string())
    );
    assert_eq!(
        GlslSwizzleValidator::validate("xyzwx", 4),
        Err("swizzle mask length 5 is not in [1,4]".to_string())
    );
}

#[test]
fn valid_masks_return_their_length() {
    assert_eq!(GlslSwizzleValidator::validate("x", 1), Ok(1));
    assert_eq!(GlslSwizzleValidator::validate("xyzw", 4), Ok(4));
    assert_eq!(GlslSwizzleValidator::validate("rgb", 3), Ok(3));
    assert_eq!(GlslSwizzleValidator::validate("stpq", 4), Ok(4));
    assert_eq!(GlslSwizzleValidator::validate("yyxx", 2), Ok(4));
}

#[test]
fn the_first_character_selects_the_set_and_the_rest_must_belong_to_it() {
    assert_eq!(
        GlslSwizzleValidator::validate("xr", 4),
        Err("swizzle char 'r' out of range for 4-component vector".to_string())
    );
    assert_eq!(
        GlslSwizzleValidator::validate("rs", 4),
        Err("swizzle char 's' out of range for 4-component vector".to_string())
    );
    assert_eq!(
        GlslSwizzleValidator::validate("xw", 3),
        Err("swizzle char 'w' out of range for 3-component vector".to_string())
    );
}

#[test]
fn an_unknown_first_character_is_reported() {
    assert_eq!(
        GlslSwizzleValidator::validate("k", 4),
        Err("unknown swizzle character 'k'".to_string())
    );
}
