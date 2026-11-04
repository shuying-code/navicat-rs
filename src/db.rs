use crate::model::{Column, DbObject, ObjKind}; // 引入模型类型
use anyhow::{anyhow, Result}; // 引入 anyhow 结果类型
use rusqlite::types::ValueRef; // 引入 rusqlite 值引用
use rusqlite::Connection as SqlConn; // 引入 rusqlite 连接
use std::time::Instant; // 引入计时器

pub struct SqliteEngine { // SQLite 引擎结构体
    pub conn: SqlConn, // 内部 rusqlite 连接
    pub path: String, // 数据库路径
} // 块结束

#[derive(Clone, Debug, Default)] // 查询结果结构体
pub struct QueryResult { // 列名
    pub columns: Vec<String>, // 行数据（字符串形式）
    pub rows: Vec<Vec<Option<String>>>, // 受影响行数
    pub affected: i64, // 耗时（毫秒）
    pub elapsed_ms: f64, // 错误信息
    pub error: Option<String>, // 原始语句
    pub statement: String, // 是否为 SELECT 查询
    pub is_select: bool, // 最后插入行 id
    pub last_insert_rowid: i64, // 字段 last_insert_rowid
} // 块结束

impl QueryResult { // 为 QueryResult 实现辅助方法
    fn err(statement: &str, e: impl std::fmt::Display) -> Self { // 构造一个错误结果
        QueryResult { // 构造
            error: Some(e.to_string()), // 错误信息
            statement: statement.to_string(), // 原语句
            ..Default::default() // 其余默认
        } // 结构结束
    } // 函数结束
} // impl 结束

fn val_to_opt(v: ValueRef<'_>) -> Option<String> { // 把 SQLite 值引用转为可显示字符串
    match v { // 匹配值类型
        ValueRef::Null => None, // NULL → None
        ValueRef::Integer(i) => Some(i.to_string()), // 整数
        ValueRef::Real(f) => { // 实数
            if f.fract() == 0.0 && f.is_finite() { // 若为有限整数值
                Some(format!("{}", f as i64)) // 用整数格式显示
            } else { // 否则
                Some(format!("{}", f)) // 用浮点格式显示
            } // if 结束
        } // 分支结束
        ValueRef::Text(t) => Some(String::from_utf8_lossy(t).to_string()), // 文本
        ValueRef::Blob(b) => Some(format!("[BLOB {} bytes]", b.len())), // BLOB 摘要
    } // match 结束
} // 函数结束

impl SqliteEngine { // 为 SqliteEngine 实现
    pub fn open(path: &str) -> Result<Self> { // 打开磁盘数据库
        let conn = SqlConn::open(path)?; // 打开连接
        conn.pragma_update(None, "foreign_keys", "ON").ok(); // 启用外键约束
        Ok(SqliteEngine { conn, path: path.to_string() }) // 返回引擎
    } // 结束

    pub fn open_memory_seeded() -> Result<Self> { // 打开并填充演示数据的内存库
        let conn = SqlConn::open_in_memory()?; // 打开内存连接
        let eng = SqliteEngine { conn, path: ":memory:".into() }; // 构造引擎
        eng.seed_demo()?; // 填充演示数据
        Ok(eng) // 返回引擎
    } // 结束

    pub fn seed_demo(&self) -> Result<()> { // 填充演示数据
        self.conn.execute_batch( // 批量执行建表与插入语句
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
            "#, // 语句
        )?; // 批量执行结束
        Ok(()) // 返回成功
    } // 结束

    pub fn objects(&self) -> Vec<DbObject> { // 列出所有表与视图对象
        let mut out = Vec::new(); // 结果向量
        let sql = "SELECT name, type FROM sqlite_master WHERE type IN ('table','view') AND name NOT LIKE 'sqlite_%' ORDER BY type DESC, name"; // 查询 sqlite_master
        if let Ok(mut st) = self.conn.prepare(sql) { // 准备语句
            if let Ok(rows) = st.query_map([], |r| { // 查询并映射
                Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)) // 取名称与类型
            }) { // 映射结束
                for row in rows.flatten() { // 遍历行
                    let kind = if row.1 == "view" { ObjKind::View } else { ObjKind::Table }; // 判断对象类型（view/table）
                    out.push(DbObject { name: row.0, kind }); // 加入结果
                } // for 结束
            } // if 结束
        } // if 结束
        out // 返回结果
    } // 结束

    pub fn columns(&self, table: &str) -> Vec<Column> { // 列出表的所有列
        let mut out = Vec::new(); // 结果向量
        let sql = format!("PRAGMA table_info({})", quote_ident(table)); // 构造 PRAGMA 语句
        if let Ok(mut st) = self.conn.prepare(&sql) { // 准备语句
            if let Ok(rows) = st.query_map([], |r| { // 查询并映射为 Column
                Ok(Column { // 构造 Column
                    cid: r.get::<_, i64>(0)?, // 列序号
                    name: r.get::<_, String>(1)?, // 列名
                    decl_type: r.get::<_, String>(2).unwrap_or_default(), // 声明类型
                    notnull: r.get::<_, i64>(3).unwrap_or(0) != 0, // 是否非空
                    default: r.get::<_, Option<String>>(4).ok().flatten(), // 默认值
                    pk: r.get::<_, i64>(5).unwrap_or(0) != 0, // 是否主键
                }) // 结构结束
            }) { // 映射结束
                for row in rows.flatten() { // 遍历行
                    out.push(row); // 加入结果
                } // for 结束
            } // if 结束
        } // if 结束
        out // 返回结果
    } // 结束

    pub fn table_has_rowid(&self, table: &str) -> bool { // 判断表是否有 rowid
        let sql = format!("SELECT rowid FROM {} LIMIT 1", quote_ident(table)); // 构造探测语句
        self.conn.prepare(&sql).is_ok() // 能否准备语句即视为有 rowid
    } // 结束

    pub fn count(&self, table: &str) -> i64 { // 统计表行数
        let sql = format!("SELECT COUNT(*) FROM {}", quote_ident(table)); // 构造 COUNT 语句
        self.conn.query_row(&sql, [], |r| r.get::<_, i64>(0)).unwrap_or(0) // 查询并返回行数
    } // 结束

    pub fn query(&self, sql: &str) -> QueryResult { // 执行查询并返回结果集
        let t0 = Instant::now(); // 计时开始
        let mut st = match self.conn.prepare(sql) { // 准备语句
            Ok(s) => s, // 成功
            Err(e) => return QueryResult::err(sql, e), // 失败返回错误结果
        }; // match 结束
        let col_count = st.column_count(); // 列数
        if col_count == 0 { // 若为 0 列（非查询语句）
            drop(st); // 释放语句
            return self.execute(sql); // 改走执行路径
        } // if 结束
        let columns: Vec<String> = (0..col_count) // 收集列名
            .map(|i| st.column_name(i).unwrap_or("").to_string()) // 取第 i 列名
            .collect(); // collect 结束
        let mut rows_out = Vec::new(); // 输出行集合
        let mut rows = match st.query([]) { // 执行查询
            Ok(r) => r, // 成功
            Err(e) => return QueryResult::err(sql, e), // 失败返回错误结果
        }; // match 结束
        loop { // 循环取行
            match rows.next() { // 匹配下一行
                Ok(Some(row)) => { // 有行
                    let mut rec = Vec::with_capacity(col_count); // 预分配一条记录
                    for i in 0..col_count { // 遍历列
                        rec.push(val_to_opt(row.get_ref(i).unwrap_or(ValueRef::Null))); // 取值转字符串
                    } // for 结束
                    rows_out.push(rec); // 加入结果行
                } // 分支结束
                Ok(None) => break, // 无更多行则跳出
                Err(e) => return QueryResult::err(sql, e), // 出错返回错误
            } // match 结束
        } // 循环结束
        QueryResult { // 构造结果
            columns, // 列名
            rows: rows_out, // 行数据
            elapsed_ms: t0.elapsed().as_secs_f64() * 1000.0, // 耗时
            statement: sql.to_string(), // 语句
            is_select: true, // 标记为 SELECT
            ..Default::default() // 其余默认
        } // 结构结束
    } // 函数结束

    pub fn execute(&self, sql: &str) -> QueryResult { // 执行非查询语句
        let t0 = Instant::now(); // 计时开始
        match self.conn.execute(sql, []) { // 执行语句
            Ok(n) => QueryResult { // 成功
                affected: n as i64, // 受影响行数
                last_insert_rowid: self.conn.last_insert_rowid(), // 最后插入行 id
                elapsed_ms: t0.elapsed().as_secs_f64() * 1000.0, // 耗时
                statement: sql.to_string(), // 语句
                ..Default::default() // 其余默认
            }, // 结构结束
            Err(e) => QueryResult::err(sql, e), // 失败返回错误结果
        } // match 结束
    } // 函数结束

    pub fn execute_batch(&self, sql: &str) -> Result<()> { // 批量执行（无返回）
        self.conn.execute_batch(sql)?; // 执行批处理
        Ok(()) // 返回成功
    } // 结束

    pub fn run_script(&self, script: &str) -> Vec<QueryResult> { // 运行多语句脚本
        let stmts = split_sql(script); // 切分为语句列表
        let mut out = Vec::new(); // 结果集合
        for s in stmts { // 遍历语句
            if s.trim().is_empty() { // 跳过空语句
                continue; // continue
            } // if 结束
            out.push(self.query(&s)); // 执行并收集结果
        } // for 结束
        if out.is_empty() { // 若无任何结果
            out.push(QueryResult { statement: script.to_string(), ..Default::default() }); // 补一条空结果
        } // if 结束
        out // 返回结果集合
    } // 函数结束

    /// Return list of tables that reference others: (child_table, child_col, parent_table, parent_col)
    pub fn foreign_keys(&self) -> Vec<(String, String, String, String)> { // 列出外键关系
        let mut out = Vec::new(); // 结果向量
        let tables: Vec<String> = self.objects().into_iter().filter(|o| o.kind == ObjKind::Table).map(|o| o.name).collect(); // 取所有表名
        for t in tables { // 遍历表
            let sql = format!("PRAGMA foreign_key_list({})", quote_ident(&t)); // 构造 foreign_key_list 语句
            if let Ok(mut st) = self.conn.prepare(&sql) { // 准备语句
                if let Ok(rows) = st.query_map([], |r| { // 查询并映射
                    Ok(( // 父表
                        r.get::<_, String>(2).unwrap_or_default(), // 子列
                        r.get::<_, String>(3).unwrap_or_default(), // 父列
                        r.get::<_, String>(4).unwrap_or_default(), // 元组结束
                    )) // 映射结束
                }) { // 遍历行
                    for row in rows.flatten() { // 循环
                        // row = (parent_table, child_col, parent_col)
                        out.push((t.clone(), row.1, row.0, row.2)); // 加入关系元组（子表,子列,父表,父列）
                    } // for 结束
                } // if 结束
            } // if 结束
        } // for 结束
        out // 返回结果
    } // 函数结束

    pub fn update_cell(&self, table: &str, rowid: i64, col: &str, val: Option<&str>) -> Result<()> { // 更新单元格
        let sql = format!("UPDATE {} SET {} = ?1 WHERE rowid = ?2", quote_ident(table), quote_ident(col)); // 构造 UPDATE 语句
        match val { // 匹配新值
            Some(v) => self.conn.execute(&sql, rusqlite::params![v, rowid])?, // 有值则绑定
            None => self.conn.execute(&sql, rusqlite::params![rusqlite::types::Null, rowid])?, // 空则绑定 NULL
        }; // match 结束
        Ok(()) // 返回成功
    } // 结束

    pub fn delete_row(&self, table: &str, rowid: i64) -> Result<()> { // 删除行
        let sql = format!("DELETE FROM {} WHERE rowid = ?1", quote_ident(table)); // 构造 DELETE 语句
        self.conn.execute(&sql, rusqlite::params![rowid])?; // 执行语句
        Ok(()) // 返回成功
    } // 结束

    pub fn insert_row(&self, table: &str, cols: &[String], vals: &[Option<String>]) -> Result<i64> { // 插入一行并返回新 rowid
        let collist: Vec<String> = cols.iter().map(|c| quote_ident(c)).collect(); // 列名列表（加引号）
        let placeholders: Vec<String> = (1..=cols.len()).map(|i| format!("?{}", i)).collect(); // 占位符列表
        let sql = format!( // 构造 INSERT 语句
            "INSERT INTO {} ({}) VALUES ({})", // 表名
            quote_ident(table), // 列清单
            collist.join(", "), // 占位符
            placeholders.join(", ") // format 结束
        ); // 参数值列表
        let params: Vec<rusqlite::types::Value> = vals // 映射参数
            .iter() // 有值转文本
            .map(|v| match v { // 空转 NULL
                Some(s) => rusqlite::types::Value::Text(s.clone()), // match 结束
                None => rusqlite::types::Value::Null, // collect 结束
            }) // 执行插入
            .collect(); // 返回最后插入 rowid
        self.conn.execute(&sql, rusqlite::params_from_iter(params))?; // 结束
        Ok(self.conn.last_insert_rowid()) // 调用 Ok
    } // 块结束

    pub fn pragma_i64(&self, pragma: &str) -> i64 { // 读取整型 PRAGMA 值
        self.conn.query_row(&format!("PRAGMA {}", pragma), [], |r| r.get::<_, i64>(0)).unwrap_or(0) // 查询并返回
    } // 结束

    pub fn pragma_str(&self, pragma: &str) -> String { // 读取字符串 PRAGMA 值
        self.conn.query_row(&format!("PRAGMA {}", pragma), [], |r| r.get::<_, String>(0)).unwrap_or_default() // 查询并返回
    } // 结束

    pub fn integrity_check(&self) -> String { // 执行完整性检查
        self.conn // 查询 integrity_check
            .query_row("PRAGMA integrity_check", [], |r| r.get::<_, String>(0)) // 取结果
            .unwrap_or_else(|e| e.to_string()) // 出错返回错误字符串
    } // 结束

    pub fn sqlite_version(&self) -> String { // 获取 SQLite 版本
        self.conn.query_row("SELECT sqlite_version()", [], |r| r.get::<_, String>(0)).unwrap_or_default() // 查询并返回版本
    } // 结束

    pub fn db_file_size(&self) -> u64 { // 获取数据库文件大小
        std::fs::metadata(&self.path).map(|m| m.len()).unwrap_or(0) // 读取文件元数据
    } // 结束

    // ----- structure objects: indexes / triggers / views -----
    pub fn indexes(&self, table: &str) -> Vec<(String, String)> { // 列出表的索引
        self.master_list("index", Some(table)) // 调用 master_list(kind=index)
    } // 结束
    pub fn triggers_of(&self, table: &str) -> Vec<(String, String)> { // 列出某表的触发器
        self.master_list("trigger", Some(table)) // 调用 master_list(kind=trigger)
    } // 结束
    pub fn triggers(&self) -> Vec<(String, String)> { // 列出所有触发器
        self.master_list("trigger", None) // 调用 master_list(kind=trigger)
    } // 结束
    pub fn views_list(&self) -> Vec<(String, String)> { // 列出所有视图
        self.master_list("view", None) // 调用 master_list(kind=view)
    } // 结束
    fn master_list(&self, kind: &str, table: Option<&str>) -> Vec<(String, String)> { // 通用 sqlite_master 列表查询
        let mut v = Vec::new(); // 结果向量
        let sql = match table { // 按是否限定表选择 SQL
            Some(_) => "SELECT name, sql FROM sqlite_master WHERE type=?1 AND tbl_name=?2 AND sql IS NOT NULL ORDER BY name", // 限定表：按 tbl_name 过滤
            None => "SELECT name, sql FROM sqlite_master WHERE type=?1 AND sql IS NOT NULL ORDER BY name", // 不限定表
        }; // match 结束
        let mut st = match self.conn.prepare(sql) { Ok(s) => s, Err(_) => return v }; // 准备语句
        let mut collect = |rows: &mut rusqlite::Rows<'_>| { // 收集闭包
            while let Ok(Some(r)) = rows.next() { // 遍历行
                let n: String = r.get(0).unwrap_or_default(); // 名称
                let s: String = r.get(1).unwrap_or_default(); // SQL
                v.push((n, s)); // 加入结果
            } // while 结束
        }; // 闭包结束
        if let Some(t) = table { // 若有表名
            if let Ok(mut rows) = st.query([kind, t]) { collect(&mut rows); } // 带表名参数查询
        } else if let Ok(mut rows) = st.query([kind]) { collect(&mut rows); } // 否则
        v // 仅带 kind 查询
    } // if 结束
    pub fn object_sql(&self, kind: &str, name: &str) -> Option<String> { // 返回结果
        self.conn // 函数结束
            .query_row("SELECT sql FROM sqlite_master WHERE type=?1 AND name=?2", rusqlite::params![kind, name], |r| r.get::<_, String>(0)) // 查询某对象的定义 SQL
            .ok() // 查询 sqlite_master.sql
    } // 取结果字符串
    pub fn execute_ddl(&self, sql: &str) -> Result<()> { // 结束
        self.conn.execute_batch(sql)?; // 执行 DDL 语句
        Ok(()) // 执行批处理
    } // 返回成功
    pub fn create_index(&self, name: &str, table: &str, cols: &[String], unique: bool) -> Result<()> { // 结束
        let cl: Vec<String> = cols.iter().map(|c| quote_ident(c)).collect(); // 创建索引
        let sql = format!( // 列名列表
            "CREATE {}INDEX {} ON {} ({})", // 构造 CREATE INDEX
            if unique { "UNIQUE " } else { "" }, // 唯一前缀
            quote_ident(name), // 索引名
            quote_ident(table), // 表名
            cl.join(", ") // 列清单
        ); // format 结束
        self.conn.execute_batch(&sql)?; // 执行语句
        Ok(()) // 返回成功
    } // 结束
    pub fn create_view(&self, name: &str, select_sql: &str) -> Result<()> { // 创建视图
        let sql = format!("CREATE VIEW {} AS {}", quote_ident(name), select_sql); // 构造 CREATE VIEW
        self.conn.execute_batch(&sql)?; // 执行语句
        Ok(()) // 返回成功
    } // 结束
    pub fn drop_object(&self, kind: &str, name: &str) -> Result<()> { // 删除对象（索引/触发器/视图）
        // kind is a trusted constant: INDEX | TRIGGER | VIEW
        let k = match kind.to_ascii_uppercase().as_str() { // 规范化并校验 kind（白名单）
            "INDEX" => "INDEX", // 索引
            "TRIGGER" => "TRIGGER", // 触发器
            "VIEW" => "VIEW", // 视图
            _ => return Err(anyhow!("unsupported object kind: {kind}")), // 其它则报错
        }; // match 结束
        self.conn.execute_batch(&format!("DROP {} IF EXISTS {}", k, quote_ident(name)))?; // 执行 DROP IF EXISTS
        Ok(()) // 返回成功
    } // 结束

    /// Dump schema + data as a SQL script (optionally a single table).
    pub fn dump_sql(&self, only_table: Option<&str>) -> String { // 导出 SQL 脚本（可限定单表）
        let mut out = String::new(); // 输出缓冲
        out.push_str("-- Navicat-RS SQL dump\nPRAGMA foreign_keys=OFF;\nBEGIN TRANSACTION;\n\n"); // 写入头部语句
        // tables
        let tables: Vec<(String, String)> = { // 收集表 DDL
            let mut v = Vec::new(); // 结果向量
            if let Ok(mut st) = self.conn.prepare("SELECT name, sql FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name") { // 查询表建表语句
                if let Ok(mut rows) = st.query([]) { // 遍历行
                    while let Ok(Some(r)) = rows.next() { // 表名
                        let n: String = r.get(0).unwrap_or_default(); // 建表 SQL
                        let s: String = r.get(1).unwrap_or_default(); // 若限定表则跳过不匹配的
                        if let Some(f) = only_table { if f != n { continue; } } // 加入结果
                        v.push((n, s)); // while 结束
                    } // if 结束
                } // if 结束
            } // 返回结果
            v // 块结束
        }; // 块结束
        for (name, create) in &tables { // 遍历表
            if !create.is_empty() { // 若有建表语句
                out.push_str(&format!("{};\n", create)); // 写入建表语句
            } // if 结束
            let cols = self.columns(name); // 取该表列定义
            let colnames: Vec<String> = cols.iter().map(|c| quote_ident(&c.name)).collect(); // 列名（加引号）
            let q = self.query(&format!("SELECT * FROM {}", quote_ident(name))); // 查询该表数据
            for row in &q.rows { // 遍历数据行
                let vals: Vec<String> = row // 构造值列表
                    .iter() // 映射每值
                    .map(|v| match v { // 字符串转义单引号
                        Some(s) => format!("'{}'", s.replace('\'', "''")), // 空为 NULL
                        None => "NULL".into(), // match 结束
                    }) // collect 结束
                    .collect(); // 写入 INSERT 语句
                out.push_str(&format!("INSERT INTO {} ({}) VALUES ({});\n", quote_ident(name), colnames.join(", "), vals.join(", "))); // for 结束
            } // 写入空行
            out.push('\n'); // for 结束
        } // 块结束
        // views / indexes / triggers (skip auto indexes without sql)
        for kind in ["view", "index", "trigger"] { // 遍历 view/index/trigger
            if only_table.is_some() && kind != "view" { // 条件分支
                // indexes/triggers for a single table dump are handled below via tbl_name filter
            } // 块结束
            let mut v = Vec::new(); // 结果向量
            let sql = if only_table.is_some() && kind != "view" { // 按限定情况选择 SQL
                "SELECT sql FROM sqlite_master WHERE type=?1 AND tbl_name=?2 AND sql IS NOT NULL" // 限定表且非视图
            } else { // 否则
                "SELECT sql FROM sqlite_master WHERE type=?1 AND sql IS NOT NULL" // match 结束
            }; // 块结束
            if let Ok(mut st) = self.conn.prepare(sql) { // 准备语句
                let txts: Vec<String> = if let Some(f) = only_table { // 收集 SQL 文本
                    if kind != "view" { // 若限定表
                        st.query_map([kind, f], |r| r.get::<_, String>(0)).ok().map(|it| it.filter_map(|x| x.ok()).collect()).unwrap_or_default() // 非视图按表名过滤
                    } else { // 视图不过滤
                        st.query_map([kind], |r| r.get::<_, String>(0)).ok().map(|it| it.filter_map(|x| x.ok()).collect()).unwrap_or_default() // if 结束
                    } // 否则
                } else { // 不限表
                    st.query_map([kind], |r| r.get::<_, String>(0)).ok().map(|it| it.filter_map(|x| x.ok()).collect()).unwrap_or_default() // if 结束
                }; // if 结束
                v = txts; // 赋值结果
            } // if 结束
            for t in v { // 遍历并写入
                out.push_str(&format!("{};\n", t)); // 写入语句
            } // for 结束
        } // for 结束
        out.push_str("\nCOMMIT;\n"); // 写入 COMMIT
        out // 返回脚本
    } // 函数结束

    /// Backup (works for both file and in-memory DBs) by writing a consistent
    /// snapshot file via VACUUM INTO.
    pub fn vacuum_into(&self, path: &str) -> Result<()> { // 用 VACUUM INTO 备份快照
        let _ = std::fs::remove_file(path); // 先删除同名旧文件
        self.conn.execute("VACUUM INTO ?1", [path])?; // 执行 VACUUM INTO
        Ok(()) // 返回成功
    } // 结束
} // impl 结束

pub fn quote_ident(name: &str) -> String { // 给标识符加双引号并转义
    format!("\"{}\"", name.replace('"', "\"\"")) // 包裹并双写内部引号
} // 结束

/// Split a SQL script into statements, respecting single/double quotes, line comments and block comments.
pub fn split_sql(script: &str) -> Vec<String> { // 切分 SQL 脚本为语句列表
    let mut out = Vec::new(); // 结果列表
    let mut cur = String::new(); // 当前语句缓冲
    let mut chars = script.chars().peekable(); // 字符迭代器
    let mut in_s = false; // 单引号状态
    let mut in_d = false; // 双引号状态
    let mut in_line = false; // 行注释状态
    let mut in_block = false; // 块注释状态
    while let Some(c) = chars.next() { // 遍历字符
        if in_line { // 若在行注释中
            cur.push(c); // 原样推入
            if c == '\n' { // 遇换行则结束行注释
                in_line = false; // if 结束
            } // continue
            continue; // if 结束
        } // 若在块注释中
        if in_block { // 原样推入
            cur.push(c); // 遇 */ 则结束块注释
            if c == '*' && chars.peek() == Some(&'/') { // 推入 /
                cur.push(chars.next().unwrap()); // 结束块注释
                in_block = false; // if 结束
            } // continue
            continue; // if 结束
        } // 若在单引号内
        if in_s { // 原样推入
            cur.push(c); // 遇单引号则退出
            if c == '\'' { // if 结束
                in_s = false; // continue
            } // if 结束
            continue; // 若在双引号内
        } // 原样推入
        if in_d { // 遇双引号则退出
            cur.push(c); // if 结束
            if c == '"' { // continue
                in_d = false; // if 结束
            } // 匹配当前字符
            continue; // 单引号：进入单引号态
        } // 置位
        match c { // 推入
            '\'' => { // 分支结束
                in_s = true; // 双引号：进入双引号态
                cur.push(c); // 置位
            } // 推入
            '"' => { // 分支结束
                in_d = true; // 若遇 -- 则进入行注释
                cur.push(c); // 置行注释态
            } // 推入
            '-' if chars.peek() == Some(&'-') => { // 分支结束
                in_line = true; // 若遇 /* 则进入块注释
                cur.push(c); // 推入
            } // 推入 *
            '/' if chars.peek() == Some(&'*') => { // 置块注释态
                cur.push(c); // 分支结束
                cur.push(chars.next().unwrap()); // 遇分号则切分语句
                in_block = true; // 推入一条（去空白）
            } // 清空缓冲
            ';' => { // 分支结束
                out.push(cur.trim().to_string()); // 其它字符原样推入
                cur.clear(); // match 结束
            } // while 结束
            _ => cur.push(c), // 若末尾尚有残余语句
        } // 推入
    } // if 结束
    if !cur.trim().is_empty() { // 返回结果
        out.push(cur.trim().to_string()); // 函数结束
    } // 函数结束
    out // 语句
} // 块结束

pub fn build_create_table(columns: &[crate::model::Column]) -> Result<String> { // 由列定义生成建表列片段
    if columns.is_empty() { // 若无列则报错
        return Err(anyhow!("no columns")); // 返回错误
    } // if 结束
    let mut defs = Vec::new(); // 列定义列表
    for c in columns { // 遍历列
        let mut d = format!("{} {}", quote_ident(&c.name), if c.decl_type.is_empty() { "TEXT" } else { &c.decl_type }); // 列名与类型（默认 TEXT）
        if c.pk { // 若为主键
            d.push_str(" PRIMARY KEY"); // 追加 PRIMARY KEY
        } // if 结束
        if c.notnull && !c.pk { // 若非空且非主键
            d.push_str(" NOT NULL"); // 追加 NOT NULL
        } // if 结束
        if let Some(def) = &c.default { // 若有默认值
            if !def.is_empty() { // 若非空
                d.push_str(&format!(" DEFAULT {}", def)); // 追加 DEFAULT
            } // if 结束
        } // if 结束
        defs.push(d); // 加入定义
    } // for 结束
    Ok(defs.join(", ")) // 用逗号连接返回
} // 函数结束
