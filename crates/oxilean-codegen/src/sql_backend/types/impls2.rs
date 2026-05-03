use std::collections::HashMap;

use super::defs::*;

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
