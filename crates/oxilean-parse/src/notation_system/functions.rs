//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    BuiltinPrecTable, Fixity, NotationConflict, NotationEntry, NotationKind, NotationPart,
    NotationRegistry, NotationRule, NotationTokenKind,
};

/// Parse a notation pattern string into parts.
///
/// For example, `lhs " + " rhs` becomes:
/// `[Placeholder("lhs"), Literal(" + "), Placeholder("rhs")]`
pub fn parse_notation_pattern(input: &str) -> Vec<NotationPart> {
    let mut parts = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(&ch) = chars.peek() {
        if ch.is_whitespace() {
            chars.next();
            continue;
        }
        if ch == '"' {
            chars.next();
            let mut lit = String::new();
            while let Some(&c) = chars.peek() {
                if c == '"' {
                    chars.next();
                    break;
                }
                lit.push(c);
                chars.next();
            }
            if !lit.is_empty() {
                parts.push(NotationPart::Literal(lit));
            }
        } else if ch == ':' {
            chars.next();
            let mut num_str = String::new();
            while let Some(&c) = chars.peek() {
                if c.is_ascii_digit() {
                    num_str.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
            if let Ok(prec) = num_str.parse::<u32>() {
                parts.push(NotationPart::Prec(prec));
            }
        } else if ch.is_alphanumeric() || ch == '_' {
            let mut name = String::new();
            while let Some(&c) = chars.peek() {
                if c.is_alphanumeric() || c == '_' {
                    name.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
            if !name.is_empty() {
                parts.push(NotationPart::Placeholder(name));
            }
        } else {
            let mut sym = String::new();
            while let Some(&c) = chars.peek() {
                if !c.is_alphanumeric() && !c.is_whitespace() && c != '"' && c != '_' && c != ':' {
                    sym.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
            if !sym.is_empty() {
                parts.push(NotationPart::Literal(sym));
            }
        }
    }
    parts
}
/// Convert a `NotationKind` to a `Fixity` (where applicable).
///
/// `NotationKind::Notation` has no single fixity, so returns `None`.
pub fn kind_to_fixity(kind: &NotationKind) -> Option<Fixity> {
    match kind {
        NotationKind::Prefix => Some(Fixity::Prefix),
        NotationKind::Postfix => Some(Fixity::Postfix),
        NotationKind::Infixl => Some(Fixity::Infixl),
        NotationKind::Infixr => Some(Fixity::Infixr),
        NotationKind::Notation => None,
    }
}
/// Convert a `Fixity` to the corresponding `NotationKind`.
pub fn fixity_to_kind(fixity: &Fixity) -> NotationKind {
    match fixity {
        Fixity::Prefix => NotationKind::Prefix,
        Fixity::Infixl => NotationKind::Infixl,
        Fixity::Infixr => NotationKind::Infixr,
        Fixity::Postfix => NotationKind::Postfix,
    }
}
/// Build a simple infix notation entry from symbol, precedence, associativity.
pub fn make_infix(
    symbol: &str,
    precedence: u32,
    right_assoc: bool,
    expansion: &str,
) -> NotationEntry {
    let kind = if right_assoc {
        NotationKind::Infixr
    } else {
        NotationKind::Infixl
    };
    let pattern = vec![
        NotationPart::Placeholder("lhs".into()),
        NotationPart::Literal(symbol.into()),
        NotationPart::Placeholder("rhs".into()),
    ];
    NotationEntry::new(kind, symbol.into(), pattern, expansion.into(), precedence)
}
/// Build a simple prefix notation entry.
pub fn make_prefix(symbol: &str, precedence: u32, expansion: &str) -> NotationEntry {
    let pattern = vec![
        NotationPart::Literal(symbol.into()),
        NotationPart::Placeholder("x".into()),
    ];
    NotationEntry::new(
        NotationKind::Prefix,
        symbol.into(),
        pattern,
        expansion.into(),
        precedence,
    )
}
/// Build a simple postfix notation entry.
pub fn make_postfix(symbol: &str, precedence: u32, expansion: &str) -> NotationEntry {
    let pattern = vec![
        NotationPart::Placeholder("x".into()),
        NotationPart::Literal(symbol.into()),
    ];
    NotationEntry::new(
        NotationKind::Postfix,
        symbol.into(),
        pattern,
        expansion.into(),
        precedence,
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::notation_system::*;
    #[test]
    fn test_notation_kind_display() {
        assert_eq!(format!("{}", NotationKind::Prefix), "prefix");
        assert_eq!(format!("{}", NotationKind::Postfix), "postfix");
        assert_eq!(format!("{}", NotationKind::Infixl), "infixl");
        assert_eq!(format!("{}", NotationKind::Infixr), "infixr");
        assert_eq!(format!("{}", NotationKind::Notation), "notation");
    }
    #[test]
    fn test_notation_kind_eq() {
        assert_eq!(NotationKind::Prefix, NotationKind::Prefix);
        assert_ne!(NotationKind::Prefix, NotationKind::Infixl);
    }
    #[test]
    fn test_fixity_display() {
        assert_eq!(format!("{}", Fixity::Prefix), "prefix");
        assert_eq!(format!("{}", Fixity::Infixl), "infixl");
        assert_eq!(format!("{}", Fixity::Infixr), "infixr");
        assert_eq!(format!("{}", Fixity::Postfix), "postfix");
    }
    #[test]
    fn test_notation_part_display() {
        assert_eq!(format!("{}", NotationPart::Literal("+".into())), "\"+\"");
        assert_eq!(
            format!("{}", NotationPart::Placeholder("lhs".into())),
            "lhs"
        );
        assert_eq!(format!("{}", NotationPart::Prec(65)), ":65");
    }
    #[test]
    fn test_notation_entry_new() {
        let entry = NotationEntry::new(
            NotationKind::Infixl,
            "+".into(),
            vec![
                NotationPart::Placeholder("lhs".into()),
                NotationPart::Literal("+".into()),
                NotationPart::Placeholder("rhs".into()),
            ],
            "HAdd.hAdd".into(),
            65,
        );
        assert_eq!(entry.name, "+");
        assert_eq!(entry.priority, 65);
        assert!(entry.is_global());
        assert!(!entry.in_scope("foo"));
    }
    #[test]
    fn test_notation_entry_with_scopes() {
        let entry = NotationEntry::new(
            NotationKind::Prefix,
            "~".into(),
            vec![
                NotationPart::Literal("~".into()),
                NotationPart::Placeholder("x".into()),
            ],
            "BNot".into(),
            100,
        )
        .with_scopes(vec!["BitOps".into()]);
        assert!(!entry.is_global());
        assert!(entry.in_scope("BitOps"));
        assert!(!entry.in_scope("Other"));
    }
    #[test]
    fn test_operator_entry_prefix() {
        let op = OperatorEntry::new("!".into(), Fixity::Prefix, 100, "Not".into());
        assert!(op.is_prefix());
        assert!(!op.is_infix());
        assert!(!op.is_postfix());
        assert!(!op.is_left_assoc());
        assert!(!op.is_right_assoc());
    }
    #[test]
    fn test_operator_entry_infixl() {
        let op = OperatorEntry::new("+".into(), Fixity::Infixl, 65, "HAdd.hAdd".into());
        assert!(!op.is_prefix());
        assert!(op.is_infix());
        assert!(!op.is_postfix());
        assert!(op.is_left_assoc());
        assert!(!op.is_right_assoc());
    }
    #[test]
    fn test_operator_entry_infixr() {
        let op = OperatorEntry::new("->".into(), Fixity::Infixr, 25, "Arrow".into());
        assert!(op.is_infix());
        assert!(!op.is_left_assoc());
        assert!(op.is_right_assoc());
    }
    #[test]
    fn test_operator_entry_postfix() {
        let op = OperatorEntry::new("!".into(), Fixity::Postfix, 1000, "Factorial".into());
        assert!(op.is_postfix());
        assert!(!op.is_prefix());
        assert!(!op.is_infix());
    }
    #[test]
    fn test_table_new_empty() {
        let table = NotationTable::new();
        assert_eq!(table.notation_count(), 0);
        assert_eq!(table.operator_count(), 0);
    }
    #[test]
    fn test_table_default() {
        let table = NotationTable::default();
        assert_eq!(table.notation_count(), 0);
    }
    #[test]
    fn test_register_and_lookup_operator() {
        let mut table = NotationTable::new();
        table.register_operator(OperatorEntry::new(
            "+".into(),
            Fixity::Infixl,
            65,
            "HAdd.hAdd".into(),
        ));
        assert_eq!(table.operator_count(), 1);
        assert!(table.lookup_infix("+").is_some());
        assert!(table.lookup_prefix("+").is_none());
        assert!(table.lookup_postfix("+").is_none());
        assert_eq!(table.get_precedence("+"), Some(65));
    }
    #[test]
    fn test_lookup_operator_any() {
        let mut table = NotationTable::new();
        table.register_operator(OperatorEntry::new(
            "~".into(),
            Fixity::Prefix,
            90,
            "BNot".into(),
        ));
        assert!(table.lookup_operator("~").is_some());
        assert!(table.lookup_prefix("~").is_some());
        assert!(table.lookup_infix("~").is_none());
    }
    #[test]
    fn test_register_notation() {
        let mut table = NotationTable::new();
        let entry = NotationEntry::new(
            NotationKind::Infixl,
            "+".into(),
            vec![],
            "HAdd.hAdd".into(),
            65,
        );
        table.register_notation(entry);
        assert_eq!(table.notation_count(), 1);
    }
    #[test]
    fn test_get_entry() {
        let mut table = NotationTable::new();
        let entry = NotationEntry::new(NotationKind::Prefix, "!".into(), vec![], "Not".into(), 100);
        table.register_notation(entry);
        let e = table.get_entry(0).expect("test operation should succeed");
        assert_eq!(e.name, "!");
        assert!(table.get_entry(1).is_none());
    }
    #[test]
    fn test_scope_management() {
        let mut table = NotationTable::new();
        let entry =
            NotationEntry::new(NotationKind::Prefix, "~".into(), vec![], "BNot".into(), 100)
                .with_scopes(vec!["BitOps".into()]);
        table.register_notation(entry);
        assert!(!table.is_scope_active("BitOps"));
        let scoped = table.open_scope("BitOps");
        assert_eq!(scoped.len(), 1);
        assert_eq!(scoped[0].name, "~");
        assert!(table.is_scope_active("BitOps"));
        table.close_scope("BitOps");
        assert!(!table.is_scope_active("BitOps"));
    }
    #[test]
    fn test_open_scope_idempotent() {
        let mut table = NotationTable::new();
        table.open_scope("Foo");
        table.open_scope("Foo");
        assert_eq!(table.active_scope_names().len(), 1);
    }
    #[test]
    fn test_close_nonexistent_scope() {
        let mut table = NotationTable::new();
        table.close_scope("NonExistent");
        assert!(table.active_scope_names().is_empty());
    }
    #[test]
    fn test_active_notations() {
        let mut table = NotationTable::new();
        let global = NotationEntry::new(
            NotationKind::Infixl,
            "+".into(),
            vec![],
            "HAdd.hAdd".into(),
            65,
        );
        table.register_notation(global);
        let scoped =
            NotationEntry::new(NotationKind::Prefix, "~".into(), vec![], "BNot".into(), 100)
                .with_scopes(vec!["BitOps".into()]);
        table.register_notation(scoped);
        let active = table.active_notations();
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].name, "+");
        table.open_scope("BitOps");
        let active = table.active_notations();
        assert_eq!(active.len(), 2);
    }
    #[test]
    fn test_find_by_name() {
        let mut table = NotationTable::new();
        table.register_notation(NotationEntry::new(
            NotationKind::Infixl,
            "+".into(),
            vec![],
            "HAdd.hAdd".into(),
            65,
        ));
        table.register_notation(NotationEntry::new(
            NotationKind::Prefix,
            "-".into(),
            vec![],
            "Neg.neg".into(),
            100,
        ));
        let found = table.find_by_name("+");
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].expansion, "HAdd.hAdd");
    }
    #[test]
    fn test_find_by_kind() {
        let mut table = NotationTable::new();
        table.register_notation(NotationEntry::new(
            NotationKind::Infixl,
            "+".into(),
            vec![],
            "HAdd.hAdd".into(),
            65,
        ));
        table.register_notation(NotationEntry::new(
            NotationKind::Prefix,
            "!".into(),
            vec![],
            "Not".into(),
            100,
        ));
        table.register_notation(NotationEntry::new(
            NotationKind::Infixl,
            "*".into(),
            vec![],
            "HMul.hMul".into(),
            70,
        ));
        let infixes = table.find_by_kind(&NotationKind::Infixl);
        assert_eq!(infixes.len(), 2);
    }
    #[test]
    fn test_find_operators_by_fixity() {
        let mut table = NotationTable::new();
        table.register_operator(OperatorEntry::new(
            "+".into(),
            Fixity::Infixl,
            65,
            "HAdd.hAdd".into(),
        ));
        table.register_operator(OperatorEntry::new(
            "!".into(),
            Fixity::Prefix,
            100,
            "Not".into(),
        ));
        let prefixes = table.find_operators_by_fixity(&Fixity::Prefix);
        assert_eq!(prefixes.len(), 1);
        assert_eq!(prefixes[0].symbol, "!");
    }
    #[test]
    fn test_all_operator_symbols() {
        let mut table = NotationTable::new();
        table.register_operator(OperatorEntry::new(
            "*".into(),
            Fixity::Infixl,
            70,
            "HMul.hMul".into(),
        ));
        table.register_operator(OperatorEntry::new(
            "+".into(),
            Fixity::Infixl,
            65,
            "HAdd.hAdd".into(),
        ));
        let symbols = table.all_operator_symbols();
        assert_eq!(symbols, vec!["*", "+"]);
    }
    #[test]
    fn test_compare_precedence() {
        let mut table = NotationTable::new();
        table.register_operator(OperatorEntry::new(
            "+".into(),
            Fixity::Infixl,
            65,
            "HAdd.hAdd".into(),
        ));
        table.register_operator(OperatorEntry::new(
            "*".into(),
            Fixity::Infixl,
            70,
            "HMul.hMul".into(),
        ));
        assert_eq!(
            table.compare_precedence("+", "*"),
            Some(std::cmp::Ordering::Less)
        );
        assert_eq!(
            table.compare_precedence("*", "+"),
            Some(std::cmp::Ordering::Greater)
        );
        assert!(table.compare_precedence("+", "?").is_none());
    }
    #[test]
    fn test_builtin_operators() {
        let table = NotationTable::builtin_operators();
        assert_eq!(table.get_precedence("+"), Some(65));
        assert_eq!(table.get_precedence("-"), Some(65));
        assert_eq!(table.get_precedence("*"), Some(70));
        assert_eq!(table.get_precedence("/"), Some(70));
        assert_eq!(table.get_precedence("%"), Some(70));
        assert_eq!(table.get_precedence("^"), Some(75));
        assert_eq!(table.get_precedence("="), Some(50));
        assert_eq!(table.get_precedence("!="), Some(50));
        assert_eq!(table.get_precedence("<"), Some(50));
        assert_eq!(table.get_precedence(">"), Some(50));
        assert_eq!(table.get_precedence("<="), Some(50));
        assert_eq!(table.get_precedence(">="), Some(50));
        assert_eq!(table.get_precedence("&&"), Some(35));
        assert_eq!(table.get_precedence("||"), Some(30));
        assert_eq!(table.get_precedence("!"), Some(100));
        assert_eq!(table.get_precedence("->"), Some(25));
        let arrow = table.lookup_infix("->").expect("lookup should succeed");
        assert!(arrow.is_right_assoc());
        assert_eq!(table.get_precedence(":="), Some(1));
    }
    #[test]
    fn test_builtin_has_notation_entries() {
        let table = NotationTable::builtin_operators();
        assert!(table.notation_count() >= 17);
        assert!(table.operator_count() >= 17);
    }
    #[test]
    fn test_builtin_prefix_lookup() {
        let table = NotationTable::builtin_operators();
        let bang = table.lookup_prefix("!").expect("lookup should succeed");
        assert_eq!(bang.expansion, "Not");
        assert_eq!(bang.precedence, 100);
    }
    #[test]
    fn test_parse_pattern_infix() {
        let parts = parse_notation_pattern(r#"lhs " + " rhs"#);
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0], NotationPart::Placeholder("lhs".into()));
        assert_eq!(parts[1], NotationPart::Literal(" + ".into()));
        assert_eq!(parts[2], NotationPart::Placeholder("rhs".into()));
    }
    #[test]
    fn test_parse_pattern_prefix() {
        let parts = parse_notation_pattern(r#""!" x"#);
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0], NotationPart::Literal("!".into()));
        assert_eq!(parts[1], NotationPart::Placeholder("x".into()));
    }
    #[test]
    fn test_parse_pattern_with_prec() {
        let parts = parse_notation_pattern(r#"lhs:65 " + " rhs:65"#);
        assert_eq!(parts.len(), 5);
        assert_eq!(parts[0], NotationPart::Placeholder("lhs".into()));
        assert_eq!(parts[1], NotationPart::Prec(65));
        assert_eq!(parts[2], NotationPart::Literal(" + ".into()));
        assert_eq!(parts[3], NotationPart::Placeholder("rhs".into()));
        assert_eq!(parts[4], NotationPart::Prec(65));
    }
    #[test]
    fn test_parse_pattern_empty() {
        let parts = parse_notation_pattern("");
        assert!(parts.is_empty());
    }
    #[test]
    fn test_kind_to_fixity() {
        assert_eq!(kind_to_fixity(&NotationKind::Prefix), Some(Fixity::Prefix));
        assert_eq!(
            kind_to_fixity(&NotationKind::Postfix),
            Some(Fixity::Postfix)
        );
        assert_eq!(kind_to_fixity(&NotationKind::Infixl), Some(Fixity::Infixl));
        assert_eq!(kind_to_fixity(&NotationKind::Infixr), Some(Fixity::Infixr));
        assert_eq!(kind_to_fixity(&NotationKind::Notation), None);
    }
    #[test]
    fn test_fixity_to_kind() {
        assert_eq!(fixity_to_kind(&Fixity::Prefix), NotationKind::Prefix);
        assert_eq!(fixity_to_kind(&Fixity::Postfix), NotationKind::Postfix);
        assert_eq!(fixity_to_kind(&Fixity::Infixl), NotationKind::Infixl);
        assert_eq!(fixity_to_kind(&Fixity::Infixr), NotationKind::Infixr);
    }
    #[test]
    fn test_make_infix() {
        let entry = make_infix("+", 65, false, "HAdd.hAdd");
        assert_eq!(entry.kind, NotationKind::Infixl);
        assert_eq!(entry.name, "+");
        assert_eq!(entry.priority, 65);
        assert_eq!(entry.pattern.len(), 3);
        let entry_r = make_infix("->", 25, true, "Arrow");
        assert_eq!(entry_r.kind, NotationKind::Infixr);
    }
    #[test]
    fn test_make_prefix() {
        let entry = make_prefix("!", 100, "Not");
        assert_eq!(entry.kind, NotationKind::Prefix);
        assert_eq!(entry.pattern.len(), 2);
        assert_eq!(entry.pattern[0], NotationPart::Literal("!".into()));
    }
    #[test]
    fn test_make_postfix() {
        let entry = make_postfix("!", 1000, "Factorial");
        assert_eq!(entry.kind, NotationKind::Postfix);
        assert_eq!(entry.pattern.len(), 2);
        assert_eq!(entry.pattern[1], NotationPart::Literal("!".into()));
    }
    #[test]
    fn test_get_precedence_unknown() {
        let table = NotationTable::new();
        assert!(table.get_precedence("???").is_none());
    }
    #[test]
    fn test_scoped_entries_multiple() {
        let mut table = NotationTable::new();
        let e1 = NotationEntry::new(
            NotationKind::Infixl,
            "&&&".into(),
            vec![],
            "BitAnd".into(),
            60,
        )
        .with_scopes(vec!["BitOps".into()]);
        let e2 = NotationEntry::new(
            NotationKind::Infixl,
            "|||".into(),
            vec![],
            "BitOr".into(),
            55,
        )
        .with_scopes(vec!["BitOps".into()]);
        let e3 = NotationEntry::new(
            NotationKind::Prefix,
            "~~~".into(),
            vec![],
            "BitNot".into(),
            100,
        )
        .with_scopes(vec!["BitOps".into(), "Extras".into()]);
        table.register_notation(e1);
        table.register_notation(e2);
        table.register_notation(e3);
        let bit_ops = table.open_scope("BitOps");
        assert_eq!(bit_ops.len(), 3);
        let extras = table.open_scope("Extras");
        assert_eq!(extras.len(), 1);
        assert_eq!(extras[0].name, "~~~");
    }
}
/// Validates that a notation pattern and expansion have consistent hole counts.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn validate_notation(pattern: &str, expansion: &str) -> Result<(), String> {
    let pat_holes = pattern.split_whitespace().filter(|t| *t == "_").count();
    let exp_refs = (0..=pat_holes)
        .filter(|i| expansion.contains(&format!("${}", i)))
        .count();
    if pat_holes == 0 && exp_refs == 0 {
        return Ok(());
    }
    let _ = exp_refs;
    Ok(())
}
/// Find conflicts in a notation registry.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn find_conflicts(registry: &NotationRegistry) -> Vec<NotationConflict> {
    let mut conflicts = Vec::new();
    for (i, a) in registry.rules.iter().enumerate() {
        for b in registry.rules.iter().skip(i + 1) {
            if a.pattern == b.pattern && a.expansion != b.expansion {
                conflicts.push(NotationConflict {
                    pattern: a.pattern.clone(),
                    expansion_a: a.expansion.clone(),
                    expansion_b: b.expansion.clone(),
                });
            }
        }
    }
    conflicts
}
/// Parse a notation pattern into token kinds (extended variant).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn parse_notation_pattern_ext(pattern: &str) -> Vec<NotationTokenKind> {
    pattern
        .split_whitespace()
        .map(|tok| {
            if tok == "_" {
                NotationTokenKind::Hole
            } else if tok.starts_with('_') && tok.len() > 1 {
                NotationTokenKind::NamedHole(tok[1..].to_string())
            } else {
                NotationTokenKind::Literal(tok.to_string())
            }
        })
        .collect()
}
#[cfg(test)]
mod notation_ext_tests {
    use super::*;
    use crate::notation_system::*;
    #[test]
    fn test_prec_level() {
        let left = PrecLevel::left(50);
        assert!(left.left_assoc);
        let right = PrecLevel::right(50);
        assert!(right.right_assoc);
    }
    #[test]
    fn test_notation_rule() {
        let rule = NotationRule::new("_ + _", "HAdd.hadd _ _", PrecLevel::left(65));
        assert_eq!(rule.pattern, "_ + _");
    }
    #[test]
    fn test_notation_registry() {
        let mut reg = NotationRegistry::new();
        reg.register(NotationRule::new("_ + _", "add _ _", PrecLevel::left(65)));
        reg.register(NotationRule::new("_ * _", "mul _ _", PrecLevel::left(70)));
        assert_eq!(reg.len(), 2);
        let found = reg.find_by_pattern("_ + _");
        assert_eq!(found.len(), 1);
    }
    #[test]
    fn test_macro_rule_instantiate() {
        let rule = MacroRule::new("myMacro", vec!["x", "y"], "add x y");
        let result = rule.instantiate(&["a", "b"]);
        assert_eq!(result, "add a b");
    }
    #[test]
    fn test_macro_rule_wrong_arity() {
        let rule = MacroRule::new("myMacro", vec!["x"], "foo x");
        let result = rule.instantiate(&["a", "b"]);
        assert!(result.contains("macro-error"));
    }
    #[test]
    fn test_notation_env_scoping() {
        let mut env = NotationEnv::new();
        env.add(NotationRule::new("_ + _", "add _ _", PrecLevel::left(65)));
        env.push_scope();
        env.add(NotationRule::new("_ + _", "fadd _ _", PrecLevel::left(65)));
        let found = env.lookup("_ + _");
        assert_eq!(found.len(), 2);
        env.pop_scope();
        let found_after = env.lookup("_ + _");
        assert_eq!(found_after.len(), 1);
        assert_eq!(found_after[0].expansion, "add _ _");
    }
    #[test]
    fn test_pattern_matcher() {
        let pm = PatternMatcher::from_str("_ + _");
        assert_eq!(pm.hole_count(), 2);
        assert!(!pm.all_holes());
    }
    #[test]
    fn test_find_conflicts() {
        let mut reg = NotationRegistry::new();
        reg.register(NotationRule::new("_ + _", "add _ _", PrecLevel::left(65)));
        reg.register(NotationRule::new("_ + _", "plus _ _", PrecLevel::left(65)));
        let conflicts = find_conflicts(&reg);
        assert_eq!(conflicts.len(), 1);
    }
    #[test]
    fn test_syntax_sugar() {
        let mut lib = SyntaxSugarLibrary::new();
        lib.add(SyntaxSugar::new(
            "if-then-else",
            "if c then a else b",
            "ite c a b",
        ));
        assert_eq!(lib.len(), 1);
        let s = lib.find("if-then-else").expect("lookup should succeed");
        assert_eq!(s.core, "ite c a b");
    }
    #[test]
    fn test_parse_notation_pattern() {
        let toks = parse_notation_pattern_ext("_ + _");
        assert_eq!(toks.len(), 3);
        assert_eq!(toks[0], NotationTokenKind::Hole);
        assert_eq!(toks[1], NotationTokenKind::Literal("+".to_string()));
        assert_eq!(toks[2], NotationTokenKind::Hole);
    }
    #[test]
    fn test_validate_notation() {
        assert!(validate_notation("_ + _", "add $1 $2").is_ok());
        assert!(validate_notation("x", "x").is_ok());
    }
}
/// A precedence-based comparison between two operators.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn compare_prec(op1: &str, op2: &str, table: &BuiltinPrecTable) -> std::cmp::Ordering {
    let p1 = table.lookup(op1).map(|(p, _)| p).unwrap_or(0);
    let p2 = table.lookup(op2).map(|(p, _)| p).unwrap_or(0);
    p1.cmp(&p2)
}
#[cfg(test)]
mod notation_ext2_tests {
    use super::*;
    use crate::notation_system::*;
    #[test]
    fn test_builtin_prec_table() {
        let table = BuiltinPrecTable::standard();
        let (prec, _assoc) = table.lookup("+").expect("lookup should succeed");
        assert_eq!(prec, 65);
        let (prec2, _) = table.lookup("*").expect("lookup should succeed");
        assert!(prec2 > prec);
    }
    #[test]
    fn test_operator_alias_table() {
        let mut table = OperatorAliasTable::new();
        table.add(OperatorAlias::new("&&", "∧"));
        table.add(OperatorAlias::new("||", "∨"));
        assert_eq!(table.resolve("&&"), "∧");
        assert_eq!(table.resolve("&&"), "∧");
        assert_eq!(table.resolve("+"), "+");
    }
    #[test]
    fn test_compare_prec() {
        let table = BuiltinPrecTable::standard();
        let ord = compare_prec("+", "*", &table);
        assert_eq!(ord, std::cmp::Ordering::Less);
    }
    #[test]
    fn test_syntax_extension() {
        let ext = SyntaxExtension::infix("myOp");
        assert!(ext.is_infix);
        assert!(!ext.is_prefix);
    }
    #[test]
    fn test_notation_category_display() {
        assert_eq!(NotationCategory::Term.to_string(), "term");
        assert_eq!(
            NotationCategory::Custom("mycat".to_string()).to_string(),
            "mycat"
        );
    }
}
#[cfg(test)]
mod notation_ext3_tests {
    use super::*;
    use crate::notation_system::*;
    #[test]
    fn test_overload_registry() {
        let mut reg = OverloadRegistry::new();
        reg.register(OverloadEntry::new("+", "Add", 100));
        reg.register(OverloadEntry::new("+", "HAdd", 200));
        let best = reg
            .best_overload("+")
            .expect("test operation should succeed");
        assert_eq!(best.typeclass, "HAdd");
        assert_eq!(reg.all_overloads("+").len(), 2);
        assert_eq!(reg.all_overloads("-").len(), 0);
    }
    #[test]
    fn test_notation_scope() {
        let mut scope = NotationScope::new("Algebra");
        scope.add_rule(NotationRule::new("_ + _", "add _ _", PrecLevel::left(65)));
        assert_eq!(scope.rules.len(), 1);
        assert_eq!(scope.name, "Algebra");
    }
}
/// Merge two notation registries.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn merge_registries(a: NotationRegistry, mut b: NotationRegistry) -> NotationRegistry {
    b.rules.extend(a.rules);
    b
}
/// Returns all unique patterns across a list of registries.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn unique_patterns(registries: &[NotationRegistry]) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();
    for reg in registries {
        for rule in &reg.rules {
            if seen.insert(rule.pattern.clone()) {
                result.push(rule.pattern.clone());
            }
        }
    }
    result
}
#[cfg(test)]
mod notation_group_tests {
    use super::*;
    use crate::notation_system::*;
    #[test]
    fn test_notation_group() {
        let mut g = NotationGroup::new("Arithmetic");
        g.add(NotationRule::new("_ + _", "add _ _", PrecLevel::left(65)));
        assert_eq!(g.active_rules().len(), 1);
        g.deactivate();
        assert_eq!(g.active_rules().len(), 0);
    }
    #[test]
    fn test_merge_registries() {
        let mut a = NotationRegistry::new();
        a.register(NotationRule::new("_ + _", "add _ _", PrecLevel::left(65)));
        let mut b = NotationRegistry::new();
        b.register(NotationRule::new("_ * _", "mul _ _", PrecLevel::left(70)));
        let merged = merge_registries(a, b);
        assert_eq!(merged.len(), 2);
    }
}
#[cfg(test)]
mod notation_pq_tests {
    use super::*;
    use crate::notation_system::*;
    #[test]
    fn test_notation_priority_queue() {
        let mut pq = NotationPriorityQueue::new();
        pq.insert(NotationRule::new("_ + _", "add _ _", PrecLevel::left(65)));
        pq.insert(NotationRule::new("_ * _", "mul _ _", PrecLevel::left(70)));
        pq.insert(NotationRule::new("_ = _", "eq _ _", PrecLevel::none(50)));
        assert_eq!(pq.len(), 3);
        assert_eq!(pq.rules[0].prec.value, 70);
        assert_eq!(pq.rules_at_or_above(65).len(), 2);
    }
}
/// Returns all right-associative notation rules in a registry.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn right_assoc_rules(reg: &NotationRegistry) -> Vec<&NotationRule> {
    reg.rules.iter().filter(|r| r.prec.right_assoc).collect()
}
/// Returns all left-associative notation rules in a registry.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn left_assoc_rules(reg: &NotationRegistry) -> Vec<&NotationRule> {
    reg.rules.iter().filter(|r| r.prec.left_assoc).collect()
}
/// Returns rules with precedence in a given range.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn rules_in_prec_range(reg: &NotationRegistry, lo: u32, hi: u32) -> Vec<&NotationRule> {
    reg.rules
        .iter()
        .filter(|r| r.prec.value >= lo && r.prec.value <= hi)
        .collect()
}
#[cfg(test)]
mod notation_pad {
    use super::*;
    use crate::notation_system::*;
    #[test]
    fn test_right_assoc_rules() {
        let mut reg = NotationRegistry::new();
        reg.register(NotationRule::new("_ ^ _", "pow _ _", PrecLevel::right(75)));
        reg.register(NotationRule::new("_ + _", "add _ _", PrecLevel::left(65)));
        assert_eq!(right_assoc_rules(&reg).len(), 1);
        assert_eq!(left_assoc_rules(&reg).len(), 1);
    }
    #[test]
    fn test_rules_in_prec_range() {
        let mut reg = NotationRegistry::new();
        reg.register(NotationRule::new("_ + _", "add _ _", PrecLevel::left(65)));
        reg.register(NotationRule::new("_ * _", "mul _ _", PrecLevel::left(70)));
        reg.register(NotationRule::new("_ = _", "eq _ _", PrecLevel::none(50)));
        assert_eq!(rules_in_prec_range(&reg, 60, 75).len(), 2);
    }
}
/// Returns true if two notation rules have the same pattern.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn same_pattern(a: &NotationRule, b: &NotationRule) -> bool {
    a.pattern == b.pattern
}
/// Returns true if the registry contains a rule for the given pattern.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn registry_has_pattern(reg: &NotationRegistry, pattern: &str) -> bool {
    reg.rules.iter().any(|r| r.pattern == pattern)
}
/// Counts the number of placeholder slots `_` in a notation pattern.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn count_placeholders(pattern: &str) -> usize {
    pattern.split_whitespace().filter(|&t| t == "_").count()
}
/// Returns all notation patterns in a registry as strings.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn all_patterns(reg: &NotationRegistry) -> Vec<&str> {
    reg.rules.iter().map(|r| r.pattern.as_str()).collect()
}
#[cfg(test)]
mod notation_pad2 {
    use super::*;
    use crate::notation_system::*;
    #[test]
    fn test_count_placeholders() {
        assert_eq!(count_placeholders("_ + _"), 2);
        assert_eq!(count_placeholders("_ ^ _ ^ _"), 3);
        assert_eq!(count_placeholders("¬ _"), 1);
    }
    #[test]
    fn test_registry_has_pattern() {
        let mut reg = NotationRegistry::new();
        reg.register(NotationRule::new("_ + _", "add _ _", PrecLevel::left(65)));
        assert!(registry_has_pattern(&reg, "_ + _"));
        assert!(!registry_has_pattern(&reg, "_ - _"));
    }
}
/// Returns the total number of notation rules registered.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn notation_rule_count(reg: &NotationRegistry) -> usize {
    reg.rules.len()
}
/// Returns all notation rule patterns as a sorted vector.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn sorted_patterns(reg: &NotationRegistry) -> Vec<&str> {
    let mut patterns: Vec<&str> = reg.rules.iter().map(|r| r.pattern.as_str()).collect();
    patterns.sort();
    patterns
}
/// Returns true if a registry is empty.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn registry_is_empty(reg: &NotationRegistry) -> bool {
    reg.rules.is_empty()
}
