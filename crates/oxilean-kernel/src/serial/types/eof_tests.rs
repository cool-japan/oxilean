//! The fixed-width reads of `OleanReader` and `CheckpointedReader`: exact
//! little-endian values up to the last byte, `UnexpectedEof` one byte short.

use super::{CheckpointedReader, OleanError, OleanReader};

#[test]
fn fixed_width_reads_succeed_to_the_end_and_fail_one_byte_short() {
    let bytes: Vec<u8> = (1..=20).collect();
    for len in 0..=bytes.len() {
        let data = &bytes[..len];

        let mut r = OleanReader::new(data);
        match r.read_u32() {
            Ok(v) => assert_eq!((len >= 4, v), (true, 0x0403_0201)),
            Err(e) => assert!(len < 4 && matches!(e, OleanError::UnexpectedEof)),
        }
        let mut r = OleanReader::new(data);
        match r.read_u64() {
            Ok(v) => assert_eq!((len >= 8, v), (true, 0x0807_0605_0403_0201)),
            Err(e) => assert!(len < 8 && matches!(e, OleanError::UnexpectedEof)),
        }
        let mut r = OleanReader::new(data);
        match r.read_i64() {
            Ok(v) => assert_eq!((len >= 8, v), (true, 0x0807_0605_0403_0201)),
            Err(e) => assert!(len < 8 && matches!(e, OleanError::UnexpectedEof)),
        }
        let mut c = CheckpointedReader::new(data);
        match c.read_u32() {
            Ok(v) => assert_eq!((len >= 4, v, c.remaining()), (true, 0x0403_0201, len - 4)),
            Err(e) => assert!(len < 4 && matches!(e, OleanError::UnexpectedEof)),
        }
    }

    // Consecutive reads advance by the width read; a short read does not.
    let mut r = OleanReader::new(&bytes);
    assert!(matches!(r.read_u32(), Ok(0x0403_0201)));
    assert!(matches!(r.read_u64(), Ok(0x0c0b_0a09_0807_0605)));
    assert!(matches!(r.read_u32(), Ok(0x100f_0e0d)));
    assert!(matches!(r.read_i64(), Err(OleanError::UnexpectedEof)));
    assert!(matches!(r.read_u32(), Ok(0x1413_1211)));
    assert!(matches!(r.read_u32(), Err(OleanError::UnexpectedEof)));
}
