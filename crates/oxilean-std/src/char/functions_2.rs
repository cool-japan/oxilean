//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::functions::*;
use super::types::*;

#[cfg(test)]
mod ext_axiom_tests {
    use super::*;
    fn base_env() -> Environment {
        let mut env = Environment::new();
        let ty1 = Expr::Sort(Level::succ(Level::zero()));
        for name in &[
            "Nat",
            "Bool",
            "String",
            "Char",
            "UInt32",
            "List",
            "Option",
            "Prod",
            "Decidable",
            "Eq",
            "Not",
            "And",
            "Or",
            "Nat.lt",
            "Nat.le",
            "Bool.true",
            "Bool.false",
            "Char.toNat",
            "Char.ofNat",
            "Char.isAscii",
            "Char.toUInt32",
            "Char.utf8Width",
            "Char.caseFold",
            "Char.lt",
            "Char.blt",
            "Char.UnicodeCategory",
            "Char.BidiCategory",
            "Char.CollationKey",
            "Char.RegexClass",
            "Char.TerminalAlphabet",
            "Char.digitToNat",
            "Char.isPrefix",
            "Char.natToDigit",
            "Char.composeWith",
            "Char.unicodeMax",
        ] {
            let _ = env.add(Declaration::Axiom {
                name: Name::str(*name),
                univ_params: vec![],
                ty: ty1.clone(),
            });
        }
        env
    }
    #[test]
    fn test_register_char_extended_axioms_runs() {
        let mut env = base_env();
        register_char_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Char.isValidScalar")).is_some());
        assert!(env.get(&Name::str("Char.unicodeMax")).is_some());
        assert!(env.get(&Name::str("Char.decEq")).is_some());
        assert!(env.get(&Name::str("Char.not_surrogate")).is_some());
        assert!(env.get(&Name::str("Char.succ")).is_some());
        assert!(env.get(&Name::str("Char.caseFold")).is_some());
        assert!(env.get(&Name::str("Char.nfcNormalize")).is_some());
        assert!(env.get(&Name::str("Char.matchesClass")).is_some());
        assert!(env.get(&Name::str("Char.isTerminal")).is_some());
        assert!(env.get(&Name::str("Char.composeWith")).is_some());
    }
    #[test]
    fn test_char_is_valid_scalar_ty_shape() {
        assert!(matches!(char_is_valid_scalar_ty(), Expr::Pi(..)));
    }
    #[test]
    fn test_char_iso_nat_ty_shape() {
        assert!(matches!(char_iso_nat_ty(), Expr::Pi(..)));
    }
    #[test]
    fn test_char_dec_eq_ty_shape() {
        assert!(matches!(char_dec_eq_ty(), Expr::Pi(..)));
    }
    #[test]
    fn test_char_lt_total_ty_shape() {
        assert!(matches!(char_lt_total_ty(), Expr::Pi(..)));
    }
    #[test]
    fn test_char_not_surrogate_ty_shape() {
        assert!(matches!(char_not_surrogate_ty(), Expr::Pi(..)));
    }
    #[test]
    fn test_char_utf8_width_range_ty_shape() {
        assert!(matches!(char_utf8_width_range_ty(), Expr::Pi(..)));
    }
    #[test]
    fn test_char_case_fold_idempotent_ty_shape() {
        assert!(matches!(char_case_fold_idempotent_ty(), Expr::Pi(..)));
    }
    #[test]
    fn test_unicode_char_ascii() {
        let uc = UnicodeChar::new('A');
        assert_eq!(uc.code_point, 65);
        assert!(uc.is_ascii);
        assert!(!uc.is_combining);
        assert!(!uc.is_surrogate);
        assert_eq!(uc.utf8_width, 1);
        assert_eq!(uc.block_name(), "Basic Latin");
    }
    #[test]
    fn test_unicode_char_combining() {
        let uc = UnicodeChar::new('\u{0301}');
        assert!(uc.is_combining);
        assert_eq!(uc.block_name(), "Combining Diacritical Marks");
    }
    #[test]
    fn test_unicode_char_multibyte() {
        let uc = UnicodeChar::new('€');
        assert_eq!(uc.utf8_width, 3);
        assert!(!uc.is_ascii);
    }
    #[test]
    fn test_unicode_char_to_expr() {
        let uc = UnicodeChar::new('A');
        assert!(matches!(uc.to_expr(), Expr::App(..)));
    }
    #[test]
    fn test_unicode_char_is_caseless() {
        assert!(UnicodeChar::new('5').is_caseless());
        assert!(!UnicodeChar::new('A').is_caseless());
    }
    #[test]
    fn test_char_classifier_classify_letter() {
        let cl = CharClassifier::standard();
        let classes = cl.classify('A');
        assert!(classes.contains(&"letter"));
        assert!(classes.contains(&"uppercase"));
        assert!(classes.contains(&"ascii"));
    }
    #[test]
    fn test_char_classifier_belongs_to() {
        let cl = CharClassifier::standard();
        assert!(cl.belongs_to('5', "digit"));
        assert!(!cl.belongs_to('A', "digit"));
    }
    #[test]
    fn test_char_classifier_class_names() {
        let cl = CharClassifier::standard();
        let names = cl.class_names();
        assert!(names.contains(&"letter"));
        assert!(names.contains(&"emoji"));
    }
    #[test]
    fn test_char_classifier_emoji() {
        let cl = CharClassifier::standard();
        assert!(cl.belongs_to('\u{1F600}', "emoji"));
    }
    #[test]
    fn test_char_encoder_utf8_ascii() {
        let enc = CharEncoder::new(CharEncoding::Utf8);
        assert_eq!(enc.encode('A'), b"A");
    }
    #[test]
    fn test_char_encoder_utf8_multibyte() {
        let enc = CharEncoder::new(CharEncoding::Utf8);
        assert_eq!(enc.encode('€').len(), 3);
    }
    #[test]
    fn test_char_encoder_utf32le() {
        let enc = CharEncoder::new(CharEncoding::Utf32Le);
        assert_eq!(enc.encode('A'), vec![65, 0, 0, 0]);
    }
    #[test]
    fn test_char_encoder_decode_first_utf8() {
        let enc = CharEncoder::new(CharEncoding::Utf8);
        assert_eq!(enc.decode_first(b"Hello"), Some(('H', 1)));
    }
    #[test]
    fn test_char_encoder_encode_str() {
        let enc = CharEncoder::new(CharEncoding::Utf8);
        assert_eq!(enc.encode_str("Hi"), b"Hi");
    }
    #[test]
    fn test_grapheme_cluster_singleton() {
        let g = GraphemeCluster::singleton('A');
        assert!(g.is_singleton());
        assert!(!g.has_combining());
        assert_eq!(g.base(), Some('A'));
        assert_eq!(g.to_string_repr(), "A");
    }
    #[test]
    fn test_grapheme_cluster_with_combining() {
        let g = GraphemeCluster::with_combining('a', ['\u{0301}']);
        assert!(!g.is_singleton());
        assert!(g.has_combining());
    }
    #[test]
    fn test_grapheme_cluster_try_compose() {
        let g = GraphemeCluster::with_combining('a', ['\u{0301}']);
        assert_eq!(g.try_compose(), Some('á'));
    }
    #[test]
    fn test_grapheme_cluster_utf8_byte_len() {
        assert_eq!(GraphemeCluster::singleton('€').utf8_byte_len(), 3);
    }
    #[test]
    fn test_char_normalizer_nfc_composes() {
        let norm = CharNormalizer::new(NormalizationForm::Nfc);
        assert_eq!(norm.normalize("a\u{0301}"), "á");
    }
    #[test]
    fn test_char_normalizer_none_passthrough() {
        let norm = CharNormalizer::new(NormalizationForm::None);
        assert_eq!(norm.normalize("hello"), "hello");
    }
    #[test]
    fn test_char_normalizer_strip_controls() {
        let norm = CharNormalizer::new(NormalizationForm::None).with_strip_controls();
        assert_eq!(norm.normalize("hello\x00world"), "helloworld");
    }
    #[test]
    fn test_char_normalizer_whitespace() {
        let norm = CharNormalizer::new(NormalizationForm::None).with_normalize_whitespace();
        assert!(!norm.normalize("hello\tworld").contains('\t'));
    }
    #[test]
    fn test_char_normalizer_description() {
        assert!(CharNormalizer::new(NormalizationForm::Nfc)
            .description()
            .contains("NFC"));
    }
    #[test]
    fn test_char_normalizer_normalize_char() {
        let norm = CharNormalizer::new(NormalizationForm::Nfc);
        assert_eq!(norm.normalize_char('A'), vec!['A']);
    }
}
