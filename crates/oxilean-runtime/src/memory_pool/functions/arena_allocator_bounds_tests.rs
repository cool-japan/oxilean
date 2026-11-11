//! Tests for `ArenaAllocator::alloc_bytes` at the edges of `usize`, and for
//! the placement of ordinary allocations.

use super::super::types::ArenaAllocator;

#[test]
fn alignment_that_overflows_the_request_is_refused() {
    let mut arena = ArenaAllocator::new(64);
    assert!(arena.alloc_bytes(2, usize::MAX).is_none());
    assert_eq!(arena.chunk_count(), 0);
    assert_eq!(arena.bytes_allocated(), 0);
    assert!(arena.alloc_bytes(16, 8).is_some());
    assert!(arena.alloc_bytes(2, usize::MAX).is_none());
    assert_eq!(arena.chunk_count(), 1);
    assert_eq!(arena.bytes_allocated(), 16);
}

#[test]
fn size_that_overflows_the_current_chunk_offset_is_refused() {
    let mut arena = ArenaAllocator::new(64);
    assert!(arena.alloc_bytes(8, 8).is_some());
    assert!(arena.alloc_bytes(usize::MAX - 2, 8).is_none());
    assert_eq!(arena.chunk_count(), 1);
    assert_eq!(arena.bytes_allocated(), 8);
}

#[test]
fn allocations_are_aligned_disjoint_and_counted() {
    let mut arena = ArenaAllocator::new(64);
    let requests = [
        (1usize, 1usize),
        (3, 2),
        (8, 8),
        (5, 16),
        (64, 64),
        (100, 8),
        (7, 4096),
        (1, 1),
        (200, 32),
    ];
    let mut spans: Vec<(usize, usize)> = Vec::new();
    for &(size, align) in &requests {
        let ptr = arena
            .alloc_bytes(size, align)
            .expect("an ordinary request is served");
        let addr = ptr.as_ptr() as usize;
        assert_eq!(addr % align, 0, "size {size} align {align}");
        for &(start, len) in &spans {
            assert!(addr + size <= start || start + len <= addr);
        }
        spans.push((addr, size));
    }
    let total: usize = requests.iter().map(|&(size, _)| size).sum();
    assert_eq!(arena.bytes_allocated(), total);
    assert!(arena.bytes_reserved() >= total);
}
