//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum SQLJoinType {
    Inner,
    Left,
    Right,
    Full,
    Cross,
    Natural,
}
impl SQLJoinType {
    #[allow(dead_code)]
    pub fn keyword(&self) -> &str {
        match self {
            SQLJoinType::Inner => "INNER JOIN",
            SQLJoinType::Left => "LEFT JOIN",
            SQLJoinType::Right => "RIGHT JOIN",
            SQLJoinType::Full => "FULL OUTER JOIN",
            SQLJoinType::Cross => "CROSS JOIN",
            SQLJoinType::Natural => "NATURAL JOIN",
        }
    }
}
#[allow(dead_code)]
pub struct SQLPreparedStatement {
    pub sql: String,
    pub parameters: Vec<SQLParameter>,
    pub dialect: SQLDialect,
}
impl SQLPreparedStatement {
    #[allow(dead_code)]
    pub fn new(sql: impl Into<String>, dialect: SQLDialect) -> Self {
        SQLPreparedStatement {
            sql: sql.into(),
            parameters: Vec::new(),
            dialect,
        }
    }
    #[allow(dead_code)]
    pub fn add_param(&mut self, name: impl Into<String>, ty: SQLType) {
        let index = self.parameters.len() + 1;
        self.parameters.push(SQLParameter {
            name: name.into(),
            ty,
            index,
        });
    }
    #[allow(dead_code)]
    pub fn placeholder(&self, index: usize) -> String {
        match self.dialect {
            SQLDialect::PostgreSQL => format!("${}", index),
            SQLDialect::MySQL | SQLDialect::SQLite => "?".to_string(),
            SQLDialect::MSSQL => format!("@p{}", index),
        }
    }
}
#[allow(dead_code)]
pub struct SQLSchemaInspector {
    pub dialect: SQLDialect,
}
impl SQLSchemaInspector {
    #[allow(dead_code)]
    pub fn new(dialect: SQLDialect) -> Self {
        SQLSchemaInspector { dialect }
    }
    #[allow(dead_code)]
    pub fn list_tables_query(&self) -> &str {
        match self.dialect {
            SQLDialect::SQLite => "SELECT name FROM sqlite_master WHERE type='table'",
            SQLDialect::PostgreSQL => "SELECT tablename FROM pg_tables WHERE schemaname='public'",
            SQLDialect::MySQL => "SHOW TABLES",
            SQLDialect::MSSQL => {
                "SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_TYPE = 'BASE TABLE'"
            }
        }
    }
    #[allow(dead_code)]
    pub fn column_info_query(&self, table: &str) -> String {
        match self.dialect {
            SQLDialect::SQLite => format!("PRAGMA table_info({})", table),
            SQLDialect::PostgreSQL => {
                format!(
                    "SELECT column_name, data_type, is_nullable FROM information_schema.columns WHERE table_name = '{}'",
                    table
                )
            }
            _ => format!("DESCRIBE {}", table),
        }
    }
    #[allow(dead_code)]
    pub fn index_info_query(&self, table: &str) -> String {
        match self.dialect {
            SQLDialect::SQLite => format!("PRAGMA index_list({})", table),
            SQLDialect::PostgreSQL => {
                format!(
                    "SELECT indexname, indexdef FROM pg_indexes WHERE tablename = '{}'",
                    table
                )
            }
            SQLDialect::MySQL => format!("SHOW INDEX FROM {}", table),
            _ => {
                format!(
                    "SELECT * FROM sys.indexes WHERE object_id = OBJECT_ID('{}')",
                    table
                )
            }
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SQLParamDirection {
    In,
    Out,
    InOut,
    Variadic,
}
impl SQLParamDirection {
    #[allow(dead_code)]
    pub fn keyword(&self) -> &str {
        match self {
            SQLParamDirection::In => "IN",
            SQLParamDirection::Out => "OUT",
            SQLParamDirection::InOut => "INOUT",
            SQLParamDirection::Variadic => "VARIADIC",
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SQLTriggerEvent {
    Insert,
    Update(Vec<String>),
    Delete,
    Truncate,
}
impl SQLTriggerEvent {
    #[allow(dead_code)]
    pub fn keyword(&self) -> String {
        match self {
            SQLTriggerEvent::Insert => "INSERT".to_string(),
            SQLTriggerEvent::Update(cols) => {
                if cols.is_empty() {
                    "UPDATE".to_string()
                } else {
                    format!("UPDATE OF {}", cols.join(", "))
                }
            }
            SQLTriggerEvent::Delete => "DELETE".to_string(),
            SQLTriggerEvent::Truncate => "TRUNCATE".to_string(),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SQLUpdateBuilder {
    pub table: String,
    pub sets: Vec<(String, String)>,
    pub where_clause: Option<String>,
    pub returning: Vec<String>,
}
impl SQLUpdateBuilder {
    #[allow(dead_code)]
    pub fn new(table: impl Into<String>) -> Self {
        SQLUpdateBuilder {
            table: table.into(),
            sets: Vec::new(),
            where_clause: None,
            returning: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn set(mut self, col: impl Into<String>, val: impl Into<String>) -> Self {
        self.sets.push((col.into(), val.into()));
        self
    }
    #[allow(dead_code)]
    pub fn where_cond(mut self, cond: impl Into<String>) -> Self {
        self.where_clause = Some(cond.into());
        self
    }
    #[allow(dead_code)]
    pub fn returning(mut self, col: impl Into<String>) -> Self {
        self.returning.push(col.into());
        self
    }
    #[allow(dead_code)]
    pub fn build(&self) -> String {
        let mut out = format!("UPDATE {} SET ", self.table);
        let parts: Vec<String> = self
            .sets
            .iter()
            .map(|(k, v)| format!("{} = {}", k, v))
            .collect();
        out.push_str(&parts.join(", "));
        if let Some(ref w) = self.where_clause {
            out.push_str(&format!(" WHERE {}", w));
        }
        if !self.returning.is_empty() {
            out.push_str(&format!(" RETURNING {}", self.returning.join(", ")));
        }
        out
    }
}
/// SQL dialect selector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SQLDialect {
    SQLite,
    PostgreSQL,
    MySQL,
    MSSQL,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SQLWindowFrame {
    Rows(SQLFrameBound, SQLFrameBound),
    Range(SQLFrameBound, SQLFrameBound),
}
#[allow(dead_code)]
pub struct SQLTransactionBuilder {
    pub statements: Vec<String>,
    pub isolation_level: Option<SQLIsolationLevel>,
}
impl SQLTransactionBuilder {
    #[allow(dead_code)]
    pub fn new() -> Self {
        SQLTransactionBuilder {
            statements: Vec::new(),
            isolation_level: None,
        }
    }
    #[allow(dead_code)]
    pub fn isolation(mut self, level: SQLIsolationLevel) -> Self {
        self.isolation_level = Some(level);
        self
    }
    #[allow(dead_code)]
    pub fn add_statement(mut self, stmt: impl Into<String>) -> Self {
        self.statements.push(stmt.into());
        self
    }
    #[allow(dead_code)]
    pub fn build(&self) -> String {
        let mut out = String::new();
        if let Some(ref level) = self.isolation_level {
            out.push_str(&format!(
                "SET TRANSACTION ISOLATION LEVEL {};\n",
                level.keyword()
            ));
        }
        out.push_str("BEGIN;\n");
        for stmt in &self.statements {
            out.push_str(stmt);
            out.push_str(";\n");
        }
        out.push_str("COMMIT;");
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SQLTableInfo {
    pub name: String,
    pub schema: Option<String>,
    pub columns: Vec<SQLColumnInfo>,
    pub indexes: Vec<SQLIndexInfo>,
    pub row_count_estimate: Option<u64>,
}
impl SQLTableInfo {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        SQLTableInfo {
            name: name.into(),
            schema: None,
            columns: Vec::new(),
            indexes: Vec::new(),
            row_count_estimate: None,
        }
    }
    #[allow(dead_code)]
    pub fn primary_key_columns(&self) -> Vec<&SQLColumnInfo> {
        self.columns.iter().filter(|c| c.is_primary_key).collect()
    }
    #[allow(dead_code)]
    pub fn nullable_columns(&self) -> Vec<&SQLColumnInfo> {
        self.columns.iter().filter(|c| c.is_nullable).collect()
    }
    #[allow(dead_code)]
    pub fn emit_describe_query(&self) -> String {
        format!(
            "SELECT column_name, data_type, is_nullable FROM information_schema.columns WHERE table_name = '{}'",
            self.name
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SQLAlterOperation {
    AddColumn(SQLColumnDef),
    DropColumn(String),
    RenameColumn(String, String),
    AlterColumnType(String, String),
    AddConstraint(SQLTableConstraint),
    DropConstraint(String),
    RenameTable(String),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SQLCreateTableBuilder {
    pub name: String,
    pub columns: Vec<SQLColumnDef>,
    pub constraints: Vec<SQLTableConstraint>,
    pub if_not_exists: bool,
    pub temporary: bool,
}
impl SQLCreateTableBuilder {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        SQLCreateTableBuilder {
            name: name.into(),
            columns: Vec::new(),
            constraints: Vec::new(),
            if_not_exists: false,
            temporary: false,
        }
    }
    #[allow(dead_code)]
    pub fn if_not_exists(mut self) -> Self {
        self.if_not_exists = true;
        self
    }
    #[allow(dead_code)]
    pub fn temporary(mut self) -> Self {
        self.temporary = true;
        self
    }
    #[allow(dead_code)]
    pub fn column(mut self, col: SQLColumnDef) -> Self {
        self.columns.push(col);
        self
    }
    #[allow(dead_code)]
    pub fn constraint(mut self, c: SQLTableConstraint) -> Self {
        self.constraints.push(c);
        self
    }
    #[allow(dead_code)]
    pub fn build(&self, dialect: &SQLDialect) -> String {
        let mut out = String::from("CREATE");
        if self.temporary {
            out.push_str(" TEMPORARY");
        }
        out.push_str(" TABLE");
        if self.if_not_exists {
            out.push_str(" IF NOT EXISTS");
        }
        out.push_str(&format!(" {} (\n", self.name));
        let mut defs: Vec<String> = self
            .columns
            .iter()
            .map(|c| format!("  {}", c.emit(dialect)))
            .collect();
        for constraint in &self.constraints {
            defs.push(format!("  {}", constraint.emit()));
        }
        out.push_str(&defs.join(",\n"));
        out.push_str("\n)");
        out
    }
}
#[allow(dead_code)]
pub struct SQLSequenceBuilder {
    pub name: String,
    pub start: i64,
    pub increment: i64,
    pub min_value: Option<i64>,
    pub max_value: Option<i64>,
    pub cycle: bool,
    pub cache: u64,
}
impl SQLSequenceBuilder {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        SQLSequenceBuilder {
            name: name.into(),
            start: 1,
            increment: 1,
            min_value: None,
            max_value: None,
            cycle: false,
            cache: 1,
        }
    }
    #[allow(dead_code)]
    pub fn start_with(mut self, n: i64) -> Self {
        self.start = n;
        self
    }
    #[allow(dead_code)]
    pub fn increment_by(mut self, n: i64) -> Self {
        self.increment = n;
        self
    }
    #[allow(dead_code)]
    pub fn min(mut self, n: i64) -> Self {
        self.min_value = Some(n);
        self
    }
    #[allow(dead_code)]
    pub fn max(mut self, n: i64) -> Self {
        self.max_value = Some(n);
        self
    }
    #[allow(dead_code)]
    pub fn cycle(mut self) -> Self {
        self.cycle = true;
        self
    }
    #[allow(dead_code)]
    pub fn cache(mut self, n: u64) -> Self {
        self.cache = n;
        self
    }
    #[allow(dead_code)]
    pub fn build(&self) -> String {
        let mut out = format!(
            "CREATE SEQUENCE {} START WITH {} INCREMENT BY {}",
            self.name, self.start, self.increment
        );
        if let Some(min) = self.min_value {
            out.push_str(&format!(" MINVALUE {}", min));
        }
        if let Some(max) = self.max_value {
            out.push_str(&format!(" MAXVALUE {}", max));
        }
        if self.cache > 1 {
            out.push_str(&format!(" CACHE {}", self.cache));
        }
        if self.cycle {
            out.push_str(" CYCLE");
        } else {
            out.push_str(" NO CYCLE");
        }
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SQLColumnDef {
    pub name: String,
    pub ty: SQLType,
    pub nullable: bool,
    pub primary_key: bool,
    pub unique: bool,
    pub default_value: Option<String>,
    pub references: Option<(String, String)>,
    pub auto_increment: bool,
}
impl SQLColumnDef {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, ty: SQLType) -> Self {
        SQLColumnDef {
            name: name.into(),
            ty,
            nullable: true,
            primary_key: false,
            unique: false,
            default_value: None,
            references: None,
            auto_increment: false,
        }
    }
    #[allow(dead_code)]
    pub fn not_null(mut self) -> Self {
        self.nullable = false;
        self
    }
    #[allow(dead_code)]
    pub fn primary_key(mut self) -> Self {
        self.primary_key = true;
        self.nullable = false;
        self
    }
    #[allow(dead_code)]
    pub fn unique(mut self) -> Self {
        self.unique = true;
        self
    }
    #[allow(dead_code)]
    pub fn default(mut self, val: impl Into<String>) -> Self {
        self.default_value = Some(val.into());
        self
    }
    #[allow(dead_code)]
    pub fn auto_increment(mut self) -> Self {
        self.auto_increment = true;
        self
    }
    #[allow(dead_code)]
    pub fn references(mut self, table: impl Into<String>, col: impl Into<String>) -> Self {
        self.references = Some((table.into(), col.into()));
        self
    }
    #[allow(dead_code)]
    pub fn emit(&self, _dialect: &SQLDialect) -> String {
        let ty_str = match &self.ty {
            SQLType::Integer => "INTEGER".to_string(),
            SQLType::Real => "REAL".to_string(),
            SQLType::Text => "TEXT".to_string(),
            SQLType::Boolean => "BOOLEAN".to_string(),
            SQLType::Blob => "BLOB".to_string(),
            SQLType::Null => "NULL".to_string(),
            SQLType::Timestamp => "TIMESTAMP".to_string(),
        };
        let mut out = format!("{} {}", self.name, ty_str);
        if !self.nullable && !self.primary_key {
            out.push_str(" NOT NULL");
        }
        if self.primary_key {
            out.push_str(" PRIMARY KEY");
        }
        if self.auto_increment {
            out.push_str(" AUTOINCREMENT");
        }
        if self.unique {
            out.push_str(" UNIQUE");
        }
        if let Some(ref dv) = self.default_value {
            out.push_str(&format!(" DEFAULT {}", dv));
        }
        if let Some((ref t, ref c)) = self.references {
            out.push_str(&format!(" REFERENCES {}({})", t, c));
        }
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SQLTrigger {
    pub name: String,
    pub table: String,
    pub timing: SQLTriggerTiming,
    pub events: Vec<SQLTriggerEvent>,
    pub function_name: String,
    pub for_each_row: bool,
    pub when_condition: Option<String>,
}
impl SQLTrigger {
    #[allow(dead_code)]
    pub fn new(
        name: impl Into<String>,
        table: impl Into<String>,
        function: impl Into<String>,
    ) -> Self {
        SQLTrigger {
            name: name.into(),
            table: table.into(),
            timing: SQLTriggerTiming::After,
            events: Vec::new(),
            function_name: function.into(),
            for_each_row: true,
            when_condition: None,
        }
    }
    #[allow(dead_code)]
    pub fn before(mut self) -> Self {
        self.timing = SQLTriggerTiming::Before;
        self
    }
    #[allow(dead_code)]
    pub fn after(mut self) -> Self {
        self.timing = SQLTriggerTiming::After;
        self
    }
    #[allow(dead_code)]
    pub fn on_insert(mut self) -> Self {
        self.events.push(SQLTriggerEvent::Insert);
        self
    }
    #[allow(dead_code)]
    pub fn on_update(mut self) -> Self {
        self.events.push(SQLTriggerEvent::Update(Vec::new()));
        self
    }
    #[allow(dead_code)]
    pub fn on_delete(mut self) -> Self {
        self.events.push(SQLTriggerEvent::Delete);
        self
    }
    #[allow(dead_code)]
    pub fn for_each_statement(mut self) -> Self {
        self.for_each_row = false;
        self
    }
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let timing = match self.timing {
            SQLTriggerTiming::Before => "BEFORE",
            SQLTriggerTiming::After => "AFTER",
            SQLTriggerTiming::InsteadOf => "INSTEAD OF",
        };
        let events: Vec<String> = self.events.iter().map(|e| e.keyword()).collect();
        let row_stmt = if self.for_each_row {
            "FOR EACH ROW"
        } else {
            "FOR EACH STATEMENT"
        };
        let mut out = format!(
            "CREATE OR REPLACE TRIGGER {} {} {} ON {} {}",
            self.name,
            timing,
            events.join(" OR "),
            self.table,
            row_stmt
        );
        if let Some(ref cond) = self.when_condition {
            out.push_str(&format!(" WHEN ({})", cond));
        }
        out.push_str(&format!(" EXECUTE FUNCTION {}();", self.function_name));
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SQLIndexBuilder {
    pub name: String,
    pub table: String,
    pub columns: Vec<String>,
    pub unique: bool,
    pub if_not_exists: bool,
    pub where_clause: Option<String>,
}
impl SQLIndexBuilder {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, table: impl Into<String>) -> Self {
        SQLIndexBuilder {
            name: name.into(),
            table: table.into(),
            columns: Vec::new(),
            unique: false,
            if_not_exists: false,
            where_clause: None,
        }
    }
    #[allow(dead_code)]
    pub fn on_column(mut self, col: impl Into<String>) -> Self {
        self.columns.push(col.into());
        self
    }
    #[allow(dead_code)]
    pub fn unique(mut self) -> Self {
        self.unique = true;
        self
    }
    #[allow(dead_code)]
    pub fn if_not_exists(mut self) -> Self {
        self.if_not_exists = true;
        self
    }
    #[allow(dead_code)]
    pub fn where_cond(mut self, cond: impl Into<String>) -> Self {
        self.where_clause = Some(cond.into());
        self
    }
    #[allow(dead_code)]
    pub fn build(&self) -> String {
        let mut out = String::from("CREATE");
        if self.unique {
            out.push_str(" UNIQUE");
        }
        out.push_str(" INDEX");
        if self.if_not_exists {
            out.push_str(" IF NOT EXISTS");
        }
        out.push_str(&format!(
            " {} ON {} ({})",
            self.name,
            self.table,
            self.columns.join(", ")
        ));
        if let Some(ref w) = self.where_clause {
            out.push_str(&format!(" WHERE {}", w));
        }
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SQLTableConstraint {
    PrimaryKey(Vec<String>),
    Unique(Vec<String>),
    ForeignKey {
        columns: Vec<String>,
        ref_table: String,
        ref_columns: Vec<String>,
        on_delete: Option<SQLForeignKeyAction>,
        on_update: Option<SQLForeignKeyAction>,
    },
    Check(String),
}
impl SQLTableConstraint {
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        match self {
            SQLTableConstraint::PrimaryKey(cols) => {
                format!("PRIMARY KEY ({})", cols.join(", "))
            }
            SQLTableConstraint::Unique(cols) => format!("UNIQUE ({})", cols.join(", ")),
            SQLTableConstraint::ForeignKey {
                columns,
                ref_table,
                ref_columns,
                on_delete,
                on_update,
            } => {
                let mut s = format!(
                    "FOREIGN KEY ({}) REFERENCES {}({})",
                    columns.join(", "),
                    ref_table,
                    ref_columns.join(", ")
                );
                if let Some(ref a) = on_delete {
                    s.push_str(&format!(" ON DELETE {}", a.keyword()));
                }
                if let Some(ref a) = on_update {
                    s.push_str(&format!(" ON UPDATE {}", a.keyword()));
                }
                s
            }
            SQLTableConstraint::Check(expr) => format!("CHECK ({})", expr),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SQLMigration {
    pub version: u32,
    pub description: String,
    pub up_statements: Vec<String>,
    pub down_statements: Vec<String>,
}
impl SQLMigration {
    #[allow(dead_code)]
    pub fn new(version: u32, description: impl Into<String>) -> Self {
        SQLMigration {
            version,
            description: description.into(),
            up_statements: Vec::new(),
            down_statements: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn up(mut self, stmt: impl Into<String>) -> Self {
        self.up_statements.push(stmt.into());
        self
    }
    #[allow(dead_code)]
    pub fn down(mut self, stmt: impl Into<String>) -> Self {
        self.down_statements.push(stmt.into());
        self
    }
    #[allow(dead_code)]
    pub fn emit_up(&self) -> String {
        format!(
            "-- Migration v{}: {}\n{}",
            self.version,
            self.description,
            self.up_statements.join(";\n")
        )
    }
    #[allow(dead_code)]
    pub fn emit_down(&self) -> String {
        format!(
            "-- Rollback v{}: {}\n{}",
            self.version,
            self.description,
            self.down_statements.join(";\n")
        )
    }
}
#[allow(dead_code)]
pub struct SQLQueryFormatter {
    pub indent_size: usize,
    pub uppercase_keywords: bool,
    pub max_line_length: usize,
}
impl SQLQueryFormatter {
    #[allow(dead_code)]
    pub fn new() -> Self {
        SQLQueryFormatter {
            indent_size: 2,
            uppercase_keywords: true,
            max_line_length: 80,
        }
    }
    #[allow(dead_code)]
    pub fn format(&self, sql: &str) -> String {
        let keywords = [
            "SELECT", "FROM", "WHERE", "JOIN", "ON", "GROUP BY", "ORDER BY", "HAVING", "LIMIT",
            "OFFSET",
        ];
        let mut result = sql.to_string();
        if !self.uppercase_keywords {
            result = result.to_lowercase();
        }
        for kw in &keywords {
            let lower = kw.to_lowercase();
            let target = if self.uppercase_keywords {
                kw
            } else {
                &lower.as_str()
            };
            let pat = format!(" {} ", target);
            let replacement = format!("\n{}{} ", " ".repeat(self.indent_size), target);
            result = result.replace(&pat, &replacement);
        }
        result
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SQLTriggerTiming {
    Before,
    After,
    InsteadOf,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SQLIndexInfo {
    pub name: String,
    pub columns: Vec<String>,
    pub is_unique: bool,
    pub is_primary: bool,
    pub index_type: String,
}
#[allow(dead_code)]
pub struct SQLAnalyzeBuilder {
    pub table: String,
    pub columns: Vec<String>,
    pub verbose: bool,
}
impl SQLAnalyzeBuilder {
    #[allow(dead_code)]
    pub fn new(table: impl Into<String>) -> Self {
        SQLAnalyzeBuilder {
            table: table.into(),
            columns: Vec::new(),
            verbose: false,
        }
    }
    #[allow(dead_code)]
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }
    #[allow(dead_code)]
    pub fn column(mut self, col: impl Into<String>) -> Self {
        self.columns.push(col.into());
        self
    }
    #[allow(dead_code)]
    pub fn build(&self) -> String {
        let verbose = if self.verbose { "VERBOSE " } else { "" };
        if self.columns.is_empty() {
            format!("ANALYZE {}{}", verbose, self.table)
        } else {
            format!(
                "ANALYZE {}{} ({})",
                verbose,
                self.table,
                self.columns.join(", ")
            )
        }
    }
}
#[allow(dead_code)]
pub struct SQLTypeMapper {
    pub source_dialect: SQLDialect,
    pub target_dialect: SQLDialect,
}
impl SQLTypeMapper {
    #[allow(dead_code)]
    pub fn new(source: SQLDialect, target: SQLDialect) -> Self {
        SQLTypeMapper {
            source_dialect: source,
            target_dialect: target,
        }
    }
    #[allow(dead_code)]
    pub fn map_integer(&self) -> &'static str {
        match self.target_dialect {
            SQLDialect::PostgreSQL => "INTEGER",
            SQLDialect::MySQL => "INT",
            SQLDialect::SQLite => "INTEGER",
            SQLDialect::MSSQL => "INT",
        }
    }
    #[allow(dead_code)]
    pub fn map_bigint(&self) -> &'static str {
        match self.target_dialect {
            SQLDialect::PostgreSQL => "BIGINT",
            SQLDialect::MySQL => "BIGINT",
            SQLDialect::SQLite => "INTEGER",
            SQLDialect::MSSQL => "BIGINT",
        }
    }
    #[allow(dead_code)]
    pub fn map_text(&self) -> &'static str {
        match self.target_dialect {
            SQLDialect::PostgreSQL | SQLDialect::SQLite => "TEXT",
            SQLDialect::MySQL => "LONGTEXT",
            SQLDialect::MSSQL => "NVARCHAR(MAX)",
        }
    }
    #[allow(dead_code)]
    pub fn map_boolean(&self) -> &'static str {
        match self.target_dialect {
            SQLDialect::PostgreSQL | SQLDialect::SQLite => "BOOLEAN",
            SQLDialect::MySQL => "TINYINT(1)",
            SQLDialect::MSSQL => "BIT",
        }
    }
    #[allow(dead_code)]
    pub fn map_timestamp(&self) -> &'static str {
        match self.target_dialect {
            SQLDialect::PostgreSQL => "TIMESTAMPTZ",
            SQLDialect::MySQL => "DATETIME",
            SQLDialect::SQLite => "DATETIME",
            SQLDialect::MSSQL => "DATETIME2",
        }
    }
    #[allow(dead_code)]
    pub fn map_json(&self) -> &'static str {
        match self.target_dialect {
            SQLDialect::PostgreSQL => "JSONB",
            SQLDialect::MySQL => "JSON",
            SQLDialect::SQLite => "TEXT",
            SQLDialect::MSSQL => "NVARCHAR(MAX)",
        }
    }
}
/// SQL statements that the backend can emit.
#[derive(Debug, Clone)]
pub enum SQLStmt {
    Select {
        cols: Vec<String>,
        from: String,
        where_: Option<SQLExpr>,
        limit: Option<usize>,
    },
    Insert {
        table: String,
        values: Vec<SQLExpr>,
    },
    Update {
        table: String,
        set_col: String,
        set_val: SQLExpr,
        where_: Option<SQLExpr>,
    },
    Delete {
        table: String,
        where_: Option<SQLExpr>,
    },
    CreateTable(SQLTable),
    DropTable(String),
}
/// The SQL code generation backend.
pub struct SQLBackend {
    pub dialect: SQLDialect,
}
impl SQLBackend {
    /// Create a new SQL backend for the given dialect.
    pub fn new(dialect: SQLDialect) -> Self {
        SQLBackend { dialect }
    }
    /// Emit a SQL type keyword for the current dialect.
    pub fn emit_type(&self, ty: &SQLType) -> &str {
        match self.dialect {
            SQLDialect::PostgreSQL => match ty {
                SQLType::Integer => "INTEGER",
                SQLType::Real => "DOUBLE PRECISION",
                SQLType::Text => "TEXT",
                SQLType::Blob => "BYTEA",
                SQLType::Null => "NULL",
                SQLType::Boolean => "BOOLEAN",
                SQLType::Timestamp => "TIMESTAMP",
            },
            SQLDialect::MySQL => match ty {
                SQLType::Integer => "INT",
                SQLType::Real => "DOUBLE",
                SQLType::Text => "TEXT",
                SQLType::Blob => "BLOB",
                SQLType::Null => "NULL",
                SQLType::Boolean => "TINYINT(1)",
                SQLType::Timestamp => "DATETIME",
            },
            SQLDialect::MSSQL => match ty {
                SQLType::Integer => "INT",
                SQLType::Real => "FLOAT",
                SQLType::Text => "NVARCHAR(MAX)",
                SQLType::Blob => "VARBINARY(MAX)",
                SQLType::Null => "NULL",
                SQLType::Boolean => "BIT",
                SQLType::Timestamp => "DATETIME2",
            },
            SQLDialect::SQLite => match ty {
                SQLType::Integer => "INTEGER",
                SQLType::Real => "REAL",
                SQLType::Text => "TEXT",
                SQLType::Blob => "BLOB",
                SQLType::Null => "NULL",
                SQLType::Boolean => "INTEGER",
                SQLType::Timestamp => "TEXT",
            },
        }
    }
    /// Emit a SQL expression as a string.
    pub fn emit_expr(&self, expr: &SQLExpr) -> String {
        match expr {
            SQLExpr::Column(name) => name.clone(),
            SQLExpr::Literal(val) => val.clone(),
            SQLExpr::BinOp(lhs, op, rhs) => {
                format!("({} {} {})", self.emit_expr(lhs), op, self.emit_expr(rhs))
            }
            SQLExpr::FuncCall(func, args) => {
                let arg_strs: Vec<String> = args.iter().map(|a| self.emit_expr(a)).collect();
                format!("{}({})", func, arg_strs.join(", "))
            }
        }
    }
    /// Emit a complete SQL statement.
    pub fn emit_stmt(&self, stmt: &SQLStmt) -> String {
        match stmt {
            SQLStmt::Select {
                cols,
                from,
                where_,
                limit,
            } => {
                let col_str = if cols.is_empty() {
                    "*".to_string()
                } else {
                    cols.join(", ")
                };
                let mut s = format!("SELECT {} FROM {}", col_str, from);
                if let Some(cond) = where_ {
                    s.push_str(&format!(" WHERE {}", self.emit_expr(cond)));
                }
                if let Some(n) = limit {
                    match self.dialect {
                        SQLDialect::MSSQL => {
                            s = format!("SELECT TOP {} {} FROM {}", n, col_str, from);
                            if let Some(cond) = where_ {
                                s.push_str(&format!(" WHERE {}", self.emit_expr(cond)));
                            }
                        }
                        _ => s.push_str(&format!(" LIMIT {}", n)),
                    }
                }
                s.push(';');
                s
            }
            SQLStmt::Insert { table, values } => {
                let val_strs: Vec<String> = values.iter().map(|v| self.emit_expr(v)).collect();
                format!("INSERT INTO {} VALUES ({});", table, val_strs.join(", "))
            }
            SQLStmt::Update {
                table,
                set_col,
                set_val,
                where_,
            } => {
                let mut s = format!(
                    "UPDATE {} SET {} = {}",
                    table,
                    set_col,
                    self.emit_expr(set_val)
                );
                if let Some(cond) = where_ {
                    s.push_str(&format!(" WHERE {}", self.emit_expr(cond)));
                }
                s.push(';');
                s
            }
            SQLStmt::Delete { table, where_ } => {
                let mut s = format!("DELETE FROM {}", table);
                if let Some(cond) = where_ {
                    s.push_str(&format!(" WHERE {}", self.emit_expr(cond)));
                }
                s.push(';');
                s
            }
            SQLStmt::CreateTable(table) => self.create_table_stmt(table),
            SQLStmt::DropTable(name) => format!("DROP TABLE IF EXISTS {};", name),
        }
    }
    /// Build a CREATE TABLE statement from a `SQLTable`.
    pub fn create_table_stmt(&self, table: &SQLTable) -> String {
        let cols: Vec<String> = table
            .columns
            .iter()
            .map(|col| {
                let mut def = format!("{} {}", col.name, self.emit_type(&col.ty));
                if col.primary_key {
                    def.push_str(" PRIMARY KEY");
                }
                if col.not_null {
                    def.push_str(" NOT NULL");
                }
                def
            })
            .collect();
        format!(
            "CREATE TABLE IF NOT EXISTS {} (\n  {}\n);",
            table.name,
            cols.join(",\n  ")
        )
    }
    /// Produce a default table schema for a named OxiLean type.
    pub fn schema_for_type(&self, type_name: &str) -> SQLTable {
        SQLTable {
            name: type_name.to_ascii_lowercase(),
            columns: vec![
                SQLColumn {
                    name: "id".to_string(),
                    ty: SQLType::Integer,
                    not_null: true,
                    primary_key: true,
                },
                SQLColumn {
                    name: "name".to_string(),
                    ty: SQLType::Text,
                    not_null: true,
                    primary_key: false,
                },
                SQLColumn {
                    name: "created_at".to_string(),
                    ty: SQLType::Timestamp,
                    not_null: false,
                    primary_key: false,
                },
            ],
        }
    }
    /// Emit a simple SELECT * from a table.
    pub fn select_all(&self, table: &str) -> String {
        self.emit_stmt(&SQLStmt::Select {
            cols: vec![],
            from: table.to_string(),
            where_: None,
            limit: None,
        })
    }
    /// Emit a SELECT with a LIMIT clause.
    pub fn select_limit(&self, table: &str, n: usize) -> String {
        self.emit_stmt(&SQLStmt::Select {
            cols: vec![],
            from: table.to_string(),
            where_: None,
            limit: Some(n),
        })
    }
    /// Emit a parameterised INSERT placeholder list (dialect-aware).
    pub fn insert_placeholders(&self, table: &str, col_count: usize) -> String {
        let placeholders: Vec<String> = (1..=col_count)
            .map(|i| match self.dialect {
                SQLDialect::PostgreSQL => format!("${}", i),
                _ => "?".to_string(),
            })
            .collect();
        format!(
            "INSERT INTO {} VALUES ({});",
            table,
            placeholders.join(", ")
        )
    }
}
/// SQL expression AST.
#[derive(Debug, Clone)]
pub enum SQLExpr {
    Column(String),
    Literal(String),
    BinOp(Box<SQLExpr>, String, Box<SQLExpr>),
    FuncCall(String, Vec<SQLExpr>),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SQLInsertBuilder {
    pub table: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub on_conflict: Option<SQLConflictAction>,
    pub returning: Vec<String>,
}
impl SQLInsertBuilder {
    #[allow(dead_code)]
    pub fn new(table: impl Into<String>) -> Self {
        SQLInsertBuilder {
            table: table.into(),
            columns: Vec::new(),
            rows: Vec::new(),
            on_conflict: None,
            returning: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn column(mut self, col: impl Into<String>) -> Self {
        self.columns.push(col.into());
        self
    }
    #[allow(dead_code)]
    pub fn values(mut self, vals: Vec<String>) -> Self {
        self.rows.push(vals);
        self
    }
    #[allow(dead_code)]
    pub fn on_conflict(mut self, action: SQLConflictAction) -> Self {
        self.on_conflict = Some(action);
        self
    }
    #[allow(dead_code)]
    pub fn returning(mut self, col: impl Into<String>) -> Self {
        self.returning.push(col.into());
        self
    }
    #[allow(dead_code)]
    pub fn build(&self) -> String {
        let mut out = format!("INSERT INTO {}", self.table);
        if !self.columns.is_empty() {
            out.push_str(&format!(" ({})", self.columns.join(", ")));
        }
        out.push_str(" VALUES");
        let rows: Vec<String> = self
            .rows
            .iter()
            .map(|row| format!("({})", row.join(", ")))
            .collect();
        out.push(' ');
        out.push_str(&rows.join(", "));
        if let Some(ref action) = self.on_conflict {
            match action {
                SQLConflictAction::Ignore => out.push_str(" ON CONFLICT DO NOTHING"),
                SQLConflictAction::Replace => out.push_str(" OR REPLACE"),
                SQLConflictAction::Update(sets) => {
                    out.push_str(" ON CONFLICT DO UPDATE SET ");
                    let parts: Vec<String> =
                        sets.iter().map(|(k, v)| format!("{} = {}", k, v)).collect();
                    out.push_str(&parts.join(", "));
                }
            }
        }
        if !self.returning.is_empty() {
            out.push_str(&format!(" RETURNING {}", self.returning.join(", ")));
        }
        out
    }
}
#[allow(dead_code)]
pub struct SQLQueryOptimizer {
    pub stats: std::collections::HashMap<String, u64>,
}
impl SQLQueryOptimizer {
    #[allow(dead_code)]
    pub fn new() -> Self {
        SQLQueryOptimizer {
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add_table_stats(&mut self, table: impl Into<String>, rows: u64) {
        self.stats.insert(table.into(), rows);
    }
    #[allow(dead_code)]
    pub fn estimate_join_cost(&self, left: &str, right: &str) -> f64 {
        let left_rows = self.stats.get(left).copied().unwrap_or(1000);
        let right_rows = self.stats.get(right).copied().unwrap_or(1000);
        (left_rows as f64) * (right_rows as f64).log2().max(1.0)
    }
    #[allow(dead_code)]
    pub fn suggest_indexes(&self, select: &SQLSelectBuilder) -> Vec<String> {
        let mut suggestions = Vec::new();
        if let Some(ref table) = select.from {
            if let Some(ref w) = select.where_clause {
                if w.contains("id") {
                    suggestions.push(format!(
                        "CREATE INDEX idx_{}_id ON {} (id)",
                        table.to_lowercase(),
                        table
                    ));
                }
                if w.contains("created_at") {
                    suggestions.push(format!(
                        "CREATE INDEX idx_{}_created_at ON {} (created_at)",
                        table.to_lowercase(),
                        table
                    ));
                }
            }
        }
        suggestions
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SQLForeignKeyAction {
    Cascade,
    SetNull,
    SetDefault,
    Restrict,
    NoAction,
}
impl SQLForeignKeyAction {
    #[allow(dead_code)]
    pub fn keyword(&self) -> &str {
        match self {
            SQLForeignKeyAction::Cascade => "CASCADE",
            SQLForeignKeyAction::SetNull => "SET NULL",
            SQLForeignKeyAction::SetDefault => "SET DEFAULT",
            SQLForeignKeyAction::Restrict => "RESTRICT",
            SQLForeignKeyAction::NoAction => "NO ACTION",
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SQLStoredProcedure {
    pub name: String,
    pub params: Vec<SQLFunctionParam>,
    pub body: String,
    pub language: String,
    pub is_function: bool,
    pub return_type: Option<String>,
}
impl SQLStoredProcedure {
    #[allow(dead_code)]
    pub fn function(name: impl Into<String>, return_type: impl Into<String>) -> Self {
        SQLStoredProcedure {
            name: name.into(),
            params: Vec::new(),
            body: String::new(),
            language: "plpgsql".to_string(),
            is_function: true,
            return_type: Some(return_type.into()),
        }
    }
    #[allow(dead_code)]
    pub fn procedure(name: impl Into<String>) -> Self {
        SQLStoredProcedure {
            name: name.into(),
            params: Vec::new(),
            body: String::new(),
            language: "plpgsql".to_string(),
            is_function: false,
            return_type: None,
        }
    }
    #[allow(dead_code)]
    pub fn param(mut self, name: impl Into<String>, ty: impl Into<String>) -> Self {
        self.params.push(SQLFunctionParam {
            name: name.into(),
            ty: ty.into(),
            direction: SQLParamDirection::In,
            default_value: None,
        });
        self
    }
    #[allow(dead_code)]
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = body.into();
        self
    }
    #[allow(dead_code)]
    pub fn language(mut self, lang: impl Into<String>) -> Self {
        self.language = lang.into();
        self
    }
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let kind = if self.is_function {
            "FUNCTION"
        } else {
            "PROCEDURE"
        };
        let params: Vec<String> = self
            .params
            .iter()
            .map(|p| format!("{} {} {}", p.direction.keyword(), p.name, p.ty))
            .collect();
        let mut out = format!(
            "CREATE OR REPLACE {} {}({})\n",
            kind,
            self.name,
            params.join(", ")
        );
        if let Some(ref ret) = self.return_type {
            out.push_str(&format!("RETURNS {}\n", ret));
        }
        out.push_str(&format!(
            "LANGUAGE {}\nAS $$\n{}\n$$;",
            self.language, self.body
        ));
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SQLJoin {
    pub join_type: SQLJoinType,
    pub table: String,
    pub alias: Option<String>,
    pub condition: Option<String>,
}
impl SQLJoin {
    #[allow(dead_code)]
    pub fn inner(table: impl Into<String>) -> Self {
        SQLJoin {
            join_type: SQLJoinType::Inner,
            table: table.into(),
            alias: None,
            condition: None,
        }
    }
    #[allow(dead_code)]
    pub fn left(table: impl Into<String>) -> Self {
        SQLJoin {
            join_type: SQLJoinType::Left,
            table: table.into(),
            alias: None,
            condition: None,
        }
    }
    #[allow(dead_code)]
    pub fn on(mut self, condition: impl Into<String>) -> Self {
        self.condition = Some(condition.into());
        self
    }
    #[allow(dead_code)]
    pub fn alias(mut self, alias: impl Into<String>) -> Self {
        self.alias = Some(alias.into());
        self
    }
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let mut out = format!("{} {}", self.join_type.keyword(), self.table);
        if let Some(ref a) = self.alias {
            out.push_str(&format!(" AS {}", a));
        }
        if let Some(ref cond) = self.condition {
            out.push_str(&format!(" ON {}", cond));
        }
        out
    }
}
#[allow(dead_code)]
pub struct SQLWithQuery {
    pub ctes: Vec<SQLCommonTableExpression>,
    pub final_query: SQLSelectBuilder,
}
impl SQLWithQuery {
    #[allow(dead_code)]
    pub fn new(final_query: SQLSelectBuilder) -> Self {
        SQLWithQuery {
            ctes: Vec::new(),
            final_query,
        }
    }
    #[allow(dead_code)]
    pub fn with(mut self, cte: SQLCommonTableExpression) -> Self {
        self.ctes.push(cte);
        self
    }
    #[allow(dead_code)]
    pub fn build(&self) -> String {
        let recursive = if self.ctes.iter().any(|c| c.recursive) {
            " RECURSIVE"
        } else {
            ""
        };
        let cte_parts: Vec<String> = self.ctes.iter().map(|c| c.emit_cte_part()).collect();
        format!(
            "WITH{} {} {}",
            recursive,
            cte_parts.join(", "),
            self.final_query.build()
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SQLColumnInfo {
    pub name: String,
    pub ordinal_position: u32,
    pub data_type: String,
    pub is_nullable: bool,
    pub column_default: Option<String>,
    pub is_primary_key: bool,
    pub is_unique: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SQLQueryPlanNode {
    SeqScan {
        table: String,
        cost: f64,
        rows: u64,
    },
    IndexScan {
        table: String,
        index: String,
        cost: f64,
        rows: u64,
    },
    HashJoin {
        cost: f64,
        rows: u64,
    },
    MergeJoin {
        cost: f64,
        rows: u64,
    },
    NestedLoop {
        cost: f64,
        rows: u64,
    },
    Sort {
        key: Vec<String>,
        cost: f64,
    },
    Aggregate {
        function: String,
        cost: f64,
    },
    Hash {
        cost: f64,
    },
}
impl SQLQueryPlanNode {
    #[allow(dead_code)]
    pub fn cost(&self) -> f64 {
        match self {
            SQLQueryPlanNode::SeqScan { cost, .. }
            | SQLQueryPlanNode::IndexScan { cost, .. }
            | SQLQueryPlanNode::HashJoin { cost, .. }
            | SQLQueryPlanNode::MergeJoin { cost, .. }
            | SQLQueryPlanNode::NestedLoop { cost, .. }
            | SQLQueryPlanNode::Sort { cost, .. }
            | SQLQueryPlanNode::Aggregate { cost, .. }
            | SQLQueryPlanNode::Hash { cost, .. } => *cost,
        }
    }
    #[allow(dead_code)]
    pub fn node_type(&self) -> &str {
        match self {
            SQLQueryPlanNode::SeqScan { .. } => "Seq Scan",
            SQLQueryPlanNode::IndexScan { .. } => "Index Scan",
            SQLQueryPlanNode::HashJoin { .. } => "Hash Join",
            SQLQueryPlanNode::MergeJoin { .. } => "Merge Join",
            SQLQueryPlanNode::NestedLoop { .. } => "Nested Loop",
            SQLQueryPlanNode::Sort { .. } => "Sort",
            SQLQueryPlanNode::Aggregate { .. } => "Aggregate",
            SQLQueryPlanNode::Hash { .. } => "Hash",
        }
    }
}
#[allow(dead_code)]
pub struct SQLQueryPlan {
    pub nodes: Vec<SQLQueryPlanNode>,
    pub total_cost: f64,
    pub estimated_rows: u64,
}
impl SQLQueryPlan {
    #[allow(dead_code)]
    pub fn new() -> Self {
        SQLQueryPlan {
            nodes: Vec::new(),
            total_cost: 0.0,
            estimated_rows: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_node(&mut self, node: SQLQueryPlanNode) {
        self.total_cost += node.cost();
        self.nodes.push(node);
    }
    #[allow(dead_code)]
    pub fn has_seq_scan(&self) -> bool {
        self.nodes
            .iter()
            .any(|n| matches!(n, SQLQueryPlanNode::SeqScan { .. }))
    }
    #[allow(dead_code)]
    pub fn has_index_scan(&self) -> bool {
        self.nodes
            .iter()
            .any(|n| matches!(n, SQLQueryPlanNode::IndexScan { .. }))
    }
    #[allow(dead_code)]
    pub fn describe(&self) -> String {
        let mut out = format!(
            "Query Plan (total cost: {:.2}, rows: {}):\n",
            self.total_cost, self.estimated_rows
        );
        for (i, node) in self.nodes.iter().enumerate() {
            out.push_str(&format!(
                "  {}: {} (cost: {:.2})\n",
                i + 1,
                node.node_type(),
                node.cost()
            ));
        }
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SQLFunctionParam {
    pub name: String,
    pub ty: String,
    pub direction: SQLParamDirection,
    pub default_value: Option<String>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SQLParameter {
    pub name: String,
    pub ty: SQLType,
    pub index: usize,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SQLAlterTableBuilder {
    pub table: String,
    pub operations: Vec<SQLAlterOperation>,
}
impl SQLAlterTableBuilder {
    #[allow(dead_code)]
    pub fn new(table: impl Into<String>) -> Self {
        SQLAlterTableBuilder {
            table: table.into(),
            operations: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add_column(mut self, col: SQLColumnDef) -> Self {
        self.operations.push(SQLAlterOperation::AddColumn(col));
        self
    }
    #[allow(dead_code)]
    pub fn drop_column(mut self, name: impl Into<String>) -> Self {
        self.operations
            .push(SQLAlterOperation::DropColumn(name.into()));
        self
    }
    #[allow(dead_code)]
    pub fn rename_column(mut self, from: impl Into<String>, to: impl Into<String>) -> Self {
        self.operations
            .push(SQLAlterOperation::RenameColumn(from.into(), to.into()));
        self
    }
    #[allow(dead_code)]
    pub fn build(&self, dialect: &SQLDialect) -> Vec<String> {
        self.operations
            .iter()
            .map(|op| match op {
                SQLAlterOperation::AddColumn(col) => {
                    format!(
                        "ALTER TABLE {} ADD COLUMN {}",
                        self.table,
                        col.emit(dialect)
                    )
                }
                SQLAlterOperation::DropColumn(name) => {
                    format!("ALTER TABLE {} DROP COLUMN {}", self.table, name)
                }
                SQLAlterOperation::RenameColumn(from, to) => {
                    format!(
                        "ALTER TABLE {} RENAME COLUMN {} TO {}",
                        self.table, from, to
                    )
                }
                SQLAlterOperation::AlterColumnType(col, ty) => {
                    format!(
                        "ALTER TABLE {} ALTER COLUMN {} TYPE {}",
                        self.table, col, ty
                    )
                }
                SQLAlterOperation::AddConstraint(c) => {
                    format!("ALTER TABLE {} ADD {}", self.table, c.emit())
                }
                SQLAlterOperation::DropConstraint(name) => {
                    format!("ALTER TABLE {} DROP CONSTRAINT {}", self.table, name)
                }
                SQLAlterOperation::RenameTable(new_name) => {
                    format!("ALTER TABLE {} RENAME TO {}", self.table, new_name)
                }
            })
            .collect()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SQLWindowFunction {
    pub function: String,
    pub partition_by: Vec<String>,
    pub order_by: Vec<(String, bool)>,
    pub frame: Option<SQLWindowFrame>,
}
impl SQLWindowFunction {
    #[allow(dead_code)]
    pub fn new(function: impl Into<String>) -> Self {
        SQLWindowFunction {
            function: function.into(),
            partition_by: Vec::new(),
            order_by: Vec::new(),
            frame: None,
        }
    }
    #[allow(dead_code)]
    pub fn partition_by(mut self, col: impl Into<String>) -> Self {
        self.partition_by.push(col.into());
        self
    }
    #[allow(dead_code)]
    pub fn order_asc(mut self, col: impl Into<String>) -> Self {
        self.order_by.push((col.into(), true));
        self
    }
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let mut out = format!("{} OVER (", self.function);
        if !self.partition_by.is_empty() {
            out.push_str(&format!("PARTITION BY {}", self.partition_by.join(", ")));
        }
        if !self.order_by.is_empty() {
            if !self.partition_by.is_empty() {
                out.push(' ');
            }
            let parts: Vec<String> = self
                .order_by
                .iter()
                .map(|(col, asc)| format!("{} {}", col, if *asc { "ASC" } else { "DESC" }))
                .collect();
            out.push_str(&format!("ORDER BY {}", parts.join(", ")));
        }
        out.push(')');
        out
    }
}
/// A table definition (schema).
#[derive(Debug, Clone)]
pub struct SQLTable {
    pub name: String,
    pub columns: Vec<SQLColumn>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SQLSelectBuilder {
    pub columns: Vec<String>,
    pub from: Option<String>,
    pub joins: Vec<SQLJoin>,
    pub where_clause: Option<String>,
    pub group_by: Vec<String>,
    pub having: Option<String>,
    pub order_by: Vec<(String, bool)>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub distinct: bool,
}
impl SQLSelectBuilder {
    #[allow(dead_code)]
    pub fn new() -> Self {
        SQLSelectBuilder {
            columns: Vec::new(),
            from: None,
            joins: Vec::new(),
            where_clause: None,
            group_by: Vec::new(),
            having: None,
            order_by: Vec::new(),
            limit: None,
            offset: None,
            distinct: false,
        }
    }
    #[allow(dead_code)]
    pub fn column(mut self, col: impl Into<String>) -> Self {
        self.columns.push(col.into());
        self
    }
    #[allow(dead_code)]
    pub fn from_table(mut self, table: impl Into<String>) -> Self {
        self.from = Some(table.into());
        self
    }
    #[allow(dead_code)]
    pub fn join(mut self, j: SQLJoin) -> Self {
        self.joins.push(j);
        self
    }
    #[allow(dead_code)]
    pub fn where_cond(mut self, cond: impl Into<String>) -> Self {
        self.where_clause = Some(cond.into());
        self
    }
    #[allow(dead_code)]
    pub fn group_by(mut self, col: impl Into<String>) -> Self {
        self.group_by.push(col.into());
        self
    }
    #[allow(dead_code)]
    pub fn order_asc(mut self, col: impl Into<String>) -> Self {
        self.order_by.push((col.into(), true));
        self
    }
    #[allow(dead_code)]
    pub fn order_desc(mut self, col: impl Into<String>) -> Self {
        self.order_by.push((col.into(), false));
        self
    }
    #[allow(dead_code)]
    pub fn limit(mut self, n: u64) -> Self {
        self.limit = Some(n);
        self
    }
    #[allow(dead_code)]
    pub fn offset(mut self, n: u64) -> Self {
        self.offset = Some(n);
        self
    }
    #[allow(dead_code)]
    pub fn distinct(mut self) -> Self {
        self.distinct = true;
        self
    }
    #[allow(dead_code)]
    pub fn build(&self) -> String {
        let mut out = String::from("SELECT");
        if self.distinct {
            out.push_str(" DISTINCT");
        }
        if self.columns.is_empty() {
            out.push_str(" *");
        } else {
            out.push(' ');
            out.push_str(&self.columns.join(", "));
        }
        if let Some(ref t) = self.from {
            out.push_str(&format!(" FROM {}", t));
        }
        for j in &self.joins {
            out.push(' ');
            out.push_str(&j.emit());
        }
        if let Some(ref w) = self.where_clause {
            out.push_str(&format!(" WHERE {}", w));
        }
        if !self.group_by.is_empty() {
            out.push_str(&format!(" GROUP BY {}", self.group_by.join(", ")));
        }
        if let Some(ref h) = self.having {
            out.push_str(&format!(" HAVING {}", h));
        }
        if !self.order_by.is_empty() {
            let parts: Vec<String> = self
                .order_by
                .iter()
                .map(|(col, asc)| format!("{} {}", col, if *asc { "ASC" } else { "DESC" }))
                .collect();
            out.push_str(&format!(" ORDER BY {}", parts.join(", ")));
        }
        if let Some(lim) = self.limit {
            out.push_str(&format!(" LIMIT {}", lim));
        }
        if let Some(off) = self.offset {
            out.push_str(&format!(" OFFSET {}", off));
        }
        out
    }
}
/// SQL column/expression types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SQLType {
    Integer,
    Real,
    Text,
    Blob,
    Null,
    Boolean,
    Timestamp,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SQLConflictAction {
    Ignore,
    Replace,
    Update(Vec<(String, String)>),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SQLDeleteBuilder {
    pub table: String,
    pub where_clause: Option<String>,
    pub returning: Vec<String>,
}
impl SQLDeleteBuilder {
    #[allow(dead_code)]
    pub fn new(table: impl Into<String>) -> Self {
        SQLDeleteBuilder {
            table: table.into(),
            where_clause: None,
            returning: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn where_cond(mut self, cond: impl Into<String>) -> Self {
        self.where_clause = Some(cond.into());
        self
    }
    #[allow(dead_code)]
    pub fn build(&self) -> String {
        let mut out = format!("DELETE FROM {}", self.table);
        if let Some(ref w) = self.where_clause {
            out.push_str(&format!(" WHERE {}", w));
        }
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SQLFrameBound {
    UnboundedPreceding,
    Preceding(u64),
    CurrentRow,
    Following(u64),
    UnboundedFollowing,
}
impl SQLFrameBound {
    #[allow(dead_code)]
    pub fn emit(&self) -> &str {
        match self {
            SQLFrameBound::UnboundedPreceding => "UNBOUNDED PRECEDING",
            SQLFrameBound::Preceding(_) => "PRECEDING",
            SQLFrameBound::CurrentRow => "CURRENT ROW",
            SQLFrameBound::Following(_) => "FOLLOWING",
            SQLFrameBound::UnboundedFollowing => "UNBOUNDED FOLLOWING",
        }
    }
}
#[allow(dead_code)]
pub struct SQLCommonTableExpression {
    pub name: String,
    pub columns: Vec<String>,
    pub query: SQLSelectBuilder,
    pub recursive: bool,
}
impl SQLCommonTableExpression {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, query: SQLSelectBuilder) -> Self {
        SQLCommonTableExpression {
            name: name.into(),
            columns: Vec::new(),
            query,
            recursive: false,
        }
    }
    #[allow(dead_code)]
    pub fn recursive(mut self) -> Self {
        self.recursive = true;
        self
    }
    #[allow(dead_code)]
    pub fn with_column(mut self, col: impl Into<String>) -> Self {
        self.columns.push(col.into());
        self
    }
    #[allow(dead_code)]
    pub fn emit_cte_part(&self) -> String {
        let col_part = if self.columns.is_empty() {
            String::new()
        } else {
            format!("({})", self.columns.join(", "))
        };
        format!("{}{} AS ({})", self.name, col_part, self.query.build())
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SQLIsolationLevel {
    ReadUncommitted,
    ReadCommitted,
    RepeatableRead,
    Serializable,
}
impl SQLIsolationLevel {
    #[allow(dead_code)]
    pub fn keyword(&self) -> &str {
        match self {
            SQLIsolationLevel::ReadUncommitted => "READ UNCOMMITTED",
            SQLIsolationLevel::ReadCommitted => "READ COMMITTED",
            SQLIsolationLevel::RepeatableRead => "REPEATABLE READ",
            SQLIsolationLevel::Serializable => "SERIALIZABLE",
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SQLViewBuilder {
    pub name: String,
    pub select: SQLSelectBuilder,
    pub replace: bool,
}
impl SQLViewBuilder {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, select: SQLSelectBuilder) -> Self {
        SQLViewBuilder {
            name: name.into(),
            select,
            replace: false,
        }
    }
    #[allow(dead_code)]
    pub fn or_replace(mut self) -> Self {
        self.replace = true;
        self
    }
    #[allow(dead_code)]
    pub fn build(&self) -> String {
        let create = if self.replace {
            "CREATE OR REPLACE VIEW"
        } else {
            "CREATE VIEW"
        };
        format!("{} {} AS {}", create, self.name, self.select.build())
    }
}
#[allow(dead_code)]
pub struct SQLMigrationRunner {
    pub migrations: Vec<SQLMigration>,
    pub current_version: u32,
}
impl SQLMigrationRunner {
    #[allow(dead_code)]
    pub fn new() -> Self {
        SQLMigrationRunner {
            migrations: Vec::new(),
            current_version: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_migration(&mut self, m: SQLMigration) {
        self.migrations.push(m);
        self.migrations.sort_by_key(|m| m.version);
    }
    #[allow(dead_code)]
    pub fn pending_migrations(&self) -> Vec<&SQLMigration> {
        self.migrations
            .iter()
            .filter(|m| m.version > self.current_version)
            .collect()
    }
    #[allow(dead_code)]
    pub fn emit_pending_sql(&self) -> String {
        self.pending_migrations()
            .iter()
            .map(|m| m.emit_up())
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}
/// A single column definition inside a CREATE TABLE.
#[derive(Debug, Clone)]
pub struct SQLColumn {
    pub name: String,
    pub ty: SQLType,
    pub not_null: bool,
    pub primary_key: bool,
}
