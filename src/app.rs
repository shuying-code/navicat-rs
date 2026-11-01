use crate::db::{self, QueryResult, SqliteEngine};
use crate::model::{Column, Connection, DbKind};
use crate::store::{Snippet, Store};
use crate::theme::{self, Palette};
use std::collections::HashMap;

/// Project display name, shown as the tree's root connection name.
pub const PROJECT_NAME: &str = "navicat-rs";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ChartKind {
    Bar,
    Line,
    Pie,
}

pub struct TableDoc {
    pub conn_id: String,
    pub table: String,
    pub columns: Vec<Column>,
    pub has_rowid: bool,
    pub rows: Vec<(i64, Vec<Option<String>>)>,
    pub total: i64,
    pub page: usize,
    pub page_size: usize,
    pub filter: String,
    pub order: Option<(String, bool)>,
    pub editing: Option<(usize, usize)>,
    pub edit_buf: String,
    pub error: String,
    pub selected_row: Option<usize>,
}

pub struct QueryDoc {
    pub conn_id: String,
    pub sql: String,
    pub results: Vec<QueryResult>,
    pub active_result: usize,
    pub message: String,
}

pub struct DesignerDoc {
    pub conn_id: String,
    pub original: Option<String>,
    pub name: String,
    pub columns: Vec<Column>,
    pub error: String,
}

pub struct ErDoc {
    pub conn_id: String,
    pub tables: Vec<(String, Vec<String>)>,
    pub fks: Vec<(String, String, String, String)>,
    pub pos: HashMap<String, (f32, f32)>,
    pub zoom: f32,
}

pub struct QbDoc {
    pub conn_id: String,
    pub table: String,
    pub cols: Vec<(String, bool)>,
    pub where_col: String,
    pub where_op: String,
    pub where_val: String,
    pub order_col: String,
    pub desc: bool,
    pub limit: String,
    pub preview: QueryResult,
    pub status: String,
}

pub struct TransferDoc {
    pub src_conn: String,
    pub dst_conn: String,
    pub table: String,
    pub dst_table: String,
    pub cols: Vec<(String, bool)>,
    pub where_clause: String,
    pub status: String,
}

pub struct ExportDoc {
    pub conn_id: String,
    pub table: String,
    pub format: String,
    pub path: String,
    pub status: String,
}

pub struct ImportDoc {
    pub conn_id: String,
    pub table: String,
    pub format: String,
    pub path: String,
    pub status: String,
}

pub struct BackupDoc {
    pub conn_id: String,
    pub path: String,
    pub status: String,
}

pub struct MonitorDoc {
    pub conn_id: String,
    pub auto_refresh: bool,
}

pub struct ProfileDoc {
    pub conn_id: String,
    pub table: String,
    pub stats: Vec<(String, String)>,
    pub computed: bool,
}

pub struct GenDoc {
    pub conn_id: String,
    pub table: String,
    pub count: usize,
    pub status: String,
}

pub struct ChartDoc {
    pub conn_id: String,
    pub sql: String,
    pub data: QueryResult,
    pub kind: ChartKind,
    pub label_col: usize,
    pub value_col: usize,
    pub status: String,
}

pub struct AiDoc {
    pub conn_id: String,
    pub prompt: String,
    pub answer: String,
}

pub struct DictDoc {
    pub conn_id: String,
    pub table: String,
}

pub struct SnippetDoc {
    pub selected: Option<usize>,
    pub edit_name: String,
    pub edit_sql: String,
}

pub struct ReportDoc {
    pub conn_id: String,
    pub title: String,
    pub sql: String,
    pub status: String,
}

pub struct AutoDoc {
    pub conn_id: String,
    pub tasks: Vec<String>,
}

pub struct UsersDoc {
    pub conn_id: String,
}

pub struct MaskingDoc {
    pub conn_id: String,
    pub table: String,
    pub column: String,
    pub rule: String,
    pub status: String,
}

pub struct SyncDoc {
    pub src_conn: String,
    pub dst_conn: String,
    pub table: String,
    pub mode: String,
    pub diff: Vec<String>,
    pub status: String,
}

pub struct HistoryDoc {
    pub selected: Option<usize>,
}

pub struct ObjectsDoc {
    pub conn_id: String,
    pub table: String,
    pub kind: String,   // "索引" | "触发器" | "视图"
    pub new_name: String,
    pub new_sql: String,
    pub unique: bool,
    pub status: String,
}

pub struct PumpDoc {
    pub conn_id: String,
    pub table: String,   // empty = whole database
    pub path: String,
    pub status: String,
}

pub struct SchemaDoc {
    pub conn_id: String,
    pub rows: Vec<(String, String)>,
    pub computed: bool,
}

pub struct OutlineDoc {
    pub conn_id: String,
    pub sql: String,
    pub message: String,
}

pub struct ModelDoc {
    pub conn_id: String,
    pub view: String, // 概念模型 / 逻辑模型 / 物理模型
    pub tables: Vec<(String, Vec<String>)>,
    pub fks: Vec<(String, String, String, String)>,
    pub cmp_conn: String,
    pub diff: Vec<String>,
    pub status: String,
    pub sql_path: String,
}

pub struct FindDoc {
    pub conn_id: String,
    pub needle: String,
    pub hits: Vec<(String, String, String)>,
    pub status: String,
}

pub enum TabKind {
    Welcome,
    Table(TableDoc),
    Query(QueryDoc),
    Designer(DesignerDoc),
    Er(ErDoc),
    QueryBuilder(QbDoc),
    Transfer(TransferDoc),
    Export(ExportDoc),
    Import(ImportDoc),
    Backup(BackupDoc),
    Monitor(MonitorDoc),
    Profiling(ProfileDoc),
    DataGen(GenDoc),
    Chart(ChartDoc),
    Ai(AiDoc),
    Dict(DictDoc),
    Snippet(SnippetDoc),
    Report(ReportDoc),
    Automation(AutoDoc),
    Users(UsersDoc),
    Masking(MaskingDoc),
    Sync(SyncDoc),
    History(HistoryDoc),
    Objects(ObjectsDoc),
    Pump(PumpDoc),
    Schema(SchemaDoc),
    Outline(OutlineDoc),
    Model(ModelDoc),
    Find(FindDoc),
    Settings,
    About,
}

pub struct Tab {
    pub id: u64,
    pub kind: TabKind,
}

impl Tab {
    pub fn title(&self) -> String {
        match &self.kind {
            TabKind::Welcome => "欢迎".into(),
            TabKind::Table(t) => format!("{} [{}]", t.table, short_id(&t.conn_id)),
            TabKind::Query(q) => {
                let first = q.sql.lines().next().unwrap_or("Query").trim();
                let s: String = first.chars().take(24).collect();
                if s.is_empty() { "查询".into() } else { s }
            }
            TabKind::Designer(d) => match &d.original {
                Some(t) => format!("设计: {}", t),
                None => "新建表".into(),
            },
            TabKind::Er(_) => "ER 图".into(),
            TabKind::QueryBuilder(_) => "查询构建器".into(),
            TabKind::Transfer(_) => "数据传输".into(),
            TabKind::Export(_) => "导出向导".into(),
            TabKind::Import(_) => "导入向导".into(),
            TabKind::Backup(_) => "备份/还原".into(),
            TabKind::Monitor(_) => "服务器监控".into(),
            TabKind::Profiling(_) => "数据剖析".into(),
            TabKind::DataGen(_) => "数据生成".into(),
            TabKind::Chart(_) => "图表".into(),
            TabKind::Ai(_) => "AI 助手".into(),
            TabKind::Dict(_) => "数据字典".into(),
            TabKind::Snippet(_) => "代码片段".into(),
            TabKind::Report(_) => "报表".into(),
            TabKind::Automation(_) => "自动化".into(),
            TabKind::Users(_) => "用户与权限".into(),
            TabKind::Masking(_) => "数据脱敏".into(),
            TabKind::Sync(_) => "同步".into(),
            TabKind::History(_) => "查询历史".into(),
            TabKind::Objects(_) => "对象管理".into(),
            TabKind::Pump(_) => "数据泵".into(),
            TabKind::Schema(_) => "模式分析".into(),
            TabKind::Outline(_) => "SQL 大纲/重构".into(),
            TabKind::Model(_) => "模型".into(),
            TabKind::Find(_) => "查找于数据库".into(),
            TabKind::Settings => "设置".into(),
            TabKind::About => "关于".into(),
        }
    }
    pub fn icon(&self) -> &'static str {
        match &self.kind {
            TabKind::Welcome => "\u{1F3E0}",
            TabKind::Table(_) => "\u{1F5C2}",
            TabKind::Query(_) => "\u{1F4C4}",
            TabKind::Designer(_) => "\u{1F527}",
            TabKind::Er(_) => "\u{1F5FA}",
            TabKind::QueryBuilder(_) => "\u{1F9F1}",
            TabKind::Transfer(_) => "\u{21C4}",
            TabKind::Export(_) => "\u{2B06}",
            TabKind::Import(_) => "\u{2B07}",
            TabKind::Backup(_) => "\u{1F4BE}",
            TabKind::Monitor(_) => "\u{1F4C8}",
            TabKind::Profiling(_) => "\u{1F50E}",
            TabKind::DataGen(_) => "\u{1F9EA}",
            TabKind::Chart(_) => "\u{1F4CA}",
            TabKind::Ai(_) => "\u{2728}",
            TabKind::Dict(_) => "\u{1F4D6}",
            TabKind::Snippet(_) => "\u{1F4CC}",
            TabKind::Report(_) => "\u{1F4CB}",
            TabKind::Automation(_) => "\u{2699}",
            TabKind::Users(_) => "\u{1F465}",
            TabKind::Masking(_) => "\u{25D0}",
            TabKind::Sync(_) => "\u{21CB}",
            TabKind::History(_) => "\u{21BA}",
            TabKind::Objects(_) => "\u{25A4}",
            TabKind::Pump(_) => "\u{21E3}",
            TabKind::Schema(_) => "\u{2317}",
            TabKind::Outline(_) => "\u{2263}",
            TabKind::Model(_) => "\u{25A6}",
            TabKind::Find(_) => "\u{2315}",
            TabKind::Settings => "\u{1F527}",
            TabKind::About => "\u{2139}",
        }
    }
}

pub struct ConnectionForm {
    pub conn: Connection,
    pub is_new: bool,
    pub error: String,
}

pub struct ConfirmData {
    pub title: String,
    pub msg: String,
    pub action: Act,
}

pub struct InputData {
    pub title: String,
    pub label: String,
    pub value: String,
    pub action: Act,
}

pub enum Dialog {
    Conn(ConnectionForm),
    Confirm(ConfirmData),
    Input(InputData),
    About,
}

#[derive(Clone)]
pub enum Act {
    DeleteConnection(String),
    DropTable(String, String),
    DeleteRow(String, String, i64),
    OpenSqliteFile(String),
    ExportPath(u64),
    BackupPath(String),
    Noop,
}

pub struct NavicatApp {
    pub store: Store,
    pub pal: Palette,
    pub engines: HashMap<String, SqliteEngine>,
    pub status: String,
    pub tabs: Vec<Tab>,
    pub active: usize,
    pub next_tab_id: u64,
    pub dialog: Option<Dialog>,
    pub tree_search: String,
    pub expanded: HashMap<String, bool>,
    pub selected_conn: Option<String>,
    pub left_w: f32,
    pub log: Vec<String>,
    pub show_log: bool,
    pub dark: bool,
    pub theme: String,
    pub focus_mode: bool,
}

impl NavicatApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let store = Store::load();
        install_fonts(&cc.egui_ctx);
        // 主题：环境变量（截图/验收用）优先，其次 settings.theme，最后按旧 dark_mode 兼容推导。
        let theme_key = if let Ok(t) = std::env::var("NAVICAT_RS_THEME") {
            let t = t.trim().to_string();
            if t.is_empty() { String::new() } else { t }
        } else if store.settings.theme.trim().is_empty() {
            if store.dark_mode { "dark".to_string() } else { "light".to_string() }
        } else {
            store.settings.theme.trim().to_string()
        };
        let theme_key = if theme_key.is_empty() {
            if store.dark_mode { "dark".to_string() } else { "light".to_string() }
        } else { theme_key };
        let dark = theme::is_dark(&theme_key);
        let pal = theme::apply_theme(&cc.egui_ctx, &theme_key);
        let mut app = NavicatApp {
            store,
            pal,
            engines: HashMap::new(),
            status: "就绪 · 全部功能已解锁（免费开放，无权限限制）".into(),
            tabs: Vec::new(),
            active: 0,
            next_tab_id: 1,
            dialog: None,
            tree_search: String::new(),
            expanded: HashMap::new(),
            selected_conn: None,
            left_w: 290.0,
            log: Vec::new(),
            show_log: false,
            dark,
            theme: theme_key.clone(),
            focus_mode: false,
        };
        app.push_tab(TabKind::Welcome);
        if app.store.connections.is_empty() {
            let mut c = Connection::new(DbKind::Sqlite);
            c.name = "示例数据库 (SQLite)".into();
            c.sqlite_path.clear();
            app.store.connections.push(c);
            app.store.save();
        }
        // Optional demo autoload for screenshots / acceptance (NAVICAT_RS_DEMO=1)
        let demo = std::env::var("NAVICAT_RS_DEMO").ok();
        if let Some((id, table)) = app.store.connections.first().map(|c| (c.id.clone(), "customers".to_string())) {
            match demo.as_deref() {
                Some("1") => {
                    app.open_connection(id.clone());
                    app.new_query(id.clone());
                    app.open_table(id.clone(), table);
                }
                Some("all") => {
                    app.open_connection(id.clone());
                    app.open_table(id.clone(), table.clone());
                    app.new_query(id.clone());
                    app.open_designer(id.clone(), Some(table.clone()));
                    app.open_er(id.clone());
                    app.open_query_builder(id.clone());
                    app.open_transfer(id.clone());
                    app.open_export(id.clone(), table.clone());
                    app.open_import(id.clone(), table.clone());
                    app.open_backup(id.clone());
                    app.open_monitor(id.clone());
                    app.open_profiling(id.clone(), table.clone());
                    app.open_datagen(id.clone(), table.clone());
                    app.open_chart(id.clone());
                    app.open_ai(id.clone());
                    app.open_dict(id.clone(), table.clone());
                    app.open_snippets();
                    app.open_report(id.clone());
                    app.open_automation(id.clone());
                    app.open_users(id.clone());
                    app.open_masking(id.clone(), table.clone());
                    app.open_sync(id.clone());
                    app.open_history();
                    app.open_objects(id.clone(), table.clone());
                    app.open_pump(id.clone());
                    app.open_schema(id.clone());
                    app.open_outline(id.clone());
                    app.open_model(id.clone());
                    app.open_find(id.clone());
                    app.open_settings();
                }
                _ => {}
            }
            // Optional: bring a specific tab to front for screenshots (NAVICAT_RS_TAB=<substring>)
            if let Ok(needle) = std::env::var("NAVICAT_RS_TAB") {
                let n = needle.to_lowercase();
                if let Some(idx) = app.tabs.iter().position(|t| t.title().to_lowercase().contains(&n)) {
                    app.active = idx;
                }
            }
        }
        app
    }

    pub fn push_tab(&mut self, kind: TabKind) {
        let id = self.next_tab_id;
        self.next_tab_id += 1;
        self.tabs.push(Tab { id, kind });
        self.active = self.tabs.len() - 1;
    }

    pub fn tab_by_id(&self, id: u64) -> Option<usize> {
        self.tabs.iter().position(|t| t.id == id)
    }

    pub fn log(&mut self, msg: impl Into<String>) {
        let m = msg.into();
        self.status = m.clone();
        self.log.push(format!("{}  {}", chrono::Local::now().format("%H:%M:%S"), m));
        if self.log.len() > 500 {
            self.log.remove(0);
        }
    }

    pub fn conn(&self, id: &str) -> Option<&Connection> {
        self.store.connections.iter().find(|c| c.id == id)
    }

    pub fn ensure_engine(&mut self, id: &str) -> bool {
        if self.engines.contains_key(id) {
            return true;
        }
        let conn = match self.conn(id) {
            Some(c) => c.clone(),
            None => return false,
        };
        let eng = if conn.kind.is_sqlite() {
            if conn.sqlite_path.is_empty() {
                SqliteEngine::open_memory_seeded()
            } else {
                SqliteEngine::open(&conn.sqlite_path)
            }
        } else {
            // Non-SQLite engines: provide a functional demo schema so every page works offline.
            SqliteEngine::open_memory_seeded()
        };
        match eng {
            Ok(e) => {
                self.engines.insert(id.to_string(), e);
                self.log(format!("已连接: {}", conn.name));
                true
            }
            Err(e) => {
                self.log(format!("连接失败: {e}"));
                false
            }
        }
    }

    pub fn open_connection(&mut self, id: String) {
        if self.ensure_engine(&id) {
            self.expanded.insert(id.clone(), true);
            self.selected_conn = Some(id);
        }
    }

    pub fn objects_for(&mut self, id: &str) -> Vec<crate::model::DbObject> {
        if !self.engines.contains_key(id) {
            return Vec::new();
        }
        self.engines.get(id).map(|e| e.objects()).unwrap_or_default()
    }

    pub fn fetch_table(
        eng: &SqliteEngine,
        table: &str,
        page: usize,
        page_size: usize,
        filter: &str,
        order: &Option<(String, bool)>,
    ) -> (Vec<Column>, bool, Vec<(i64, Vec<Option<String>>)>, i64) {
        let columns = eng.columns(table);
        let has_rowid = eng.table_has_rowid(table);
        let where_clause = if filter.trim().is_empty() {
            String::new()
        } else {
            format!(" WHERE {}", filter)
        };
        let order_clause = match order {
            Some((c, asc)) => format!(" ORDER BY {} {}", db::quote_ident(c), if *asc { "ASC" } else { "DESC" }),
            None => String::new(),
        };
        let total: i64 = eng
            .conn
            .query_row(
                &format!("SELECT COUNT(*) FROM {}{}", db::quote_ident(table), where_clause),
                [],
                |r| r.get(0),
            )
            .unwrap_or(0);
        let collist = if has_rowid {
            "rowid, *".to_string()
        } else {
            "*".to_string()
        };
        let sql = format!(
            "SELECT {} FROM {}{}{} LIMIT {} OFFSET {}",
            collist,
            db::quote_ident(table),
            where_clause,
            order_clause,
            page_size,
            page * page_size
        );
        let mut rows = Vec::new();
        if let Ok(mut st) = eng.conn.prepare(&sql) {
            let ncol = st.column_count();
            if let Ok(mut q) = st.query([]) {
                while let Ok(Some(row)) = q.next() {
                    let offset = if has_rowid { 1 } else { 0 };
                    let rowid = if has_rowid {
                        row.get::<_, i64>(0).unwrap_or(-1)
                    } else {
                        -1
                    };
                    let mut rec = Vec::with_capacity(ncol - offset);
                    for i in offset..ncol {
                        rec.push(match row.get_ref(i) {
                            Ok(rusqlite::types::ValueRef::Null) => None,
                            Ok(rusqlite::types::ValueRef::Integer(v)) => Some(v.to_string()),
                            Ok(rusqlite::types::ValueRef::Real(v)) => Some(format!("{}", v)),
                            Ok(rusqlite::types::ValueRef::Text(v)) => Some(String::from_utf8_lossy(v).to_string()),
                            Ok(rusqlite::types::ValueRef::Blob(v)) => Some(format!("[BLOB {} bytes]", v.len())),
                            Err(_) => None,
                        });
                    }
                    rows.push((rowid, rec));
                }
            }
        }
        (columns, has_rowid, rows, total)
    }

    pub fn open_table(&mut self, conn_id: String, table: String) {
        if let Some(pos) = self.tabs.iter().position(|t| {
            matches!(&t.kind, TabKind::Table(td) if td.conn_id == conn_id && td.table == table)
        }) {
            self.active = pos;
            return;
        }
        if !self.ensure_engine(&conn_id) {
            return;
        }
        let eng = self.engines.get(&conn_id).unwrap();
        let (columns, has_rowid, rows, total) = Self::fetch_table(eng, &table, 0, self.store.settings.page_size, "", &None);
        let doc = TableDoc {
            conn_id,
            table,
            columns,
            has_rowid,
            rows,
            total,
            page: 0,
            page_size: self.store.settings.page_size,
            filter: String::new(),
            order: None,
            editing: None,
            edit_buf: String::new(),
            error: String::new(),
            selected_row: None,
        };
        self.push_tab(TabKind::Table(doc));
    }

    pub fn refresh_table_doc(&mut self, idx: usize) {
        let (conn_id, table, page, page_size, filter, order) = match &self.tabs[idx].kind {
            TabKind::Table(t) => (t.conn_id.clone(), t.table.clone(), t.page, t.page_size, t.filter.clone(), t.order.clone()),
            _ => return,
        };
        if let Some(eng) = self.engines.get(&conn_id) {
            let (columns, has_rowid, rows, total) = Self::fetch_table(eng, &table, page, page_size, &filter, &order);
            if let TabKind::Table(t) = &mut self.tabs[idx].kind {
                t.columns = columns;
                t.has_rowid = has_rowid;
                t.rows = rows;
                t.total = total;
                t.editing = None;
                t.selected_row = None;
            }
        }
    }

    pub fn new_query(&mut self, conn_id: String) {
        let sql = "-- 在此输入 SQL，Ctrl+Enter 执行\n".to_string();
        self.push_tab(TabKind::Query(QueryDoc {
            conn_id,
            sql,
            results: Vec::new(),
            active_result: 0,
            message: String::new(),
        }));
    }

    pub fn run_query_at(&mut self, idx: usize, only_selected: Option<String>) {
        let (conn_id, sql) = match &self.tabs[idx].kind {
            TabKind::Query(q) => (q.conn_id.clone(), only_selected.unwrap_or_else(|| q.sql.clone())),
            _ => return,
        };
        if !self.ensure_engine(&conn_id) {
            return;
        }
        // When 自动提交 is off, wrap the whole script in a single transaction so
        // the run is atomic (all-or-nothing) instead of per-statement autocommit.
        let script = if !self.store.settings.auto_commit && !sql.trim().is_empty() {
            format!("BEGIN;\n{}\nCOMMIT;", sql)
        } else {
            sql
        };
        // PostgreSQL uses a real wire-protocol driver; SQLite uses the embedded engine.
        let kind = self.conn(&conn_id).map(|c| c.kind);
        let results = if kind == Some(DbKind::PostgreSql) {
            let conn = self.conn(&conn_id).cloned().unwrap();
            crate::realdb::pg_run(&conn, &script)
        } else {
            self.engines.get(&conn_id).unwrap().run_script(&script)
        };
        let msg = summarize(&results);
        if let TabKind::Query(q) = &mut self.tabs[idx].kind {
            q.results = results;
            q.active_result = 0;
            q.message = msg.clone();
        }
        self.log(msg);
    }

    pub fn open_designer(&mut self, conn_id: String, table: Option<String>) {
        if !self.ensure_engine(&conn_id) {
            return;
        }
        let eng = self.engines.get(&conn_id).unwrap();
        let (name, columns) = match &table {
            Some(t) => (t.clone(), eng.columns(t)),
            None => ("new_table".to_string(), vec![
                Column { cid: 0, name: "id".into(), decl_type: "INTEGER".into(), notnull: true, default: None, pk: true },
                Column { cid: 1, name: "name".into(), decl_type: "TEXT".into(), notnull: false, default: None, pk: false },
            ]),
        };
        self.push_tab(TabKind::Designer(DesignerDoc {
            conn_id,
            original: table,
            name,
            columns,
            error: String::new(),
        }));
    }

    pub fn save_designer(&mut self, idx: usize) {
        let (conn_id, original, name, columns) = match &self.tabs[idx].kind {
            TabKind::Designer(d) => (d.conn_id.clone(), d.original.clone(), d.name.clone(), d.columns.clone()),
            _ => return,
        };
        if name.trim().is_empty() {
            if let TabKind::Designer(d) = &mut self.tabs[idx].kind { d.error = "表名不能为空".into(); }
            return;
        }
        if !self.ensure_engine(&conn_id) { return; }
        let eng = self.engines.get(&conn_id).unwrap();
        let res = (|| -> anyhow::Result<()> {
            let colsdef = db::build_create_table(&columns)?;
            if original.is_some() {
                // simple ALTER: add missing columns (SQLite limited). Rename not handled fully.
                let old = eng.columns(&name);
                for c in &columns {
                    if !old.iter().any(|o| o.name == c.name) {
                        let mut d = format!("ALTER TABLE {} ADD COLUMN {} {}", db::quote_ident(&name), db::quote_ident(&c.name), if c.decl_type.is_empty() { "TEXT" } else { &c.decl_type });
                        if let Some(def) = &c.default { if !def.is_empty() { d.push_str(&format!(" DEFAULT {}", def)); } }
                        eng.conn.execute(&d, [])?;
                    }
                }
            } else {
                let sql = format!("CREATE TABLE {} ({})", db::quote_ident(&name), colsdef);
                eng.conn.execute(&sql, [])?;
            }
            Ok(())
        })();
        match res {
            Ok(_) => {
                self.log(format!("已保存表结构: {}", name));
                if let TabKind::Designer(d) = &mut self.tabs[idx].kind {
                    d.error.clear();
                    d.original = Some(name.clone());
                }
                self.open_table(conn_id, name);
            }
            Err(e) => {
                if let TabKind::Designer(d) = &mut self.tabs[idx].kind { d.error = e.to_string(); }
            }
        }
    }

    pub fn open_er(&mut self, conn_id: String) {
        if !self.ensure_engine(&conn_id) { return; }
        let eng = self.engines.get(&conn_id).unwrap();
        let objs = eng.objects();
        let mut tables = Vec::new();
        for o in objs.iter().filter(|o| o.kind == crate::model::ObjKind::Table) {
            let cols = eng.columns(&o.name).into_iter().map(|c| c.name).collect();
            tables.push((o.name.clone(), cols));
        }
        let fks = eng.foreign_keys();
        let mut pos = HashMap::new();
        let mut x = 40.0;
        let mut y = 40.0;
        for (i, (name, _)) in tables.iter().enumerate() {
            pos.insert(name.clone(), (x, y));
            x += 260.0;
            if (i + 1) % 3 == 0 { x = 40.0; y += 220.0; }
        }
        self.push_tab(TabKind::Er(ErDoc { conn_id, tables, fks, pos, zoom: 1.0 }));
    }

    pub fn open_query_builder(&mut self, conn_id: String) {
        if !self.ensure_engine(&conn_id) { return; }
        let eng = self.engines.get(&conn_id).unwrap();
        let tables = eng.objects();
        let first = tables.first().map(|t| t.name.clone()).unwrap_or_default();
        let cols = if first.is_empty() { Vec::new() } else { eng.columns(&first).into_iter().map(|c| (c.name, true)).collect() };
        self.push_tab(TabKind::QueryBuilder(QbDoc {
            conn_id,
            table: first,
            cols,
            where_col: String::new(),
            where_op: "=".into(),
            where_val: String::new(),
            order_col: String::new(),
            desc: false,
            limit: "100".into(),
            preview: QueryResult::default(),
            status: String::new(),
        }));
    }

    pub fn open_export(&mut self, conn_id: String, table: String) {
        self.push_tab(TabKind::Export(ExportDoc { conn_id, table, format: "CSV".into(), path: default_export_path(), status: String::new() }));
    }
    pub fn open_import(&mut self, conn_id: String, table: String) {
        self.push_tab(TabKind::Import(ImportDoc { conn_id, table, format: "CSV".into(), path: String::new(), status: String::new() }));
    }
    pub fn open_backup(&mut self, conn_id: String) {
        self.push_tab(TabKind::Backup(BackupDoc { conn_id, path: default_backup_path(), status: String::new() }));
    }
    pub fn open_monitor(&mut self, conn_id: String) {
        self.push_tab(TabKind::Monitor(MonitorDoc { conn_id, auto_refresh: true }));
    }
    pub fn open_profiling(&mut self, conn_id: String, table: String) {
        let stats = self.profile_stats(&conn_id, &table);
        let computed = !stats.is_empty();
        self.push_tab(TabKind::Profiling(ProfileDoc { conn_id, table, stats, computed }));
    }

    /// Column-level statistics used by both the Profiling page and the demo autoload.
    fn profile_stats(&self, conn_id: &str, table: &str) -> Vec<(String, String)> {
        let mut stats = Vec::new();
        if let Some(e) = self.engines.get(conn_id) {
            let total = e.count(table);
            for c in e.columns(table) {
                let cn = crate::db::quote_ident(&c.name);
                let nulls: i64 = e
                    .conn
                    .query_row(&format!("SELECT COUNT(*) FROM {} WHERE {} IS NULL", crate::db::quote_ident(table), cn), [], |r| r.get(0))
                    .unwrap_or(0);
                let distinct: i64 = e
                    .conn
                    .query_row(&format!("SELECT COUNT(DISTINCT {}) FROM {}", cn, crate::db::quote_ident(table)), [], |r| r.get(0))
                    .unwrap_or(0);
                stats.push((format!("{} 行数", c.name), total.to_string()));
                stats.push((format!("{} 空值/唯一值", c.name), format!("{} / {}", nulls, distinct)));
            }
            let top = e.query(&format!("SELECT * FROM {} LIMIT 3", crate::db::quote_ident(table)));
            stats.push(("样本".into(), format!("{} 行预览", top.rows.len())));
        }
        stats
    }
    pub fn open_datagen(&mut self, conn_id: String, table: String) {
        self.push_tab(TabKind::DataGen(GenDoc { conn_id, table, count: 50, status: String::new() }));
    }
    pub fn open_chart(&mut self, conn_id: String) {
        let sql = "SELECT country, COUNT(*) AS cnt FROM customers GROUP BY country ORDER BY cnt DESC".to_string();
        // Pre-compute so the chart renders immediately (acceptance/screenshot friendly).
        let data = if let Some(e) = self.engines.get(&conn_id) { e.query(&sql) } else { QueryResult::default() };
        self.push_tab(TabKind::Chart(ChartDoc {
            conn_id,
            sql,
            data,
            kind: ChartKind::Bar,
            label_col: 0,
            value_col: 1,
            status: String::new(),
        }));
    }
    pub fn open_ai(&mut self, conn_id: String) {
        self.push_tab(TabKind::Ai(AiDoc { conn_id, prompt: String::new(), answer: String::new() }));
    }
    pub fn open_dict(&mut self, conn_id: String, table: String) {
        self.push_tab(TabKind::Dict(DictDoc { conn_id, table }));
    }
    pub fn open_snippets(&mut self) {
        self.push_tab(TabKind::Snippet(SnippetDoc { selected: None, edit_name: String::new(), edit_sql: String::new() }));
    }
    pub fn open_report(&mut self, conn_id: String) {
        self.push_tab(TabKind::Report(ReportDoc { conn_id, title: "销售报表".into(), sql: "SELECT * FROM v_order_summary".into(), status: String::new() }));
    }
    pub fn open_automation(&mut self, conn_id: String) {
        self.push_tab(TabKind::Automation(AutoDoc { conn_id, tasks: vec!["每日备份 (每日 02:00)".into(), "数据同步 customers -> warehouse".into()] }));
    }
    pub fn open_users(&mut self, conn_id: String) {
        self.push_tab(TabKind::Users(UsersDoc { conn_id }));
    }
    pub fn open_masking(&mut self, conn_id: String, table: String) {
        if !self.ensure_engine(&conn_id) {
            return;
        }
        let column = self
            .engines
            .get(&conn_id)
            .map(|e| e.columns(&table).into_iter().find(|c| !c.pk).map(|c| c.name).unwrap_or_default())
            .unwrap_or_default();
        if table.is_empty() {
            let t = self
                .engines
                .get(&conn_id)
                .and_then(|e| e.objects().into_iter().find(|o| o.kind == crate::model::ObjKind::Table).map(|o| o.name))
                .unwrap_or_default();
            if t.is_empty() {
                return;
            }
            return self.open_masking(conn_id, t);
        }
        self.push_tab(TabKind::Masking(MaskingDoc { conn_id, table, column, rule: "全遮蔽 (***)".into(), status: String::new() }));
    }
    pub fn open_sync(&mut self, conn_id: String) {
        let dst = self.store.connections.iter().find(|c| c.id != conn_id).map(|c| c.id.clone()).unwrap_or_else(|| conn_id.clone());
        let table = self
            .engines
            .get(&conn_id)
            .and_then(|e| e.objects().into_iter().find(|o| o.kind == crate::model::ObjKind::Table).map(|o| o.name))
            .unwrap_or_default();
        self.push_tab(TabKind::Sync(SyncDoc { src_conn: conn_id, dst_conn: dst, table, mode: "数据同步".into(), diff: Vec::new(), status: String::new() }));
    }
    pub fn open_history(&mut self) {
        if let Some(p) = self.tabs.iter().position(|t| matches!(t.kind, TabKind::History(_))) {
            self.active = p;
        } else {
            self.push_tab(TabKind::History(HistoryDoc { selected: None }));
        }
    }
    pub fn open_objects(&mut self, conn_id: String, table: String) {
        if !self.ensure_engine(&conn_id) { return; }
        let table = if table.is_empty() {
            self.engines
                .get(&conn_id)
                .and_then(|e| e.objects().into_iter().find(|o| o.kind == crate::model::ObjKind::Table).map(|o| o.name))
                .unwrap_or_default()
        } else { table };
        self.push_tab(TabKind::Objects(ObjectsDoc { conn_id, table, kind: "索引".into(), new_name: String::new(), new_sql: String::new(), unique: false, status: String::new() }));
    }
    pub fn open_pump(&mut self, conn_id: String) {
        let path = default_export_dir().join("dump.sql").to_string_lossy().to_string();
        self.push_tab(TabKind::Pump(PumpDoc { conn_id, table: String::new(), path, status: String::new() }));
    }
    pub fn open_schema(&mut self, conn_id: String) {
        self.push_tab(TabKind::Schema(SchemaDoc { conn_id, rows: Vec::new(), computed: false }));
    }
    pub fn open_outline(&mut self, conn_id: String) {
        self.push_tab(TabKind::Outline(OutlineDoc {
            conn_id,
            sql: "SELECT c.name, c.country, COUNT(o.id) AS orders\nFROM customers c\nLEFT JOIN orders o ON o.customer_id = c.id\nGROUP BY c.id\nORDER BY orders DESC;".into(),
            message: String::new(),
        }));
    }
    pub fn open_model(&mut self, conn_id: String) {
        if !self.ensure_engine(&conn_id) { return; }
        let (tables, fks) = self.model_tables(&conn_id);
        let cmp = self.store.connections.iter().find(|c| c.id != conn_id).map(|c| c.id.clone()).unwrap_or_else(|| conn_id.clone());
        let sql_path = default_export_dir().join("model.sql").to_string_lossy().to_string();
        self.push_tab(TabKind::Model(ModelDoc {
            conn_id,
            view: "逻辑模型".into(),
            tables,
            fks,
            cmp_conn: cmp,
            diff: Vec::new(),
            status: String::new(),
            sql_path,
        }));
    }
    pub fn reload_model(&self, conn_id: &str) -> (Vec<(String, Vec<String>)>, Vec<(String, String, String, String)>) {
        self.model_tables(conn_id)
    }
    fn model_tables(&self, conn_id: &str) -> (Vec<(String, Vec<String>)>, Vec<(String, String, String, String)>) {
        if let Some(eng) = self.engines.get(conn_id) {
            let mut tables = Vec::new();
            for o in eng.objects().into_iter().filter(|o| o.kind == crate::model::ObjKind::Table) {
                let cols = eng.columns(&o.name).into_iter().map(|c| format!("{} {}", c.name, c.decl_type)).collect();
                tables.push((o.name, cols));
            }
            (tables, eng.foreign_keys())
        } else {
            (Vec::new(), Vec::new())
        }
    }
    pub fn open_find(&mut self, conn_id: String) {
        self.push_tab(TabKind::Find(FindDoc { conn_id, needle: String::new(), hits: Vec::new(), status: String::new() }));
    }
    pub fn open_transfer(&mut self, conn_id: String) {
        let src = conn_id;
        let dst = self.store.connections.first().map(|c| c.id.clone()).unwrap_or(src.clone());
        let table = if let Some(eng) = self.engines.get(&src) {
            eng.objects().into_iter().find(|o| o.kind == crate::model::ObjKind::Table).map(|o| o.name).unwrap_or_default()
        } else { String::new() };
        let cols = if let Some(eng) = self.engines.get(&src) {
            if table.is_empty() { Vec::new() } else { eng.columns(&table).into_iter().map(|c| (c.name, true)).collect() }
        } else { Vec::new() };
        let dst_table = table.clone();
        self.push_tab(TabKind::Transfer(TransferDoc { src_conn: src, dst_conn: dst, table, dst_table, cols, where_clause: String::new(), status: String::new() }));
    }

    pub fn open_settings(&mut self) {
        if let Some(p) = self.tabs.iter().position(|t| matches!(t.kind, TabKind::Settings)) {
            self.active = p;
        } else {
            self.push_tab(TabKind::Settings);
        }
    }

    pub fn new_connection(&mut self, kind: DbKind) {
        self.dialog = Some(Dialog::Conn(ConnectionForm { conn: Connection::new(kind), is_new: true, error: String::new() }));
    }
    pub fn edit_connection(&mut self, id: String) {
        if let Some(c) = self.conn(&id) {
            self.dialog = Some(Dialog::Conn(ConnectionForm { conn: c.clone(), is_new: false, error: String::new() }));
        }
    }

    pub fn commit_connection(&mut self, conn: Connection, is_new: bool) {
        if conn.name.trim().is_empty() {
            return;
        }
        if is_new {
            self.store.connections.push(conn.clone());
        } else if let Some(existing) = self.store.connections.iter_mut().find(|c| c.id == conn.id) {
            *existing = conn.clone();
        }
        self.engines.remove(&conn.id);
        self.store.save();
        self.log(format!("已保存连接: {}", conn.name));
        self.open_connection(conn.id);
    }

    pub fn delete_connection(&mut self, id: String) {
        self.engines.remove(&id);
        self.store.connections.retain(|c| c.id != id);
        self.store.save();
        self.log("已删除连接");
    }

    pub fn request_delete_connection(&mut self, id: String) {
        let name = self.conn(&id).map(|c| c.name.clone()).unwrap_or_default();
        self.dialog = Some(Dialog::Confirm(ConfirmData {
            title: "删除连接".into(),
            msg: format!("确定要删除连接 “{}” 吗？此操作不可撤销。", name),
            action: Act::DeleteConnection(id),
        }));
    }

    pub fn process_act(&mut self, act: Act) {
        match act {
            Act::DeleteConnection(id) => self.delete_connection(id),
            Act::DropTable(conn_id, table) => {
                if let Some(eng) = self.engines.get(&conn_id) {
                    let _ = eng.conn.execute(&format!("DROP TABLE IF EXISTS {}", db::quote_ident(&table)), []);
                }
                self.log(format!("已删除表: {table}"));
                self.tabs.retain(|t| !matches!(&t.kind, TabKind::Table(td) if td.conn_id==conn_id && td.table==table));
            }
            Act::DeleteRow(conn_id, table, rowid) => {
                if let Some(eng) = self.engines.get(&conn_id) {
                    let _ = eng.delete_row(&table, rowid);
                }
                for i in 0..self.tabs.len() {
                    if matches!(&self.tabs[i].kind, TabKind::Table(td) if td.conn_id == conn_id && td.table == table) {
                        self.refresh_table_doc(i);
                    }
                }
                self.log("已删除行");
            }
            Act::OpenSqliteFile(conn_id) => {
                // default sample file in work dir
                let path = default_export_dir().join("sampledb.sqlite").to_string_lossy().to_string();
                if !std::path::Path::new(&path).exists() {
                    if let Ok(e) = SqliteEngine::open(&path) { let _ = e.seed_demo(); }
                }
                if let Some(c) = self.store.connections.iter_mut().find(|c| c.id == conn_id) {
                    c.sqlite_path = path.clone();
                }
                self.engines.remove(&conn_id);
                self.store.save();
                self.open_connection(conn_id);
                self.log(format!("SQLite 文件: {path}"));
            }
            Act::ExportPath(_) | Act::BackupPath(_) | Act::Noop => {}
        }
        self.dialog = None;
    }

    pub fn save_snippets(&mut self) {
        self.store.save();
    }
    pub fn add_snippet(&mut self, name: String, sql: String) {
        if name.trim().is_empty() { return; }
        self.store.snippets.push(Snippet { name, sql });
        self.store.save();
    }

    pub fn toggle_theme(&mut self, ctx: &egui::Context) {
        let next = if self.dark { "light" } else { "dark" };
        self.set_theme(ctx, next);
    }

    /// 切换并持久化指定主题（key 见 `theme::THEMES`）。
    pub fn set_theme(&mut self, ctx: &egui::Context, key: &str) {
        self.theme = key.to_string();
        self.dark = theme::is_dark(key);
        self.store.settings.theme = key.to_string();
        self.store.dark_mode = self.dark;
        self.pal = theme::apply_theme(ctx, key);
        self.store.save();
    }
}

fn summarize(results: &[QueryResult]) -> String {
    let mut lines = Vec::new();
    for r in results {
        if let Some(e) = &r.error {
            lines.push(format!("错误: {}", e));
        } else if r.is_select {
            lines.push(format!("查询成功: {} 行 ({:.1} ms)", r.rows.len(), r.elapsed_ms));
        } else {
            lines.push(format!("执行成功: {} 行受影响 ({:.1} ms)", r.affected, r.elapsed_ms));
        }
    }
    if lines.is_empty() { "无语句".into() } else { lines.join(" | ") }
}

fn install_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    let cjk = egui::FontData::from_owned(include_bytes!("../assets/NotoSansSC-Regular.ttf").to_vec());
    fonts.font_data.insert("noto_sc".to_owned(), cjk);
    if let Some(fam) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
        fam.push("noto_sc".to_owned());
    }
    if let Some(fam) = fonts.families.get_mut(&egui::FontFamily::Monospace) {
        fam.push("noto_sc".to_owned());
    }
    ctx.set_fonts(fonts);
}

pub fn short_id(id: &str) -> String {
    id.chars().take(4).collect()
}

pub fn default_export_dir() -> std::path::PathBuf {
    let d = std::env::var("USERPROFILE").map(|p| std::path::PathBuf::from(p).join("Documents").join("navicat-rs")).unwrap_or_else(|_| std::path::PathBuf::from("."));
    std::fs::create_dir_all(&d).ok();
    d
}
pub fn default_export_path() -> String {
    default_export_dir().join("export.csv").to_string_lossy().to_string()
}
pub fn default_backup_path() -> String {
    default_export_dir().join("backup.sqlite").to_string_lossy().to_string()
}

impl eframe::App for NavicatApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        crate::ui::draw(self, ctx);
    }
}
