//! `Cfg::is_in_cnf` on single-symbol right-hand sides, against a verbatim
//! copy of the previous test, which counted chars and then took the first
//! with `expect`.

use super::Cfg;

fn old_single_terminal_ok(a: &str, terminals: &[char]) -> bool {
    !(a.chars().count() != 1 || !terminals.contains(&a.chars().next().expect("exactly one char")))
}

#[test]
fn single_symbol_rules_are_judged_as_before() {
    let terminals = vec!['a', 'b', 'é'];
    for rhs in ["a", "b", "é", "c", "", "ab", "aa", "é\u{301}", "A", "S"] {
        let cfg = Cfg {
            variables: vec!["S".to_string(), "A".to_string()],
            terminals: terminals.clone(),
            rules: vec![("S".to_string(), vec![rhs.to_string()])],
            start: "S".to_string(),
        };
        assert_eq!(
            cfg.is_in_cnf(),
            old_single_terminal_ok(rhs, &terminals),
            "{rhs:?}"
        );
    }
}
