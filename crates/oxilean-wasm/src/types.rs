//! Type definitions for the WASM API
#![allow(dead_code)]

/// Represents a check result from OxiLean
#[derive(Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize))]
pub struct CheckResult {
    pub success: bool,
    pub declarations: Vec<DeclInfo>,
    pub errors: Vec<ErrorInfo>,
    pub warnings: Vec<WarningInfo>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize))]
pub struct DeclInfo {
    pub name: String,
    pub kind: DeclKind,
    pub ty: String,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize))]
#[cfg_attr(feature = "wasm", serde(rename_all = "camelCase"))]
pub enum DeclKind {
    Theorem,
    Definition,
    Axiom,
    Inductive,
    Structure,
    Class,
    Instance,
    Other(String),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize))]
pub struct ErrorInfo {
    pub message: String,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub source: Option<String>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize))]
pub struct WarningInfo {
    pub message: String,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize))]
pub struct ReplResult {
    pub output: String,
    pub goals: Vec<GoalInfo>,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize))]
pub struct GoalInfo {
    pub tag: String,
    pub hypotheses: Vec<HypInfo>,
    pub target: String,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize))]
pub struct HypInfo {
    pub name: String,
    pub ty: String,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize))]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionKind,
    pub detail: Option<String>,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize))]
#[cfg_attr(feature = "wasm", serde(rename_all = "camelCase"))]
pub enum CompletionKind {
    Keyword,
    Function,
    Theorem,
    Definition,
    Variable,
    Tactic,
    Snippet,
}

impl std::fmt::Display for DeclKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeclKind::Theorem => write!(f, "theorem"),
            DeclKind::Definition => write!(f, "definition"),
            DeclKind::Axiom => write!(f, "axiom"),
            DeclKind::Inductive => write!(f, "inductive"),
            DeclKind::Structure => write!(f, "structure"),
            DeclKind::Class => write!(f, "class"),
            DeclKind::Instance => write!(f, "instance"),
            DeclKind::Other(s) => write!(f, "{}", s),
        }
    }
}
