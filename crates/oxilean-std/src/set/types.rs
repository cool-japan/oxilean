//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::HashSet;

/// Finite relation as a set of ordered pairs.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Relation<A: Clone + Eq + std::hash::Hash> {
    pub domain: Vec<A>,
    pub pairs: std::collections::HashSet<(usize, usize)>,
}
#[allow(dead_code)]
impl<A: Clone + Eq + std::hash::Hash> Relation<A> {
    pub fn new(domain: Vec<A>) -> Self {
        Relation {
            domain,
            pairs: std::collections::HashSet::new(),
        }
    }
    pub fn add_pair(&mut self, a: &A, b: &A) {
        let i = self.domain.iter().position(|x| x == a);
        let j = self.domain.iter().position(|x| x == b);
        if let (Some(i), Some(j)) = (i, j) {
            self.pairs.insert((i, j));
        }
    }
    pub fn contains(&self, a: &A, b: &A) -> bool {
        let i = self.domain.iter().position(|x| x == a);
        let j = self.domain.iter().position(|x| x == b);
        match (i, j) {
            (Some(i), Some(j)) => self.pairs.contains(&(i, j)),
            _ => false,
        }
    }
    pub fn is_reflexive(&self) -> bool {
        (0..self.domain.len()).all(|i| self.pairs.contains(&(i, i)))
    }
    pub fn is_symmetric(&self) -> bool {
        self.pairs
            .iter()
            .all(|&(i, j)| self.pairs.contains(&(j, i)))
    }
    pub fn is_transitive(&self) -> bool {
        let n = self.domain.len();
        for i in 0..n {
            for j in 0..n {
                if self.pairs.contains(&(i, j)) {
                    for k in 0..n {
                        if self.pairs.contains(&(j, k)) && !self.pairs.contains(&(i, k)) {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
    pub fn is_equivalence(&self) -> bool {
        self.is_reflexive() && self.is_symmetric() && self.is_transitive()
    }
    pub fn is_antisymmetric(&self) -> bool {
        self.pairs
            .iter()
            .all(|&(i, j)| i == j || !self.pairs.contains(&(j, i)))
    }
    pub fn is_partial_order(&self) -> bool {
        self.is_reflexive() && self.is_antisymmetric() && self.is_transitive()
    }
    pub fn equivalence_class(&self, a: &A) -> Vec<A> {
        let i = self.domain.iter().position(|x| x == a);
        match i {
            None => Vec::new(),
            Some(i) => self
                .pairs
                .iter()
                .filter(|&&(x, _)| x == i)
                .map(|&(_, j)| self.domain[j].clone())
                .collect(),
        }
    }
}
/// Set partition into disjoint non-empty blocks.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SetPartition {
    pub n: usize,
    pub blocks: Vec<std::collections::HashSet<usize>>,
}
#[allow(dead_code)]
impl SetPartition {
    pub fn discrete(n: usize) -> Self {
        SetPartition {
            n,
            blocks: (0..n)
                .map(|i| {
                    let mut s = std::collections::HashSet::new();
                    s.insert(i);
                    s
                })
                .collect(),
        }
    }
    pub fn trivial(n: usize) -> Self {
        let mut block = std::collections::HashSet::new();
        for i in 0..n {
            block.insert(i);
        }
        SetPartition {
            n,
            blocks: vec![block],
        }
    }
    pub fn n_blocks(&self) -> usize {
        self.blocks.len()
    }
    pub fn block_of(&self, element: usize) -> Option<usize> {
        self.blocks.iter().position(|b| b.contains(&element))
    }
    pub fn merge_blocks(&mut self, i: usize, j: usize) {
        if i == j || i >= self.blocks.len() || j >= self.blocks.len() {
            return;
        }
        let block_j = self.blocks[j].clone();
        let block_i = &mut self.blocks[i];
        for x in block_j {
            block_i.insert(x);
        }
        self.blocks.remove(j);
    }
    pub fn is_valid(&self) -> bool {
        let mut covered = vec![false; self.n];
        for block in &self.blocks {
            if block.is_empty() {
                return false;
            }
            for &x in block {
                if x >= self.n || covered[x] {
                    return false;
                }
                covered[x] = true;
            }
        }
        covered.iter().all(|&c| c)
    }
}
/// Ordered pair (Kuratowski definition: {a} and {a,b} as sets).
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderedPair<A: Clone + Eq, B: Clone + Eq> {
    pub fst: A,
    pub snd: B,
}
#[allow(dead_code)]
impl<A: Clone + Eq, B: Clone + Eq> OrderedPair<A, B> {
    pub fn new(a: A, b: B) -> Self {
        OrderedPair { fst: a, snd: b }
    }
    pub fn swap(self) -> OrderedPair<B, A> {
        OrderedPair {
            fst: self.snd,
            snd: self.fst,
        }
    }
}
