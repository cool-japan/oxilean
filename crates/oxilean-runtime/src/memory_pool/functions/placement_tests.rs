//! Where `PoolAllocator` and `ArenaAllocator` place values: a type aligned
//! to 4096 bytes, a type with a destructor whose slot is freed and reused,
//! an aligned zero-sized type, and byte regions at alignments that are and
//! are not powers of two. Every value is written and read back, so a run
//! under Miri checks each access against its allocation.

use super::super::types::{ArenaAllocator, PoolAllocator, PoolConfig};

/// A value aligned to 4096 bytes.
#[repr(align(4096))]
struct Page([u8; 4096]);

#[test]
fn pool_slots_of_a_page_aligned_type_are_aligned_and_hold_their_values() {
    let mut pool: PoolAllocator<Page> =
        PoolAllocator::with_config(PoolConfig::default().with_block_size(3));
    let mut ptrs = Vec::new();
    for i in 0..7u8 {
        let p = pool
            .allocate(Page([i; 4096]))
            .expect("allocation should succeed");
        assert_eq!(p.as_ptr() as usize % 4096, 0);
        ptrs.push(p);
    }
    for (i, p) in (0u8..).zip(&ptrs) {
        // SAFETY: `p` was returned by `allocate` on `pool`, which wrote
        // `Page([i; 4096])` into its slot; nothing has been deallocated and
        // the pool has not been reset, cleared or dropped since.
        let page = unsafe { &*p.as_ptr() };
        assert_eq!((page.0[0], page.0[4095]), (i, i));
    }
    for p in ptrs.drain(..) {
        // SAFETY: `p` was returned by `allocate` on this pool, each pointer
        // is deallocated once, and the pool was not reset or cleared since.
        unsafe { pool.deallocate(p) };
    }
    let p = pool
        .allocate(Page([9; 4096]))
        .expect("a freed slot is reused");
    assert_eq!(p.as_ptr() as usize % 4096, 0);
    // SAFETY: `p` was just returned by `allocate` on `pool`, which wrote
    // `Page([9; 4096])` into a freed slot; the pool is untouched until this
    // read.
    assert_eq!(unsafe { (*p.as_ptr()).0[4095] }, 9);
    // SAFETY: `p` was returned by `allocate` on this pool, is deallocated
    // once, and the pool was not reset or cleared since.
    unsafe { pool.deallocate(p) };
}

#[test]
fn pool_reuses_the_slot_of_a_deallocated_value_with_a_destructor() {
    let mut pool: PoolAllocator<String> =
        PoolAllocator::with_config(PoolConfig::default().with_block_size(2));
    let a = pool
        .allocate(String::from("a"))
        .expect("allocation should succeed");
    let b = pool
        .allocate(String::from("bb"))
        .expect("allocation should succeed");
    let c = pool
        .allocate(String::from("ccc"))
        .expect("allocation should succeed");
    // SAFETY: `b` was returned by `allocate` on `pool`, which wrote "bb"
    // into its slot; nothing has been deallocated and the pool has not been
    // reset or cleared since.
    assert_eq!(unsafe { &*b.as_ptr() }, "bb");
    // SAFETY: `b` was returned by `allocate` on this pool, is deallocated
    // once, and is not used again; the pool was not reset or cleared since.
    unsafe { pool.deallocate(b) };
    let d = pool
        .allocate(String::from("dddd"))
        .expect("a freed slot is reused");
    // SAFETY: `d` was just returned by `allocate` on `pool`, which wrote
    // "dddd" into a free slot; the pool is untouched until this read.
    assert_eq!(unsafe { &*d.as_ptr() }, "dddd");
    for p in [a, c, d] {
        // SAFETY: `a`, `c` and `d` were returned by `allocate` on this pool
        // and are not used after this loop; each is deallocated once, and
        // the pool was not reset or cleared since.
        unsafe { pool.deallocate(p) };
    }
    assert_eq!(pool.allocated(), 0);
}

#[test]
fn arena_values_of_a_page_aligned_type_and_of_an_aligned_zero_sized_type() {
    let mut arena = ArenaAllocator::new(64);
    for i in 0..20u8 {
        let small = arena.alloc_value(i).expect("allocation should succeed");
        let page = arena
            .alloc_value(Page([i; 4096]))
            .expect("allocation should succeed");
        assert_eq!(page.as_ptr() as usize % 4096, 0);
        // SAFETY: `alloc_value` wrote `i` at `small` and `Page([i; 4096])`
        // at `page`, each aligned for its type inside a chunk `arena` owns;
        // the later allocation moved past the earlier one, and the arena is
        // not reset or dropped before these reads.
        let (small, page) = unsafe { (*small.as_ptr(), &*page.as_ptr()) };
        assert_eq!((small, page.0[0], page.0[4095]), (i, i, i));
    }
    #[repr(align(8))]
    struct Aligned8;
    let z = arena
        .alloc_value(Aligned8)
        .expect("allocation should succeed");
    assert_eq!(z.as_ptr() as usize % 8, 0);
    assert!(arena.alloc_value(()).is_some());
}

#[test]
fn arena_byte_regions_lie_inside_their_chunk_at_every_alignment() {
    let mut arena = ArenaAllocator::new(64);
    for align in [0usize, 1, 2, 3, 4, 5, 7, 8, 16, 33, 64, 4096] {
        for size in [1usize, 2, 3, 7, 63, 64, 65, 200] {
            let p = arena
                .alloc_bytes(size, align)
                .expect("allocation should succeed");
            // SAFETY: `alloc_bytes` returned the start of `size` bytes inside
            // a chunk `arena` owns (its own `SAFETY:` comments establish the
            // bound for every alignment), which no other allocation overlaps
            // and which stays allocated until `reset` or the arena's drop;
            // `u8` needs no alignment.
            unsafe { std::ptr::write_bytes(p.as_ptr(), 0xAB, size) };
        }
    }
    arena.reset();
    let p = arena
        .alloc_bytes(100, 64)
        .expect("allocation should succeed");
    // SAFETY: as in the loop above; `reset` invalidated the earlier regions,
    // none of which is used again.
    unsafe { std::ptr::write_bytes(p.as_ptr(), 3, 100) };
}
