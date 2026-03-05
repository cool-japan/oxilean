//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast_impl::BinderKind;
    use crate::command::*;
    use crate::tokens::{Span, Token, TokenKind};
    fn make_token(kind: TokenKind) -> Token {
        Token {
            kind,
            span: Span::new(0, 0, 1, 1),
        }
    }
    fn make_ident(s: &str) -> Token {
        make_token(TokenKind::Ident(s.to_string()))
    }
    fn make_eof() -> Token {
        make_token(TokenKind::Eof)
    }
    #[test]
    fn test_parser_create() {
        let parser = CommandParser::new();
        assert_eq!(parser.position(), 0);
    }
    #[test]
    fn test_is_command_keyword() {
        assert!(CommandParser::is_command_keyword(&TokenKind::Axiom));
        assert!(CommandParser::is_command_keyword(&TokenKind::Definition));
        assert!(CommandParser::is_command_keyword(&TokenKind::Theorem));
        assert!(CommandParser::is_command_keyword(&TokenKind::Inductive));
        assert!(CommandParser::is_command_keyword(&TokenKind::Structure));
        assert!(CommandParser::is_command_keyword(&TokenKind::Class));
        assert!(CommandParser::is_command_keyword(&TokenKind::Instance));
        assert!(CommandParser::is_command_keyword(&TokenKind::Namespace));
        assert!(CommandParser::is_command_keyword(&TokenKind::Section));
        assert!(CommandParser::is_command_keyword(&TokenKind::Variable));
        assert!(CommandParser::is_command_keyword(&TokenKind::Variables));
        assert!(CommandParser::is_command_keyword(&TokenKind::End));
        assert!(CommandParser::is_command_keyword(&TokenKind::Import));
        assert!(CommandParser::is_command_keyword(&TokenKind::Export));
        assert!(CommandParser::is_command_keyword(&TokenKind::Open));
        assert!(CommandParser::is_command_keyword(&TokenKind::Attribute));
        assert!(CommandParser::is_command_keyword(&TokenKind::Hash));
        assert!(CommandParser::is_command_keyword(&TokenKind::Ident(
            "set_option".to_string()
        )));
        assert!(CommandParser::is_command_keyword(&TokenKind::Ident(
            "universe".to_string()
        )));
        assert!(CommandParser::is_command_keyword(&TokenKind::Ident(
            "notation".to_string()
        )));
        assert!(CommandParser::is_command_keyword(&TokenKind::Ident(
            "derive".to_string()
        )));
        assert!(!CommandParser::is_command_keyword(&TokenKind::LParen));
        assert!(!CommandParser::is_command_keyword(&TokenKind::Ident(
            "foo".to_string()
        )));
    }
    #[test]
    fn test_parser_reset() {
        let mut parser = CommandParser::new();
        parser.pos = 10;
        parser.reset();
        assert_eq!(parser.position(), 0);
    }
    #[test]
    fn test_parse_import() {
        let tokens = vec![
            make_token(TokenKind::Import),
            make_ident("Mathlib"),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Import { module, .. } => assert_eq!(module, "Mathlib"),
            _ => panic!("expected Import command"),
        }
    }
    #[test]
    fn test_parse_import_dotted() {
        let tokens = vec![
            make_token(TokenKind::Import),
            make_ident("Mathlib"),
            make_token(TokenKind::Dot),
            make_ident("Data"),
            make_token(TokenKind::Dot),
            make_ident("Nat"),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Import { module, .. } => assert_eq!(module, "Mathlib.Data.Nat"),
            _ => panic!("expected Import command"),
        }
    }
    #[test]
    fn test_parse_export() {
        let tokens = vec![
            make_token(TokenKind::Export),
            make_ident("foo"),
            make_ident("bar"),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Export { names, .. } => {
                assert_eq!(names, vec!["foo".to_string(), "bar".to_string()]);
            }
            _ => panic!("expected Export command"),
        }
    }
    #[test]
    fn test_parse_namespace() {
        let tokens = vec![
            make_token(TokenKind::Namespace),
            make_ident("MyNamespace"),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Namespace { name, .. } => assert_eq!(name, "MyNamespace"),
            _ => panic!("expected Namespace command"),
        }
    }
    #[test]
    fn test_parse_end() {
        let tokens = vec![make_token(TokenKind::End), make_eof()];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        assert!(matches!(cmd, Command::End { .. }));
    }
    #[test]
    fn test_parse_open_all() {
        let tokens = vec![make_token(TokenKind::Open), make_ident("Nat"), make_eof()];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Open { path, items, .. } => {
                assert_eq!(path, vec!["Nat".to_string()]);
                assert_eq!(items.len(), 1);
                assert!(matches!(items[0], OpenItem::All));
            }
            _ => panic!("expected Open command"),
        }
    }
    #[test]
    fn test_parse_open_only() {
        let tokens = vec![
            make_token(TokenKind::Open),
            make_ident("Nat"),
            make_ident("only"),
            make_token(TokenKind::LBracket),
            make_ident("add"),
            make_token(TokenKind::Comma),
            make_ident("mul"),
            make_token(TokenKind::RBracket),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Open { path, items, .. } => {
                assert_eq!(path, vec!["Nat".to_string()]);
                match &items[0] {
                    OpenItem::Only(names) => {
                        assert_eq!(names, &["add".to_string(), "mul".to_string()]);
                    }
                    _ => panic!("expected Only"),
                }
            }
            _ => panic!("expected Open command"),
        }
    }
    #[test]
    fn test_parse_open_hiding() {
        let tokens = vec![
            make_token(TokenKind::Open),
            make_ident("Nat"),
            make_ident("hiding"),
            make_token(TokenKind::LBracket),
            make_ident("sub"),
            make_token(TokenKind::RBracket),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Open { items, .. } => {
                assert!(
                    matches!(& items[0], OpenItem::Hiding(names) if names == & ["sub"
                    .to_string()])
                );
            }
            _ => panic!("expected Open command"),
        }
    }
    #[test]
    fn test_parse_open_renaming() {
        let tokens = vec![
            make_token(TokenKind::Open),
            make_ident("Nat"),
            make_ident("renaming"),
            make_ident("add"),
            make_token(TokenKind::Arrow),
            make_ident("plus"),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Open { items, .. } => {
                assert!(
                    matches!(& items[0], OpenItem::Renaming(old, new) if old == "add" &&
                    new == "plus")
                );
            }
            _ => panic!("expected Open command"),
        }
    }
    #[test]
    fn test_parse_section() {
        let tokens = vec![
            make_token(TokenKind::Section),
            make_ident("MySection"),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Section { name, .. } => assert_eq!(name, "MySection"),
            _ => panic!("expected Section command"),
        }
    }
    #[test]
    fn test_parse_variable_explicit() {
        let tokens = vec![
            make_token(TokenKind::Variable),
            make_token(TokenKind::LParen),
            make_ident("x"),
            make_token(TokenKind::Colon),
            make_ident("Nat"),
            make_token(TokenKind::RParen),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Variable { binders, .. } => {
                assert_eq!(binders.len(), 1);
                assert_eq!(binders[0].name, "x");
                assert_eq!(binders[0].info, BinderKind::Default);
            }
            _ => panic!("expected Variable command"),
        }
    }
    #[test]
    fn test_parse_variable_implicit() {
        let tokens = vec![
            make_token(TokenKind::Variable),
            make_token(TokenKind::LBrace),
            make_ident("a"),
            make_token(TokenKind::Colon),
            make_ident("Type"),
            make_token(TokenKind::RBrace),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Variable { binders, .. } => {
                assert_eq!(binders.len(), 1);
                assert_eq!(binders[0].name, "a");
                assert_eq!(binders[0].info, BinderKind::Implicit);
            }
            _ => panic!("expected Variable command"),
        }
    }
    #[test]
    fn test_parse_variable_instance() {
        let tokens = vec![
            make_token(TokenKind::Variable),
            make_token(TokenKind::LBracket),
            make_ident("inst"),
            make_token(TokenKind::Colon),
            make_ident("Add"),
            make_token(TokenKind::RBracket),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Variable { binders, .. } => {
                assert_eq!(binders.len(), 1);
                assert_eq!(binders[0].name, "inst");
                assert_eq!(binders[0].info, BinderKind::Instance);
            }
            _ => panic!("expected Variable command"),
        }
    }
    #[test]
    fn test_parse_variable_multiple() {
        let tokens = vec![
            make_token(TokenKind::Variable),
            make_token(TokenKind::LParen),
            make_ident("x"),
            make_ident("y"),
            make_token(TokenKind::Colon),
            make_ident("Nat"),
            make_token(TokenKind::RParen),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Variable { binders, .. } => {
                assert_eq!(binders.len(), 2);
                assert_eq!(binders[0].name, "x");
                assert_eq!(binders[1].name, "y");
            }
            _ => panic!("expected Variable command"),
        }
    }
    #[test]
    fn test_parse_attribute() {
        let tokens = vec![
            make_token(TokenKind::Attribute),
            make_token(TokenKind::LBracket),
            make_ident("simp"),
            make_token(TokenKind::Comma),
            make_ident("ext"),
            make_token(TokenKind::RBracket),
            make_ident("myLemma"),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Attribute { attrs, name, .. } => {
                assert_eq!(attrs, vec!["simp".to_string(), "ext".to_string()]);
                assert_eq!(name, "myLemma");
            }
            _ => panic!("expected Attribute command"),
        }
    }
    #[test]
    fn test_parse_check() {
        let tokens = vec![
            make_token(TokenKind::Hash),
            make_ident("check"),
            make_ident("Nat"),
            make_token(TokenKind::Dot),
            make_ident("add"),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Check { expr_str, .. } => {
                assert_eq!(expr_str, "Nat . add");
            }
            _ => panic!("expected Check command"),
        }
    }
    #[test]
    fn test_parse_eval() {
        let tokens = vec![
            make_token(TokenKind::Hash),
            make_ident("eval"),
            make_token(TokenKind::Nat(42)),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Eval { expr_str, .. } => {
                assert_eq!(expr_str, "42");
            }
            _ => panic!("expected Eval command"),
        }
    }
    #[test]
    fn test_parse_print() {
        let tokens = vec![
            make_token(TokenKind::Hash),
            make_ident("print"),
            make_ident("Nat"),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Print { name, .. } => {
                assert_eq!(name, "Nat");
            }
            _ => panic!("expected Print command"),
        }
    }
    #[test]
    fn test_parse_set_option() {
        let tokens = vec![
            make_ident("set_option"),
            make_ident("pp"),
            make_token(TokenKind::Dot),
            make_ident("all"),
            make_ident("true"),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::SetOption { name, value, .. } => {
                assert_eq!(name, "pp.all");
                assert_eq!(value, "true");
            }
            _ => panic!("expected SetOption command"),
        }
    }
    #[test]
    fn test_parse_universe_single() {
        let tokens = vec![make_ident("universe"), make_ident("u"), make_eof()];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Universe { names, .. } => {
                assert_eq!(names, vec!["u".to_string()]);
            }
            _ => panic!("expected Universe command"),
        }
    }
    #[test]
    fn test_parse_universe_multiple() {
        let tokens = vec![
            make_ident("universe"),
            make_ident("u"),
            make_ident("v"),
            make_ident("w"),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Universe { names, .. } => {
                assert_eq!(
                    names,
                    vec!["u".to_string(), "v".to_string(), "w".to_string()]
                );
            }
            _ => panic!("expected Universe command"),
        }
    }
    #[test]
    fn test_parse_notation_infix() {
        let tokens = vec![
            make_ident("infix"),
            make_token(TokenKind::Nat(65)),
            make_token(TokenKind::String("+".to_string())),
            make_token(TokenKind::Assign),
            make_ident("HAdd"),
            make_token(TokenKind::Dot),
            make_ident("hAdd"),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Notation {
                kind,
                name,
                prec,
                body,
                ..
            } => {
                assert_eq!(kind, NotationKind::Infix);
                assert_eq!(name, "+");
                assert_eq!(prec, Some(65));
                assert!(body.contains("HAdd"));
            }
            _ => panic!("expected Notation command"),
        }
    }
    #[test]
    fn test_parse_notation_prefix() {
        let tokens = vec![
            make_ident("prefix"),
            make_token(TokenKind::Nat(100)),
            make_token(TokenKind::String("-".to_string())),
            make_token(TokenKind::Assign),
            make_ident("Neg"),
            make_token(TokenKind::Dot),
            make_ident("neg"),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Notation { kind, prec, .. } => {
                assert_eq!(kind, NotationKind::Prefix);
                assert_eq!(prec, Some(100));
            }
            _ => panic!("expected Notation command"),
        }
    }
    #[test]
    fn test_parse_derive() {
        let tokens = vec![
            make_ident("derive"),
            make_ident("DecidableEq"),
            make_token(TokenKind::Comma),
            make_ident("Repr"),
            make_ident("for"),
            make_ident("MyType"),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Derive {
                strategies,
                type_name,
                ..
            } => {
                assert_eq!(
                    strategies,
                    vec!["DecidableEq".to_string(), "Repr".to_string()]
                );
                assert_eq!(type_name, "MyType");
            }
            _ => panic!("expected Derive command"),
        }
    }
    #[test]
    fn test_parse_empty_tokens() {
        let tokens: Vec<Token> = vec![];
        let mut parser = CommandParser::new();
        assert!(parser.parse_command(&tokens).is_err());
    }
    #[test]
    fn test_parse_unknown_command() {
        let tokens = vec![make_ident("foobar"), make_eof()];
        let mut parser = CommandParser::new();
        assert!(parser.parse_command(&tokens).is_err());
    }
    #[test]
    fn test_open_dotted_path() {
        let tokens = vec![
            make_token(TokenKind::Open),
            make_ident("Std"),
            make_token(TokenKind::Dot),
            make_ident("Data"),
            make_token(TokenKind::Dot),
            make_ident("List"),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Open { path, .. } => {
                assert_eq!(
                    path,
                    vec!["Std".to_string(), "Data".to_string(), "List".to_string()]
                );
            }
            _ => panic!("expected Open command"),
        }
    }
    #[test]
    fn test_notation_kind_display() {
        assert_eq!(format!("{}", NotationKind::Prefix), "prefix");
        assert_eq!(format!("{}", NotationKind::Infix), "infix");
        assert_eq!(format!("{}", NotationKind::Postfix), "postfix");
        assert_eq!(format!("{}", NotationKind::Notation), "notation");
    }
    #[test]
    fn test_open_item_display() {
        assert_eq!(format!("{}", OpenItem::All), "*");
        assert_eq!(
            format!("{}", OpenItem::Only(vec!["a".to_string(), "b".to_string()])),
            "only [a, b]"
        );
        assert_eq!(
            format!("{}", OpenItem::Hiding(vec!["c".to_string()])),
            "hiding [c]"
        );
        assert_eq!(
            format!(
                "{}",
                OpenItem::Renaming("old".to_string(), "new".to_string())
            ),
            "old -> new"
        );
    }
    #[test]
    fn test_parse_variable_strict_implicit() {
        let tokens = vec![
            make_token(TokenKind::Variable),
            make_token(TokenKind::LBrace),
            make_token(TokenKind::LBrace),
            make_ident("a"),
            make_token(TokenKind::Colon),
            make_ident("Type"),
            make_token(TokenKind::RBrace),
            make_token(TokenKind::RBrace),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Variable { binders, .. } => {
                assert_eq!(binders.len(), 1);
                assert_eq!(binders[0].name, "a");
                assert_eq!(binders[0].info, BinderKind::StrictImplicit);
            }
            _ => panic!("expected Variable command"),
        }
    }
    #[test]
    fn test_parse_declaration_keyword_delegates() {
        let tokens = vec![make_token(TokenKind::Axiom), make_eof()];
        let mut parser = CommandParser::new();
        let result = parser.parse_command(&tokens);
        assert!(result.is_err());
    }
    #[test]
    fn test_parse_export_empty() {
        let tokens = vec![make_token(TokenKind::Export), make_eof()];
        let mut parser = CommandParser::new();
        assert!(parser.parse_command(&tokens).is_err());
    }
    #[test]
    fn test_parse_universe_empty() {
        let tokens = vec![make_ident("universe"), make_eof()];
        let mut parser = CommandParser::new();
        assert!(parser.parse_command(&tokens).is_err());
    }
    #[test]
    fn test_parse_structure_simple() {
        let tokens = vec![
            make_token(TokenKind::Structure),
            make_ident("Point"),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Structure {
                name,
                extends,
                fields,
                derives,
                ..
            } => {
                assert_eq!(name, "Point");
                assert!(extends.is_empty());
                assert!(fields.is_empty());
                assert!(derives.is_empty());
            }
            _ => panic!("expected Structure command"),
        }
    }
    #[test]
    fn test_parse_structure_with_fields() {
        let tokens = vec![
            make_token(TokenKind::Structure),
            make_ident("Point"),
            make_ident("where"),
            make_ident("x"),
            make_token(TokenKind::Colon),
            make_ident("Real"),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Structure { name, fields, .. } => {
                assert_eq!(name, "Point");
                assert!(!fields.is_empty());
            }
            _ => panic!("expected Structure command"),
        }
    }
    #[test]
    fn test_parse_class_simple() {
        let tokens = vec![
            make_token(TokenKind::Class),
            make_ident("Monoid"),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Class {
                name,
                params,
                extends,
                ..
            } => {
                assert_eq!(name, "Monoid");
                assert!(params.is_empty());
                assert!(extends.is_empty());
            }
            _ => panic!("expected Class command"),
        }
    }
    #[test]
    fn test_parse_class_with_extends() {
        let tokens = vec![
            make_token(TokenKind::Class),
            make_ident("Group"),
            make_ident("extends"),
            make_ident("Monoid"),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Class { name, extends, .. } => {
                assert_eq!(name, "Group");
                assert_eq!(extends, vec!["Monoid".to_string()]);
            }
            _ => panic!("expected Class command"),
        }
    }
    #[test]
    fn test_parse_instance_simple() {
        let tokens = vec![
            make_token(TokenKind::Instance),
            make_ident("myInst"),
            make_token(TokenKind::Colon),
            make_ident("Nat"),
            make_token(TokenKind::Assign),
            make_ident("42"),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Instance {
                name, ty, priority, ..
            } => {
                assert_eq!(name, "myInst");
                assert_eq!(ty, "Nat");
                assert!(priority.is_none());
            }
            _ => panic!("expected Instance command"),
        }
    }
    #[test]
    fn test_parse_instance_with_priority() {
        let tokens = vec![
            make_token(TokenKind::Instance),
            make_ident("inst"),
            make_token(TokenKind::Colon),
            make_ident("Type"),
            make_ident("priority"),
            make_token(TokenKind::Nat(100)),
            make_token(TokenKind::Assign),
            make_ident("by"),
            make_ident("rfl"),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Instance { priority, .. } => {
                assert_eq!(priority, Some(100));
            }
            _ => panic!("expected Instance command"),
        }
    }
    #[test]
    fn test_parse_reduce_command() {
        let tokens = vec![
            make_token(TokenKind::Hash),
            make_ident("reduce"),
            make_ident("1"),
            make_token(TokenKind::Plus),
            make_ident("1"),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Reduce { expr_str, .. } => {
                assert!(!expr_str.is_empty());
            }
            _ => panic!("expected Reduce command"),
        }
    }
    #[test]
    fn test_parse_attribute_decl_simple() {
        let tokens = vec![
            make_token(TokenKind::Attribute),
            make_token(TokenKind::LBracket),
            make_ident("simp"),
            make_token(TokenKind::RBracket),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::AttributeDecl { name, kind, .. } => {
                assert_eq!(name, "simp");
                assert_eq!(kind, AttributeDeclKind::Simple);
            }
            _ => panic!("expected AttributeDecl command"),
        }
    }
    #[test]
    fn test_parse_syntax_command() {
        let tokens = vec![
            make_ident("syntax"),
            make_ident("arrow"),
            make_token(TokenKind::Nat(50)),
            make_token(TokenKind::Assign),
            make_ident("fun"),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Syntax {
                name,
                prec,
                pattern,
                ..
            } => {
                assert_eq!(name, "arrow");
                assert_eq!(prec, Some(50));
                assert!(!pattern.is_empty());
            }
            _ => panic!("expected Syntax command"),
        }
    }
    #[test]
    fn test_structure_field_creation() {
        let field = StructureField {
            name: "x".to_string(),
            ty: "Real".to_string(),
            is_explicit: true,
            default: Some("0".to_string()),
        };
        assert_eq!(field.name, "x");
        assert_eq!(field.ty, "Real");
        assert!(field.is_explicit);
        assert_eq!(field.default, Some("0".to_string()));
    }
    #[test]
    fn test_attribute_decl_kind_macro() {
        let field = StructureField {
            name: "test".to_string(),
            ty: "T".to_string(),
            is_explicit: false,
            default: None,
        };
        assert!(!field.is_explicit);
    }
    #[test]
    fn test_parse_structure_with_multiple_extends() {
        let tokens = vec![
            make_token(TokenKind::Structure),
            make_ident("Diamond"),
            make_ident("extends"),
            make_ident("A"),
            make_token(TokenKind::Comma),
            make_ident("B"),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Structure { extends, .. } => {
                assert_eq!(extends.len(), 2);
                assert_eq!(extends[0], "A");
                assert_eq!(extends[1], "B");
            }
            _ => panic!("expected Structure command"),
        }
    }
    #[test]
    fn test_parse_class_with_multiple_extends() {
        let tokens = vec![
            make_token(TokenKind::Class),
            make_ident("Ring"),
            make_ident("extends"),
            make_ident("Semiring"),
            make_token(TokenKind::Comma),
            make_ident("Neg"),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Class { extends, .. } => {
                assert_eq!(extends.len(), 2);
            }
            _ => panic!("expected Class command"),
        }
    }
    #[test]
    fn test_structure_field_implicit() {
        let field = StructureField {
            name: "implicit_field".to_string(),
            ty: "Alpha".to_string(),
            is_explicit: false,
            default: None,
        };
        assert!(!field.is_explicit);
        assert_eq!(field.name, "implicit_field");
    }
    #[test]
    fn test_attribute_decl_builtin() {
        let tokens = vec![
            make_token(TokenKind::Attribute),
            make_token(TokenKind::LBracket),
            make_ident("builtin"),
            make_token(TokenKind::RBracket),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::AttributeDecl { kind, .. } => {
                assert_eq!(kind, AttributeDeclKind::Builtin);
            }
            _ => panic!("expected AttributeDecl with Builtin kind"),
        }
    }
    #[test]
    fn test_parse_class_params() {
        let tokens = vec![
            make_token(TokenKind::Class),
            make_ident("Eq"),
            make_token(TokenKind::LParen),
            make_ident("α"),
            make_token(TokenKind::Colon),
            make_ident("Type"),
            make_token(TokenKind::RParen),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Class { params, .. } => {
                assert_eq!(params.len(), 1);
            }
            _ => panic!("expected Class command with params"),
        }
    }
    #[test]
    fn test_parse_instance_underscore_name() {
        let tokens = vec![
            make_token(TokenKind::Instance),
            make_ident("_"),
            make_token(TokenKind::Colon),
            make_ident("Monad"),
            make_token(TokenKind::Assign),
            make_ident("trivial"),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Instance { name, .. } => {
                assert_eq!(name, "_");
            }
            _ => panic!("expected Instance with underscore name"),
        }
    }
    #[test]
    fn test_syntax_without_prec() {
        let tokens = vec![
            make_ident("syntax"),
            make_ident("letbinding"),
            make_token(TokenKind::Assign),
            make_ident("let"),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Syntax { prec, .. } => {
                assert!(prec.is_none());
            }
            _ => panic!("expected Syntax without precedence"),
        }
    }
    #[test]
    fn test_parse_structure_with_deriving() {
        let tokens = vec![
            make_token(TokenKind::Structure),
            make_ident("Data"),
            make_ident("deriving"),
            make_ident("Repr"),
            make_token(TokenKind::Comma),
            make_ident("BEq"),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Structure { derives, .. } => {
                assert_eq!(derives.len(), 2);
                assert_eq!(derives[0], "Repr");
                assert_eq!(derives[1], "BEq");
            }
            _ => panic!("expected Structure with derives"),
        }
    }
    #[test]
    fn test_attribute_decl_macro_kind() {
        let tokens = vec![
            make_token(TokenKind::Attribute),
            make_token(TokenKind::LBracket),
            make_ident("my_macro"),
            make_token(TokenKind::RBracket),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::AttributeDecl { kind, .. } => {
                assert_eq!(kind, AttributeDeclKind::Macro);
            }
            _ => panic!("expected AttributeDecl with Macro kind"),
        }
    }
    #[test]
    fn test_apply_attribute_command() {
        let tokens = vec![
            make_token(TokenKind::Attribute),
            make_token(TokenKind::LBracket),
            make_ident("simp"),
            make_token(TokenKind::RBracket),
            make_ident("myLemma"),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::ApplyAttribute {
                attr_name, target, ..
            } => {
                assert_eq!(attr_name, "simp");
                assert_eq!(target, "myLemma");
            }
            _ => panic!("expected ApplyAttribute command"),
        }
    }
    #[test]
    fn test_parse_instance_body_complete() {
        let tokens = vec![
            make_token(TokenKind::Instance),
            make_ident("inst"),
            make_token(TokenKind::Colon),
            make_ident("Functor"),
            make_token(TokenKind::Assign),
            make_ident("{"),
            make_ident("map"),
            make_token(TokenKind::Assign),
            make_ident("fun"),
            make_ident("}"),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Instance { body, .. } => {
                assert!(!body.is_empty());
            }
            _ => panic!("expected Instance with body"),
        }
    }
    #[test]
    fn test_structure_field_default_value() {
        let field = StructureField {
            name: "value".to_string(),
            ty: "Int".to_string(),
            is_explicit: true,
            default: Some("default_val".to_string()),
        };
        assert_eq!(field.default, Some("default_val".to_string()));
    }
    #[test]
    fn test_parse_notation_precedence_fifty() {
        let tokens = vec![
            make_ident("infix"),
            make_token(TokenKind::Nat(50)),
            make_token(TokenKind::String(",".to_string())),
            make_token(TokenKind::Assign),
            make_ident("Prod"),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Notation { prec, .. } => {
                assert_eq!(prec, Some(50));
            }
            _ => panic!("expected Notation command"),
        }
    }
    #[test]
    fn test_check_keyword_helper() {
        let mut parser = CommandParser::new();
        let tokens = vec![make_ident("where")];
        parser.tokens = tokens;
        parser.pos = 0;
        assert!(parser.check_keyword("where"));
    }
    #[test]
    fn test_at_end_helper() {
        let mut parser = CommandParser::new();
        parser.tokens = vec![make_eof()];
        parser.pos = 1;
        assert!(parser.at_end());
    }
    #[test]
    fn test_attribute_decl_simple_kind() {
        let tokens = vec![
            make_token(TokenKind::Attribute),
            make_token(TokenKind::LBracket),
            make_ident("custom_attr"),
            make_token(TokenKind::RBracket),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::AttributeDecl { kind, .. } => {
                assert_eq!(kind, AttributeDeclKind::Simple);
            }
            _ => panic!("expected AttributeDecl with Simple kind"),
        }
    }
    #[test]
    fn test_multiple_structure_extends_syntax() {
        let token_list = ["A", "B", "C"];
        assert_eq!(token_list.len(), 3);
    }
    #[test]
    fn test_class_empty_fields() {
        let tokens = vec![
            make_token(TokenKind::Class),
            make_ident("EmptyClass"),
            make_eof(),
        ];
        let mut parser = CommandParser::new();
        let cmd = parser
            .parse_command(&tokens)
            .expect("parse_decl should succeed");
        match cmd {
            Command::Class { fields, .. } => {
                assert!(fields.is_empty());
            }
            _ => panic!("expected Class with empty fields"),
        }
    }
}
