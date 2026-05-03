//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::super::functions::*;
use super::defs::*;
use super::impls1::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// Dhall expression AST.
impl DhallExtWorklist {
    #[allow(dead_code)]
    pub fn new(capacity: usize) -> Self {
        Self {
            items: std::collections::VecDeque::new(),
            present: vec![false; capacity],
        }
    }
    #[allow(dead_code)]
    pub fn push(&mut self, id: usize) {
        if id < self.present.len() && !self.present[id] {
            self.present[id] = true;
            self.items.push_back(id);
        }
    }
    #[allow(dead_code)]
    pub fn push_front(&mut self, id: usize) {
        if id < self.present.len() && !self.present[id] {
            self.present[id] = true;
            self.items.push_front(id);
        }
    }
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<usize> {
        let id = self.items.pop_front()?;
        if id < self.present.len() {
            self.present[id] = false;
        }
        Some(id)
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.items.len()
    }
    #[allow(dead_code)]
    pub fn contains(&self, id: usize) -> bool {
        id < self.present.len() && self.present[id]
    }
    #[allow(dead_code)]
    pub fn drain_all(&mut self) -> Vec<usize> {
        let v: Vec<usize> = self.items.drain(..).collect();
        for &id in &v {
            if id < self.present.len() {
                self.present[id] = false;
            }
        }
        v
    }
}
/// Pass execution phase for DhallExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DhallExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl DhallExtPassPhase {
    #[allow(dead_code)]
    pub fn is_early(&self) -> bool {
        matches!(self, Self::Early)
    }
    #[allow(dead_code)]
    pub fn is_middle(&self) -> bool {
        matches!(self, Self::Middle)
    }
    #[allow(dead_code)]
    pub fn is_late(&self) -> bool {
        matches!(self, Self::Late)
    }
    #[allow(dead_code)]
    pub fn is_finalize(&self) -> bool {
        matches!(self, Self::Finalize)
    }
    #[allow(dead_code)]
    pub fn order(&self) -> u32 {
        match self {
            Self::Early => 0,
            Self::Middle => 1,
            Self::Late => 2,
            Self::Finalize => 3,
        }
    }
    #[allow(dead_code)]
    pub fn from_order(n: u32) -> Option<Self> {
        match n {
            0 => Some(Self::Early),
            1 => Some(Self::Middle),
            2 => Some(Self::Late),
            3 => Some(Self::Finalize),
            _ => None,
        }
    }
}
/// Statistics for DhallX2 passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct DhallX2PassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl DhallX2PassStats {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn visit(&mut self) {
        self.nodes_visited += 1;
    }
    #[allow(dead_code)]
    pub fn modify(&mut self) {
        self.nodes_modified += 1;
        self.changed = true;
    }
    #[allow(dead_code)]
    pub fn iterate(&mut self) {
        self.iterations += 1;
    }
    #[allow(dead_code)]
    pub fn error(&mut self) {
        self.errors += 1;
    }
    #[allow(dead_code)]
    pub fn efficiency(&self) -> f64 {
        if self.nodes_visited == 0 {
            0.0
        } else {
            self.nodes_modified as f64 / self.nodes_visited as f64
        }
    }
    #[allow(dead_code)]
    pub fn merge(&mut self, o: &DhallX2PassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
/// A Dhall record value: `{ field1 = v1, field2 = v2 }`
#[derive(Debug, Clone, PartialEq)]
pub struct DhallRecord {
    /// Ordered list of (field name, value) pairs
    pub fields: Vec<(String, DhallExpr)>,
}
impl DhallRecord {
    /// Create a new empty record.
    pub fn new() -> Self {
        DhallRecord { fields: vec![] }
    }
    /// Add a field.
    pub fn field(mut self, name: impl Into<String>, value: DhallExpr) -> Self {
        self.fields.push((name.into(), value));
        self
    }
    pub(crate) fn emit(&self, indent: usize) -> String {
        if self.fields.is_empty() {
            return "{=}".into();
        }
        let ind2 = " ".repeat(indent + 2);
        let ind = " ".repeat(indent);
        let parts: Vec<String> = self
            .fields
            .iter()
            .map(|(k, v)| format!("{}{} = {}", ind2, k, v.emit(indent + 2)))
            .collect();
        format!("{{\n{}\n{}}}", parts.join(",\n"), ind)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum DhallPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl DhallPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            DhallPassPhase::Analysis => "analysis",
            DhallPassPhase::Transformation => "transformation",
            DhallPassPhase::Verification => "verification",
            DhallPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(
            self,
            DhallPassPhase::Transformation | DhallPassPhase::Cleanup
        )
    }
}
/// Analysis cache for DhallX2.
#[allow(dead_code)]
#[derive(Debug)]
pub struct DhallX2Cache {
    pub(crate) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(crate) cap: usize,
    pub(crate) total_hits: u64,
    pub(crate) total_misses: u64,
}
impl DhallX2Cache {
    #[allow(dead_code)]
    pub fn new(cap: usize) -> Self {
        Self {
            entries: Vec::new(),
            cap,
            total_hits: 0,
            total_misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: u64) -> Option<&[u8]> {
        for e in self.entries.iter_mut() {
            if e.0 == key && e.2 {
                e.3 += 1;
                self.total_hits += 1;
                return Some(&e.1);
            }
        }
        self.total_misses += 1;
        None
    }
    #[allow(dead_code)]
    pub fn put(&mut self, key: u64, data: Vec<u8>) {
        if self.entries.len() >= self.cap {
            self.entries.retain(|e| e.2);
            if self.entries.len() >= self.cap {
                self.entries.remove(0);
            }
        }
        self.entries.push((key, data, true, 0));
    }
    #[allow(dead_code)]
    pub fn invalidate(&mut self) {
        for e in self.entries.iter_mut() {
            e.2 = false;
        }
    }
    #[allow(dead_code)]
    pub fn hit_rate(&self) -> f64 {
        let t = self.total_hits + self.total_misses;
        if t == 0 {
            0.0
        } else {
            self.total_hits as f64 / t as f64
        }
    }
    #[allow(dead_code)]
    pub fn live_count(&self) -> usize {
        self.entries.iter().filter(|e| e.2).count()
    }
}
/// The Dhall code generation backend.
///
/// Converts OxiLean IR constructs into Dhall configuration language output.
pub struct DhallBackend {
    /// Whether to emit types with full annotation (default true)
    pub emit_annotations: bool,
}
impl DhallBackend {
    /// Create a new DhallBackend with default settings.
    pub fn new() -> Self {
        DhallBackend {
            emit_annotations: true,
        }
    }
    /// Emit a DhallExpr to string.
    pub fn emit_expr(&self, expr: &DhallExpr, indent: usize) -> String {
        expr.emit(indent)
    }
    /// Emit a complete DhallFile to string.
    pub fn emit_file(&self, file: &DhallFile) -> String {
        file.emit()
    }
    /// Emit a DhallRecord to string.
    pub fn emit_record(&self, record: &DhallRecord, indent: usize) -> String {
        record.emit(indent)
    }
    /// Emit a DhallFunction to string.
    pub fn emit_function(&self, func: &DhallFunction, indent: usize) -> String {
        func.emit(indent)
    }
    /// Build a Dhall record schema (record type) from a field map.
    pub fn make_schema(&self, fields: Vec<(&str, DhallType)>) -> DhallExpr {
        DhallExpr::RecordType(
            fields
                .into_iter()
                .map(|(k, t)| (k.to_string(), t))
                .collect(),
        )
    }
    /// Build a `List/map` application.
    pub fn make_list_map(
        &self,
        input_type: DhallType,
        output_type: DhallType,
        func: DhallExpr,
        list: DhallExpr,
    ) -> DhallExpr {
        DhallExpr::Application(
            Box::new(DhallExpr::Application(
                Box::new(DhallExpr::Application(
                    Box::new(DhallExpr::Application(
                        Box::new(DhallExpr::Var("List/map".into())),
                        Box::new(DhallExpr::BuiltinType(input_type)),
                    )),
                    Box::new(DhallExpr::BuiltinType(output_type)),
                )),
                Box::new(func),
            )),
            Box::new(list),
        )
    }
    /// Build a `Natural/fold`-style loop body.
    #[allow(clippy::too_many_arguments)]
    pub fn make_natural_fold(
        &self,
        n: DhallExpr,
        result_type: DhallType,
        succ: DhallExpr,
        zero: DhallExpr,
    ) -> DhallExpr {
        DhallExpr::Application(
            Box::new(DhallExpr::Application(
                Box::new(DhallExpr::Application(
                    Box::new(DhallExpr::Application(
                        Box::new(DhallExpr::Var("Natural/fold".into())),
                        Box::new(n),
                    )),
                    Box::new(DhallExpr::BuiltinType(result_type)),
                )),
                Box::new(succ),
            )),
            Box::new(zero),
        )
    }
    /// Build a configuration record for a service-like schema.
    #[allow(clippy::too_many_arguments)]
    pub fn make_service_config(
        &self,
        enable: bool,
        name: &str,
        port: u64,
        extra_fields: Vec<(String, DhallExpr)>,
    ) -> DhallExpr {
        let mut fields = vec![
            ("enable".to_string(), DhallExpr::BoolLit(enable)),
            ("name".to_string(), DhallExpr::TextLit(name.to_string())),
            ("port".to_string(), DhallExpr::NaturalLit(port)),
        ];
        fields.extend(extra_fields);
        DhallExpr::RecordLit(Box::new(DhallRecord { fields }))
    }
    /// Build a union type for an enumeration.
    pub fn make_enum(&self, variants: Vec<&str>) -> DhallType {
        DhallType::Union(
            variants
                .into_iter()
                .map(|v| (v.to_string(), None))
                .collect(),
        )
    }
    /// Build an Optional handling pattern with `merge`.
    pub fn make_optional_merge(
        &self,
        optional_value: DhallExpr,
        some_handler: DhallExpr,
        none_value: DhallExpr,
        result_type: DhallType,
    ) -> DhallExpr {
        let handler = DhallExpr::RecordLit(Box::new(DhallRecord {
            fields: vec![
                ("Some".to_string(), some_handler),
                ("None".to_string(), none_value),
            ],
        }));
        DhallExpr::Merge(
            Box::new(handler),
            Box::new(optional_value),
            Some(result_type),
        )
    }
    /// Generate a Dhall prelude-style package.dhall skeleton.
    pub fn make_package(&self, _module_name: &str, exports: Vec<(&str, DhallExpr)>) -> DhallFile {
        let record = DhallExpr::RecordLit(Box::new(DhallRecord {
            fields: exports
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect(),
        }));
        DhallFile::new(record).declare(DhallDecl::new(
            "version",
            DhallExpr::TextLit("1.0.0".into()),
        ))
    }
}
/// Pass execution phase for DhallX2.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DhallX2PassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl DhallX2PassPhase {
    #[allow(dead_code)]
    pub fn is_early(&self) -> bool {
        matches!(self, Self::Early)
    }
    #[allow(dead_code)]
    pub fn is_middle(&self) -> bool {
        matches!(self, Self::Middle)
    }
    #[allow(dead_code)]
    pub fn is_late(&self) -> bool {
        matches!(self, Self::Late)
    }
    #[allow(dead_code)]
    pub fn is_finalize(&self) -> bool {
        matches!(self, Self::Finalize)
    }
    #[allow(dead_code)]
    pub fn order(&self) -> u32 {
        match self {
            Self::Early => 0,
            Self::Middle => 1,
            Self::Late => 2,
            Self::Finalize => 3,
        }
    }
    #[allow(dead_code)]
    pub fn from_order(n: u32) -> Option<Self> {
        match n {
            0 => Some(Self::Early),
            1 => Some(Self::Middle),
            2 => Some(Self::Late),
            3 => Some(Self::Finalize),
            _ => None,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DhallDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl DhallDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        DhallDominatorTree {
            idom: vec![None; size],
            dom_children: vec![Vec::new(); size],
            dom_depth: vec![0; size],
        }
    }
    #[allow(dead_code)]
    pub fn set_idom(&mut self, node: usize, idom: u32) {
        self.idom[node] = Some(idom);
    }
    #[allow(dead_code)]
    pub fn dominates(&self, a: usize, b: usize) -> bool {
        if a == b {
            return true;
        }
        let mut cur = b;
        loop {
            match self.idom[cur] {
                Some(parent) if parent as usize == a => return true,
                Some(parent) if parent as usize == cur => return false,
                Some(parent) => cur = parent as usize,
                None => return false,
            }
        }
    }
    #[allow(dead_code)]
    pub fn depth(&self, node: usize) -> u32 {
        self.dom_depth.get(node).copied().unwrap_or(0)
    }
}
/// Analysis cache for DhallExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct DhallExtCache {
    pub(crate) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(crate) cap: usize,
    pub(crate) total_hits: u64,
    pub(crate) total_misses: u64,
}
impl DhallExtCache {
    #[allow(dead_code)]
    pub fn new(cap: usize) -> Self {
        Self {
            entries: Vec::new(),
            cap,
            total_hits: 0,
            total_misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: u64) -> Option<&[u8]> {
        for e in self.entries.iter_mut() {
            if e.0 == key && e.2 {
                e.3 += 1;
                self.total_hits += 1;
                return Some(&e.1);
            }
        }
        self.total_misses += 1;
        None
    }
    #[allow(dead_code)]
    pub fn put(&mut self, key: u64, data: Vec<u8>) {
        if self.entries.len() >= self.cap {
            self.entries.retain(|e| e.2);
            if self.entries.len() >= self.cap {
                self.entries.remove(0);
            }
        }
        self.entries.push((key, data, true, 0));
    }
    #[allow(dead_code)]
    pub fn invalidate(&mut self) {
        for e in self.entries.iter_mut() {
            e.2 = false;
        }
    }
    #[allow(dead_code)]
    pub fn hit_rate(&self) -> f64 {
        let t = self.total_hits + self.total_misses;
        if t == 0 {
            0.0
        } else {
            self.total_hits as f64 / t as f64
        }
    }
    #[allow(dead_code)]
    pub fn live_count(&self) -> usize {
        self.entries.iter().filter(|e| e.2).count()
    }
}
/// Dhall type-level expressions (a subset of DhallExpr, named for clarity).
#[derive(Debug, Clone, PartialEq)]
pub enum DhallType {
    /// `Bool`
    Bool,
    /// `Natural`
    Natural,
    /// `Integer`
    Integer,
    /// `Double`
    Double,
    /// `Text`
    Text,
    /// `List T`
    List(Box<DhallType>),
    /// `Optional T`
    Optional(Box<DhallType>),
    /// Record type: `{ field1 : T1, field2 : T2 }`
    Record(Vec<(String, DhallType)>),
    /// Union type: `< Ctor1 : T1 | Ctor2 | Ctor3 : T3 >`
    Union(Vec<(String, Option<DhallType>)>),
    /// Function type: `T1 -> T2`
    Function(Box<DhallType>, Box<DhallType>),
    /// Dependent function type: `forall (x : T1) -> T2`
    Forall(String, Box<DhallType>, Box<DhallType>),
    /// `Type` universe
    Type,
    /// `Kind` universe (type of types)
    Kind,
    /// `Sort` universe (type of kinds)
    Sort,
    /// Named type reference: `Natural/show`, `MyRecord`
    Named(String),
}
/// Configuration for DhallExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DhallExtPassConfig {
    pub name: String,
    pub phase: DhallExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl DhallExtPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: DhallExtPassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: DhallExtPassPhase) -> Self {
        self.phase = phase;
        self
    }
    #[allow(dead_code)]
    pub fn with_max_iter(mut self, n: usize) -> Self {
        self.max_iterations = n;
        self
    }
    #[allow(dead_code)]
    pub fn with_debug(mut self, d: u32) -> Self {
        self.debug = d;
        self
    }
    #[allow(dead_code)]
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
    #[allow(dead_code)]
    pub fn with_timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = Some(ms);
        self
    }
    #[allow(dead_code)]
    pub fn is_debug_enabled(&self) -> bool {
        self.debug > 0
    }
}
/// Configuration for DhallX2 passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DhallX2PassConfig {
    pub name: String,
    pub phase: DhallX2PassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl DhallX2PassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: DhallX2PassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: DhallX2PassPhase) -> Self {
        self.phase = phase;
        self
    }
    #[allow(dead_code)]
    pub fn with_max_iter(mut self, n: usize) -> Self {
        self.max_iterations = n;
        self
    }
    #[allow(dead_code)]
    pub fn with_debug(mut self, d: u32) -> Self {
        self.debug = d;
        self
    }
    #[allow(dead_code)]
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
    #[allow(dead_code)]
    pub fn with_timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = Some(ms);
        self
    }
    #[allow(dead_code)]
    pub fn is_debug_enabled(&self) -> bool {
        self.debug > 0
    }
}
/// Liveness analysis for DhallExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct DhallExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl DhallExtLiveness {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            live_in: vec![Vec::new(); n],
            live_out: vec![Vec::new(); n],
            defs: vec![Vec::new(); n],
            uses: vec![Vec::new(); n],
        }
    }
    #[allow(dead_code)]
    pub fn live_in(&self, b: usize, v: usize) -> bool {
        self.live_in.get(b).map(|s| s.contains(&v)).unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn live_out(&self, b: usize, v: usize) -> bool {
        self.live_out
            .get(b)
            .map(|s| s.contains(&v))
            .unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn add_def(&mut self, b: usize, v: usize) {
        if let Some(s) = self.defs.get_mut(b) {
            if !s.contains(&v) {
                s.push(v);
            }
        }
    }
    #[allow(dead_code)]
    pub fn add_use(&mut self, b: usize, v: usize) {
        if let Some(s) = self.uses.get_mut(b) {
            if !s.contains(&v) {
                s.push(v);
            }
        }
    }
    #[allow(dead_code)]
    pub fn var_is_used_in_block(&self, b: usize, v: usize) -> bool {
        self.uses.get(b).map(|s| s.contains(&v)).unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn var_is_def_in_block(&self, b: usize, v: usize) -> bool {
        self.defs.get(b).map(|s| s.contains(&v)).unwrap_or(false)
    }
}
/// Pass registry for DhallExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct DhallExtPassRegistry {
    pub(crate) configs: Vec<DhallExtPassConfig>,
    pub(crate) stats: Vec<DhallExtPassStats>,
}
impl DhallExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: DhallExtPassConfig) {
        self.stats.push(DhallExtPassStats::new());
        self.configs.push(c);
    }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.configs.len()
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.configs.is_empty()
    }
    #[allow(dead_code)]
    pub fn get(&self, i: usize) -> Option<&DhallExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&DhallExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&DhallExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &DhallExtPassPhase) -> Vec<&DhallExtPassConfig> {
        self.configs
            .iter()
            .filter(|c| c.enabled && &c.phase == ph)
            .collect()
    }
    #[allow(dead_code)]
    pub fn total_nodes_visited(&self) -> usize {
        self.stats.iter().map(|s| s.nodes_visited).sum()
    }
    #[allow(dead_code)]
    pub fn any_changed(&self) -> bool {
        self.stats.iter().any(|s| s.changed)
    }
}
/// A Dhall function (lambda): `\(label : annotation) -> body`
#[derive(Debug, Clone, PartialEq)]
pub struct DhallFunction {
    /// Parameter label
    pub label: String,
    /// Parameter type annotation
    pub annotation: DhallType,
    /// Function body
    pub body: Box<DhallExpr>,
}
impl DhallFunction {
    /// Create a new function.
    pub fn new(label: impl Into<String>, annotation: DhallType, body: DhallExpr) -> Self {
        DhallFunction {
            label: label.into(),
            annotation,
            body: Box::new(body),
        }
    }
    pub(crate) fn emit(&self, indent: usize) -> String {
        format!(
            r"\({} : {}) -> {}",
            self.label,
            self.annotation,
            self.body.emit(indent)
        )
    }
}
