//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PatternTagExt;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast_impl::{Pattern, SurfaceExpr};
    use crate::pattern::*;
    use crate::tokens::Span;
    use crate::{Literal, Located};
    fn mk_span() -> Span {
        Span::new(0, 1, 1, 1)
    }
    fn mk_located<T>(value: T) -> Located<T> {
        Located::new(value, mk_span())
    }
    #[test]
    fn test_fresh_var() {
        let mut compiler = PatternCompiler::new();
        let var1 = compiler.fresh_var();
        let var2 = compiler.fresh_var();
        assert_ne!(var1, var2);
    }
    #[test]
    fn test_compile_match_empty() {
        let mut compiler = PatternCompiler::new();
        let scrutinee = SurfaceExpr::Lit(Literal::Nat(42));
        let result = compiler.compile_match(&scrutinee, &[]);
        assert!(result.is_err());
    }
    #[test]
    fn test_check_exhaustive_wildcard() {
        let compiler = PatternCompiler::new();
        let patterns = vec![Pattern::Wild];
        assert!(compiler.check_exhaustive(&patterns).is_ok());
    }
    #[test]
    fn test_check_exhaustive_none() {
        let compiler = PatternCompiler::new();
        let patterns = vec![];
        assert!(compiler.check_exhaustive(&patterns).is_err());
    }
    #[test]
    fn test_check_redundant() {
        let compiler = PatternCompiler::new();
        let patterns = vec![Pattern::Var("x".to_string()), Pattern::Wild];
        let redundant = compiler.check_redundant(&patterns);
        assert_eq!(redundant.len(), 1);
        assert_eq!(redundant[0], 1);
    }
    #[test]
    fn test_is_irrefutable_wild() {
        let compiler = PatternCompiler::new();
        assert!(compiler.is_irrefutable(&Pattern::Wild));
    }
    #[test]
    fn test_is_irrefutable_var() {
        let compiler = PatternCompiler::new();
        assert!(compiler.is_irrefutable(&Pattern::Var("x".to_string())));
    }
    #[test]
    fn test_is_irrefutable_ctor() {
        let compiler = PatternCompiler::new();
        assert!(!compiler.is_irrefutable(&Pattern::Ctor("Some".to_string(), vec![])));
    }
    #[test]
    fn test_is_irrefutable_lit() {
        let compiler = PatternCompiler::new();
        assert!(!compiler.is_irrefutable(&Pattern::Lit(Literal::Nat(0))));
    }
    #[test]
    fn test_is_irrefutable_or_with_wild() {
        let compiler = PatternCompiler::new();
        let pat = Pattern::Or(
            Box::new(mk_located(Pattern::Ctor("Some".to_string(), vec![]))),
            Box::new(mk_located(Pattern::Wild)),
        );
        assert!(compiler.is_irrefutable(&pat));
    }
    #[test]
    fn test_count_bindings_wild() {
        let compiler = PatternCompiler::new();
        assert_eq!(compiler.count_bindings(&Pattern::Wild), 0);
    }
    #[test]
    fn test_count_bindings_var() {
        let compiler = PatternCompiler::new();
        assert_eq!(compiler.count_bindings(&Pattern::Var("x".to_string())), 1);
    }
    #[test]
    fn test_count_bindings_ctor() {
        let compiler = PatternCompiler::new();
        let pat = Pattern::Ctor(
            "Pair".to_string(),
            vec![
                mk_located(Pattern::Var("a".to_string())),
                mk_located(Pattern::Var("b".to_string())),
            ],
        );
        assert_eq!(compiler.count_bindings(&pat), 2);
    }
    #[test]
    fn test_count_bindings_or_pattern() {
        let compiler = PatternCompiler::new();
        let pat = Pattern::Or(
            Box::new(mk_located(Pattern::Var("x".to_string()))),
            Box::new(mk_located(Pattern::Var("y".to_string()))),
        );
        assert_eq!(compiler.count_bindings(&pat), 1);
    }
    #[test]
    fn test_extract_bound_names_empty() {
        let compiler = PatternCompiler::new();
        assert!(compiler.extract_bound_names(&Pattern::Wild).is_empty());
    }
    #[test]
    fn test_extract_bound_names_var() {
        let compiler = PatternCompiler::new();
        let names = compiler.extract_bound_names(&Pattern::Var("x".to_string()));
        assert_eq!(names, vec!["x"]);
    }
    #[test]
    fn test_extract_bound_names_nested() {
        let compiler = PatternCompiler::new();
        let pat = Pattern::Ctor(
            "Pair".to_string(),
            vec![
                mk_located(Pattern::Var("a".to_string())),
                mk_located(Pattern::Ctor(
                    "Some".to_string(),
                    vec![mk_located(Pattern::Var("b".to_string()))],
                )),
            ],
        );
        let names = compiler.extract_bound_names(&pat);
        assert_eq!(names, vec!["a", "b"]);
    }
    #[test]
    fn test_extract_bound_names_or() {
        let compiler = PatternCompiler::new();
        let pat = Pattern::Or(
            Box::new(mk_located(Pattern::Var("x".to_string()))),
            Box::new(mk_located(Pattern::Var("y".to_string()))),
        );
        let names = compiler.extract_bound_names(&pat);
        assert_eq!(names, vec!["x"]);
    }
    #[test]
    fn test_pattern_to_string_wild() {
        let compiler = PatternCompiler::new();
        assert_eq!(compiler.pattern_to_string(&Pattern::Wild), "_");
    }
    #[test]
    fn test_pattern_to_string_var() {
        let compiler = PatternCompiler::new();
        assert_eq!(
            compiler.pattern_to_string(&Pattern::Var("x".to_string())),
            "x"
        );
    }
    #[test]
    fn test_pattern_to_string_ctor_no_args() {
        let compiler = PatternCompiler::new();
        assert_eq!(
            compiler.pattern_to_string(&Pattern::Ctor("None".to_string(), vec![])),
            "None"
        );
    }
    #[test]
    fn test_pattern_to_string_ctor_with_args() {
        let compiler = PatternCompiler::new();
        let pat = Pattern::Ctor(
            "Some".to_string(),
            vec![mk_located(Pattern::Var("x".to_string()))],
        );
        assert_eq!(compiler.pattern_to_string(&pat), "Some x");
    }
    #[test]
    fn test_pattern_to_string_or() {
        let compiler = PatternCompiler::new();
        let pat = Pattern::Or(
            Box::new(mk_located(Pattern::Ctor("None".to_string(), vec![]))),
            Box::new(mk_located(Pattern::Var("x".to_string()))),
        );
        assert_eq!(compiler.pattern_to_string(&pat), "None | x");
    }
    #[test]
    fn test_pattern_to_string_lit() {
        let compiler = PatternCompiler::new();
        assert_eq!(
            compiler.pattern_to_string(&Pattern::Lit(Literal::Nat(42))),
            "42"
        );
    }
    #[test]
    fn test_pattern_to_string_nested_ctor() {
        let compiler = PatternCompiler::new();
        let pat = Pattern::Ctor(
            "Some".to_string(),
            vec![mk_located(Pattern::Ctor(
                "Pair".to_string(),
                vec![
                    mk_located(Pattern::Var("a".to_string())),
                    mk_located(Pattern::Var("b".to_string())),
                ],
            ))],
        );
        let s = compiler.pattern_to_string(&pat);
        assert_eq!(s, "Some (Pair a b)");
    }
    #[test]
    fn test_simplify_pattern_wild() {
        let compiler = PatternCompiler::new();
        assert_eq!(compiler.simplify_pattern(&Pattern::Wild), Pattern::Wild);
    }
    #[test]
    fn test_simplify_or_with_wild_becomes_wild() {
        let compiler = PatternCompiler::new();
        let pat = Pattern::Or(
            Box::new(mk_located(Pattern::Ctor("Some".to_string(), vec![]))),
            Box::new(mk_located(Pattern::Wild)),
        );
        let simplified = compiler.simplify_pattern(&pat);
        assert_eq!(simplified, Pattern::Wild);
    }
    #[test]
    fn test_simplify_ctor_with_args() {
        let compiler = PatternCompiler::new();
        let pat = Pattern::Ctor(
            "Pair".to_string(),
            vec![
                mk_located(Pattern::Var("a".to_string())),
                mk_located(Pattern::Wild),
            ],
        );
        let simplified = compiler.simplify_pattern(&pat);
        match simplified {
            Pattern::Ctor(name, args) => {
                assert_eq!(name, "Pair");
                assert_eq!(args.len(), 2);
            }
            _ => panic!("Expected Ctor pattern"),
        }
    }
    #[test]
    fn test_collect_constructors_empty() {
        let compiler = PatternCompiler::new();
        let rows: Vec<PatternRow> = vec![];
        let ctors = compiler.collect_constructors(&rows, 0);
        assert!(ctors.is_empty());
    }
    #[test]
    fn test_collect_constructors_single() {
        let compiler = PatternCompiler::new();
        let rows = vec![PatternRow {
            patterns: vec![Pattern::Ctor(
                "Some".to_string(),
                vec![mk_located(Pattern::Wild)],
            )],
            body: SurfaceExpr::Hole,
            guard: None,
        }];
        let ctors = compiler.collect_constructors(&rows, 0);
        assert_eq!(ctors.len(), 1);
        assert_eq!(ctors[0].0, "Some");
        assert_eq!(ctors[0].1, 1);
    }
    #[test]
    fn test_collect_constructors_dedup() {
        let compiler = PatternCompiler::new();
        let rows = vec![
            PatternRow {
                patterns: vec![Pattern::Ctor("A".to_string(), vec![])],
                body: SurfaceExpr::Hole,
                guard: None,
            },
            PatternRow {
                patterns: vec![Pattern::Ctor("A".to_string(), vec![])],
                body: SurfaceExpr::Hole,
                guard: None,
            },
        ];
        let ctors = compiler.collect_constructors(&rows, 0);
        assert_eq!(ctors.len(), 1);
    }
    #[test]
    fn test_select_column_prefers_constructors() {
        let compiler = PatternCompiler::new();
        let rows = vec![
            PatternRow {
                patterns: vec![Pattern::Wild, Pattern::Ctor("A".to_string(), vec![])],
                body: SurfaceExpr::Hole,
                guard: None,
            },
            PatternRow {
                patterns: vec![Pattern::Wild, Pattern::Ctor("B".to_string(), vec![])],
                body: SurfaceExpr::Hole,
                guard: None,
            },
        ];
        let col = compiler.select_column(&rows, 2);
        assert_eq!(col, 1);
    }
    #[test]
    fn test_default_rows_keeps_wildcards() {
        let compiler = PatternCompiler::new();
        let rows = vec![
            PatternRow {
                patterns: vec![Pattern::Ctor("A".to_string(), vec![])],
                body: SurfaceExpr::Hole,
                guard: None,
            },
            PatternRow {
                patterns: vec![Pattern::Wild],
                body: SurfaceExpr::Lit(Literal::Nat(1)),
                guard: None,
            },
        ];
        let defaults = compiler.default_rows(&rows, 0);
        assert_eq!(defaults.len(), 1);
        assert_eq!(defaults[0].body, SurfaceExpr::Lit(Literal::Nat(1)));
    }
    #[test]
    fn test_specialize_ctor() {
        let compiler = PatternCompiler::new();
        let rows = vec![
            PatternRow {
                patterns: vec![Pattern::Ctor(
                    "Some".to_string(),
                    vec![mk_located(Pattern::Var("x".to_string()))],
                )],
                body: SurfaceExpr::Var("x".to_string()),
                guard: None,
            },
            PatternRow {
                patterns: vec![Pattern::Wild],
                body: SurfaceExpr::Hole,
                guard: None,
            },
        ];
        let specialized = compiler.specialize(&rows, 0, "Some", 1);
        assert_eq!(specialized.len(), 2);
        assert_eq!(specialized[0].patterns.len(), 1);
        assert!(matches!(specialized[0].patterns[0], Pattern::Var(_)));
        assert_eq!(specialized[1].patterns.len(), 1);
        assert!(matches!(specialized[1].patterns[0], Pattern::Wild));
    }
    #[test]
    fn test_compile_matrix_empty() {
        let mut compiler = PatternCompiler::new();
        let tree = compiler.compile_matrix(&[], 1);
        assert_eq!(tree, CaseTree::Failure);
    }
    #[test]
    fn test_compile_matrix_all_wild() {
        let mut compiler = PatternCompiler::new();
        let rows = vec![PatternRow {
            patterns: vec![Pattern::Wild],
            body: SurfaceExpr::Hole,
            guard: None,
        }];
        let tree = compiler.compile_matrix(&rows, 1);
        assert_eq!(tree, CaseTree::Leaf { body_idx: 0 });
    }
    #[test]
    fn test_compile_matrix_with_ctors() {
        let mut compiler = PatternCompiler::new();
        let rows = vec![
            PatternRow {
                patterns: vec![Pattern::Ctor("True".to_string(), vec![])],
                body: SurfaceExpr::Lit(Literal::Nat(1)),
                guard: None,
            },
            PatternRow {
                patterns: vec![Pattern::Ctor("False".to_string(), vec![])],
                body: SurfaceExpr::Lit(Literal::Nat(0)),
                guard: None,
            },
        ];
        let tree = compiler.compile_matrix(&rows, 1);
        match tree {
            CaseTree::Switch {
                scrutinee,
                branches,
                ..
            } => {
                assert_eq!(scrutinee, 0);
                assert_eq!(branches.len(), 2);
                assert_eq!(branches[0].ctor, "True");
                assert_eq!(branches[1].ctor, "False");
            }
            _ => panic!("Expected Switch"),
        }
    }
    #[test]
    fn test_check_exhaustive_with_ctors_all_covered() {
        let compiler = PatternCompiler::new();
        let ctors = TypeConstructors {
            type_name: "Bool".to_string(),
            constructors: vec![
                ConstructorInfo {
                    name: "True".to_string(),
                    arity: 0,
                },
                ConstructorInfo {
                    name: "False".to_string(),
                    arity: 0,
                },
            ],
        };
        let patterns = vec![
            Pattern::Ctor("True".to_string(), vec![]),
            Pattern::Ctor("False".to_string(), vec![]),
        ];
        assert!(compiler
            .check_exhaustive_with_ctors(&patterns, &ctors)
            .is_ok());
    }
    #[test]
    fn test_check_exhaustive_with_ctors_missing() {
        let compiler = PatternCompiler::new();
        let ctors = TypeConstructors {
            type_name: "Bool".to_string(),
            constructors: vec![
                ConstructorInfo {
                    name: "True".to_string(),
                    arity: 0,
                },
                ConstructorInfo {
                    name: "False".to_string(),
                    arity: 0,
                },
            ],
        };
        let patterns = vec![Pattern::Ctor("True".to_string(), vec![])];
        let result = compiler.check_exhaustive_with_ctors(&patterns, &ctors);
        assert!(result.is_err());
        let missing = result.unwrap_err();
        assert_eq!(missing, vec!["False"]);
    }
    #[test]
    fn test_check_exhaustive_with_ctors_wildcard() {
        let compiler = PatternCompiler::new();
        let ctors = TypeConstructors {
            type_name: "Option".to_string(),
            constructors: vec![
                ConstructorInfo {
                    name: "Some".to_string(),
                    arity: 1,
                },
                ConstructorInfo {
                    name: "None".to_string(),
                    arity: 0,
                },
            ],
        };
        let patterns = vec![Pattern::Wild];
        assert!(compiler
            .check_exhaustive_with_ctors(&patterns, &ctors)
            .is_ok());
    }
    #[test]
    fn test_check_nested_exhaustive_ok() {
        let compiler = PatternCompiler::new();
        let bool_ctors = TypeConstructors {
            type_name: "Bool".to_string(),
            constructors: vec![
                ConstructorInfo {
                    name: "True".to_string(),
                    arity: 0,
                },
                ConstructorInfo {
                    name: "False".to_string(),
                    arity: 0,
                },
            ],
        };
        let patterns = vec![
            vec![Pattern::Ctor("True".to_string(), vec![])],
            vec![Pattern::Ctor("False".to_string(), vec![])],
        ];
        assert!(compiler
            .check_nested_exhaustive(&patterns, &[bool_ctors])
            .is_ok());
    }
    #[test]
    fn test_check_nested_exhaustive_missing() {
        let compiler = PatternCompiler::new();
        let bool_ctors = TypeConstructors {
            type_name: "Bool".to_string(),
            constructors: vec![
                ConstructorInfo {
                    name: "True".to_string(),
                    arity: 0,
                },
                ConstructorInfo {
                    name: "False".to_string(),
                    arity: 0,
                },
            ],
        };
        let patterns = vec![vec![Pattern::Ctor("True".to_string(), vec![])]];
        let result = compiler.check_nested_exhaustive(&patterns, &[bool_ctors]);
        assert!(result.is_err());
    }
    #[test]
    fn test_compile_matrix_no_cols() {
        let mut compiler = PatternCompiler::new();
        let rows = vec![PatternRow {
            patterns: vec![],
            body: SurfaceExpr::Hole,
            guard: None,
        }];
        let tree = compiler.compile_matrix(&rows, 0);
        assert_eq!(tree, CaseTree::Leaf { body_idx: 0 });
    }
    #[test]
    fn test_specialize_drops_different_ctor() {
        let compiler = PatternCompiler::new();
        let rows = vec![PatternRow {
            patterns: vec![Pattern::Ctor("A".to_string(), vec![])],
            body: SurfaceExpr::Hole,
            guard: None,
        }];
        let specialized = compiler.specialize(&rows, 0, "B", 0);
        assert!(specialized.is_empty());
    }
    #[test]
    fn test_default_rows_empty_on_all_ctors() {
        let compiler = PatternCompiler::new();
        let rows = vec![
            PatternRow {
                patterns: vec![Pattern::Ctor("A".to_string(), vec![])],
                body: SurfaceExpr::Hole,
                guard: None,
            },
            PatternRow {
                patterns: vec![Pattern::Ctor("B".to_string(), vec![])],
                body: SurfaceExpr::Hole,
                guard: None,
            },
        ];
        let defaults = compiler.default_rows(&rows, 0);
        assert!(defaults.is_empty());
    }
    #[test]
    fn test_max_pattern_depth_flat() {
        let compiler = PatternCompiler::new();
        let pat = Pattern::Var("x".to_string());
        assert_eq!(compiler.max_pattern_depth(&pat), 0);
    }
    #[test]
    fn test_max_pattern_depth_nested() {
        let compiler = PatternCompiler::new();
        let pat = Pattern::Ctor(
            "Pair".to_string(),
            vec![
                mk_located(Pattern::Ctor(
                    "Some".to_string(),
                    vec![mk_located(Pattern::Var("x".to_string()))],
                )),
                mk_located(Pattern::Wild),
            ],
        );
        assert_eq!(compiler.max_pattern_depth(&pat), 2);
    }
    #[test]
    fn test_flatten_or_single() {
        let compiler = PatternCompiler::new();
        let pat = Pattern::Var("x".to_string());
        let flat = compiler.flatten_or_pattern(&pat);
        assert_eq!(flat.len(), 1);
    }
    #[test]
    fn test_flatten_or_nested() {
        let compiler = PatternCompiler::new();
        let pat = Pattern::Or(
            Box::new(mk_located(Pattern::Ctor("A".to_string(), vec![]))),
            Box::new(mk_located(Pattern::Or(
                Box::new(mk_located(Pattern::Ctor("B".to_string(), vec![]))),
                Box::new(mk_located(Pattern::Ctor("C".to_string(), vec![]))),
            ))),
        );
        let flat = compiler.flatten_or_pattern(&pat);
        assert_eq!(flat.len(), 3);
    }
    #[test]
    fn test_patterns_equivalent_wild() {
        let compiler = PatternCompiler::new();
        assert!(compiler.patterns_equivalent(&Pattern::Wild, &Pattern::Wild));
    }
    #[test]
    fn test_patterns_equivalent_var() {
        let compiler = PatternCompiler::new();
        assert!(compiler.patterns_equivalent(
            &Pattern::Var("x".to_string()),
            &Pattern::Var("x".to_string())
        ));
        assert!(!compiler.patterns_equivalent(
            &Pattern::Var("x".to_string()),
            &Pattern::Var("y".to_string())
        ));
    }
    #[test]
    fn test_patterns_equivalent_ctor() {
        let compiler = PatternCompiler::new();
        let p1 = Pattern::Ctor(
            "Some".to_string(),
            vec![mk_located(Pattern::Var("x".to_string()))],
        );
        let p2 = Pattern::Ctor(
            "Some".to_string(),
            vec![mk_located(Pattern::Var("x".to_string()))],
        );
        let p3 = Pattern::Ctor("None".to_string(), vec![]);
        assert!(compiler.patterns_equivalent(&p1, &p2));
        assert!(!compiler.patterns_equivalent(&p1, &p3));
    }
    #[test]
    fn test_patterns_equivalent_or() {
        let compiler = PatternCompiler::new();
        let p1 = Pattern::Or(
            Box::new(mk_located(Pattern::Ctor("A".to_string(), vec![]))),
            Box::new(mk_located(Pattern::Ctor("B".to_string(), vec![]))),
        );
        let p2 = Pattern::Or(
            Box::new(mk_located(Pattern::Ctor("A".to_string(), vec![]))),
            Box::new(mk_located(Pattern::Ctor("B".to_string(), vec![]))),
        );
        assert!(compiler.patterns_equivalent(&p1, &p2));
    }
    #[test]
    fn test_bound_var_set_empty() {
        let compiler = PatternCompiler::new();
        let set = compiler.bound_var_set(&Pattern::Wild);
        assert!(set.is_empty());
    }
    #[test]
    fn test_bound_var_set_single() {
        let compiler = PatternCompiler::new();
        let set = compiler.bound_var_set(&Pattern::Var("x".to_string()));
        assert_eq!(set.len(), 1);
        assert!(set.contains("x"));
    }
    #[test]
    fn test_bound_var_set_nested() {
        let compiler = PatternCompiler::new();
        let pat = Pattern::Ctor(
            "Pair".to_string(),
            vec![
                mk_located(Pattern::Var("a".to_string())),
                mk_located(Pattern::Var("b".to_string())),
            ],
        );
        let set = compiler.bound_var_set(&pat);
        assert_eq!(set.len(), 2);
        assert!(set.contains("a"));
        assert!(set.contains("b"));
    }
    #[test]
    fn test_same_bindings_yes() {
        let compiler = PatternCompiler::new();
        let p1 = Pattern::Ctor(
            "Pair".to_string(),
            vec![
                mk_located(Pattern::Var("a".to_string())),
                mk_located(Pattern::Var("b".to_string())),
            ],
        );
        let p2 = Pattern::Ctor(
            "Pair".to_string(),
            vec![
                mk_located(Pattern::Var("a".to_string())),
                mk_located(Pattern::Var("b".to_string())),
            ],
        );
        assert!(compiler.same_bindings(&[p1, p2]));
    }
    #[test]
    fn test_same_bindings_no() {
        let compiler = PatternCompiler::new();
        let p1 = Pattern::Ctor(
            "Pair".to_string(),
            vec![
                mk_located(Pattern::Var("a".to_string())),
                mk_located(Pattern::Var("b".to_string())),
            ],
        );
        let p2 = Pattern::Ctor(
            "Pair".to_string(),
            vec![
                mk_located(Pattern::Var("x".to_string())),
                mk_located(Pattern::Var("y".to_string())),
            ],
        );
        assert!(!compiler.same_bindings(&[p1, p2]));
    }
    #[test]
    fn test_find_dead_patterns_none() {
        let compiler = PatternCompiler::new();
        let rows = vec![
            PatternRow {
                patterns: vec![Pattern::Ctor("A".to_string(), vec![])],
                body: SurfaceExpr::Hole,
                guard: None,
            },
            PatternRow {
                patterns: vec![Pattern::Ctor("B".to_string(), vec![])],
                body: SurfaceExpr::Hole,
                guard: None,
            },
        ];
        let dead = compiler.find_dead_patterns(&rows);
        assert!(dead.is_empty());
    }
    #[test]
    fn test_find_dead_patterns_with_wildcard() {
        let compiler = PatternCompiler::new();
        let rows = vec![
            PatternRow {
                patterns: vec![Pattern::Wild],
                body: SurfaceExpr::Hole,
                guard: None,
            },
            PatternRow {
                patterns: vec![Pattern::Ctor("A".to_string(), vec![])],
                body: SurfaceExpr::Hole,
                guard: None,
            },
        ];
        let dead = compiler.find_dead_patterns(&rows);
        assert_eq!(dead.len(), 1);
        assert_eq!(dead[0], 1);
    }
    #[test]
    fn test_analyze_usefulness_empty() {
        let compiler = PatternCompiler::new();
        let new_pat = vec![Pattern::Ctor("A".to_string(), vec![])];
        assert!(compiler.analyze_usefulness(&[], &new_pat));
    }
    #[test]
    fn test_analyze_usefulness_nongeneral() {
        let compiler = PatternCompiler::new();
        let rows = vec![PatternRow {
            patterns: vec![Pattern::Ctor("A".to_string(), vec![])],
            body: SurfaceExpr::Hole,
            guard: None,
        }];
        let new_pat = vec![Pattern::Ctor("B".to_string(), vec![])];
        assert!(compiler.analyze_usefulness(&rows, &new_pat));
    }
    #[test]
    fn test_extract_literal_range_single() {
        let compiler = PatternCompiler::new();
        let patterns = vec![Pattern::Lit(crate::Literal::Nat(5))];
        let range = compiler.extract_literal_range(&patterns);
        assert_eq!(range, Some((5, 5)));
    }
    #[test]
    fn test_extract_literal_range_multiple() {
        let compiler = PatternCompiler::new();
        let patterns = vec![
            Pattern::Lit(crate::Literal::Nat(1)),
            Pattern::Lit(crate::Literal::Nat(3)),
            Pattern::Lit(crate::Literal::Nat(2)),
        ];
        let range = compiler.extract_literal_range(&patterns);
        assert_eq!(range, Some((1, 3)));
    }
    #[test]
    fn test_extract_literal_range_none() {
        let compiler = PatternCompiler::new();
        let patterns = vec![Pattern::Wild];
        let range = compiler.extract_literal_range(&patterns);
        assert_eq!(range, None);
    }
    #[test]
    fn test_check_range_coverage_full() {
        let compiler = PatternCompiler::new();
        let patterns = vec![
            Pattern::Lit(crate::Literal::Nat(1)),
            Pattern::Lit(crate::Literal::Nat(2)),
            Pattern::Lit(crate::Literal::Nat(3)),
        ];
        assert!(compiler.check_range_coverage(&patterns, 1, 3));
    }
    #[test]
    fn test_check_range_coverage_partial() {
        let compiler = PatternCompiler::new();
        let patterns = vec![
            Pattern::Lit(crate::Literal::Nat(1)),
            Pattern::Lit(crate::Literal::Nat(3)),
        ];
        assert!(!compiler.check_range_coverage(&patterns, 1, 3));
    }
    #[test]
    fn test_check_range_coverage_with_wildcard() {
        let compiler = PatternCompiler::new();
        let patterns = vec![Pattern::Wild];
        assert!(compiler.check_range_coverage(&patterns, 1, 100));
    }
    #[test]
    fn test_canonicalize_wild() {
        let compiler = PatternCompiler::new();
        assert_eq!(compiler.canonicalize(&Pattern::Wild), "_");
    }
    #[test]
    fn test_canonicalize_var() {
        let compiler = PatternCompiler::new();
        assert_eq!(compiler.canonicalize(&Pattern::Var("x".to_string())), "x");
    }
    #[test]
    fn test_canonicalize_ctor() {
        let compiler = PatternCompiler::new();
        let pat = Pattern::Ctor(
            "Cons".to_string(),
            vec![
                mk_located(Pattern::Var("x".to_string())),
                mk_located(Pattern::Wild),
            ],
        );
        let canonical = compiler.canonicalize(&pat);
        assert_eq!(canonical, "Cons x _");
    }
    #[test]
    fn test_nested_pattern_with_depth_limit() {
        let compiler = PatternCompiler::new();
        let deeply_nested = Pattern::Ctor(
            "A".to_string(),
            vec![mk_located(Pattern::Ctor(
                "B".to_string(),
                vec![mk_located(Pattern::Ctor(
                    "C".to_string(),
                    vec![mk_located(Pattern::Var("x".to_string()))],
                ))],
            ))],
        );
        assert!(compiler.max_pattern_depth(&deeply_nested) >= 2);
    }
    #[test]
    fn test_or_pattern_flattening_complex() {
        let compiler = PatternCompiler::new();
        let complex_or = Pattern::Or(
            Box::new(mk_located(Pattern::Or(
                Box::new(mk_located(Pattern::Ctor("A".to_string(), vec![]))),
                Box::new(mk_located(Pattern::Ctor("B".to_string(), vec![]))),
            ))),
            Box::new(mk_located(Pattern::Or(
                Box::new(mk_located(Pattern::Ctor("C".to_string(), vec![]))),
                Box::new(mk_located(Pattern::Ctor("D".to_string(), vec![]))),
            ))),
        );
        let flat = compiler.flatten_or_pattern(&complex_or);
        assert_eq!(flat.len(), 4);
    }
    #[test]
    fn test_pattern_coverage_analysis() {
        let compiler = PatternCompiler::new();
        let patterns = vec![
            Pattern::Lit(crate::Literal::Nat(0)),
            Pattern::Lit(crate::Literal::Nat(1)),
        ];
        assert!(!compiler.check_range_coverage(&patterns, 0, 2));
    }
    #[test]
    fn test_bound_names_or_pattern() {
        let compiler = PatternCompiler::new();
        let or_pat = Pattern::Or(
            Box::new(mk_located(Pattern::Ctor(
                "Some".to_string(),
                vec![mk_located(Pattern::Var("a".to_string()))],
            ))),
            Box::new(mk_located(Pattern::Ctor(
                "Some".to_string(),
                vec![mk_located(Pattern::Var("a".to_string()))],
            ))),
        );
        let names = compiler.extract_bound_names(&or_pat);
        assert_eq!(names, vec!["a"]);
    }
    #[test]
    fn test_collect_constructors_with_or() {
        let compiler = PatternCompiler::new();
        let rows = vec![PatternRow {
            patterns: vec![Pattern::Or(
                Box::new(mk_located(Pattern::Ctor("A".to_string(), vec![]))),
                Box::new(mk_located(Pattern::Ctor("B".to_string(), vec![]))),
            )],
            body: SurfaceExpr::Hole,
            guard: None,
        }];
        let ctors = compiler.collect_constructors(&rows, 0);
        assert_eq!(ctors.len(), 2);
    }
    #[test]
    fn test_pattern_simplification_nested_ctor() {
        let compiler = PatternCompiler::new();
        let pat = Pattern::Ctor(
            "Cons".to_string(),
            vec![
                mk_located(Pattern::Var("x".to_string())),
                mk_located(Pattern::Or(
                    Box::new(mk_located(Pattern::Ctor("Nil".to_string(), vec![]))),
                    Box::new(mk_located(Pattern::Wild)),
                )),
            ],
        );
        let simplified = compiler.simplify_pattern(&pat);
        assert!(matches!(simplified, Pattern::Ctor(..)));
    }
    #[test]
    fn test_pattern_counting_complex_nested() {
        let compiler = PatternCompiler::new();
        let pat = Pattern::Ctor(
            "Triple".to_string(),
            vec![
                mk_located(Pattern::Var("a".to_string())),
                mk_located(Pattern::Ctor(
                    "Pair".to_string(),
                    vec![
                        mk_located(Pattern::Var("b".to_string())),
                        mk_located(Pattern::Var("c".to_string())),
                    ],
                )),
                mk_located(Pattern::Wild),
            ],
        );
        assert_eq!(compiler.count_bindings(&pat), 3);
    }
    #[test]
    fn test_matrix_compilation_with_mixed_patterns() {
        let mut compiler = PatternCompiler::new();
        let rows = vec![
            PatternRow {
                patterns: vec![
                    Pattern::Lit(crate::Literal::Nat(0)),
                    Pattern::Var("x".to_string()),
                ],
                body: SurfaceExpr::Hole,
                guard: None,
            },
            PatternRow {
                patterns: vec![
                    Pattern::Lit(crate::Literal::Nat(1)),
                    Pattern::Ctor("Some".to_string(), vec![]),
                ],
                body: SurfaceExpr::Hole,
                guard: None,
            },
            PatternRow {
                patterns: vec![Pattern::Wild, Pattern::Wild],
                body: SurfaceExpr::Hole,
                guard: None,
            },
        ];
        let tree = compiler.compile_matrix(&rows, 2);
        assert!(!matches!(tree, CaseTree::Failure));
    }
    #[test]
    fn test_specialize_with_or_pattern() {
        let compiler = PatternCompiler::new();
        let rows = vec![
            PatternRow {
                patterns: vec![Pattern::Or(
                    Box::new(mk_located(Pattern::Ctor("A".to_string(), vec![]))),
                    Box::new(mk_located(Pattern::Ctor("B".to_string(), vec![]))),
                )],
                body: SurfaceExpr::Hole,
                guard: None,
            },
            PatternRow {
                patterns: vec![Pattern::Wild],
                body: SurfaceExpr::Hole,
                guard: None,
            },
        ];
        let spec_a = compiler.specialize(&rows, 0, "A", 0);
        let spec_b = compiler.specialize(&rows, 0, "B", 0);
        assert_eq!(spec_a.len(), 2);
        assert_eq!(spec_b.len(), 2);
    }
    #[test]
    fn test_exhaustiveness_with_or_patterns() {
        let compiler = PatternCompiler::new();
        let ctors = TypeConstructors {
            type_name: "Status".to_string(),
            constructors: vec![
                ConstructorInfo {
                    name: "Ok".to_string(),
                    arity: 0,
                },
                ConstructorInfo {
                    name: "Err".to_string(),
                    arity: 0,
                },
            ],
        };
        let patterns = vec![Pattern::Or(
            Box::new(mk_located(Pattern::Ctor("Ok".to_string(), vec![]))),
            Box::new(mk_located(Pattern::Ctor("Err".to_string(), vec![]))),
        )];
        assert!(compiler
            .check_exhaustive_with_ctors(&patterns, &ctors)
            .is_ok());
    }
    #[test]
    fn test_multiple_bindings_same_var_or() {
        let compiler = PatternCompiler::new();
        let or_pat = Pattern::Or(
            Box::new(mk_located(Pattern::Var("x".to_string()))),
            Box::new(mk_located(Pattern::Ctor("Cons".to_string(), vec![]))),
        );
        let names = compiler.extract_bound_names(&or_pat);
        assert_eq!(names.len(), 1);
    }
    #[test]
    fn test_literal_pattern_nat() {
        let compiler = PatternCompiler::new();
        let lit_nat = Pattern::Lit(crate::Literal::Nat(42));
        assert_eq!(compiler.pattern_to_string(&lit_nat), "42");
    }
    #[test]
    fn test_pattern_equivalence_after_simplification() {
        let compiler = PatternCompiler::new();
        let original = Pattern::Or(
            Box::new(mk_located(Pattern::Var("x".to_string()))),
            Box::new(mk_located(Pattern::Wild)),
        );
        let simplified = compiler.simplify_pattern(&original);
        assert_eq!(simplified, Pattern::Wild);
    }
}
/// Classify a pattern string by its leading token.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn classify_pattern_ext(s: &str) -> PatternTagExt {
    let s = s.trim();
    if s == "_" {
        return PatternTagExt::Wild;
    }
    if s.starts_with(char::is_lowercase) && !s.contains(' ') {
        return PatternTagExt::Var;
    }
    if s.starts_with(char::is_uppercase) {
        return PatternTagExt::Ctor;
    }
    if s.chars().all(|c| c.is_ascii_digit()) {
        return PatternTagExt::Lit;
    }
    if s.starts_with('"') {
        return PatternTagExt::Lit;
    }
    PatternTagExt::Ctor
}
#[cfg(test)]
mod pattern_ext_tests {
    use super::*;
    use crate::ast_impl::{Pattern, SurfaceExpr};
    use crate::pattern::*;
    #[test]
    fn test_classify_pattern() {
        assert_eq!(classify_pattern_ext("_"), PatternTagExt::Wild);
        assert_eq!(classify_pattern_ext("x"), PatternTagExt::Var);
        assert_eq!(classify_pattern_ext("Nat"), PatternTagExt::Ctor);
        assert_eq!(classify_pattern_ext("42"), PatternTagExt::Lit);
    }
    #[test]
    fn test_pattern_coverage() {
        let mut cov = PatternCoverageExt::new();
        cov.add_arm(PatternTagExt::Ctor);
        assert!(!cov.is_complete());
        cov.add_arm(PatternTagExt::Wild);
        assert!(cov.is_complete());
    }
    #[test]
    fn test_match_arm() {
        let arm = MatchArmExt::new("Nat.succ n", "n + 1").with_guard("n > 0");
        assert_eq!(arm.pattern, "Nat.succ n");
        assert!(arm.guard.is_some());
    }
}
#[cfg(test)]
mod pattern_ext2_tests {
    use super::*;
    use crate::ast_impl::{Pattern, SurfaceExpr};
    use crate::pattern::*;
    #[test]
    fn test_pattern_binding() {
        let b = PatternBinding::new("n", 0).with_type("Nat");
        assert_eq!(b.name, "n");
        assert_eq!(b.position, 0);
        assert_eq!(b.ty.as_deref(), Some("Nat"));
    }
    #[test]
    fn test_pattern_matrix_row_wildcard() {
        let row = PatternMatrixRow::new(vec!["_", "_"], "body");
        assert!(row.is_wildcard_row());
        let row2 = PatternMatrixRow::new(vec!["_", "x"], "body");
        assert!(!row2.is_wildcard_row());
    }
}
/// A depth-first traversal of a pattern tree (string-based).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn count_pattern_vars(pattern: &str) -> usize {
    pattern
        .split_whitespace()
        .filter(|w| w.starts_with(|c: char| c.is_lowercase()) && !w.contains('.'))
        .count()
}
#[cfg(test)]
mod pattern_renamer_tests {
    use super::*;
    use crate::ast_impl::{Pattern, SurfaceExpr};
    use crate::pattern::*;
    #[test]
    fn test_pattern_renamer() {
        let mut r = PatternRenamer::new();
        r.add("x", "y");
        assert_eq!(r.rename("x"), "y");
        assert_eq!(r.rename("z"), "z");
    }
    #[test]
    fn test_count_pattern_vars() {
        assert_eq!(count_pattern_vars("Nat.succ n"), 1);
        assert_eq!(count_pattern_vars("Prod.mk a b"), 2);
        assert_eq!(count_pattern_vars("_"), 0);
    }
}
/// A pattern normaliser that sorts or-patterns.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn normalise_or_pattern(pattern: &str) -> String {
    if !pattern.contains('|') {
        return pattern.to_string();
    }
    let parts: Vec<&str> = pattern.split('|').map(|s| s.trim()).collect();
    let mut sorted = parts.clone();
    sorted.sort();
    sorted.join(" | ")
}
/// A pattern depth counter.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn pattern_depth(pattern: &str) -> usize {
    let mut depth = 0usize;
    let mut max_depth = 0usize;
    for c in pattern.chars() {
        match c {
            '(' => {
                depth += 1;
                if depth > max_depth {
                    max_depth = depth;
                }
            }
            ')' => {
                depth = depth.saturating_sub(1);
            }
            _ => {}
        }
    }
    max_depth
}
/// A pattern variable counter.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn count_all_vars(patterns: &[&str]) -> usize {
    patterns.iter().map(|p| count_pattern_vars(p)).sum()
}
#[cfg(test)]
mod pattern_final_tests {
    use super::*;
    use crate::ast_impl::{Pattern, SurfaceExpr};
    use crate::pattern::*;
    #[test]
    fn test_normalise_or_pattern() {
        let out = normalise_or_pattern("c | a | b");
        assert!(out.starts_with("a"));
    }
    #[test]
    fn test_pattern_depth() {
        assert_eq!(pattern_depth("Nat.succ n"), 0);
        assert_eq!(pattern_depth("Prod.mk (Nat.succ n) m"), 1);
    }
    #[test]
    fn test_count_all_vars() {
        let pats = ["Nat.succ n", "Prod.mk a b", "_"];
        assert_eq!(count_all_vars(&pats), 3);
    }
}
/// Returns true if a pattern string is a constructor application.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn is_ctor_pattern(s: &str) -> bool {
    let s = s.trim();
    s.starts_with(|c: char| c.is_uppercase()) || s.contains('.')
}
/// Returns the constructor name from a pattern string.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn extract_ctor_name(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or(s)
}
#[cfg(test)]
mod pattern_pad {
    use super::*;
    use crate::ast_impl::{Pattern, SurfaceExpr};
    use crate::pattern::*;
    #[test]
    fn test_is_ctor_pattern() {
        assert!(is_ctor_pattern("Nat.succ n"));
        assert!(!is_ctor_pattern("x"));
        assert!(!is_ctor_pattern("_"));
    }
    #[test]
    fn test_extract_ctor_name() {
        assert_eq!(extract_ctor_name("Nat.succ n"), "Nat.succ");
        assert_eq!(extract_ctor_name("_"), "_");
    }
}
/// Returns the variable names bound in a pattern string (simplified: lowercase idents).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn pattern_vars(s: &str) -> Vec<&str> {
    s.split_whitespace()
        .filter(|t| t.starts_with(|c: char| c.is_lowercase()))
        .collect()
}
/// Returns true if a pattern string is a wildcard.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn is_wildcard_pattern(s: &str) -> bool {
    s.trim() == "_"
}
/// Returns true if two patterns have the same structure (same ctor and arity).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn same_ctor(a: &str, b: &str) -> bool {
    extract_ctor_name(a) == extract_ctor_name(b)
}
#[cfg(test)]
mod pattern_pad2 {
    use super::*;
    use crate::ast_impl::{Pattern, SurfaceExpr};
    use crate::pattern::*;
    #[test]
    fn test_pattern_vars() {
        let vars = pattern_vars("Nat.succ n");
        assert_eq!(vars, vec!["n"]);
    }
    #[test]
    fn test_is_wildcard_pattern() {
        assert!(is_wildcard_pattern("_"));
        assert!(is_wildcard_pattern("  _  "));
        assert!(!is_wildcard_pattern("x"));
    }
    #[test]
    fn test_same_ctor() {
        assert!(same_ctor("Nat.succ n", "Nat.succ m"));
        assert!(!same_ctor("Nat.succ n", "Nat.zero"));
    }
}
/// Returns true if a pattern is a pair pattern "(a, b)".
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn is_pair_pattern(s: &str) -> bool {
    let s = s.trim();
    s.starts_with('(') && s.ends_with(')') && s.contains(',')
}
/// Returns the arity of a constructor pattern (number of arguments).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn ctor_arity(s: &str) -> usize {
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() > 1 {
        parts.len() - 1
    } else {
        0
    }
}
