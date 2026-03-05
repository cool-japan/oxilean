//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::functions::*;
use std::collections::{HashSet, VecDeque};

/// PHP function/method parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct PHPParam {
    /// Parameter name (without `$`)
    pub name: std::string::String,
    /// Optional type hint
    pub ty: Option<PHPType>,
    /// Optional default value
    pub default: Option<PHPExpr>,
    /// Whether this is a reference parameter (`&$name`)
    pub by_ref: bool,
    /// Whether this is a variadic parameter (`...$name`)
    pub variadic: bool,
    /// Whether this is a promoted constructor property (PHP 8.0+)
    pub promoted: Option<PHPVisibility>,
}
impl PHPParam {
    /// Create a simple parameter with a name.
    pub fn simple(name: impl Into<std::string::String>) -> Self {
        PHPParam {
            name: name.into(),
            ty: None,
            default: None,
            by_ref: false,
            variadic: false,
            promoted: None,
        }
    }
    /// Create a typed parameter.
    pub fn typed(name: impl Into<std::string::String>, ty: PHPType) -> Self {
        PHPParam {
            name: name.into(),
            ty: Some(ty),
            default: None,
            by_ref: false,
            variadic: false,
            promoted: None,
        }
    }
    /// Create a parameter with a default value.
    pub fn with_default(
        name: impl Into<std::string::String>,
        ty: Option<PHPType>,
        default: PHPExpr,
    ) -> Self {
        PHPParam {
            name: name.into(),
            ty,
            default: Some(default),
            by_ref: false,
            variadic: false,
            promoted: None,
        }
    }
}
/// A complete PHP script / file.
#[derive(Debug, Clone)]
pub struct PHPScript {
    /// Whether to include `declare(strict_types=1)`
    pub strict_types: bool,
    /// Namespace (optional)
    pub namespace: Option<std::string::String>,
    /// `use` import statements
    pub uses: Vec<(std::string::String, Option<std::string::String>)>,
    /// Top-level functions
    pub functions: Vec<PHPFunction>,
    /// Top-level classes
    pub classes: Vec<PHPClass>,
    /// Interfaces
    pub interfaces: Vec<PHPInterface>,
    /// Traits
    pub traits: Vec<PHPTrait>,
    /// Enums
    pub enums: Vec<PHPEnum>,
    /// Top-level statements (main body)
    pub main: Vec<std::string::String>,
}
impl PHPScript {
    /// Create a new PHP script.
    pub fn new() -> Self {
        PHPScript {
            strict_types: true,
            namespace: None,
            uses: vec![],
            functions: vec![],
            classes: vec![],
            interfaces: vec![],
            traits: vec![],
            enums: vec![],
            main: vec![],
        }
    }
}
/// Statistics for PHPExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct PHPExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl PHPExtPassStats {
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
    pub fn merge(&mut self, o: &PHPExtPassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
/// Worklist for PHPExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PHPExtWorklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
impl PHPExtWorklist {
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
/// PHP trait declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct PHPTrait {
    /// Trait name
    pub name: std::string::String,
    /// Properties
    pub properties: Vec<PHPProperty>,
    /// Methods
    pub methods: Vec<PHPFunction>,
}
impl PHPTrait {
    /// Create a new trait.
    pub fn new(name: impl Into<std::string::String>) -> Self {
        PHPTrait {
            name: name.into(),
            properties: vec![],
            methods: vec![],
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PHPAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, PHPCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl PHPAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        PHPAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&PHPCacheEntry> {
        if self.entries.contains_key(key) {
            self.hits += 1;
            self.entries.get(key)
        } else {
            self.misses += 1;
            None
        }
    }
    #[allow(dead_code)]
    pub fn insert(&mut self, key: String, data: Vec<u8>) {
        if self.entries.len() >= self.max_size {
            if let Some(oldest) = self.entries.keys().next().cloned() {
                self.entries.remove(&oldest);
            }
        }
        self.entries.insert(
            key.clone(),
            PHPCacheEntry {
                key,
                data,
                timestamp: 0,
                valid: true,
            },
        );
    }
    #[allow(dead_code)]
    pub fn invalidate(&mut self, key: &str) {
        if let Some(entry) = self.entries.get_mut(key) {
            entry.valid = false;
        }
    }
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    #[allow(dead_code)]
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            return 0.0;
        }
        self.hits as f64 / total as f64
    }
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.entries.len()
    }
}
/// PHP class declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct PHPClass {
    /// Class name
    pub name: std::string::String,
    /// Optional parent class
    pub parent: Option<std::string::String>,
    /// Implemented interfaces
    pub interfaces: Vec<std::string::String>,
    /// Used traits
    pub traits: Vec<std::string::String>,
    /// Whether this is abstract
    pub is_abstract: bool,
    /// Whether this is final
    pub is_final: bool,
    /// Whether this is readonly (PHP 8.2+)
    pub is_readonly: bool,
    /// Properties
    pub properties: Vec<PHPProperty>,
    /// Methods
    pub methods: Vec<PHPFunction>,
    /// Class constants
    pub constants: Vec<(std::string::String, PHPType, std::string::String)>,
}
impl PHPClass {
    /// Create a new empty class.
    pub fn new(name: impl Into<std::string::String>) -> Self {
        PHPClass {
            name: name.into(),
            parent: None,
            interfaces: vec![],
            traits: vec![],
            is_abstract: false,
            is_final: false,
            is_readonly: false,
            properties: vec![],
            methods: vec![],
            constants: vec![],
        }
    }
    /// Create an abstract class.
    pub fn abstract_class(name: impl Into<std::string::String>) -> Self {
        let mut cls = PHPClass::new(name);
        cls.is_abstract = true;
        cls
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PHPWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl PHPWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        PHPWorklist {
            items: std::collections::VecDeque::new(),
            in_worklist: std::collections::HashSet::new(),
        }
    }
    #[allow(dead_code)]
    pub fn push(&mut self, item: u32) -> bool {
        if self.in_worklist.insert(item) {
            self.items.push_back(item);
            true
        } else {
            false
        }
    }
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<u32> {
        let item = self.items.pop_front()?;
        self.in_worklist.remove(&item);
        Some(item)
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
    pub fn contains(&self, item: u32) -> bool {
        self.in_worklist.contains(&item)
    }
}
#[allow(dead_code)]
pub struct PHPPassRegistry {
    pub(super) configs: Vec<PHPPassConfig>,
    pub(super) stats: std::collections::HashMap<String, PHPPassStats>,
}
impl PHPPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        PHPPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: PHPPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), PHPPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&PHPPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&PHPPassStats> {
        self.stats.get(name)
    }
    #[allow(dead_code)]
    pub fn total_passes(&self) -> usize {
        self.configs.len()
    }
    #[allow(dead_code)]
    pub fn enabled_count(&self) -> usize {
        self.enabled_passes().len()
    }
    #[allow(dead_code)]
    pub fn update_stats(&mut self, name: &str, changes: u64, time_ms: u64, iter: u32) {
        if let Some(stats) = self.stats.get_mut(name) {
            stats.record_run(changes, time_ms, iter);
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PHPLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl PHPLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        PHPLivenessInfo {
            live_in: vec![std::collections::HashSet::new(); block_count],
            live_out: vec![std::collections::HashSet::new(); block_count],
            defs: vec![std::collections::HashSet::new(); block_count],
            uses: vec![std::collections::HashSet::new(); block_count],
        }
    }
    #[allow(dead_code)]
    pub fn add_def(&mut self, block: usize, var: u32) {
        if block < self.defs.len() {
            self.defs[block].insert(var);
        }
    }
    #[allow(dead_code)]
    pub fn add_use(&mut self, block: usize, var: u32) {
        if block < self.uses.len() {
            self.uses[block].insert(var);
        }
    }
    #[allow(dead_code)]
    pub fn is_live_in(&self, block: usize, var: u32) -> bool {
        self.live_in
            .get(block)
            .map(|s| s.contains(&var))
            .unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn is_live_out(&self, block: usize, var: u32) -> bool {
        self.live_out
            .get(block)
            .map(|s| s.contains(&var))
            .unwrap_or(false)
    }
}
/// Pass registry for PHPExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct PHPExtPassRegistry {
    pub(super) configs: Vec<PHPExtPassConfig>,
    pub(super) stats: Vec<PHPExtPassStats>,
}
impl PHPExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: PHPExtPassConfig) {
        self.stats.push(PHPExtPassStats::new());
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
    pub fn get(&self, i: usize) -> Option<&PHPExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&PHPExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&PHPExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &PHPExtPassPhase) -> Vec<&PHPExtPassConfig> {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PHPDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl PHPDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        PHPDominatorTree {
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
/// Liveness analysis for PHPExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct PHPExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl PHPExtLiveness {
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
/// Constant folding helper for PHPExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct PHPExtConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
impl PHPExtConstFolder {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            folds: 0,
            failures: 0,
            enabled: true,
        }
    }
    #[allow(dead_code)]
    pub fn add_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_add(b)
    }
    #[allow(dead_code)]
    pub fn sub_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_sub(b)
    }
    #[allow(dead_code)]
    pub fn mul_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_mul(b)
    }
    #[allow(dead_code)]
    pub fn div_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_div(b)
        }
    }
    #[allow(dead_code)]
    pub fn rem_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_rem(b)
        }
    }
    #[allow(dead_code)]
    pub fn neg_i64(&mut self, a: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_neg()
    }
    #[allow(dead_code)]
    pub fn shl_i64(&mut self, a: i64, s: u32) -> Option<i64> {
        if s >= 64 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_shl(s)
        }
    }
    #[allow(dead_code)]
    pub fn shr_i64(&mut self, a: i64, s: u32) -> Option<i64> {
        if s >= 64 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_shr(s)
        }
    }
    #[allow(dead_code)]
    pub fn and_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a & b
    }
    #[allow(dead_code)]
    pub fn or_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a | b
    }
    #[allow(dead_code)]
    pub fn xor_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a ^ b
    }
    #[allow(dead_code)]
    pub fn not_i64(&mut self, a: i64) -> i64 {
        self.folds += 1;
        !a
    }
    #[allow(dead_code)]
    pub fn fold_count(&self) -> usize {
        self.folds
    }
    #[allow(dead_code)]
    pub fn failure_count(&self) -> usize {
        self.failures
    }
    #[allow(dead_code)]
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    #[allow(dead_code)]
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    #[allow(dead_code)]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
/// A PHP top-level function or method.
#[derive(Debug, Clone, PartialEq)]
pub struct PHPFunction {
    /// Function name
    pub name: std::string::String,
    /// Parameters
    pub params: Vec<PHPParam>,
    /// Optional return type
    pub return_type: Option<PHPType>,
    /// Body lines (raw PHP code)
    pub body: Vec<std::string::String>,
    /// Whether this is a static method
    pub is_static: bool,
    /// Whether this is abstract
    pub is_abstract: bool,
    /// Visibility (for methods)
    pub visibility: Option<PHPVisibility>,
    /// Docblock comment
    pub doc_comment: Option<std::string::String>,
}
impl PHPFunction {
    /// Create a simple function with a name and body.
    pub fn new(name: impl Into<std::string::String>, body: Vec<std::string::String>) -> Self {
        PHPFunction {
            name: name.into(),
            params: vec![],
            return_type: None,
            body,
            is_static: false,
            is_abstract: false,
            visibility: None,
            doc_comment: None,
        }
    }
    /// Create a method with visibility.
    pub fn method(
        name: impl Into<std::string::String>,
        visibility: PHPVisibility,
        params: Vec<PHPParam>,
        return_type: Option<PHPType>,
        body: Vec<std::string::String>,
    ) -> Self {
        PHPFunction {
            name: name.into(),
            params,
            return_type,
            body,
            is_static: false,
            is_abstract: false,
            visibility: Some(visibility),
            doc_comment: None,
        }
    }
}
/// PHP member visibility modifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PHPVisibility {
    Public,
    Protected,
    Private,
}
/// PHP 8.1 backed enum variant.
#[derive(Debug, Clone, PartialEq)]
pub struct PHPEnumCase {
    /// Variant name
    pub name: std::string::String,
    /// Backing value for backed enums
    pub value: Option<std::string::String>,
}
/// PHP type for type hints and declarations.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PHPType {
    /// `int`
    Int,
    /// `float`
    Float,
    /// `string`
    String,
    /// `bool`
    Bool,
    /// `array`
    Array,
    /// `null`
    Null,
    /// `mixed`
    Mixed,
    /// `callable`
    Callable,
    /// `void`
    Void,
    /// `never`
    Never,
    /// `object`
    Object,
    /// `iterable`
    Iterable,
    /// `?T` (nullable type)
    Nullable(Box<PHPType>),
    /// `T1|T2|...` (union type)
    Union(Vec<PHPType>),
    /// `T1&T2&...` (intersection type, PHP 8.1+)
    Intersection(Vec<PHPType>),
    /// Named class/interface type
    Named(std::string::String),
    /// `self`
    Self_,
    /// `static`
    Static,
    /// `parent`
    Parent,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PHPCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// Pass execution phase for PHPExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PHPExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl PHPExtPassPhase {
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
#[derive(Debug, Clone, PartialEq)]
pub enum PHPPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl PHPPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            PHPPassPhase::Analysis => "analysis",
            PHPPassPhase::Transformation => "transformation",
            PHPPassPhase::Verification => "verification",
            PHPPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, PHPPassPhase::Transformation | PHPPassPhase::Cleanup)
    }
}
/// Configuration for PHPExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PHPExtPassConfig {
    pub name: String,
    pub phase: PHPExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl PHPExtPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: PHPExtPassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: PHPExtPassPhase) -> Self {
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
/// Analysis cache for PHPExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct PHPExtCache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
impl PHPExtCache {
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
/// PHP 8.x code generation backend for OxiLean.
pub struct PHPBackend {
    /// Indent string (default: 4 spaces)
    pub(super) indent: std::string::String,
    /// Name mangling table
    pub(super) mangle_cache: HashMap<std::string::String, std::string::String>,
    /// Whether to emit docblocks
    pub(super) emit_docs: bool,
}
impl PHPBackend {
    /// Create a new PHPBackend with default settings.
    pub fn new() -> Self {
        PHPBackend {
            indent: "    ".to_string(),
            mangle_cache: HashMap::new(),
            emit_docs: true,
        }
    }
    /// Create a PHPBackend with a custom indent string.
    pub fn with_indent(indent: impl Into<std::string::String>) -> Self {
        PHPBackend {
            indent: indent.into(),
            mangle_cache: HashMap::new(),
            emit_docs: true,
        }
    }
    /// Emit a PHP type hint as a string.
    pub fn emit_type(&self, ty: &PHPType) -> std::string::String {
        format!("{}", ty)
    }
    /// Emit a PHP expression as a string.
    pub fn emit_expr(&self, expr: &PHPExpr) -> std::string::String {
        format!("{}", expr)
    }
    /// Mangle an OxiLean name to a valid PHP identifier.
    ///
    /// PHP identifiers must match `[a-zA-Z_\x7f-\xff][a-zA-Z0-9_\x7f-\xff]*`.
    pub fn mangle_name(&self, name: &str) -> std::string::String {
        if let Some(cached) = self.mangle_cache.get(name) {
            return cached.clone();
        }
        let mut result = std::string::String::new();
        let mut first = true;
        for c in name.chars() {
            if first {
                if c.is_alphabetic() || c == '_' {
                    result.push(c);
                } else {
                    result.push('_');
                    if c.is_alphanumeric() {
                        result.push(c);
                    }
                }
                first = false;
            } else {
                match c {
                    'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => result.push(c),
                    '.' | ':' | '\'' => result.push('_'),
                    '-' => result.push('_'),
                    _ => {
                        let code = c as u32;
                        result.push_str(&format!("_u{:04X}_", code));
                    }
                }
            }
        }
        let reserved = [
            "abstract",
            "and",
            "array",
            "as",
            "break",
            "callable",
            "case",
            "catch",
            "class",
            "clone",
            "const",
            "continue",
            "declare",
            "default",
            "die",
            "do",
            "echo",
            "else",
            "elseif",
            "empty",
            "enddeclare",
            "endfor",
            "endforeach",
            "endif",
            "endswitch",
            "endwhile",
            "enum",
            "eval",
            "exit",
            "extends",
            "final",
            "finally",
            "fn",
            "for",
            "foreach",
            "function",
            "global",
            "goto",
            "if",
            "implements",
            "include",
            "include_once",
            "instanceof",
            "insteadof",
            "interface",
            "isset",
            "list",
            "match",
            "namespace",
            "new",
            "null",
            "or",
            "print",
            "private",
            "protected",
            "public",
            "readonly",
            "require",
            "require_once",
            "return",
            "static",
            "switch",
            "throw",
            "trait",
            "try",
            "unset",
            "use",
            "var",
            "while",
            "xor",
            "yield",
        ];
        if reserved.contains(&result.as_str()) {
            result.push_str("_ox");
        }
        result
    }
    /// Emit a parameter declaration.
    pub(super) fn emit_param(&self, param: &PHPParam) -> std::string::String {
        format_param(param)
    }
    /// Emit a PHP function (top-level or standalone).
    pub fn emit_function(&self, func: &PHPFunction) -> std::string::String {
        let mut out = std::string::String::new();
        if self.emit_docs {
            if let Some(doc) = &func.doc_comment {
                out.push_str("/**\n");
                for line in doc.lines() {
                    out.push_str(&format!(" * {}\n", line));
                }
                out.push_str(" */\n");
            }
        }
        if let Some(vis) = &func.visibility {
            out.push_str(&format!("{} ", vis));
        }
        if func.is_static {
            out.push_str("static ");
        }
        if func.is_abstract {
            out.push_str("abstract ");
        }
        let params_s: Vec<std::string::String> =
            func.params.iter().map(|p| self.emit_param(p)).collect();
        out.push_str(&format!("function {}({})", func.name, params_s.join(", ")));
        if let Some(ret) = &func.return_type {
            out.push_str(&format!(": {}", ret));
        }
        if func.is_abstract {
            out.push_str(";\n");
        } else {
            out.push_str("\n{\n");
            for line in &func.body {
                out.push_str(&format!("{}{}\n", self.indent, line));
            }
            out.push_str("}\n");
        }
        out
    }
    /// Emit a PHP property declaration.
    pub(super) fn emit_property(&self, prop: &PHPProperty) -> std::string::String {
        let mut s = format!("{} ", prop.visibility);
        if prop.is_static {
            s.push_str("static ");
        }
        if prop.readonly {
            s.push_str("readonly ");
        }
        if let Some(ty) = &prop.ty {
            s.push_str(&format!("{} ", ty));
        }
        s.push_str(&format!("${}", prop.name));
        if let Some(default) = &prop.default {
            s.push_str(&format!(" = {}", default));
        }
        s.push(';');
        s
    }
    /// Emit a PHP interface declaration.
    pub fn emit_interface(&self, iface: &PHPInterface) -> std::string::String {
        let mut out = std::string::String::new();
        out.push_str(&format!("interface {}", iface.name));
        if !iface.extends.is_empty() {
            out.push_str(&format!(" extends {}", iface.extends.join(", ")));
        }
        out.push_str("\n{\n");
        for (name, val) in &iface.constants {
            out.push_str(&format!("{}const {} = {};\n", self.indent, name, val));
        }
        for method in &iface.methods {
            out.push_str(&self.indent_block(&self.emit_function(method)));
        }
        out.push_str("}\n");
        out
    }
    /// Emit a PHP trait declaration.
    pub fn emit_trait(&self, tr: &PHPTrait) -> std::string::String {
        let mut out = std::string::String::new();
        out.push_str(&format!("trait {}\n{{\n", tr.name));
        for prop in &tr.properties {
            out.push_str(&format!("{}{}\n", self.indent, self.emit_property(prop)));
        }
        for method in &tr.methods {
            out.push_str(&self.indent_block(&self.emit_function(method)));
        }
        out.push_str("}\n");
        out
    }
    /// Emit a PHP 8.1 enum declaration.
    pub fn emit_enum(&self, en: &PHPEnum) -> std::string::String {
        let mut out = std::string::String::new();
        out.push_str(&format!("enum {}", en.name));
        if let Some(bt) = &en.backing_type {
            out.push_str(&format!(": {}", bt));
        }
        if !en.implements.is_empty() {
            out.push_str(&format!(" implements {}", en.implements.join(", ")));
        }
        out.push_str("\n{\n");
        for case in &en.cases {
            if let Some(val) = &case.value {
                out.push_str(&format!("{}case {} = {};\n", self.indent, case.name, val));
            } else {
                out.push_str(&format!("{}case {};\n", self.indent, case.name));
            }
        }
        for method in &en.methods {
            out.push_str(&self.indent_block(&self.emit_function(method)));
        }
        out.push_str("}\n");
        out
    }
    /// Emit a PHP class declaration.
    pub fn emit_class(&self, class: &PHPClass) -> std::string::String {
        let mut out = std::string::String::new();
        if class.is_abstract {
            out.push_str("abstract ");
        }
        if class.is_final {
            out.push_str("final ");
        }
        if class.is_readonly {
            out.push_str("readonly ");
        }
        out.push_str(&format!("class {}", class.name));
        if let Some(parent) = &class.parent {
            out.push_str(&format!(" extends {}", parent));
        }
        if !class.interfaces.is_empty() {
            out.push_str(&format!(" implements {}", class.interfaces.join(", ")));
        }
        out.push_str("\n{\n");
        for tr in &class.traits {
            out.push_str(&format!("{}use {};\n", self.indent, tr));
        }
        if !class.traits.is_empty() {
            out.push('\n');
        }
        for (name, ty, val) in &class.constants {
            out.push_str(&format!(
                "{}const {}: {} = {};\n",
                self.indent, name, ty, val
            ));
        }
        for prop in &class.properties {
            out.push_str(&format!("{}{}\n", self.indent, self.emit_property(prop)));
        }
        if !class.properties.is_empty() {
            out.push('\n');
        }
        for method in &class.methods {
            out.push_str(&self.indent_block(&self.emit_function(method)));
            out.push('\n');
        }
        out.push_str("}\n");
        out
    }
    /// Emit a complete PHP script.
    pub fn emit_script(&self, script: &PHPScript) -> std::string::String {
        let mut out = std::string::String::from("<?php\n");
        if script.strict_types {
            out.push_str("declare(strict_types=1);\n\n");
        }
        if let Some(ns) = &script.namespace {
            out.push_str(&format!("namespace {};\n\n", ns));
        }
        for (path, alias) in &script.uses {
            match alias {
                Some(a) => out.push_str(&format!("use {} as {};\n", path, a)),
                None => out.push_str(&format!("use {};\n", path)),
            }
        }
        if !script.uses.is_empty() {
            out.push('\n');
        }
        for iface in &script.interfaces {
            out.push_str(&self.emit_interface(iface));
            out.push('\n');
        }
        for tr in &script.traits {
            out.push_str(&self.emit_trait(tr));
            out.push('\n');
        }
        for en in &script.enums {
            out.push_str(&self.emit_enum(en));
            out.push('\n');
        }
        for class in &script.classes {
            out.push_str(&self.emit_class(class));
            out.push('\n');
        }
        for func in &script.functions {
            out.push_str(&self.emit_function(func));
            out.push('\n');
        }
        for line in &script.main {
            out.push_str(line);
            out.push('\n');
        }
        out
    }
    /// Indent each line of a block by one level.
    pub(super) fn indent_block(&self, block: &str) -> std::string::String {
        block
            .lines()
            .map(|line| {
                if line.trim().is_empty() {
                    std::string::String::new()
                } else {
                    format!("{}{}", self.indent, line)
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
            + "\n"
    }
    /// Emit a namespace block.
    pub fn emit_namespace(&self, ns: &PHPNamespace) -> std::string::String {
        let mut script = PHPScript::new();
        script.namespace = Some(ns.path.clone());
        script.uses = ns.uses.clone();
        script.functions = ns.functions.clone();
        script.classes = ns.classes.clone();
        script.interfaces = ns.interfaces.clone();
        script.traits = ns.traits.clone();
        script.enums = ns.enums.clone();
        self.emit_script(&script)
    }
}
/// Dependency graph for PHPExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PHPExtDepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
impl PHPExtDepGraph {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            n,
            adj: vec![Vec::new(); n],
            rev: vec![Vec::new(); n],
            edge_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.n && to < self.n {
            if !self.adj[from].contains(&to) {
                self.adj[from].push(to);
                self.rev[to].push(from);
                self.edge_count += 1;
            }
        }
    }
    #[allow(dead_code)]
    pub fn succs(&self, n: usize) -> &[usize] {
        self.adj.get(n).map(|v| v.as_slice()).unwrap_or(&[])
    }
    #[allow(dead_code)]
    pub fn preds(&self, n: usize) -> &[usize] {
        self.rev.get(n).map(|v| v.as_slice()).unwrap_or(&[])
    }
    #[allow(dead_code)]
    pub fn topo_sort(&self) -> Option<Vec<usize>> {
        let mut deg: Vec<usize> = (0..self.n).map(|i| self.rev[i].len()).collect();
        let mut q: std::collections::VecDeque<usize> =
            (0..self.n).filter(|&i| deg[i] == 0).collect();
        let mut out = Vec::with_capacity(self.n);
        while let Some(u) = q.pop_front() {
            out.push(u);
            for &v in &self.adj[u] {
                deg[v] -= 1;
                if deg[v] == 0 {
                    q.push_back(v);
                }
            }
        }
        if out.len() == self.n {
            Some(out)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn has_cycle(&self) -> bool {
        self.topo_sort().is_none()
    }
    #[allow(dead_code)]
    pub fn reachable(&self, start: usize) -> Vec<usize> {
        let mut vis = vec![false; self.n];
        let mut stk = vec![start];
        let mut out = Vec::new();
        while let Some(u) = stk.pop() {
            if u < self.n && !vis[u] {
                vis[u] = true;
                out.push(u);
                for &v in &self.adj[u] {
                    if !vis[v] {
                        stk.push(v);
                    }
                }
            }
        }
        out
    }
    #[allow(dead_code)]
    pub fn scc(&self) -> Vec<Vec<usize>> {
        let mut visited = vec![false; self.n];
        let mut order = Vec::new();
        for i in 0..self.n {
            if !visited[i] {
                let mut stk = vec![(i, 0usize)];
                while let Some((u, idx)) = stk.last_mut() {
                    if !visited[*u] {
                        visited[*u] = true;
                    }
                    if *idx < self.adj[*u].len() {
                        let v = self.adj[*u][*idx];
                        *idx += 1;
                        if !visited[v] {
                            stk.push((v, 0));
                        }
                    } else {
                        order.push(*u);
                        stk.pop();
                    }
                }
            }
        }
        let mut comp = vec![usize::MAX; self.n];
        let mut components: Vec<Vec<usize>> = Vec::new();
        for &start in order.iter().rev() {
            if comp[start] == usize::MAX {
                let cid = components.len();
                let mut component = Vec::new();
                let mut stk = vec![start];
                while let Some(u) = stk.pop() {
                    if comp[u] == usize::MAX {
                        comp[u] = cid;
                        component.push(u);
                        for &v in &self.rev[u] {
                            if comp[v] == usize::MAX {
                                stk.push(v);
                            }
                        }
                    }
                }
                components.push(component);
            }
        }
        components
    }
    #[allow(dead_code)]
    pub fn node_count(&self) -> usize {
        self.n
    }
    #[allow(dead_code)]
    pub fn edge_count(&self) -> usize {
        self.edge_count
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct PHPPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl PHPPassStats {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn record_run(&mut self, changes: u64, time_ms: u64, iterations: u32) {
        self.total_runs += 1;
        self.successful_runs += 1;
        self.total_changes += changes;
        self.time_ms += time_ms;
        self.iterations_used = iterations;
    }
    #[allow(dead_code)]
    pub fn average_changes_per_run(&self) -> f64 {
        if self.total_runs == 0 {
            return 0.0;
        }
        self.total_changes as f64 / self.total_runs as f64
    }
    #[allow(dead_code)]
    pub fn success_rate(&self) -> f64 {
        if self.total_runs == 0 {
            return 0.0;
        }
        self.successful_runs as f64 / self.total_runs as f64
    }
    #[allow(dead_code)]
    pub fn format_summary(&self) -> String {
        format!(
            "Runs: {}/{}, Changes: {}, Time: {}ms",
            self.successful_runs, self.total_runs, self.total_changes, self.time_ms
        )
    }
}
/// Dominator tree for PHPExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PHPExtDomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
impl PHPExtDomTree {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            idom: vec![None; n],
            children: vec![Vec::new(); n],
            depth: vec![0; n],
        }
    }
    #[allow(dead_code)]
    pub fn set_idom(&mut self, node: usize, dom: usize) {
        if node < self.idom.len() {
            self.idom[node] = Some(dom);
            if dom < self.children.len() {
                self.children[dom].push(node);
            }
            self.depth[node] = if dom < self.depth.len() {
                self.depth[dom] + 1
            } else {
                1
            };
        }
    }
    #[allow(dead_code)]
    pub fn dominates(&self, a: usize, mut b: usize) -> bool {
        if a == b {
            return true;
        }
        let n = self.idom.len();
        for _ in 0..n {
            match self.idom.get(b).copied().flatten() {
                None => return false,
                Some(p) if p == a => return true,
                Some(p) if p == b => return false,
                Some(p) => b = p,
            }
        }
        false
    }
    #[allow(dead_code)]
    pub fn children_of(&self, n: usize) -> &[usize] {
        self.children.get(n).map(|v| v.as_slice()).unwrap_or(&[])
    }
    #[allow(dead_code)]
    pub fn depth_of(&self, n: usize) -> usize {
        self.depth.get(n).copied().unwrap_or(0)
    }
    #[allow(dead_code)]
    pub fn lca(&self, mut a: usize, mut b: usize) -> usize {
        let n = self.idom.len();
        for _ in 0..(2 * n) {
            if a == b {
                return a;
            }
            if self.depth_of(a) > self.depth_of(b) {
                a = self.idom.get(a).and_then(|x| *x).unwrap_or(a);
            } else {
                b = self.idom.get(b).and_then(|x| *x).unwrap_or(b);
            }
        }
        0
    }
}
/// PHP 8.1 enum declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct PHPEnum {
    /// Enum name
    pub name: std::string::String,
    /// Backing type (int or string), `None` for pure enums
    pub backing_type: Option<PHPType>,
    /// Cases
    pub cases: Vec<PHPEnumCase>,
    /// Implemented interfaces
    pub implements: Vec<std::string::String>,
    /// Methods
    pub methods: Vec<PHPFunction>,
}
impl PHPEnum {
    /// Create a new pure enum.
    pub fn new(name: impl Into<std::string::String>) -> Self {
        PHPEnum {
            name: name.into(),
            backing_type: None,
            cases: vec![],
            implements: vec![],
            methods: vec![],
        }
    }
    /// Create a string-backed enum.
    pub fn string_backed(name: impl Into<std::string::String>) -> Self {
        PHPEnum {
            name: name.into(),
            backing_type: Some(PHPType::String),
            cases: vec![],
            implements: vec![],
            methods: vec![],
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PHPPassConfig {
    pub phase: PHPPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl PHPPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: PHPPassPhase) -> Self {
        PHPPassConfig {
            phase,
            enabled: true,
            max_iterations: 10,
            debug_output: false,
            pass_name: name.into(),
        }
    }
    #[allow(dead_code)]
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
    #[allow(dead_code)]
    pub fn with_debug(mut self) -> Self {
        self.debug_output = true;
        self
    }
    #[allow(dead_code)]
    pub fn max_iter(mut self, n: u32) -> Self {
        self.max_iterations = n;
        self
    }
}
#[allow(dead_code)]
pub struct PHPConstantFoldingHelper;
impl PHPConstantFoldingHelper {
    #[allow(dead_code)]
    pub fn fold_add_i64(a: i64, b: i64) -> Option<i64> {
        a.checked_add(b)
    }
    #[allow(dead_code)]
    pub fn fold_sub_i64(a: i64, b: i64) -> Option<i64> {
        a.checked_sub(b)
    }
    #[allow(dead_code)]
    pub fn fold_mul_i64(a: i64, b: i64) -> Option<i64> {
        a.checked_mul(b)
    }
    #[allow(dead_code)]
    pub fn fold_div_i64(a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            None
        } else {
            a.checked_div(b)
        }
    }
    #[allow(dead_code)]
    pub fn fold_add_f64(a: f64, b: f64) -> f64 {
        a + b
    }
    #[allow(dead_code)]
    pub fn fold_mul_f64(a: f64, b: f64) -> f64 {
        a * b
    }
    #[allow(dead_code)]
    pub fn fold_neg_i64(a: i64) -> Option<i64> {
        a.checked_neg()
    }
    #[allow(dead_code)]
    pub fn fold_not_bool(a: bool) -> bool {
        !a
    }
    #[allow(dead_code)]
    pub fn fold_and_bool(a: bool, b: bool) -> bool {
        a && b
    }
    #[allow(dead_code)]
    pub fn fold_or_bool(a: bool, b: bool) -> bool {
        a || b
    }
    #[allow(dead_code)]
    pub fn fold_shl_i64(a: i64, b: u32) -> Option<i64> {
        a.checked_shl(b)
    }
    #[allow(dead_code)]
    pub fn fold_shr_i64(a: i64, b: u32) -> Option<i64> {
        a.checked_shr(b)
    }
    #[allow(dead_code)]
    pub fn fold_rem_i64(a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            None
        } else {
            Some(a % b)
        }
    }
    #[allow(dead_code)]
    pub fn fold_bitand_i64(a: i64, b: i64) -> i64 {
        a & b
    }
    #[allow(dead_code)]
    pub fn fold_bitor_i64(a: i64, b: i64) -> i64 {
        a | b
    }
    #[allow(dead_code)]
    pub fn fold_bitxor_i64(a: i64, b: i64) -> i64 {
        a ^ b
    }
    #[allow(dead_code)]
    pub fn fold_bitnot_i64(a: i64) -> i64 {
        !a
    }
}
/// PHP expression AST node.
#[derive(Debug, Clone, PartialEq)]
pub enum PHPExpr {
    /// Literal value (int, float, string, bool, null)
    Lit(std::string::String),
    /// Variable reference: `$name`
    Var(std::string::String),
    /// Binary operation: `lhs op rhs`
    BinOp(Box<PHPExpr>, std::string::String, Box<PHPExpr>),
    /// Unary operation: `op operand`
    UnaryOp(std::string::String, Box<PHPExpr>),
    /// Function/method call: `name(args...)`
    FuncCall(std::string::String, Vec<PHPExpr>),
    /// Array literal: `[expr, ...]`
    ArrayLit(Vec<PHPExpr>),
    /// Associative array literal: `[key => val, ...]`
    ArrayMap(Vec<(PHPExpr, PHPExpr)>),
    /// Object instantiation: `new ClassName(args...)`
    New(std::string::String, Vec<PHPExpr>),
    /// Property access: `$obj->prop`
    Arrow(Box<PHPExpr>, std::string::String),
    /// Null-safe property access: `$obj?->prop`
    NullSafe(Box<PHPExpr>, std::string::String),
    /// Static property/method access: `Class::member`
    StaticAccess(std::string::String, std::string::String),
    /// Array index: `$arr[idx]`
    Index(Box<PHPExpr>, Box<PHPExpr>),
    /// Ternary: `$cond ? $then : $else`
    Ternary(Box<PHPExpr>, Box<PHPExpr>, Box<PHPExpr>),
    /// Null coalescing: `$a ?? $b`
    NullCoalesce(Box<PHPExpr>, Box<PHPExpr>),
    /// Closure / anonymous function
    Closure {
        params: Vec<PHPParam>,
        use_vars: Vec<std::string::String>,
        return_type: Option<PHPType>,
        body: Vec<std::string::String>,
    },
    /// Arrow function (PHP 7.4+): `fn($x) => expr`
    ArrowFn {
        params: Vec<PHPParam>,
        return_type: Option<PHPType>,
        body: Box<PHPExpr>,
    },
    /// Match expression (PHP 8.0+)
    Match {
        subject: Box<PHPExpr>,
        arms: Vec<(PHPExpr, PHPExpr)>,
        default: Option<Box<PHPExpr>>,
    },
    /// Named argument: `name: value`
    NamedArg(std::string::String, Box<PHPExpr>),
    /// Spread operator: `...$arr`
    Spread(Box<PHPExpr>),
    /// Cast: `(type)$expr`
    Cast(std::string::String, Box<PHPExpr>),
    /// `isset($var)`
    Isset(Box<PHPExpr>),
    /// `empty($var)`
    Empty(Box<PHPExpr>),
}
/// A PHP class property.
#[derive(Debug, Clone, PartialEq)]
pub struct PHPProperty {
    /// Property name (without `$`)
    pub name: std::string::String,
    /// Optional type hint
    pub ty: Option<PHPType>,
    /// Visibility
    pub visibility: PHPVisibility,
    /// Whether this is static
    pub is_static: bool,
    /// Whether this is readonly (PHP 8.1+)
    pub readonly: bool,
    /// Optional default value expression (as string)
    pub default: Option<std::string::String>,
}
impl PHPProperty {
    /// Create a public property.
    pub fn public(name: impl Into<std::string::String>, ty: Option<PHPType>) -> Self {
        PHPProperty {
            name: name.into(),
            ty,
            visibility: PHPVisibility::Public,
            is_static: false,
            readonly: false,
            default: None,
        }
    }
    /// Create a private property.
    pub fn private(name: impl Into<std::string::String>, ty: Option<PHPType>) -> Self {
        PHPProperty {
            name: name.into(),
            ty,
            visibility: PHPVisibility::Private,
            is_static: false,
            readonly: false,
            default: None,
        }
    }
}
/// A PHP namespace block.
#[derive(Debug, Clone, PartialEq)]
pub struct PHPNamespace {
    /// Namespace path (e.g. `OxiLean\\Runtime`)
    pub path: std::string::String,
    /// `use` import statements
    pub uses: Vec<(std::string::String, Option<std::string::String>)>,
    /// Functions in this namespace
    pub functions: Vec<PHPFunction>,
    /// Classes in this namespace
    pub classes: Vec<PHPClass>,
    /// Interfaces in this namespace
    pub interfaces: Vec<PHPInterface>,
    /// Traits in this namespace
    pub traits: Vec<PHPTrait>,
    /// Enums in this namespace
    pub enums: Vec<PHPEnum>,
}
impl PHPNamespace {
    /// Create a new namespace.
    pub fn new(path: impl Into<std::string::String>) -> Self {
        PHPNamespace {
            path: path.into(),
            uses: vec![],
            functions: vec![],
            classes: vec![],
            interfaces: vec![],
            traits: vec![],
            enums: vec![],
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PHPDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl PHPDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        PHPDepGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add_node(&mut self, id: u32) {
        if !self.nodes.contains(&id) {
            self.nodes.push(id);
        }
    }
    #[allow(dead_code)]
    pub fn add_dep(&mut self, dep: u32, dependent: u32) {
        self.add_node(dep);
        self.add_node(dependent);
        self.edges.push((dep, dependent));
    }
    #[allow(dead_code)]
    pub fn dependents_of(&self, node: u32) -> Vec<u32> {
        self.edges
            .iter()
            .filter(|(d, _)| *d == node)
            .map(|(_, dep)| *dep)
            .collect()
    }
    #[allow(dead_code)]
    pub fn dependencies_of(&self, node: u32) -> Vec<u32> {
        self.edges
            .iter()
            .filter(|(_, dep)| *dep == node)
            .map(|(d, _)| *d)
            .collect()
    }
    #[allow(dead_code)]
    pub fn topological_sort(&self) -> Vec<u32> {
        let mut in_degree: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();
        for &n in &self.nodes {
            in_degree.insert(n, 0);
        }
        for (_, dep) in &self.edges {
            *in_degree.entry(*dep).or_insert(0) += 1;
        }
        let mut queue: std::collections::VecDeque<u32> = self
            .nodes
            .iter()
            .filter(|&&n| in_degree[&n] == 0)
            .copied()
            .collect();
        let mut result = Vec::new();
        while let Some(node) = queue.pop_front() {
            result.push(node);
            for dep in self.dependents_of(node) {
                let cnt = in_degree.entry(dep).or_insert(0);
                *cnt = cnt.saturating_sub(1);
                if *cnt == 0 {
                    queue.push_back(dep);
                }
            }
        }
        result
    }
    #[allow(dead_code)]
    pub fn has_cycle(&self) -> bool {
        self.topological_sort().len() < self.nodes.len()
    }
}
/// PHP interface declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct PHPInterface {
    /// Interface name
    pub name: std::string::String,
    /// Interfaces this extends
    pub extends: Vec<std::string::String>,
    /// Method signatures (abstract methods)
    pub methods: Vec<PHPFunction>,
    /// Constants
    pub constants: Vec<(std::string::String, std::string::String)>,
}
impl PHPInterface {
    /// Create a new interface.
    pub fn new(name: impl Into<std::string::String>) -> Self {
        PHPInterface {
            name: name.into(),
            extends: vec![],
            methods: vec![],
            constants: vec![],
        }
    }
}
