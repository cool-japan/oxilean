//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    SQLAlterTableBuilder, SQLAnalyzeBuilder, SQLBackend, SQLColumn, SQLColumnDef, SQLColumnInfo,
    SQLCommonTableExpression, SQLCreateTableBuilder, SQLDeleteBuilder, SQLDialect, SQLExpr,
    SQLIndexBuilder, SQLInsertBuilder, SQLIsolationLevel, SQLJoin, SQLMigration,
    SQLMigrationRunner, SQLQueryFormatter, SQLQueryOptimizer, SQLQueryPlan, SQLQueryPlanNode,
    SQLSchemaInspector, SQLSelectBuilder, SQLSequenceBuilder, SQLStmt, SQLStoredProcedure,
    SQLTable, SQLTableInfo, SQLTransactionBuilder, SQLTrigger, SQLType, SQLTypeMapper,
    SQLUpdateBuilder, SQLViewBuilder, SQLWindowFunction, SQLWithQuery,
};

#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn sqlite() -> SQLBackend {
        SQLBackend::new(SQLDialect::SQLite)
    }
    pub(super) fn pg() -> SQLBackend {
        SQLBackend::new(SQLDialect::PostgreSQL)
    }
    pub(super) fn mysql() -> SQLBackend {
        SQLBackend::new(SQLDialect::MySQL)
    }
    pub(super) fn mssql() -> SQLBackend {
        SQLBackend::new(SQLDialect::MSSQL)
    }
    #[test]
    pub(super) fn test_emit_type_sqlite() {
        let b = sqlite();
        assert_eq!(b.emit_type(&SQLType::Integer), "INTEGER");
        assert_eq!(b.emit_type(&SQLType::Real), "REAL");
        assert_eq!(b.emit_type(&SQLType::Text), "TEXT");
        assert_eq!(b.emit_type(&SQLType::Blob), "BLOB");
        assert_eq!(b.emit_type(&SQLType::Boolean), "INTEGER");
        assert_eq!(b.emit_type(&SQLType::Timestamp), "TEXT");
    }
    #[test]
    pub(super) fn test_emit_type_postgresql() {
        let b = pg();
        assert_eq!(b.emit_type(&SQLType::Real), "DOUBLE PRECISION");
        assert_eq!(b.emit_type(&SQLType::Blob), "BYTEA");
        assert_eq!(b.emit_type(&SQLType::Boolean), "BOOLEAN");
        assert_eq!(b.emit_type(&SQLType::Timestamp), "TIMESTAMP");
    }
    #[test]
    pub(super) fn test_emit_type_mysql() {
        let b = mysql();
        assert_eq!(b.emit_type(&SQLType::Integer), "INT");
        assert_eq!(b.emit_type(&SQLType::Real), "DOUBLE");
        assert_eq!(b.emit_type(&SQLType::Boolean), "TINYINT(1)");
        assert_eq!(b.emit_type(&SQLType::Timestamp), "DATETIME");
    }
    #[test]
    pub(super) fn test_emit_type_mssql() {
        let b = mssql();
        assert_eq!(b.emit_type(&SQLType::Integer), "INT");
        assert_eq!(b.emit_type(&SQLType::Real), "FLOAT");
        assert_eq!(b.emit_type(&SQLType::Text), "NVARCHAR(MAX)");
        assert_eq!(b.emit_type(&SQLType::Blob), "VARBINARY(MAX)");
        assert_eq!(b.emit_type(&SQLType::Boolean), "BIT");
        assert_eq!(b.emit_type(&SQLType::Timestamp), "DATETIME2");
    }
    #[test]
    pub(super) fn test_select_all() {
        let b = sqlite();
        assert_eq!(b.select_all("users"), "SELECT * FROM users;");
    }
    #[test]
    pub(super) fn test_select_with_limit_sqlite() {
        let b = sqlite();
        assert_eq!(b.select_limit("users", 10), "SELECT * FROM users LIMIT 10;");
    }
    #[test]
    pub(super) fn test_select_with_limit_mssql() {
        let b = mssql();
        assert_eq!(b.select_limit("users", 5), "SELECT TOP 5 * FROM users;");
    }
    #[test]
    pub(super) fn test_select_with_where() {
        let b = pg();
        let stmt = SQLStmt::Select {
            cols: vec!["id".to_string(), "name".to_string()],
            from: "users".to_string(),
            where_: Some(SQLExpr::BinOp(
                Box::new(SQLExpr::Column("id".to_string())),
                "=".to_string(),
                Box::new(SQLExpr::Literal("42".to_string())),
            )),
            limit: None,
        };
        let out = b.emit_stmt(&stmt);
        assert!(out.contains("WHERE (id = 42)"));
        assert!(out.contains("SELECT id, name FROM users"));
    }
    #[test]
    pub(super) fn test_insert_stmt() {
        let b = sqlite();
        let stmt = SQLStmt::Insert {
            table: "users".to_string(),
            values: vec![
                SQLExpr::Literal("1".to_string()),
                SQLExpr::Literal("'Alice'".to_string()),
            ],
        };
        assert_eq!(b.emit_stmt(&stmt), "INSERT INTO users VALUES (1, 'Alice');");
    }
    #[test]
    pub(super) fn test_update_stmt() {
        let b = mysql();
        let stmt = SQLStmt::Update {
            table: "users".to_string(),
            set_col: "name".to_string(),
            set_val: SQLExpr::Literal("'Bob'".to_string()),
            where_: Some(SQLExpr::BinOp(
                Box::new(SQLExpr::Column("id".to_string())),
                "=".to_string(),
                Box::new(SQLExpr::Literal("1".to_string())),
            )),
        };
        let out = b.emit_stmt(&stmt);
        assert!(out.contains("UPDATE users SET name = 'Bob'"));
        assert!(out.contains("WHERE (id = 1)"));
    }
    #[test]
    pub(super) fn test_delete_stmt() {
        let b = sqlite();
        let stmt = SQLStmt::Delete {
            table: "logs".to_string(),
            where_: Some(SQLExpr::BinOp(
                Box::new(SQLExpr::Column("age".to_string())),
                ">".to_string(),
                Box::new(SQLExpr::Literal("30".to_string())),
            )),
        };
        let out = b.emit_stmt(&stmt);
        assert_eq!(out, "DELETE FROM logs WHERE (age > 30);");
    }
    #[test]
    pub(super) fn test_drop_table() {
        let b = pg();
        assert_eq!(
            b.emit_stmt(&SQLStmt::DropTable("old_table".to_string())),
            "DROP TABLE IF EXISTS old_table;"
        );
    }
    #[test]
    pub(super) fn test_create_table_stmt() {
        let b = sqlite();
        let table = SQLTable {
            name: "items".to_string(),
            columns: vec![
                SQLColumn {
                    name: "id".to_string(),
                    ty: SQLType::Integer,
                    not_null: true,
                    primary_key: true,
                },
                SQLColumn {
                    name: "label".to_string(),
                    ty: SQLType::Text,
                    not_null: true,
                    primary_key: false,
                },
            ],
        };
        let out = b.create_table_stmt(&table);
        assert!(out.contains("CREATE TABLE IF NOT EXISTS items"));
        assert!(out.contains("id INTEGER PRIMARY KEY NOT NULL"));
        assert!(out.contains("label TEXT NOT NULL"));
    }
    #[test]
    pub(super) fn test_schema_for_type() {
        let b = pg();
        let schema = b.schema_for_type("User");
        assert_eq!(schema.name, "user");
        assert_eq!(schema.columns.len(), 3);
        assert_eq!(schema.columns[0].name, "id");
        assert!(schema.columns[0].primary_key);
    }
    #[test]
    pub(super) fn test_func_call_expr() {
        let b = sqlite();
        let expr = SQLExpr::FuncCall("COUNT".to_string(), vec![SQLExpr::Column("*".to_string())]);
        assert_eq!(b.emit_expr(&expr), "COUNT(*)");
    }
    #[test]
    pub(super) fn test_insert_placeholders_sqlite() {
        let b = sqlite();
        let out = b.insert_placeholders("events", 3);
        assert_eq!(out, "INSERT INTO events VALUES (?, ?, ?);");
    }
    #[test]
    pub(super) fn test_insert_placeholders_postgresql() {
        let b = pg();
        let out = b.insert_placeholders("events", 3);
        assert_eq!(out, "INSERT INTO events VALUES ($1, $2, $3);");
    }
    #[test]
    pub(super) fn test_nested_binop_expr() {
        let b = sqlite();
        let expr = SQLExpr::BinOp(
            Box::new(SQLExpr::BinOp(
                Box::new(SQLExpr::Column("a".to_string())),
                "+".to_string(),
                Box::new(SQLExpr::Literal("1".to_string())),
            )),
            "*".to_string(),
            Box::new(SQLExpr::Literal("2".to_string())),
        );
        assert_eq!(b.emit_expr(&expr), "((a + 1) * 2)");
    }
}
#[cfg(test)]
mod sql_extended_tests {
    use super::*;
    #[test]
    pub(super) fn test_select_builder() {
        let sql = SQLSelectBuilder::new()
            .column("id")
            .column("name")
            .from_table("users")
            .where_cond("age > 18")
            .order_asc("name")
            .limit(10)
            .build();
        assert!(sql.contains("SELECT id, name FROM users"));
        assert!(sql.contains("WHERE age > 18"));
        assert!(sql.contains("ORDER BY name ASC"));
        assert!(sql.contains("LIMIT 10"));
    }
    #[test]
    pub(super) fn test_insert_builder() {
        let sql = SQLInsertBuilder::new("users")
            .column("name")
            .column("email")
            .values(vec![
                "'Alice'".to_string(),
                "'alice@example.com'".to_string(),
            ])
            .build();
        assert!(sql.contains("INSERT INTO users"));
        assert!(sql.contains("(name, email)"));
        assert!(sql.contains("VALUES"));
    }
    #[test]
    pub(super) fn test_update_builder() {
        let sql = SQLUpdateBuilder::new("users")
            .set("email", "'new@example.com'")
            .where_cond("id = 1")
            .build();
        assert!(sql.contains("UPDATE users SET"));
        assert!(sql.contains("email = 'new@example.com'"));
        assert!(sql.contains("WHERE id = 1"));
    }
    #[test]
    pub(super) fn test_delete_builder() {
        let sql = SQLDeleteBuilder::new("sessions")
            .where_cond("expires_at < NOW()")
            .build();
        assert!(sql.contains("DELETE FROM sessions"));
        assert!(sql.contains("WHERE expires_at < NOW()"));
    }
    #[test]
    pub(super) fn test_create_table() {
        let sql = SQLCreateTableBuilder::new("orders")
            .if_not_exists()
            .column(
                SQLColumnDef::new("id", SQLType::Integer)
                    .primary_key()
                    .auto_increment(),
            )
            .column(SQLColumnDef::new("user_id", SQLType::Integer).not_null())
            .column(SQLColumnDef::new("total", SQLType::Real).not_null())
            .build(&SQLDialect::SQLite);
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS orders"));
        assert!(sql.contains("id INTEGER PRIMARY KEY"));
        assert!(sql.contains("user_id INTEGER NOT NULL"));
    }
    #[test]
    pub(super) fn test_index_builder() {
        let sql = SQLIndexBuilder::new("idx_email", "users")
            .on_column("email")
            .unique()
            .build();
        assert!(sql.contains("CREATE UNIQUE INDEX idx_email ON users (email)"));
    }
    #[test]
    pub(super) fn test_migration() {
        let m = SQLMigration::new(1, "Add users table")
            .up("CREATE TABLE users (id INTEGER PRIMARY KEY)")
            .down("DROP TABLE users");
        assert!(m.emit_up().contains("-- Migration v1"));
        assert!(m.emit_down().contains("-- Rollback v1"));
        assert!(m.emit_down().contains("DROP TABLE users"));
    }
    #[test]
    pub(super) fn test_migration_runner() {
        let mut runner = SQLMigrationRunner::new();
        runner.add_migration(SQLMigration::new(1, "Init").up("CREATE TABLE t1 (id INT)"));
        runner.add_migration(
            SQLMigration::new(2, "Add col").up("ALTER TABLE t1 ADD COLUMN name TEXT"),
        );
        assert_eq!(runner.pending_migrations().len(), 2);
        let sql = runner.emit_pending_sql();
        assert!(sql.contains("Migration v1"));
        assert!(sql.contains("Migration v2"));
    }
    #[test]
    pub(super) fn test_view_builder() {
        let select = SQLSelectBuilder::new().column("*").from_table("users");
        let view = SQLViewBuilder::new("active_users", select)
            .or_replace()
            .build();
        assert!(view.contains("CREATE OR REPLACE VIEW active_users AS SELECT"));
    }
    #[test]
    pub(super) fn test_window_function() {
        let wf = SQLWindowFunction::new("ROW_NUMBER()")
            .partition_by("department")
            .order_asc("salary");
        let s = wf.emit();
        assert!(s.contains("ROW_NUMBER()"));
        assert!(s.contains("PARTITION BY department"));
        assert!(s.contains("ORDER BY salary ASC"));
    }
    #[test]
    pub(super) fn test_with_query() {
        let inner = SQLSelectBuilder::new().column("id").from_table("employees");
        let cte = SQLCommonTableExpression::new("emp_cte", inner);
        let outer = SQLSelectBuilder::new().column("*").from_table("emp_cte");
        let with_q = SQLWithQuery::new(outer).with(cte).build();
        assert!(with_q.contains("WITH emp_cte AS"));
        assert!(with_q.contains("FROM emp_cte"));
    }
    #[test]
    pub(super) fn test_transaction() {
        let tx = SQLTransactionBuilder::new()
            .isolation(SQLIsolationLevel::Serializable)
            .add_statement("UPDATE accounts SET balance = balance - 100 WHERE id = 1")
            .add_statement("UPDATE accounts SET balance = balance + 100 WHERE id = 2")
            .build();
        assert!(tx.contains("SERIALIZABLE"));
        assert!(tx.contains("BEGIN;"));
        assert!(tx.contains("COMMIT;"));
    }
    #[test]
    pub(super) fn test_join_emit() {
        let j = SQLJoin::inner("orders").alias("o").on("u.id = o.user_id");
        let s = j.emit();
        assert!(s.contains("INNER JOIN orders AS o ON u.id = o.user_id"));
    }
    #[test]
    pub(super) fn test_analyze_builder() {
        let sql = SQLAnalyzeBuilder::new("users")
            .verbose()
            .column("email")
            .build();
        assert!(sql.contains("ANALYZE VERBOSE users"));
        assert!(sql.contains("(email)"));
    }
}
#[cfg(test)]
mod sql_schema_tests {
    use super::*;
    #[test]
    pub(super) fn test_table_info() {
        let mut info = SQLTableInfo::new("users");
        info.columns.push(SQLColumnInfo {
            name: "id".to_string(),
            ordinal_position: 1,
            data_type: "INTEGER".to_string(),
            is_nullable: false,
            column_default: None,
            is_primary_key: true,
            is_unique: true,
        });
        info.columns.push(SQLColumnInfo {
            name: "email".to_string(),
            ordinal_position: 2,
            data_type: "TEXT".to_string(),
            is_nullable: true,
            column_default: None,
            is_primary_key: false,
            is_unique: true,
        });
        assert_eq!(info.primary_key_columns().len(), 1);
        assert_eq!(info.nullable_columns().len(), 1);
        let q = info.emit_describe_query();
        assert!(q.contains("users"));
    }
    #[test]
    pub(super) fn test_schema_inspector() {
        let inspector = SQLSchemaInspector::new(SQLDialect::SQLite);
        let q = inspector.list_tables_query();
        assert!(q.contains("sqlite_master"));
        let col_q = inspector.column_info_query("users");
        assert!(col_q.contains("PRAGMA"));
        let idx_q = inspector.index_info_query("users");
        assert!(idx_q.contains("PRAGMA"));
    }
    #[test]
    pub(super) fn test_query_plan() {
        let mut plan = SQLQueryPlan::new();
        plan.add_node(SQLQueryPlanNode::SeqScan {
            table: "users".to_string(),
            cost: 100.0,
            rows: 1000,
        });
        plan.add_node(SQLQueryPlanNode::IndexScan {
            table: "orders".to_string(),
            index: "idx_user_id".to_string(),
            cost: 10.0,
            rows: 10,
        });
        assert!(plan.has_seq_scan());
        assert!(plan.has_index_scan());
        assert!((plan.total_cost - 110.0).abs() < 0.01);
        let desc = plan.describe();
        assert!(desc.contains("Seq Scan"));
        assert!(desc.contains("Index Scan"));
    }
    #[test]
    pub(super) fn test_query_optimizer() {
        let mut opt = SQLQueryOptimizer::new();
        opt.add_table_stats("users", 100000);
        opt.add_table_stats("orders", 5000000);
        let cost = opt.estimate_join_cost("users", "orders");
        assert!(cost > 0.0);
        let select = SQLSelectBuilder::new()
            .column("*")
            .from_table("users")
            .where_cond("id = 1");
        let suggestions = opt.suggest_indexes(&select);
        assert!(suggestions.iter().any(|s| s.contains("idx_users_id")));
    }
    #[test]
    pub(super) fn test_query_formatter() {
        let formatter = SQLQueryFormatter::new();
        let sql = "SELECT * FROM users WHERE id = 1 ORDER BY name";
        let formatted = formatter.format(sql);
        assert!(!formatted.is_empty());
    }
}
#[cfg(test)]
mod sql_proc_tests {
    use super::*;
    #[test]
    pub(super) fn test_stored_function() {
        let func = SQLStoredProcedure::function("get_user_count", "INTEGER")
            .param("dept_id", "INTEGER")
            .body("BEGIN RETURN (SELECT COUNT(*) FROM users WHERE department_id = dept_id); END;")
            .emit();
        assert!(func.contains("CREATE OR REPLACE FUNCTION get_user_count"));
        assert!(func.contains("RETURNS INTEGER"));
        assert!(func.contains("dept_id INTEGER"));
    }
    #[test]
    pub(super) fn test_trigger() {
        let trigger = SQLTrigger::new("audit_users", "users", "log_changes")
            .after()
            .on_insert()
            .on_update()
            .emit();
        assert!(trigger.contains("CREATE OR REPLACE TRIGGER audit_users"));
        assert!(trigger.contains("AFTER INSERT OR UPDATE ON users"));
        assert!(trigger.contains("EXECUTE FUNCTION log_changes()"));
    }
    #[test]
    pub(super) fn test_sequence() {
        let seq = SQLSequenceBuilder::new("user_id_seq")
            .start_with(1000)
            .increment_by(1)
            .max(i64::MAX)
            .build();
        assert!(seq.contains("CREATE SEQUENCE user_id_seq"));
        assert!(seq.contains("START WITH 1000"));
    }
    #[test]
    pub(super) fn test_procedure() {
        let proc = SQLStoredProcedure::procedure("cleanup_sessions")
            .body("BEGIN DELETE FROM sessions WHERE expires_at < NOW(); END;")
            .emit();
        assert!(proc.contains("CREATE OR REPLACE PROCEDURE cleanup_sessions"));
        assert!(!proc.contains("RETURNS"));
    }
}
#[allow(dead_code)]
pub fn sql_escape_string(s: &str) -> String {
    format!("'{}'", s.replace('\'', "''"))
}
#[allow(dead_code)]
pub fn sql_is_reserved(word: &str) -> bool {
    const RESERVED: &[&str] = &[
        "SELECT", "FROM", "WHERE", "TABLE", "INDEX", "CREATE", "DROP", "INSERT", "UPDATE",
        "DELETE", "JOIN", "GROUP", "ORDER", "BY", "HAVING", "LIMIT", "OFFSET", "ON", "AS", "AND",
        "OR", "NOT", "NULL", "TRUE", "FALSE",
    ];
    RESERVED.iter().any(|r| r.eq_ignore_ascii_case(word))
}
#[allow(dead_code)]
pub fn sql_quote_if_needed(ident: &str) -> String {
    if sql_is_reserved(ident) || ident.contains(' ') || ident.contains('-') {
        format!(r#""{}""#, ident)
    } else {
        ident.to_string()
    }
}
#[allow(dead_code)]
pub fn sql_dialect_name(dialect: &SQLDialect) -> &'static str {
    match dialect {
        SQLDialect::PostgreSQL => "PostgreSQL",
        SQLDialect::MySQL => "MySQL",
        SQLDialect::SQLite => "SQLite",
        SQLDialect::MSSQL => "Microsoft SQL Server",
    }
}
#[allow(dead_code)]
pub fn sql_supports_returning(dialect: &SQLDialect) -> bool {
    matches!(dialect, SQLDialect::PostgreSQL | SQLDialect::SQLite)
}
#[allow(dead_code)]
pub fn sql_auto_increment_syntax(dialect: &SQLDialect) -> &'static str {
    match dialect {
        SQLDialect::PostgreSQL => "SERIAL",
        SQLDialect::MySQL => "AUTO_INCREMENT",
        SQLDialect::SQLite => "AUTOINCREMENT",
        SQLDialect::MSSQL => "IDENTITY(1,1)",
    }
}
#[allow(dead_code)]
pub fn sql_current_timestamp(dialect: &SQLDialect) -> &'static str {
    match dialect {
        SQLDialect::MySQL => "NOW()",
        SQLDialect::SQLite => "datetime('now')",
        _ => "CURRENT_TIMESTAMP",
    }
}
#[allow(dead_code)]
pub fn sql_string_concat(dialect: &SQLDialect, a: &str, b: &str) -> String {
    match dialect {
        SQLDialect::MySQL => format!("CONCAT({}, {})", a, b),
        _ => format!("{} || {}", a, b),
    }
}
#[allow(dead_code)]
pub fn sql_limit_offset(dialect: &SQLDialect, limit: u64, offset: u64) -> String {
    match dialect {
        SQLDialect::MSSQL | SQLDialect::SQLite => {
            format!("OFFSET {} ROWS FETCH NEXT {} ROWS ONLY", offset, limit)
        }
        _ => format!("LIMIT {} OFFSET {}", limit, offset),
    }
}
#[cfg(test)]
mod sql_util_tests {
    use super::*;
    #[test]
    pub(super) fn test_sql_escape_string() {
        let s = sql_escape_string("it's a test");
        assert_eq!(s, "'it''s a test'");
    }
    #[test]
    pub(super) fn test_sql_is_reserved() {
        assert!(sql_is_reserved("SELECT"));
        assert!(sql_is_reserved("select"));
        assert!(!sql_is_reserved("username"));
    }
    #[test]
    pub(super) fn test_sql_quote_if_needed() {
        assert_eq!(sql_quote_if_needed("username"), "username");
        assert!(sql_quote_if_needed("SELECT").starts_with('"'));
    }
    #[test]
    pub(super) fn test_sql_dialect_name() {
        assert_eq!(sql_dialect_name(&SQLDialect::PostgreSQL), "PostgreSQL");
        assert_eq!(sql_dialect_name(&SQLDialect::SQLite), "SQLite");
    }
    #[test]
    pub(super) fn test_sql_supports_returning() {
        assert!(sql_supports_returning(&SQLDialect::PostgreSQL));
        assert!(sql_supports_returning(&SQLDialect::SQLite));
        assert!(!sql_supports_returning(&SQLDialect::MySQL));
    }
    #[test]
    pub(super) fn test_sql_limit_offset() {
        let pg = sql_limit_offset(&SQLDialect::PostgreSQL, 10, 20);
        assert_eq!(pg, "LIMIT 10 OFFSET 20");
        let ms = sql_limit_offset(&SQLDialect::MSSQL, 10, 20);
        assert!(ms.contains("FETCH NEXT 10 ROWS"));
    }
    #[test]
    pub(super) fn test_sql_concat() {
        let pg = sql_string_concat(&SQLDialect::PostgreSQL, "first_name", "last_name");
        assert!(pg.contains("||"));
        let mysql = sql_string_concat(&SQLDialect::MySQL, "first_name", "last_name");
        assert!(mysql.starts_with("CONCAT"));
    }
}
#[allow(dead_code)]
pub const SQL_BACKEND_VERSION: &str = "1.0.0";
#[allow(dead_code)]
pub fn sql_version() -> &'static str {
    SQL_BACKEND_VERSION
}
#[cfg(test)]
mod sql_alter_tests {
    use super::*;
    #[test]
    pub(super) fn test_type_mapper() {
        let mapper = SQLTypeMapper::new(SQLDialect::PostgreSQL, SQLDialect::MySQL);
        assert_eq!(mapper.map_integer(), "INT");
        assert_eq!(mapper.map_boolean(), "TINYINT(1)");
        assert_eq!(mapper.map_json(), "JSON");
    }
    #[test]
    pub(super) fn test_alter_table() {
        let stmts = SQLAlterTableBuilder::new("users")
            .add_column(SQLColumnDef::new("phone", SQLType::Text).not_null())
            .drop_column("legacy_field")
            .rename_column("email", "email_address")
            .build(&SQLDialect::PostgreSQL);
        assert_eq!(stmts.len(), 3);
        assert!(stmts[0].contains("ADD COLUMN phone"));
        assert!(stmts[1].contains("DROP COLUMN legacy_field"));
        assert!(stmts[2].contains("RENAME COLUMN email TO email_address"));
    }
}
