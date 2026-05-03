//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum BranchPredictability {
    AlwaysTaken,
    AlwaysNotTaken,
    MostlyTaken(f64),
    MostlyNotTaken(f64),
    Unpredictable,
}
impl BranchPredictability {
    #[allow(dead_code)]
    pub fn from_frequency(taken_freq: f64, total: f64) -> Self {
        if total == 0.0 {
            return BranchPredictability::Unpredictable;
        }
        let ratio = taken_freq / total;
        if ratio >= 0.95 {
            BranchPredictability::AlwaysTaken
        } else if ratio <= 0.05 {
            BranchPredictability::AlwaysNotTaken
        } else if ratio >= 0.75 {
            BranchPredictability::MostlyTaken(ratio)
        } else if ratio <= 0.25 {
            BranchPredictability::MostlyNotTaken(ratio)
        } else {
            BranchPredictability::Unpredictable
        }
    }
    #[allow(dead_code)]
    pub fn is_biased(&self) -> bool {
        !matches!(self, BranchPredictability::Unpredictable)
    }
    #[allow(dead_code)]
    pub fn emit_hint(&self) -> Option<&str> {
        match self {
            BranchPredictability::AlwaysTaken | BranchPredictability::MostlyTaken(_) => {
                Some("[[likely]]")
            }
            BranchPredictability::AlwaysNotTaken | BranchPredictability::MostlyNotTaken(_) => {
                Some("[[unlikely]]")
            }
            BranchPredictability::Unpredictable => None,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ThinLtoPgoData {
    pub module_hash: u64,
    pub function_profiles: Vec<FunctionProfile>,
    pub summary_flags: u32,
}
impl ThinLtoPgoData {
    #[allow(dead_code)]
    pub fn new(module_hash: u64) -> Self {
        ThinLtoPgoData {
            module_hash,
            function_profiles: Vec::new(),
            summary_flags: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_profile(&mut self, profile: FunctionProfile) {
        self.function_profiles.push(profile);
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.function_profiles.is_empty()
    }
    #[allow(dead_code)]
    pub fn hot_function_names(&self, threshold: u64) -> Vec<&str> {
        self.function_profiles
            .iter()
            .filter(|p| p.is_hot_function(threshold))
            .map(|p| p.name.as_str())
            .collect()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PgoAnnotatedFunction {
    pub name: String,
    pub entry_count: u64,
    pub inline_hint: Option<InlineHint>,
    pub hot_attributes: Vec<String>,
}
impl PgoAnnotatedFunction {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, entry_count: u64) -> Self {
        PgoAnnotatedFunction {
            name: name.into(),
            entry_count,
            inline_hint: None,
            hot_attributes: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn with_inline_hint(mut self, hint: InlineHint) -> Self {
        self.inline_hint = Some(hint);
        self
    }
    #[allow(dead_code)]
    pub fn add_hot_attribute(&mut self, attr: impl Into<String>) {
        self.hot_attributes.push(attr.into());
    }
    #[allow(dead_code)]
    pub fn emit_llvm_attrs(&self) -> String {
        let mut attrs = Vec::new();
        attrs.push(format!(
            "!prof !{{!\"func_entry_count\", i64 {}}}",
            self.entry_count
        ));
        if let Some(ref hint) = self.inline_hint {
            match hint {
                InlineHint::AlwaysInline => attrs.push("alwaysinline".to_string()),
                InlineHint::NeverInline => attrs.push("noinline".to_string()),
                InlineHint::InlineWithBenefit(b) => {
                    attrs.push(format!("inlinehint /* benefit: {:.2} */", b))
                }
            }
        }
        attrs.join(" ")
    }
}
#[allow(dead_code)]
pub struct WholeProgramDevirt {
    pub vtable_map: std::collections::HashMap<String, Vec<String>>,
    pub call_profiles: Vec<VirtualCallRecord>,
    pub min_speculation_threshold: f64,
}
impl WholeProgramDevirt {
    #[allow(dead_code)]
    pub fn new() -> Self {
        WholeProgramDevirt {
            vtable_map: std::collections::HashMap::new(),
            call_profiles: Vec::new(),
            min_speculation_threshold: 0.8,
        }
    }
    #[allow(dead_code)]
    pub fn register_vtable(&mut self, class: impl Into<String>, methods: Vec<String>) {
        self.vtable_map.insert(class.into(), methods);
    }
    #[allow(dead_code)]
    pub fn add_call_profile(&mut self, profile: VirtualCallRecord) {
        self.call_profiles.push(profile);
    }
    #[allow(dead_code)]
    pub fn speculation_opportunities(&self) -> Vec<(&VirtualCallRecord, &str, f64)> {
        self.call_profiles
            .iter()
            .filter_map(|p| {
                if let Some((target, ratio)) = p.dominant_target() {
                    if ratio >= self.min_speculation_threshold && !p.is_monomorphic() {
                        return Some((p, target, ratio));
                    }
                }
                None
            })
            .collect()
    }
    #[allow(dead_code)]
    pub fn class_count(&self) -> usize {
        self.vtable_map.len()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EdgeProfile {
    pub from_block: u32,
    pub to_block: u32,
    pub execution_count: u64,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProfileMerger {
    pub(super) profiles: Vec<RawProfileData>,
    pub(super) weight_mode: MergeWeightMode,
}
impl ProfileMerger {
    #[allow(dead_code)]
    pub fn new(mode: MergeWeightMode) -> Self {
        ProfileMerger {
            profiles: Vec::new(),
            weight_mode: mode,
        }
    }
    #[allow(dead_code)]
    pub fn add_profile(&mut self, profile: RawProfileData) {
        self.profiles.push(profile);
    }
    #[allow(dead_code)]
    pub fn merge_all(&self) -> Option<RawProfileData> {
        if self.profiles.is_empty() {
            return None;
        }
        let mut result = self.profiles[0].clone();
        for p in &self.profiles[1..] {
            result.merge(p);
        }
        match self.weight_mode {
            MergeWeightMode::Equal => {
                let n = self.profiles.len() as u64;
                result.normalize(n);
            }
            MergeWeightMode::MaxCount => {
                let max = result.max_count();
                if max > 0 {
                    result.normalize(max);
                }
            }
            MergeWeightMode::Proportional => {}
        }
        Some(result)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RawProfileData {
    pub version: u32,
    pub num_counters: u64,
    pub data: Vec<u64>,
}
impl RawProfileData {
    #[allow(dead_code)]
    pub fn new(version: u32) -> Self {
        RawProfileData {
            version,
            num_counters: 0,
            data: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add_counter(&mut self, value: u64) {
        self.data.push(value);
        self.num_counters += 1;
    }
    #[allow(dead_code)]
    pub fn merge(&mut self, other: &RawProfileData) {
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a = a.saturating_add(*b);
        }
    }
    #[allow(dead_code)]
    pub fn max_count(&self) -> u64 {
        self.data.iter().copied().max().unwrap_or(0)
    }
    #[allow(dead_code)]
    pub fn total_count(&self) -> u64 {
        self.data.iter().sum()
    }
    #[allow(dead_code)]
    pub fn normalize(&mut self, factor: u64) {
        if factor == 0 {
            return;
        }
        for v in &mut self.data {
            *v /= factor;
        }
    }
}
#[allow(dead_code)]
pub struct PgoExtra {
    pub x: u32,
}
impl PgoExtra {
    #[allow(dead_code)]
    pub fn new() -> Self {
        PgoExtra { x: 0 }
    }
    #[allow(dead_code)]
    pub fn value(&self) -> u32 {
        self.x
    }
    #[allow(dead_code)]
    pub fn increment(&mut self) {
        self.x += 1;
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum PgoDataFormat {
    LlvmRaw,
    LlvmText,
    GccGcda,
    SpeeddataAutofdo,
    PropellerProtobuf,
}
impl PgoDataFormat {
    #[allow(dead_code)]
    pub fn file_extension(&self) -> &str {
        match self {
            PgoDataFormat::LlvmRaw => "profraw",
            PgoDataFormat::LlvmText => "proftext",
            PgoDataFormat::GccGcda => "gcda",
            PgoDataFormat::SpeeddataAutofdo => "afdo",
            PgoDataFormat::PropellerProtobuf => "propeller",
        }
    }
    #[allow(dead_code)]
    pub fn merge_tool(&self) -> &str {
        match self {
            PgoDataFormat::LlvmRaw | PgoDataFormat::LlvmText => "llvm-profdata",
            PgoDataFormat::GccGcda => "gcov",
            PgoDataFormat::SpeeddataAutofdo => "create_gcov",
            PgoDataFormat::PropellerProtobuf => "propeller_opt",
        }
    }
    #[allow(dead_code)]
    pub fn emit_merge_command(&self, inputs: &[&str], output: &str) -> String {
        match self {
            PgoDataFormat::LlvmRaw => {
                format!(
                    "llvm-profdata merge -output={} {}",
                    output,
                    inputs.join(" ")
                )
            }
            PgoDataFormat::LlvmText => {
                format!(
                    "llvm-profdata merge -text -output={} {}",
                    output,
                    inputs.join(" ")
                )
            }
            PgoDataFormat::GccGcda => {
                format!("gcov --merge {} -o {}", inputs.join(" "), output)
            }
            _ => format!("# merge not supported for {:?}", self),
        }
    }
}
#[allow(dead_code)]
pub struct ContextSensitiveProfile {
    pub context_stack: Vec<String>,
    pub entry_count: u64,
    pub children: Vec<ContextSensitiveProfile>,
}
impl ContextSensitiveProfile {
    #[allow(dead_code)]
    pub fn new(context: Vec<String>, count: u64) -> Self {
        ContextSensitiveProfile {
            context_stack: context,
            entry_count: count,
            children: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add_child(&mut self, child: ContextSensitiveProfile) {
        self.children.push(child);
    }
    #[allow(dead_code)]
    pub fn depth(&self) -> usize {
        self.context_stack.len()
    }
    #[allow(dead_code)]
    pub fn total_count_in_subtree(&self) -> u64 {
        let child_total: u64 = self
            .children
            .iter()
            .map(|c| c.total_count_in_subtree())
            .sum();
        self.entry_count + child_total
    }
    #[allow(dead_code)]
    pub fn flatten(&self) -> Vec<(&[String], u64)> {
        let mut result = vec![(self.context_stack.as_slice(), self.entry_count)];
        for child in &self.children {
            result.extend(child.flatten());
        }
        result
    }
}
/// Configuration knobs for the PGO pass.
#[derive(Debug, Clone)]
pub struct PgoConfig {
    /// Call-count threshold above which a function is considered "hot".
    pub hot_threshold: u64,
    /// Whether to inline hot callees at their call sites.
    pub inline_hot: bool,
    /// Whether to emit specialised clones of hot polymorphic functions.
    pub specialize_hot: bool,
    /// Upper bound on the IR-node size of a callee eligible for inlining.
    pub max_inline_size: usize,
}
/// The profile-guided optimisation pass.
///
/// Consumes a [`ProfileData`] snapshot and a [`PgoConfig`], then answers
/// per-site queries such as "should I inline this callee?" and produces a
/// bulk [`OptAction`] recommendation list via [`PgoPass::optimize_call_sites`].
pub struct PgoPass {
    pub config: PgoConfig,
    pub profile: ProfileData,
}
impl PgoPass {
    /// Create a new `PgoPass` with the given configuration and an empty
    /// profile.
    pub fn new(config: PgoConfig) -> Self {
        Self {
            config,
            profile: ProfileData::new(),
        }
    }
    /// Replace the current profile with `data`.
    pub fn load_profile(&mut self, data: ProfileData) {
        self.profile = data;
    }
    /// Return `true` when `func_name` should be inlined at a call site whose
    /// callee has an estimated IR size of `size_estimate` nodes.
    ///
    /// Inlining is enabled when:
    /// - `config.inline_hot` is set, AND
    /// - `func_name` is hot (exceeds the threshold), AND
    /// - `size_estimate <= config.max_inline_size`.
    pub fn should_inline(&self, func_name: &str, size_estimate: usize) -> bool {
        self.config.inline_hot
            && self.profile.is_hot(func_name)
            && size_estimate <= self.config.max_inline_size
    }
    /// Return `true` when a specialised clone of `func_name` should be
    /// generated.
    ///
    /// Specialisation is enabled when:
    /// - `config.specialize_hot` is set, AND
    /// - `func_name` is hot.
    pub fn should_specialize(&self, func_name: &str) -> bool {
        self.config.specialize_hot && self.profile.is_hot(func_name)
    }
    /// Produce a list of [`OptAction`] recommendations for a collection of
    /// `(function_name, size_estimate)` pairs representing call sites.
    ///
    /// The index of each entry in `functions` is used as the `call_site`
    /// discriminant for `Specialize` actions.
    pub fn optimize_call_sites(&self, functions: &[(String, usize)]) -> Vec<OptAction> {
        functions
            .iter()
            .enumerate()
            .map(|(idx, (name, size))| {
                if self.should_inline(name, *size) {
                    OptAction::Inline {
                        caller: format!("__caller_{}", idx),
                        callee: name.clone(),
                    }
                } else if self.should_specialize(name) {
                    OptAction::Specialize {
                        func: name.clone(),
                        call_site: idx,
                    }
                } else {
                    OptAction::Noop
                }
            })
            .collect()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum InlineHint {
    AlwaysInline,
    NeverInline,
    InlineWithBenefit(f64),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InstrumentationCounters {
    pub function_entry: u64,
    pub branch_taken: Vec<u64>,
    pub branch_not_taken: Vec<u64>,
    pub value_profiles: Vec<u64>,
}
impl InstrumentationCounters {
    #[allow(dead_code)]
    pub fn new(branch_count: usize, value_count: usize) -> Self {
        InstrumentationCounters {
            function_entry: 0,
            branch_taken: vec![0; branch_count],
            branch_not_taken: vec![0; branch_count],
            value_profiles: vec![0; value_count],
        }
    }
    #[allow(dead_code)]
    pub fn record_entry(&mut self) {
        self.function_entry += 1;
    }
    #[allow(dead_code)]
    pub fn record_branch(&mut self, branch_id: usize, taken: bool) {
        if branch_id < self.branch_taken.len() {
            if taken {
                self.branch_taken[branch_id] += 1;
            } else {
                self.branch_not_taken[branch_id] += 1;
            }
        }
    }
    #[allow(dead_code)]
    pub fn record_value(&mut self, value_id: usize, _value: u64) {
        if value_id < self.value_profiles.len() {
            self.value_profiles[value_id] += 1;
        }
    }
    #[allow(dead_code)]
    pub fn branch_bias(&self, branch_id: usize) -> Option<f64> {
        if branch_id >= self.branch_taken.len() {
            return None;
        }
        let taken = self.branch_taken[branch_id];
        let not_taken = self.branch_not_taken[branch_id];
        let total = taken + not_taken;
        if total == 0 {
            return None;
        }
        Some(taken as f64 / total as f64)
    }
    #[allow(dead_code)]
    pub fn serialize(&self) -> Vec<u64> {
        let mut data = vec![self.function_entry];
        data.extend_from_slice(&self.branch_taken);
        data.extend_from_slice(&self.branch_not_taken);
        data.extend_from_slice(&self.value_profiles);
        data
    }
}
#[allow(dead_code)]
pub struct DevirtualizationPass {
    pub records: Vec<VirtualCallRecord>,
    pub monomorphic_threshold: f64,
}
impl DevirtualizationPass {
    #[allow(dead_code)]
    pub fn new() -> Self {
        DevirtualizationPass {
            records: Vec::new(),
            monomorphic_threshold: 0.95,
        }
    }
    #[allow(dead_code)]
    pub fn add_record(&mut self, rec: VirtualCallRecord) {
        self.records.push(rec);
    }
    #[allow(dead_code)]
    pub fn devirtualize_candidates(&self) -> Vec<&VirtualCallRecord> {
        self.records
            .iter()
            .filter(|r| {
                if let Some((_, ratio)) = r.dominant_target() {
                    ratio >= self.monomorphic_threshold
                } else {
                    false
                }
            })
            .collect()
    }
    #[allow(dead_code)]
    pub fn speculation_candidates(&self) -> Vec<&VirtualCallRecord> {
        self.records
            .iter()
            .filter(|r| r.is_bimorphic() && !r.is_monomorphic())
            .collect()
    }
}
#[allow(dead_code)]
pub struct BoltInstrumentationConfig {
    pub reorder_blocks: bool,
    pub reorder_functions: bool,
    pub split_functions: bool,
    pub dyno_stats: bool,
    pub plt_call_opt: bool,
    pub peepholes: bool,
}
impl BoltInstrumentationConfig {
    #[allow(dead_code)]
    pub fn default_bolt() -> Self {
        BoltInstrumentationConfig {
            reorder_blocks: true,
            reorder_functions: true,
            split_functions: true,
            dyno_stats: false,
            plt_call_opt: true,
            peepholes: true,
        }
    }
    #[allow(dead_code)]
    pub fn emit_flags(&self) -> Vec<String> {
        let mut flags = Vec::new();
        if self.reorder_blocks {
            flags.push("--reorder-blocks=ext-tsp".to_string());
        }
        if self.reorder_functions {
            flags.push("--reorder-functions=hfsort".to_string());
        }
        if self.split_functions {
            flags.push("--split-functions".to_string());
        }
        if self.dyno_stats {
            flags.push("--dyno-stats".to_string());
        }
        if self.plt_call_opt {
            flags.push("--plt=hot".to_string());
        }
        if self.peepholes {
            flags.push("--peepholes=all".to_string());
        }
        flags
    }
}
/// Collected profiling data: call frequencies and call-graph edge weights.
#[derive(Debug, Clone, Default)]
pub struct ProfileData {
    /// How many times each function was called.
    pub call_counts: HashMap<String, u64>,
    /// Functions whose call count exceeds the hot threshold.
    pub hot_functions: Vec<String>,
    /// Edge weights in the dynamic call graph: `(caller, callee) -> count`.
    pub edge_counts: HashMap<(String, String), u64>,
}
impl ProfileData {
    /// Create an empty `ProfileData`.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record one call to `func`.
    pub fn record_call(&mut self, func: &str) {
        *self.call_counts.entry(func.to_owned()).or_insert(0) += 1;
    }
    /// Record one traversal of the call-graph edge `caller → callee`.
    pub fn record_edge(&mut self, caller: &str, callee: &str) {
        *self
            .edge_counts
            .entry((caller.to_owned(), callee.to_owned()))
            .or_insert(0) += 1;
    }
    /// Populate `hot_functions` with every function whose call count exceeds
    /// `threshold`.  The list is sorted descending by call count for
    /// deterministic output.
    pub fn mark_hot(&mut self, threshold: u64) {
        let mut hot: Vec<(String, u64)> = self
            .call_counts
            .iter()
            .filter(|(_, &count)| count > threshold)
            .map(|(name, &count)| (name.clone(), count))
            .collect();
        hot.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        self.hot_functions = hot.into_iter().map(|(name, _)| name).collect();
    }
    /// Return `true` if `func` is in the hot-functions list.
    pub fn is_hot(&self, func: &str) -> bool {
        self.hot_functions.iter().any(|f| f == func)
    }
    /// Return the top-`k` functions sorted descending by call count.
    /// If `k` exceeds the number of recorded functions the full list is
    /// returned.
    pub fn top_k_functions(&self, k: usize) -> Vec<(String, u64)> {
        let mut entries: Vec<(String, u64)> = self
            .call_counts
            .iter()
            .map(|(name, &count)| (name.clone(), count))
            .collect();
        entries.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        entries.into_iter().take(k).collect()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PgoWorkflow {
    pub phase: PgoPhase,
    pub input_profile: Option<String>,
    pub output_profile: Option<String>,
    pub optimization_level: u8,
}
impl PgoWorkflow {
    #[allow(dead_code)]
    pub fn new_instrumentation() -> Self {
        PgoWorkflow {
            phase: PgoPhase::Instrumentation,
            input_profile: None,
            output_profile: Some("default.profraw".to_string()),
            optimization_level: 0,
        }
    }
    #[allow(dead_code)]
    pub fn new_optimization(profile: impl Into<String>) -> Self {
        PgoWorkflow {
            phase: PgoPhase::Optimization,
            input_profile: Some(profile.into()),
            output_profile: None,
            optimization_level: 3,
        }
    }
    #[allow(dead_code)]
    pub fn emit_flags(&self) -> Vec<String> {
        let mut flags = Vec::new();
        match self.phase {
            PgoPhase::Instrumentation => {
                flags.push("-fprofile-generate".to_string());
                if let Some(ref out) = self.output_profile {
                    flags.push(format!("-fprofile-dir={}", out));
                }
            }
            PgoPhase::Optimization => {
                if let Some(ref inp) = self.input_profile {
                    flags.push(format!("-fprofile-use={}", inp));
                }
                flags.push(format!("-O{}", self.optimization_level));
            }
            _ => {}
        }
        flags
    }
}
/// An optimisation action recommended by the PGO pass.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptAction {
    /// Inline `callee` into `caller`.
    Inline { caller: String, callee: String },
    /// Emit a specialised clone of `func` for the given `call_site` index.
    Specialize { func: String, call_site: usize },
    /// No optimisation is applicable.
    Noop,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum MergeWeightMode {
    Equal,
    Proportional,
    MaxCount,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VirtualCallRecord {
    pub callsite_id: u32,
    pub targets: Vec<(String, u64)>,
    pub total_calls: u64,
}
impl VirtualCallRecord {
    #[allow(dead_code)]
    pub fn new(callsite_id: u32) -> Self {
        VirtualCallRecord {
            callsite_id,
            targets: Vec::new(),
            total_calls: 0,
        }
    }
    #[allow(dead_code)]
    pub fn record_call(&mut self, type_name: impl Into<String>, count: u64) {
        let name = type_name.into();
        if let Some(entry) = self.targets.iter_mut().find(|(n, _)| n == &name) {
            entry.1 += count;
        } else {
            self.targets.push((name, count));
        }
        self.total_calls += count;
    }
    #[allow(dead_code)]
    pub fn dominant_target(&self) -> Option<(&str, f64)> {
        if self.total_calls == 0 {
            return None;
        }
        self.targets
            .iter()
            .max_by_key(|(_, c)| c)
            .map(|(name, count)| (name.as_str(), *count as f64 / self.total_calls as f64))
    }
    #[allow(dead_code)]
    pub fn is_monomorphic(&self) -> bool {
        if let Some((_, ratio)) = self.dominant_target() {
            ratio >= 0.99
        } else {
            false
        }
    }
    #[allow(dead_code)]
    pub fn is_bimorphic(&self) -> bool {
        self.targets.len() == 2
    }
}
#[allow(dead_code)]
pub struct GlobalInstrumentationRegistry {
    pub(super) functions: std::collections::HashMap<String, InstrumentationCounters>,
}
impl GlobalInstrumentationRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        GlobalInstrumentationRegistry {
            functions: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, name: impl Into<String>, branches: usize, values: usize) {
        self.functions
            .insert(name.into(), InstrumentationCounters::new(branches, values));
    }
    #[allow(dead_code)]
    pub fn get_mut(&mut self, name: &str) -> Option<&mut InstrumentationCounters> {
        self.functions.get_mut(name)
    }
    #[allow(dead_code)]
    pub fn get(&self, name: &str) -> Option<&InstrumentationCounters> {
        self.functions.get(name)
    }
    #[allow(dead_code)]
    pub fn total_entries(&self) -> u64 {
        self.functions.values().map(|c| c.function_entry).sum()
    }
    #[allow(dead_code)]
    pub fn function_count(&self) -> usize {
        self.functions.len()
    }
    #[allow(dead_code)]
    pub fn export_profile(&self) -> Vec<SampleRecord> {
        self.functions
            .iter()
            .map(|(name, counters)| {
                let mut rec = SampleRecord::new(name.clone());
                rec.head_samples = counters.function_entry;
                rec.body_samples = counters.branch_taken.iter().sum::<u64>()
                    + counters.branch_not_taken.iter().sum::<u64>();
                rec
            })
            .collect()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum PgoDecision {
    Inlined { callee: String, benefit: f64 },
    NotInlined { callee: String, reason: String },
    Unrolled { loop_id: u32, factor: u32 },
    Vectorized { loop_id: u32, width: u32 },
    HotColdSplit { function: String },
    BlockReordered { function: String, blocks: u32 },
    StackPromotion { site_id: u32 },
    Devirtualized { callsite: u32, target: String },
}
impl PgoDecision {
    #[allow(dead_code)]
    pub fn description(&self) -> String {
        match self {
            PgoDecision::Inlined { callee, benefit } => {
                format!("Inlined {} (benefit: {:.2})", callee, benefit)
            }
            PgoDecision::NotInlined { callee, reason } => {
                format!("Not inlined {}: {}", callee, reason)
            }
            PgoDecision::Unrolled { loop_id, factor } => {
                format!("Unrolled loop {} by {}x", loop_id, factor)
            }
            PgoDecision::Vectorized { loop_id, width } => {
                format!("Vectorized loop {} with width {}", loop_id, width)
            }
            PgoDecision::HotColdSplit { function } => {
                format!("Hot-cold split: {}", function)
            }
            PgoDecision::BlockReordered { function, blocks } => {
                format!("Reordered {} blocks in {}", blocks, function)
            }
            PgoDecision::StackPromotion { site_id } => {
                format!("Stack promotion at site {}", site_id)
            }
            PgoDecision::Devirtualized { callsite, target } => {
                format!("Devirtualized callsite {} -> {}", callsite, target)
            }
        }
    }
    #[allow(dead_code)]
    pub fn is_beneficial(&self) -> bool {
        !matches!(self, PgoDecision::NotInlined { .. })
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProfileSummary {
    pub total_samples: u64,
    pub max_function_count: u64,
    pub num_functions: usize,
    pub detailed_summary: Vec<(u32, u64, u64)>,
}
impl ProfileSummary {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ProfileSummary {
            total_samples: 0,
            max_function_count: 0,
            num_functions: 0,
            detailed_summary: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn compute_from_profiles(profiles: &[FunctionProfile]) -> Self {
        let total_samples: u64 = profiles.iter().map(|p| p.total_calls).sum();
        let max_function_count = profiles.iter().map(|p| p.total_calls).max().unwrap_or(0);
        ProfileSummary {
            total_samples,
            max_function_count,
            num_functions: profiles.len(),
            detailed_summary: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.total_samples == 0
    }
}
#[allow(dead_code)]
pub struct PgoInfrastructure {
    pub enabled: bool,
    pub phase: String,
}
impl PgoInfrastructure {
    #[allow(dead_code)]
    pub fn new(phase: impl Into<String>) -> Self {
        PgoInfrastructure {
            enabled: true,
            phase: phase.into(),
        }
    }
    #[allow(dead_code)]
    pub fn phase(&self) -> &str {
        &self.phase
    }
}
#[allow(dead_code)]
pub struct SampleRecord {
    pub function_name: String,
    pub head_samples: u64,
    pub body_samples: u64,
    pub callsites: Vec<(u64, String, u64)>,
}
impl SampleRecord {
    #[allow(dead_code)]
    pub fn new(function_name: impl Into<String>) -> Self {
        SampleRecord {
            function_name: function_name.into(),
            head_samples: 0,
            body_samples: 0,
            callsites: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add_callsite(&mut self, offset: u64, callee: impl Into<String>, count: u64) {
        self.callsites.push((offset, callee.into(), count));
    }
    #[allow(dead_code)]
    pub fn total_samples(&self) -> u64 {
        self.head_samples + self.body_samples
    }
    #[allow(dead_code)]
    pub fn emit_prof_text(&self) -> String {
        let mut out = format!("{}:{}\n", self.function_name, self.head_samples);
        for (offset, callee, count) in &self.callsites {
            out.push_str(&format!(" {}: {} {} {}\n", offset, callee, count, count));
        }
        out
    }
}
/// A lightweight instrumentation pass that generates profiling stubs and
/// summary reports from collected data.
pub struct InstrumentationPass;
impl InstrumentationPass {
    /// Create a new `InstrumentationPass`.
    pub fn new() -> Self {
        Self
    }
    /// Return a source-level instrumentation stub for `name`.
    ///
    /// The stub is a comment-annotated placeholder that a real backend would
    /// lower to a counter increment in the emitted code.
    pub fn instrument_function(&self, name: &str) -> String {
        format!(
            "/* [PGO] instrumentation stub for `{}` */\n\
             __pgo_counter_increment(\"{}\");",
            name, name
        )
    }
    /// Generate a human-readable profile report from `data`.
    pub fn generate_profile_report(&self, data: &ProfileData) -> String {
        let mut lines: Vec<String> = Vec::new();
        lines.push("=== PGO Profile Report ===".to_owned());
        lines.push(format!("Hot functions ({}):", data.hot_functions.len()));
        for f in &data.hot_functions {
            let count = data.call_counts.get(f).copied().unwrap_or(0);
            lines.push(format!("  {} — {} calls", f, count));
        }
        lines.push(format!(
            "Total tracked functions: {}",
            data.call_counts.len()
        ));
        lines.push(format!(
            "Total call-graph edges: {}",
            data.edge_counts.len()
        ));
        lines.join("\n")
    }
}
#[allow(dead_code)]
pub struct PgoPassManager {
    pub passes: Vec<String>,
    pub feedback: PgoFeedback,
    pub heuristic: InlineHeuristic,
    pub hot_cold_split: HotColdSplit,
}
impl PgoPassManager {
    #[allow(dead_code)]
    pub fn new(feedback: PgoFeedback) -> Self {
        PgoPassManager {
            passes: Vec::new(),
            feedback,
            heuristic: InlineHeuristic::default_heuristic(),
            hot_cold_split: HotColdSplit::new(80.0),
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: impl Into<String>) {
        self.passes.push(pass.into());
    }
    #[allow(dead_code)]
    pub fn compute_hot_cold(&mut self) {
        let profiles = self.feedback.profiles.clone();
        self.hot_cold_split.classify(&profiles);
    }
    #[allow(dead_code)]
    pub fn is_hot_function(&self, name: &str) -> bool {
        self.hot_cold_split.hot_functions.iter().any(|f| f == name)
    }
    #[allow(dead_code)]
    pub fn pass_count(&self) -> usize {
        self.passes.len()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LoopIterationProfile {
    pub loop_id: u32,
    pub function_name: String,
    pub iteration_counts: Vec<u64>,
    pub trip_count_avg: f64,
    pub trip_count_max: u64,
}
impl LoopIterationProfile {
    #[allow(dead_code)]
    pub fn new(loop_id: u32, function_name: impl Into<String>) -> Self {
        LoopIterationProfile {
            loop_id,
            function_name: function_name.into(),
            iteration_counts: Vec::new(),
            trip_count_avg: 0.0,
            trip_count_max: 0,
        }
    }
    #[allow(dead_code)]
    pub fn record_execution(&mut self, iterations: u64) {
        self.iteration_counts.push(iterations);
        if iterations > self.trip_count_max {
            self.trip_count_max = iterations;
        }
        let sum: u64 = self.iteration_counts.iter().sum();
        self.trip_count_avg = sum as f64 / self.iteration_counts.len() as f64;
    }
    #[allow(dead_code)]
    pub fn is_constant_trip_count(&self) -> bool {
        if self.iteration_counts.len() < 2 {
            return true;
        }
        let first = self.iteration_counts[0];
        self.iteration_counts.iter().all(|&c| c == first)
    }
    #[allow(dead_code)]
    pub fn estimated_unroll_factor(&self) -> u32 {
        if self.trip_count_avg <= 1.0 {
            return 1;
        }
        if self.trip_count_avg <= 4.0 {
            return 2;
        }
        if self.trip_count_avg <= 8.0 {
            return 4;
        }
        if self.trip_count_avg <= 16.0 {
            return 8;
        }
        16
    }
}
#[allow(dead_code)]
pub struct PgoFeedback {
    pub run_count: u32,
    pub profiles: Vec<FunctionProfile>,
    pub call_graph: CallGraph,
}
impl PgoFeedback {
    #[allow(dead_code)]
    pub fn new() -> Self {
        PgoFeedback {
            run_count: 0,
            profiles: Vec::new(),
            call_graph: CallGraph::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add_profile(&mut self, profile: FunctionProfile) {
        self.profiles.push(profile);
    }
    #[allow(dead_code)]
    pub fn increment_run(&mut self) {
        self.run_count += 1;
    }
    #[allow(dead_code)]
    pub fn normalize_counts(&mut self) {
        if self.run_count == 0 {
            return;
        }
        for p in &mut self.profiles {
            p.total_calls /= self.run_count as u64;
            for b in &mut p.blocks {
                b.execution_count /= self.run_count as u64;
            }
        }
    }
    #[allow(dead_code)]
    pub fn top_hot_functions(&self, n: usize, threshold: u64) -> Vec<&FunctionProfile> {
        let mut hot: Vec<&FunctionProfile> = self
            .profiles
            .iter()
            .filter(|p| p.is_hot_function(threshold))
            .collect();
        hot.sort_by_key(|b| std::cmp::Reverse(b.total_calls));
        hot.truncate(n);
        hot
    }
}
#[allow(dead_code)]
pub struct SampleBasedProfileGenerator {
    pub stack_traces: Vec<Vec<String>>,
    pub sample_interval: u64,
}
impl SampleBasedProfileGenerator {
    #[allow(dead_code)]
    pub fn new(sample_interval: u64) -> Self {
        SampleBasedProfileGenerator {
            stack_traces: Vec::new(),
            sample_interval,
        }
    }
    #[allow(dead_code)]
    pub fn add_trace(&mut self, trace: Vec<String>) {
        self.stack_traces.push(trace);
    }
    #[allow(dead_code)]
    pub fn build_flat_profile(&self) -> std::collections::HashMap<String, u64> {
        let mut counts = std::collections::HashMap::new();
        for trace in &self.stack_traces {
            if let Some(top) = trace.first() {
                *counts.entry(top.clone()).or_insert(0) += 1;
            }
        }
        counts
    }
    #[allow(dead_code)]
    pub fn build_inclusive_profile(&self) -> std::collections::HashMap<String, u64> {
        let mut counts = std::collections::HashMap::new();
        for trace in &self.stack_traces {
            let mut seen = std::collections::HashSet::new();
            for frame in trace {
                if seen.insert(frame.clone()) {
                    *counts.entry(frame.clone()).or_insert(0) += 1;
                }
            }
        }
        counts
    }
    #[allow(dead_code)]
    pub fn top_functions(&self, n: usize) -> Vec<(String, u64)> {
        let profile = self.build_flat_profile();
        let mut entries: Vec<(String, u64)> = profile.into_iter().collect();
        entries.sort_by_key(|b| std::cmp::Reverse(b.1));
        entries.truncate(n);
        entries
    }
}
#[allow(dead_code)]
pub struct AutoFdoConfig {
    pub perf_data_file: String,
    pub binary_path: String,
    pub profile_output: String,
    pub sampling_frequency: u32,
}
impl AutoFdoConfig {
    #[allow(dead_code)]
    pub fn new(binary: impl Into<String>) -> Self {
        AutoFdoConfig {
            perf_data_file: "perf.data".to_string(),
            binary_path: binary.into(),
            profile_output: "profile.afdo".to_string(),
            sampling_frequency: 4000,
        }
    }
    #[allow(dead_code)]
    pub fn emit_perf_command(&self) -> String {
        format!(
            "perf record -e cycles:u -j any,u -a -F {} -o {} -- {}",
            self.sampling_frequency, self.perf_data_file, self.binary_path
        )
    }
    #[allow(dead_code)]
    pub fn emit_create_gcov_command(&self) -> String {
        format!(
            "create_gcov --binary={} --profile={} --gcov={}",
            self.binary_path, self.perf_data_file, self.profile_output
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FunctionProfile {
    pub name: String,
    pub total_calls: u64,
    pub blocks: Vec<BlockProfile>,
    pub edges: Vec<EdgeProfile>,
    pub average_call_depth: f64,
}
impl FunctionProfile {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        FunctionProfile {
            name: name.into(),
            total_calls: 0,
            blocks: Vec::new(),
            edges: Vec::new(),
            average_call_depth: 0.0,
        }
    }
    #[allow(dead_code)]
    pub fn add_block(&mut self, block_id: u32, count: u64, hot_threshold: u64) {
        self.blocks.push(BlockProfile {
            block_id,
            execution_count: count,
            is_hot: count >= hot_threshold,
        });
    }
    #[allow(dead_code)]
    pub fn hot_blocks(&self) -> Vec<&BlockProfile> {
        self.blocks.iter().filter(|b| b.is_hot).collect()
    }
    #[allow(dead_code)]
    pub fn total_block_executions(&self) -> u64 {
        self.blocks.iter().map(|b| b.execution_count).sum()
    }
    #[allow(dead_code)]
    pub fn is_hot_function(&self, threshold: u64) -> bool {
        self.total_calls >= threshold
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum PgoPhase {
    Instrumentation,
    Training,
    Optimization,
    Verification,
}
#[allow(dead_code)]
pub struct CoverageReport {
    pub total_lines: u64,
    pub covered_lines: u64,
    pub total_branches: u64,
    pub covered_branches: u64,
    pub function_coverage: Vec<(String, bool)>,
}
impl CoverageReport {
    #[allow(dead_code)]
    pub fn new() -> Self {
        CoverageReport {
            total_lines: 0,
            covered_lines: 0,
            total_branches: 0,
            covered_branches: 0,
            function_coverage: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn line_coverage_pct(&self) -> f64 {
        if self.total_lines == 0 {
            return 0.0;
        }
        (self.covered_lines as f64 / self.total_lines as f64) * 100.0
    }
    #[allow(dead_code)]
    pub fn branch_coverage_pct(&self) -> f64 {
        if self.total_branches == 0 {
            return 0.0;
        }
        (self.covered_branches as f64 / self.total_branches as f64) * 100.0
    }
    #[allow(dead_code)]
    pub fn function_coverage_pct(&self) -> f64 {
        if self.function_coverage.is_empty() {
            return 0.0;
        }
        let covered = self.function_coverage.iter().filter(|(_, c)| *c).count();
        (covered as f64 / self.function_coverage.len() as f64) * 100.0
    }
    #[allow(dead_code)]
    pub fn add_function(&mut self, name: impl Into<String>, covered: bool) {
        self.function_coverage.push((name.into(), covered));
    }
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        format!(
            "Lines: {:.1}%, Branches: {:.1}%, Functions: {:.1}%",
            self.line_coverage_pct(),
            self.branch_coverage_pct(),
            self.function_coverage_pct()
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PropellerEdge {
    pub from_addr: u64,
    pub to_addr: u64,
    pub weight: u64,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BlockProfile {
    pub block_id: u32,
    pub execution_count: u64,
    pub is_hot: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InlineHeuristic {
    pub call_count_threshold: u64,
    pub size_limit: usize,
    pub depth_limit: u32,
    pub benefit_multiplier: f64,
}
impl InlineHeuristic {
    #[allow(dead_code)]
    pub fn aggressive() -> Self {
        InlineHeuristic {
            call_count_threshold: 10,
            size_limit: 500,
            depth_limit: 10,
            benefit_multiplier: 2.0,
        }
    }
    #[allow(dead_code)]
    pub fn conservative() -> Self {
        InlineHeuristic {
            call_count_threshold: 100,
            size_limit: 50,
            depth_limit: 3,
            benefit_multiplier: 0.5,
        }
    }
    #[allow(dead_code)]
    pub fn default_heuristic() -> Self {
        InlineHeuristic {
            call_count_threshold: 50,
            size_limit: 100,
            depth_limit: 5,
            benefit_multiplier: 1.0,
        }
    }
    #[allow(dead_code)]
    pub fn should_inline(&self, call_count: u64, callee_size: usize, current_depth: u32) -> bool {
        call_count >= self.call_count_threshold
            && callee_size <= self.size_limit
            && current_depth <= self.depth_limit
    }
    #[allow(dead_code)]
    pub fn compute_benefit(&self, call_count: u64, callee_size: usize) -> f64 {
        let base = call_count as f64 / (callee_size as f64 + 1.0);
        base * self.benefit_multiplier
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HotColdSplit {
    pub hot_functions: Vec<String>,
    pub cold_functions: Vec<String>,
    pub hot_threshold_percentile: f64,
}
impl HotColdSplit {
    #[allow(dead_code)]
    pub fn new(hot_threshold: f64) -> Self {
        HotColdSplit {
            hot_functions: Vec::new(),
            cold_functions: Vec::new(),
            hot_threshold_percentile: hot_threshold,
        }
    }
    #[allow(dead_code)]
    pub fn classify(&mut self, profiles: &[FunctionProfile]) {
        if profiles.is_empty() {
            return;
        }
        let mut counts: Vec<(String, u64)> = profiles
            .iter()
            .map(|p| (p.name.clone(), p.total_calls))
            .collect();
        counts.sort_by_key(|b| std::cmp::Reverse(b.1));
        let total: u64 = counts.iter().map(|(_, c)| c).sum();
        let threshold = (total as f64 * self.hot_threshold_percentile / 100.0) as u64;
        let mut cumulative = 0u64;
        for (name, count) in &counts {
            cumulative += count;
            if cumulative <= threshold {
                self.hot_functions.push(name.clone());
            } else {
                self.cold_functions.push(name.clone());
            }
        }
    }
    #[allow(dead_code)]
    pub fn hot_count(&self) -> usize {
        self.hot_functions.len()
    }
    #[allow(dead_code)]
    pub fn cold_count(&self) -> usize {
        self.cold_functions.len()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CallGraph {
    pub edges: Vec<(String, String, u64)>,
}
impl CallGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        CallGraph { edges: Vec::new() }
    }
    #[allow(dead_code)]
    pub fn add_edge(&mut self, caller: impl Into<String>, callee: impl Into<String>, count: u64) {
        self.edges.push((caller.into(), callee.into(), count));
    }
    #[allow(dead_code)]
    pub fn callers_of(&self, target: &str) -> Vec<(&str, u64)> {
        self.edges
            .iter()
            .filter(|(_, callee, _)| callee == target)
            .map(|(caller, _, count)| (caller.as_str(), *count))
            .collect()
    }
    #[allow(dead_code)]
    pub fn callees_of(&self, source: &str) -> Vec<(&str, u64)> {
        self.edges
            .iter()
            .filter(|(caller, _, _)| caller == source)
            .map(|(_, callee, count)| (callee.as_str(), *count))
            .collect()
    }
    #[allow(dead_code)]
    pub fn total_call_count(&self) -> u64 {
        self.edges.iter().map(|(_, _, c)| c).sum()
    }
    #[allow(dead_code)]
    pub fn hot_call_sites(&self, threshold: u64) -> Vec<(&str, &str, u64)> {
        self.edges
            .iter()
            .filter(|(_, _, c)| *c >= threshold)
            .map(|(caller, callee, count)| (caller.as_str(), callee.as_str(), *count))
            .collect()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MemoryAccessPattern {
    pub access_id: u32,
    pub is_sequential: bool,
    pub stride: i64,
    pub cache_hit_rate: f64,
    pub access_count: u64,
}
impl MemoryAccessPattern {
    #[allow(dead_code)]
    pub fn new(access_id: u32) -> Self {
        MemoryAccessPattern {
            access_id,
            is_sequential: false,
            stride: 0,
            cache_hit_rate: 0.0,
            access_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn is_cache_friendly(&self) -> bool {
        self.is_sequential && self.stride > 0 && self.stride <= 64 && self.cache_hit_rate >= 0.9
    }
    #[allow(dead_code)]
    pub fn prefetch_distance(&self) -> i64 {
        if self.is_sequential {
            8
        } else {
            0
        }
    }
}
#[allow(dead_code)]
pub struct AllocationProfile {
    pub function_name: String,
    pub allocation_sites: Vec<AllocationSiteProfile>,
}
#[allow(dead_code)]
impl AllocationProfile {
    pub fn new(function_name: impl Into<String>) -> Self {
        AllocationProfile {
            function_name: function_name.into(),
            allocation_sites: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add_site(&mut self, site: AllocationSiteProfile) {
        self.allocation_sites.push(site);
    }
    #[allow(dead_code)]
    pub fn stack_promotion_candidates(&self) -> Vec<&AllocationSiteProfile> {
        self.allocation_sites
            .iter()
            .filter(|s| s.stack_promotion_candidate())
            .collect()
    }
    #[allow(dead_code)]
    pub fn total_allocations(&self) -> u64 {
        self.allocation_sites.iter().map(|s| s.alloc_count).sum()
    }
}
#[allow(dead_code)]
pub struct PgoOptimizationLog {
    pub(super) decisions: Vec<(String, PgoDecision)>,
    pub(super) total_beneficial: u32,
    pub(super) total_non_beneficial: u32,
}
impl PgoOptimizationLog {
    #[allow(dead_code)]
    pub fn new() -> Self {
        PgoOptimizationLog {
            decisions: Vec::new(),
            total_beneficial: 0,
            total_non_beneficial: 0,
        }
    }
    #[allow(dead_code)]
    pub fn record(&mut self, function: impl Into<String>, decision: PgoDecision) {
        if decision.is_beneficial() {
            self.total_beneficial += 1;
        } else {
            self.total_non_beneficial += 1;
        }
        self.decisions.push((function.into(), decision));
    }
    #[allow(dead_code)]
    pub fn generate_report(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("PGO Optimization Report:\n"));
        out.push_str(&format!("  Total decisions: {}\n", self.decisions.len()));
        out.push_str(&format!("  Beneficial: {}\n", self.total_beneficial));
        out.push_str(&format!(
            "  Non-beneficial: {}\n\n",
            self.total_non_beneficial
        ));
        for (func, decision) in &self.decisions {
            out.push_str(&format!("  [{}] {}\n", func, decision.description()));
        }
        out
    }
    #[allow(dead_code)]
    pub fn filter_by_function(&self, name: &str) -> Vec<&PgoDecision> {
        self.decisions
            .iter()
            .filter(|(f, _)| f == name)
            .map(|(_, d)| d)
            .collect()
    }
    #[allow(dead_code)]
    pub fn inline_decisions(&self) -> Vec<&PgoDecision> {
        self.decisions
            .iter()
            .filter(|(_, d)| {
                matches!(
                    d,
                    PgoDecision::Inlined { .. } | PgoDecision::NotInlined { .. }
                )
            })
            .map(|(_, d)| d)
            .collect()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AllocationSiteProfile {
    pub site_id: u32,
    pub alloc_count: u64,
    pub avg_size: f64,
    pub max_size: u64,
    pub live_at_exit: u64,
}
impl AllocationSiteProfile {
    #[allow(dead_code)]
    pub fn new(site_id: u32) -> Self {
        AllocationSiteProfile {
            site_id,
            alloc_count: 0,
            avg_size: 0.0,
            max_size: 0,
            live_at_exit: 0,
        }
    }
    #[allow(dead_code)]
    pub fn is_short_lived(&self) -> bool {
        self.alloc_count > 0 && self.live_at_exit == 0
    }
    #[allow(dead_code)]
    pub fn stack_promotion_candidate(&self) -> bool {
        self.is_short_lived() && self.max_size <= 4096
    }
}
#[allow(dead_code)]
pub struct PgoStatisticsReport {
    pub total_functions: usize,
    pub hot_functions: usize,
    pub cold_functions: usize,
    pub inlined_callsites: usize,
    pub devirtualized_sites: usize,
    pub stack_promoted_sites: usize,
    pub blocks_reordered: u64,
    pub loops_unrolled: usize,
    pub loops_vectorized: usize,
}
impl PgoStatisticsReport {
    #[allow(dead_code)]
    pub fn new() -> Self {
        PgoStatisticsReport {
            total_functions: 0,
            hot_functions: 0,
            cold_functions: 0,
            inlined_callsites: 0,
            devirtualized_sites: 0,
            stack_promoted_sites: 0,
            blocks_reordered: 0,
            loops_unrolled: 0,
            loops_vectorized: 0,
        }
    }
    #[allow(dead_code)]
    pub fn from_log(log: &PgoOptimizationLog) -> Self {
        let mut rep = Self::new();
        for (_, decision) in &log.decisions {
            match decision {
                PgoDecision::Inlined { .. } => rep.inlined_callsites += 1,
                PgoDecision::Devirtualized { .. } => rep.devirtualized_sites += 1,
                PgoDecision::StackPromotion { .. } => rep.stack_promoted_sites += 1,
                PgoDecision::Unrolled { .. } => rep.loops_unrolled += 1,
                PgoDecision::Vectorized { .. } => rep.loops_vectorized += 1,
                PgoDecision::BlockReordered { blocks, .. } => {
                    rep.blocks_reordered += *blocks as u64;
                }
                _ => {}
            }
        }
        rep
    }
    #[allow(dead_code)]
    pub fn format_summary(&self) -> String {
        format!(
            "Functions: {} ({} hot, {} cold)\n\
             Inlined: {} callsites\n\
             Devirtualized: {} sites\n\
             Stack promoted: {} sites\n\
             Loops unrolled: {}, vectorized: {}\n\
             Blocks reordered: {}",
            self.total_functions,
            self.hot_functions,
            self.cold_functions,
            self.inlined_callsites,
            self.devirtualized_sites,
            self.stack_promoted_sites,
            self.loops_unrolled,
            self.loops_vectorized,
            self.blocks_reordered
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PropellerFunctionInfo {
    pub name: String,
    pub address: u64,
    pub size: u64,
    pub entry_count: u64,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PropellerProfile {
    pub binary_id: String,
    pub hot_functions: Vec<PropellerFunctionInfo>,
    pub cfg_edges: Vec<PropellerEdge>,
}
impl PropellerProfile {
    #[allow(dead_code)]
    pub fn new(binary_id: impl Into<String>) -> Self {
        PropellerProfile {
            binary_id: binary_id.into(),
            hot_functions: Vec::new(),
            cfg_edges: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add_function(&mut self, func: PropellerFunctionInfo) {
        self.hot_functions.push(func);
    }
    #[allow(dead_code)]
    pub fn add_edge(&mut self, edge: PropellerEdge) {
        self.cfg_edges.push(edge);
    }
    #[allow(dead_code)]
    pub fn total_edge_weight(&self) -> u64 {
        self.cfg_edges.iter().map(|e| e.weight).sum()
    }
    #[allow(dead_code)]
    pub fn emit_protobuf_format(&self) -> String {
        let mut out = format!("binary_id: \"{}\"\n", self.binary_id);
        for f in &self.hot_functions {
            out.push_str(&format!(
                "function {{ name: \"{}\" addr: {} size: {} count: {} }}\n",
                f.name, f.address, f.size, f.entry_count
            ));
        }
        out
    }
}
