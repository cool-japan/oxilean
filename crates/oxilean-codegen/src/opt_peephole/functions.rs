//! Functions for the peephole optimisation pass.

use super::types::{PeepInstr, PeepPattern, PeepReplacement, PeepResult, PeepRule};

// ── Standard rule set ─────────────────────────────────────────────────────────

/// Build the built-in set of peephole optimisation rules.
///
/// Rules are ordered by descending priority so callers can rely on stable
/// behaviour.  The rules provided are:
///
/// | Rule name              | Pattern                      | Replacement           |
/// |------------------------|------------------------------|-----------------------|
/// | `add_zero`             | `Const(0)`, `Add`            | *(empty)*             |
/// | `mul_one`              | `Const(1)`, `Mul`            | *(empty)*             |
/// | `mul_zero`             | `Const(0)`, `Mul`            | `Pop`, `Const(0)`     |
/// | `double_neg`           | `Neg`, `Neg`                 | *(empty)*             |
/// | `dup_pop`              | `Dup`, `Pop`                 | *(empty)*             |
/// | `dead_nop`             | `Nop`                        | *(empty)*             |
/// | `store_load_elim`      | `Store(x)`, `Load(x)`       | `Dup`, `Store(x)`     |
pub fn standard_rules() -> Vec<PeepRule> {
    let mut rules = vec![
        // add_zero: Const(0) Add  →  ε  (adding zero is identity)
        PeepRule {
            pattern: PeepPattern {
                instrs: vec![PeepInstr::Const(0), PeepInstr::Add],
                name: "add_zero_pattern".to_string(),
            },
            replacement: PeepReplacement {
                instrs: vec![],
                name: "add_zero_replacement".to_string(),
            },
            priority: 100,
        },
        // mul_one: Const(1) Mul  →  ε  (multiplying by one is identity)
        PeepRule {
            pattern: PeepPattern {
                instrs: vec![PeepInstr::Const(1), PeepInstr::Mul],
                name: "mul_one_pattern".to_string(),
            },
            replacement: PeepReplacement {
                instrs: vec![],
                name: "mul_one_replacement".to_string(),
            },
            priority: 100,
        },
        // mul_zero: Const(0) Mul  →  Pop Const(0)  (multiplying by zero)
        PeepRule {
            pattern: PeepPattern {
                instrs: vec![PeepInstr::Const(0), PeepInstr::Mul],
                name: "mul_zero_pattern".to_string(),
            },
            replacement: PeepReplacement {
                instrs: vec![PeepInstr::Pop, PeepInstr::Const(0)],
                name: "mul_zero_replacement".to_string(),
            },
            priority: 90,
        },
        // double_neg: Neg Neg  →  ε  (double negation cancels)
        PeepRule {
            pattern: PeepPattern {
                instrs: vec![PeepInstr::Neg, PeepInstr::Neg],
                name: "double_neg_pattern".to_string(),
            },
            replacement: PeepReplacement {
                instrs: vec![],
                name: "double_neg_replacement".to_string(),
            },
            priority: 80,
        },
        // dup_pop: Dup Pop  →  ε  (dup then immediately discard)
        PeepRule {
            pattern: PeepPattern {
                instrs: vec![PeepInstr::Dup, PeepInstr::Pop],
                name: "dup_pop_pattern".to_string(),
            },
            replacement: PeepReplacement {
                instrs: vec![],
                name: "dup_pop_replacement".to_string(),
            },
            priority: 80,
        },
        // dead_nop: Nop  →  ε  (remove useless no-ops)
        PeepRule {
            pattern: PeepPattern {
                instrs: vec![PeepInstr::Nop],
                name: "dead_nop_pattern".to_string(),
            },
            replacement: PeepReplacement {
                instrs: vec![],
                name: "dead_nop_replacement".to_string(),
            },
            priority: 50,
        },
    ];

    // store_load_elim rules are added dynamically by run_peephole using
    // apply_rule with variable-capturing logic — however we also include a
    // representative rule here for callers that apply rules manually.
    // The variable name "x" is a sentinel; `apply_rule` performs name-aware
    // matching when it detects a Store/Load pair with matching names.
    rules.push(PeepRule {
        pattern: PeepPattern {
            instrs: vec![
                PeepInstr::Store("__x__".to_string()),
                PeepInstr::Load("__x__".to_string()),
            ],
            name: "store_load_elim_pattern".to_string(),
        },
        replacement: PeepReplacement {
            instrs: vec![PeepInstr::Dup, PeepInstr::Store("__x__".to_string())],
            name: "store_load_elim_replacement".to_string(),
        },
        priority: 70,
    });

    // Sort descending by priority so the highest-priority rules are tried first.
    rules.sort_by_key(|x| std::cmp::Reverse(x.priority));
    rules
}

// ── Rule application ──────────────────────────────────────────────────────────

/// Try to apply `rule` at position `offset` within `instrs`.
///
/// Returns `Some(new_instrs)` when the pattern matches (replacing
/// `instrs[offset..offset + pattern_len]` with the replacement), otherwise
/// `None`.
///
/// Special case: when the rule pattern is a `Store(x)` / `Load(x)` with the
/// sentinel name `"__x__"`, the function matches any matching Store/Load pair
/// and substitutes the actual variable name into the replacement.
pub fn apply_rule(instrs: &[PeepInstr], rule: &PeepRule, offset: usize) -> Option<Vec<PeepInstr>> {
    let pat = &rule.pattern.instrs;
    if pat.is_empty() || offset + pat.len() > instrs.len() {
        return None;
    }

    let window = &instrs[offset..offset + pat.len()];

    // Check whether this rule uses the store-load-elim sentinel pattern.
    let is_store_load_sentinel = matches!(
        (pat.first(), pat.get(1)),
        (Some(PeepInstr::Store(s)), Some(PeepInstr::Load(_)))
            if s == "__x__"
    );

    if is_store_load_sentinel {
        // Match any Store(name) followed by Load(same_name).
        if let (Some(PeepInstr::Store(store_name)), Some(PeepInstr::Load(load_name))) =
            (window.first(), window.get(1))
        {
            if store_name == load_name {
                let var_name = store_name.clone();
                let mut new_instrs: Vec<PeepInstr> = instrs[..offset].to_vec();
                for repl_instr in &rule.replacement.instrs {
                    let concrete = match repl_instr {
                        PeepInstr::Store(s) if s == "__x__" => PeepInstr::Store(var_name.clone()),
                        PeepInstr::Load(s) if s == "__x__" => PeepInstr::Load(var_name.clone()),
                        other => other.clone(),
                    };
                    new_instrs.push(concrete);
                }
                new_instrs.extend_from_slice(&instrs[offset + pat.len()..]);
                return Some(new_instrs);
            }
        }
        return None;
    }

    // Normal structural match.
    if window == pat {
        let mut new_instrs: Vec<PeepInstr> = instrs[..offset].to_vec();
        new_instrs.extend_from_slice(&rule.replacement.instrs);
        new_instrs.extend_from_slice(&instrs[offset + pat.len()..]);
        Some(new_instrs)
    } else {
        None
    }
}

// ── Full peephole pass ────────────────────────────────────────────────────────

/// Run the peephole optimiser to a fixed point.
///
/// Rules are tried in priority order (highest first).  A single pass scans the
/// instruction list left-to-right; after any change the scan restarts from the
/// beginning to allow cascading optimisations.  Iteration stops when a full pass
/// produces no change.
pub fn run_peephole(instrs: Vec<PeepInstr>, rules: &[PeepRule]) -> PeepResult {
    // Sort rules by descending priority (stable, so ties keep their order).
    let mut sorted_rules: Vec<&PeepRule> = rules.iter().collect();
    sorted_rules.sort_by_key(|x| std::cmp::Reverse(x.priority));

    let original_count = instrs.len();
    let mut current = instrs;
    let mut rules_applied: Vec<String> = Vec::new();

    loop {
        let mut changed = false;
        'outer: for offset in 0..current.len() {
            for rule in &sorted_rules {
                if let Some(new_instrs) = apply_rule(&current, rule, offset) {
                    rules_applied.push(rule.pattern.name.clone());
                    current = new_instrs;
                    changed = true;
                    break 'outer;
                }
            }
        }
        if !changed {
            break;
        }
    }

    let final_count = current.len();
    let reduction = original_count.saturating_sub(final_count);

    PeepResult {
        instructions: current,
        rules_applied,
        reduction,
    }
}

// ── Utility functions ─────────────────────────────────────────────────────────

/// Count the number of (non-`Nop`) instructions in `instrs`.
pub fn instruction_count(instrs: &[PeepInstr]) -> usize {
    instrs
        .iter()
        .filter(|i| !matches!(i, PeepInstr::Nop))
        .count()
}

/// Convert a single [`PeepInstr`] to a human-readable string.
pub fn peep_instr_to_string(instr: &PeepInstr) -> String {
    match instr {
        PeepInstr::Const(n) => format!("Const({})", n),
        PeepInstr::Add => "Add".to_string(),
        PeepInstr::Sub => "Sub".to_string(),
        PeepInstr::Mul => "Mul".to_string(),
        PeepInstr::Div => "Div".to_string(),
        PeepInstr::Neg => "Neg".to_string(),
        PeepInstr::Load(s) => format!("Load({})", s),
        PeepInstr::Store(s) => format!("Store({})", s),
        PeepInstr::Branch(l) => format!("Branch({})", l),
        PeepInstr::Jump(l) => format!("Jump({})", l),
        PeepInstr::Ret => "Ret".to_string(),
        PeepInstr::Dup => "Dup".to_string(),
        PeepInstr::Pop => "Pop".to_string(),
        PeepInstr::Swap => "Swap".to_string(),
        PeepInstr::Nop => "Nop".to_string(),
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::super::types::PeepInstr;
    use super::*;

    fn const_(n: i64) -> PeepInstr {
        PeepInstr::Const(n)
    }
    fn load(s: &str) -> PeepInstr {
        PeepInstr::Load(s.to_string())
    }
    fn store(s: &str) -> PeepInstr {
        PeepInstr::Store(s.to_string())
    }

    // ── standard_rules ────────────────────────────────────────────────────────

    #[test]
    fn test_standard_rules_non_empty() {
        assert!(!standard_rules().is_empty());
    }

    #[test]
    fn test_standard_rules_sorted_by_priority() {
        let rules = standard_rules();
        let priorities: Vec<i32> = rules.iter().map(|r| r.priority).collect();
        let mut sorted = priorities.clone();
        sorted.sort_by(|a, b| b.cmp(a));
        assert_eq!(priorities, sorted);
    }

    #[test]
    fn test_standard_rules_contains_add_zero() {
        let rules = standard_rules();
        assert!(rules.iter().any(|r| r.pattern.name == "add_zero_pattern"));
    }

    #[test]
    fn test_standard_rules_contains_mul_one() {
        let rules = standard_rules();
        assert!(rules.iter().any(|r| r.pattern.name == "mul_one_pattern"));
    }

    #[test]
    fn test_standard_rules_contains_double_neg() {
        let rules = standard_rules();
        assert!(rules.iter().any(|r| r.pattern.name == "double_neg_pattern"));
    }

    #[test]
    fn test_standard_rules_contains_dead_nop() {
        let rules = standard_rules();
        assert!(rules.iter().any(|r| r.pattern.name == "dead_nop_pattern"));
    }

    #[test]
    fn test_standard_rules_contains_store_load() {
        let rules = standard_rules();
        assert!(rules
            .iter()
            .any(|r| r.pattern.name == "store_load_elim_pattern"));
    }

    // ── apply_rule ────────────────────────────────────────────────────────────

    #[test]
    fn test_apply_rule_add_zero() {
        let rules = standard_rules();
        let add_zero = rules
            .iter()
            .find(|r| r.pattern.name == "add_zero_pattern")
            .expect("add_zero_pattern not found");
        let instrs = vec![const_(0), PeepInstr::Add];
        let result = apply_rule(&instrs, add_zero, 0);
        assert_eq!(result, Some(vec![]));
    }

    #[test]
    fn test_apply_rule_no_match() {
        let rules = standard_rules();
        let add_zero = rules
            .iter()
            .find(|r| r.pattern.name == "add_zero_pattern")
            .expect("add_zero_pattern not found");
        let instrs = vec![const_(5), PeepInstr::Add];
        assert_eq!(apply_rule(&instrs, add_zero, 0), None);
    }

    #[test]
    fn test_apply_rule_at_offset() {
        let rules = standard_rules();
        let dbl_neg = rules
            .iter()
            .find(|r| r.pattern.name == "double_neg_pattern")
            .expect("double_neg_pattern not found");
        let instrs = vec![const_(1), PeepInstr::Neg, PeepInstr::Neg, PeepInstr::Ret];
        let result = apply_rule(&instrs, dbl_neg, 1);
        assert_eq!(result, Some(vec![const_(1), PeepInstr::Ret]));
    }

    #[test]
    fn test_apply_rule_store_load_elim() {
        let rules = standard_rules();
        let rule = rules
            .iter()
            .find(|r| r.pattern.name == "store_load_elim_pattern")
            .expect("store_load_elim_pattern not found");
        let instrs = vec![store("v"), load("v")];
        let result = apply_rule(&instrs, rule, 0);
        assert_eq!(result, Some(vec![PeepInstr::Dup, store("v")]));
    }

    #[test]
    fn test_apply_rule_store_load_different_names_no_match() {
        let rules = standard_rules();
        let rule = rules
            .iter()
            .find(|r| r.pattern.name == "store_load_elim_pattern")
            .expect("store_load_elim_pattern not found");
        let instrs = vec![store("a"), load("b")];
        assert_eq!(apply_rule(&instrs, rule, 0), None);
    }

    #[test]
    fn test_apply_rule_offset_out_of_bounds() {
        let rules = standard_rules();
        let dead_nop = rules
            .iter()
            .find(|r| r.pattern.name == "dead_nop_pattern")
            .expect("dead_nop_pattern not found");
        let instrs = vec![PeepInstr::Nop];
        assert_eq!(apply_rule(&instrs, dead_nop, 5), None);
    }

    // ── run_peephole ──────────────────────────────────────────────────────────

    #[test]
    fn test_run_peephole_removes_add_zero() {
        let rules = standard_rules();
        let instrs = vec![load("x"), const_(0), PeepInstr::Add];
        let result = run_peephole(instrs, &rules);
        assert_eq!(result.instructions, vec![load("x")]);
        assert_eq!(result.reduction, 2);
    }

    #[test]
    fn test_run_peephole_removes_mul_one() {
        let rules = standard_rules();
        let instrs = vec![load("x"), const_(1), PeepInstr::Mul];
        let result = run_peephole(instrs, &rules);
        assert_eq!(result.instructions, vec![load("x")]);
        assert_eq!(result.reduction, 2);
    }

    #[test]
    fn test_run_peephole_removes_double_neg() {
        let rules = standard_rules();
        let instrs = vec![load("x"), PeepInstr::Neg, PeepInstr::Neg];
        let result = run_peephole(instrs, &rules);
        assert_eq!(result.instructions, vec![load("x")]);
        assert_eq!(result.reduction, 2);
    }

    #[test]
    fn test_run_peephole_removes_dup_pop() {
        let rules = standard_rules();
        let instrs = vec![load("x"), PeepInstr::Dup, PeepInstr::Pop];
        let result = run_peephole(instrs, &rules);
        assert_eq!(result.instructions, vec![load("x")]);
        assert_eq!(result.reduction, 2);
    }

    #[test]
    fn test_run_peephole_removes_nop() {
        let rules = standard_rules();
        let instrs = vec![PeepInstr::Nop, load("x"), PeepInstr::Nop, PeepInstr::Ret];
        let result = run_peephole(instrs, &rules);
        assert_eq!(result.instructions, vec![load("x"), PeepInstr::Ret]);
        assert_eq!(result.reduction, 2);
    }

    #[test]
    fn test_run_peephole_store_load_elim() {
        let rules = standard_rules();
        let instrs = vec![const_(5), store("x"), load("x")];
        let result = run_peephole(instrs, &rules);
        assert_eq!(
            result.instructions,
            vec![const_(5), PeepInstr::Dup, store("x")]
        );
    }

    #[test]
    fn test_run_peephole_no_change() {
        let rules = standard_rules();
        let instrs = vec![load("x"), load("y"), PeepInstr::Add, PeepInstr::Ret];
        let result = run_peephole(instrs.clone(), &rules);
        assert_eq!(result.instructions, instrs);
        assert_eq!(result.reduction, 0);
    }

    #[test]
    fn test_run_peephole_cascading() {
        // Const(0) Add Neg Neg  =>  ε ε => (empty after removing both patterns)
        let rules = standard_rules();
        let instrs = vec![
            load("x"),
            const_(0),
            PeepInstr::Add,
            PeepInstr::Neg,
            PeepInstr::Neg,
        ];
        let result = run_peephole(instrs, &rules);
        // Const(0) Add eliminated → load("x") Neg Neg
        // Neg Neg eliminated → load("x")
        assert_eq!(result.instructions, vec![load("x")]);
        assert_eq!(result.reduction, 4);
    }

    #[test]
    fn test_run_peephole_rules_applied_logged() {
        let rules = standard_rules();
        let instrs = vec![load("x"), const_(0), PeepInstr::Add];
        let result = run_peephole(instrs, &rules);
        assert!(!result.rules_applied.is_empty());
        assert!(result
            .rules_applied
            .contains(&"add_zero_pattern".to_string()));
    }

    #[test]
    fn test_run_peephole_empty_input() {
        let rules = standard_rules();
        let result = run_peephole(vec![], &rules);
        assert!(result.instructions.is_empty());
        assert_eq!(result.reduction, 0);
    }

    // ── instruction_count ─────────────────────────────────────────────────────

    #[test]
    fn test_instruction_count_excludes_nop() {
        let instrs = vec![PeepInstr::Nop, load("x"), PeepInstr::Nop, PeepInstr::Ret];
        assert_eq!(instruction_count(&instrs), 2);
    }

    #[test]
    fn test_instruction_count_all_nops() {
        let instrs = vec![PeepInstr::Nop, PeepInstr::Nop];
        assert_eq!(instruction_count(&instrs), 0);
    }

    #[test]
    fn test_instruction_count_empty() {
        assert_eq!(instruction_count(&[]), 0);
    }

    // ── peep_instr_to_string ──────────────────────────────────────────────────

    #[test]
    fn test_peep_instr_to_string_const() {
        assert_eq!(peep_instr_to_string(&const_(42)), "Const(42)");
    }

    #[test]
    fn test_peep_instr_to_string_add() {
        assert_eq!(peep_instr_to_string(&PeepInstr::Add), "Add");
    }

    #[test]
    fn test_peep_instr_to_string_load() {
        assert_eq!(peep_instr_to_string(&load("myVar")), "Load(myVar)");
    }

    #[test]
    fn test_peep_instr_to_string_store() {
        assert_eq!(peep_instr_to_string(&store("myVar")), "Store(myVar)");
    }

    #[test]
    fn test_peep_instr_to_string_nop() {
        assert_eq!(peep_instr_to_string(&PeepInstr::Nop), "Nop");
    }

    #[test]
    fn test_peep_instr_to_string_ret() {
        assert_eq!(peep_instr_to_string(&PeepInstr::Ret), "Ret");
    }

    #[test]
    fn test_peep_instr_to_string_branch() {
        assert_eq!(
            peep_instr_to_string(&PeepInstr::Branch("label".to_string())),
            "Branch(label)"
        );
    }

    #[test]
    fn test_peep_instr_to_string_neg() {
        assert_eq!(peep_instr_to_string(&PeepInstr::Neg), "Neg");
    }
}
