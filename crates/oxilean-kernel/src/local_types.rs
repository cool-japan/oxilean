//! The types of the free variables in scope, as the reducer and the def-eq
//! checker see them.
//!
//! K-like iota reduction, structure eta and proof irrelevance have to TYPE
//! terms that mention local free variables, so the reducer and the def-eq
//! checker each keep a mirror of the surrounding checker's local context: the
//! type of every free variable, by id. Lean's kernel reads the same
//! information from its local context.
//!
//! # Scope
//!
//! A variable is in the mirror from the moment its binder is opened
//! ([`LocalTypes::bind`]) until the binder is closed
//! ([`LocalTypes::unbind`]). Through 0.1.3 nothing was ever removed: every
//! binder opened while checking a declaration stayed in the mirrors until
//! the checker was dropped — in a replay of a Mathlib slice, up to 16,745
//! entries in one mirror for one declaration, none of them still in scope —
//! and every K-like or structure-eta attempt copied all of them into a
//! nested checker.
//!
//! # What outlives a binder
//!
//! Two facts about a closed variable still matter, and both are kept in
//! space bounded by the depth of the context, not by the number of binders
//! opened:
//!
//! * **Its id was used.** Cached reductions and def-eq verdicts may mention a
//!   closed variable. They are never looked up again as long as the id is
//!   never bound again — and if it IS bound again, with whatever type, they
//!   must not be reused. [`LocalTypes::bind`] reports that case and the owner
//!   drops its caches. (Ids are dispensed monotonically, so the kernel's own
//!   checking never re-binds one; the public `TypeChecker::push_local` can.)
//!   The closed ids are kept as ranges: binders close in stack order, so the
//!   ids closed so far are a handful of contiguous runs.
//! * **Its id is not free for a nested checker.** A checker seeded with these
//!   locals mints its own variables past every id this mirror has EVER held
//!   ([`LocalTypes::max_own_id`], [`LocalTypes::max_opened_id`]) — not past
//!   the live ones only — so the variables it mints are distinct from every
//!   variable the surrounding checker has used, in scope or not. That is the
//!   guarantee the nested K guard depends on (S12), and removing entries
//!   leaves it exactly as it was.

use crate::def_eq::types::DEF_EQ_OPENED_FVAR_BASE;
use crate::{Expr, FVarId};
use std::collections::HashMap;

/// The types of the free variables currently in scope, by id. See the
/// module documentation.
#[derive(Default)]
pub(crate) struct LocalTypes {
    /// Variables whose binder is open.
    live: HashMap<u64, Expr>,
    /// Ids that were bound and have since been unbound.
    closed: IdRanges,
    /// The largest id ever bound below [`DEF_EQ_OPENED_FVAR_BASE`] (a
    /// checker's own locals), in scope or not.
    max_own: Option<u64>,
    /// The largest id ever bound at or above [`DEF_EQ_OPENED_FVAR_BASE`]
    /// (binders a def-eq checker opened), in scope or not.
    max_opened: Option<u64>,
}

impl LocalTypes {
    /// An empty mirror.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Record `ty` as the type of the variable `id`, which is now in scope.
    ///
    /// Returns `true` when results computed earlier that mention `id` may no
    /// longer hold, and the owner must drop its caches:
    ///
    /// * `id` is in scope with a DIFFERENT type; or
    /// * `id` was in scope before and its binder has been closed — its
    ///   earlier type is no longer known, so any re-binding counts.
    ///
    /// Binding an id for the first time, or binding an in-scope id to the
    /// type it already has, returns `false`.
    #[must_use = "a `true` result means the owner's caches are stale"]
    pub(crate) fn bind(&mut self, id: FVarId, ty: Expr) -> bool {
        let stale = match self.live.get(&id.0) {
            Some(previous) => *previous != ty,
            None => self.closed.contains(id.0),
        };
        let max = if id.0 < DEF_EQ_OPENED_FVAR_BASE {
            &mut self.max_own
        } else {
            &mut self.max_opened
        };
        if max.is_none_or(|m| id.0 > m) {
            *max = Some(id.0);
        }
        self.live.insert(id.0, ty);
        stale
    }

    /// The binder of `id` has been closed: forget its type. Nothing typed
    /// from here on mentions the variable. A no-op for an id not in scope.
    pub(crate) fn unbind(&mut self, id: FVarId) {
        if self.live.remove(&id.0).is_some() {
            self.closed.insert(id.0);
        }
    }

    /// The type of `id`, if it is in scope.
    pub(crate) fn get(&self, id: FVarId) -> Option<&Expr> {
        self.live.get(&id.0)
    }

    /// The variables in scope, with their types.
    pub(crate) fn live(&self) -> &HashMap<u64, Expr> {
        &self.live
    }

    /// The largest id ever bound below [`DEF_EQ_OPENED_FVAR_BASE`], whether
    /// or not it is still in scope.
    pub(crate) fn max_own_id(&self) -> Option<u64> {
        self.max_own
    }

    /// The largest id ever bound at or above [`DEF_EQ_OPENED_FVAR_BASE`],
    /// whether or not it is still in scope.
    pub(crate) fn max_opened_id(&self) -> Option<u64> {
        self.max_opened
    }

    /// How many ranges the closed ids are stored as (for tests and
    /// diagnostics).
    #[cfg(test)]
    pub(crate) fn closed_range_count(&self) -> usize {
        self.closed.ranges.len()
    }
}

/// A set of ids, stored as inclusive ranges: ascending, disjoint, and with a
/// gap between any two (adjacent ranges are merged on insertion).
#[derive(Default)]
struct IdRanges {
    ranges: Vec<(u64, u64)>,
}

impl IdRanges {
    /// The end of the last range that starts at or before `id`, and the
    /// index of the first range that starts after it.
    fn locate(&self, id: u64) -> (Option<u64>, usize) {
        let next = self.ranges.partition_point(|&(start, _)| start <= id);
        let previous_end = next
            .checked_sub(1)
            .and_then(|previous| self.ranges.get(previous))
            .map(|&(_, end)| end);
        (previous_end, next)
    }

    fn contains(&self, id: u64) -> bool {
        let (previous_end, _) = self.locate(id);
        previous_end.is_some_and(|end| id <= end)
    }

    fn insert(&mut self, id: u64) {
        let (previous_end, next) = self.locate(id);
        if previous_end.is_some_and(|end| id <= end) {
            return;
        }
        let joins_previous = previous_end.is_some_and(|end| end.checked_add(1) == Some(id));
        let next_start = self.ranges.get(next).map(|&(start, _)| start);
        let joins_next = next_start.is_some_and(|start| id.checked_add(1) == Some(start));
        match (joins_previous, joins_next) {
            (true, true) => {
                // `id` fills the one-id gap between two ranges: merge them.
                let (_, next_end) = self.ranges.remove(next);
                if let Some(previous) = next.checked_sub(1).and_then(|p| self.ranges.get_mut(p)) {
                    previous.1 = next_end;
                }
            }
            (true, false) => {
                if let Some(previous) = next.checked_sub(1).and_then(|p| self.ranges.get_mut(p)) {
                    previous.1 = id;
                }
            }
            (false, true) => {
                if let Some(following) = self.ranges.get_mut(next) {
                    following.0 = id;
                }
            }
            (false, false) => self.ranges.insert(next, (id, id)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Level, Name};

    fn ty(name: &str) -> Expr {
        Expr::Const(Name::str(name), vec![])
    }

    /// Reference model for [`IdRanges`].
    fn check_against_set(ops: &[u64]) {
        let mut ranges = IdRanges::default();
        let mut set = std::collections::BTreeSet::new();
        for &id in ops {
            ranges.insert(id);
            set.insert(id);
            // Invariant: ascending, disjoint, a gap between neighbours.
            for pair in ranges.ranges.windows(2) {
                assert!(
                    pair[0].0 <= pair[0].1,
                    "inverted range in {:?}",
                    ranges.ranges
                );
                assert!(
                    pair[0]
                        .1
                        .checked_add(1)
                        .is_some_and(|after| after < pair[1].0),
                    "ranges touch or overlap: {:?}",
                    ranges.ranges
                );
            }
            // Same members as the set.
            let covered: u64 = ranges.ranges.iter().map(|&(s, e)| e - s + 1).sum();
            assert_eq!(
                covered,
                set.len() as u64,
                "{:?} vs {:?}",
                ranges.ranges,
                set
            );
            for &member in &set {
                assert!(
                    ranges.contains(member),
                    "{member} missing from {:?}",
                    ranges.ranges
                );
            }
        }
        // Non-members around every member.
        for &member in &set {
            for probe in [member.wrapping_sub(1), member.wrapping_add(1)] {
                assert_eq!(
                    ranges.contains(probe),
                    set.contains(&probe),
                    "probe {probe} in {:?}",
                    ranges.ranges
                );
            }
        }
    }

    #[test]
    fn id_ranges_match_a_set_under_every_insertion_order() {
        // Every permutation of six ids with gaps, runs and both ends of `u64`.
        let ids = [0u64, 1, 2, 4, 5, u64::MAX];
        let mut order: Vec<usize> = (0..ids.len()).collect();
        // Heap's algorithm, iterative.
        let mut counters = vec![0usize; ids.len()];
        let ops: Vec<u64> = order.iter().map(|&i| ids[i]).collect();
        check_against_set(&ops);
        let mut i = 0;
        let mut permutations = 1;
        while i < ids.len() {
            if counters[i] < i {
                if i % 2 == 0 {
                    order.swap(0, i);
                } else {
                    order.swap(counters[i], i);
                }
                let ops: Vec<u64> = order.iter().map(|&k| ids[k]).collect();
                check_against_set(&ops);
                permutations += 1;
                counters[i] += 1;
                i = 0;
            } else {
                counters[i] = 0;
                i += 1;
            }
        }
        assert_eq!(permutations, 720);
        // Repeats, and the top of the range next to its neighbour.
        check_against_set(&[3, 3, 2, 4, 3, u64::MAX, u64::MAX - 1, u64::MAX]);
    }

    #[test]
    fn id_ranges_stay_small_when_binders_close_in_stack_order() {
        // Open ids 0..depth, then repeatedly open and close a binder at the
        // top: the closed ids are one run however many binders were opened.
        let mut locals = LocalTypes::new();
        let depth = 8u64;
        for id in 0..depth {
            assert!(!locals.bind(FVarId(id), ty("A")));
        }
        for id in depth..depth + 10_000 {
            assert!(!locals.bind(FVarId(id), ty("A")));
            locals.unbind(FVarId(id));
        }
        assert_eq!(locals.live().len(), depth as usize);
        assert_eq!(locals.closed_range_count(), 1);
        // Closing the rest, innermost first, extends that run downwards.
        for id in (0..depth).rev() {
            locals.unbind(FVarId(id));
            assert_eq!(locals.closed_range_count(), 1);
        }
        assert!(locals.live().is_empty());
    }

    #[test]
    fn bind_reports_a_conflicting_type_and_a_reused_id() {
        let mut locals = LocalTypes::new();
        let x = FVarId(7);
        // First binding, and the same type again: nothing is stale.
        assert!(!locals.bind(x, ty("A")));
        assert!(!locals.bind(x, ty("A")));
        assert_eq!(locals.get(x), Some(&ty("A")));
        // A different type for an id in scope.
        assert!(locals.bind(x, ty("B")));
        assert_eq!(locals.get(x), Some(&ty("B")));
        // Out of scope: the type is gone.
        locals.unbind(x);
        assert_eq!(locals.get(x), None);
        assert!(locals.live().is_empty());
        // Bound again — even to the type it last had — the id is a reuse.
        assert!(locals.bind(x, ty("B")));
        assert_eq!(locals.get(x), Some(&ty("B")));
        // In scope again with that type: binding it again changes nothing.
        assert!(!locals.bind(x, ty("B")));
        // An id never seen is not a reuse, wherever it falls.
        assert!(!locals.bind(FVarId(6), ty("A")));
        assert!(!locals.bind(FVarId(8), ty("A")));
        // Unbinding an id that is not in scope records nothing.
        locals.unbind(FVarId(100));
        assert!(!locals.bind(FVarId(100), ty("A")));
    }

    #[test]
    fn the_largest_ids_ever_bound_survive_unbinding() {
        let mut locals = LocalTypes::new();
        assert_eq!(locals.max_own_id(), None);
        assert_eq!(locals.max_opened_id(), None);
        let own = [3u64, 9, 5];
        let opened = [
            DEF_EQ_OPENED_FVAR_BASE,
            DEF_EQ_OPENED_FVAR_BASE + 4,
            DEF_EQ_OPENED_FVAR_BASE + 2,
        ];
        for id in own.iter().chain(opened.iter()) {
            let _ = locals.bind(FVarId(*id), Expr::Sort(Level::zero()));
        }
        assert_eq!(locals.max_own_id(), Some(9));
        assert_eq!(locals.max_opened_id(), Some(DEF_EQ_OPENED_FVAR_BASE + 4));
        for id in own.iter().chain(opened.iter()) {
            locals.unbind(FVarId(*id));
        }
        assert!(locals.live().is_empty());
        // Closed, but still not free for a nested checker to mint.
        assert_eq!(locals.max_own_id(), Some(9));
        assert_eq!(locals.max_opened_id(), Some(DEF_EQ_OPENED_FVAR_BASE + 4));
        // The boundary id belongs to the opened range.
        assert_eq!(DEF_EQ_OPENED_FVAR_BASE - 1, (1u64 << 62) - 1);
        let _ = locals.bind(FVarId(DEF_EQ_OPENED_FVAR_BASE - 1), ty("A"));
        assert_eq!(locals.max_own_id(), Some(DEF_EQ_OPENED_FVAR_BASE - 1));
        assert_eq!(locals.max_opened_id(), Some(DEF_EQ_OPENED_FVAR_BASE + 4));
    }
}
