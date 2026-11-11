//! `Parser::parse_calc` on input it rejects.

use super::*;
use crate::lexer::Lexer;
use crate::ParseErrorKind;

fn parse(src: &str) -> Result<Located<SurfaceExpr>, ParseError> {
    let tokens = Lexer::new(src).tokenize();
    Parser::new(tokens).parse_calc()
}

#[test]
fn input_that_does_not_start_with_calc_is_rejected() {
    match parse("foo 1 2") {
        Err(e) => match e.kind {
            ParseErrorKind::UnexpectedToken { expected, .. } => {
                assert_eq!(expected, vec!["calc".to_string()]);
            }
            other => panic!("unexpected error kind {other:?}"),
        },
        Ok(v) => panic!("must not parse, got {:?}", v.value),
    }
}

#[test]
fn a_step_without_a_relation_identifier_is_rejected() {
    match parse("calc 1 := 2") {
        Err(e) => match e.kind {
            ParseErrorKind::UnexpectedToken { expected, .. } => {
                assert_eq!(expected, vec!["identifier".to_string()]);
            }
            other => panic!("unexpected error kind {other:?}"),
        },
        Ok(v) => panic!("must not parse, got {:?}", v.value),
    }
}
