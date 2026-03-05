//! # RtObject - Trait Implementations
//!
//! This module contains trait implementations for `RtObject`.
//!
//! ## Implemented Traits
//!
//! - `Debug`
//! - `Display`
//! - `PartialEq`
//! - `Eq`
//! - `Hash`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::fmt;
use std::hash::{Hash, Hasher};

use super::functions::{
    TAG_ARRAY, TAG_BOOL, TAG_CHAR, TAG_CLOSURE, TAG_CTOR, TAG_EXTERNAL, TAG_FLOAT_BITS, TAG_HEAP,
    TAG_INT, TAG_IO_ACTION, TAG_SMALL_NAT, TAG_STRING, TAG_TASK, TAG_THUNK, TAG_UNIT,
};
use super::rtobject_type::RtObject;

impl fmt::Debug for RtObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.tag() {
            TAG_SMALL_NAT => write!(f, "RtObject::Nat({})", self.payload()),
            TAG_BOOL => write!(f, "RtObject::Bool({})", self.payload() != 0),
            TAG_UNIT => write!(f, "RtObject::Unit"),
            TAG_CHAR => {
                if let Some(c) = self.as_char() {
                    write!(f, "RtObject::Char({:?})", c)
                } else {
                    write!(f, "RtObject::Char(<invalid>)")
                }
            }
            TAG_CTOR => write!(f, "RtObject::Ctor({})", self.payload()),
            TAG_INT => {
                if let Some(n) = self.as_small_int() {
                    write!(f, "RtObject::Int({})", n)
                } else {
                    write!(f, "RtObject::Int(<invalid>)")
                }
            }
            TAG_FLOAT_BITS => write!(f, "RtObject::Float(bits={})", self.payload()),
            TAG_HEAP => write!(f, "RtObject::Heap(id={})", self.payload()),
            TAG_CLOSURE => write!(f, "RtObject::Closure(id={})", self.payload()),
            TAG_ARRAY => write!(f, "RtObject::Array(id={})", self.payload()),
            TAG_STRING => write!(f, "RtObject::String(id={})", self.payload()),
            TAG_THUNK => write!(f, "RtObject::Thunk(id={})", self.payload()),
            TAG_IO_ACTION => write!(f, "RtObject::IoAction(id={})", self.payload()),
            TAG_TASK => write!(f, "RtObject::Task(id={})", self.payload()),
            TAG_EXTERNAL => write!(f, "RtObject::External(id={})", self.payload()),
            _ => {
                write!(
                    f,
                    "RtObject::Unknown(tag={}, payload={})",
                    self.tag(),
                    self.payload()
                )
            }
        }
    }
}

impl fmt::Display for RtObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.tag() {
            TAG_SMALL_NAT => write!(f, "{}", self.payload()),
            TAG_BOOL => {
                if self.payload() != 0 {
                    write!(f, "true")
                } else {
                    write!(f, "false")
                }
            }
            TAG_UNIT => write!(f, "()"),
            TAG_CHAR => {
                if let Some(c) = self.as_char() {
                    write!(f, "{}", c)
                } else {
                    write!(f, "<invalid char>")
                }
            }
            TAG_CTOR => write!(f, "ctor#{}", self.payload()),
            TAG_INT => {
                if let Some(n) = self.as_small_int() {
                    write!(f, "{}", n)
                } else {
                    write!(f, "<invalid int>")
                }
            }
            _ => write!(f, "<object tag={}>", self.tag()),
        }
    }
}

impl PartialEq for RtObject {
    fn eq(&self, other: &Self) -> bool {
        self.bits == other.bits
    }
}

impl Eq for RtObject {}

impl Hash for RtObject {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bits.hash(state);
    }
}
