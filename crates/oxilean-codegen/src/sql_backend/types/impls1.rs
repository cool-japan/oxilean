use std::collections::HashMap;

use super::defs::*;

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
