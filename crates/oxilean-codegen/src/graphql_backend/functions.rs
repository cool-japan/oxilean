//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    GQLBackend, GQLBatchRequest, GQLCacheControl, GQLConnection, GQLDataloader, GQLDeferDirective,
    GQLDeprecation, GQLDirective, GQLDirectiveLocation, GQLEnumDef, GQLEnumValue, GQLError,
    GQLFederationExtension, GQLField, GQLFragment, GQLInputField, GQLInputObject, GQLInterface,
    GQLIntrospectionQuery, GQLLiveQueryExtension, GQLMockDataGenerator, GQLObject, GQLOperation,
    GQLPersistedQuery, GQLPersistedQueryStore, GQLQueryComplexity, GQLRateLimitDirective,
    GQLResolverSignature, GQLResponse, GQLScalar, GQLSchema, GQLSchemaBuilder, GQLSchemaComparator,
    GQLSchemaExtended, GQLSchemaRegistry, GQLSdlPrinter, GQLSelectionField, GQLStreamDirective,
    GQLType, GQLTypeNameMap, GQLTypeSystemDocument, GQLUnion,
};

#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn backend() -> GQLBackend {
        GQLBackend
    }
    pub(super) fn simple_object() -> GQLObject {
        GQLObject {
            name: "User".to_string(),
            fields: vec![
                GQLField {
                    name: "id".to_string(),
                    ty: GQLType::NonNull(Box::new(GQLType::Scalar("ID".to_string()))),
                    nullable: false,
                    description: None,
                    args: vec![],
                },
                GQLField {
                    name: "name".to_string(),
                    ty: GQLType::Scalar("String".to_string()),
                    nullable: true,
                    description: Some("The user's display name.".to_string()),
                    args: vec![],
                },
            ],
            implements: vec![],
            description: None,
        }
    }
    #[test]
    pub(super) fn test_emit_scalar_type() {
        let b = backend();
        assert_eq!(b.emit_type(&GQLType::Scalar("Int".to_string())), "Int");
        assert_eq!(
            b.emit_type(&GQLType::Scalar("String".to_string())),
            "String"
        );
    }
    #[test]
    pub(super) fn test_emit_list_type() {
        let b = backend();
        let ty = GQLType::List(Box::new(GQLType::Scalar("String".to_string())));
        assert_eq!(b.emit_type(&ty), "[String]");
    }
    #[test]
    pub(super) fn test_emit_nonnull_type() {
        let b = backend();
        let ty = GQLType::NonNull(Box::new(GQLType::Scalar("ID".to_string())));
        assert_eq!(b.emit_type(&ty), "ID!");
    }
    #[test]
    pub(super) fn test_emit_nonnull_list_type() {
        let b = backend();
        let ty = GQLType::NonNull(Box::new(GQLType::List(Box::new(GQLType::NonNull(
            Box::new(GQLType::Scalar("Int".to_string())),
        )))));
        assert_eq!(b.emit_type(&ty), "[Int!]!");
    }
    #[test]
    pub(super) fn test_emit_field_nonnull() {
        let b = backend();
        let field = GQLField {
            name: "id".to_string(),
            ty: GQLType::NonNull(Box::new(GQLType::Scalar("ID".to_string()))),
            nullable: false,
            description: None,
            args: vec![],
        };
        let out = b.emit_field(&field);
        assert!(out.contains("id: ID!"), "got: {}", out);
    }
    #[test]
    pub(super) fn test_emit_field_nullable_with_description() {
        let b = backend();
        let field = GQLField {
            name: "bio".to_string(),
            ty: GQLType::Scalar("String".to_string()),
            nullable: true,
            description: Some("Short biography.".to_string()),
            args: vec![],
        };
        let out = b.emit_field(&field);
        assert!(out.contains("\"\"\"Short biography.\"\"\""));
        assert!(out.contains("bio: String"));
    }
    #[test]
    pub(super) fn test_emit_object_no_implements() {
        let b = backend();
        let obj = simple_object();
        let out = b.emit_object(&obj);
        assert!(out.starts_with("type User {"));
        assert!(out.contains("id: ID!"));
        assert!(out.contains("name: String"));
        assert!(out.ends_with('}'));
    }
    #[test]
    pub(super) fn test_emit_object_with_implements() {
        let b = backend();
        let obj = GQLObject {
            name: "Admin".to_string(),
            fields: vec![GQLField {
                name: "role".to_string(),
                ty: GQLType::Scalar("String".to_string()),
                nullable: true,
                description: None,
                args: vec![],
            }],
            implements: vec!["Node".to_string(), "Actor".to_string()],
            description: None,
        };
        let out = b.emit_object(&obj);
        assert!(out.contains("type Admin implements Node & Actor {"));
    }
    #[test]
    pub(super) fn test_emit_enum() {
        let b = backend();
        let e = GQLEnumDef {
            name: "Status".to_string(),
            values: vec![
                GQLEnumValue {
                    name: "ACTIVE".to_string(),
                    description: None,
                },
                GQLEnumValue {
                    name: "INACTIVE".to_string(),
                    description: Some("Deactivated account.".to_string()),
                },
            ],
        };
        let out = b.emit_enum(&e);
        assert!(out.starts_with("enum Status {"));
        assert!(out.contains("ACTIVE"));
        assert!(out.contains("INACTIVE"));
        assert!(out.contains("\"\"\"Deactivated account.\"\"\""));
    }
    #[test]
    pub(super) fn test_emit_schema_no_mutation() {
        let b = backend();
        let schema = GQLSchema {
            types: vec![simple_object()],
            query_type: "Query".to_string(),
            mutation_type: None,
        };
        let out = b.emit_schema(&schema);
        assert!(out.contains("schema {"));
        assert!(out.contains("query: Query"));
        assert!(!out.contains("mutation:"));
        assert!(out.contains("type User {"));
    }
    #[test]
    pub(super) fn test_emit_schema_with_mutation() {
        let b = backend();
        let schema = GQLSchema {
            types: vec![simple_object()],
            query_type: "Query".to_string(),
            mutation_type: Some("Mutation".to_string()),
        };
        let out = b.emit_schema(&schema);
        assert!(out.contains("mutation: Mutation"));
    }
    #[test]
    pub(super) fn test_generate_resolver_stubs() {
        let b = backend();
        let schema = GQLSchema {
            types: vec![simple_object()],
            query_type: "Query".to_string(),
            mutation_type: None,
        };
        let out = b.generate_resolver_stubs(&schema);
        assert!(out.contains("fn resolve_user_id"));
        assert!(out.contains("fn resolve_user_name"));
        assert!(out.contains("todo!()"));
    }
    #[test]
    pub(super) fn test_schema_from_triples() {
        let b = backend();
        let schema = b.schema_from_triples(
            "Query",
            &[("Query", "users", "String"), ("Query", "version", "String")],
        );
        assert_eq!(schema.query_type, "Query");
        assert_eq!(schema.types.len(), 1);
        assert_eq!(schema.types[0].name, "Query");
        assert_eq!(schema.types[0].fields.len(), 2);
    }
    #[test]
    pub(super) fn test_interface_and_union_type_emit() {
        let b = backend();
        let iface = GQLType::Interface("Node".to_string());
        let union_ty = GQLType::Union("SearchResult".to_string());
        assert_eq!(b.emit_type(&iface), "Node");
        assert_eq!(b.emit_type(&union_ty), "SearchResult");
    }
}
#[allow(dead_code)]
pub(super) fn emit_gql_type(ty: &GQLType) -> String {
    match ty {
        GQLType::Scalar(s) => s.clone(),
        GQLType::Object(o) => o.clone(),
        GQLType::Interface(i) => i.clone(),
        GQLType::Union(u) => u.clone(),
        GQLType::Enum(e) => e.clone(),
        GQLType::List(inner) => format!("[{}]", emit_gql_type(inner)),
        GQLType::NonNull(inner) => format!("{}!", emit_gql_type(inner)),
    }
}
#[allow(dead_code)]
pub(super) fn ts_type(ty: &GQLType) -> &str {
    match ty {
        GQLType::Scalar(s) if s == "String" => "string",
        GQLType::Scalar(s) if s == "Int" || s == "Float" => "number",
        GQLType::Scalar(s) if s == "Boolean" => "boolean",
        GQLType::Scalar(s) if s == "ID" => "string",
        GQLType::NonNull(_inner) => "any",
        GQLType::List(_inner) => "any[]",
        _ => "any",
    }
}
#[allow(dead_code)]
pub(super) fn rust_type(ty: &GQLType) -> &str {
    match ty {
        GQLType::Scalar(s) if s == "String" || s == "ID" => "String",
        GQLType::Scalar(s) if s == "Int" => "i64",
        GQLType::Scalar(s) if s == "Float" => "f64",
        GQLType::Scalar(s) if s == "Boolean" => "bool",
        GQLType::NonNull(_) => "Box<dyn std::any::Any>",
        _ => "serde_json::Value",
    }
}
#[allow(dead_code)]
pub(super) fn go_type(ty: &GQLType) -> &str {
    match ty {
        GQLType::Scalar(s) if s == "String" || s == "ID" => "string",
        GQLType::Scalar(s) if s == "Int" => "int64",
        GQLType::Scalar(s) if s == "Float" => "float64",
        GQLType::Scalar(s) if s == "Boolean" => "bool",
        _ => "interface{}",
    }
}
#[allow(dead_code)]
pub(super) fn py_type(ty: &GQLType) -> &str {
    match ty {
        GQLType::Scalar(s) if s == "String" || s == "ID" => "str",
        GQLType::Scalar(s) if s == "Int" => "int",
        GQLType::Scalar(s) if s == "Float" => "float",
        GQLType::Scalar(s) if s == "Boolean" => "bool",
        GQLType::List(_) => "list",
        _ => "object",
    }
}
#[cfg(test)]
mod graphql_extended_tests {
    use super::*;
    #[test]
    pub(super) fn test_directive_emit() {
        let d = GQLDirective::new("auth")
            .with_location(GQLDirectiveLocation::Field)
            .with_location(GQLDirectiveLocation::Object);
        let emitted = d.emit();
        assert!(emitted.contains("directive @auth"));
        assert!(emitted.contains("FIELD"));
    }
    #[test]
    pub(super) fn test_union_emit() {
        let mut u = GQLUnion::new("SearchResult");
        u.add_member("User");
        u.add_member("Post");
        let s = u.emit();
        assert!(s.contains("union SearchResult = User | Post"));
    }
    #[test]
    pub(super) fn test_connection_type() {
        let conn = GQLConnection::new("User");
        let edge = conn.emit_edge_type();
        let connection = conn.emit_connection_type();
        assert!(edge.contains("type UserEdge"));
        assert!(connection.contains("type UserConnection"));
        assert!(connection.contains("pageInfo: PageInfo!"));
    }
    #[test]
    pub(super) fn test_schema_emit() {
        let mut schema = GQLSchemaExtended::new();
        schema.set_query_type("Query");
        let emitted = schema.emit();
        assert!(emitted.contains("schema {"));
        assert!(emitted.contains("query: Query"));
    }
    #[test]
    pub(super) fn test_complexity_calculation() {
        let limits = GQLQueryComplexity::default_limits();
        let selections = vec![
            GQLSelectionField::new("user"),
            GQLSelectionField::new("posts"),
        ];
        let complexity = limits.calculate_selection_complexity(&selections, 0);
        assert!(complexity >= 2);
    }
    #[test]
    pub(super) fn test_scalar_emit() {
        let scalar = GQLScalar::new("DateTime");
        let s = scalar.emit();
        assert_eq!(s, "scalar DateTime");
    }
    #[test]
    pub(super) fn test_input_object() {
        let mut inp = GQLInputObject::new("CreateUserInput");
        inp.add_field(GQLInputField {
            name: "email".to_string(),
            ty: GQLType::NonNull(Box::new(GQLType::Scalar("String".to_string()))),
            default_value: None,
            description: None,
            directives: Vec::new(),
        });
        let s = inp.emit();
        assert!(s.contains("input CreateUserInput"));
        assert!(s.contains("email:"));
    }
    #[test]
    pub(super) fn test_operation_emit() {
        let op = GQLOperation::query("GetUser");
        let s = op.emit();
        assert!(s.contains("query GetUser"));
    }
    #[test]
    pub(super) fn test_registry() {
        let mut registry = GQLSchemaRegistry::new();
        registry.register("v1", GQLSchemaExtended::new());
        assert!(registry.get("v1").is_some());
        assert_eq!(registry.list_names().len(), 1);
    }
    #[test]
    pub(super) fn test_deprecation() {
        let dep = GQLDeprecation::new("oldField", "Use newField instead");
        let d = dep.emit_directive();
        assert!(d.contains("@deprecated"));
        assert!(d.contains("Use newField instead"));
    }
    #[test]
    pub(super) fn test_mock_generator() {
        let mut gen = GQLMockDataGenerator::new(42);
        let s = gen.generate_for_type(&GQLType::Scalar("String".to_string()));
        assert!(s.contains("mock_string"));
        let i = gen.generate_for_type(&GQLType::Scalar("Int".to_string()));
        assert!(!i.is_empty());
    }
}
#[cfg(test)]
mod graphql_introspection_tests {
    use super::*;
    #[test]
    pub(super) fn test_schema_builder() {
        let schema = GQLSchemaBuilder::new()
            .scalar(GQLScalar::new("DateTime"))
            .query_type("Query")
            .build();
        assert_eq!(schema.scalars.len(), 1);
        assert_eq!(schema.query_type, Some("Query".to_string()));
    }
    #[test]
    pub(super) fn test_error_json() {
        let err = GQLError::new("Not found").with_location(1, 5);
        let json = err.emit_json();
        assert!(json.contains("Not found"));
        assert!(json.contains("\"line\":1"));
    }
    #[test]
    pub(super) fn test_response_success() {
        let r = GQLResponse::success("{\"user\":{\"id\":\"1\"}}");
        assert!(r.is_success());
    }
    #[test]
    pub(super) fn test_cache_control() {
        let cc = GQLCacheControl::public(300);
        let d = cc.emit_directive();
        assert!(d.contains("maxAge: 300"));
        assert!(d.contains("PUBLIC"));
    }
    #[test]
    pub(super) fn test_federation_extension() {
        let mut fed = GQLFederationExtension::new("products");
        fed.add_key("id");
        let dirs = fed.emit_key_directives();
        assert!(dirs.contains("@key(fields: \"id\")"));
    }
    #[test]
    pub(super) fn test_batch_request() {
        let mut batch = GQLBatchRequest::new(3);
        assert!(batch.add(GQLOperation::query("Q1")));
        assert!(batch.add(GQLOperation::query("Q2")));
        assert!(batch.add(GQLOperation::query("Q3")));
        assert!(!batch.add(GQLOperation::query("Q4")));
        assert_eq!(batch.operations.len(), 3);
    }
    #[test]
    pub(super) fn test_dataloader_ts_emit() {
        let loader = GQLDataloader::new("User");
        let ts = loader.emit_ts_loader();
        assert!(ts.contains("DataLoader"));
        assert!(ts.contains("maxBatchSize: 100"));
    }
    #[test]
    pub(super) fn test_rate_limit_directive() {
        let rl = GQLRateLimitDirective::new(100, 60);
        let emitted = rl.emit();
        assert!(emitted.contains("@rateLimit"));
        assert!(emitted.contains("limit: 100"));
    }
    #[test]
    pub(super) fn test_type_system_document() {
        let mut doc = GQLTypeSystemDocument::new();
        doc.extend_type("Query", &["health: Boolean!"]);
        let emitted = doc.emit();
        assert!(emitted.contains("extend type Query"));
        assert!(emitted.contains("health: Boolean!"));
    }
    #[test]
    pub(super) fn test_introspection_query() {
        let q = GQLIntrospectionQuery::full_introspection_query();
        assert!(q.contains("__schema"));
    }
    #[test]
    pub(super) fn test_live_query_extension() {
        let mut lq = GQLLiveQueryExtension::new(500);
        lq.add_key("user:1");
        assert_eq!(lq.invalidation_keys.len(), 1);
        let header = lq.emit_extension_header();
        assert!(header.contains("@live"));
    }
    #[test]
    pub(super) fn test_resolver_signature() {
        let sig = GQLResolverSignature::new(
            "User",
            "posts",
            GQLType::List(Box::new(GQLType::Object("Post".to_string()))),
        );
        let ts = sig.emit_ts_signature();
        assert!(ts.contains("posts"));
        assert!(ts.contains("async"));
    }
    #[test]
    pub(super) fn test_interface_emit() {
        let mut iface = GQLInterface::new("Node");
        iface.add_field(GQLField {
            name: "id".to_string(),
            ty: GQLType::NonNull(Box::new(GQLType::Scalar("ID".to_string()))),
            nullable: false,
            description: None,
            args: Vec::new(),
        });
        let s = iface.emit();
        assert!(s.contains("interface Node"));
        assert!(s.contains("id:"));
    }
    #[test]
    pub(super) fn test_fragment_emit() {
        let frag = GQLFragment::new("UserFields", "User");
        let s = frag.emit();
        assert!(s.contains("fragment UserFields on User"));
    }
}
#[cfg(test)]
mod graphql_advanced_tests {
    use super::*;
    #[test]
    pub(super) fn test_persisted_query() {
        let pq = GQLPersistedQuery::new("{ user { id name } }");
        assert!(!pq.hash.is_empty());
        let ext = pq.emit_apq_extension();
        assert!(ext.contains("persistedQuery"));
        assert!(ext.contains("sha256Hash"));
    }
    #[test]
    pub(super) fn test_persisted_query_store() {
        let mut store = GQLPersistedQueryStore::new();
        let pq = GQLPersistedQuery::new("{ user { id } }");
        let hash = pq.hash.clone();
        store.register(pq);
        assert_eq!(store.count(), 1);
        assert!(store.lookup(&hash).is_some());
    }
    #[test]
    pub(super) fn test_defer_directive() {
        let defer = GQLDeferDirective::new().with_label("profile");
        let s = defer.emit();
        assert!(s.contains("@defer"));
        assert!(s.contains("label: \"profile\""));
    }
    #[test]
    pub(super) fn test_stream_directive() {
        let stream = GQLStreamDirective::new(2);
        let s = stream.emit();
        assert!(s.contains("@stream"));
        assert!(s.contains("initialCount: 2"));
    }
    #[test]
    pub(super) fn test_schema_comparator_added() {
        let mut old = GQLSchemaExtended::new();
        let mut new_schema = GQLSchemaExtended::new();
        new_schema.objects.push(GQLObject {
            name: "User".to_string(),
            fields: Vec::new(),
            description: None,
            implements: Vec::new(),
        });
        let cmp = GQLSchemaComparator::new(old, new_schema);
        assert_eq!(cmp.added_types(), vec!["User".to_string()]);
        assert!(!cmp.is_breaking_change());
    }
    #[test]
    pub(super) fn test_schema_comparator_removed() {
        let mut old = GQLSchemaExtended::new();
        old.objects.push(GQLObject {
            name: "User".to_string(),
            fields: Vec::new(),
            description: None,
            implements: Vec::new(),
        });
        let new_schema = GQLSchemaExtended::new();
        let cmp = GQLSchemaComparator::new(old, new_schema);
        assert_eq!(cmp.removed_types(), vec!["User".to_string()]);
        assert!(cmp.is_breaking_change());
    }
    #[test]
    pub(super) fn test_schema_comparator_changelog() {
        let mut old = GQLSchemaExtended::new();
        let mut new_schema = GQLSchemaExtended::new();
        old.objects.push(GQLObject {
            name: "Deleted".to_string(),
            fields: Vec::new(),
            description: None,
            implements: Vec::new(),
        });
        new_schema.objects.push(GQLObject {
            name: "Added".to_string(),
            fields: Vec::new(),
            description: None,
            implements: Vec::new(),
        });
        let cmp = GQLSchemaComparator::new(old, new_schema);
        let log = cmp.generate_changelog();
        assert!(log.contains("+ Added type: Added"));
        assert!(log.contains("- Removed type: Deleted"));
    }
}
#[cfg(test)]
mod graphql_sdl_tests {
    use super::*;
    #[test]
    pub(super) fn test_sdl_printer_object() {
        let obj = GQLObject {
            name: "User".to_string(),
            fields: vec![GQLField {
                name: "id".to_string(),
                ty: GQLType::Scalar("ID".to_string()),
                nullable: false,
                description: None,
                args: Vec::new(),
            }],
            description: None,
            implements: Vec::new(),
        };
        let printer = GQLSdlPrinter::new();
        let s = printer.print_object(&obj);
        assert!(s.contains("type User {"));
        assert!(s.contains("id:"));
    }
    #[test]
    pub(super) fn test_type_name_map() {
        let map = GQLTypeNameMap::new();
        assert_eq!(map.lookup("Int"), Some(&"i32".to_string()));
        assert_eq!(map.lookup("Boolean"), Some(&"bool".to_string()));
        assert!(map.lookup("Unknown").is_none());
    }
    #[test]
    pub(super) fn test_sdl_printer_schema() {
        let mut schema = GQLSchemaExtended::new();
        schema.scalars.push(GQLScalar::new("Date"));
        schema.objects.push(GQLObject {
            name: "Post".to_string(),
            fields: vec![GQLField {
                name: "title".to_string(),
                ty: GQLType::Scalar("String".to_string()),
                nullable: true,
                description: None,
                args: Vec::new(),
            }],
            description: None,
            implements: Vec::new(),
        });
        let printer = GQLSdlPrinter::new();
        let s = printer.print_schema(&schema);
        assert!(s.contains("scalar Date"));
        assert!(s.contains("type Post"));
    }
}
#[allow(dead_code)]
pub fn gql_type_name(ty: &GQLType) -> String {
    match ty {
        GQLType::Scalar(s)
        | GQLType::Object(s)
        | GQLType::Interface(s)
        | GQLType::Union(s)
        | GQLType::Enum(s) => s.clone(),
        GQLType::List(inner) | GQLType::NonNull(inner) => gql_type_name(inner),
    }
}
#[allow(dead_code)]
pub fn is_builtin_scalar(name: &str) -> bool {
    matches!(name, "Int" | "Float" | "String" | "Boolean" | "ID")
}
