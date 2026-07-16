//! A small, self-contained SHA-256 implementation.
//!
//! It lives inside the product's dependency closure, which a reviewer audits,
//! so — like the kernel's bignum — it is hand-rolled, `unsafe`-free, and short
//! enough to read by eye rather than pulling in a crypto crate. It is used only
//! to fingerprint the *input file* for the JSON report's provenance field; it is
//! never on the verdict path, so it is outside the trusted computing base in the
//! sense that matters (it cannot turn a wrong proof into a right one). It is a
//! straight transcription of FIPS 180-4 and is differential-tested against known
//! answer vectors in the unit tests below.

/// The SHA-256 round constants (first 32 bits of the fractional parts of the
/// cube roots of the first 64 primes).
const K: [u32; 64] = [
    0x428a_2f98,
    0x7137_4491,
    0xb5c0_fbcf,
    0xe9b5_dba5,
    0x3956_c25b,
    0x59f1_11f1,
    0x923f_82a4,
    0xab1c_5ed5,
    0xd807_aa98,
    0x1283_5b01,
    0x2431_85be,
    0x550c_7dc3,
    0x72be_5d74,
    0x80de_b1fe,
    0x9bdc_06a7,
    0xc19b_f174,
    0xe49b_69c1,
    0xefbe_4786,
    0x0fc1_9dc6,
    0x240c_a1cc,
    0x2de9_2c6f,
    0x4a74_84aa,
    0x5cb0_a9dc,
    0x76f9_88da,
    0x983e_5152,
    0xa831_c66d,
    0xb003_27c8,
    0xbf59_7fc7,
    0xc6e0_0bf3,
    0xd5a7_9147,
    0x06ca_6351,
    0x1429_2967,
    0x27b7_0a85,
    0x2e1b_2138,
    0x4d2c_6dfc,
    0x5338_0d13,
    0x650a_7354,
    0x766a_0abb,
    0x81c2_c92e,
    0x9272_2c85,
    0xa2bf_e8a1,
    0xa81a_664b,
    0xc24b_8b70,
    0xc76c_51a3,
    0xd192_e819,
    0xd699_0624,
    0xf40e_3585,
    0x106a_a070,
    0x19a4_c116,
    0x1e37_6c08,
    0x2748_774c,
    0x34b0_bcb5,
    0x391c_0cb3,
    0x4ed8_aa4a,
    0x5b9c_ca4f,
    0x682e_6ff3,
    0x748f_82ee,
    0x78a5_636f,
    0x84c8_7814,
    0x8cc7_0208,
    0x90be_fffa,
    0xa450_6ceb,
    0xbef9_a3f7,
    0xc671_78f2,
];

/// The eight initial hash values (first 32 bits of the fractional parts of the
/// square roots of the first 8 primes).
const H0: [u32; 8] = [
    0x6a09_e667,
    0xbb67_ae85,
    0x3c6e_f372,
    0xa54f_f53a,
    0x510e_527f,
    0x9b05_688c,
    0x1f83_d9ab,
    0x5be0_cd19,
];

/// An incremental SHA-256 hasher. Feed bytes with [`Sha256::update`], then call
/// [`Sha256::finalize_hex`] once to obtain the lowercase hex digest.
pub struct Sha256 {
    h: [u32; 8],
    /// The current partial 64-byte block.
    block: [u8; 64],
    /// Bytes currently held in `block` (`0..64`).
    filled: usize,
    /// Total message length in bytes (for the padding length field).
    total_len: u64,
}

impl Default for Sha256 {
    fn default() -> Self {
        Self::new()
    }
}

impl Sha256 {
    /// A fresh hasher seeded with the FIPS 180-4 initial values.
    #[must_use]
    pub fn new() -> Self {
        Self {
            h: H0,
            block: [0u8; 64],
            filled: 0,
            total_len: 0,
        }
    }

    /// Feed a chunk of message bytes.
    pub fn update(&mut self, mut data: &[u8]) {
        self.total_len = self.total_len.wrapping_add(data.len() as u64);
        while !data.is_empty() {
            let take = core::cmp::min(64 - self.filled, data.len());
            self.block[self.filled..self.filled + take].copy_from_slice(&data[..take]);
            self.filled += take;
            data = &data[take..];
            if self.filled == 64 {
                let block = self.block;
                self.compress(&block);
                self.filled = 0;
            }
        }
    }

    /// Consume the hasher and return the 64-character lowercase hex digest.
    #[must_use]
    pub fn finalize_hex(mut self) -> String {
        // Length in bits, captured before we append padding.
        let bit_len = self.total_len.wrapping_mul(8);

        // Append the mandatory 0x80 byte, then zero-pad to a 56 mod 64 boundary,
        // then the 64-bit big-endian bit length.
        self.update(&[0x80]);
        while self.filled != 56 {
            self.update(&[0x00]);
        }
        self.update(&bit_len.to_be_bytes());
        // `filled` is now 0: the final block was just compressed.

        let mut out = String::with_capacity(64);
        for word in self.h {
            for byte in word.to_be_bytes() {
                push_hex_byte(&mut out, byte);
            }
        }
        out
    }

    /// The FIPS 180-4 compression function over one 64-byte block.
    fn compress(&mut self, block: &[u8; 64]) {
        let mut w = [0u32; 64];
        for (i, chunk) in block.chunks_exact(4).enumerate() {
            w[i] = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }

        let mut a = self.h[0];
        let mut b = self.h[1];
        let mut c = self.h[2];
        let mut d = self.h[3];
        let mut e = self.h[4];
        let mut f = self.h[5];
        let mut g = self.h[6];
        let mut h = self.h[7];

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let t1 = h
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }

        self.h[0] = self.h[0].wrapping_add(a);
        self.h[1] = self.h[1].wrapping_add(b);
        self.h[2] = self.h[2].wrapping_add(c);
        self.h[3] = self.h[3].wrapping_add(d);
        self.h[4] = self.h[4].wrapping_add(e);
        self.h[5] = self.h[5].wrapping_add(f);
        self.h[6] = self.h[6].wrapping_add(g);
        self.h[7] = self.h[7].wrapping_add(h);
    }
}

/// Append a byte as two lowercase hex digits.
fn push_hex_byte(out: &mut String, byte: u8) {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    out.push(HEX[(byte >> 4) as usize] as char);
    out.push(HEX[(byte & 0x0f) as usize] as char);
}

/// Convenience: hash a byte slice and return the lowercase hex digest.
#[must_use]
pub fn hex_digest(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize_hex()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        assert_eq!(
            hex_digest(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn abc() {
        assert_eq!(
            hex_digest(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn two_block_vector() {
        // FIPS 180-4 example B.2: spans a block boundary (56 bytes).
        assert_eq!(
            hex_digest(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
        );
    }

    #[test]
    fn incremental_matches_oneshot() {
        let data = b"The quick brown fox jumps over the lazy dog";
        let oneshot = hex_digest(data);
        let mut h = Sha256::new();
        // Feed in awkward chunk sizes to exercise the block buffering.
        h.update(&data[..1]);
        h.update(&data[1..7]);
        h.update(&data[7..40]);
        h.update(&data[40..]);
        assert_eq!(h.finalize_hex(), oneshot);
        assert_eq!(
            oneshot,
            "d7a8fbb307d7809469ca9abcb0082e4f8d5651e46d3cdb762d02d0bf37c9e592"
        );
    }

    #[test]
    fn long_message_multi_block() {
        // 1,000,000 'a' characters — the classic FIPS long test.
        let data = vec![b'a'; 1_000_000];
        assert_eq!(
            hex_digest(&data),
            "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0"
        );
    }
}
