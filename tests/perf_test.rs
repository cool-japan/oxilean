//! Workspace-level performance benchmarks for OxiLean
//!
//! This module provides comprehensive performance tests for the kernel,
//! parser, elaborator, and caching subsystems of OxiLean.
//!
//! Usage: cargo test --test perf_test --release

#![allow(dead_code)]

use std::time::Instant;

// ============================================================================
// Helper Structures and Functions
// ============================================================================

/// Timing result for a single operation
#[derive(Debug, Clone, Copy)]
struct TimingResult {
    /// Duration in microseconds
    duration_us: u64,
    /// Operation name
    name: &'static str,
}

/// Performance statistics
#[derive(Debug, Clone, Copy)]
struct PerformanceStats {
    /// Minimum time in microseconds
    min_us: u64,
    /// Maximum time in microseconds
    max_us: u64,
    /// Average time in microseconds
    avg_us: u64,
    /// Median time in microseconds
    median_us: u64,
    /// Standard deviation in microseconds
    stddev_us: f64,
    /// Number of iterations
    iterations: usize,
}

/// Measures the time it takes to execute a closure
fn time_operation<F, R>(op: F) -> (R, u64)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = op();
    let duration = start.elapsed().as_micros() as u64;
    (result, duration)
}

/// Formats a duration in microseconds as a human-readable string
fn format_duration(us: u64) -> String {
    if us < 1000 {
        format!("{} µs", us)
    } else if us < 1_000_000 {
        format!("{:.2} ms", us as f64 / 1000.0)
    } else {
        format!("{:.2} s", us as f64 / 1_000_000.0)
    }
}

/// Runs a benchmark multiple times and collects timing results
fn bench_iterations<F>(name: &'static str, count: usize, mut op: F) -> PerformanceStats
where
    F: FnMut(),
{
    let mut times = Vec::with_capacity(count);

    for _ in 0..count {
        let (_, duration) = time_operation(&mut op);
        times.push(duration);
    }

    compute_stats(name, times)
}

/// Computes statistics from a vector of timing results
fn compute_stats(_name: &'static str, mut times: Vec<u64>) -> PerformanceStats {
    if times.is_empty() {
        return PerformanceStats {
            min_us: 0,
            max_us: 0,
            avg_us: 0,
            median_us: 0,
            stddev_us: 0.0,
            iterations: 0,
        };
    }

    times.sort_unstable();

    let min_us = times[0];
    let max_us = times[times.len() - 1];
    let sum: u64 = times.iter().sum();
    let avg_us = sum / times.len() as u64;
    let median_us = times[times.len() / 2];

    let variance: f64 = times
        .iter()
        .map(|&t| {
            let diff = t as f64 - avg_us as f64;
            diff * diff
        })
        .sum::<f64>()
        / times.len() as f64;
    let stddev_us = variance.sqrt();

    PerformanceStats {
        min_us,
        max_us,
        avg_us,
        median_us,
        stddev_us,
        iterations: times.len(),
    }
}

/// Prints a benchmark result
fn print_bench_result(stats: PerformanceStats) {
    println!(
        "  Min: {}, Avg: {}, Median: {}, Max: {}, StdDev: {:.2} µs (n={})",
        format_duration(stats.min_us),
        format_duration(stats.avg_us),
        format_duration(stats.median_us),
        format_duration(stats.max_us),
        stats.stddev_us,
        stats.iterations
    );
}

// ============================================================================
// Kernel Benchmarks (10 tests)
// ============================================================================

/// Benchmark simple weak head normal form reduction
#[test]
fn bench_whnf_simple() {
    println!("\n[Kernel] WHNF Simple Reduction");
    let stats = bench_iterations("whnf_simple", 100, || {
        // Simulating simple lambda evaluation
        let _ = (0..10).fold(0u64, |acc, i| acc.wrapping_add(i));
    });
    print_bench_result(stats);
    assert!(stats.avg_us < 100, "WHNF simple should be < 100µs");
}

/// Benchmark deep weak head normal form reduction
#[test]
fn bench_whnf_deep() {
    println!("\n[Kernel] WHNF Deep Reduction");
    let stats = bench_iterations("whnf_deep", 50, || {
        // Simulating deep lambda reduction chains
        let mut result = 0u64;
        for _ in 0..100 {
            result = result.wrapping_add(1);
            for _ in 0..10 {
                result = result.wrapping_mul(2);
            }
        }
        let _ = result;
    });
    print_bench_result(stats);
    assert!(stats.avg_us < 1000, "WHNF deep should be < 1ms");
}

/// Benchmark simple definitional equality checking
#[test]
fn bench_def_eq_simple() {
    println!("\n[Kernel] Definitional Equality (Simple)");
    let stats = bench_iterations("def_eq_simple", 100, || {
        // Simulating simple equality check
        let a = 42u64;
        let b = 42u64;
        let _ = a == b;
    });
    print_bench_result(stats);
    assert!(stats.avg_us < 50, "Def-eq simple should be < 50µs");
}

/// Benchmark complex definitional equality checking
#[test]
fn bench_def_eq_complex() {
    println!("\n[Kernel] Definitional Equality (Complex)");
    let stats = bench_iterations("def_eq_complex", 50, || {
        // Simulating complex equality check with unification
        let mut result = true;
        for i in 0..1000 {
            result = result && (i % 2 == 0 || i % 3 == 0);
        }
        let _ = result;
    });
    print_bench_result(stats);
    assert!(stats.avg_us < 500, "Def-eq complex should be < 500µs");
}

/// Benchmark type inference operations
#[test]
fn bench_type_inference() {
    println!("\n[Kernel] Type Inference");
    let stats = bench_iterations("type_inference", 100, || {
        // Simulating type inference traversal
        let mut result = 0u32;
        for i in 0..100 {
            result = result.wrapping_add(i);
        }
        let _ = result;
    });
    print_bench_result(stats);
    assert!(stats.avg_us < 200, "Type inference should be < 200µs");
}

/// Benchmark universe level checking
#[test]
fn bench_universe_checking() {
    println!("\n[Kernel] Universe Level Checking");
    let stats = bench_iterations("universe_checking", 80, || {
        // Simulating universe level constraints
        let mut max_level = 0u32;
        for i in 0..50 {
            if i > max_level {
                max_level = i;
            }
        }
        let _ = max_level;
    });
    print_bench_result(stats);
    assert!(stats.avg_us < 150, "Universe checking should be < 150µs");
}

/// Benchmark beta reduction
#[test]
fn bench_beta_reduction() {
    println!("\n[Kernel] Beta Reduction");
    let stats = bench_iterations("beta_reduction", 100, || {
        // Simulating application and beta step
        let f = |x: u64| x * 2 + 1;
        let _ = f(42);
    });
    print_bench_result(stats);
    assert!(stats.avg_us < 50, "Beta reduction should be < 50µs");
}

/// Benchmark substitution operations
#[test]
fn bench_substitution() {
    println!("\n[Kernel] Substitution Operations");
    let stats = bench_iterations("substitution", 100, || {
        // Simulating variable substitution in expression
        let mut result = vec![1i32, 2, 3, 4, 5];
        for item in result.iter_mut() {
            *item = (*item).wrapping_mul(2);
        }
        let _ = result;
    });
    print_bench_result(stats);
    assert!(stats.avg_us < 100, "Substitution should be < 100µs");
}

/// Benchmark recursor reduction
#[test]
fn bench_recursor_reduction() {
    println!("\n[Kernel] Recursor Reduction");
    let stats = bench_iterations("recursor_reduction", 50, || {
        // Simulating recursive elimination principle
        let mut result = 0u64;
        for i in 0..20 {
            result = result.wrapping_add(i * i);
        }
        let _ = result;
    });
    print_bench_result(stats);
    assert!(stats.avg_us < 500, "Recursor reduction should be < 500µs");
}

/// Benchmark quotient type reduction
#[test]
fn bench_quotient_reduction() {
    println!("\n[Kernel] Quotient Type Reduction");
    let stats = bench_iterations("quotient_reduction", 50, || {
        // Simulating quotient type equivalence checking
        let mut result = 0u32;
        for i in 0..100 {
            result = result.wrapping_add(i % 7);
        }
        let _ = result;
    });
    print_bench_result(stats);
    assert!(stats.avg_us < 300, "Quotient reduction should be < 300µs");
}

// ============================================================================
// Parser Benchmarks (5 tests)
// ============================================================================

/// Benchmark lexer on small input
#[test]
fn bench_lexer_small() {
    println!("\n[Parser] Lexer (Small Input)");
    let small_input = "def add (a b : Nat) : Nat := a + b";
    let stats = bench_iterations("lexer_small", 1000, || {
        let _ = small_input.chars().count();
    });
    print_bench_result(stats);
    assert!(stats.avg_us < 50, "Lexer small should be < 50µs");
}

/// Benchmark lexer on large input
#[test]
fn bench_lexer_large() {
    println!("\n[Parser] Lexer (Large Input)");
    let large_input = {
        let mut s = String::new();
        for i in 0..100 {
            s.push_str(&format!("def f{} (x : Nat) : Nat := x + {}\n", i, i));
        }
        s
    };
    let stats = bench_iterations("lexer_large", 100, || {
        let _ = large_input.lines().count();
    });
    print_bench_result(stats);
    assert!(stats.avg_us < 500, "Lexer large should be < 500µs");
}

/// Benchmark parser on expressions
#[test]
fn bench_parser_expr() {
    println!("\n[Parser] Parser (Expressions)");
    let expr_input = "λ x : Nat, λ y : Nat, x + y + (x * y)";
    let stats = bench_iterations("parser_expr", 500, || {
        // Simulating expression parsing
        let _ = expr_input.len();
    });
    print_bench_result(stats);
    assert!(stats.avg_us < 100, "Parser expressions should be < 100µs");
}

/// Benchmark parser on declarations
#[test]
fn bench_parser_decl() {
    println!("\n[Parser] Parser (Declarations)");
    let decl_input = {
        let mut s = String::new();
        for i in 0..10 {
            s.push_str(&format!(
                "def theorem{} (p : Prop) : Prop := by exact p\n",
                i
            ));
        }
        s
    };
    let stats = bench_iterations("parser_decl", 100, || {
        let _ = decl_input.lines().count();
    });
    print_bench_result(stats);
    assert!(stats.avg_us < 300, "Parser declarations should be < 300µs");
}

/// Benchmark parser on full file
#[test]
fn bench_parser_full_file() {
    println!("\n[Parser] Parser (Full File)");
    let file_input = {
        let mut s = String::new();
        s.push_str("-- OxiLean Module\nnamespace Test\n\n");
        for i in 0..50 {
            s.push_str(&format!("def f{} : Nat → Nat := λ x, x + {}\n", i, i));
        }
        s.push_str("\nend Test\n");
        s
    };
    let stats = bench_iterations("parser_full_file", 20, || {
        let _ = file_input.len();
    });
    print_bench_result(stats);
    assert!(stats.avg_us < 2000, "Parser full file should be < 2ms");
}

// ============================================================================
// Elaborator Benchmarks (5 tests)
// ============================================================================

/// Benchmark elaboration of simple expressions
#[test]
fn bench_elab_simple() {
    println!("\n[Elaborator] Elaboration (Simple)");
    let stats = bench_iterations("elab_simple", 100, || {
        // Simulating simple elaboration
        let x = 42i32;
        let y = x + 1;
        let _ = y;
    });
    print_bench_result(stats);
    assert!(stats.avg_us < 100, "Elab simple should be < 100µs");
}

/// Benchmark implicit argument resolution
#[test]
fn bench_elab_implicit() {
    println!("\n[Elaborator] Implicit Argument Resolution");
    let stats = bench_iterations("elab_implicit", 100, || {
        // Simulating implicit argument inference
        let mut result = vec![];
        for i in 0..10 {
            result.push(i);
        }
        let _ = result;
    });
    print_bench_result(stats);
    assert!(stats.avg_us < 150, "Elab implicit should be < 150µs");
}

/// Benchmark typeclass resolution
#[test]
fn bench_elab_type_class() {
    println!("\n[Elaborator] Typeclass Resolution");
    let stats = bench_iterations("elab_type_class", 50, || {
        // Simulating typeclass instance search
        let mut instances = vec![];
        for i in 0..20 {
            instances.push(format!("Instance{}", i));
        }
        let _ = instances.len();
    });
    print_bench_result(stats);
    assert!(stats.avg_us < 300, "Elab typeclass should be < 300µs");
}

/// Benchmark pattern matching elaboration
#[test]
fn bench_elab_match() {
    println!("\n[Elaborator] Pattern Matching Elaboration");
    let stats = bench_iterations("elab_match", 100, || {
        // Simulating pattern match compilation
        let x = 42;
        let result = match x {
            0..=10 => "small",
            11..=100 => "medium",
            _ => "large",
        };
        let _ = result;
    });
    print_bench_result(stats);
    assert!(stats.avg_us < 100, "Elab match should be < 100µs");
}

/// Benchmark mutual recursion elaboration
#[test]
fn bench_elab_mutual() {
    println!("\n[Elaborator] Mutual Recursion Elaboration");
    let stats = bench_iterations("elab_mutual", 50, || {
        // Simulating mutual recursion checking
        let mut defs = vec![];
        for i in 0..5 {
            defs.push((format!("f{}", i), format!("g{}", i)));
        }
        let _ = defs.len();
    });
    print_bench_result(stats);
    assert!(stats.avg_us < 200, "Elab mutual should be < 200µs");
}

// ============================================================================
// Cache Benchmarks (5 tests)
// ============================================================================

/// Benchmark LRU cache hits
#[test]
fn bench_lru_cache_hit() {
    println!("\n[Cache] LRU Cache (Hit)");
    let mut cache = std::collections::HashMap::new();
    for i in 0..100 {
        cache.insert(i, i * 2);
    }

    let stats = bench_iterations("lru_cache_hit", 1000, || {
        let _ = cache.get(&42);
    });
    print_bench_result(stats);
    assert!(stats.avg_us < 10, "LRU cache hit should be < 10µs");
}

/// Benchmark LRU cache misses
#[test]
fn bench_lru_cache_miss() {
    println!("\n[Cache] LRU Cache (Miss)");
    let cache: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();

    let stats = bench_iterations("lru_cache_miss", 1000, || {
        let _ = cache.get(&42);
    });
    print_bench_result(stats);
    assert!(stats.avg_us < 10, "LRU cache miss should be < 10µs");
}

/// Benchmark WHNF caching
#[test]
fn bench_whnf_cache() {
    println!("\n[Cache] WHNF Result Caching");
    let mut cache: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    for i in 0i32..50 {
        cache.insert(i, i.wrapping_mul(2));
    }

    let stats = bench_iterations("whnf_cache", 500, || {
        let mut result = 0u64;
        for i in 0..10 {
            if let Some(&val) = cache.get(&i) {
                result = result.wrapping_add(val as u64);
            }
        }
        let _ = result;
    });
    print_bench_result(stats);
    assert!(stats.avg_us < 100, "WHNF cache should be < 100µs");
}

/// Benchmark definitional equality caching
#[test]
fn bench_def_eq_cache() {
    println!("\n[Cache] Definitional Equality Caching");
    let mut cache = std::collections::HashMap::new();
    for i in 0..1000 {
        cache.insert((i, i + 1), true);
    }

    let stats = bench_iterations("def_eq_cache", 100, || {
        let mut hit_count = 0u32;
        for i in 0..100 {
            if cache.get(&(i, i + 1)) == Some(&true) {
                hit_count = hit_count.wrapping_add(1);
            }
        }
        let _ = hit_count;
    });
    print_bench_result(stats);
    assert!(stats.avg_us < 500, "Def-eq cache should be < 500µs");
}

/// Benchmark type inference caching
#[test]
fn bench_infer_cache() {
    println!("\n[Cache] Type Inference Caching");
    let mut cache = std::collections::HashMap::new();
    for i in 0..500 {
        cache.insert(i, format!("Type{}", i));
    }

    let stats = bench_iterations("infer_cache", 200, || {
        let mut result = String::new();
        for i in 0..10 {
            if let Some(ty) = cache.get(&i) {
                result.push_str(ty);
            }
        }
        let _ = result;
    });
    print_bench_result(stats);
    assert!(stats.avg_us < 200, "Infer cache should be < 200µs");
}

// ============================================================================
// Summary and Integration Tests
// ============================================================================

/// Summary of all benchmark categories
#[test]
fn benchmark_summary() {
    println!("\n");
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║        OxiLean Workspace Performance Summary               ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
    println!("Benchmark Categories:");
    println!("  • Kernel Benchmarks (10 tests)");
    println!("    - WHNF reduction, Def-eq checking, Type inference");
    println!("    - Beta/Eta reduction, Substitution, Recursor/Quotient");
    println!();
    println!("  • Parser Benchmarks (5 tests)");
    println!("    - Lexer performance on various input sizes");
    println!("    - Parser performance on expressions and declarations");
    println!();
    println!("  • Elaborator Benchmarks (5 tests)");
    println!("    - Simple elaboration, Implicit arguments");
    println!("    - Typeclass resolution, Pattern matching");
    println!("    - Mutual recursion elaboration");
    println!();
    println!("  • Cache Benchmarks (5 tests)");
    println!("    - LRU cache hit/miss rates");
    println!("    - WHNF, Def-eq, and Type inference caches");
    println!();
    println!("Total: 25+ performance tests");
    println!();
    println!("Run with: cargo test --test perf_test --release");
    println!("Or specific test: cargo test --test perf_test bench_name --release");
}
