//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// A memory layout descriptor for a Rust/C type.
///
/// Tracks size, alignment, and stride (size rounded up to alignment).
#[allow(dead_code)]
pub struct MemoryLayout {
    /// Name of the type.
    pub type_name: String,
    /// Size of the type in bytes.
    pub size: usize,
    /// Alignment of the type in bytes.
    pub align: usize,
}
#[allow(dead_code)]
impl MemoryLayout {
    /// Create a new `MemoryLayout`.
    pub fn new(type_name: impl Into<String>, size: usize, align: usize) -> Self {
        Self {
            type_name: type_name.into(),
            size,
            align,
        }
    }
    /// The stride: size rounded up to the alignment boundary.
    pub fn stride(&self) -> usize {
        (self.size + self.align - 1) & !(self.align - 1)
    }
    /// Check whether this layout is valid (align is a power of two, size ≤ stride).
    pub fn is_valid(&self) -> bool {
        self.align > 0 && self.align.is_power_of_two() && self.size <= self.stride()
    }
    /// Compute the offset needed to align `offset` to this type's alignment.
    pub fn align_offset(&self, offset: usize) -> usize {
        let mask = self.align - 1;
        (offset + mask) & !mask
    }
    /// Return a description of this layout.
    pub fn describe(&self) -> String {
        format!(
            "{}: size={}, align={}, stride={}",
            self.type_name,
            self.size,
            self.align,
            self.stride()
        )
    }
}
/// A C-ABI struct layout: a list of fields with names, sizes, and alignments.
#[allow(dead_code)]
pub struct CStructLayout {
    /// Name of the struct.
    pub name: String,
    /// Fields: (name, size, alignment).
    pub fields: Vec<(String, usize, usize)>,
}
#[allow(dead_code)]
impl CStructLayout {
    /// Create a new (empty) `CStructLayout`.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            fields: Vec::new(),
        }
    }
    /// Add a field with the given name, size, and alignment.
    pub fn add_field(mut self, name: impl Into<String>, size: usize, align: usize) -> Self {
        self.fields.push((name.into(), size, align));
        self
    }
    /// Compute the byte offset of the n-th field following C ABI rules.
    pub fn field_offset(&self, idx: usize) -> usize {
        let mut offset = 0usize;
        for (i, (_, size, align)) in self.fields.iter().enumerate() {
            let mask = align - 1;
            offset = (offset + mask) & !mask;
            if i == idx {
                return offset;
            }
            offset += size;
        }
        offset
    }
    /// Total size of the struct including trailing padding.
    pub fn total_size(&self) -> usize {
        if self.fields.is_empty() {
            return 0;
        }
        let max_align = self.fields.iter().map(|(_, _, a)| *a).max().unwrap_or(1);
        let mut offset = 0usize;
        for (_, size, align) in &self.fields {
            let mask = align - 1;
            offset = (offset + mask) & !mask;
            offset += size;
        }
        let mask = max_align - 1;
        (offset + mask) & !mask
    }
    /// Describe this struct layout.
    pub fn describe(&self) -> String {
        let offsets: Vec<String> = (0..self.fields.len())
            .map(|i| {
                format!(
                    "  [{}] {} @ offset {}",
                    i,
                    self.fields[i].0,
                    self.field_offset(i)
                )
            })
            .collect();
        format!(
            "struct {} {{\n{}\n}} size={}",
            self.name,
            offsets.join("\n"),
            self.total_size()
        )
    }
}
/// A pointer representation descriptor (tagged, fat, raw, vtable).
#[allow(dead_code)]
pub enum PointerKind {
    /// A simple raw pointer.
    Raw,
    /// A fat pointer: (data, metadata).
    Fat { metadata_size: usize },
    /// A tagged pointer: low bits encode a tag.
    Tagged { tag_bits: u32 },
    /// A vtable pointer for a trait object.
    VTable { num_methods: usize },
}
/// Represents a pointer with its kind and target address width.
#[allow(dead_code)]
pub struct PointerRepr {
    /// Kind of this pointer.
    pub kind: PointerKind,
    /// Address size in bytes (e.g. 8 on 64-bit).
    pub addr_bytes: usize,
}
#[allow(dead_code)]
impl PointerRepr {
    /// Create a raw pointer for a 64-bit target.
    pub fn raw64() -> Self {
        Self {
            kind: PointerKind::Raw,
            addr_bytes: 8,
        }
    }
    /// Create a fat pointer for a 64-bit target.
    pub fn fat64(metadata_size: usize) -> Self {
        Self {
            kind: PointerKind::Fat { metadata_size },
            addr_bytes: 8,
        }
    }
    /// Create a tagged pointer with `tag_bits` low-order tag bits.
    pub fn tagged(tag_bits: u32) -> Self {
        Self {
            kind: PointerKind::Tagged { tag_bits },
            addr_bytes: 8,
        }
    }
    /// Total size in bytes of this pointer representation.
    pub fn total_size(&self) -> usize {
        match &self.kind {
            PointerKind::Raw => self.addr_bytes,
            PointerKind::Fat { metadata_size } => self.addr_bytes + metadata_size,
            PointerKind::Tagged { .. } => self.addr_bytes,
            PointerKind::VTable { .. } => self.addr_bytes * 2,
        }
    }
    /// Describe this pointer representation.
    pub fn describe(&self) -> String {
        match &self.kind {
            PointerKind::Raw => format!("*raw ({} bytes)", self.addr_bytes),
            PointerKind::Fat { metadata_size } => {
                format!(
                    "*fat (addr={} meta={} total={})",
                    self.addr_bytes,
                    metadata_size,
                    self.total_size()
                )
            }
            PointerKind::Tagged { tag_bits } => {
                format!("*tagged ({} tag bits, {} bytes)", tag_bits, self.addr_bytes)
            }
            PointerKind::VTable { num_methods } => {
                format!(
                    "*dyn ({} methods, {} bytes)",
                    num_methods,
                    self.total_size()
                )
            }
        }
    }
}
/// Describes an IEEE 754 floating-point format.
#[allow(dead_code)]
pub struct Ieee754Descriptor {
    /// Name of the format (e.g. "binary32", "binary64").
    pub name: String,
    /// Total width in bits.
    pub total_bits: u32,
    /// Number of exponent bits.
    pub exponent_bits: u32,
    /// Number of mantissa (significand) bits (not counting implicit leading 1).
    pub mantissa_bits: u32,
}
#[allow(dead_code)]
impl Ieee754Descriptor {
    /// Create a new `Ieee754Descriptor`.
    pub fn new(name: impl Into<String>, total: u32, exp: u32, mant: u32) -> Self {
        Self {
            name: name.into(),
            total_bits: total,
            exponent_bits: exp,
            mantissa_bits: mant,
        }
    }
    /// IEEE 754 binary32 (single precision).
    pub fn binary32() -> Self {
        Self::new("binary32", 32, 8, 23)
    }
    /// IEEE 754 binary64 (double precision).
    pub fn binary64() -> Self {
        Self::new("binary64", 64, 11, 52)
    }
    /// The exponent bias: `2^(exponent_bits - 1) - 1`.
    pub fn exponent_bias(&self) -> u32 {
        (1u32 << (self.exponent_bits - 1)) - 1
    }
    /// The maximum representable finite exponent.
    pub fn max_exponent(&self) -> i32 {
        self.exponent_bias() as i32
    }
    /// Check whether the format is valid (bits add up).
    pub fn is_valid(&self) -> bool {
        self.total_bits == 1 + self.exponent_bits + self.mantissa_bits
    }
    /// Describe this format.
    pub fn describe(&self) -> String {
        format!(
            "{}: total={} sign=1 exp={} mant={} bias={}",
            self.name,
            self.total_bits,
            self.exponent_bits,
            self.mantissa_bits,
            self.exponent_bias()
        )
    }
}
/// A descriptor for a bit-field: its name, offset and width within a word.
#[allow(dead_code)]
pub struct BitfieldDescriptor {
    /// Name of the bit-field.
    pub name: String,
    /// Bit offset within the containing word.
    pub offset: u32,
    /// Width in bits.
    pub width: u32,
}
#[allow(dead_code)]
impl BitfieldDescriptor {
    /// Create a new `BitfieldDescriptor`.
    pub fn new(name: impl Into<String>, offset: u32, width: u32) -> Self {
        Self {
            name: name.into(),
            offset,
            width,
        }
    }
    /// The bitmask for this field (within a 64-bit word).
    pub fn mask(&self) -> u64 {
        let raw = if self.width >= 64 {
            !0u64
        } else {
            (1u64 << self.width) - 1
        };
        raw << self.offset
    }
    /// Extract this field's value from a word.
    pub fn extract(&self, word: u64) -> u64 {
        (word & self.mask()) >> self.offset
    }
    /// Insert `value` into `word` for this field.
    pub fn insert(&self, word: u64, value: u64) -> u64 {
        let m = self.mask();
        (word & !m) | ((value << self.offset) & m)
    }
    /// Check that this field fits within a word of `word_bits` bits.
    pub fn fits_in(&self, word_bits: u32) -> bool {
        self.offset + self.width <= word_bits
    }
}
/// The concrete state of a CPU register file (for simulation).
#[allow(dead_code)]
pub struct RegisterFileState {
    /// Register values indexed by register number.
    pub regs: Vec<u64>,
}
#[allow(dead_code)]
impl RegisterFileState {
    /// Create a zeroed register file with `count` registers.
    pub fn new(count: usize) -> Self {
        Self {
            regs: vec![0u64; count],
        }
    }
    /// Read the value of register `r`.
    pub fn read(&self, r: usize) -> Option<u64> {
        self.regs.get(r).copied()
    }
    /// Write `value` to register `r`.
    pub fn write(&mut self, r: usize, value: u64) {
        if r < self.regs.len() {
            self.regs[r] = value;
        }
    }
    /// Verify the read-after-write property for a register.
    pub fn verify_raw(&mut self, r: usize, v: u64) -> bool {
        self.write(r, v);
        self.read(r) == Some(v)
    }
    /// Number of registers.
    pub fn count(&self) -> usize {
        self.regs.len()
    }
    /// Describe the register file state.
    pub fn describe(&self) -> String {
        let parts: Vec<String> = self
            .regs
            .iter()
            .enumerate()
            .map(|(i, v)| format!("r{}=0x{:x}", i, v))
            .collect();
        format!("[{}]", parts.join(", "))
    }
}
