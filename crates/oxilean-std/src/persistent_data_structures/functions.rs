//! Persistent (immutable) data structures — implementations and tests.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use super::types::{
    MapNode, PersistentMap, PersistentQueue, PersistentSet, PersistentStack, PersistentVec,
    StackNode, VecNode, BRANCHING, BRANCHING_BITS,
};

// ── helpers ───────────────────────────────────────────────────────────────────

pub(super) fn hash_one<K: Hash>(k: &K) -> u64 {
    let mut h = DefaultHasher::new();
    k.hash(&mut h);
    h.finish()
}

/// Maximum number of elements a tree of the given `shift` depth can hold.
///
/// At `shift = BRANCHING_BITS` (single level) the root is an Internal with
/// up to BRANCHING leaf children → capacity = BRANCHING * BRANCHING.
/// More generally: capacity = BRANCHING^(shift/BRANCHING_BITS + 1).
fn capacity_for_shift(shift: usize) -> usize {
    let levels = shift / BRANCHING_BITS + 1; // number of tree levels (leaves count as 1)
    BRANCHING.pow(levels as u32)
}

/// Insert `val` as the element at position `elem_count` (0-indexed) into `node`.
///
/// Precondition: the subtree has room for one more element.
fn push_into<T: Clone>(
    node: &Arc<VecNode<T>>,
    shift: usize,
    elem_count: usize,
    val: T,
) -> VecNode<T> {
    match node.as_ref() {
        VecNode::Leaf { values } => {
            // The leaf has room (precondition ensures this).
            let mut new_vals = values.clone();
            new_vals.push(val);
            VecNode::Leaf { values: new_vals }
        }
        VecNode::Internal { children, size } => {
            // Determine which child the new element belongs to.
            let child_idx = (elem_count >> shift) & (BRANCHING - 1);
            let child_capacity = capacity_for_shift(shift - BRANCHING_BITS);
            let child_elem_count = elem_count % child_capacity;

            if child_idx < children.len() {
                // Recurse into the existing child.
                let new_child = push_into(
                    &children[child_idx],
                    shift - BRANCHING_BITS,
                    child_elem_count,
                    val,
                );
                let mut new_children = children.clone();
                new_children[child_idx] = Arc::new(new_child);
                VecNode::Internal {
                    children: new_children,
                    size: size + 1,
                }
            } else {
                // Need a new child (a singleton spine).
                let new_child = build_singleton_spine(val, shift - BRANCHING_BITS);
                let mut new_children = children.clone();
                new_children.push(Arc::new(new_child));
                VecNode::Internal {
                    children: new_children,
                    size: size + 1,
                }
            }
        }
    }
}

/// Build a spine of Internal nodes down to a single-element Leaf containing `val`.
///
/// `shift` is the shift of the node to build (not the parent's shift).
/// - shift == 0 (leaf level): returns Leaf([val])
/// - shift > 0 (internal level): returns Internal(\[build_singleton_spine(val, shift - BRANCHING_BITS)\])
fn build_singleton_spine<T: Clone>(val: T, shift: usize) -> VecNode<T> {
    if shift == 0 || shift < BRANCHING_BITS {
        VecNode::Leaf { values: vec![val] }
    } else {
        let child = build_singleton_spine(val, shift - BRANCHING_BITS);
        VecNode::Internal {
            children: vec![Arc::new(child)],
            size: 1,
        }
    }
}

// ── VecNode helpers ───────────────────────────────────────────────────────────

impl<T: Clone> VecNode<T> {
    pub(super) fn size(&self) -> usize {
        match self {
            VecNode::Internal { size, .. } => *size,
            VecNode::Leaf { values } => values.len(),
        }
    }

    pub(super) fn get(&self, idx: usize, shift: usize) -> Option<&T> {
        match self {
            VecNode::Leaf { values } => values.get(idx),
            VecNode::Internal { children, .. } => {
                let child_idx = (idx >> shift) & (BRANCHING - 1);
                children.get(child_idx).and_then(|child| {
                    let lower = idx & ((1 << shift) - 1);
                    if shift >= BRANCHING_BITS {
                        child.get(lower, shift - BRANCHING_BITS)
                    } else {
                        child.get(lower, 0)
                    }
                })
            }
        }
    }

    pub(super) fn set(&self, idx: usize, val: T, shift: usize) -> Self {
        match self {
            VecNode::Leaf { values } => {
                let mut new_vals = values.clone();
                if idx < new_vals.len() {
                    new_vals[idx] = val;
                }
                VecNode::Leaf { values: new_vals }
            }
            VecNode::Internal { children, size } => {
                let child_idx = (idx >> shift) & (BRANCHING - 1);
                let mut new_children = children.clone();
                if child_idx < new_children.len() {
                    let lower = idx & ((1 << shift) - 1);
                    let new_child = if shift >= BRANCHING_BITS {
                        new_children[child_idx].set(lower, val, shift - BRANCHING_BITS)
                    } else {
                        new_children[child_idx].set(lower, val, 0)
                    };
                    new_children[child_idx] = Arc::new(new_child);
                }
                VecNode::Internal {
                    children: new_children,
                    size: *size,
                }
            }
        }
    }

    /// Push `val` into the tree.
    ///
    /// `elem_count` is the total number of elements currently in the subtree.
    /// `shift` is the current level's bit-shift.
    /// Returns the updated node, or `None` if this subtree is completely full.
    pub(super) fn push_leaf(&self, val: T, shift: usize, elem_count: usize) -> Option<Self> {
        match self {
            VecNode::Leaf { values } => {
                if values.len() < BRANCHING {
                    let mut new_vals = values.clone();
                    new_vals.push(val);
                    Some(VecNode::Leaf { values: new_vals })
                } else {
                    None // leaf is full
                }
            }
            VecNode::Internal { children, size } => {
                // Capacity of a single child subtree at the next level down.
                // At shift=BRANCHING_BITS, the child is a leaf: capacity = BRANCHING.
                // At shift>BRANCHING_BITS, child is internal: capacity = BRANCHING^(shift/BRANCHING_BITS).
                let child_capacity: usize = if shift > BRANCHING_BITS {
                    1usize << shift // each child covers 2^shift elements
                } else {
                    BRANCHING // each child is a leaf
                };

                let last_child_elem_count = elem_count % child_capacity;
                // If last_child_elem_count == 0 the last child is either full or absent.
                let last_full = last_child_elem_count == 0 && !children.is_empty();

                if !last_full {
                    // Try to append into the last child.
                    if let Some(last) = children.last() {
                        let child_shift = if shift > BRANCHING_BITS {
                            shift - BRANCHING_BITS
                        } else {
                            0
                        };
                        let result =
                            last.push_leaf(val.clone(), child_shift, last_child_elem_count);
                        if let Some(new_last) = result {
                            let mut new_children = children.clone();
                            let last_idx = new_children.len() - 1;
                            new_children[last_idx] = Arc::new(new_last);
                            return Some(VecNode::Internal {
                                children: new_children,
                                size: size + 1,
                            });
                        }
                    }
                }

                // Last child is full (or absent) — add a new child if there is room.
                if children.len() < BRANCHING {
                    let new_leaf = Arc::new(VecNode::Leaf { values: vec![val] });
                    let mut new_children = children.clone();
                    new_children.push(new_leaf);
                    Some(VecNode::Internal {
                        children: new_children,
                        size: size + 1,
                    })
                } else {
                    None // this internal node is also full
                }
            }
        }
    }

    /// Collect all values in order.
    pub(super) fn collect_values<'a>(&'a self, out: &mut Vec<&'a T>) {
        match self {
            VecNode::Leaf { values } => {
                for v in values {
                    out.push(v);
                }
            }
            VecNode::Internal { children, .. } => {
                for child in children {
                    child.collect_values(out);
                }
            }
        }
    }
}

// ── PersistentVec ─────────────────────────────────────────────────────────────

impl<T: Clone> PersistentVec<T> {
    /// Create an empty persistent vector.
    pub fn new() -> Self {
        PersistentVec {
            root: None,
            len: 0,
            shift: BRANCHING_BITS,
        }
    }

    /// Return the number of elements.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Return true if empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Return the element at `idx`, or `None` if out of bounds.
    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx >= self.len {
            return None;
        }
        self.root.as_ref().and_then(|r| r.get(idx, self.shift))
    }

    /// Return a new vector with `val` appended.
    pub fn push(&self, val: T) -> Self {
        let new_len = self.len + 1;
        match &self.root {
            None => {
                let leaf = VecNode::Leaf { values: vec![val] };
                PersistentVec {
                    root: Some(Arc::new(leaf)),
                    len: new_len,
                    shift: BRANCHING_BITS,
                }
            }
            Some(root) => {
                // Check if the current tree (of depth determined by `shift`) has room.
                // Capacity at the current shift = BRANCHING^(shift/BRANCHING_BITS + 1).
                let capacity = capacity_for_shift(self.shift);
                if self.len < capacity {
                    // There is room; descend and add.
                    let new_root = push_into(root, self.shift, self.len, val);
                    PersistentVec {
                        root: Some(Arc::new(new_root)),
                        len: new_len,
                        shift: self.shift,
                    }
                } else {
                    // Root is full — grow the tree by one level.
                    let new_shift = self.shift + BRANCHING_BITS;
                    // The new element will be the first element under the second child.
                    // We need a spine of Internal nodes from the new root's child level
                    // down to a Leaf containing just `val`.
                    let spine = build_singleton_spine(val, self.shift);
                    let new_root = VecNode::Internal {
                        children: vec![root.clone(), Arc::new(spine)],
                        size: new_len,
                    };
                    PersistentVec {
                        root: Some(Arc::new(new_root)),
                        len: new_len,
                        shift: new_shift,
                    }
                }
            }
        }
    }

    /// Return a new vector with the element at `idx` replaced by `val`.
    ///
    /// Returns the original vector unchanged if `idx` is out of bounds.
    pub fn set(&self, idx: usize, val: T) -> Self {
        if idx >= self.len {
            return self.clone();
        }
        match &self.root {
            None => self.clone(),
            Some(root) => {
                let new_root = root.set(idx, val, self.shift);
                PersistentVec {
                    root: Some(Arc::new(new_root)),
                    len: self.len,
                    shift: self.shift,
                }
            }
        }
    }

    /// Iterate over all elements in order.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let mut collected: Vec<&T> = Vec::with_capacity(self.len);
        if let Some(root) = &self.root {
            root.collect_values(&mut collected);
        }
        collected.into_iter()
    }
}

// ── MapNode helpers ───────────────────────────────────────────────────────────

/// Number of bits used per HAMT level.
const HAMT_BITS: u32 = 5;
/// Mask for one HAMT level.
const HAMT_MASK: u64 = 0x1f;

fn hamt_index(hash: u64, depth: u32) -> usize {
    ((hash >> (depth * HAMT_BITS)) & HAMT_MASK) as usize
}

fn popcount_below(bitmap: u32, bit: usize) -> usize {
    (bitmap & ((1u32 << bit) - 1)).count_ones() as usize
}

impl<K: Clone + Eq + Hash, V: Clone> MapNode<K, V> {
    fn lookup<'a>(&'a self, key: &K, hash: u64, depth: u32) -> Option<&'a V> {
        match self {
            MapNode::Empty => None,
            MapNode::Leaf {
                key: k,
                value,
                hash: h,
            } => {
                if *h == hash && k == key {
                    Some(value)
                } else {
                    None
                }
            }
            MapNode::Inner { bitmap, children } => {
                let bit = hamt_index(hash, depth);
                if *bitmap & (1u32 << bit) == 0 {
                    return None;
                }
                let pos = popcount_below(*bitmap, bit);
                children[pos].lookup(key, hash, depth + 1)
            }
        }
    }

    fn insert_node(self: &Arc<Self>, key: K, value: V, hash: u64, depth: u32) -> Arc<Self> {
        match self.as_ref() {
            MapNode::Empty => Arc::new(MapNode::Leaf { key, value, hash }),
            MapNode::Leaf {
                key: k,
                value: v,
                hash: h,
            } => {
                if *h == hash && *k == key {
                    // Replace
                    return Arc::new(MapNode::Leaf { key, value, hash });
                }
                // Collision — expand into Inner
                let existing = Arc::new(MapNode::Leaf {
                    key: k.clone(),
                    value: v.clone(),
                    hash: *h,
                });
                let existing_bit = hamt_index(*h, depth);
                let new_bit = hamt_index(hash, depth);
                if existing_bit == new_bit {
                    // Same slot — recurse deeper
                    let child = existing.insert_node(key, value, hash, depth + 1);
                    let bitmap = 1u32 << existing_bit;
                    Arc::new(MapNode::Inner {
                        bitmap,
                        children: vec![child],
                    })
                } else {
                    let new_leaf = Arc::new(MapNode::Leaf { key, value, hash });
                    let (bit_a, node_a, bit_b, node_b) = if existing_bit < new_bit {
                        (existing_bit, existing, new_bit, new_leaf)
                    } else {
                        (new_bit, new_leaf, existing_bit, existing)
                    };
                    let bitmap = (1u32 << bit_a) | (1u32 << bit_b);
                    Arc::new(MapNode::Inner {
                        bitmap,
                        children: vec![node_a, node_b],
                    })
                }
            }
            MapNode::Inner { bitmap, children } => {
                let bit = hamt_index(hash, depth);
                let flag = 1u32 << bit;
                let pos = popcount_below(*bitmap, bit);
                let mut new_children = children.clone();
                if *bitmap & flag == 0 {
                    // New slot
                    let leaf = Arc::new(MapNode::Leaf { key, value, hash });
                    new_children.insert(pos, leaf);
                    Arc::new(MapNode::Inner {
                        bitmap: bitmap | flag,
                        children: new_children,
                    })
                } else {
                    // Existing slot
                    new_children[pos] = new_children[pos].insert_node(key, value, hash, depth + 1);
                    Arc::new(MapNode::Inner {
                        bitmap: *bitmap,
                        children: new_children,
                    })
                }
            }
        }
    }

    fn remove_node(self: &Arc<Self>, key: &K, hash: u64, depth: u32) -> Option<Arc<Self>> {
        match self.as_ref() {
            MapNode::Empty => Some(self.clone()),
            MapNode::Leaf {
                key: k, hash: h, ..
            } => {
                if *h == hash && k == key {
                    None
                } else {
                    Some(self.clone())
                }
            }
            MapNode::Inner { bitmap, children } => {
                let bit = hamt_index(hash, depth);
                let flag = 1u32 << bit;
                if *bitmap & flag == 0 {
                    return Some(self.clone());
                }
                let pos = popcount_below(*bitmap, bit);
                let updated = children[pos].remove_node(key, hash, depth + 1);
                let mut new_children = children.clone();
                let new_bitmap;
                match updated {
                    None => {
                        new_children.remove(pos);
                        new_bitmap = bitmap & !flag;
                    }
                    Some(node) => {
                        new_children[pos] = node;
                        new_bitmap = *bitmap;
                    }
                }
                if new_children.is_empty() {
                    None
                } else {
                    Some(Arc::new(MapNode::Inner {
                        bitmap: new_bitmap,
                        children: new_children,
                    }))
                }
            }
        }
    }

    fn count(&self) -> usize {
        match self {
            MapNode::Empty => 0,
            MapNode::Leaf { .. } => 1,
            MapNode::Inner { children, .. } => children.iter().map(|c| c.count()).sum(),
        }
    }

    fn collect_keys<'a>(&'a self, out: &mut Vec<&'a K>) {
        match self {
            MapNode::Empty => {}
            MapNode::Leaf { key, .. } => out.push(key),
            MapNode::Inner { children, .. } => {
                for c in children {
                    c.collect_keys(out);
                }
            }
        }
    }
}

// ── PersistentMap ─────────────────────────────────────────────────────────────

impl<K: Clone + Eq + Hash, V: Clone> PersistentMap<K, V> {
    /// Create an empty map.
    pub fn new() -> Self {
        PersistentMap { root: None, len: 0 }
    }

    /// Number of entries.
    pub fn len(&self) -> usize {
        self.len
    }

    /// True if empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Look up a key.
    pub fn get(&self, key: &K) -> Option<&V> {
        let hash = hash_one(key);
        self.root.as_ref().and_then(|r| r.lookup(key, hash, 0))
    }

    /// Return true if the map contains `key`.
    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// Return a new map with `(key, value)` inserted (or updated).
    pub fn insert(&self, key: K, value: V) -> Self {
        let hash = hash_one(&key);
        let had_key = self.contains_key(&key);
        let new_root = match &self.root {
            None => Arc::new(MapNode::Leaf { key, value, hash }),
            Some(root) => root.insert_node(key, value, hash, 0),
        };
        let new_len = if had_key { self.len } else { self.len + 1 };
        PersistentMap {
            root: Some(new_root),
            len: new_len,
        }
    }

    /// Return a new map with `key` removed.
    pub fn remove(&self, key: &K) -> Self {
        if !self.contains_key(key) {
            return self.clone();
        }
        let hash = hash_one(key);
        let new_root = self.root.as_ref().and_then(|r| r.remove_node(key, hash, 0));
        PersistentMap {
            root: new_root,
            len: self.len - 1,
        }
    }

    /// Iterate over keys.
    pub fn keys(&self) -> Vec<&K> {
        let mut out = Vec::with_capacity(self.len);
        if let Some(root) = &self.root {
            root.collect_keys(&mut out);
        }
        out
    }
}

// ── PersistentSet ─────────────────────────────────────────────────────────────

impl<T: Clone + Eq + Hash> PersistentSet<T> {
    /// Create an empty set.
    pub fn new() -> Self {
        PersistentSet {
            map: PersistentMap::new(),
        }
    }

    /// Number of elements.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// True if empty.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Return a new set with `val` inserted.
    pub fn insert(&self, val: T) -> Self {
        PersistentSet {
            map: self.map.insert(val, ()),
        }
    }

    /// Return true if `val` is in the set.
    pub fn contains(&self, val: &T) -> bool {
        self.map.contains_key(val)
    }

    /// Return a new set with `val` removed.
    pub fn remove(&self, val: &T) -> Self {
        PersistentSet {
            map: self.map.remove(val),
        }
    }

    /// Return the union of `self` and `other`.
    pub fn union(&self, other: &Self) -> Self {
        let mut result = self.clone();
        for k in other.map.keys() {
            result = result.insert(k.clone());
        }
        result
    }

    /// Return the intersection of `self` and `other`.
    pub fn intersection(&self, other: &Self) -> Self {
        let mut result = PersistentSet::new();
        for k in self.map.keys() {
            if other.contains(k) {
                result = result.insert(k.clone());
            }
        }
        result
    }
}

// ── PersistentQueue ───────────────────────────────────────────────────────────

impl<T: Clone> PersistentQueue<T> {
    /// Create an empty queue.
    pub fn new() -> Self {
        PersistentQueue {
            front: Vec::new(),
            back: Vec::new(),
        }
    }

    /// Number of elements.
    pub fn len(&self) -> usize {
        self.front.len() + self.back.len()
    }

    /// True if empty.
    pub fn is_empty(&self) -> bool {
        self.front.is_empty() && self.back.is_empty()
    }

    /// Peek at the front element without removing it.
    pub fn peek(&self) -> Option<&T> {
        self.front.first()
    }

    /// Return a new queue with `val` added at the back.
    pub fn push_back(&self, val: T) -> Self {
        let mut new_back = self.back.clone();
        new_back.push(val);
        Self::rebalance(self.front.clone(), new_back)
    }

    /// Remove and return the front element together with the resulting queue.
    ///
    /// Returns `None` if the queue is empty.
    pub fn pop_front(&self) -> Option<(T, Self)> {
        if self.front.is_empty() {
            return None;
        }
        let val = self.front[0].clone();
        let new_front = self.front[1..].to_vec();
        let new_queue = Self::rebalance(new_front, self.back.clone());
        Some((val, new_queue))
    }

    /// Maintain the invariant: if front is empty but back is not, move back into front.
    ///
    /// `back` stores elements in insertion order (oldest first at index 0).
    /// When front is exhausted, we move back directly into front (no reversal).
    fn rebalance(front: Vec<T>, back: Vec<T>) -> Self {
        if front.is_empty() && !back.is_empty() {
            PersistentQueue {
                front: back,
                back: Vec::new(),
            }
        } else {
            PersistentQueue { front, back }
        }
    }
}

// ── PersistentStack ───────────────────────────────────────────────────────────

impl<T: Clone> PersistentStack<T> {
    /// Create an empty stack.
    pub fn new() -> Self {
        PersistentStack { head: None, len: 0 }
    }

    /// Number of elements.
    pub fn len(&self) -> usize {
        self.len
    }

    /// True if empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Peek at the top element.
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.value)
    }

    /// Return a new stack with `val` pushed on top.
    pub fn push(&self, val: T) -> Self {
        let node = StackNode {
            value: val,
            tail: self.head.clone(),
        };
        PersistentStack {
            head: Some(Arc::new(node)),
            len: self.len + 1,
        }
    }

    /// Pop the top element, returning it and the resulting stack.
    ///
    /// Returns `None` if the stack is empty.
    pub fn pop(&self) -> Option<(T, Self)> {
        self.head.as_ref().map(|n| {
            let val = n.value.clone();
            let new_stack = PersistentStack {
                head: n.tail.clone(),
                len: self.len - 1,
            };
            (val, new_stack)
        })
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── PersistentVec tests ───────────────────────────────────────────────────

    #[test]
    fn vec_new_is_empty() {
        let v: PersistentVec<i32> = PersistentVec::new();
        assert_eq!(v.len(), 0);
        assert!(v.is_empty());
    }

    #[test]
    fn vec_push_and_get() {
        let v = PersistentVec::new().push(10).push(20).push(30);
        assert_eq!(v.len(), 3);
        assert_eq!(v.get(0), Some(&10));
        assert_eq!(v.get(1), Some(&20));
        assert_eq!(v.get(2), Some(&30));
        assert_eq!(v.get(3), None);
    }

    #[test]
    fn vec_persistence_after_push() {
        let v0 = PersistentVec::new().push(1).push(2);
        let v1 = v0.push(3);
        // v0 is unaffected
        assert_eq!(v0.len(), 2);
        assert_eq!(v0.get(0), Some(&1));
        assert_eq!(v1.len(), 3);
        assert_eq!(v1.get(2), Some(&3));
    }

    #[test]
    fn vec_set_persistence() {
        let v = PersistentVec::new().push(1).push(2).push(3);
        let v2 = v.set(1, 99);
        // Original unchanged
        assert_eq!(v.get(1), Some(&2));
        // New version updated
        assert_eq!(v2.get(1), Some(&99));
        assert_eq!(v2.get(0), Some(&1));
        assert_eq!(v2.get(2), Some(&3));
    }

    #[test]
    fn vec_set_out_of_bounds() {
        let v = PersistentVec::new().push(1);
        let v2 = v.set(100, 42);
        assert_eq!(v2.len(), 1); // unchanged
    }

    #[test]
    fn vec_iter_order() {
        let v = (0..10i32).fold(PersistentVec::new(), |acc, x| acc.push(x));
        let collected: Vec<i32> = v.iter().copied().collect();
        assert_eq!(collected, (0..10).collect::<Vec<i32>>());
    }

    #[test]
    fn vec_large_push() {
        let count = 100usize;
        let v = (0..count).fold(PersistentVec::new(), |acc, x| acc.push(x));
        assert_eq!(v.len(), count);
        for i in 0..count {
            assert_eq!(v.get(i), Some(&i));
        }
    }

    #[test]
    fn vec_multiple_versions_coexist() {
        let v0: PersistentVec<u32> = PersistentVec::new();
        let v1 = v0.push(1);
        let v2 = v1.push(2);
        let v3 = v2.push(3);
        // All versions independent
        assert_eq!(v0.len(), 0);
        assert_eq!(v1.len(), 1);
        assert_eq!(v2.len(), 2);
        assert_eq!(v3.len(), 3);
    }

    // ── PersistentMap tests ───────────────────────────────────────────────────

    #[test]
    fn map_new_is_empty() {
        let m: PersistentMap<String, i32> = PersistentMap::new();
        assert_eq!(m.len(), 0);
        assert!(m.is_empty());
    }

    #[test]
    fn map_insert_and_get() {
        let m = PersistentMap::new()
            .insert("a".to_string(), 1)
            .insert("b".to_string(), 2)
            .insert("c".to_string(), 3);
        assert_eq!(m.get(&"a".to_string()), Some(&1));
        assert_eq!(m.get(&"b".to_string()), Some(&2));
        assert_eq!(m.get(&"c".to_string()), Some(&3));
        assert_eq!(m.get(&"d".to_string()), None);
    }

    #[test]
    fn map_persistence_after_insert() {
        let m0 = PersistentMap::new().insert(1u32, "one".to_string());
        let m1 = m0.insert(2u32, "two".to_string());
        // m0 unchanged
        assert_eq!(m0.len(), 1);
        assert_eq!(m0.get(&2), None);
        // m1 has both
        assert_eq!(m1.len(), 2);
        assert_eq!(m1.get(&1), Some(&"one".to_string()));
    }

    #[test]
    fn map_update_existing_key() {
        let m0 = PersistentMap::new().insert("x".to_string(), 10i32);
        let m1 = m0.insert("x".to_string(), 20);
        assert_eq!(m1.len(), 1);
        assert_eq!(m1.get(&"x".to_string()), Some(&20));
        // m0 still has original value
        assert_eq!(m0.get(&"x".to_string()), Some(&10));
    }

    #[test]
    fn map_remove() {
        let m = PersistentMap::new()
            .insert(1u32, "a".to_string())
            .insert(2u32, "b".to_string());
        let m2 = m.remove(&1);
        assert_eq!(m2.len(), 1);
        assert_eq!(m2.get(&1), None);
        assert_eq!(m2.get(&2), Some(&"b".to_string()));
        // Original unchanged
        assert_eq!(m.get(&1), Some(&"a".to_string()));
    }

    #[test]
    fn map_contains_key() {
        let m = PersistentMap::new().insert(42u32, true);
        assert!(m.contains_key(&42));
        assert!(!m.contains_key(&0));
    }

    #[test]
    fn map_large_insert() {
        let m = (0u32..50).fold(PersistentMap::new(), |acc, i| acc.insert(i, i * i));
        assert_eq!(m.len(), 50);
        for i in 0u32..50 {
            assert_eq!(m.get(&i), Some(&(i * i)));
        }
    }

    // ── PersistentSet tests ───────────────────────────────────────────────────

    #[test]
    fn set_new_is_empty() {
        let s: PersistentSet<i32> = PersistentSet::new();
        assert!(s.is_empty());
    }

    #[test]
    fn set_insert_and_contains() {
        let s = PersistentSet::new().insert(1i32).insert(2).insert(3);
        assert!(s.contains(&1));
        assert!(s.contains(&2));
        assert!(!s.contains(&4));
    }

    #[test]
    fn set_persistence_after_insert() {
        let s0 = PersistentSet::new().insert(10i32);
        let s1 = s0.insert(20);
        assert!(!s0.contains(&20));
        assert!(s1.contains(&10));
        assert!(s1.contains(&20));
    }

    #[test]
    fn set_remove() {
        let s = PersistentSet::new().insert(1i32).insert(2).insert(3);
        let s2 = s.remove(&2);
        assert!(!s2.contains(&2));
        assert!(s2.contains(&1));
        assert!(s2.contains(&3));
        // Original unchanged
        assert!(s.contains(&2));
    }

    #[test]
    fn set_union() {
        let s1 = PersistentSet::new().insert(1i32).insert(2);
        let s2 = PersistentSet::new().insert(2i32).insert(3);
        let u = s1.union(&s2);
        assert!(u.contains(&1));
        assert!(u.contains(&2));
        assert!(u.contains(&3));
        assert_eq!(u.len(), 3);
    }

    #[test]
    fn set_intersection() {
        let s1 = PersistentSet::new().insert(1i32).insert(2).insert(3);
        let s2 = PersistentSet::new().insert(2i32).insert(3).insert(4);
        let inter = s1.intersection(&s2);
        assert!(!inter.contains(&1));
        assert!(inter.contains(&2));
        assert!(inter.contains(&3));
        assert!(!inter.contains(&4));
        assert_eq!(inter.len(), 2);
    }

    // ── PersistentQueue tests ─────────────────────────────────────────────────

    #[test]
    fn queue_new_is_empty() {
        let q: PersistentQueue<i32> = PersistentQueue::new();
        assert!(q.is_empty());
        assert_eq!(q.peek(), None);
    }

    #[test]
    fn queue_push_and_pop() {
        let q = PersistentQueue::new()
            .push_back(1i32)
            .push_back(2)
            .push_back(3);
        let (v, q2) = q.pop_front().expect("non-empty");
        assert_eq!(v, 1);
        let (v2, q3) = q2.pop_front().expect("non-empty");
        assert_eq!(v2, 2);
        let (v3, q4) = q3.pop_front().expect("non-empty");
        assert_eq!(v3, 3);
        assert!(q4.is_empty());
    }

    #[test]
    fn queue_persistence() {
        let q0 = PersistentQueue::new().push_back(10i32).push_back(20);
        let (_, q1) = q0.pop_front().expect("non-empty");
        // q0 still has 2 elements
        assert_eq!(q0.len(), 2);
        assert_eq!(q0.peek(), Some(&10));
        // q1 has 1 element
        assert_eq!(q1.len(), 1);
        assert_eq!(q1.peek(), Some(&20));
    }

    #[test]
    fn queue_pop_empty() {
        let q: PersistentQueue<i32> = PersistentQueue::new();
        assert!(q.pop_front().is_none());
    }

    #[test]
    fn queue_fifo_order() {
        let q = (0..5i32).fold(PersistentQueue::new(), |acc, x| acc.push_back(x));
        let mut current = q;
        for expected in 0..5i32 {
            let (val, next) = current.pop_front().expect("non-empty");
            assert_eq!(val, expected);
            current = next;
        }
        assert!(current.is_empty());
    }

    // ── PersistentStack tests ─────────────────────────────────────────────────

    #[test]
    fn stack_new_is_empty() {
        let s: PersistentStack<i32> = PersistentStack::new();
        assert!(s.is_empty());
        assert_eq!(s.peek(), None);
    }

    #[test]
    fn stack_push_and_pop() {
        let s = PersistentStack::new().push(1i32).push(2).push(3);
        assert_eq!(s.peek(), Some(&3));
        let (v, s2) = s.pop().expect("non-empty");
        assert_eq!(v, 3);
        let (v2, s3) = s2.pop().expect("non-empty");
        assert_eq!(v2, 2);
        let (v3, s4) = s3.pop().expect("non-empty");
        assert_eq!(v3, 1);
        assert!(s4.is_empty());
    }

    #[test]
    fn stack_persistence() {
        let s0 = PersistentStack::new().push(10i32).push(20);
        let (_, s1) = s0.pop().expect("non-empty");
        // s0 still intact
        assert_eq!(s0.len(), 2);
        assert_eq!(s0.peek(), Some(&20));
        // s1 has one element
        assert_eq!(s1.len(), 1);
        assert_eq!(s1.peek(), Some(&10));
    }

    #[test]
    fn stack_pop_empty() {
        let s: PersistentStack<i32> = PersistentStack::new();
        assert!(s.pop().is_none());
    }

    #[test]
    fn stack_lifo_order() {
        let s = (0..5i32).fold(PersistentStack::new(), |acc, x| acc.push(x));
        let mut current = s;
        for expected in (0..5i32).rev() {
            let (val, next) = current.pop().expect("non-empty");
            assert_eq!(val, expected);
            current = next;
        }
        assert!(current.is_empty());
    }

    #[test]
    fn stack_multiple_branches_from_same_base() {
        let base = PersistentStack::new().push(1i32).push(2);
        let branch_a = base.push(10);
        let branch_b = base.push(20);
        assert_eq!(branch_a.peek(), Some(&10));
        assert_eq!(branch_b.peek(), Some(&20));
        // base unchanged
        assert_eq!(base.peek(), Some(&2));
    }
}
