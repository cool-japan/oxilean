use super::super::functions::access_flags;
use super::super::functions::JvmResult;
use crate::lcnf::*;
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JVMLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}

/// Analysis cache for JVMExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct JVMExtCache {
    pub(crate) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(crate) cap: usize,
    pub(crate) total_hits: u64,
    pub(crate) total_misses: u64,
}

/// Constant folding helper for JVMExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct JVMExtConstFolder {
    pub(crate) folds: usize,
    pub(crate) failures: usize,
    pub(crate) enabled: bool,
}

/// Code-generation errors for the JVM backend.
#[derive(Debug, Clone)]
pub enum JvmCodegenError {
    /// An unsupported LCNF construct was encountered.
    Unsupported(String),
    /// A name lookup failed.
    UnknownVar(String),
    /// Internal invariant violation.
    Internal(String),
}

/// The JVM backend that compiles LCNF to a `JvmClass` IR.
pub struct JvmBackend {
    pub(crate) config: JvmConfig,
    /// Counter for generating unique label names.
    pub(crate) label_counter: u32,
    /// Mapping from LCNF variable names to local-variable slot numbers.
    pub(crate) locals: HashMap<String, u16>,
    /// Next available local-variable slot.
    pub(crate) next_local: u16,
}

/// A single JVM instruction together with optional debug metadata.
#[derive(Debug, Clone)]
pub struct JvmInstruction {
    /// The actual opcode (and its inline operands).
    pub opcode: JvmOpcode,
    /// Optional source-line number for debugging.
    pub line: Option<u32>,
}

/// A representative subset of JVM bytecode opcodes.
///
/// Names follow the JVM specification closely, with `_` appended where the
/// mnemonic is a Rust keyword (e.g. `Return_`).
#[derive(Debug, Clone, PartialEq)]
pub enum JvmOpcode {
    /// Push `null` reference onto the operand stack.
    AconstNull,
    /// Push `int` constant −1 through 5 (`iconst_<i>`).
    Iconst(i32),
    /// Push `long` constant 0 or 1 (`lconst_<l>`).
    Lconst(i64),
    /// Push `float` constant 0.0, 1.0, or 2.0 (`fconst_<f>`).
    Fconst(f32),
    /// Push `double` constant 0.0 or 1.0 (`dconst_<d>`).
    Dconst(f64),
    /// Push `byte` immediate as `int` (`bipush`).
    Bipush(i8),
    /// Push `short` immediate as `int` (`sipush`).
    Sipush(i16),
    /// Load constant from constant pool (`ldc` / `ldc_w` / `ldc2_w`).
    Ldc(u16),
    /// Load `int` from local variable `n` (`iload`).
    Iload(u16),
    /// Load `long` from local variable `n` (`lload`).
    Lload(u16),
    /// Load `float` from local variable `n` (`fload`).
    Fload(u16),
    /// Load `double` from local variable `n` (`dload`).
    Dload(u16),
    /// Load reference from local variable `n` (`aload`).
    Aload(u16),
    /// Store `int` to local variable `n` (`istore`).
    Istore(u16),
    /// Store `long` to local variable `n` (`lstore`).
    Lstore(u16),
    /// Store `float` to local variable `n` (`fstore`).
    Fstore(u16),
    /// Store `double` to local variable `n` (`dstore`).
    Dstore(u16),
    /// Store reference to local variable `n` (`astore`).
    Astore(u16),
    /// Load `int` from array (`iaload`).
    Iaload,
    /// Load reference from array (`aaload`).
    Aaload,
    /// Store `int` into array (`iastore`).
    Iastore,
    /// Store reference into array (`aastore`).
    Aastore,
    /// Discard top value (`pop`).
    Pop,
    /// Discard top one or two values (`pop2`).
    Pop2,
    /// Duplicate the top value (`dup`).
    Dup,
    /// Swap the two top values (`swap`).
    Swap,
    /// Add two `int` values (`iadd`).
    Iadd,
    /// Subtract two `int` values (`isub`).
    Isub,
    /// Multiply two `int` values (`imul`).
    Imul,
    /// Divide two `int` values (`idiv`).
    Idiv,
    /// `int` remainder (`irem`).
    Irem,
    /// Negate `int` (`ineg`).
    Ineg,
    /// Add two `long` values (`ladd`).
    Ladd,
    /// Subtract two `long` values (`lsub`).
    Lsub,
    /// Multiply two `long` values (`lmul`).
    Lmul,
    /// Divide two `long` values (`ldiv`).
    Ldiv,
    /// Add two `double` values (`dadd`).
    Dadd,
    /// Subtract two `double` values (`dsub`).
    Dsub,
    /// Multiply two `double` values (`dmul`).
    Dmul,
    /// Divide two `double` values (`ddiv`).
    Ddiv,
    /// Convert `int` to `long` (`i2l`).
    I2l,
    /// Convert `int` to `double` (`i2d`).
    I2d,
    /// Convert `long` to `int` (`l2i`).
    L2i,
    /// Convert `double` to `int` (`d2i`).
    D2i,
    /// Compare two `int` values and branch if equal (`if_icmpeq`).
    IfIcmpeq(i16),
    /// Compare two `int` values and branch if not equal (`if_icmpne`).
    IfIcmpne(i16),
    /// Compare two `int` values and branch if less than (`if_icmplt`).
    IfIcmplt(i16),
    /// Compare two `int` values and branch if greater than or equal (`if_icmpge`).
    IfIcmpge(i16),
    /// Compare two `int` values and branch if greater than (`if_icmpgt`).
    IfIcmpgt(i16),
    /// Compare two `int` values and branch if less than or equal (`if_icmple`).
    IfIcmple(i16),
    /// Branch if reference is `null` (`ifnull`).
    Ifnull(i16),
    /// Branch if reference is not `null` (`ifnonnull`).
    Ifnonnull(i16),
    /// Branch if `int` is zero (`ifeq`).
    Ifeq(i16),
    /// Branch if `int` is non-zero (`ifne`).
    Ifne(i16),
    /// Compare two `long` values; push −1, 0, or 1 (`lcmp`).
    Lcmp,
    /// Unconditional branch (`goto`).
    Goto(i16),
    /// Return `void` from method (`return`).
    Return_,
    /// Return `int` from method (`ireturn`).
    Ireturn,
    /// Return `long` from method (`lreturn`).
    Lreturn,
    /// Return `float` from method (`freturn`).
    Freturn,
    /// Return `double` from method (`dreturn`).
    Dreturn,
    /// Return reference from method (`areturn`).
    Areturn,
    /// Throw an exception (`athrow`).
    Athrow,
    /// Get instance field value (`getfield`).
    Getfield {
        class: String,
        name: String,
        descriptor: String,
    },
    /// Set instance field value (`putfield`).
    Putfield {
        class: String,
        name: String,
        descriptor: String,
    },
    /// Get static field value (`getstatic`).
    Getstatic {
        class: String,
        name: String,
        descriptor: String,
    },
    /// Set static field value (`putstatic`).
    Putstatic {
        class: String,
        name: String,
        descriptor: String,
    },
    /// Invoke instance method (`invokevirtual`).
    Invokevirtual {
        class: String,
        name: String,
        descriptor: String,
    },
    /// Invoke interface method (`invokeinterface`).
    Invokeinterface {
        class: String,
        name: String,
        descriptor: String,
        count: u8,
    },
    /// Invoke a special (constructor / private / super) method (`invokespecial`).
    Invokespecial {
        class: String,
        name: String,
        descriptor: String,
    },
    /// Invoke a static method (`invokestatic`).
    Invokestatic {
        class: String,
        name: String,
        descriptor: String,
    },
    /// Create new object (`new`).
    New(String),
    /// Create new array of primitive type (`newarray`).
    Newarray(JvmType),
    /// Create new array of reference type (`anewarray`).
    Anewarray(String),
    /// Get array length (`arraylength`).
    Arraylength,
    /// Check whether object is instance of class (`instanceof`).
    Instanceof(String),
    /// Cast object to class, throwing if incompatible (`checkcast`).
    Checkcast(String),
    /// Label pseudo-instruction used for branch target resolution.
    Label(String),
    /// Increment local variable by constant (`iinc`).
    Iinc { index: u16, constant: i16 },
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum JVMPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}

/// An entry in the class-file constant pool.
#[derive(Debug, Clone, PartialEq)]
pub enum ConstantPoolEntry {
    /// CONSTANT_Utf8 — string data
    Utf8(String),
    /// CONSTANT_Integer — 32-bit integer constant
    Integer(i32),
    /// CONSTANT_Long — 64-bit integer constant
    Long(i64),
    /// CONSTANT_Float — 32-bit float constant
    Float(f32),
    /// CONSTANT_Double — 64-bit float constant
    Double(f64),
    /// CONSTANT_Class — reference to a class by Utf8 index
    Class { name_index: u16 },
    /// CONSTANT_String — reference to a Utf8 string value
    StringRef { string_index: u16 },
    /// CONSTANT_Fieldref
    Fieldref {
        class_index: u16,
        name_and_type_index: u16,
    },
    /// CONSTANT_Methodref
    Methodref {
        class_index: u16,
        name_and_type_index: u16,
    },
    /// CONSTANT_InterfaceMethodref
    InterfaceMethodref {
        class_index: u16,
        name_and_type_index: u16,
    },
    /// CONSTANT_NameAndType
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JVMPassConfig {
    pub phase: JVMPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}

#[allow(dead_code)]
pub struct JVMConstantFoldingHelper;

/// Liveness analysis for JVMExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct JVMExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}

/// Worklist for JVMExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JVMExtWorklist {
    pub(crate) items: std::collections::VecDeque<usize>,
    pub(crate) present: Vec<bool>,
}

/// Complete representation of a JVM class file (IR level).
#[derive(Debug, Clone)]
pub struct JvmClass {
    /// Binary class name using `/` separators (e.g. `"com/example/Foo"`).
    pub name: String,
    /// Superclass binary name (`"java/lang/Object"` by default).
    pub superclass: String,
    /// Implemented interfaces (binary names).
    pub interfaces: Vec<String>,
    /// Instance and static fields.
    pub fields: Vec<JvmField>,
    /// Methods.
    pub methods: Vec<JvmMethod>,
    /// Class-level access flags.
    pub access_flags: u16,
    /// Constant pool.
    pub constant_pool: ConstantPool,
    /// Class-file major version (e.g. 65 = Java 21).
    pub major_version: u16,
    /// Source file attribute (optional).
    pub source_file: Option<String>,
}

/// Configuration for JVMExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JVMExtPassConfig {
    pub name: String,
    pub phase: JVMExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}

/// Dominator tree for JVMExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JVMExtDomTree {
    pub(crate) idom: Vec<Option<usize>>,
    pub(crate) children: Vec<Vec<usize>>,
    pub(crate) depth: Vec<usize>,
}

/// Pass registry for JVMExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct JVMExtPassRegistry {
    pub(crate) configs: Vec<JVMExtPassConfig>,
    pub(crate) stats: Vec<JVMExtPassStats>,
}

/// Pass execution phase for JVMExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JVMExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}

/// A single JVM method (Code attribute + metadata).
#[derive(Debug, Clone)]
pub struct JvmMethod {
    /// Simple method name (e.g. `"<init>"`, `"apply"`).
    pub name: String,
    /// Method descriptor string (e.g. `"(I)V"`).
    pub descriptor: String,
    /// Access flags bitmask.
    pub access_flags: u16,
    /// Bytecode instructions.
    pub code: Vec<JvmInstruction>,
    /// Maximum operand-stack depth.
    pub max_stack: u16,
    /// Maximum number of local variables (including `this`).
    pub max_locals: u16,
    /// Exception table entries.
    pub exceptions: Vec<ExceptionEntry>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JVMAnalysisCache {
    pub(crate) entries: std::collections::HashMap<String, JVMCacheEntry>,
    pub(crate) max_size: usize,
    pub(crate) hits: u64,
    pub(crate) misses: u64,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JVMCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}

/// Statistics for JVMExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct JVMExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}

/// Dependency graph for JVMExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JVMExtDepGraph {
    pub(crate) n: usize,
    pub(crate) adj: Vec<Vec<usize>>,
    pub(crate) rev: Vec<Vec<usize>>,
    pub(crate) edge_count: usize,
}

/// One row of the JVM exception table.
#[derive(Debug, Clone)]
pub struct ExceptionEntry {
    /// Start of the try region (instruction index, not byte offset).
    pub start: u16,
    /// Exclusive end of the try region.
    pub end: u16,
    /// Handler instruction index.
    pub handler: u16,
    /// Catch type class name (`None` → finally / catch-all).
    pub catch_type: Option<String>,
}

/// Method descriptor helper.
#[derive(Debug, Clone)]
pub struct MethodDescriptor {
    /// Parameter types.
    pub params: Vec<JvmType>,
    /// Return type.
    pub return_type: JvmType,
}

/// JVM field/method descriptor types.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JvmType {
    /// `B` — signed 8-bit integer
    Byte,
    /// `S` — signed 16-bit integer
    Short,
    /// `I` — signed 32-bit integer
    Int,
    /// `J` — signed 64-bit integer
    Long,
    /// `F` — 32-bit IEEE 754 float
    Float,
    /// `D` — 64-bit IEEE 754 double
    Double,
    /// `Z` — boolean
    Boolean,
    /// `C` — UTF-16 code unit
    Char,
    /// `V` — no return value
    Void,
    /// `Lclass/name;` — object reference
    Object(String),
    /// `[T` — array of T
    Array(Box<JvmType>),
    /// Erased generic type variable (represented as `java/lang/Object`)
    Generic(String),
}

#[allow(dead_code)]
pub struct JVMPassRegistry {
    pub(crate) configs: Vec<JVMPassConfig>,
    pub(crate) stats: std::collections::HashMap<String, JVMPassStats>,
}

/// Configuration for the JVM backend.
#[derive(Debug, Clone)]
pub struct JvmConfig {
    /// Package prefix (e.g. `"com.example"`).
    pub package: String,
    /// Java class-file major version (default 65 = Java 21).
    pub class_version: u16,
    /// Emit debug line-number tables.
    pub emit_line_numbers: bool,
    /// Whether to generate sealed-interface hierarchies for ADT types.
    pub sealed_adt: bool,
}

/// Minimal constant pool builder.
#[derive(Debug, Clone, Default)]
pub struct ConstantPool {
    pub(crate) entries: Vec<ConstantPoolEntry>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JVMWorklist {
    pub(crate) items: std::collections::VecDeque<u32>,
    pub(crate) in_worklist: std::collections::HashSet<u32>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JVMDepGraph {
    pub(crate) nodes: Vec<u32>,
    pub(crate) edges: Vec<(u32, u32)>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JVMDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}

/// A field (instance or static) inside a JVM class.
#[derive(Debug, Clone)]
pub struct JvmField {
    /// Field name.
    pub name: String,
    /// Field descriptor (e.g. `"I"`, `"Ljava/lang/String;"`).
    pub descriptor: String,
    /// Access flags bitmask.
    pub access_flags: u16,
    /// Optional constant-value attribute (for static final fields).
    pub constant_value: Option<ConstantPoolEntry>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct JVMPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
