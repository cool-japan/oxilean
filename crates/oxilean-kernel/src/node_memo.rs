//! Memo tables over the shared subterms of one term.
//!
//! An [`Expr`] is a DAG: a subterm referenced by several parents is one
//! allocation ([`Node`]). A walk that follows every edge visits such a
//! subterm once per PATH to it, and a walk that rebuilds the term as it goes
//! turns the DAG into a tree — a term of a few thousand nodes whose paths
//! number in the millions becomes millions of nodes. The proofs `omega` and
//! `simp` produce are shaped like that, and so was the tail of declarations
//! that ran out of the per-declaration budget.
//!
//! The walks that rebuild a term (substitution, lifting, abstraction,
//! universe instantiation) and the ones that only read it therefore keep a
//! [`NodeMemo`]: the result for each shared subterm they have finished,
//! keyed by the subterm's ADDRESS and the binder depth it was reached at.
//! Reaching it again returns the stored result, so a shared subterm is
//! processed once and stays shared in the output.
//!
//! # Why keying by address is sound
//!
//! An address identifies an allocation only while the allocation is alive.
//! A memo is created for one walk over one term and borrows that term for
//! its whole life (`'term`); [`NodeMemo::get`] and [`NodeMemo::insert`] take
//! `&'term Node`, so every key is a subterm of the borrowed term, which
//! cannot be freed — or its address reused — before the memo is gone. The
//! memo never outlives the walk and is never consulted for any other term.
//!
//! The table changes how often a subterm is visited, never what is computed
//! for it: the walk's result for a subterm is a function of the subterm and
//! the depth alone.

use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};
use std::marker::PhantomData;

use crate::structural_hash::{mix, mix_bytes};
use crate::{Expr, Node};

/// A hasher for keys made of a few machine words (an address, a depth).
///
/// Not keyed: these tables live for one walk, their keys are addresses and
/// depths rather than anything read from the input, and a collision costs a
/// probe, never a wrong answer.
#[derive(Clone, Copy, Default)]
pub(crate) struct WordHasher(u64);

impl Hasher for WordHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.0 = mix_bytes(self.0, bytes);
    }
    #[inline]
    fn write_u32(&mut self, value: u32) {
        self.0 = mix(self.0, u64::from(value));
    }
    #[inline]
    fn write_u64(&mut self, value: u64) {
        self.0 = mix(self.0, value);
    }
    #[inline]
    fn write_usize(&mut self, value: usize) {
        // `usize` is at most 64 bits wide on every supported target.
        self.0 = mix(self.0, value as u64);
    }
}

/// [`WordHasher`] as a `BuildHasher`.
pub(crate) type WordBuildHasher = BuildHasherDefault<WordHasher>;

/// The results a walk has computed for the shared subterms of the term it
/// is walking, by subterm and binder depth. See the module documentation.
pub(crate) struct NodeMemo<'term, V> {
    /// Allocated on the first insertion: most walks meet no shared subterm
    /// that needs rebuilding and never pay for a table.
    done: HashMap<(*const Expr, u32), V, WordBuildHasher>,
    /// The term whose subterms are the keys, borrowed for the memo's life.
    term: PhantomData<&'term Expr>,
}

impl<'term, V> NodeMemo<'term, V> {
    /// An empty memo.
    #[inline]
    pub(crate) fn new() -> Self {
        Self {
            done: HashMap::default(),
            term: PhantomData,
        }
    }

    /// The result stored for `node` at `depth`, if the walk has finished it
    /// there before.
    #[inline]
    pub(crate) fn get(&self, node: &'term Node, depth: u32) -> Option<&V> {
        if self.done.is_empty() {
            return None;
        }
        self.done.get(&(node.as_ptr(), depth))
    }

    /// Store the result of the walk for `node` at `depth`.
    #[inline]
    pub(crate) fn insert(&mut self, node: &'term Node, depth: u32, value: V) {
        self.done.insert((node.as_ptr(), depth), value);
    }

    /// How many results are stored (for tests).
    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.done.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Level, Name};

    fn leaf(name: &str) -> Expr {
        Expr::Const(Name::str(name), vec![])
    }

    #[test]
    fn a_result_is_found_under_the_node_and_depth_it_was_stored_for() {
        let shared = Node::new(leaf("c"));
        let other = Node::new(leaf("c"));
        let term = Expr::App(shared.clone(), other.clone());
        let Expr::App(first, second) = &term else {
            panic!("an application");
        };
        let mut memo: NodeMemo<'_, u32> = NodeMemo::new();
        assert!(memo.get(first, 0).is_none(), "empty memo");
        memo.insert(first, 0, 7);
        assert_eq!(memo.get(first, 0), Some(&7));
        assert!(memo.get(first, 1).is_none(), "another depth");
        // An equal subterm in another allocation is another key.
        assert!(memo.get(second, 0).is_none(), "another allocation");
        memo.insert(first, 1, 8);
        memo.insert(second, 0, 9);
        assert_eq!(memo.len(), 3);
        assert_eq!(memo.get(first, 1), Some(&8));
        assert_eq!(memo.get(second, 0), Some(&9));
    }

    #[test]
    fn two_edges_to_one_allocation_are_one_key() {
        let shared = Node::new(Expr::Sort(Level::zero()));
        let term = Expr::App(shared.clone(), shared);
        let Expr::App(first, second) = &term else {
            panic!("an application");
        };
        let mut memo: NodeMemo<'_, &str> = NodeMemo::new();
        memo.insert(first, 3, "done");
        assert_eq!(memo.get(second, 3), Some(&"done"));
    }

    #[test]
    fn the_word_hasher_separates_addresses_and_depths() {
        use std::hash::BuildHasher;
        let build = WordBuildHasher::default();
        let hash = |key: (usize, u32)| build.hash_one(key);
        // Addresses of 80-byte allocations are multiples of 16; depths are
        // small. Neither may collapse onto a few values.
        let keys: Vec<(usize, u32)> = (0..2048usize)
            .flat_map(|i| (0..4u32).map(move |depth| (0x1000_0000 + i * 16, depth)))
            .collect();
        let hashes: std::collections::HashSet<u64> = keys.iter().map(|&key| hash(key)).collect();
        assert_eq!(hashes.len(), keys.len());
        let low: std::collections::HashSet<u64> = hashes.iter().map(|h| h & 0xFFF).collect();
        assert!(
            low.len() > 3000,
            "low 12 bits take only {} values",
            low.len()
        );
        assert_eq!(hash((1, 2)), hash((1, 2)));
        assert_ne!(hash((1, 2)), hash((2, 1)));
    }
}
