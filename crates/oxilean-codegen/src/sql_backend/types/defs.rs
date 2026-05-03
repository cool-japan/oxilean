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

#[allow(dead_code)]
pub struct SQLPreparedStatement {
    pub sql: String,
    pub parameters: Vec<SQLParameter>,
    pub dialect: SQLDialect,
}

#[allow(dead_code)]
pub struct SQLSchemaInspector {
    pub dialect: SQLDialect,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SQLParamDirection {
    In,
    Out,
    InOut,
    Variadic,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SQLTriggerEvent {
    Insert,
    Update(Vec<String>),
    Delete,
    Truncate,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SQLUpdateBuilder {
    pub table: String,
    pub sets: Vec<(String, String)>,
    pub where_clause: Option<String>,
    pub returning: Vec<String>,
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

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SQLTableInfo {
    pub name: String,
    pub schema: Option<String>,
    pub columns: Vec<SQLColumnInfo>,
    pub indexes: Vec<SQLIndexInfo>,
    pub row_count_estimate: Option<u64>,
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

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SQLMigration {
    pub version: u32,
    pub description: String,
    pub up_statements: Vec<String>,
    pub down_statements: Vec<String>,
}

#[allow(dead_code)]
pub struct SQLQueryFormatter {
    pub indent_size: usize,
    pub uppercase_keywords: bool,
    pub max_line_length: usize,
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

#[allow(dead_code)]
pub struct SQLTypeMapper {
    pub source_dialect: SQLDialect,
    pub target_dialect: SQLDialect,
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

#[allow(dead_code)]
pub struct SQLQueryOptimizer {
    pub stats: std::collections::HashMap<String, u64>,
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

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SQLJoin {
    pub join_type: SQLJoinType,
    pub table: String,
    pub alias: Option<String>,
    pub condition: Option<String>,
}

#[allow(dead_code)]
pub struct SQLWithQuery {
    pub ctes: Vec<SQLCommonTableExpression>,
    pub final_query: SQLSelectBuilder,
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

#[allow(dead_code)]
pub struct SQLQueryPlan {
    pub nodes: Vec<SQLQueryPlanNode>,
    pub total_cost: f64,
    pub estimated_rows: u64,
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

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SQLWindowFunction {
    pub function: String,
    pub partition_by: Vec<String>,
    pub order_by: Vec<(String, bool)>,
    pub frame: Option<SQLWindowFrame>,
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

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SQLFrameBound {
    UnboundedPreceding,
    Preceding(u64),
    CurrentRow,
    Following(u64),
    UnboundedFollowing,
}

#[allow(dead_code)]
pub struct SQLCommonTableExpression {
    pub name: String,
    pub columns: Vec<String>,
    pub query: SQLSelectBuilder,
    pub recursive: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SQLIsolationLevel {
    ReadUncommitted,
    ReadCommitted,
    RepeatableRead,
    Serializable,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SQLViewBuilder {
    pub name: String,
    pub select: SQLSelectBuilder,
    pub replace: bool,
}

#[allow(dead_code)]
pub struct SQLMigrationRunner {
    pub migrations: Vec<SQLMigration>,
    pub current_version: u32,
}

/// A single column definition inside a CREATE TABLE.
#[derive(Debug, Clone)]
pub struct SQLColumn {
    pub name: String,
    pub ty: SQLType,
    pub not_null: bool,
    pub primary_key: bool,
}
