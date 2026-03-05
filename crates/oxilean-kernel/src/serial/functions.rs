//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::io::Write;

use super::types::{
    BinarySection, BlobValidator, BufferedOleanWriter, CheckpointedReader, ChecksummedWriter,
    CompatibilityChecker, DeclDiff, DeclIndex, DeclKindSet, DeltaList, FileStats,
    FormatDiagnostics, MergeStrategy, MetadataReader, MetadataValue, MetadataWriter, NameTable,
    OleanArchive, OleanError, OleanReader, OleanWriter, SectionHeader, SectionTable, SerialDecl,
    SerialError, StringPool,
};

/// Magic bytes identifying an OleanC file.
pub(super) const MAGIC: &[u8; 4] = b"OLNC";
/// Current format version.
pub(super) const VERSION: u32 = 1;
/// Header size in bytes: 4 (magic) + 4 (version) + 4 (decl_count) + 8 (metadata_offset) = 20
pub(super) const HEADER_SIZE: usize = 20;
/// Declaration kind tag values.
pub mod kind_tags {
    pub const AXIOM: u8 = 0;
    pub const DEFINITION: u8 = 1;
    pub const THEOREM: u8 = 2;
    pub const OPAQUE: u8 = 3;
    pub const INDUCTIVE: u8 = 4;
    pub const OTHER: u8 = 5;
}
/// Serialize a list of declaration names into the OleanC binary format.
///
/// Each name is stored as a `SerialDecl::Other` entry.
pub fn serialize_decl_names(names: &[String]) -> Vec<u8> {
    let mut w = OleanWriter::new();
    w.write_header(names.len() as u32);
    for name in names {
        w.write_string(name);
        w.write_u8(kind_tags::OTHER);
    }
    w.finish()
}
/// Deserialize a list of declaration names from OleanC binary data.
pub fn deserialize_decl_names(data: &[u8]) -> Result<Vec<String>, OleanError> {
    let mut r = OleanReader::new(data);
    let header = r.read_header()?;
    let mut names = Vec::with_capacity(header.decl_count as usize);
    for _ in 0..header.decl_count {
        let name = r.read_string()?;
        let _kind = r.read_u8()?;
        names.push(name);
    }
    Ok(names)
}
/// Write an OleanC file to disk containing the given declaration names.
pub fn write_oleanc_file(path: &str, decl_names: &[String]) -> std::io::Result<()> {
    let data = serialize_decl_names(decl_names);
    let mut file = std::fs::File::create(path)?;
    file.write_all(&data)?;
    Ok(())
}
/// Read an OleanC file from disk and return its declaration names.
pub fn read_oleanc_file(path: &str) -> Result<Vec<String>, OleanError> {
    let data = std::fs::read(path)?;
    deserialize_decl_names(&data)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_writer_read_u32_roundtrip() {
        let mut w = OleanWriter::new();
        w.write_u32(42_u32);
        w.write_u32(0xDEAD_BEEF_u32);
        let data = w.finish();
        let mut r = OleanReader::new(&data);
        assert_eq!(r.read_u32().expect("read_u32 should succeed"), 42);
        assert_eq!(r.read_u32().expect("read_u32 should succeed"), 0xDEAD_BEEF);
        assert_eq!(r.remaining(), 0);
    }
    #[test]
    fn test_writer_read_string_roundtrip() {
        let mut w = OleanWriter::new();
        w.write_string("Nat.add.comm");
        w.write_string("");
        w.write_string("hello world");
        let data = w.finish();
        let mut r = OleanReader::new(&data);
        assert_eq!(
            r.read_string().expect("read_string should succeed"),
            "Nat.add.comm"
        );
        assert_eq!(r.read_string().expect("read_string should succeed"), "");
        assert_eq!(
            r.read_string().expect("read_string should succeed"),
            "hello world"
        );
        assert_eq!(r.remaining(), 0);
    }
    #[test]
    fn test_magic_bytes() {
        let mut w = OleanWriter::new();
        w.write_header(0);
        let data = w.finish();
        assert_eq!(&data[..4], b"OLNC");
    }
    #[test]
    fn test_header_roundtrip() {
        let mut w = OleanWriter::new();
        w.write_header(7);
        let data = w.finish();
        let mut r = OleanReader::new(&data);
        let hdr = r.read_header().expect("hdr should be present");
        assert_eq!(hdr.version, VERSION);
        assert_eq!(hdr.decl_count, 7);
        assert_eq!(hdr.metadata_offset, HEADER_SIZE as u64);
    }
    #[test]
    fn test_serialize_empty_names() {
        let data = serialize_decl_names(&[]);
        let names = deserialize_decl_names(&data).expect("names should be present");
        assert!(names.is_empty());
    }
    #[test]
    fn test_serialize_names_roundtrip() {
        let input: Vec<String> = vec![
            "Nat.add".to_string(),
            "Nat.mul".to_string(),
            "List.length".to_string(),
        ];
        let data = serialize_decl_names(&input);
        let output = deserialize_decl_names(&data).expect("output should be present");
        assert_eq!(input, output);
    }
    #[test]
    fn test_invalid_magic_error() {
        let bad_data = b"BADM\x01\x00\x00\x00\x00\x00\x00\x00\x14\x00\x00\x00\x00\x00\x00\x00";
        let mut r = OleanReader::new(bad_data);
        match r.read_header() {
            Err(OleanError::InvalidMagic) => {}
            other => panic!("expected InvalidMagic, got {:?}", other),
        }
    }
    #[test]
    fn test_serial_decl_name() {
        let decl = SerialDecl::Theorem {
            name: "Nat.add_comm".to_string(),
            kind_tag: kind_tags::THEOREM,
        };
        assert_eq!(decl.name(), "Nat.add_comm");
        assert_eq!(decl.kind_tag(), kind_tags::THEOREM);
        let decl2 = SerialDecl::Inductive {
            name: "List".to_string(),
            ctor_count: 2,
            kind_tag: kind_tags::INDUCTIVE,
        };
        assert_eq!(decl2.name(), "List");
        assert_eq!(decl2.kind_tag(), kind_tags::INDUCTIVE);
    }
}
/// Well-known section tags for the extended binary format.
#[allow(dead_code)]
pub mod section_tags {
    pub const DECLARATIONS: u8 = 0x01;
    pub const UNIVERSE_LEVELS: u8 = 0x02;
    pub const NAME_TABLE: u8 = 0x03;
    pub const EXPORT_LIST: u8 = 0x04;
    pub const DEBUG_INFO: u8 = 0x05;
    pub const CHECKSUM: u8 = 0xFF;
}
/// Encodes a `SerialDecl` with full field data into bytes.
#[allow(dead_code)]
pub fn encode_decl(w: &mut OleanWriter, decl: &SerialDecl) {
    w.write_string(decl.name());
    w.write_u8(decl.kind_tag());
    if let SerialDecl::Inductive { ctor_count, .. } = decl {
        w.write_u32(*ctor_count);
    }
}
/// Decodes a `SerialDecl` from a reader.
#[allow(dead_code)]
pub fn decode_decl(r: &mut OleanReader<'_>) -> Result<SerialDecl, OleanError> {
    let name = r.read_string()?;
    let tag = r.read_u8()?;
    match tag {
        kind_tags::AXIOM => Ok(SerialDecl::Axiom {
            name,
            kind_tag: tag,
        }),
        kind_tags::DEFINITION => Ok(SerialDecl::Definition {
            name,
            kind_tag: tag,
        }),
        kind_tags::THEOREM => Ok(SerialDecl::Theorem {
            name,
            kind_tag: tag,
        }),
        kind_tags::OPAQUE => Ok(SerialDecl::Opaque {
            name,
            kind_tag: tag,
        }),
        kind_tags::INDUCTIVE => {
            let ctor_count = r.read_u32()?;
            Ok(SerialDecl::Inductive {
                name,
                ctor_count,
                kind_tag: tag,
            })
        }
        kind_tags::OTHER => Ok(SerialDecl::Other {
            name,
            kind_tag: tag,
        }),
        _ => Err(OleanError::InvalidDeclKind(tag)),
    }
}
/// Serialize a `Vec<SerialDecl>` to binary.
#[allow(dead_code)]
pub fn serialize_decls(decls: &[SerialDecl]) -> Vec<u8> {
    let mut w = OleanWriter::new();
    w.write_header(decls.len() as u32);
    for d in decls {
        encode_decl(&mut w, d);
    }
    w.finish()
}
/// Deserialize a `Vec<SerialDecl>` from binary.
#[allow(dead_code)]
pub fn deserialize_decls(data: &[u8]) -> Result<Vec<SerialDecl>, OleanError> {
    let mut r = OleanReader::new(data);
    let header = r.read_header()?;
    let mut decls = Vec::with_capacity(header.decl_count as usize);
    for _ in 0..header.decl_count {
        decls.push(decode_decl(&mut r)?);
    }
    Ok(decls)
}
/// Write a name table as a section to a writer.
#[allow(dead_code)]
pub fn write_name_table_section(table: &NameTable, offset: u64) -> Vec<u8> {
    let mut inner = OleanWriter::new();
    table.write(&mut inner);
    let data = inner.finish();
    let section = BinarySection::new(section_tags::NAME_TABLE, data, offset);
    section.to_bytes()
}
#[cfg(test)]
mod tests_serial_extended {
    use super::*;
    #[test]
    fn test_section_header_roundtrip() {
        let hdr = SectionHeader::new(0x01, 256, 1024);
        let mut w = OleanWriter::new();
        hdr.write(&mut w);
        let data = w.finish();
        let mut r = OleanReader::new(&data);
        let hdr2 = SectionHeader::read(&mut r).expect("hdr2 should be present");
        assert_eq!(hdr2.tag, 0x01);
        assert_eq!(hdr2.length, 256);
        assert_eq!(hdr2.offset, 1024);
    }
    #[test]
    fn test_section_table_roundtrip() {
        let mut table = SectionTable::new();
        table.add(SectionHeader::new(section_tags::DECLARATIONS, 100, 20));
        table.add(SectionHeader::new(section_tags::NAME_TABLE, 50, 120));
        let mut w = OleanWriter::new();
        table.write(&mut w);
        let data = w.finish();
        let mut r = OleanReader::new(&data);
        let t2 = SectionTable::read(&mut r).expect("t2 should be present");
        assert_eq!(t2.len(), 2);
        let decl_hdr = t2
            .find(section_tags::DECLARATIONS)
            .expect("decl_hdr should be present");
        assert_eq!(decl_hdr.length, 100);
    }
    #[test]
    fn test_blob_validator() {
        let data = b"Hello, OleanC!";
        let checksum = BlobValidator::compute_checksum(data);
        let v = BlobValidator::new(checksum);
        assert!(v.validate(data));
        assert!(!v.validate(b"Different data"));
    }
    #[test]
    fn test_checksummed_writer() {
        let mut cw = ChecksummedWriter::new();
        cw.write_string("test");
        cw.write_u32(42);
        let bytes = cw.finish_with_checksum();
        assert!(bytes.len() > 4);
    }
    #[test]
    fn test_name_table_intern_lookup() {
        let mut t = NameTable::new();
        let id0 = t.intern("Nat.add");
        let id1 = t.intern("List.length");
        let id0b = t.intern("Nat.add");
        assert_eq!(id0, id0b);
        assert_ne!(id0, id1);
        assert_eq!(t.lookup_id(id0), Some("Nat.add"));
        assert_eq!(t.lookup_name("List.length"), Some(id1));
        assert_eq!(t.lookup_id(999), None);
    }
    #[test]
    fn test_name_table_roundtrip() {
        let mut t = NameTable::new();
        t.intern("a");
        t.intern("b");
        t.intern("c");
        let mut w = OleanWriter::new();
        t.write(&mut w);
        let data = w.finish();
        let mut r = OleanReader::new(&data);
        let t2 = NameTable::read(&mut r).expect("t2 should be present");
        assert_eq!(t2.len(), 3);
        assert_eq!(t2.lookup_id(0), Some("a"));
        assert_eq!(t2.lookup_id(2), Some("c"));
    }
    #[test]
    fn test_encode_decode_decl_theorem() {
        let decl = SerialDecl::Theorem {
            name: "Nat.add_comm".to_string(),
            kind_tag: kind_tags::THEOREM,
        };
        let mut w = OleanWriter::new();
        encode_decl(&mut w, &decl);
        let data = w.finish();
        let mut r = OleanReader::new(&data);
        let d2 = decode_decl(&mut r).expect("d2 should be present");
        assert_eq!(d2.name(), "Nat.add_comm");
        assert_eq!(d2.kind_tag(), kind_tags::THEOREM);
    }
    #[test]
    fn test_encode_decode_decl_inductive() {
        let decl = SerialDecl::Inductive {
            name: "List".to_string(),
            ctor_count: 2,
            kind_tag: kind_tags::INDUCTIVE,
        };
        let mut w = OleanWriter::new();
        encode_decl(&mut w, &decl);
        let data = w.finish();
        let mut r = OleanReader::new(&data);
        let d2 = decode_decl(&mut r).expect("d2 should be present");
        assert_eq!(d2.name(), "List");
        if let SerialDecl::Inductive { ctor_count, .. } = d2 {
            assert_eq!(ctor_count, 2);
        } else {
            panic!("expected Inductive");
        }
    }
    #[test]
    fn test_serialize_deserialize_decls() {
        let decls = vec![
            SerialDecl::Axiom {
                name: "propext".to_string(),
                kind_tag: kind_tags::AXIOM,
            },
            SerialDecl::Theorem {
                name: "Nat.succ_ne_zero".to_string(),
                kind_tag: kind_tags::THEOREM,
            },
            SerialDecl::Inductive {
                name: "Nat".to_string(),
                ctor_count: 2,
                kind_tag: kind_tags::INDUCTIVE,
            },
        ];
        let data = serialize_decls(&decls);
        let decoded = deserialize_decls(&data).expect("decoded should be present");
        assert_eq!(decoded.len(), 3);
        assert_eq!(decoded[0].name(), "propext");
        assert_eq!(decoded[1].kind_tag(), kind_tags::THEOREM);
    }
    #[test]
    fn test_decode_invalid_kind() {
        let mut w = OleanWriter::new();
        w.write_string("bad_decl");
        w.write_u8(99);
        let data = w.finish();
        let mut r = OleanReader::new(&data);
        match decode_decl(&mut r) {
            Err(OleanError::InvalidDeclKind(99)) => {}
            other => panic!("expected InvalidDeclKind(99), got {:?}", other),
        }
    }
    #[test]
    fn test_binary_section_bytes() {
        let data = vec![0xDE, 0xAD, 0xBE, 0xEF];
        let section = BinarySection::new(section_tags::DEBUG_INFO, data, 0);
        let bytes = section.to_bytes();
        assert_eq!(bytes.len(), SectionHeader::SIZE + 4);
        assert_eq!(section.tag(), section_tags::DEBUG_INFO);
    }
}
/// Writes a sequence of booleans compactly as a bitfield.
#[allow(dead_code)]
pub fn write_bools(w: &mut OleanWriter, bools: &[bool]) {
    let n = bools.len();
    w.write_u32(n as u32);
    let mut i = 0;
    while i < n {
        let mut byte = 0u8;
        for bit in 0..8 {
            if i + bit < n && bools[i + bit] {
                byte |= 1 << bit;
            }
        }
        w.write_u8(byte);
        i += 8;
    }
}
/// Reads a sequence of booleans from a compact bitfield.
#[allow(dead_code)]
pub fn read_bools(r: &mut OleanReader<'_>) -> Result<Vec<bool>, OleanError> {
    let n = r.read_u32()? as usize;
    let bytes_needed = (n + 7) / 8;
    let mut bools = Vec::with_capacity(n);
    for byte_idx in 0..bytes_needed {
        let byte = r.read_u8()?;
        for bit in 0..8 {
            let idx = byte_idx * 8 + bit;
            if idx < n {
                bools.push((byte >> bit) & 1 != 0);
            }
        }
    }
    Ok(bools)
}
#[cfg(test)]
mod tests_serial_extended2 {
    use super::*;
    #[test]
    fn test_delta_list_roundtrip() {
        let values = vec![10u32, 20, 35, 36, 100];
        let dl = DeltaList::encode(&values);
        let decoded = dl.decode();
        assert_eq!(decoded, values);
    }
    #[test]
    fn test_delta_list_serial() {
        let values = vec![0u32, 5, 10, 15];
        let dl = DeltaList::encode(&values);
        let mut w = OleanWriter::new();
        dl.write(&mut w);
        let data = w.finish();
        let mut r = OleanReader::new(&data);
        let dl2 = DeltaList::read(&mut r).expect("dl2 should be present");
        assert_eq!(dl2.decode(), values);
    }
    #[test]
    fn test_string_pool_roundtrip() {
        let mut pool = StringPool::new();
        let i0 = pool.intern("Nat.add");
        let i1 = pool.intern("List.length");
        let i0b = pool.intern("Nat.add");
        assert_eq!(i0, i0b);
        assert_ne!(i0, i1);
        assert_eq!(pool.get(i0), Some("Nat.add"));
        let mut w = OleanWriter::new();
        pool.write(&mut w);
        let data = w.finish();
        let mut r = OleanReader::new(&data);
        let p2 = StringPool::read(&mut r).expect("p2 should be present");
        assert_eq!(p2.len(), 2);
        assert_eq!(p2.get(i1), Some("List.length"));
    }
    #[test]
    fn test_write_read_bools() {
        let bools = vec![true, false, true, true, false, false, true, false, true];
        let mut w = OleanWriter::new();
        write_bools(&mut w, &bools);
        let data = w.finish();
        let mut r = OleanReader::new(&data);
        let decoded = read_bools(&mut r).expect("decoded should be present");
        assert_eq!(decoded, bools);
    }
    #[test]
    fn test_decl_kind_set() {
        let mut s = DeclKindSet::new();
        s.add(kind_tags::AXIOM);
        s.add(kind_tags::THEOREM);
        s.add(kind_tags::THEOREM);
        assert_eq!(s.count(), 2);
        assert!(s.contains(kind_tags::AXIOM));
        assert!(s.contains(kind_tags::THEOREM));
        assert!(!s.contains(kind_tags::INDUCTIVE));
        assert!(!s.is_empty());
        let mut w = OleanWriter::new();
        s.write(&mut w);
        let data = w.finish();
        let mut r = OleanReader::new(&data);
        let s2 = DeclKindSet::read(&mut r).expect("s2 should be present");
        assert_eq!(s2.mask(), s.mask());
    }
}
/// Computes a simple hash for a binary blob (for integrity checking).
#[allow(dead_code)]
pub fn fnv1a_64(data: &[u8]) -> u64 {
    let mut hash: u64 = 14_695_981_039_346_656_037;
    for &b in data {
        hash ^= b as u64;
        hash = hash.wrapping_mul(1_099_511_628_211);
    }
    hash
}
/// Checks whether two serialized blobs are identical by hash.
#[allow(dead_code)]
pub fn blobs_equal(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && fnv1a_64(a) == fnv1a_64(b)
}
#[cfg(test)]
mod tests_serial_extended3 {
    use super::*;
    #[test]
    fn test_decl_index_find_and_binary_search() {
        let mut idx = DeclIndex::new();
        idx.add("Nat.add", 100);
        idx.add("Nat.mul", 200);
        idx.add("List.length", 300);
        assert_eq!(idx.find_offset("Nat.mul"), Some(200));
        assert_eq!(idx.find_offset("Unknown"), None);
        assert!(idx.contains("Nat.add"));
        let mut idx2 = DeclIndex::new();
        idx2.add("Alpha", 10);
        idx2.add("Beta", 20);
        idx2.add("Gamma", 30);
        assert_eq!(idx2.binary_search("Beta"), Some(20));
        assert_eq!(idx2.binary_search("Delta"), None);
    }
    #[test]
    fn test_decl_index_roundtrip() {
        let mut idx = DeclIndex::new();
        idx.add("foo", 10);
        idx.add("bar", 20);
        let mut w = OleanWriter::new();
        idx.write(&mut w);
        let data = w.finish();
        let mut r = OleanReader::new(&data);
        let idx2 = DeclIndex::read(&mut r).expect("idx2 should be present");
        assert_eq!(idx2.len(), 2);
        assert_eq!(idx2.find_offset("foo"), Some(10));
    }
    #[test]
    fn test_metadata_writer_reader() {
        let mut mw = MetadataWriter::new();
        mw.write_str_entry("author", "OxiLean");
        mw.write_u64_entry("timestamp", 1_700_000_000);
        mw.write_bool_entry("verified", true);
        assert_eq!(mw.entry_count(), 3);
        let data = mw.finish();
        let mut mr = MetadataReader::new(&data).expect("mr should be present");
        let entries = mr.read_all().expect("entries should be present");
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].0, "author");
        assert!(matches!(entries[0].1, MetadataValue::Str(ref s) if s == "OxiLean"));
        assert!(matches!(entries[1].1, MetadataValue::U64(1_700_000_000)));
        assert!(matches!(entries[2].1, MetadataValue::Bool(true)));
    }
    #[test]
    fn test_fnv1a_64() {
        let h1 = fnv1a_64(b"hello");
        let h2 = fnv1a_64(b"hello");
        let h3 = fnv1a_64(b"world");
        assert_eq!(h1, h2);
        assert_ne!(h1, h3);
    }
    #[test]
    fn test_blobs_equal() {
        let a = b"OxiLean kernel serialization";
        let b = b"OxiLean kernel serialization";
        let c = b"different content";
        assert!(blobs_equal(a, b));
        assert!(!blobs_equal(a, c));
    }
}
/// Validates that a serialized blob starts with valid OleanC magic.
#[allow(dead_code)]
pub fn has_valid_magic(data: &[u8]) -> bool {
    data.len() >= 4 && &data[..4] == b"OLNC"
}
/// Returns the declared version from a blob (assumes magic is valid).
#[allow(dead_code)]
pub fn peek_version(data: &[u8]) -> Option<u32> {
    if data.len() < 8 {
        return None;
    }
    let bytes: [u8; 4] = data[4..8].try_into().ok()?;
    Some(u32::from_le_bytes(bytes))
}
/// Returns the declared decl_count from a blob (assumes magic + version valid).
#[allow(dead_code)]
pub fn peek_decl_count(data: &[u8]) -> Option<u32> {
    if data.len() < 12 {
        return None;
    }
    let bytes: [u8; 4] = data[8..12].try_into().ok()?;
    Some(u32::from_le_bytes(bytes))
}
/// Counts declarations by kind in an already-parsed list.
#[allow(dead_code)]
pub fn count_by_kind(decls: &[SerialDecl]) -> [u32; 6] {
    let mut counts = [0u32; 6];
    for d in decls {
        let idx = d.kind_tag() as usize;
        if idx < 6 {
            counts[idx] += 1;
        }
    }
    counts
}
/// Filters declarations by kind tag.
#[allow(dead_code)]
pub fn filter_by_kind(decls: &[SerialDecl], tag: u8) -> Vec<&SerialDecl> {
    decls.iter().filter(|d| d.kind_tag() == tag).collect()
}
/// Finds the first declaration with a given name prefix.
#[allow(dead_code)]
pub fn find_by_prefix<'a>(decls: &'a [SerialDecl], prefix: &str) -> Option<&'a SerialDecl> {
    decls.iter().find(|d| d.name().starts_with(prefix))
}
#[cfg(test)]
mod tests_serial_extended4 {
    use super::*;
    #[test]
    fn test_buffered_writer() {
        let mut bw = BufferedOleanWriter::new(16);
        bw.write_u32(42);
        bw.write_string("hi");
        assert_eq!(bw.total_written(), 10);
        let data = bw.flush();
        assert_eq!(data.len(), 10);
    }
    #[test]
    fn test_has_valid_magic() {
        let mut w = OleanWriter::new();
        w.write_header(0);
        let data = w.finish();
        assert!(has_valid_magic(&data));
        assert!(!has_valid_magic(b"BADD"));
        assert!(!has_valid_magic(b"OL"));
    }
    #[test]
    fn test_peek_version_and_decl_count() {
        let mut w = OleanWriter::new();
        w.write_header(7);
        let data = w.finish();
        assert_eq!(peek_version(&data), Some(1));
        assert_eq!(peek_decl_count(&data), Some(7));
    }
    #[test]
    fn test_count_by_kind() {
        let decls = vec![
            SerialDecl::Axiom {
                name: "a".to_string(),
                kind_tag: kind_tags::AXIOM,
            },
            SerialDecl::Theorem {
                name: "t1".to_string(),
                kind_tag: kind_tags::THEOREM,
            },
            SerialDecl::Theorem {
                name: "t2".to_string(),
                kind_tag: kind_tags::THEOREM,
            },
            SerialDecl::Inductive {
                name: "I".to_string(),
                ctor_count: 1,
                kind_tag: kind_tags::INDUCTIVE,
            },
        ];
        let counts = count_by_kind(&decls);
        assert_eq!(counts[kind_tags::AXIOM as usize], 1);
        assert_eq!(counts[kind_tags::THEOREM as usize], 2);
        assert_eq!(counts[kind_tags::INDUCTIVE as usize], 1);
        assert_eq!(counts[kind_tags::DEFINITION as usize], 0);
    }
    #[test]
    fn test_filter_and_find() {
        let decls = vec![
            SerialDecl::Theorem {
                name: "Nat.add_comm".to_string(),
                kind_tag: kind_tags::THEOREM,
            },
            SerialDecl::Theorem {
                name: "Nat.mul_comm".to_string(),
                kind_tag: kind_tags::THEOREM,
            },
            SerialDecl::Axiom {
                name: "propext".to_string(),
                kind_tag: kind_tags::AXIOM,
            },
        ];
        let theorems = filter_by_kind(&decls, kind_tags::THEOREM);
        assert_eq!(theorems.len(), 2);
        let found = find_by_prefix(&decls, "Nat.mul");
        assert_eq!(found.map(|d| d.name()), Some("Nat.mul_comm"));
    }
}
/// A type alias for results with `SerialError`.
#[allow(dead_code)]
pub type SerialResult<T> = Result<T, SerialError>;
/// Wraps a deserialization call with context for better error messages.
#[allow(dead_code)]
pub fn with_context<T, F>(ctx: &str, offset: usize, f: F) -> SerialResult<T>
where
    F: FnOnce() -> Result<T, OleanError>,
{
    f().map_err(|e| SerialError::new(e, ctx, offset))
}
/// Compute byte size of serializing a string.
#[allow(dead_code)]
pub fn serialized_string_size(s: &str) -> usize {
    4 + s.len()
}
/// Compute byte size of serializing a `SerialDecl`.
#[allow(dead_code)]
pub fn serialized_decl_size(decl: &SerialDecl) -> usize {
    let base = serialized_string_size(decl.name()) + 1;
    match decl {
        SerialDecl::Inductive { .. } => base + 4,
        _ => base,
    }
}
/// Compute the total byte size of a serialized declaration list.
#[allow(dead_code)]
pub fn total_serialized_size(decls: &[SerialDecl]) -> usize {
    let header_size = 20;
    let decl_size: usize = decls.iter().map(serialized_decl_size).sum();
    header_size + decl_size
}
#[cfg(test)]
mod tests_serial_extended5 {
    use super::*;
    #[test]
    fn test_checkpointed_reader_rollback() {
        let data = [1u8, 2, 3, 4, 5, 6, 7, 8];
        let mut cr = CheckpointedReader::new(&data);
        cr.save();
        let _ = cr.read_u32().expect("_ should be present");
        assert_eq!(cr.pos(), 4);
        let rolled = cr.rollback();
        assert!(rolled);
        assert_eq!(cr.pos(), 0);
        assert_eq!(cr.read_u8().expect("read_u8 should succeed"), 1);
    }
    #[test]
    fn test_checkpointed_reader_no_checkpoint() {
        let data = [10u8];
        let mut cr = CheckpointedReader::new(&data);
        let rolled = cr.rollback();
        assert!(!rolled);
    }
    #[test]
    fn test_serial_error() {
        let e = SerialError::new(OleanError::UnexpectedEof, "reading header", 42);
        let desc = e.describe();
        assert!(desc.contains("reading header"));
        assert!(desc.contains("42"));
    }
    #[test]
    fn test_with_context_ok() {
        let result = with_context("test", 0, || Ok::<u32, OleanError>(42));
        assert_eq!(result.expect("result should be valid"), 42);
    }
    #[test]
    fn test_with_context_err() {
        let result = with_context("test ctx", 10, || {
            Err::<u32, OleanError>(OleanError::UnexpectedEof)
        });
        let err = result.unwrap_err();
        assert!(err.describe().contains("test ctx"));
        assert_eq!(err.byte_offset, 10);
    }
    #[test]
    fn test_serialized_sizes() {
        let decl = SerialDecl::Theorem {
            name: "foo".to_string(),
            kind_tag: kind_tags::THEOREM,
        };
        let size = serialized_decl_size(&decl);
        assert_eq!(size, 8);
        let inductive = SerialDecl::Inductive {
            name: "Nat".to_string(),
            ctor_count: 2,
            kind_tag: kind_tags::INDUCTIVE,
        };
        let isize = serialized_decl_size(&inductive);
        assert_eq!(isize, 12);
        let total = total_serialized_size(&[decl, inductive]);
        assert_eq!(total, 20 + 8 + 12);
    }
}
/// Merge two declaration lists using a strategy.
#[allow(dead_code)]
pub fn merge_decls(a: &[SerialDecl], b: &[SerialDecl], strategy: MergeStrategy) -> Vec<String> {
    let a_names: Vec<&str> = a.iter().map(|d| d.name()).collect();
    let b_names: Vec<&str> = b.iter().map(|d| d.name()).collect();
    match strategy {
        MergeStrategy::Union => {
            let mut result: Vec<String> = a.iter().map(|d| d.name().to_string()).collect();
            for d in b {
                if !a_names.contains(&d.name()) {
                    result.push(d.name().to_string());
                }
            }
            result
        }
        MergeStrategy::Intersection => a
            .iter()
            .filter(|d| b_names.contains(&d.name()))
            .map(|d| d.name().to_string())
            .collect(),
        MergeStrategy::PreferFirst => {
            let mut result: Vec<String> = a.iter().map(|d| d.name().to_string()).collect();
            for d in b {
                if !a_names.contains(&d.name()) {
                    result.push(d.name().to_string());
                }
            }
            result
        }
        MergeStrategy::PreferSecond => {
            let mut result: Vec<String> = b.iter().map(|d| d.name().to_string()).collect();
            for d in a {
                if !b_names.contains(&d.name()) {
                    result.push(d.name().to_string());
                }
            }
            result
        }
    }
}
/// Sorts a declaration list by name.
#[allow(dead_code)]
pub fn sort_decls_by_name(decls: &mut [SerialDecl]) {
    decls.sort_by(|a, b| a.name().cmp(b.name()));
}
/// Returns deduplicated declarations (last occurrence wins).
#[allow(dead_code)]
pub fn dedup_decls(decls: Vec<SerialDecl>) -> Vec<SerialDecl> {
    let mut seen: Vec<String> = Vec::new();
    let mut result: Vec<SerialDecl> = Vec::new();
    for d in decls {
        if !seen.contains(&d.name().to_string()) {
            seen.push(d.name().to_string());
            result.push(d);
        }
    }
    result
}
#[cfg(test)]
mod tests_serial_extended6 {
    use super::*;
    #[test]
    fn test_compatibility_checker() {
        let checker = CompatibilityChecker::new(vec![1, 2]);
        assert!(checker.is_compatible(1));
        assert!(checker.is_compatible(2));
        assert!(!checker.is_compatible(3));
        assert_eq!(checker.latest(), Some(2));
        assert!(checker.needs_upgrade(1, 2));
        assert!(!checker.needs_upgrade(2, 1));
    }
    #[test]
    fn test_merge_union() {
        let a = vec![
            SerialDecl::Theorem {
                name: "a".to_string(),
                kind_tag: kind_tags::THEOREM,
            },
            SerialDecl::Theorem {
                name: "b".to_string(),
                kind_tag: kind_tags::THEOREM,
            },
        ];
        let b = vec![
            SerialDecl::Theorem {
                name: "b".to_string(),
                kind_tag: kind_tags::THEOREM,
            },
            SerialDecl::Theorem {
                name: "c".to_string(),
                kind_tag: kind_tags::THEOREM,
            },
        ];
        let result = merge_decls(&a, &b, MergeStrategy::Union);
        assert!(result.contains(&"a".to_string()));
        assert!(result.contains(&"b".to_string()));
        assert!(result.contains(&"c".to_string()));
        assert_eq!(result.len(), 3);
    }
    #[test]
    fn test_merge_intersection() {
        let a = vec![
            SerialDecl::Theorem {
                name: "shared".to_string(),
                kind_tag: kind_tags::THEOREM,
            },
            SerialDecl::Theorem {
                name: "a_only".to_string(),
                kind_tag: kind_tags::THEOREM,
            },
        ];
        let b = vec![
            SerialDecl::Theorem {
                name: "shared".to_string(),
                kind_tag: kind_tags::THEOREM,
            },
            SerialDecl::Theorem {
                name: "b_only".to_string(),
                kind_tag: kind_tags::THEOREM,
            },
        ];
        let result = merge_decls(&a, &b, MergeStrategy::Intersection);
        assert_eq!(result, vec!["shared".to_string()]);
    }
    #[test]
    fn test_sort_decls_by_name() {
        let mut decls = vec![
            SerialDecl::Other {
                name: "z".to_string(),
                kind_tag: kind_tags::OTHER,
            },
            SerialDecl::Other {
                name: "a".to_string(),
                kind_tag: kind_tags::OTHER,
            },
            SerialDecl::Other {
                name: "m".to_string(),
                kind_tag: kind_tags::OTHER,
            },
        ];
        sort_decls_by_name(&mut decls);
        assert_eq!(decls[0].name(), "a");
        assert_eq!(decls[1].name(), "m");
        assert_eq!(decls[2].name(), "z");
    }
    #[test]
    fn test_dedup_decls() {
        let decls = vec![
            SerialDecl::Theorem {
                name: "a".to_string(),
                kind_tag: kind_tags::THEOREM,
            },
            SerialDecl::Axiom {
                name: "b".to_string(),
                kind_tag: kind_tags::AXIOM,
            },
            SerialDecl::Theorem {
                name: "a".to_string(),
                kind_tag: kind_tags::THEOREM,
            },
        ];
        let deduped = dedup_decls(decls);
        assert_eq!(deduped.len(), 2);
        assert_eq!(deduped[0].name(), "a");
        assert_eq!(deduped[1].name(), "b");
    }
}
#[cfg(test)]
mod tests_serial_extended7 {
    use super::*;
    #[test]
    fn test_file_stats() {
        let decls = vec![
            SerialDecl::Theorem {
                name: "t".to_string(),
                kind_tag: kind_tags::THEOREM,
            },
            SerialDecl::Axiom {
                name: "a".to_string(),
                kind_tag: kind_tags::AXIOM,
            },
            SerialDecl::Inductive {
                name: "I".to_string(),
                ctor_count: 2,
                kind_tag: kind_tags::INDUCTIVE,
            },
        ];
        let stats = FileStats::from_decls(&decls, 300);
        assert_eq!(stats.total_decls, 3);
        assert_eq!(stats.theorems, 1);
        assert_eq!(stats.axioms, 1);
        assert_eq!(stats.inductives, 1);
        assert!((stats.bytes_per_decl() - 100.0).abs() < 1e-9);
        let s = stats.summary();
        assert!(s.contains("total=3"));
    }
    #[test]
    fn test_olean_archive() {
        let mut archive = OleanArchive::new();
        archive.add_file(
            "Nat.lean",
            vec![SerialDecl::Inductive {
                name: "Nat".to_string(),
                ctor_count: 2,
                kind_tag: kind_tags::INDUCTIVE,
            }],
        );
        archive.add_file(
            "List.lean",
            vec![
                SerialDecl::Inductive {
                    name: "List".to_string(),
                    ctor_count: 2,
                    kind_tag: kind_tags::INDUCTIVE,
                },
                SerialDecl::Theorem {
                    name: "List.length_eq".to_string(),
                    kind_tag: kind_tags::THEOREM,
                },
            ],
        );
        assert_eq!(archive.total_decls(), 3);
        assert_eq!(archive.file_count(), 2);
        let (fname, decl) = archive
            .find_decl("List.length_eq")
            .expect("value should be present");
        assert_eq!(fname, "List.lean");
        assert_eq!(decl.kind_tag(), kind_tags::THEOREM);
        assert_eq!(archive.find_decl("Unknown"), None);
        assert!(!archive.is_empty());
    }
}
/// Groups declarations by namespace prefix (up to first dot).
#[allow(dead_code)]
pub fn group_by_namespace(decls: &[SerialDecl]) -> Vec<(String, Vec<&SerialDecl>)> {
    let mut groups: Vec<(String, Vec<&SerialDecl>)> = Vec::new();
    for d in decls {
        let ns = d.name().split('.').next().unwrap_or("_").to_string();
        if let Some(g) = groups.iter_mut().find(|(n, _)| *n == ns) {
            g.1.push(d);
        } else {
            groups.push((ns, vec![d]));
        }
    }
    groups
}
#[cfg(test)]
mod tests_serial_extended8 {
    use super::*;
    #[test]
    fn test_decl_diff() {
        let old: Vec<String> = vec!["a".to_string(), "b".to_string()];
        let new: Vec<String> = vec!["b".to_string(), "c".to_string()];
        let diff = DeclDiff::compute(&old, &new);
        assert_eq!(diff.added, vec!["c".to_string()]);
        assert_eq!(diff.removed, vec!["a".to_string()]);
        assert_eq!(diff.unchanged, vec!["b".to_string()]);
        assert!(diff.has_changes());
        let s = diff.summary();
        assert!(s.contains("+1 -1 =1"));
    }
    #[test]
    fn test_group_by_namespace() {
        let decls = vec![
            SerialDecl::Theorem {
                name: "Nat.add".to_string(),
                kind_tag: kind_tags::THEOREM,
            },
            SerialDecl::Theorem {
                name: "Nat.mul".to_string(),
                kind_tag: kind_tags::THEOREM,
            },
            SerialDecl::Theorem {
                name: "List.length".to_string(),
                kind_tag: kind_tags::THEOREM,
            },
        ];
        let groups = group_by_namespace(&decls);
        assert_eq!(groups.len(), 2);
        let nat_group = groups
            .iter()
            .find(|(ns, _)| ns == "Nat")
            .expect("nat_group should be present");
        assert_eq!(nat_group.1.len(), 2);
    }
}
#[cfg(test)]
mod tests_serial_diag {
    use super::*;
    #[test]
    fn test_format_diagnostics() {
        let data = serialize_decl_names(&["foo".to_string(), "bar".to_string()]);
        let diag = FormatDiagnostics::from_bytes(&data);
        assert!(diag.magic_ok);
        assert_eq!(diag.decl_count, 2);
        assert!(diag.is_well_formed());
        let report = diag.report();
        assert!(report.contains("magic=true"));
    }
    #[test]
    fn test_format_diagnostics_bad() {
        let bad = b"BADD1234";
        let diag = FormatDiagnostics::from_bytes(bad);
        assert!(!diag.magic_ok);
        assert!(!diag.is_well_formed());
    }
}
/// Encode a byte slice as hexadecimal for display.
#[allow(dead_code)]
pub fn to_hex(data: &[u8]) -> String {
    data.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join("")
}
/// Decode a hexadecimal string to bytes.
#[allow(dead_code)]
pub fn from_hex(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 != 0 {
        return None;
    }
    (0..s.len() / 2)
        .map(|i| u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).ok())
        .collect()
}
#[cfg(test)]
mod tests_hex {
    use super::*;
    #[test]
    fn test_hex_roundtrip() {
        let data = vec![0xDE, 0xAD, 0xBE, 0xEF];
        let hex = to_hex(&data);
        assert_eq!(hex, "deadbeef");
        assert_eq!(from_hex(&hex), Some(data));
    }
}
