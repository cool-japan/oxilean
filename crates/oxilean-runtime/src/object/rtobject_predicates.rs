//! # RtObject - predicates Methods
//!
//! This module contains method implementations for `RtObject`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{
    PAYLOAD_MASK, TAG_ARRAY, TAG_BOOL, TAG_CHAR, TAG_CLOSURE, TAG_CTOR, TAG_EXTERNAL,
    TAG_FLOAT_BITS, TAG_HEAP, TAG_INT, TAG_IO_ACTION, TAG_SMALL_NAT, TAG_STRING, TAG_TASK,
    TAG_THUNK, TAG_UNIT,
};

use super::rtobject_type::RtObject;

impl RtObject {
    /// Get the tag byte.
    pub fn tag(&self) -> u8 {
        (self.bits >> 56) as u8
    }
    /// Check if this is an inline (non-heap) value.
    pub fn is_inline(&self) -> bool {
        let t = self.tag();
        t == TAG_SMALL_NAT
            || t == TAG_BOOL
            || t == TAG_UNIT
            || t == TAG_CHAR
            || t == TAG_CTOR
            || t == TAG_INT
            || t == TAG_FLOAT_BITS
    }
    /// Check if this is a natural number (small or big).
    pub fn is_nat(&self) -> bool {
        self.tag() == TAG_SMALL_NAT || self.tag() == TAG_HEAP
    }
    /// Check if this is a boolean.
    pub fn is_bool(&self) -> bool {
        self.tag() == TAG_BOOL
    }
    /// Check if this is the unit value.
    pub fn is_unit(&self) -> bool {
        self.tag() == TAG_UNIT
    }
    /// Check if this is a character.
    pub fn is_char(&self) -> bool {
        self.tag() == TAG_CHAR
    }
    /// Check if this is a small constructor.
    pub fn is_small_ctor(&self) -> bool {
        self.tag() == TAG_CTOR
    }
    /// Check if this is a closure reference.
    pub fn is_closure(&self) -> bool {
        self.tag() == TAG_CLOSURE || self.tag() == TAG_HEAP
    }
    /// Check if this is a string reference.
    pub fn is_string_ref(&self) -> bool {
        self.tag() == TAG_STRING
    }
    /// Check if this is an array reference.
    pub fn is_array_ref(&self) -> bool {
        self.tag() == TAG_ARRAY
    }
    /// Check if this is a thunk reference.
    pub fn is_thunk_ref(&self) -> bool {
        self.tag() == TAG_THUNK
    }
    /// Check if this is an IO action reference.
    pub fn is_io_action(&self) -> bool {
        self.tag() == TAG_IO_ACTION
    }
    /// Check if this is a task reference.
    pub fn is_task_ref(&self) -> bool {
        self.tag() == TAG_TASK
    }
    /// Check if this is an external object reference.
    pub fn is_external_ref(&self) -> bool {
        self.tag() == TAG_EXTERNAL
    }
    /// Extract a boolean value.
    pub fn as_bool(&self) -> Option<bool> {
        if self.tag() != TAG_BOOL {
            return None;
        }
        Some(self.payload() != 0)
    }
    /// Extract a small natural number.
    pub fn as_small_nat(&self) -> Option<u64> {
        if self.tag() != TAG_SMALL_NAT {
            return None;
        }
        Some(self.payload())
    }
    /// Extract a character value.
    pub fn as_char(&self) -> Option<char> {
        if self.tag() != TAG_CHAR {
            return None;
        }
        char::from_u32(self.payload() as u32)
    }
    /// Extract a small constructor index.
    pub fn as_small_ctor(&self) -> Option<u32> {
        if self.tag() != TAG_CTOR {
            return None;
        }
        Some(self.payload() as u32)
    }
    /// Extract a small signed integer.
    pub fn as_small_int(&self) -> Option<i64> {
        if self.tag() != TAG_INT {
            return None;
        }
        let payload = self.payload();
        if (payload >> 55) & 1 == 1 {
            Some((payload | !PAYLOAD_MASK) as i64)
        } else {
            Some(payload as i64)
        }
    }
    /// Extract float bits.
    pub fn as_float_bits(&self) -> Option<u64> {
        if self.tag() != TAG_FLOAT_BITS {
            return None;
        }
        Some(self.payload())
    }
}
