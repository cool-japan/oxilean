//! Tests for the product-type case of `translate_type_to_agda`.

use super::translate_type_to_agda;

#[test]
fn translates_both_spellings_of_a_product() {
    assert_eq!(translate_type_to_agda("Nat × Bool"), "ℕ × Bool");
    assert_eq!(translate_type_to_agda("Nat * Int"), "ℕ × ℤ");
    assert_eq!(translate_type_to_agda("Nat × Bool * Int"), "ℕ × Bool × ℤ");
}
