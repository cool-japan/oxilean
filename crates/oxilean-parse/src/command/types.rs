//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{Binder, Decl, Span};

/// Kind of notation declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotationKind {
    /// Prefix operator
    Prefix,
    /// Infix operator
    Infix,
    /// Postfix operator
    Postfix,
    /// General notation
    Notation,
}
/// Items to selectively open from a namespace.
#[derive(Debug, Clone, PartialEq)]
pub enum OpenItem {
    /// Open everything
    All,
    /// Open only these names
    Only(Vec<String>),
    /// Open everything except these
    Hiding(Vec<String>),
    /// Rename a single identifier (old, new)
    Renaming(String, String),
}
/// A top-level command.
#[derive(Debug, Clone)]
pub enum Command {
    /// Declaration command
    Decl(Decl),
    /// Import command
    Import {
        /// Module name
        module: String,
        /// Source span
        span: Span,
    },
    /// Export command
    Export {
        /// Names to export
        names: Vec<String>,
        /// Source span
        span: Span,
    },
    /// Namespace command
    Namespace {
        /// Namespace name
        name: String,
        /// Source span
        span: Span,
    },
    /// End namespace/section command
    End {
        /// Source span
        span: Span,
    },
    /// Set option command
    SetOption {
        /// Option name
        name: String,
        /// Option value
        value: String,
        /// Source span
        span: Span,
    },
    /// Open namespace command
    Open {
        /// Dotted path to namespace
        path: Vec<String>,
        /// Items to selectively open
        items: Vec<OpenItem>,
        /// Source span
        span: Span,
    },
    /// Section command
    Section {
        /// Section name
        name: String,
        /// Source span
        span: Span,
    },
    /// Variable declaration
    Variable {
        /// Binders declared
        binders: Vec<Binder>,
        /// Source span
        span: Span,
    },
    /// Attribute command
    Attribute {
        /// Attribute names
        attrs: Vec<String>,
        /// Target name
        name: String,
        /// Source span
        span: Span,
    },
    /// Check command (#check)
    Check {
        /// Expression to check (as string)
        expr_str: String,
        /// Source span
        span: Span,
    },
    /// Eval command (#eval)
    Eval {
        /// Expression to evaluate (as string)
        expr_str: String,
        /// Source span
        span: Span,
    },
    /// Print command (#print)
    Print {
        /// Name to print
        name: String,
        /// Source span
        span: Span,
    },
    /// Reduce command (#reduce)
    Reduce {
        /// Expression to reduce (as string)
        expr_str: String,
        /// Source span
        span: Span,
    },
    /// Universe declaration
    Universe {
        /// Universe variable names
        names: Vec<String>,
        /// Source span
        span: Span,
    },
    /// Notation declaration
    Notation {
        /// Notation kind
        kind: NotationKind,
        /// Operator or notation name
        name: String,
        /// Precedence level
        prec: Option<u32>,
        /// Body (definition) of the notation
        body: String,
        /// Source span
        span: Span,
    },
    /// Derive command
    Derive {
        /// Strategies to derive (e.g. DecidableEq, Repr)
        strategies: Vec<String>,
        /// Type name to derive for
        type_name: String,
        /// Source span
        span: Span,
    },
    /// Structure command with fields and optional extends
    Structure {
        /// Structure name
        name: String,
        /// Extends clause (parent structures)
        extends: Vec<String>,
        /// Field declarations
        fields: Vec<StructureField>,
        /// Derive strategies
        derives: Vec<String>,
        /// Source span
        span: Span,
    },
    /// Class command
    Class {
        /// Class name
        name: String,
        /// Class parameters (binders)
        params: Vec<Binder>,
        /// Extends clause
        extends: Vec<String>,
        /// Methods/fields
        fields: Vec<StructureField>,
        /// Source span
        span: Span,
    },
    /// Instance declaration with priority
    Instance {
        /// Instance name
        name: String,
        /// Instance type
        ty: String,
        /// Priority (optional)
        priority: Option<u32>,
        /// Implementation body
        body: String,
        /// Source span
        span: Span,
    },
    /// Attribute declaration
    AttributeDecl {
        /// Attribute name
        name: String,
        /// Attribute kind
        kind: AttributeDeclKind,
        /// Source span
        span: Span,
    },
    /// Attribute application
    ApplyAttribute {
        /// Attribute name
        attr_name: String,
        /// Target name
        target: String,
        /// Attribute parameters
        params: Vec<String>,
        /// Source span
        span: Span,
    },
    /// Syntax command
    Syntax {
        /// Syntax name
        name: String,
        /// Precedence
        prec: Option<u32>,
        /// Pattern
        pattern: String,
        /// Source span
        span: Span,
    },
    /// Precedence declaration
    Precedence {
        /// Operator name
        name: String,
        /// Precedence level
        level: u32,
        /// Source span
        span: Span,
    },
}
/// Attribute declaration kind
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributeDeclKind {
    /// Simple attribute
    Simple,
    /// Attribute with macro expansion
    Macro,
    /// Attribute with builtin implementation
    Builtin,
}
/// Field declaration for structures and classes
#[derive(Debug, Clone, PartialEq)]
pub struct StructureField {
    /// Field name
    pub name: String,
    /// Field type
    pub ty: String,
    /// Whether field is explicit or implicit
    pub is_explicit: bool,
    /// Default value (if any)
    pub default: Option<String>,
}
