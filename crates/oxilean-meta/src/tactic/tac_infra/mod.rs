//! TacMod extended utility infrastructure: TacModExt, TacModExtUtil, TacModExtMap,
//! TacModWindow, TacModBuilder, TacModStateMachine, TacModWorkQueue, TacModCounterMap.

use std::collections::{HashMap, VecDeque};

// ── TacModExt ─────────────────────────────────────────────────────────────────

/// An extended utility type for TacMod.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TacModExt {
    /// A tag for identifying this utility instance.
    pub tag: u32,
    /// An optional description string.
    pub description: Option<String>,
}

#[allow(dead_code)]
impl TacModExt {
    /// Creates a new default instance.
    pub fn new() -> Self {
        Self {
            tag: 0,
            description: None,
        }
    }

    /// Sets the tag.
    pub fn with_tag(mut self, tag: u32) -> Self {
        self.tag = tag;
        self
    }

    /// Sets the description.
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Returns `true` if the description is set.
    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }
}

// ── TacModExtUtil ─────────────────────────────────────────────────────────────

/// Extended utility buffer for TacMod.
pub struct TacModExtUtil {
    pub key: String,
    pub data: Vec<i64>,
    pub active: bool,
    pub flags: u32,
}

#[allow(dead_code)]
impl TacModExtUtil {
    pub fn new(key: &str) -> Self {
        TacModExtUtil {
            key: key.to_string(),
            data: Vec::new(),
            active: true,
            flags: 0,
        }
    }

    pub fn push(&mut self, v: i64) {
        self.data.push(v);
    }
    pub fn pop(&mut self) -> Option<i64> {
        self.data.pop()
    }
    pub fn sum(&self) -> i64 {
        self.data.iter().sum()
    }
    pub fn min_val(&self) -> Option<i64> {
        self.data.iter().copied().reduce(i64::min)
    }
    pub fn max_val(&self) -> Option<i64> {
        self.data.iter().copied().reduce(i64::max)
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn clear(&mut self) {
        self.data.clear();
    }
    pub fn set_flag(&mut self, bit: u32) {
        self.flags |= 1 << bit;
    }
    pub fn has_flag(&self, bit: u32) -> bool {
        self.flags & (1 << bit) != 0
    }
    pub fn deactivate(&mut self) {
        self.active = false;
    }
    pub fn activate(&mut self) {
        self.active = true;
    }
}

// ── TacModExtMap ──────────────────────────────────────────────────────────────

/// An extended map for TacMod keys to values.
#[allow(dead_code)]
pub struct TacModExtMap<V> {
    pub data: HashMap<String, V>,
    pub default_key: Option<String>,
}

#[allow(dead_code)]
impl<V: Clone + Default> TacModExtMap<V> {
    pub fn new() -> Self {
        TacModExtMap {
            data: HashMap::new(),
            default_key: None,
        }
    }

    pub fn insert(&mut self, key: &str, value: V) {
        self.data.insert(key.to_string(), value);
    }

    pub fn get(&self, key: &str) -> Option<&V> {
        self.data.get(key)
    }

    pub fn get_or_default(&self, key: &str) -> V {
        self.data.get(key).cloned().unwrap_or_default()
    }

    pub fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }
    pub fn remove(&mut self, key: &str) -> Option<V> {
        self.data.remove(key)
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn set_default(&mut self, key: &str) {
        self.default_key = Some(key.to_string());
    }

    pub fn keys_sorted(&self) -> Vec<&String> {
        let mut keys: Vec<&String> = self.data.keys().collect();
        keys.sort();
        keys
    }
}

impl<V: Clone + Default> Default for TacModExtMap<V> {
    fn default() -> Self {
        Self::new()
    }
}

// ── TacModWindow ──────────────────────────────────────────────────────────────

/// A sliding window accumulator for TacMod.
#[allow(dead_code)]
pub struct TacModWindow {
    pub buffer: VecDeque<f64>,
    pub capacity: usize,
    pub running_sum: f64,
}

#[allow(dead_code)]
impl TacModWindow {
    pub fn new(capacity: usize) -> Self {
        TacModWindow {
            buffer: VecDeque::new(),
            capacity,
            running_sum: 0.0,
        }
    }

    pub fn push(&mut self, v: f64) {
        if self.buffer.len() >= self.capacity {
            if let Some(old) = self.buffer.pop_front() {
                self.running_sum -= old;
            }
        }
        self.buffer.push_back(v);
        self.running_sum += v;
    }

    pub fn mean(&self) -> f64 {
        if self.buffer.is_empty() {
            0.0
        } else {
            self.running_sum / self.buffer.len() as f64
        }
    }

    pub fn variance(&self) -> f64 {
        if self.buffer.len() < 2 {
            return 0.0;
        }
        let m = self.mean();
        self.buffer.iter().map(|&x| (x - m).powi(2)).sum::<f64>() / self.buffer.len() as f64
    }

    pub fn std_dev(&self) -> f64 {
        self.variance().sqrt()
    }
    pub fn len(&self) -> usize {
        self.buffer.len()
    }
    pub fn is_full(&self) -> bool {
        self.buffer.len() >= self.capacity
    }
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}

// ── TacModBuilder ─────────────────────────────────────────────────────────────

/// A builder pattern for TacMod.
#[allow(dead_code)]
pub struct TacModBuilder {
    pub name: String,
    pub items: Vec<String>,
    pub config: HashMap<String, String>,
}

#[allow(dead_code)]
impl TacModBuilder {
    pub fn new(name: &str) -> Self {
        TacModBuilder {
            name: name.to_string(),
            items: Vec::new(),
            config: HashMap::new(),
        }
    }

    pub fn add_item(mut self, item: &str) -> Self {
        self.items.push(item.to_string());
        self
    }

    pub fn set_config(mut self, key: &str, value: &str) -> Self {
        self.config.insert(key.to_string(), value.to_string());
        self
    }

    pub fn item_count(&self) -> usize {
        self.items.len()
    }
    pub fn has_config(&self, key: &str) -> bool {
        self.config.contains_key(key)
    }
    pub fn get_config(&self, key: &str) -> Option<&str> {
        self.config.get(key).map(|s| s.as_str())
    }

    pub fn build_summary(&self) -> String {
        format!(
            "{}: {} items, {} config keys",
            self.name,
            self.items.len(),
            self.config.len()
        )
    }
}

// ── TacModState / TacModStateMachine ─────────────────────────────────────────

/// A state machine for TacMod.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TacModState {
    Initial,
    Running,
    Paused,
    Complete,
    Failed(String),
}

#[allow(dead_code)]
impl TacModState {
    pub fn is_terminal(&self) -> bool {
        matches!(self, TacModState::Complete | TacModState::Failed(_))
    }

    pub fn can_run(&self) -> bool {
        matches!(self, TacModState::Initial | TacModState::Paused)
    }
    pub fn is_running(&self) -> bool {
        matches!(self, TacModState::Running)
    }
    pub fn error_msg(&self) -> Option<&str> {
        match self {
            TacModState::Failed(s) => Some(s),
            _ => None,
        }
    }
}

/// A state machine controller for TacMod.
#[allow(dead_code)]
pub struct TacModStateMachine {
    pub state: TacModState,
    pub transitions: usize,
    pub history: Vec<String>,
}

#[allow(dead_code)]
impl TacModStateMachine {
    pub fn new() -> Self {
        TacModStateMachine {
            state: TacModState::Initial,
            transitions: 0,
            history: Vec::new(),
        }
    }

    pub fn transition_to(&mut self, new_state: TacModState) -> bool {
        if self.state.is_terminal() {
            return false;
        }
        let desc = format!("{:?} -> {:?}", self.state, new_state);
        self.state = new_state;
        self.transitions += 1;
        self.history.push(desc);
        true
    }

    pub fn start(&mut self) -> bool {
        self.transition_to(TacModState::Running)
    }
    pub fn pause(&mut self) -> bool {
        self.transition_to(TacModState::Paused)
    }
    pub fn complete(&mut self) -> bool {
        self.transition_to(TacModState::Complete)
    }
    pub fn fail(&mut self, msg: &str) -> bool {
        self.transition_to(TacModState::Failed(msg.to_string()))
    }
    pub fn num_transitions(&self) -> usize {
        self.transitions
    }
}

impl Default for TacModStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

// ── TacModWorkQueue ───────────────────────────────────────────────────────────

/// A work queue for TacMod items.
#[allow(dead_code)]
pub struct TacModWorkQueue {
    pub pending: VecDeque<String>,
    pub processed: Vec<String>,
    pub capacity: usize,
}

#[allow(dead_code)]
impl TacModWorkQueue {
    pub fn new(capacity: usize) -> Self {
        TacModWorkQueue {
            pending: VecDeque::new(),
            processed: Vec::new(),
            capacity,
        }
    }

    pub fn enqueue(&mut self, item: String) -> bool {
        if self.pending.len() >= self.capacity {
            return false;
        }
        self.pending.push_back(item);
        true
    }

    pub fn dequeue(&mut self) -> Option<String> {
        let item = self.pending.pop_front()?;
        self.processed.push(item.clone());
        Some(item)
    }

    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
    pub fn processed_count(&self) -> usize {
        self.processed.len()
    }
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
    pub fn is_full(&self) -> bool {
        self.pending.len() >= self.capacity
    }
    pub fn total_processed(&self) -> usize {
        self.processed.len()
    }
}

// ── TacModCounterMap ──────────────────────────────────────────────────────────

/// A counter map for TacMod frequency analysis.
#[allow(dead_code)]
pub struct TacModCounterMap {
    pub counts: HashMap<String, usize>,
    pub total: usize,
}

#[allow(dead_code)]
impl TacModCounterMap {
    pub fn new() -> Self {
        TacModCounterMap {
            counts: HashMap::new(),
            total: 0,
        }
    }

    pub fn increment(&mut self, key: &str) {
        *self.counts.entry(key.to_string()).or_insert(0) += 1;
        self.total += 1;
    }

    pub fn count(&self, key: &str) -> usize {
        *self.counts.get(key).unwrap_or(&0)
    }

    pub fn frequency(&self, key: &str) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.count(key) as f64 / self.total as f64
        }
    }

    pub fn most_common(&self) -> Option<(&String, usize)> {
        self.counts
            .iter()
            .max_by_key(|(_, &v)| v)
            .map(|(k, &v)| (k, v))
    }

    pub fn num_unique(&self) -> usize {
        self.counts.len()
    }
    pub fn is_empty(&self) -> bool {
        self.counts.is_empty()
    }
}

impl Default for TacModCounterMap {
    fn default() -> Self {
        Self::new()
    }
}

// ── TacModExt2 ────────────────────────────────────────────────────────────────

/// An extended utility type for TacMod (variant 2).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TacModExt2 {
    /// A numeric tag.
    pub tag: u32,
}

#[allow(dead_code)]
impl TacModExt2 {
    /// Creates a new instance.
    pub fn new() -> Self {
        Self { tag: 0 }
    }
}
