//! Tests pinning where a bump arena places an allocation that needs a new
//! chunk, and that a `ScopedArena` hands its arena back to the pool whole.

use super::super::functions::{DEFAULT_CHUNK_SIZE, MIN_CHUNK_SIZE};
use super::super::types::{ArenaPool, BumpArena, ScopedArena};

#[test]
fn oversized_allocation_starts_a_new_chunk_at_offset_zero() {
    let mut arena = BumpArena::with_chunk_size(MIN_CHUNK_SIZE);
    let small = arena.alloc_aligned(16, 8);
    assert_eq!((small.chunk, small.offset), (0, 0));
    let big = arena.alloc_aligned(MIN_CHUNK_SIZE * 3, 64);
    assert_eq!((big.chunk, big.offset), (1, 0));
    assert_eq!(arena.num_chunks(), 2);
    assert_eq!(arena.bytes_used(), 16 + MIN_CHUNK_SIZE * 3);
    assert_eq!(
        arena.total_capacity(),
        MIN_CHUNK_SIZE + MIN_CHUNK_SIZE * 3 + 64
    );
    assert_eq!(arena.stats().total_chunks_allocated, 1);
    assert_eq!(arena.stats().total_allocations, 2);
    let next = arena.alloc_aligned(8, 8);
    assert_eq!((next.chunk, next.offset), (1, MIN_CHUNK_SIZE * 3));
    let full = arena.alloc_aligned(MIN_CHUNK_SIZE, 8);
    assert_eq!((full.chunk, full.offset), (2, 0));
    assert_eq!(
        arena.total_capacity(),
        MIN_CHUNK_SIZE + MIN_CHUNK_SIZE * 3 + 64 + MIN_CHUNK_SIZE
    );
}

#[test]
fn scoped_arena_returns_its_chunks_and_statistics_to_the_pool() {
    let mut pool = ArenaPool::new();
    {
        let mut scoped = ScopedArena::new(&mut pool);
        let first = scoped.alloc(32);
        assert_eq!((first.chunk, first.offset), (0, 0));
        let big = scoped.arena_mut().alloc(DEFAULT_CHUNK_SIZE * 2);
        assert_eq!((big.chunk, big.offset), (1, 0));
        assert_eq!(scoped.arena().num_chunks(), 2);
        assert_eq!(scoped.arena().stats().total_allocations, 2);
    }
    assert_eq!(pool.available_count(), 1);
    let reused = pool.acquire();
    assert_eq!(reused.num_chunks(), 2);
    assert_eq!(reused.bytes_used(), 0);
    assert_eq!(reused.stats().total_allocations, 2);
    assert_eq!(reused.stats().total_resets, 1);
}
