//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// Fibonacci number utilities.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FibonacciUtil;
#[allow(dead_code)]
impl FibonacciUtil {
    /// Compute the nth Fibonacci number iteratively.
    pub fn fib(n: u64) -> u64 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        let mut a = 0u64;
        let mut b = 1u64;
        for _ in 2..=n {
            let c = a.saturating_add(b);
            a = b;
            b = c;
        }
        b
    }
    /// Check if a number is a Fibonacci number.
    pub fn is_fibonacci(n: u64) -> bool {
        let check = |k: u64| -> bool {
            let sqrt = (k as f64).sqrt() as u64;
            sqrt * sqrt == k || (sqrt + 1) * (sqrt + 1) == k
        };
        let n5 = 5u64.saturating_mul(n.saturating_mul(n));
        check(n5.saturating_add(4)) || (n5 >= 4 && check(n5 - 4))
    }
    /// Zeckendorf representation: every positive integer is uniquely
    /// a sum of non-consecutive Fibonacci numbers.
    pub fn zeckendorf(mut n: u64) -> Vec<u64> {
        if n == 0 {
            return vec![0];
        }
        let mut all_fibs = vec![1u64, 1];
        while *all_fibs
            .last()
            .expect("all_fibs is non-empty: initialized with [1, 1]")
            < n
        {
            let len = all_fibs.len();
            all_fibs.push(all_fibs[len - 1].saturating_add(all_fibs[len - 2]));
        }
        all_fibs.retain(|&x| x <= n);
        all_fibs.dedup();
        let mut result = Vec::new();
        for &fib in all_fibs.iter().rev() {
            if fib <= n {
                result.push(fib);
                n -= fib;
            }
        }
        result
    }
}
/// Arithmetic function utilities.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ArithmeticFunctions;
#[allow(dead_code)]
impl ArithmeticFunctions {
    /// Euler's totient function phi(n).
    pub fn euler_totient(n: u64) -> u64 {
        if n == 0 {
            return 0;
        }
        let mut result = n;
        let mut temp = n;
        let mut p = 2u64;
        while p * p <= temp {
            if temp % p == 0 {
                while temp % p == 0 {
                    temp /= p;
                }
                result -= result / p;
            }
            p += 1;
        }
        if temp > 1 {
            result -= result / temp;
        }
        result
    }
    /// Möbius function mu(n).
    pub fn mobius(mut n: u64) -> i32 {
        if n == 1 {
            return 1;
        }
        let mut num_prime_factors = 0i32;
        let mut p = 2u64;
        while p * p <= n {
            if n % p == 0 {
                num_prime_factors += 1;
                n /= p;
                if n % p == 0 {
                    return 0;
                }
            }
            p += 1;
        }
        if n > 1 {
            num_prime_factors += 1;
        }
        if num_prime_factors % 2 == 0 {
            1
        } else {
            -1
        }
    }
    /// Number of divisors d(n).
    pub fn num_divisors(n: u64) -> u64 {
        if n == 0 {
            return 0;
        }
        (1..=n).filter(|&d| n % d == 0).count() as u64
    }
    /// Sum of divisors sigma(n).
    pub fn sum_of_divisors(n: u64) -> u64 {
        if n == 0 {
            return 0;
        }
        (1..=n).filter(|&d| n % d == 0).sum()
    }
}
/// Collatz conjecture utilities.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CollatzUtil;
#[allow(dead_code)]
impl CollatzUtil {
    /// Collatz sequence starting from n.
    pub fn sequence(mut n: u64) -> Vec<u64> {
        let mut seq = vec![n];
        while n != 1 {
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
            seq.push(n);
            if seq.len() > 10_000 {
                break;
            }
        }
        seq
    }
    /// Stopping time: number of steps to reach 1.
    pub fn stopping_time(n: u64) -> Option<usize> {
        let seq = Self::sequence(n);
        if *seq
            .last()
            .expect("seq is non-empty: sequence(n) always includes n")
            == 1
        {
            Some(seq.len() - 1)
        } else {
            None
        }
    }
}
