//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{Declaration, Environment, Expr, Level, Name};

use super::types::{
    LevenshteinMetric2, RollingHashExt, StringEncoderExt, StringMonoidExt, SubstringFinder2,
};

#[cfg(test)]
mod str_ext2_tests {
    use super::*;
    #[test]
    fn test_register_string_extended_axioms() {
        let mut env = oxilean_kernel::Environment::new();
        register_string_extended_axioms(&mut env);
        assert!(env.contains(&Name::str("String.Ext.ConcatAssoc")));
        assert!(env.contains(&Name::str("String.Ext.KmpCorrect")));
        assert!(env.contains(&Name::str("String.Ext.EditDistTriangle")));
        assert!(env.contains(&Name::str("String.Ext.SuffixArraySorted")));
        assert!(env.contains(&Name::str("String.Ext.Utf8Roundtrip")));
    }
    #[test]
    fn test_str_concat_assoc() {
        assert!(str_concat_assoc_check("foo", "bar", "baz"));
    }
    #[test]
    fn test_str_left_id() {
        assert!(str_left_id_check("hello"));
        assert!(str_left_id_check(""));
    }
    #[test]
    fn test_str_right_id() {
        assert!(str_right_id_check("world"));
        assert!(str_right_id_check(""));
    }
    #[test]
    fn test_str_length_additive() {
        assert!(str_length_additive("hello", " world"));
        assert!(str_length_additive("", "abc"));
    }
    #[test]
    fn test_str_lex_lt_irrefl() {
        assert!(str_lex_lt_irrefl("apple"));
        assert!(str_lex_lt_irrefl(""));
    }
    #[test]
    fn test_str_lex_lt_trans() {
        assert!(str_lex_lt_trans("apple", "banana", "cherry"));
        assert!(str_lex_lt_trans("z", "a", "m"));
    }
    #[test]
    fn test_str_substring() {
        assert_eq!(str_substring("hello world", 6, 5), "world");
        assert_eq!(str_substring("hello", 0, 3), "hel");
    }
    #[test]
    fn test_str_prefix_refl() {
        assert!(str_prefix_refl("hello"));
        assert!(str_prefix_refl(""));
    }
    #[test]
    fn test_str_prefix_trans() {
        assert!(str_prefix_trans("he", "hel", "hello"));
        assert!(str_prefix_trans("x", "xy", "z"));
    }
    #[test]
    fn test_str_split_join_roundtrip() {
        assert!(str_split_join_roundtrip("a,b,c", ","));
        assert!(str_split_join_roundtrip("hello", ","));
        assert!(str_split_join_roundtrip("", ","));
    }
    #[test]
    fn test_str_trim_idempotent() {
        assert!(str_trim_idempotent("  hello  "));
        assert!(str_trim_idempotent("clean"));
        assert!(str_trim_idempotent(""));
    }
    #[test]
    fn test_str_to_upper_idempotent() {
        assert!(str_to_upper_idempotent("Hello World"));
        assert!(str_to_upper_idempotent("ALREADY UPPER"));
    }
    #[test]
    fn test_str_to_lower_idempotent() {
        assert!(str_to_lower_idempotent("Hello World"));
        assert!(str_to_lower_idempotent("already lower"));
    }
    #[test]
    fn test_str_contains_refl() {
        assert!(str_contains_refl("hello"));
        assert!(str_contains_refl(""));
    }
    #[test]
    fn test_str_find_replace() {
        assert_eq!(
            str_find_replace("hello world", "world", "Rust"),
            "hello Rust"
        );
        assert_eq!(str_find_replace("aaa", "a", "b"), "bbb");
    }
    #[test]
    fn test_str_replace_id() {
        assert!(str_replace_id("hello world", "world"));
        assert!(str_replace_id("test", "xyz"));
    }
    #[test]
    fn test_str_utf8_roundtrip() {
        assert!(str_utf8_roundtrip("hello"));
        assert!(str_utf8_roundtrip(""));
        assert!(str_utf8_roundtrip("Unicode: \u{00E9}"));
    }
    #[test]
    fn test_str_utf16_len() {
        let s = "hello";
        assert_eq!(str_utf16_len(s), 5);
    }
    #[test]
    fn test_str_char_to_string() {
        assert_eq!(str_char_to_string('A'), "A");
    }
    #[test]
    fn test_str_hash_consistent() {
        assert!(str_hash_consistent("hello", "hello"));
        assert!(str_hash_consistent("a", "b"));
    }
    #[test]
    fn test_str_kmp_search() {
        let positions = str_kmp_search("ababab", "ab");
        assert_eq!(positions, vec![0, 2, 4]);
    }
    #[test]
    fn test_str_rabin_karp_search() {
        let positions = str_rabin_karp_search("ababab", "ab");
        assert_eq!(positions, vec![0, 2, 4]);
    }
    #[test]
    fn test_str_kmp_matches_naive() {
        assert!(str_kmp_matches_naive("hello world", "world"));
        assert!(str_kmp_matches_naive("ababab", "ab"));
        assert!(str_kmp_matches_naive("no match", "xyz"));
    }
    #[test]
    fn test_str_rabin_karp_matches_kmp() {
        assert!(str_rabin_karp_matches_kmp("hello world", "world"));
        assert!(str_rabin_karp_matches_kmp("ababab", "ab"));
    }
    #[test]
    fn test_str_aho_corasick_search() {
        let results = str_aho_corasick_search("hello world", &["hello", "world"]);
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_str_suffix_array() {
        let sa = str_suffix_array("banana");
        assert_eq!(sa.len(), 6);
    }
    #[test]
    fn test_str_suffix_array_sorted() {
        assert!(str_suffix_array_sorted("banana"));
        assert!(str_suffix_array_sorted("abcdef"));
        assert!(str_suffix_array_sorted(""));
    }
    #[test]
    fn test_str_lcp_array() {
        let lcp = str_lcp_array("banana");
        assert_eq!(lcp.len(), 6);
    }
    #[test]
    fn test_str_levenshtein() {
        assert_eq!(str_levenshtein("kitten", "sitting"), 3);
        assert_eq!(str_levenshtein("", "abc"), 3);
        assert_eq!(str_levenshtein("abc", "abc"), 0);
    }
    #[test]
    fn test_str_edit_dist_zero() {
        assert!(str_edit_dist_zero("hello"));
        assert!(str_edit_dist_zero(""));
    }
    #[test]
    fn test_str_edit_dist_sym() {
        assert!(str_edit_dist_sym("kitten", "sitting"));
        assert!(str_edit_dist_sym("abc", "xyz"));
    }
    #[test]
    fn test_str_edit_dist_triangle() {
        assert!(str_edit_dist_triangle("abc", "abd", "xyz"));
        assert!(str_edit_dist_triangle("", "a", "ab"));
    }
    #[test]
    fn test_str_unicode_valid() {
        assert!(str_unicode_valid("hello"));
        assert!(str_unicode_valid("\u{00E9}"));
    }
    #[test]
    fn test_str_count_alnum() {
        assert_eq!(str_count_alnum("hello world 123"), 13);
        assert_eq!(str_count_alnum("!@#$"), 0);
    }
    #[test]
    fn test_str_dec_eq_refl() {
        assert!(str_dec_eq_refl("hello"));
        assert!(str_dec_eq_refl(""));
    }
    #[test]
    fn test_str_dec_eq_sym() {
        assert!(str_dec_eq_sym("abc", "abc"));
        assert!(str_dec_eq_sym("abc", "xyz"));
    }
    #[test]
    fn test_string_monoid_ext() {
        let mut m = StringMonoidExt::new();
        assert!(m.is_identity());
        m.mappend("hello");
        assert!(!m.is_identity());
        m.mappend(" world");
        assert_eq!(m.buffer, "hello world");
    }
    #[test]
    fn test_substring_finder2() {
        let finder = SubstringFinder2::new("ababab", "ab");
        let naive = finder.find_all();
        let kmp = finder.find_kmp();
        assert_eq!(naive, kmp);
        assert_eq!(naive, vec![0, 2, 4]);
    }
    #[test]
    fn test_string_encoder_ext() {
        let enc = StringEncoderExt::utf8();
        assert!(enc.roundtrip("hello"));
        assert!(enc.roundtrip(""));
        let bytes = enc.encode("abc");
        assert_eq!(bytes, b"abc");
    }
    #[test]
    fn test_levenshtein_metric2() {
        let m = LevenshteinMetric2::new();
        assert!(m.identity_law("hello"));
        assert!(m.symmetry_law("kitten", "sitting"));
        assert_eq!(m.distance("abc", "abc"), 0);
        let m2 = LevenshteinMetric2::with_max(2);
        assert!(m2.within_threshold("abc", "abd"));
        assert!(!m2.within_threshold("abc", "xyz"));
    }
    #[test]
    fn test_rolling_hash_ext() {
        let rh = RollingHashExt::new();
        let positions = rh.find("ababab", "ab");
        assert_eq!(positions, vec![0, 2, 4]);
        assert_eq!(rh.hash_str("hello"), rh.hash_str("hello"));
    }
}
