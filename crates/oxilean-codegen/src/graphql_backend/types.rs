//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum GQLIntrospectionKind {
    Scalar,
    Object,
    Interface,
    Union,
    Enum,
    InputObject,
    List,
    NonNull,
}
#[allow(dead_code)]
pub struct GQLRateLimitDirective {
    pub max_calls: u32,
    pub window_seconds: u32,
    pub per: GQLRateLimitPer,
}
impl GQLRateLimitDirective {
    #[allow(dead_code)]
    pub fn new(max_calls: u32, window_seconds: u32) -> Self {
        GQLRateLimitDirective {
            max_calls,
            window_seconds,
            per: GQLRateLimitPer::User,
        }
    }
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let per = match self.per {
            GQLRateLimitPer::Ip => "IP",
            GQLRateLimitPer::User => "USER",
            GQLRateLimitPer::Global => "GLOBAL",
        };
        format!(
            "@rateLimit(limit: {}, duration: {}, per: \"{}\")",
            self.max_calls, self.window_seconds, per
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GQLInputObject {
    pub name: String,
    pub fields: Vec<GQLInputField>,
    pub description: Option<String>,
    pub directives: Vec<String>,
}
impl GQLInputObject {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        GQLInputObject {
            name: name.into(),
            fields: Vec::new(),
            description: None,
            directives: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add_field(&mut self, field: GQLInputField) {
        self.fields.push(field);
    }
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let mut out = String::new();
        if let Some(ref d) = self.description {
            out.push_str(&format!("\"\"\"\n{}\n\"\"\"\n", d));
        }
        out.push_str(&format!("input {} {{\n", self.name));
        for f in &self.fields {
            if let Some(ref d) = f.description {
                out.push_str(&format!("  \"\"\"{}\"\"\"\n", d));
            }
            out.push_str(&format!("  {}: {}", f.name, emit_gql_type(&f.ty)));
            if let Some(ref dv) = f.default_value {
                out.push_str(&format!(" = {}", dv));
            }
            out.push('\n');
        }
        out.push('}');
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GQLErrorLocation {
    pub line: u32,
    pub column: u32,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GQLPersistedQuery {
    pub hash: String,
    pub query: String,
    pub version: u32,
}
impl GQLPersistedQuery {
    #[allow(dead_code)]
    pub fn new(query: impl Into<String>) -> Self {
        let q = query.into();
        let hash = format!("{:x}", q.len() * 31 + 7);
        GQLPersistedQuery {
            hash,
            query: q,
            version: 1,
        }
    }
    #[allow(dead_code)]
    pub fn emit_apq_extension(&self) -> String {
        format!(
            "{{\"persistedQuery\":{{\"version\":{},\"sha256Hash\":\"{}\"}}}}",
            self.version, self.hash
        )
    }
}
#[allow(dead_code)]
pub struct GQLTypeNameMap {
    pub(super) mapping: std::collections::HashMap<String, String>,
}
impl GQLTypeNameMap {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut m = std::collections::HashMap::new();
        m.insert("Int".to_string(), "i32".to_string());
        m.insert("Float".to_string(), "f64".to_string());
        m.insert("String".to_string(), "String".to_string());
        m.insert("Boolean".to_string(), "bool".to_string());
        m.insert("ID".to_string(), "String".to_string());
        GQLTypeNameMap { mapping: m }
    }
    #[allow(dead_code)]
    pub fn add_mapping(&mut self, gql: impl Into<String>, rust: impl Into<String>) {
        self.mapping.insert(gql.into(), rust.into());
    }
    #[allow(dead_code)]
    pub fn lookup(&self, gql_name: &str) -> Option<&String> {
        self.mapping.get(gql_name)
    }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.mapping.len()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GQLCacheControl {
    pub max_age_seconds: u32,
    pub scope: GQLCacheScope,
    pub inherit_max_age: bool,
}
impl GQLCacheControl {
    #[allow(dead_code)]
    pub fn public(max_age: u32) -> Self {
        GQLCacheControl {
            max_age_seconds: max_age,
            scope: GQLCacheScope::Public,
            inherit_max_age: false,
        }
    }
    #[allow(dead_code)]
    pub fn private(max_age: u32) -> Self {
        GQLCacheControl {
            max_age_seconds: max_age,
            scope: GQLCacheScope::Private,
            inherit_max_age: false,
        }
    }
    #[allow(dead_code)]
    pub fn emit_directive(&self) -> String {
        let scope = match self.scope {
            GQLCacheScope::Public => "PUBLIC",
            GQLCacheScope::Private => "PRIVATE",
        };
        format!(
            "@cacheControl(maxAge: {}, scope: {})",
            self.max_age_seconds, scope
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GQLDataloader {
    pub batch_size: usize,
    pub cache_ttl_ms: u64,
    pub entity_type: String,
}
impl GQLDataloader {
    #[allow(dead_code)]
    pub fn new(entity_type: impl Into<String>) -> Self {
        GQLDataloader {
            batch_size: 100,
            cache_ttl_ms: 5000,
            entity_type: entity_type.into(),
        }
    }
    #[allow(dead_code)]
    pub fn emit_ts_loader(&self) -> String {
        format!(
            "const {}Loader = new DataLoader<string, {}>(\n  async (ids) => load{}ByIds(ids),\n  {{ maxBatchSize: {} }}\n);",
            self.entity_type.to_lowercase(), self.entity_type, self.entity_type, self
            .batch_size
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GQLDeprecation {
    pub field_name: String,
    pub reason: String,
    pub since_version: Option<String>,
    pub replacement: Option<String>,
}
impl GQLDeprecation {
    #[allow(dead_code)]
    pub fn new(field_name: impl Into<String>, reason: impl Into<String>) -> Self {
        GQLDeprecation {
            field_name: field_name.into(),
            reason: reason.into(),
            since_version: None,
            replacement: None,
        }
    }
    #[allow(dead_code)]
    pub fn emit_directive(&self) -> String {
        format!("@deprecated(reason: \"{}\")", self.reason)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GQLOperation {
    pub op_type: GQLOperationType,
    pub name: Option<String>,
    pub variables: Vec<GQLVariable>,
    pub selections: Vec<GQLSelectionField>,
}
impl GQLOperation {
    #[allow(dead_code)]
    pub fn query(name: impl Into<String>) -> Self {
        GQLOperation {
            op_type: GQLOperationType::Query,
            name: Some(name.into()),
            variables: Vec::new(),
            selections: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn mutation(name: impl Into<String>) -> Self {
        GQLOperation {
            op_type: GQLOperationType::Mutation,
            name: Some(name.into()),
            variables: Vec::new(),
            selections: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let op_name = match self.op_type {
            GQLOperationType::Query => "query",
            GQLOperationType::Mutation => "mutation",
            GQLOperationType::Subscription => "subscription",
        };
        let mut out = String::new();
        out.push_str(op_name);
        if let Some(ref name) = self.name {
            out.push(' ');
            out.push_str(name);
        }
        if !self.variables.is_empty() {
            out.push('(');
            let vars: Vec<String> = self
                .variables
                .iter()
                .map(|v| format!("${}: {}", v.name, emit_gql_type(&v.ty)))
                .collect();
            out.push_str(&vars.join(", "));
            out.push(')');
        }
        out.push_str(" {\n");
        for sel in &self.selections {
            out.push_str(&sel.emit(1));
            out.push('\n');
        }
        out.push('}');
        out
    }
}
#[allow(dead_code)]
pub struct GQLCodegen {
    pub(super) schema: GQLSchemaExtended,
    pub(super) language: GQLCodegenTarget,
}
impl GQLCodegen {
    #[allow(dead_code)]
    pub fn new(schema: GQLSchemaExtended, target: GQLCodegenTarget) -> Self {
        GQLCodegen {
            schema,
            language: target,
        }
    }
    #[allow(dead_code)]
    pub fn generate_types(&self) -> String {
        match self.language {
            GQLCodegenTarget::TypeScript => self.gen_typescript(),
            GQLCodegenTarget::Rust => self.gen_rust(),
            GQLCodegenTarget::Go => self.gen_go(),
            GQLCodegenTarget::Python => self.gen_python(),
        }
    }
    #[allow(dead_code)]
    pub(super) fn gen_typescript(&self) -> String {
        let mut out = String::new();
        for obj in &self.schema.objects {
            out.push_str(&format!("export interface {} {{\n", obj.name));
            for f in &obj.fields {
                out.push_str(&format!("  {}: {};\n", f.name, ts_type(&f.ty)));
            }
            out.push_str("}\n\n");
        }
        for e in &self.schema.enums {
            out.push_str(&format!("export enum {} {{\n", e.name));
            for v in &e.values {
                out.push_str(&format!("  {} = \"{}\",\n", v.name, v.name));
            }
            out.push_str("}\n\n");
        }
        out
    }
    #[allow(dead_code)]
    pub(super) fn gen_rust(&self) -> String {
        let mut out = String::new();
        for obj in &self.schema.objects {
            out.push_str("#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]\n");
            out.push_str(&format!("pub struct {} {{\n", obj.name));
            for f in &obj.fields {
                out.push_str(&format!("    pub {}: {},\n", f.name, rust_type(&f.ty)));
            }
            out.push_str("}\n\n");
        }
        out
    }
    #[allow(dead_code)]
    pub(super) fn gen_go(&self) -> String {
        let mut out = String::new();
        for obj in &self.schema.objects {
            out.push_str(&format!("type {} struct {{\n", obj.name));
            for f in &obj.fields {
                let fname: String = f
                    .name
                    .chars()
                    .enumerate()
                    .map(|(i, c)| {
                        if i == 0 {
                            c.to_uppercase().next().unwrap_or(c)
                        } else {
                            c
                        }
                    })
                    .collect();
                out.push_str(&format!(
                    "    {} {} `json:\"{}\"`\n",
                    fname,
                    go_type(&f.ty),
                    f.name
                ));
            }
            out.push_str("}\n\n");
        }
        out
    }
    #[allow(dead_code)]
    pub(super) fn gen_python(&self) -> String {
        let mut out = String::new();
        out.push_str("from dataclasses import dataclass\nfrom typing import Optional, List\n\n");
        for obj in &self.schema.objects {
            out.push_str("@dataclass\n");
            out.push_str(&format!("class {}:\n", obj.name));
            for f in &obj.fields {
                out.push_str(&format!("    {}: {}\n", f.name, py_type(&f.ty)));
            }
            out.push('\n');
        }
        out
    }
}
/// A complete GraphQL schema (SDL-level).
#[derive(Debug, Clone)]
pub struct GQLSchema {
    pub types: Vec<GQLObject>,
    pub query_type: String,
    pub mutation_type: Option<String>,
}
#[allow(dead_code)]
pub struct GQLLiveQueryExtension {
    pub throttle_ms: u64,
    pub invalidation_keys: Vec<String>,
}
impl GQLLiveQueryExtension {
    #[allow(dead_code)]
    pub fn new(throttle_ms: u64) -> Self {
        GQLLiveQueryExtension {
            throttle_ms,
            invalidation_keys: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add_key(&mut self, key: impl Into<String>) {
        self.invalidation_keys.push(key.into());
    }
    #[allow(dead_code)]
    pub fn emit_extension_header(&self) -> String {
        format!("@live(throttle: {})", self.throttle_ms)
    }
}
/// A GraphQL object type definition.
#[derive(Debug, Clone)]
pub struct GQLObject {
    pub name: String,
    pub fields: Vec<GQLField>,
    /// Names of interfaces this object implements.
    pub implements: Vec<String>,
    /// Optional description for the object type.
    pub description: Option<String>,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct GQLSchemaExtended {
    pub objects: Vec<GQLObject>,
    pub interfaces: Vec<GQLInterface>,
    pub unions: Vec<GQLUnion>,
    pub enums: Vec<GQLEnumDef>,
    pub scalars: Vec<GQLScalar>,
    pub input_objects: Vec<GQLInputObject>,
    pub directives: Vec<GQLDirective>,
    pub query_type: Option<String>,
    pub mutation_type: Option<String>,
    pub subscription_type: Option<String>,
}
impl GQLSchemaExtended {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn add_object(&mut self, obj: GQLObject) {
        self.objects.push(obj);
    }
    #[allow(dead_code)]
    pub fn add_interface(&mut self, iface: GQLInterface) {
        self.interfaces.push(iface);
    }
    #[allow(dead_code)]
    pub fn add_scalar(&mut self, scalar: GQLScalar) {
        self.scalars.push(scalar);
    }
    #[allow(dead_code)]
    pub fn set_query_type(&mut self, name: impl Into<String>) {
        self.query_type = Some(name.into());
    }
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let mut out = String::new();
        for scalar in &self.scalars {
            out.push_str(&scalar.emit());
            out.push_str("\n\n");
        }
        for iface in &self.interfaces {
            out.push_str(&iface.emit());
            out.push_str("\n\n");
        }
        for obj in &self.objects {
            out.push_str(&format!("type {} {{\n", obj.name));
            for f in &obj.fields {
                out.push_str(&format!("  {}: {}\n", f.name, emit_gql_type(&f.ty)));
            }
            out.push_str("}\n\n");
        }
        for u in &self.unions {
            out.push_str(&u.emit());
            out.push_str("\n\n");
        }
        for e in &self.enums {
            out.push_str(&format!("enum {} {{\n", e.name));
            for v in &e.values {
                out.push_str(&format!("  {}\n", v.name));
            }
            out.push_str("}\n\n");
        }
        for inp in &self.input_objects {
            out.push_str(&inp.emit());
            out.push_str("\n\n");
        }
        if self.query_type.is_some() || self.mutation_type.is_some() {
            out.push_str("schema {\n");
            if let Some(ref q) = self.query_type {
                out.push_str(&format!("  query: {}\n", q));
            }
            if let Some(ref m) = self.mutation_type {
                out.push_str(&format!("  mutation: {}\n", m));
            }
            if let Some(ref s) = self.subscription_type {
                out.push_str(&format!("  subscription: {}\n", s));
            }
            out.push('}');
        }
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GQLError {
    pub message: String,
    pub locations: Vec<GQLErrorLocation>,
    pub path: Vec<String>,
    pub extensions: std::collections::HashMap<String, String>,
}
impl GQLError {
    #[allow(dead_code)]
    pub fn new(message: impl Into<String>) -> Self {
        GQLError {
            message: message.into(),
            locations: Vec::new(),
            path: Vec::new(),
            extensions: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn with_location(mut self, line: u32, column: u32) -> Self {
        self.locations.push(GQLErrorLocation { line, column });
        self
    }
    #[allow(dead_code)]
    pub fn emit_json(&self) -> String {
        let locs: Vec<String> = self
            .locations
            .iter()
            .map(|l| format!("{{\"line\":{},\"column\":{}}}", l.line, l.column))
            .collect();
        format!(
            "{{\"message\":\"{}\",\"locations\":[{}]}}",
            self.message,
            locs.join(",")
        )
    }
}
/// A GraphQL type reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GQLType {
    /// A named scalar (e.g. `String`, `Int`, `Boolean`, `ID`, `Float`).
    Scalar(String),
    /// A named object type reference.
    Object(String),
    /// A list wrapper (`[T]`).
    List(Box<GQLType>),
    /// A non-null wrapper (`T!`).
    NonNull(Box<GQLType>),
    /// A named enum type reference.
    Enum(String),
    /// A named interface type reference.
    Interface(String),
    /// A named union type reference.
    Union(String),
}
#[allow(dead_code)]
pub struct GQLSchemaComparator {
    pub old_schema: GQLSchemaExtended,
    pub new_schema: GQLSchemaExtended,
}
impl GQLSchemaComparator {
    #[allow(dead_code)]
    pub fn new(old_schema: GQLSchemaExtended, new_schema: GQLSchemaExtended) -> Self {
        GQLSchemaComparator {
            old_schema,
            new_schema,
        }
    }
    #[allow(dead_code)]
    pub fn added_types(&self) -> Vec<String> {
        let old_names: std::collections::HashSet<_> =
            self.old_schema.objects.iter().map(|o| &o.name).collect();
        self.new_schema
            .objects
            .iter()
            .filter(|o| !old_names.contains(&o.name))
            .map(|o| o.name.clone())
            .collect()
    }
    #[allow(dead_code)]
    pub fn removed_types(&self) -> Vec<String> {
        let new_names: std::collections::HashSet<_> =
            self.new_schema.objects.iter().map(|o| &o.name).collect();
        self.old_schema
            .objects
            .iter()
            .filter(|o| !new_names.contains(&o.name))
            .map(|o| o.name.clone())
            .collect()
    }
    #[allow(dead_code)]
    pub fn is_breaking_change(&self) -> bool {
        !self.removed_types().is_empty()
    }
    #[allow(dead_code)]
    pub fn generate_changelog(&self) -> String {
        let added = self.added_types();
        let removed = self.removed_types();
        let mut log = String::new();
        for t in &added {
            log.push_str(&format!("+ Added type: {}\n", t));
        }
        for t in &removed {
            log.push_str(&format!("- Removed type: {}\n", t));
        }
        log
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GQLVariable {
    pub name: String,
    pub ty: GQLType,
    pub default_value: Option<String>,
}
#[allow(dead_code)]
pub struct GQLPersistedQueryStore {
    pub(super) store: std::collections::HashMap<String, GQLPersistedQuery>,
}
impl GQLPersistedQueryStore {
    #[allow(dead_code)]
    pub fn new() -> Self {
        GQLPersistedQueryStore {
            store: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, pq: GQLPersistedQuery) -> &str {
        let hash = pq.hash.clone();
        self.store.insert(hash.clone(), pq);
        &self.store[&hash].hash
    }
    #[allow(dead_code)]
    pub fn lookup(&self, hash: &str) -> Option<&GQLPersistedQuery> {
        self.store.get(hash)
    }
    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        self.store.len()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GQLQueryComplexity {
    pub max_depth: u32,
    pub max_breadth: u32,
    pub max_complexity: u64,
    pub per_field_cost: u32,
    pub per_list_multiplier: u32,
}
impl GQLQueryComplexity {
    #[allow(dead_code)]
    pub fn default_limits() -> Self {
        GQLQueryComplexity {
            max_depth: 10,
            max_breadth: 50,
            max_complexity: 1000,
            per_field_cost: 1,
            per_list_multiplier: 10,
        }
    }
    #[allow(dead_code)]
    pub fn calculate_selection_complexity(
        &self,
        selections: &[GQLSelectionField],
        depth: u32,
    ) -> u64 {
        if depth > self.max_depth {
            return u64::MAX;
        }
        let mut total = 0u64;
        for sel in selections {
            total = total.saturating_add(self.per_field_cost as u64);
            if !sel.selection_set.is_empty() {
                total = total.saturating_add(
                    self.calculate_selection_complexity(&sel.selection_set, depth + 1)
                        .saturating_mul(self.per_list_multiplier as u64),
                );
            }
        }
        total
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum GQLDirectiveValue {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Enum(String),
    Null,
    List(Vec<GQLDirectiveValue>),
    Object(Vec<(String, GQLDirectiveValue)>),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GQLConnection {
    pub node_type: String,
    pub edge_fields: Vec<GQLField>,
    pub connection_fields: Vec<GQLField>,
}
impl GQLConnection {
    #[allow(dead_code)]
    pub fn new(node_type: impl Into<String>) -> Self {
        GQLConnection {
            node_type: node_type.into(),
            edge_fields: Vec::new(),
            connection_fields: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn emit_edge_type(&self) -> String {
        let edge_name = format!("{}Edge", self.node_type);
        let mut out = format!("type {} {{\n", edge_name);
        out.push_str(&format!("  node: {}\n", self.node_type));
        out.push_str("  cursor: String!\n");
        for f in &self.edge_fields {
            out.push_str(&format!("  {}: {}\n", f.name, emit_gql_type(&f.ty)));
        }
        out.push('}');
        out
    }
    #[allow(dead_code)]
    pub fn emit_connection_type(&self) -> String {
        let conn_name = format!("{}Connection", self.node_type);
        let edge_name = format!("{}Edge", self.node_type);
        let mut out = format!("type {} {{\n", conn_name);
        out.push_str(&format!("  edges: [{}]\n", edge_name));
        out.push_str("  pageInfo: PageInfo!\n");
        out.push_str("  totalCount: Int!\n");
        for f in &self.connection_fields {
            out.push_str(&format!("  {}: {}\n", f.name, emit_gql_type(&f.ty)));
        }
        out.push('}');
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GQLIntrospectionType {
    pub kind: GQLIntrospectionKind,
    pub name: Option<String>,
    pub description: Option<String>,
    pub fields: Vec<GQLIntrospectionField>,
    pub interfaces: Vec<String>,
    pub possible_types: Vec<String>,
    pub enum_values: Vec<String>,
    pub input_fields: Vec<String>,
    pub of_type: Option<Box<GQLIntrospectionType>>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GQLIntrospectionField {
    pub name: String,
    pub description: Option<String>,
    pub args: Vec<GQLInputField>,
    pub ty: GQLIntrospectionType,
    pub is_deprecated: bool,
    pub deprecation_reason: Option<String>,
}
#[allow(dead_code)]
pub struct GQLTypeSystemDocument {
    pub type_defs: Vec<String>,
    pub directives: Vec<GQLDirective>,
    pub scalars: Vec<GQLScalar>,
    pub schema_extensions: Vec<String>,
}
impl GQLTypeSystemDocument {
    #[allow(dead_code)]
    pub fn new() -> Self {
        GQLTypeSystemDocument {
            type_defs: Vec::new(),
            directives: Vec::new(),
            scalars: Vec::new(),
            schema_extensions: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn extend_type(&mut self, type_name: &str, fields: &[&str]) {
        let mut ext = format!("extend type {} {{\n", type_name);
        for field in fields {
            ext.push_str(&format!("  {}\n", field));
        }
        ext.push('}');
        self.schema_extensions.push(ext);
    }
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let mut parts = Vec::new();
        for s in &self.scalars {
            parts.push(s.emit());
        }
        for d in &self.directives {
            parts.push(d.emit());
        }
        for td in &self.type_defs {
            parts.push(td.clone());
        }
        for ext in &self.schema_extensions {
            parts.push(ext.clone());
        }
        parts.join("\n\n")
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct GQLSchemaBuilder {
    pub(super) objects: Vec<GQLObject>,
    pub(super) interfaces: Vec<GQLInterface>,
    pub(super) unions: Vec<GQLUnion>,
    pub(super) enums: Vec<GQLEnumDef>,
    pub(super) scalars: Vec<GQLScalar>,
    pub(super) input_objects: Vec<GQLInputObject>,
    pub(super) custom_directives: Vec<GQLDirective>,
    pub(super) query_type: Option<String>,
    pub(super) mutation_type: Option<String>,
}
impl GQLSchemaBuilder {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn object(mut self, obj: GQLObject) -> Self {
        self.objects.push(obj);
        self
    }
    #[allow(dead_code)]
    pub fn interface(mut self, iface: GQLInterface) -> Self {
        self.interfaces.push(iface);
        self
    }
    #[allow(dead_code)]
    pub fn union_type(mut self, u: GQLUnion) -> Self {
        self.unions.push(u);
        self
    }
    #[allow(dead_code)]
    pub fn enum_type(mut self, e: GQLEnumDef) -> Self {
        self.enums.push(e);
        self
    }
    #[allow(dead_code)]
    pub fn scalar(mut self, s: GQLScalar) -> Self {
        self.scalars.push(s);
        self
    }
    #[allow(dead_code)]
    pub fn input(mut self, inp: GQLInputObject) -> Self {
        self.input_objects.push(inp);
        self
    }
    #[allow(dead_code)]
    pub fn query_type(mut self, name: impl Into<String>) -> Self {
        self.query_type = Some(name.into());
        self
    }
    #[allow(dead_code)]
    pub fn mutation_type(mut self, name: impl Into<String>) -> Self {
        self.mutation_type = Some(name.into());
        self
    }
    #[allow(dead_code)]
    pub fn build(self) -> GQLSchemaExtended {
        GQLSchemaExtended {
            objects: self.objects,
            interfaces: self.interfaces,
            unions: self.unions,
            enums: self.enums,
            scalars: self.scalars,
            input_objects: self.input_objects,
            directives: self.custom_directives,
            query_type: self.query_type,
            mutation_type: self.mutation_type,
            subscription_type: None,
        }
    }
}
#[allow(dead_code)]
pub struct GQLBatchRequest {
    pub operations: Vec<GQLOperation>,
    pub max_batch_size: usize,
}
impl GQLBatchRequest {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        GQLBatchRequest {
            operations: Vec::new(),
            max_batch_size: max_size,
        }
    }
    #[allow(dead_code)]
    pub fn add(&mut self, op: GQLOperation) -> bool {
        if self.operations.len() >= self.max_batch_size {
            return false;
        }
        self.operations.push(op);
        true
    }
    #[allow(dead_code)]
    pub fn emit_batch_json(&self) -> String {
        let ops: Vec<String> = self
            .operations
            .iter()
            .map(|op| {
                format!(
                    "{{\"query\":\"{}\"}}",
                    op.emit().replace('\n', " ").replace('"', "\\\"")
                )
            })
            .collect();
        format!("[{}]", ops.join(","))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum GQLDirectiveLocation {
    Field,
    FieldDefinition,
    Object,
    Interface,
    Union,
    Enum,
    EnumValue,
    InputObject,
    InputFieldDefinition,
    Argument,
    Schema,
    Scalar,
    Query,
    Mutation,
    Subscription,
    FragmentDefinition,
    FragmentSpread,
    InlineFragment,
}
/// A GraphQL enum type definition.
#[derive(Debug, Clone)]
pub struct GQLEnumDef {
    pub name: String,
    pub values: Vec<GQLEnumValue>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GQLStreamDirective {
    pub label: Option<String>,
    pub initial_count: u32,
    pub if_condition: Option<String>,
}
impl GQLStreamDirective {
    #[allow(dead_code)]
    pub fn new(initial_count: u32) -> Self {
        GQLStreamDirective {
            label: None,
            initial_count,
            if_condition: None,
        }
    }
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let mut args = vec![format!("initialCount: {}", self.initial_count)];
        if let Some(ref label) = self.label {
            args.push(format!("label: \"{}\"", label));
        }
        format!("@stream({})", args.join(", "))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum GQLOperationType {
    Query,
    Mutation,
    Subscription,
}
#[allow(dead_code)]
pub struct GQLSdlPrinter {
    pub(super) indent_size: usize,
    pub(super) include_descriptions: bool,
    pub(super) include_deprecated: bool,
}
impl GQLSdlPrinter {
    #[allow(dead_code)]
    pub fn new() -> Self {
        GQLSdlPrinter {
            indent_size: 2,
            include_descriptions: true,
            include_deprecated: true,
        }
    }
    #[allow(dead_code)]
    pub fn with_indent(mut self, size: usize) -> Self {
        self.indent_size = size;
        self
    }
    #[allow(dead_code)]
    pub fn without_descriptions(mut self) -> Self {
        self.include_descriptions = false;
        self
    }
    #[allow(dead_code)]
    pub fn print_object(&self, obj: &GQLObject) -> String {
        let indent = " ".repeat(self.indent_size);
        let mut out = format!("type {} {{\n", obj.name);
        for field in &obj.fields {
            if self.include_descriptions {
                if let Some(ref d) = field.description {
                    out.push_str(&format!("{}\"\"\"{}\"\"\"  \n", indent, d));
                }
            }
            out.push_str(&format!(
                "{}{}: {}\n",
                indent,
                field.name,
                emit_gql_type(&field.ty)
            ));
        }
        out.push('}');
        out
    }
    #[allow(dead_code)]
    pub fn print_enum(&self, e: &GQLEnumDef) -> String {
        let indent = " ".repeat(self.indent_size);
        let mut out = format!("enum {} {{\n", e.name);
        for v in &e.values {
            out.push_str(&format!("{}{}\n", indent, v.name));
        }
        out.push('}');
        out
    }
    #[allow(dead_code)]
    pub fn print_schema(&self, schema: &GQLSchemaExtended) -> String {
        let mut parts = Vec::new();
        for s in &schema.scalars {
            parts.push(s.emit());
        }
        for iface in &schema.interfaces {
            parts.push(iface.emit());
        }
        for obj in &schema.objects {
            parts.push(self.print_object(obj));
        }
        for u in &schema.unions {
            parts.push(u.emit());
        }
        for e in &schema.enums {
            parts.push(self.print_enum(e));
        }
        for inp in &schema.input_objects {
            parts.push(inp.emit());
        }
        parts.join("\n\n")
    }
}
#[allow(dead_code)]
pub enum GQLRateLimitPer {
    Ip,
    User,
    Global,
}
#[allow(dead_code)]
pub struct GQLIntrospectionQuery;
impl GQLIntrospectionQuery {
    #[allow(dead_code)]
    pub fn full_introspection_query() -> &'static str {
        "query IntrospectionQuery { __schema { queryType { name } mutationType { name } subscriptionType { name } types { ...FullType } directives { name description locations args { ...InputValue } } } } fragment FullType on __Type { kind name description fields(includeDeprecated: true) { name description args { ...InputValue } type { ...TypeRef } isDeprecated deprecationReason } inputFields { ...InputValue } interfaces { ...TypeRef } enumValues(includeDeprecated: true) { name description isDeprecated deprecationReason } possibleTypes { ...TypeRef } } fragment InputValue on __InputValue { name description type { ...TypeRef } defaultValue } fragment TypeRef on __Type { kind name ofType { kind name ofType { kind name ofType { kind name ofType { kind name ofType { kind name ofType { kind name } } } } } } }"
    }
    #[allow(dead_code)]
    pub fn type_query(type_name: &str) -> String {
        format!(
            "{{ __type(name: \"{}\") {{ name kind description fields {{ name type {{ name kind }} }} }} }}",
            type_name
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GQLDirective {
    pub name: String,
    pub locations: Vec<GQLDirectiveLocation>,
    pub args: Vec<GQLDirectiveArg>,
    pub is_repeatable: bool,
    pub description: Option<String>,
}
impl GQLDirective {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        GQLDirective {
            name: name.into(),
            locations: Vec::new(),
            args: Vec::new(),
            is_repeatable: false,
            description: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_location(mut self, loc: GQLDirectiveLocation) -> Self {
        self.locations.push(loc);
        self
    }
    #[allow(dead_code)]
    pub fn with_arg(mut self, arg: GQLDirectiveArg) -> Self {
        self.args.push(arg);
        self
    }
    #[allow(dead_code)]
    pub fn repeatable(mut self) -> Self {
        self.is_repeatable = true;
        self
    }
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let mut out = String::new();
        if let Some(ref desc) = self.description {
            out.push_str(&format!("\"\"\"{}\"\"\"\n", desc));
        }
        out.push_str(&format!("directive @{}", self.name));
        if !self.args.is_empty() {
            out.push('(');
            for (i, arg) in self.args.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                out.push_str(&arg.name);
            }
            out.push(')');
        }
        if self.is_repeatable {
            out.push_str(" repeatable");
        }
        out.push_str(" on ");
        let locs: Vec<&str> = self
            .locations
            .iter()
            .map(|l| match l {
                GQLDirectiveLocation::Field => "FIELD",
                GQLDirectiveLocation::FieldDefinition => "FIELD_DEFINITION",
                GQLDirectiveLocation::Object => "OBJECT",
                GQLDirectiveLocation::Interface => "INTERFACE",
                GQLDirectiveLocation::Union => "UNION",
                GQLDirectiveLocation::Enum => "ENUM",
                GQLDirectiveLocation::EnumValue => "ENUM_VALUE",
                GQLDirectiveLocation::InputObject => "INPUT_OBJECT",
                GQLDirectiveLocation::InputFieldDefinition => "INPUT_FIELD_DEFINITION",
                GQLDirectiveLocation::Argument => "ARGUMENT_DEFINITION",
                GQLDirectiveLocation::Schema => "SCHEMA",
                GQLDirectiveLocation::Scalar => "SCALAR",
                GQLDirectiveLocation::Query => "QUERY",
                GQLDirectiveLocation::Mutation => "MUTATION",
                GQLDirectiveLocation::Subscription => "SUBSCRIPTION",
                GQLDirectiveLocation::FragmentDefinition => "FRAGMENT_DEFINITION",
                GQLDirectiveLocation::FragmentSpread => "FRAGMENT_SPREAD",
                GQLDirectiveLocation::InlineFragment => "INLINE_FRAGMENT",
            })
            .collect();
        out.push_str(&locs.join(" | "));
        out
    }
}
#[allow(dead_code)]
pub struct GQLSubscriptionBuilder {
    pub(super) operations: Vec<GQLOperation>,
    pub(super) reconnect_delay_ms: u64,
    pub(super) max_reconnect_attempts: u32,
}
impl GQLSubscriptionBuilder {
    #[allow(dead_code)]
    pub fn new() -> Self {
        GQLSubscriptionBuilder {
            operations: Vec::new(),
            reconnect_delay_ms: 1000,
            max_reconnect_attempts: 5,
        }
    }
    #[allow(dead_code)]
    pub fn add_operation(&mut self, op: GQLOperation) {
        self.operations.push(op);
    }
    #[allow(dead_code)]
    pub fn set_reconnect_delay(&mut self, ms: u64) {
        self.reconnect_delay_ms = ms;
    }
    #[allow(dead_code)]
    pub fn build_ws_message(&self, op: &GQLOperation) -> String {
        format!(
            "{{\"type\":\"subscribe\",\"payload\":{{\"query\":\"{}\"}}}}",
            op.emit().replace('\n', "\\n")
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GQLUnion {
    pub name: String,
    pub members: Vec<String>,
    pub description: Option<String>,
}
impl GQLUnion {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        GQLUnion {
            name: name.into(),
            members: Vec::new(),
            description: None,
        }
    }
    #[allow(dead_code)]
    pub fn add_member(&mut self, member: impl Into<String>) {
        self.members.push(member.into());
    }
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let mut out = String::new();
        if let Some(ref d) = self.description {
            out.push_str(&format!("\"\"\"{}\"\"\"\n", d));
        }
        out.push_str(&format!(
            "union {} = {}",
            self.name,
            self.members.join(" | ")
        ));
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GQLInterface {
    pub name: String,
    pub fields: Vec<GQLField>,
    pub implements: Vec<String>,
    pub description: Option<String>,
}
impl GQLInterface {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        GQLInterface {
            name: name.into(),
            fields: Vec::new(),
            implements: Vec::new(),
            description: None,
        }
    }
    #[allow(dead_code)]
    pub fn add_field(&mut self, field: GQLField) {
        self.fields.push(field);
    }
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let mut out = String::new();
        if let Some(ref d) = self.description {
            out.push_str(&format!("\"\"\"\n{}\n\"\"\"\n", d));
        }
        out.push_str(&format!("interface {}", self.name));
        if !self.implements.is_empty() {
            out.push_str(&format!(" implements {}", self.implements.join(" & ")));
        }
        out.push_str(" {\n");
        for f in &self.fields {
            out.push_str(&format!("  {}: {}\n", f.name, emit_gql_type(&f.ty)));
        }
        out.push('}');
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GQLDirectiveArg {
    pub name: String,
    pub value: GQLDirectiveValue,
}
#[allow(dead_code)]
pub struct GQLSchemaRegistry {
    pub(super) schemas: std::collections::HashMap<String, GQLSchemaExtended>,
}
impl GQLSchemaRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        GQLSchemaRegistry {
            schemas: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, name: impl Into<String>, schema: GQLSchemaExtended) {
        self.schemas.insert(name.into(), schema);
    }
    #[allow(dead_code)]
    pub fn get(&self, name: &str) -> Option<&GQLSchemaExtended> {
        self.schemas.get(name)
    }
    #[allow(dead_code)]
    pub fn list_names(&self) -> Vec<&String> {
        self.schemas.keys().collect()
    }
    #[allow(dead_code)]
    pub fn merge(&mut self, base: &str, overlay: &str) -> Option<GQLSchemaExtended> {
        let base_schema = self.schemas.get(base)?.clone();
        let overlay_schema = self.schemas.get(overlay)?.clone();
        let mut merged = base_schema;
        for obj in overlay_schema.objects {
            merged.objects.push(obj);
        }
        for iface in overlay_schema.interfaces {
            merged.interfaces.push(iface);
        }
        Some(merged)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GQLInputField {
    pub name: String,
    pub ty: GQLType,
    pub default_value: Option<String>,
    pub description: Option<String>,
    pub directives: Vec<String>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GQLNullabilityAnnotation {
    pub field_path: Vec<String>,
    pub nullable: bool,
}
/// A GraphQL enum value.
#[derive(Debug, Clone)]
pub struct GQLEnumValue {
    pub name: String,
    pub description: Option<String>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum GQLCodegenTarget {
    TypeScript,
    Rust,
    Go,
    Python,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GQLResolverSignature {
    pub type_name: String,
    pub field_name: String,
    pub args: Vec<GQLField>,
    pub return_type: GQLType,
    pub is_async: bool,
}
impl GQLResolverSignature {
    #[allow(dead_code)]
    pub fn new(
        type_name: impl Into<String>,
        field_name: impl Into<String>,
        return_type: GQLType,
    ) -> Self {
        GQLResolverSignature {
            type_name: type_name.into(),
            field_name: field_name.into(),
            args: Vec::new(),
            return_type,
            is_async: true,
        }
    }
    #[allow(dead_code)]
    pub fn emit_ts_signature(&self) -> String {
        let args_str = self
            .args
            .iter()
            .map(|f| format!("{}: {}", f.name, ts_type(&f.ty)))
            .collect::<Vec<_>>()
            .join(", ");
        let ret = ts_type(&self.return_type);
        if self.is_async {
            format!("async {}({}) Promise<{}>", self.field_name, args_str, ret)
        } else {
            format!("{}({}) {}", self.field_name, args_str, ret)
        }
    }
}
/// A single field on a GraphQL object or interface.
#[derive(Debug, Clone)]
pub struct GQLField {
    pub name: String,
    pub ty: GQLType,
    /// Whether the field is nullable (i.e. NOT wrapped in NonNull).
    pub nullable: bool,
    pub description: Option<String>,
    /// Arguments this field accepts.
    pub args: Vec<GQLFieldArg>,
}
/// An argument to a GraphQL field.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GQLFieldArg {
    pub name: String,
    pub ty: GQLType,
    pub default_value: Option<String>,
}
/// The GraphQL SDL emitter backend.
pub struct GQLBackend;
impl GQLBackend {
    /// Emit a GraphQL type reference string.
    pub fn emit_type(&self, ty: &GQLType) -> String {
        match ty {
            GQLType::Scalar(name) => name.clone(),
            GQLType::Object(name) => name.clone(),
            GQLType::Enum(name) => name.clone(),
            GQLType::Interface(name) => name.clone(),
            GQLType::Union(name) => name.clone(),
            GQLType::List(inner) => format!("[{}]", self.emit_type(inner)),
            GQLType::NonNull(inner) => format!("{}!", self.emit_type(inner)),
        }
    }
    /// Emit a single field definition line.
    pub fn emit_field(&self, field: &GQLField) -> String {
        let mut ty_str = self.emit_type(&field.ty);
        if !field.nullable {
            if !ty_str.ends_with('!') {
                ty_str.push('!');
            }
        }
        let mut s = String::new();
        if let Some(desc) = &field.description {
            s.push_str(&format!("  \"\"\"{}\"\"\"\n", desc));
        }
        s.push_str(&format!("  {}: {}", field.name, ty_str));
        s
    }
    /// Emit a full `type Foo { ... }` SDL block.
    pub fn emit_object(&self, obj: &GQLObject) -> String {
        let mut s = String::new();
        s.push_str(&format!("type {}", obj.name));
        if !obj.implements.is_empty() {
            s.push_str(&format!(" implements {}", obj.implements.join(" & ")));
        }
        s.push_str(" {\n");
        for field in &obj.fields {
            s.push_str(&self.emit_field(field));
            s.push('\n');
        }
        s.push('}');
        s
    }
    /// Emit an `enum Foo { ... }` SDL block.
    pub fn emit_enum(&self, e: &GQLEnumDef) -> String {
        let mut s = format!("enum {} {{\n", e.name);
        for val in &e.values {
            if let Some(desc) = &val.description {
                s.push_str(&format!("  \"\"\"{}\"\"\"\n", desc));
            }
            s.push_str(&format!("  {}\n", val.name));
        }
        s.push('}');
        s
    }
    /// Emit the top-level `schema { ... }` block and all type definitions.
    pub fn emit_schema(&self, schema: &GQLSchema) -> String {
        let mut s = String::new();
        s.push_str("schema {\n");
        s.push_str(&format!("  query: {}\n", schema.query_type));
        if let Some(ref mt) = schema.mutation_type {
            s.push_str(&format!("  mutation: {}\n", mt));
        }
        s.push_str("}\n\n");
        for obj in &schema.types {
            s.push_str(&self.emit_object(obj));
            s.push_str("\n\n");
        }
        s.trim_end().to_string()
    }
    /// Generate simple resolver stub functions (as pseudo-Rust) for each field.
    pub fn generate_resolver_stubs(&self, schema: &GQLSchema) -> String {
        let mut s = String::new();
        for obj in &schema.types {
            s.push_str(&format!("// Resolvers for {}\n", obj.name));
            for field in &obj.fields {
                let ret = self.emit_type(&field.ty);
                s.push_str(&format!(
                    "fn resolve_{}_{}(ctx: &Context) -> {} {{\n    todo!()\n}}\n",
                    obj.name.to_ascii_lowercase(),
                    field.name,
                    ret
                ));
            }
            s.push('\n');
        }
        s
    }
    /// Build a minimal schema from a list of (type_name, field_name, scalar) triples.
    pub fn schema_from_triples(
        &self,
        query_type: &str,
        triples: &[(&str, &str, &str)],
    ) -> GQLSchema {
        use std::collections::BTreeMap;
        let mut map: BTreeMap<String, Vec<GQLField>> = BTreeMap::new();
        for (type_name, field_name, scalar) in triples {
            map.entry(type_name.to_string())
                .or_default()
                .push(GQLField {
                    name: field_name.to_string(),
                    ty: GQLType::Scalar(scalar.to_string()),
                    nullable: true,
                    description: None,
                    args: vec![],
                });
        }
        let types = map
            .into_iter()
            .map(|(name, fields)| GQLObject {
                name,
                fields,
                implements: vec![],
                description: None,
            })
            .collect();
        GQLSchema {
            types,
            query_type: query_type.to_string(),
            mutation_type: None,
        }
    }
}
#[allow(dead_code)]
pub struct GQLMockDataGenerator {
    pub(super) seed: u64,
}
impl GQLMockDataGenerator {
    #[allow(dead_code)]
    pub fn new(seed: u64) -> Self {
        GQLMockDataGenerator { seed }
    }
    #[allow(dead_code)]
    pub fn generate_for_type(&mut self, ty: &GQLType) -> String {
        match ty {
            GQLType::Scalar(s) if s == "String" || s == "ID" => {
                format!("\"mock_string_{}\"", self.seed)
            }
            GQLType::Scalar(s) if s == "Int" => {
                self.seed += 1;
                format!("{}", self.seed % 1000)
            }
            GQLType::Scalar(s) if s == "Float" => {
                format!("{:.2}", (self.seed as f64) * 0.1)
            }
            GQLType::Scalar(s) if s == "Boolean" => {
                if self.seed % 2 == 0 {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            GQLType::List(inner) => format!("[{}]", self.generate_for_type(inner)),
            GQLType::NonNull(inner) => self.generate_for_type(inner),
            _ => "null".to_string(),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum GQLCacheScope {
    Public,
    Private,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GQLScalar {
    pub name: String,
    pub description: Option<String>,
    pub directives: Vec<String>,
}
impl GQLScalar {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        GQLScalar {
            name: name.into(),
            description: None,
            directives: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let mut out = String::new();
        if let Some(ref d) = self.description {
            out.push_str(&format!("\"\"\"{}\"\"\"\n", d));
        }
        out.push_str(&format!("scalar {}", self.name));
        if !self.directives.is_empty() {
            out.push(' ');
            out.push_str(
                &self
                    .directives
                    .iter()
                    .map(|d| format!("@{}", d))
                    .collect::<Vec<_>>()
                    .join(" "),
            );
        }
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GQLDeferDirective {
    pub label: Option<String>,
    pub if_condition: Option<String>,
}
impl GQLDeferDirective {
    #[allow(dead_code)]
    pub fn new() -> Self {
        GQLDeferDirective {
            label: None,
            if_condition: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let mut parts = vec!["@defer".to_string()];
        let mut args = Vec::new();
        if let Some(ref label) = self.label {
            args.push(format!("label: \"{}\"", label));
        }
        if let Some(ref cond) = self.if_condition {
            args.push(format!("if: {}", cond));
        }
        if !args.is_empty() {
            parts.push(format!("({})", args.join(", ")));
        }
        parts.join("")
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GQLFragment {
    pub name: String,
    pub on_type: String,
    pub selections: Vec<GQLSelectionField>,
    pub directives: Vec<String>,
}
impl GQLFragment {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, on_type: impl Into<String>) -> Self {
        GQLFragment {
            name: name.into(),
            on_type: on_type.into(),
            selections: Vec::new(),
            directives: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let mut out = format!("fragment {} on {} {{\n", self.name, self.on_type);
        for sel in &self.selections {
            out.push_str(&sel.emit(1));
            out.push('\n');
        }
        out.push('}');
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GQLResponse {
    pub data: Option<String>,
    pub errors: Vec<GQLError>,
}
impl GQLResponse {
    #[allow(dead_code)]
    pub fn success(data: impl Into<String>) -> Self {
        GQLResponse {
            data: Some(data.into()),
            errors: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn error(err: GQLError) -> Self {
        GQLResponse {
            data: None,
            errors: vec![err],
        }
    }
    #[allow(dead_code)]
    pub fn is_success(&self) -> bool {
        self.errors.is_empty()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GQLPaginationArgs {
    pub first: Option<u32>,
    pub last: Option<u32>,
    pub after: Option<String>,
    pub before: Option<String>,
}
#[allow(dead_code)]
pub struct GQLValidator {
    pub(super) schema: GQLSchemaExtended,
    pub(super) errors: Vec<String>,
}
impl GQLValidator {
    #[allow(dead_code)]
    pub fn new(schema: GQLSchemaExtended) -> Self {
        GQLValidator {
            schema,
            errors: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn validate_object(&mut self, obj: &GQLObject) {
        if obj.name.is_empty() {
            self.errors.push("Object type must have a name".to_string());
        }
        if obj.fields.is_empty() {
            self.errors.push(format!(
                "Object type '{}' must have at least one field",
                obj.name
            ));
        }
        for field in &obj.fields {
            self.validate_field_name(&field.name);
        }
    }
    #[allow(dead_code)]
    pub fn validate_field_name(&mut self, name: &str) {
        if name.starts_with("__") {
            self.errors
                .push(format!("Field name '{}' must not begin with '__'", name));
        }
    }
    #[allow(dead_code)]
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }
    #[allow(dead_code)]
    pub fn errors(&self) -> &[String] {
        &self.errors
    }
}
#[allow(dead_code)]
pub struct GQLFederationExtension {
    pub service_name: String,
    pub keys: Vec<String>,
    pub externals: Vec<String>,
    pub requires: Vec<String>,
    pub provides: Vec<String>,
}
impl GQLFederationExtension {
    #[allow(dead_code)]
    pub fn new(service_name: impl Into<String>) -> Self {
        GQLFederationExtension {
            service_name: service_name.into(),
            keys: Vec::new(),
            externals: Vec::new(),
            requires: Vec::new(),
            provides: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add_key(&mut self, key: impl Into<String>) {
        self.keys.push(key.into());
    }
    #[allow(dead_code)]
    pub fn emit_key_directives(&self) -> String {
        self.keys
            .iter()
            .map(|k| format!("@key(fields: \"{}\")", k))
            .collect::<Vec<_>>()
            .join(" ")
    }
    #[allow(dead_code)]
    pub fn emit_service_query() -> &'static str {
        "type Query { _service: _Service! } type _Service { sdl: String }"
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GQLSelectionField {
    pub alias: Option<String>,
    pub name: String,
    pub arguments: Vec<(String, String)>,
    pub directives: Vec<String>,
    pub selection_set: Vec<GQLSelectionField>,
}
impl GQLSelectionField {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        GQLSelectionField {
            alias: None,
            name: name.into(),
            arguments: Vec::new(),
            directives: Vec::new(),
            selection_set: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn emit(&self, indent: usize) -> String {
        let pad = "  ".repeat(indent);
        let mut out = String::new();
        out.push_str(&pad);
        if let Some(ref alias) = self.alias {
            out.push_str(&format!("{}: ", alias));
        }
        out.push_str(&self.name);
        if !self.arguments.is_empty() {
            out.push('(');
            let args: Vec<String> = self
                .arguments
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect();
            out.push_str(&args.join(", "));
            out.push(')');
        }
        if !self.selection_set.is_empty() {
            out.push_str(" {\n");
            for sel in &self.selection_set {
                out.push_str(&sel.emit(indent + 1));
                out.push('\n');
            }
            out.push_str(&pad);
            out.push('}');
        }
        out
    }
}
