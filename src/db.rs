use crate::model::{Column, DbObject, ObjKind};
use anyhow::{anyhow, Result};
use rusqlite::types::ValueRef;
use rusqlite::Connection as SqlConn;
use std::time::Instant;

pub struct SqliteEngine {
    pub conn: SqlConn,
    pub path: String,
}

#[derive(Clone, Debug, Default)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Option<String>>>,
    pub affected: i64,
    pub elapsed_ms: f64,
    pub error: Option<String>,
    pub statement: String,
    pub is_select: bool,
    pub last_insert_rowid: i64,
}

impl QueryResult {
    fn err(statement: &str, e: impl std::fmt::Display) -> Self {
        QueryResult {
            error: Some(e.to_string()),
            statement: statement.to_string(),
            ..Default::default()
        }
    }
}

fn val_to_opt(v: ValueRef<'_>) -> Option<String> {
    match v {
        ValueRef::Null => None,
        ValueRef::Integer(i) => Some(i.to_string()),
        ValueRef::Real(f) => {
            if f.fract() == 0.0 && f.is_finite() {
                Some(format!("{}", f as i64))
            } else {
                Some(format!("{}", f))
            }
        }
        ValueRef::Text(t) => Some(String::from_utf8_lossy(t).to_string()),
        ValueRef::Blob(b) => Some(format!("[BLOB {} bytes]", b.len())),
    }
}

impl SqliteEngine {
    pub fn open(path: &str) -> Result<Self> {
        let conn = SqlConn::open(path)?;
        conn.pragma_update(None, "foreign_keys", "ON").ok();
        Ok(SqliteEngine { conn, path: path.to_string() })
    }

    pub fn open_memory_seeded() -> Result<Self> {
        let conn = SqlConn::open_in_memory()?;
        let eng = SqliteEngine { conn, path: ":memory:".into() };
        eng.seed_demo()?;
        Ok(eng)
    }

    pub fn seed_demo(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            CREATE TABLE customers(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                email TEXT,
                country TEXT,
                created_at TEXT
            );
            CREATE TABLE products(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                sku TEXT NOT NULL UNIQUE,
                name TEXT NOT NULL,
                price REAL,
                stock INTEGER
            );
            CREATE TABLE orders(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                customer_id INTEGER REFERENCES customers(id),
                product_id INTEGER REFERENCES products(id),
                quantity INTEGER,
                total REAL,
                order_date TEXT
            );
            CREATE TABLE employees(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                title TEXT,
                department TEXT,
                salary REAL,
                hiredate TEXT
            );
            CREATE VIEW v_order_summary AS
              SELECT o.id, c.name AS customer, p.name AS product, o.quantity, o.total
              FROM orders o
              JOIN customers c ON c.id = o.customer_id
              JOIN products p ON p.id = o.product_id;

            INSERT INTO customers(name,email,country,created_at) VALUES
             ('Alice Zhang','alice@example.com','China','2024-01-05'),
             ('Bob Li','bob@example.com','China','2024-02-11'),
             ('Carol Wang','carol@example.com','Singapore','2024-03-02'),
             ('David Chen','david@example.com','USA','2024-03-19'),
             ('Eve Liu','eve@example.com','UK','2024-04-01'),
             ('Frank Wu','frank@example.com','China','2024-04-22');
            INSERT INTO products(sku,name,price,stock) VALUES
             ('A-100','Wireless Mouse',19.9,120),
             ('A-101','Mechanical Keyboard',89.0,42),
             ('A-102','27in Monitor',219.0,15),
             ('A-103','USB-C Hub',35.5,88),
             ('A-104','Webcam 1080p',49.9,33);
            INSERT INTO orders(customer_id,product_id,quantity,total,order_date) VALUES
             (1,1,2,39.8,'2024-05-01'),
             (1,2,1,89.0,'2024-05-01'),
             (2,3,1,219.0,'2024-05-03'),
             (3,4,3,106.5,'2024-05-04'),
             (4,5,2,99.8,'2024-05-06'),
             (5,1,5,99.5,'2024-05-08'),
             (6,2,2,178.0,'2024-05-09'),
             (2,5,1,49.9,'2024-05-10');
            INSERT INTO employees(name,title,department,salary,hiredate) VALUES
             ('Grace Hopper','Engineer','R&D',18000,'2020-01-10'),
             ('Alan Turing','Architect','R&D',22000,'2019-06-01'),
             ('Linus T','Maintainer','R&D',20000,'2021-02-15'),
             ('Mary J','Manager','Ops',16000,'2022-08-20');
            "#,
        )?;
        Ok(())
    }

    pub fn objects(&self) -> Vec<DbObject> {
        let mut out = Vec::new();
        let sql = "SELECT name, type FROM sqlite_master WHERE type IN ('table','view') AND name NOT LIKE 'sqlite_%' ORDER BY type DESC, name";
        if let Ok(mut st) = self.conn.prepare(sql) {
            if let Ok(rows) = st.query_map([], |r| {
                Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?))
            }) {
                for row in rows.flatten() {
                    let kind = if row.1 == "view" { ObjKind::View } else { ObjKind::Table };
                    out.push(DbObject { name: row.0, kind });
                }
            }
        }
        out
    }

    pub fn columns(&self, table: &str) -> Vec<Column> {
        let mut out = Vec::new();
        let sql = format!("PRAGMA table_info({})", quote_ident(table));
        if let Ok(mut st) = self.conn.prepare(&sql) {
            if let Ok(rows) = st.query_map([], |r| {
                Ok(Column {
                    cid: r.get::<_, i64>(0)?,
                    name: r.get::<_, String>(1)?,
                    decl_type: r.get::<_, String>(2).unwrap_or_default(),
                    notnull: r.get::<_, i64>(3).unwrap_or(0) != 0,
                    default: r.get::<_, Option<String>>(4).ok().flatten(),
                    pk: r.get::<_, i64>(5).unwrap_or(0) != 0,
                })
            }) {
                for row in rows.flatten() {
                    out.push(row);
                }
            }
        }
        out
    }

    pub fn table_has_rowid(&self, table: &str) -> bool {
        let sql = format!("SELECT rowid FROM {} LIMIT 1", quote_ident(table));
        self.conn.prepare(&sql).is_ok()
    }

    pub fn count(&self, table: &str) -> i64 {
        let sql = format!("SELECT COUNT(*) FROM {}", quote_ident(table));
        self.conn.query_row(&sql, [], |r| r.get::<_, i64>(0)).unwrap_or(0)
    }

    pub fn query(&self, sql: &str) -> QueryResult {
        let t0 = Instant::now();
        let mut st = match self.conn.prepare(sql) {
            Ok(s) => s,
            Err(e) => return QueryResult::err(sql, e),
        };
        let col_count = st.column_count();
        if col_count == 0 {
            drop(st);
            return self.execute(sql);
        }
        let columns: Vec<String> = (0..col_count)
            .map(|i| st.column_name(i).unwrap_or("").to_string())
            .collect();
        let mut rows_out = Vec::new();
        let mut rows = match st.query([]) {
            Ok(r) => r,
            Err(e) => return QueryResult::err(sql, e),
        };
        loop {
            match rows.next() {
                Ok(Some(row)) => {
                    let mut rec = Vec::with_capacity(col_count);
                    for i in 0..col_count {
                        rec.push(val_to_opt(row.get_ref(i).unwrap_or(ValueRef::Null)));
                    }
                    rows_out.push(rec);
                }
                Ok(None) => break,
                Err(e) => return QueryResult::err(sql, e),
            }
        }
        QueryResult {
            columns,
            rows: rows_out,
            elapsed_ms: t0.elapsed().as_secs_f64() * 1000.0,
            statement: sql.to_string(),
            is_select: true,
            ..Default::default()
        }
    }

    pub fn execute(&self, sql: &str) -> QueryResult {
        let t0 = Instant::now();
        match self.conn.execute(sql, []) {
            Ok(n) => QueryResult {
                affected: n as i64,
                last_insert_rowid: self.conn.last_insert_rowid(),
                elapsed_ms: t0.elapsed().as_secs_f64() * 1000.0,
                statement: sql.to_string(),
                ..Default::default()
            },
            Err(e) => QueryResult::err(sql, e),
        }
    }

    pub fn execute_batch(&self, sql: &str) -> Result<()> {
        self.conn.execute_batch(sql)?;
        Ok(())
    }

    pub fn run_script(&self, script: &str) -> Vec<QueryResult> {
        let stmts = split_sql(script);
        let mut out = Vec::new();
        for s in stmts {
            if s.trim().is_empty() {
                continue;
            }
            out.push(self.query(&s));
        }
        if out.is_empty() {
            out.push(QueryResult { statement: script.to_string(), ..Default::default() });
        }
        out
    }

    /// Return list of tables that reference others: (child_table, child_col, parent_table, parent_col)
    pub fn foreign_keys(&self) -> Vec<(String, String, String, String)> {
        let mut out = Vec::new();
        let tables: Vec<String> = self.objects().into_iter().filter(|o| o.kind == ObjKind::Table).map(|o| o.name).collect();
        for t in tables {
            let sql = format!("PRAGMA foreign_key_list({})", quote_ident(&t));
            if let Ok(mut st) = self.conn.prepare(&sql) {
                if let Ok(rows) = st.query_map([], |r| {
                    Ok((
                        r.get::<_, String>(2).unwrap_or_default(),
                        r.get::<_, String>(3).unwrap_or_default(),
                        r.get::<_, String>(4).unwrap_or_default(),
                    ))
                }) {
                    for row in rows.flatten() {
                        // row = (parent_table, child_col, parent_col)
                        out.push((t.clone(), row.1, row.0, row.2));
                    }
                }
            }
        }
        out
    }

    pub fn update_cell(&self, table: &str, rowid: i64, col: &str, val: Option<&str>) -> Result<()> {
        let sql = format!("UPDATE {} SET {} = ?1 WHERE rowid = ?2", quote_ident(table), quote_ident(col));
        match val {
            Some(v) => self.conn.execute(&sql, rusqlite::params![v, rowid])?,
            None => self.conn.execute(&sql, rusqlite::params![rusqlite::types::Null, rowid])?,
        };
        Ok(())
    }

    pub fn delete_row(&self, table: &str, rowid: i64) -> Result<()> {
        let sql = format!("DELETE FROM {} WHERE rowid = ?1", quote_ident(table));
        self.conn.execute(&sql, rusqlite::params![rowid])?;
        Ok(())
    }

    pub fn insert_row(&self, table: &str, cols: &[String], vals: &[Option<String>]) -> Result<i64> {
        let collist: Vec<String> = cols.iter().map(|c| quote_ident(c)).collect();
        let placeholders: Vec<String> = (1..=cols.len()).map(|i| format!("?{}", i)).collect();
        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            quote_ident(table),
            collist.join(", "),
            placeholders.join(", ")
        );
        let params: Vec<rusqlite::types::Value> = vals
            .iter()
            .map(|v| match v {
                Some(s) => rusqlite::types::Value::Text(s.clone()),
                None => rusqlite::types::Value::Null,
            })
            .collect();
        self.conn.execute(&sql, rusqlite::params_from_iter(params))?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn pragma_i64(&self, pragma: &str) -> i64 {
        self.conn.query_row(&format!("PRAGMA {}", pragma), [], |r| r.get::<_, i64>(0)).unwrap_or(0)
    }

    pub fn pragma_str(&self, pragma: &str) -> String {
        self.conn.query_row(&format!("PRAGMA {}", pragma), [], |r| r.get::<_, String>(0)).unwrap_or_default()
    }

    pub fn integrity_check(&self) -> String {
        self.conn
            .query_row("PRAGMA integrity_check", [], |r| r.get::<_, String>(0))
            .unwrap_or_else(|e| e.to_string())
    }

    pub fn sqlite_version(&self) -> String {
        self.conn.query_row("SELECT sqlite_version()", [], |r| r.get::<_, String>(0)).unwrap_or_default()
    }

    pub fn db_file_size(&self) -> u64 {
        std::fs::metadata(&self.path).map(|m| m.len()).unwrap_or(0)
    }

    // ----- structure objects: indexes / triggers / views -----
    pub fn indexes(&self, table: &str) -> Vec<(String, String)> {
        self.master_list("index", Some(table))
    }
    pub fn triggers_of(&self, table: &str) -> Vec<(String, String)> {
        self.master_list("trigger", Some(table))
    }
    pub fn triggers(&self) -> Vec<(String, String)> {
        self.master_list("trigger", None)
    }
    pub fn views_list(&self) -> Vec<(String, String)> {
        self.master_list("view", None)
    }
    fn master_list(&self, kind: &str, table: Option<&str>) -> Vec<(String, String)> {
        let mut v = Vec::new();
        let sql = match table {
            Some(_) => "SELECT name, sql FROM sqlite_master WHERE type=?1 AND tbl_name=?2 AND sql IS NOT NULL ORDER BY name",
            None => "SELECT name, sql FROM sqlite_master WHERE type=?1 AND sql IS NOT NULL ORDER BY name",
        };
        let mut st = match self.conn.prepare(sql) { Ok(s) => s, Err(_) => return v };
        let mut collect = |rows: &mut rusqlite::Rows<'_>| {
            while let Ok(Some(r)) = rows.next() {
                let n: String = r.get(0).unwrap_or_default();
                let s: String = r.get(1).unwrap_or_default();
                v.push((n, s));
            }
        };
        if let Some(t) = table {
            if let Ok(mut rows) = st.query([kind, t]) { collect(&mut rows); }
        } else if let Ok(mut rows) = st.query([kind]) { collect(&mut rows); }
        v
    }
    pub fn object_sql(&self, kind: &str, name: &str) -> Option<String> {
        self.conn
            .query_row("SELECT sql FROM sqlite_master WHERE type=?1 AND name=?2", rusqlite::params![kind, name], |r| r.get::<_, String>(0))
            .ok()
    }
    pub fn execute_ddl(&self, sql: &str) -> Result<()> {
        self.conn.execute_batch(sql)?;
        Ok(())
    }
    pub fn create_index(&self, name: &str, table: &str, cols: &[String], unique: bool) -> Result<()> {
        let cl: Vec<String> = cols.iter().map(|c| quote_ident(c)).collect();
        let sql = format!(
            "CREATE {}INDEX {} ON {} ({})",
            if unique { "UNIQUE " } else { "" },
            quote_ident(name),
            quote_ident(table),
            cl.join(", ")
        );
        self.conn.execute_batch(&sql)?;
        Ok(())
    }
    pub fn create_view(&self, name: &str, select_sql: &str) -> Result<()> {
        let sql = format!("CREATE VIEW {} AS {}", quote_ident(name), select_sql);
        self.conn.execute_batch(&sql)?;
        Ok(())
    }
    pub fn drop_object(&self, kind: &str, name: &str) -> Result<()> {
        // kind is a trusted constant: INDEX | TRIGGER | VIEW
        let k = match kind.to_ascii_uppercase().as_str() {
            "INDEX" => "INDEX",
            "TRIGGER" => "TRIGGER",
            "VIEW" => "VIEW",
            _ => return Err(anyhow!("unsupported object kind: {kind}")),
        };
        self.conn.execute_batch(&format!("DROP {} IF EXISTS {}", k, quote_ident(name)))?;
        Ok(())
    }

    /// Dump schema + data as a SQL script (optionally a single table).
    pub fn dump_sql(&self, only_table: Option<&str>) -> String {
        let mut out = String::new();
        out.push_str("-- Navicat-RS SQL dump\nPRAGMA foreign_keys=OFF;\nBEGIN TRANSACTION;\n\n");
        // tables
        let tables: Vec<(String, String)> = {
            let mut v = Vec::new();
            if let Ok(mut st) = self.conn.prepare("SELECT name, sql FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name") {
                if let Ok(mut rows) = st.query([]) {
                    while let Ok(Some(r)) = rows.next() {
                        let n: String = r.get(0).unwrap_or_default();
                        let s: String = r.get(1).unwrap_or_default();
                        if let Some(f) = only_table { if f != n { continue; } }
                        v.push((n, s));
                    }
                }
            }
            v
        };
        for (name, create) in &tables {
            if !create.is_empty() {
                out.push_str(&format!("{};\n", create));
            }
            let cols = self.columns(name);
            let colnames: Vec<String> = cols.iter().map(|c| quote_ident(&c.name)).collect();
            let q = self.query(&format!("SELECT * FROM {}", quote_ident(name)));
            for row in &q.rows {
                let vals: Vec<String> = row
                    .iter()
                    .map(|v| match v {
                        Some(s) => format!("'{}'", s.replace('\'', "''")),
                        None => "NULL".into(),
                    })
                    .collect();
                out.push_str(&format!("INSERT INTO {} ({}) VALUES ({});\n", quote_ident(name), colnames.join(", "), vals.join(", ")));
            }
            out.push('\n');
        }
        // views / indexes / triggers (skip auto indexes without sql)
        for kind in ["view", "index", "trigger"] {
            if only_table.is_some() && kind != "view" {
                // indexes/triggers for a single table dump are handled below via tbl_name filter
            }
            let mut v = Vec::new();
            let sql = if only_table.is_some() && kind != "view" {
                "SELECT sql FROM sqlite_master WHERE type=?1 AND tbl_name=?2 AND sql IS NOT NULL"
            } else {
                "SELECT sql FROM sqlite_master WHERE type=?1 AND sql IS NOT NULL"
            };
            if let Ok(mut st) = self.conn.prepare(sql) {
                let txts: Vec<String> = if let Some(f) = only_table {
                    if kind != "view" {
                        st.query_map([kind, f], |r| r.get::<_, String>(0)).ok().map(|it| it.filter_map(|x| x.ok()).collect()).unwrap_or_default()
                    } else {
                        st.query_map([kind], |r| r.get::<_, String>(0)).ok().map(|it| it.filter_map(|x| x.ok()).collect()).unwrap_or_default()
                    }
                } else {
                    st.query_map([kind], |r| r.get::<_, String>(0)).ok().map(|it| it.filter_map(|x| x.ok()).collect()).unwrap_or_default()
                };
                v = txts;
            }
            for t in v {
                out.push_str(&format!("{};\n", t));
            }
        }
        out.push_str("\nCOMMIT;\n");
        out
    }

    /// Backup (works for both file and in-memory DBs) by writing a consistent
    /// snapshot file via VACUUM INTO.
    pub fn vacuum_into(&self, path: &str) -> Result<()> {
        let _ = std::fs::remove_file(path);
        self.conn.execute("VACUUM INTO ?1", [path])?;
        Ok(())
    }
}

pub fn quote_ident(name: &str) -> String {
    format!("\"{}\"", name.replace('"', "\"\""))
}

/// Split a SQL script into statements, respecting single/double quotes, line comments and block comments.
pub fn split_sql(script: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut chars = script.chars().peekable();
    let mut in_s = false;
    let mut in_d = false;
    let mut in_line = false;
    let mut in_block = false;
    while let Some(c) = chars.next() {
        if in_line {
            cur.push(c);
            if c == '\n' {
                in_line = false;
            }
            continue;
        }
        if in_block {
            cur.push(c);
            if c == '*' && chars.peek() == Some(&'/') {
                cur.push(chars.next().unwrap());
                in_block = false;
            }
            continue;
        }
        if in_s {
            cur.push(c);
            if c == '\'' {
                in_s = false;
            }
            continue;
        }
        if in_d {
            cur.push(c);
            if c == '"' {
                in_d = false;
            }
            continue;
        }
        match c {
            '\'' => {
                in_s = true;
                cur.push(c);
            }
            '"' => {
                in_d = true;
                cur.push(c);
            }
            '-' if chars.peek() == Some(&'-') => {
                in_line = true;
                cur.push(c);
            }
            '/' if chars.peek() == Some(&'*') => {
                cur.push(c);
                cur.push(chars.next().unwrap());
                in_block = true;
            }
            ';' => {
                out.push(cur.trim().to_string());
                cur.clear();
            }
            _ => cur.push(c),
        }
    }
    if !cur.trim().is_empty() {
        out.push(cur.trim().to_string());
    }
    out
}

pub fn build_create_table(columns: &[crate::model::Column]) -> Result<String> {
    if columns.is_empty() {
        return Err(anyhow!("no columns"));
    }
    let mut defs = Vec::new();
    for c in columns {
        let mut d = format!("{} {}", quote_ident(&c.name), if c.decl_type.is_empty() { "TEXT" } else { &c.decl_type });
        if c.pk {
            d.push_str(" PRIMARY KEY");
        }
        if c.notnull && !c.pk {
            d.push_str(" NOT NULL");
        }
        if let Some(def) = &c.default {
            if !def.is_empty() {
                d.push_str(&format!(" DEFAULT {}", def));
            }
        }
        defs.push(d);
    }
    Ok(defs.join(", "))
}
