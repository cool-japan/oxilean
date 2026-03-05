//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// Integers represented as equivalence classes of pairs `(pos, neg)` where
/// the value is `pos − neg` (as natural numbers).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IntegerQuotient {
    /// Representative: (m, n) represents the integer m − n.
    pub pos: u64,
    pub neg: u64,
}
#[allow(dead_code)]
impl IntegerQuotient {
    /// Create from a signed integer value.
    pub fn from_i64(n: i64) -> Self {
        if n >= 0 {
            Self {
                pos: n as u64,
                neg: 0,
            }
        } else {
            Self {
                pos: 0,
                neg: (-n) as u64,
            }
        }
    }
    /// Reduce to canonical form (either pos=0 or neg=0).
    pub fn reduce(&self) -> Self {
        if self.pos >= self.neg {
            Self {
                pos: self.pos - self.neg,
                neg: 0,
            }
        } else {
            Self {
                pos: 0,
                neg: self.neg - self.pos,
            }
        }
    }
    /// Value as i64.
    pub fn to_i64(&self) -> i64 {
        (self.pos as i64) - (self.neg as i64)
    }
    /// Addition: (m,n) + (m',n') = (m+m', n+n').
    pub fn add(&self, other: &Self) -> Self {
        Self {
            pos: self.pos + other.pos,
            neg: self.neg + other.neg,
        }
        .reduce()
    }
    /// Negation: -(m,n) = (n,m).
    pub fn negate(&self) -> Self {
        Self {
            pos: self.neg,
            neg: self.pos,
        }
    }
    /// Multiplication: (m,n) * (m',n') = (m*m' + n*n', m*n' + n*m').
    pub fn mul(&self, other: &Self) -> Self {
        let pos = self.pos * other.pos + self.neg * other.neg;
        let neg = self.pos * other.neg + self.neg * other.pos;
        Self { pos, neg }.reduce()
    }
    /// Equivalence: (m,n) ~ (m',n') iff m+n' = m'+n.
    pub fn equiv(&self, other: &Self) -> bool {
        self.pos + other.neg == other.pos + self.neg
    }
}
/// A map between quotient types that is well-defined (respects the relation).
///
/// Stores the underlying raw function and a proof obligation (checked lazily).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct QuotientMap {
    /// Name / description of the map.
    pub name: String,
    /// Is the map known to be well-defined on equivalence classes?
    pub well_defined: bool,
    /// Is the map injective on equivalence classes?
    pub injective: bool,
    /// Is the map surjective onto the target quotient?
    pub surjective: bool,
}
#[allow(dead_code)]
impl QuotientMap {
    /// Create a new `QuotientMap` descriptor.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            well_defined: false,
            injective: false,
            surjective: false,
        }
    }
    /// Mark this map as well-defined (proved externally).
    pub fn mark_well_defined(mut self) -> Self {
        self.well_defined = true;
        self
    }
    /// Mark this map as injective.
    pub fn mark_injective(mut self) -> Self {
        self.injective = true;
        self
    }
    /// Mark this map as surjective.
    pub fn mark_surjective(mut self) -> Self {
        self.surjective = true;
        self
    }
    /// Is this map bijective?
    pub fn is_bijective(&self) -> bool {
        self.injective && self.surjective
    }
    /// Is this map a quotient isomorphism?
    pub fn is_isomorphism(&self) -> bool {
        self.well_defined && self.is_bijective()
    }
}
/// A concrete, computationally-checkable setoid over values of type `T`.
///
/// The equivalence relation is given by a user-supplied function.
#[allow(dead_code)]
pub struct ConcreteSetoid<T> {
    /// The equivalence predicate.  Must be an equivalence relation.
    pub equiv: Box<dyn Fn(&T, &T) -> bool>,
}
#[allow(dead_code)]
impl<T: Clone> ConcreteSetoid<T> {
    /// Create a new setoid with the given equivalence predicate.
    pub fn new(equiv: impl Fn(&T, &T) -> bool + 'static) -> Self {
        Self {
            equiv: Box::new(equiv),
        }
    }
    /// Check if two values are equivalent.
    pub fn are_equiv(&self, a: &T, b: &T) -> bool {
        (self.equiv)(a, b)
    }
    /// Partition a slice into equivalence classes.
    pub fn partition(&self, xs: &[T]) -> Vec<Vec<T>> {
        let mut classes: Vec<Vec<T>> = Vec::new();
        for x in xs {
            let mut found = false;
            for cls in &mut classes {
                if (self.equiv)(x, &cls[0]) {
                    cls.push(x.clone());
                    found = true;
                    break;
                }
            }
            if !found {
                classes.push(vec![x.clone()]);
            }
        }
        classes
    }
    /// Count the number of equivalence classes in a slice.
    pub fn count_classes(&self, xs: &[T]) -> usize {
        self.partition(xs).len()
    }
    /// Check that a function `f : T → U` respects the equivalence relation.
    pub fn function_respects<U: PartialEq>(&self, xs: &[T], f: impl Fn(&T) -> U) -> bool {
        for i in 0..xs.len() {
            for j in 0..xs.len() {
                if (self.equiv)(&xs[i], &xs[j]) && f(&xs[i]) != f(&xs[j]) {
                    return false;
                }
            }
        }
        true
    }
}
/// An element of the free group over an alphabet `char`, represented as a
/// reduced word (alternating generators and their inverses).
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreeGroupElement {
    /// Letters: positive = generator, negative = inverse.
    /// E.g., `[(b'a', true), (b'b', false)]` means `a * b⁻¹`.
    pub letters: Vec<(char, bool)>,
}
#[allow(dead_code)]
impl FreeGroupElement {
    /// The identity element (empty word).
    pub fn identity() -> Self {
        Self { letters: vec![] }
    }
    /// A single generator.
    pub fn generator(c: char) -> Self {
        Self {
            letters: vec![(c, true)],
        }
    }
    /// The inverse of a single generator.
    pub fn generator_inv(c: char) -> Self {
        Self {
            letters: vec![(c, false)],
        }
    }
    /// Concatenate and reduce (cancel adjacent inverse pairs).
    pub fn mul(&self, other: &Self) -> Self {
        let mut result = self.letters.clone();
        for &letter in &other.letters {
            if let Some(&last) = result.last() {
                if last.0 == letter.0 && last.1 != letter.1 {
                    result.pop();
                    continue;
                }
            }
            result.push(letter);
        }
        Self { letters: result }
    }
    /// Inverse: reverse the word and flip all polarities.
    pub fn inverse(&self) -> Self {
        let letters = self.letters.iter().rev().map(|&(c, b)| (c, !b)).collect();
        Self { letters }
    }
    /// Length (number of letters in reduced form).
    pub fn length(&self) -> usize {
        self.letters.len()
    }
    /// Check if this is the identity.
    pub fn is_identity(&self) -> bool {
        self.letters.is_empty()
    }
}
/// Rational numbers as equivalence classes of pairs `(numerator, denominator)`.
/// The equivalence is (p, q) ~ (p', q') iff p * q' = p' * q.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RationalQuotient {
    /// Numerator.
    pub numer: i64,
    /// Denominator (nonzero).
    pub denom: i64,
}
#[allow(dead_code)]
impl RationalQuotient {
    /// Create a new rational, panicking if denom = 0.
    pub fn new(numer: i64, denom: i64) -> Self {
        assert!(denom != 0, "denominator must be nonzero");
        Self { numer, denom }
    }
    /// Compute GCD using the Euclidean algorithm.
    fn gcd(mut a: i64, mut b: i64) -> i64 {
        a = a.abs();
        b = b.abs();
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a
    }
    /// Reduce to canonical form.
    pub fn reduce(&self) -> Self {
        if self.numer == 0 {
            return Self { numer: 0, denom: 1 };
        }
        let g = Self::gcd(self.numer.abs(), self.denom.abs());
        let sign = if self.denom < 0 { -1 } else { 1 };
        Self {
            numer: sign * self.numer / g,
            denom: sign * self.denom / g,
        }
    }
    /// Equivalence: p*q' = p'*q.
    pub fn equiv(&self, other: &Self) -> bool {
        self.numer * other.denom == other.numer * self.denom
    }
    /// Addition: p/q + p'/q' = (p*q' + p'*q) / (q*q').
    pub fn add(&self, other: &Self) -> Self {
        Self {
            numer: self.numer * other.denom + other.numer * self.denom,
            denom: self.denom * other.denom,
        }
        .reduce()
    }
    /// Multiplication: p/q * p'/q' = (p*p') / (q*q').
    pub fn mul(&self, other: &Self) -> Self {
        Self {
            numer: self.numer * other.numer,
            denom: self.denom * other.denom,
        }
        .reduce()
    }
    /// Convert to f64 for display/comparison.
    pub fn to_f64(&self) -> f64 {
        (self.numer as f64) / (self.denom as f64)
    }
}
