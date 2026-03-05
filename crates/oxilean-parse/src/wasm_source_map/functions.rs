//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    AbsoluteMapping, DecodedSegment, SourceMap, SourceMapOptions, SourceMapStats, SourcePos2,
    VlqEncoder, WasmAnnotation, WasmAnnotationTable,
};

/// Base64 alphabet used by VLQ encoding.
const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
#[cfg(test)]
mod tests {
    use super::*;
    use crate::wasm_source_map::*;
    #[test]
    fn test_source_mapping_new() {
        let m = SourceMapping::new(1, 2, 3, 4, "foo.oxilean");
        assert_eq!(m.generated_line, 1);
        assert_eq!(m.generated_col, 2);
        assert_eq!(m.source_line, 3);
        assert_eq!(m.source_col, 4);
        assert_eq!(m.source_file, "foo.oxilean");
    }
    #[test]
    fn test_vlq_encode_zero() {
        let encoded = VlqEncoder::encode_vlq(0);
        assert_eq!(encoded, b"A");
    }
    #[test]
    fn test_vlq_encode_positive() {
        let encoded = VlqEncoder::encode_vlq(1);
        assert_eq!(encoded, b"C");
    }
    #[test]
    fn test_vlq_encode_negative() {
        let encoded = VlqEncoder::encode_vlq(-1);
        assert_eq!(encoded, b"D");
    }
    #[test]
    fn test_vlq_roundtrip() {
        for val in [-100i64, -1, 0, 1, 42, 1000, 65535] {
            let encoded = VlqEncoder::encode_vlq(val);
            let (decoded, _) = VlqEncoder::decode_vlq(&encoded);
            assert_eq!(decoded, val, "roundtrip failed for {val}");
        }
    }
    #[test]
    fn test_source_map_add_source_dedup() {
        let mut sm = SourceMap::new();
        let idx1 = sm.add_source("a.lean");
        let idx2 = sm.add_source("b.lean");
        let idx3 = sm.add_source("a.lean");
        assert_eq!(idx1, 0);
        assert_eq!(idx2, 1);
        assert_eq!(idx3, 0);
        assert_eq!(sm.sources.len(), 2);
    }
    #[test]
    fn test_source_map_lookup_source() {
        let mut sm = SourceMap::new();
        sm.add_source("f.lean");
        sm.add_mapping(SourceMapping::new(0, 0, 1, 0, "f.lean"));
        sm.add_mapping(SourceMapping::new(0, 5, 1, 5, "f.lean"));
        sm.add_mapping(SourceMapping::new(1, 0, 2, 0, "f.lean"));
        let found = sm.lookup_source(0, 7);
        assert!(found.is_some());
        assert_eq!(
            found.expect("test operation should succeed").generated_col,
            5
        );
        assert!(sm.lookup_source(5, 0).is_none());
    }
    #[test]
    fn test_wasm_source_map_builder() {
        let mut builder = WasmSourceMapBuilder::new("main.lean");
        builder.record_token(0, 0, 1, 0);
        builder.record_token(0, 4, 1, 4);
        let sm = builder.build();
        assert_eq!(sm.mappings.len(), 2);
        assert_eq!(sm.sources[0], "main.lean");
    }
    #[test]
    fn test_source_map_to_json_version() {
        let sm = SourceMap::new();
        let json = sm.to_json();
        assert!(json.contains("\"version\":3"));
    }
}
/// Decode the VLQ-encoded mappings string from a Source Map v3 into a
/// `Vec<Vec<DecodedSegment>>` (outer = lines, inner = segments per line).
#[allow(dead_code)]
pub fn decode_mappings(mappings: &str) -> Vec<Vec<DecodedSegment>> {
    let mut result: Vec<Vec<DecodedSegment>> = Vec::new();
    let mut current_line: Vec<DecodedSegment> = Vec::new();
    let mut prev_gen_col: i64 = 0;
    let mut prev_src_file: i64 = 0;
    let mut prev_src_line: i64 = 0;
    let mut prev_src_col: i64 = 0;
    let mut segment_bytes = Vec::new();
    for ch in mappings.chars() {
        match ch {
            ';' => {
                if !segment_bytes.is_empty() {
                    if let Some(seg) = decode_one_segment(
                        &segment_bytes,
                        &mut prev_gen_col,
                        &mut prev_src_file,
                        &mut prev_src_line,
                        &mut prev_src_col,
                    ) {
                        current_line.push(seg);
                    }
                    segment_bytes.clear();
                }
                result.push(current_line.clone());
                current_line.clear();
                prev_gen_col = 0;
            }
            ',' => {
                if !segment_bytes.is_empty() {
                    if let Some(seg) = decode_one_segment(
                        &segment_bytes,
                        &mut prev_gen_col,
                        &mut prev_src_file,
                        &mut prev_src_line,
                        &mut prev_src_col,
                    ) {
                        current_line.push(seg);
                    }
                    segment_bytes.clear();
                }
            }
            _ => {
                segment_bytes.push(ch as u8);
            }
        }
    }
    if !segment_bytes.is_empty() {
        if let Some(seg) = decode_one_segment(
            &segment_bytes,
            &mut prev_gen_col,
            &mut prev_src_file,
            &mut prev_src_line,
            &mut prev_src_col,
        ) {
            current_line.push(seg);
        }
    }
    if !current_line.is_empty() {
        result.push(current_line);
    }
    result
}
/// Decode one VLQ segment (multiple VLQ values) into a `DecodedSegment`.
#[allow(dead_code)]
pub(super) fn decode_one_segment(
    bytes: &[u8],
    prev_gen_col: &mut i64,
    prev_src_file: &mut i64,
    prev_src_line: &mut i64,
    prev_src_col: &mut i64,
) -> Option<DecodedSegment> {
    let mut values = Vec::new();
    let mut pos = 0;
    while pos < bytes.len() {
        let (val, consumed) = VlqEncoder::decode_vlq(&bytes[pos..]);
        if consumed == 0 {
            break;
        }
        values.push(val);
        pos += consumed;
    }
    if values.is_empty() {
        return None;
    }
    let gen_col_delta = values[0];
    *prev_gen_col += gen_col_delta;
    if values.len() >= 4 {
        *prev_src_file += values[1];
        *prev_src_line += values[2];
        *prev_src_col += values[3];
        Some(DecodedSegment::full(
            *prev_gen_col,
            *prev_src_file,
            *prev_src_line,
            *prev_src_col,
        ))
    } else {
        Some(DecodedSegment::generated_only(*prev_gen_col))
    }
}
/// Convert a `SourceMap` into a flat list of `AbsoluteMapping`s.
#[allow(dead_code)]
pub fn to_absolute_mappings(sm: &SourceMap) -> Vec<AbsoluteMapping> {
    sm.mappings
        .iter()
        .map(|m| {
            let idx = sm
                .sources
                .iter()
                .position(|s| *s == m.source_file)
                .unwrap_or(0);
            AbsoluteMapping::from_mapping(m, idx)
        })
        .collect()
}
/// Generate a JSON source map with options.
#[allow(dead_code)]
pub fn generate_source_map_json(sm: &SourceMap, opts: &SourceMapOptions) -> String {
    let sources_json = sm
        .sources
        .iter()
        .map(|s| format!("\"{}\"", s.replace('"', "\\\"")))
        .collect::<Vec<_>>()
        .join(",");
    let mappings_str = sm.encode_mappings();
    let mut fields = format!(
        "\"version\":{},\"sources\":[{}],\"mappings\":\"{}\"",
        sm.version, sources_json, mappings_str
    );
    if let Some(root) = &opts.source_root {
        fields.push_str(&format!(",\"sourceRoot\":\"{}\"", root));
    }
    if !sm.names.is_empty() && opts.include_names {
        let names_json = sm
            .names
            .iter()
            .map(|n| format!("\"{}\"", n.replace('"', "\\\"")))
            .collect::<Vec<_>>()
            .join(",");
        fields.push_str(&format!(",\"names\":[{}]", names_json));
    }
    format!("{{{}}}", fields)
}
/// Compute a "compression ratio" for VLQ-encoded mapping data.
#[allow(dead_code)]
pub fn vlq_compression_ratio(unencoded_values: &[i64], encoded: &str) -> f64 {
    let unencoded_size = unencoded_values.len() * 8;
    let encoded_size = encoded.len();
    if unencoded_size == 0 {
        1.0
    } else {
        encoded_size as f64 / unencoded_size as f64
    }
}
/// Render a source map as a human-readable summary.
#[allow(dead_code)]
pub fn summarize_source_map(sm: &SourceMap) -> String {
    let stats = SourceMapStats::from_map(sm);
    format!(
        "SourceMap v{}: {} source(s), {} mapping(s) across {} line(s), ~{} bytes encoded",
        sm.version, stats.source_count, stats.mapping_count, stats.line_count, stats.encoded_size
    )
}
/// Merge a list of source maps into one.
#[allow(dead_code)]
pub fn merge_source_maps(maps: Vec<SourceMap>) -> SourceMap {
    let mut result = SourceMap::new();
    for sm in maps {
        result.merge(&sm);
    }
    result
}
/// Validate that VLQ encoding is self-consistent for a range of values.
#[allow(dead_code)]
pub fn validate_vlq_codec(values: &[i64]) -> bool {
    for &v in values {
        let encoded = VlqEncoder::encode_vlq(v);
        let (decoded, _) = VlqEncoder::decode_vlq(&encoded);
        if decoded != v {
            return false;
        }
    }
    true
}
#[cfg(test)]
mod extended_wasm_tests {
    use super::*;
    use crate::wasm_source_map::*;
    #[test]
    fn test_decoded_segment_full() {
        let s = DecodedSegment::full(0, 0, 1, 0);
        assert!(s.has_source());
        assert_eq!(s.src_file, Some(0));
    }
    #[test]
    fn test_decoded_segment_generated_only() {
        let s = DecodedSegment::generated_only(3);
        assert!(!s.has_source());
        assert_eq!(s.gen_col, 3);
    }
    #[test]
    fn test_source_position_display() {
        let p = SourcePosition::new("foo.lean", 3, 5);
        assert_eq!(format!("{}", p), "foo.lean:3:5");
    }
    #[test]
    fn test_generated_position_display() {
        let p = GeneratedPosition::new(1, 2);
        assert_eq!(format!("{}", p), "1:2");
    }
    #[test]
    fn test_source_map_add_name() {
        let mut sm = SourceMap::new();
        let i1 = sm.add_name("myFunc");
        let i2 = sm.add_name("otherFunc");
        let i3 = sm.add_name("myFunc");
        assert_eq!(i1, 0);
        assert_eq!(i2, 1);
        assert_eq!(i3, 0);
    }
    #[test]
    fn test_source_map_validate_ok() {
        let mut sm = SourceMap::new();
        sm.add_source("a.lean");
        sm.add_mapping(SourceMapping::new(0, 0, 1, 0, "a.lean"));
        assert!(sm.validate().is_ok());
    }
    #[test]
    fn test_source_map_validate_err() {
        let mut sm = SourceMap::new();
        sm.add_mapping(SourceMapping::new(0, 0, 1, 0, "unknown.lean"));
        assert!(sm.validate().is_err());
    }
    #[test]
    fn test_source_map_sort_mappings() {
        let mut sm = SourceMap::new();
        sm.add_source("f.lean");
        sm.add_mapping(SourceMapping::new(0, 5, 1, 5, "f.lean"));
        sm.add_mapping(SourceMapping::new(0, 0, 1, 0, "f.lean"));
        sm.sort_mappings();
        assert_eq!(sm.mappings[0].generated_col, 0);
        assert_eq!(sm.mappings[1].generated_col, 5);
    }
    #[test]
    fn test_source_map_merge() {
        let mut sm1 = SourceMap::new();
        sm1.add_source("a.lean");
        sm1.add_mapping(SourceMapping::new(0, 0, 1, 0, "a.lean"));
        let mut sm2 = SourceMap::new();
        sm2.add_source("b.lean");
        sm2.add_mapping(SourceMapping::new(1, 0, 2, 0, "b.lean"));
        sm1.merge(&sm2);
        assert_eq!(sm1.mapping_count(), 2);
        assert_eq!(sm1.source_count(), 2);
    }
    #[test]
    fn test_base64_encode_decode_roundtrip() {
        let data = b"hello source map world";
        let encoded = Base64Util::encode(data);
        let decoded = Base64Util::decode(&encoded);
        assert_eq!(decoded, data);
    }
    #[test]
    fn test_base64_encode_empty() {
        let encoded = Base64Util::encode(b"");
        assert!(encoded.is_empty());
    }
    #[test]
    fn test_vlq_stream_push_finish() {
        let mut stream = VlqStream::new();
        stream.push(0);
        stream.push(1);
        stream.push(-1);
        let s = stream.finish();
        assert!(!s.is_empty());
    }
    #[test]
    fn test_vlq_stream_empty() {
        let stream = VlqStream::new();
        assert!(stream.is_empty());
    }
    #[test]
    fn test_source_map_stats_from_map() {
        let mut sm = SourceMap::new();
        sm.add_source("x.lean");
        sm.add_mapping(SourceMapping::new(0, 0, 1, 0, "x.lean"));
        sm.add_mapping(SourceMapping::new(0, 5, 1, 5, "x.lean"));
        let stats = SourceMapStats::from_map(&sm);
        assert_eq!(stats.mapping_count, 2);
        assert_eq!(stats.source_count, 1);
    }
    #[test]
    fn test_source_map_stats_display() {
        let sm = SourceMap::new();
        let stats = SourceMapStats::from_map(&sm);
        let s = format!("{}", stats);
        assert!(s.contains("SourceMapStats"));
    }
    #[test]
    fn test_summarize_source_map() {
        let sm = SourceMap::new();
        let s = summarize_source_map(&sm);
        assert!(s.contains("SourceMap"));
    }
    #[test]
    fn test_merge_source_maps() {
        let mut sm1 = SourceMap::new();
        sm1.add_source("a.lean");
        sm1.add_mapping(SourceMapping::new(0, 0, 1, 0, "a.lean"));
        let mut sm2 = SourceMap::new();
        sm2.add_source("b.lean");
        sm2.add_mapping(SourceMapping::new(1, 0, 2, 0, "b.lean"));
        let merged = merge_source_maps(vec![sm1, sm2]);
        assert_eq!(merged.mapping_count(), 2);
    }
    #[test]
    fn test_validate_vlq_codec() {
        let vals = vec![-100, -1, 0, 1, 42, 1000];
        assert!(validate_vlq_codec(&vals));
    }
    #[test]
    fn test_source_range_contains() {
        let r = SourceRange::new(
            SourcePosition::new("f.lean", 1, 0),
            SourcePosition::new("f.lean", 3, 10),
        );
        assert!(r.contains_position(&SourcePosition::new("f.lean", 2, 5)));
        assert!(!r.contains_position(&SourcePosition::new("f.lean", 5, 0)));
        assert!(!r.contains_position(&SourcePosition::new("other.lean", 2, 5)));
    }
    #[test]
    fn test_source_range_display() {
        let r = SourceRange::new(
            SourcePosition::new("f.lean", 1, 0),
            SourcePosition::new("f.lean", 2, 5),
        );
        let s = format!("{}", r);
        assert!(s.contains("->"));
    }
    #[test]
    fn test_source_map_diff_empty() {
        let sm = SourceMap::new();
        let diff = SourceMapDiff::compute(&sm, &sm);
        assert!(diff.is_empty());
        assert_eq!(diff.change_count(), 0);
    }
    #[test]
    fn test_source_map_diff_added() {
        let sm_old = SourceMap::new();
        let mut sm_new = SourceMap::new();
        sm_new.add_source("a.lean");
        sm_new.add_mapping(SourceMapping::new(0, 0, 1, 0, "a.lean"));
        let diff = SourceMapDiff::compute(&sm_old, &sm_new);
        assert_eq!(diff.added.len(), 1);
        assert_eq!(diff.removed.len(), 0);
    }
    #[test]
    fn test_source_map_diff_removed() {
        let mut sm_old = SourceMap::new();
        sm_old.add_source("a.lean");
        sm_old.add_mapping(SourceMapping::new(0, 0, 1, 0, "a.lean"));
        let sm_new = SourceMap::new();
        let diff = SourceMapDiff::compute(&sm_old, &sm_new);
        assert_eq!(diff.removed.len(), 1);
        assert_eq!(diff.added.len(), 0);
    }
    #[test]
    fn test_source_map_index_build_and_lookup() {
        let mut sm = SourceMap::new();
        sm.add_source("f.lean");
        sm.add_mapping(SourceMapping::new(0, 0, 1, 0, "f.lean"));
        sm.add_mapping(SourceMapping::new(0, 5, 1, 5, "f.lean"));
        sm.add_mapping(SourceMapping::new(1, 0, 2, 0, "f.lean"));
        let idx = SourceMapIndex::build(&sm);
        assert_eq!(idx.len(), 3);
        let found = idx.lookup(0, 3);
        assert!(found.is_some());
    }
    #[test]
    fn test_source_map_index_empty() {
        let sm = SourceMap::new();
        let idx = SourceMapIndex::build(&sm);
        assert!(idx.is_empty());
        assert!(idx.lookup(0, 0).is_none());
    }
    #[test]
    fn test_multi_file_source_map() {
        let mut mf = MultiFileSourceMap::new();
        let sm = SourceMap::new();
        mf.add("output.js", sm);
        assert_eq!(mf.len(), 1);
        assert!(mf.get("output.js").is_some());
        assert!(mf.get("missing.js").is_none());
    }
    #[test]
    fn test_multi_file_source_map_index_json() {
        let mut mf = MultiFileSourceMap::new();
        mf.add("a.js", SourceMap::new());
        mf.add("b.js", SourceMap::new());
        let json = mf.to_index_json();
        assert!(json.contains("a.js"));
        assert!(json.contains("b.js"));
    }
    #[test]
    fn test_source_map_options_default() {
        let opts = SourceMapOptions::default();
        assert!(!opts.embed_sources);
        assert!(!opts.include_names);
        assert!(opts.source_root.is_none());
    }
    #[test]
    fn test_source_map_options_builder() {
        let opts = SourceMapOptions::new()
            .with_embedded_sources()
            .with_source_root("/src");
        assert!(opts.embed_sources);
        assert_eq!(opts.source_root.as_deref(), Some("/src"));
    }
    #[test]
    fn test_generate_source_map_json_with_options() {
        let mut sm = SourceMap::new();
        sm.add_source("a.lean");
        let opts = SourceMapOptions::new().with_source_root("/src");
        let json = generate_source_map_json(&sm, &opts);
        assert!(json.contains("sourceRoot"));
        assert!(json.contains("/src"));
    }
    #[test]
    fn test_reverse_source_map() {
        let mut sm = SourceMap::new();
        sm.add_source("f.lean");
        sm.add_mapping(SourceMapping::new(0, 0, 5, 3, "f.lean"));
        let rev = ReverseSourceMap::build(sm);
        let pos = rev.original(0, 0);
        assert!(pos.is_some());
        let p = pos.expect("test operation should succeed");
        assert_eq!(p.line, 5);
        assert_eq!(p.col, 3);
    }
    #[test]
    fn test_reverse_source_map_not_found() {
        let sm = SourceMap::new();
        let rev = ReverseSourceMap::build(sm);
        assert!(rev.original(0, 0).is_none());
    }
    #[test]
    fn test_wasm_builder_set_file() {
        let mut builder = WasmSourceMapBuilder::new("main.lean");
        builder.set_file("other.lean");
        assert_eq!(builder.current_file, "other.lean");
        assert_eq!(builder.source_map.source_count(), 2);
    }
    #[test]
    fn test_wasm_builder_to_json() {
        let builder = WasmSourceMapBuilder::new("main.lean");
        let json = builder.to_json();
        assert!(json.contains("version"));
    }
    #[test]
    fn test_wasm_builder_mapping_count() {
        let mut builder = WasmSourceMapBuilder::new("main.lean");
        builder.record_token(0, 0, 1, 0);
        builder.record_token(0, 5, 1, 5);
        assert_eq!(builder.mapping_count(), 2);
    }
    #[test]
    fn test_vlq_compression_ratio() {
        let values: Vec<i64> = (0..10).collect();
        let encoded = VlqEncoder::encode_segment(&values);
        let ratio = vlq_compression_ratio(&values, &encoded);
        assert!(ratio > 0.0);
    }
    #[test]
    fn test_decode_mappings_empty() {
        let decoded = decode_mappings("");
        assert!(decoded.is_empty());
    }
    #[test]
    fn test_to_absolute_mappings() {
        let mut sm = SourceMap::new();
        sm.add_source("f.lean");
        sm.add_mapping(SourceMapping::new(0, 0, 1, 0, "f.lean"));
        let abs = to_absolute_mappings(&sm);
        assert_eq!(abs.len(), 1);
        assert_eq!(abs[0].src_file, 0);
    }
    #[test]
    fn test_source_to_generated_map() {
        let mut sg = SourceToGeneratedMap::new("source.lean");
        let mut sm = SourceMap::new();
        sm.add_source("source.lean");
        sm.add_mapping(SourceMapping::new(5, 3, 10, 2, "source.lean"));
        sg.add_generated("output.wasm", sm);
        let results = sg.find_generated(10, 2);
        assert!(!results.is_empty());
    }
    #[test]
    fn test_source_map_clear_mappings() {
        let mut sm = SourceMap::new();
        sm.add_source("f.lean");
        sm.add_mapping(SourceMapping::new(0, 0, 1, 0, "f.lean"));
        sm.clear_mappings();
        assert_eq!(sm.mapping_count(), 0);
    }
}
#[cfg(test)]
mod wasm_sourcemap_ext_tests {
    use super::*;
    use crate::wasm_source_map::*;
    #[test]
    fn test_source_map_entry() {
        let entry = SourceMapEntry::new(0, 0, 1, 0).with_name(2);
        assert_eq!(entry.gen_col, 0);
        assert_eq!(entry.name_idx, Some(2));
    }
    #[test]
    fn test_source_map_group_sort() {
        let mut group = SourceMapGroup::new();
        group.add(SourceMapEntry::new(5, 0, 1, 5));
        group.add(SourceMapEntry::new(0, 0, 1, 0));
        group.sort();
        assert_eq!(group.entries[0].gen_col, 0);
        assert_eq!(group.entries[1].gen_col, 5);
    }
    #[test]
    fn test_full_source_map() {
        let mut sm = FullSourceMap::new();
        let src_idx = sm.add_source("test.lean");
        assert_eq!(src_idx, 0);
        let mut group = SourceMapGroup::new();
        group.add(SourceMapEntry::new(0, src_idx, 1, 0));
        sm.add_group(group);
        assert_eq!(sm.total_segments(), 1);
    }
    #[test]
    fn test_vlq_encode() {
        let s = VlqCodec::encode(0);
        assert_eq!(s, "A");
        let s2 = VlqCodec::encode(1);
        assert!(!s2.is_empty());
    }
    #[test]
    fn test_source_map_lookup() {
        let sm = SourceMapBuilder::new()
            .source("test.lean")
            .map_col(0, 0, 1, 0)
            .map_col(5, 0, 1, 5)
            .build();
        let entry = sm.lookup(0, 3);
        assert!(entry.is_some());
        assert_eq!(entry.expect("test operation should succeed").gen_col, 0);
        let entry2 = sm.lookup(0, 7);
        assert_eq!(entry2.expect("test operation should succeed").gen_col, 5);
    }
    #[test]
    fn test_source_map_builder() {
        let sm = SourceMapBuilder::new()
            .source("a.lean")
            .source("b.lean")
            .map_col(0, 0, 1, 0)
            .next_line()
            .map_col(0, 1, 2, 0)
            .build();
        assert_eq!(sm.sources.len(), 2);
        assert_eq!(sm.groups.len(), 2);
    }
}
#[cfg(test)]
mod wasm_sourcemap_ext2_tests {
    use super::*;
    use crate::wasm_source_map::*;
    #[test]
    fn test_source_map_merger() {
        let sm1 = SourceMapBuilder::new()
            .source("a.lean")
            .map_col(0, 0, 1, 0)
            .build();
        let sm2 = SourceMapBuilder::new()
            .source("b.lean")
            .map_col(0, 0, 1, 0)
            .build();
        let mut merger = SourceMapMerger::new();
        merger.merge(sm1);
        merger.merge(sm2);
        let merged = merger.finish();
        assert_eq!(merged.sources.len(), 2);
        assert_eq!(merged.groups.len(), 2);
    }
    #[test]
    fn test_source_map_validator() {
        let sm = SourceMapBuilder::new()
            .source("a.lean")
            .map_col(0, 0, 1, 0)
            .build();
        let errors = SourceMapValidator::validate_source_indices(&sm);
        assert!(errors.is_empty());
    }
    #[test]
    fn test_source_map_validator_out_of_bounds() {
        let mut sm = FullSourceMap::new();
        sm.add_source("a.lean");
        let mut group = SourceMapGroup::new();
        group.add(SourceMapEntry::new(0, 99, 1, 0));
        sm.add_group(group);
        let errors = SourceMapValidator::validate_source_indices(&sm);
        assert!(!errors.is_empty());
    }
}
/// Converts a flat byte offset to a SourcePos2 using a line map.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn offset_to_source_pos(src: &str, offset: usize) -> SourcePos2 {
    let mut line = 1u32;
    let mut col = 1u32;
    for (i, c) in src.char_indices() {
        if i >= offset {
            break;
        }
        if c == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    SourcePos2::new(line, col)
}
#[cfg(test)]
mod wasm_ext3_tests {
    use super::*;
    use crate::wasm_source_map::*;
    #[test]
    fn test_source_map_stats() {
        let sm = SourceMapBuilder::new()
            .source("a.lean")
            .map_col(0, 0, 1, 0)
            .next_line()
            .map_col(0, 0, 2, 0)
            .build();
        let stats = SourceMapStatsExt::from_map(&sm);
        assert_eq!(stats.total_segments, 2);
        assert_eq!(stats.source_count, 1);
        let out = stats.format();
        assert!(out.contains("segments=2"));
    }
    #[test]
    fn test_source_pos2() {
        let p1 = SourcePos2::new(1, 5);
        let p2 = SourcePos2::new(2, 1);
        assert!(p1.before(&p2));
        let r = SourceRangeExt::new(p1, p2);
        assert!(r.contains(SourcePos2::new(1, 10)));
        assert!(!r.contains(SourcePos2::new(3, 1)));
    }
    #[test]
    fn test_offset_to_source_pos() {
        let src = "hello\nworld";
        let pos = offset_to_source_pos(src, 6);
        assert_eq!(pos.line, 2);
        assert_eq!(pos.col, 1);
    }
}
#[cfg(test)]
mod wasm_annotation_tests {
    use super::*;
    use crate::wasm_source_map::*;
    #[test]
    fn test_wasm_annotation() {
        let ann = WasmAnnotation::new(100, 0, 5, 3).with_func("myFunc");
        assert_eq!(ann.wasm_offset, 100);
        assert_eq!(ann.func_name.as_deref(), Some("myFunc"));
    }
    #[test]
    fn test_wasm_annotation_table() {
        let mut table = WasmAnnotationTable::new();
        table.add(WasmAnnotation::new(0, 0, 1, 0));
        table.add(WasmAnnotation::new(50, 0, 5, 0));
        table.add(WasmAnnotation::new(100, 0, 10, 0));
        let found = table.lookup(75).expect("lookup should succeed");
        assert_eq!(found.wasm_offset, 50);
        let found2 = table.lookup(100).expect("lookup should succeed");
        assert_eq!(found2.line, 10);
    }
}
/// A simple WASM instruction counter.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn count_annotations_in_range(table: &WasmAnnotationTable, lo: u32, hi: u32) -> usize {
    table
        .annotations
        .iter()
        .filter(|a| a.wasm_offset >= lo && a.wasm_offset < hi)
        .count()
}
/// Returns annotations for a specific source file.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn annotations_for_source(
    table: &WasmAnnotationTable,
    source_idx: u32,
) -> Vec<&WasmAnnotation> {
    table
        .annotations
        .iter()
        .filter(|a| a.source_idx == source_idx)
        .collect()
}
/// Returns the line range covered by annotations in a table.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn annotation_line_range(table: &WasmAnnotationTable) -> Option<(u32, u32)> {
    if table.is_empty() {
        return None;
    }
    let min_line = table
        .annotations
        .iter()
        .map(|a| a.line)
        .min()
        .expect("annotations non-empty per is_empty check above");
    let max_line = table
        .annotations
        .iter()
        .map(|a| a.line)
        .max()
        .expect("annotations non-empty per is_empty check above");
    Some((min_line, max_line))
}
#[cfg(test)]
mod wasm_pad {
    use super::*;
    use crate::wasm_source_map::*;
    #[test]
    fn test_count_annotations_in_range() {
        let mut t = WasmAnnotationTable::new();
        t.add(WasmAnnotation::new(0, 0, 1, 0));
        t.add(WasmAnnotation::new(10, 0, 2, 0));
        t.add(WasmAnnotation::new(100, 0, 5, 0));
        assert_eq!(count_annotations_in_range(&t, 0, 20), 2);
    }
    #[test]
    fn test_annotation_line_range() {
        let mut t = WasmAnnotationTable::new();
        t.add(WasmAnnotation::new(0, 0, 3, 0));
        t.add(WasmAnnotation::new(10, 0, 7, 0));
        assert_eq!(annotation_line_range(&t), Some((3, 7)));
    }
}
/// Returns the total number of annotations in a table.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn total_annotations(table: &WasmAnnotationTable) -> usize {
    table.annotations.len()
}
/// Returns the maximum wasm offset in an annotation table, if any.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn max_wasm_offset(table: &WasmAnnotationTable) -> Option<u32> {
    table.annotations.iter().map(|a| a.wasm_offset).max()
}
/// Returns the minimum wasm offset in an annotation table, if any.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn min_wasm_offset(table: &WasmAnnotationTable) -> Option<u32> {
    table.annotations.iter().map(|a| a.wasm_offset).min()
}
#[cfg(test)]
mod wasm_pad2 {
    use super::*;
    use crate::wasm_source_map::*;
    #[test]
    fn test_total_annotations() {
        let mut t = WasmAnnotationTable::new();
        t.add(WasmAnnotation::new(0, 0, 1, 0));
        t.add(WasmAnnotation::new(4, 0, 2, 0));
        assert_eq!(total_annotations(&t), 2);
    }
    #[test]
    fn test_max_min_wasm_offset() {
        let mut t = WasmAnnotationTable::new();
        t.add(WasmAnnotation::new(0, 0, 1, 0));
        t.add(WasmAnnotation::new(100, 0, 5, 0));
        assert_eq!(max_wasm_offset(&t), Some(100));
        assert_eq!(min_wasm_offset(&t), Some(0));
    }
    #[test]
    fn test_coverage_record() {
        let mut t = WasmAnnotationTable::new();
        t.add(WasmAnnotation::new(0, 0, 1, 0));
        t.add(WasmAnnotation::new(4, 0, 2, 0));
        let mut cov = WasmCoverageRecord::new();
        cov.mark(0);
        assert!(cov.was_executed(0));
        assert!(!cov.was_executed(4));
        assert!((cov.coverage_fraction(&t) - 0.5).abs() < 1e-9);
    }
}
/// Returns annotations sorted by wasm offset.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn annotations_sorted_by_offset(table: &WasmAnnotationTable) -> Vec<&WasmAnnotation> {
    let mut anns: Vec<&WasmAnnotation> = table.annotations.iter().collect();
    anns.sort_by_key(|a| a.wasm_offset);
    anns
}
/// Returns unique source indices referenced in an annotation table.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn unique_source_indices(table: &WasmAnnotationTable) -> Vec<u32> {
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();
    for ann in &table.annotations {
        if seen.insert(ann.source_idx) {
            result.push(ann.source_idx);
        }
    }
    result.sort();
    result
}
#[cfg(test)]
mod wasm_pad3 {
    use super::*;
    use crate::wasm_source_map::*;
    #[test]
    fn test_wasm_offset_range() {
        let r = WasmOffsetRange::new(10, 20);
        assert_eq!(r.len(), 10);
        assert!(r.contains(15));
        assert!(!r.contains(5));
        let r2 = WasmOffsetRange::new(15, 25);
        let ov = r.overlap(&r2);
        assert_eq!(ov, Some(WasmOffsetRange::new(15, 20)));
    }
    #[test]
    fn test_unique_source_indices() {
        let mut t = WasmAnnotationTable::new();
        t.add(WasmAnnotation::new(0, 0, 1, 0));
        t.add(WasmAnnotation::new(4, 1, 2, 0));
        t.add(WasmAnnotation::new(8, 0, 3, 0));
        let idxs = unique_source_indices(&t);
        assert_eq!(idxs, vec![0, 1]);
    }
}
