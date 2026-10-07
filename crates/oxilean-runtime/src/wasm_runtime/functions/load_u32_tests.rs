//! Tests pinning `WasmMemory::load_u32` at the end of memory and at the
//! edge of `usize`.

use super::super::types::WasmMemory;

#[test]
fn loads_little_endian_words_up_to_the_last_full_word() {
    let mut mem = WasmMemory::new(1);
    let last = WasmMemory::PAGE_SIZE - 4;
    assert!(mem.store_u32(last, 0x0102_0304));
    assert_eq!(mem.load_u32(last), Some(0x0102_0304));
    assert_eq!(mem.load_bytes(last, 4), Some(&[4u8, 3, 2, 1][..]));
    assert!(mem.store_u32(1, 0xA1B2_C3D4));
    assert_eq!(mem.load_u32(1), Some(0xA1B2_C3D4));
}

#[test]
fn refuses_words_that_end_past_memory_or_overflow() {
    let mem = WasmMemory::new(1);
    assert_eq!(mem.load_u32(WasmMemory::PAGE_SIZE - 3), None);
    assert_eq!(mem.load_u32(WasmMemory::PAGE_SIZE), None);
    assert_eq!(mem.load_u32(WasmMemory::PAGE_SIZE + 1), None);
    assert_eq!(mem.load_u32(usize::MAX - 2), None);
    assert_eq!(mem.load_u32(usize::MAX), None);
    let empty = WasmMemory::new(0);
    assert_eq!(empty.load_u32(0), None);
}
