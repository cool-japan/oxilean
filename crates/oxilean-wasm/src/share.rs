//! Share-via-URL support for the OxiLean playground.
//!
//! Provides compression (OxiARC DEFLATE) + base64url encoding so that a proof
//! session can be serialised into a URL fragment and later restored by anyone
//! who opens that URL.
//!
//! The pure-Rust helpers (`base64url_encode`, `base64url_decode`) are always
//! compiled.  The `#[wasm_bindgen]`-annotated entry points are only compiled
//! when targeting `wasm32`, keeping the host-target build (and its tests) clean.

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

// ── base64url (RFC 4648 §5, no padding) ─────────────────────────────────────

const BASE64URL_TABLE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

/// Encode arbitrary bytes as a URL-safe base64url string (no `+`, `/`, or `=`).
pub(crate) fn base64url_encode(data: &[u8]) -> String {
    let mut out = String::with_capacity((data.len() * 4 + 2) / 3);
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as usize;
        let b1 = if chunk.len() > 1 {
            chunk[1] as usize
        } else {
            0
        };
        let b2 = if chunk.len() > 2 {
            chunk[2] as usize
        } else {
            0
        };
        let v = (b0 << 16) | (b1 << 8) | b2;
        out.push(BASE64URL_TABLE[(v >> 18) & 0x3f] as char);
        out.push(BASE64URL_TABLE[(v >> 12) & 0x3f] as char);
        if chunk.len() > 1 {
            out.push(BASE64URL_TABLE[(v >> 6) & 0x3f] as char);
        }
        if chunk.len() > 2 {
            out.push(BASE64URL_TABLE[v & 0x3f] as char);
        }
    }
    out
}

/// Decode a URL-safe base64url string (no padding required).
///
/// Returns `Err(String)` if the input contains characters outside the
/// base64url alphabet.
pub(crate) fn base64url_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.is_empty() {
        return Ok(Vec::new());
    }

    let indices: Result<Vec<u8>, String> = s
        .chars()
        .map(|c| match c {
            'A'..='Z' => Ok(c as u8 - b'A'),
            'a'..='z' => Ok(c as u8 - b'a' + 26),
            '0'..='9' => Ok(c as u8 - b'0' + 52),
            '-' => Ok(62),
            '_' => Ok(63),
            other => Err(format!("invalid base64url character: {:?}", other)),
        })
        .collect();
    let chars = indices?;

    let mut out = Vec::with_capacity((chars.len() * 3 + 3) / 4);
    for chunk in chars.chunks(4) {
        let b0 = chunk[0] as usize;
        let b1 = if chunk.len() > 1 {
            chunk[1] as usize
        } else {
            0
        };
        let b2 = if chunk.len() > 2 {
            chunk[2] as usize
        } else {
            0
        };
        let b3 = if chunk.len() > 3 {
            chunk[3] as usize
        } else {
            0
        };
        let v = (b0 << 18) | (b1 << 12) | (b2 << 6) | b3;
        out.push((v >> 16) as u8);
        if chunk.len() > 2 {
            out.push((v >> 8) as u8);
        }
        if chunk.len() > 3 {
            out.push(v as u8);
        }
    }
    Ok(out)
}

// ── WASM entry points ────────────────────────────────────────────────────────

/// Compress `source` with DEFLATE (level 6) and return a base64url-encoded
/// string safe for embedding in a URL fragment (`#<result>`).
///
/// Called from JavaScript as `compress_share(sourceCode)`.
#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "compressShare")]
pub fn compress_share(source: &str) -> Result<String, JsValue> {
    let compressed = oxiarc_deflate::deflate(source.as_bytes(), 6)
        .map_err(|e| JsValue::from_str(&format!("deflate error: {e}")))?;
    Ok(base64url_encode(&compressed))
}

/// Decompress a base64url-encoded URL fragment back to the original source text.
///
/// Called from JavaScript as `decompress_share(fragment)`.
#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "decompressShare")]
pub fn decompress_share(encoded: &str) -> Result<String, JsValue> {
    let bytes = base64url_decode(encoded)
        .map_err(|e| JsValue::from_str(&format!("base64url decode error: {e}")))?;
    let decompressed = oxiarc_deflate::inflate(&bytes)
        .map_err(|e| JsValue::from_str(&format!("inflate error: {e}")))?;
    String::from_utf8(decompressed)
        .map_err(|e| JsValue::from_str(&format!("utf-8 decode error: {e}")))
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::{base64url_decode, base64url_encode};

    #[test]
    fn test_base64url_roundtrip_basic() {
        let original = b"hello world, this is a test of oxilean playground share";
        let encoded = base64url_encode(original);
        // URL-safe: no +, /, or = characters
        assert!(!encoded.contains('+'), "must not contain '+'");
        assert!(!encoded.contains('/'), "must not contain '/'");
        assert!(!encoded.contains('='), "must not contain '='");
        let decoded = base64url_decode(&encoded).expect("decode should succeed");
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_base64url_empty() {
        let encoded = base64url_encode(b"");
        assert!(encoded.is_empty(), "empty input → empty output");
        let decoded = base64url_decode("").expect("empty decode should succeed");
        assert!(decoded.is_empty());
    }

    #[test]
    fn test_base64url_single_byte() {
        // Single byte encodes as 2 chars
        let encoded = base64url_encode(b"A");
        assert_eq!(encoded.len(), 2);
        let decoded = base64url_decode(&encoded).expect("single byte roundtrip");
        assert_eq!(decoded, b"A");
    }

    #[test]
    fn test_base64url_two_bytes() {
        let encoded = base64url_encode(b"AB");
        assert_eq!(encoded.len(), 3);
        let decoded = base64url_decode(&encoded).expect("two byte roundtrip");
        assert_eq!(decoded, b"AB");
    }

    #[test]
    fn test_base64url_invalid_char() {
        let result = base64url_decode("abc+def");
        assert!(result.is_err(), "'+' is not valid base64url");
    }

    #[test]
    fn test_compress_decompress_roundtrip() {
        let source = "theorem test_nat_add (n : Nat) : n + 0 = n := by omega";
        let compressed =
            oxiarc_deflate::deflate(source.as_bytes(), 6).expect("deflate should succeed");
        let decompressed = oxiarc_deflate::inflate(&compressed).expect("inflate should succeed");
        assert_eq!(
            decompressed,
            source.as_bytes(),
            "roundtrip should be lossless"
        );
    }

    #[test]
    fn test_compress_decompress_unicode() {
        // OxiLean uses Unicode in theorems (∀, ∃, ∧, ∨, →)
        let source = "theorem unicode_ex (α : Type) (h : ∀ x : α, x = x) : True := trivial";
        let compressed = oxiarc_deflate::deflate(source.as_bytes(), 6).expect("deflate unicode");
        let decompressed = oxiarc_deflate::inflate(&compressed).expect("inflate unicode");
        assert_eq!(String::from_utf8(decompressed).expect("valid utf8"), source);
    }

    #[test]
    fn test_full_pipeline_roundtrip() {
        // Simulate what the WASM exports do end-to-end (minus wasm_bindgen).
        let source = "-- OxiLean playground share test\ntheorem refl_ex (n : Nat) : n = n := rfl";
        let compressed =
            oxiarc_deflate::deflate(source.as_bytes(), 6).expect("deflate full pipeline");
        let encoded = base64url_encode(&compressed);
        // Verify URL-safe
        assert!(!encoded.contains('+'));
        assert!(!encoded.contains('/'));
        assert!(!encoded.contains('='));
        // Decode back
        let bytes = base64url_decode(&encoded).expect("decode full pipeline");
        let decompressed = oxiarc_deflate::inflate(&bytes).expect("inflate full pipeline");
        let recovered = String::from_utf8(decompressed).expect("utf8 full pipeline");
        assert_eq!(recovered, source);
    }
}
