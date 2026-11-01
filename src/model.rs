use egui::Color32;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum DbKind {
    Sqlite,
    MySql,
    MariaDb,
    PostgreSql,
    Oracle,
    SqlServer,
    MongoDb,
    Redis,
    Snowflake,
    Dameng,
    Odbc,
}

impl DbKind {
    pub fn all() -> Vec<DbKind> {
        vec![
            DbKind::MySql,
            DbKind::MariaDb,
            DbKind::PostgreSql,
            DbKind::Oracle,
            DbKind::SqlServer,
            DbKind::Sqlite,
            DbKind::MongoDb,
            DbKind::Redis,
            DbKind::Snowflake,
            DbKind::Dameng,
            DbKind::Odbc,
        ]
    }
    pub fn label(&self) -> &'static str {
        match self {
            DbKind::Sqlite => "SQLite",
            DbKind::MySql => "MySQL",
            DbKind::MariaDb => "MariaDB",
            DbKind::PostgreSql => "PostgreSQL",
            DbKind::Oracle => "Oracle",
            DbKind::SqlServer => "SQL Server",
            DbKind::MongoDb => "MongoDB",
            DbKind::Redis => "Redis",
            DbKind::Snowflake => "Snowflake",
            DbKind::Dameng => "达梦 Dameng",
            DbKind::Odbc => "ODBC",
        }
    }
    pub fn short(&self) -> &'static str {
        match self {
            DbKind::Sqlite => "SQ",
            DbKind::MySql => "MY",
            DbKind::MariaDb => "MA",
            DbKind::PostgreSql => "PG",
            DbKind::Oracle => "OR",
            DbKind::SqlServer => "MS",
            DbKind::MongoDb => "MO",
            DbKind::Redis => "RE",
            DbKind::Snowflake => "SF",
            DbKind::Dameng => "DM",
            DbKind::Odbc => "OD",
        }
    }
    pub fn default_port(&self) -> u16 {
        match self {
            DbKind::MySql | DbKind::MariaDb => 3306,
            DbKind::PostgreSql => 5432,
            DbKind::Oracle => 1521,
            DbKind::SqlServer => 1433,
            DbKind::MongoDb => 27017,
            DbKind::Redis => 6379,
            DbKind::Snowflake => 443,
            DbKind::Dameng => 5236,
            DbKind::Sqlite => 0,
            DbKind::Odbc => 0,
        }
    }
    pub fn color(&self) -> Color32 {
        match self {
            DbKind::Sqlite => Color32::from_rgb(0x2E, 0x86, 0xC1),
            DbKind::MySql => Color32::from_rgb(0xE0, 0x8A, 0x1E),
            DbKind::MariaDb => Color32::from_rgb(0x8E, 0x44, 0xAD),
            DbKind::PostgreSql => Color32::from_rgb(0x33, 0x6F, 0x9E),
            DbKind::Oracle => Color32::from_rgb(0xC7, 0x46, 0x34),
            DbKind::SqlServer => Color32::from_rgb(0xB7, 0x2E, 0x2E),
            DbKind::MongoDb => Color32::from_rgb(0x4E, 0xA5, 0x38),
            DbKind::Redis => Color32::from_rgb(0xD1, 0x3B, 0x35),
            DbKind::Snowflake => Color32::from_rgb(0x29, 0xB5, 0xE8),
            DbKind::Dameng => Color32::from_rgb(0x2F, 0x9E, 0x8F),
            DbKind::Odbc => Color32::from_rgb(0x7A, 0x7F, 0x88),
        }
    }
    pub fn is_sqlite(&self) -> bool {
        matches!(self, DbKind::Sqlite)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Connection {
    pub id: String,
    pub name: String,
    pub kind: DbKind,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    #[serde(default)]
    pub sqlite_path: String,
    #[serde(default)]
    pub schema: String,
    #[serde(default)]
    pub ssl: bool,
    #[serde(default)]
    pub ssh_tunnel: bool,
    #[serde(default)]
    pub ssh_host: String,
    #[serde(default)]
    pub ssh_port: u16,
    #[serde(default)]
    pub ssh_user: String,
    #[serde(default)]
    pub save_password: bool,
    #[serde(default)]
    pub color: [u8; 3],
}

impl Connection {
    pub fn new(kind: DbKind) -> Self {
        let mut c = Connection {
            id: uuid::Uuid::new_v4().to_string(),
            name: String::new(),
            kind,
            host: "localhost".into(),
            port: kind.default_port(),
            username: String::new(),
            password: String::new(),
            database: String::new(),
            sqlite_path: String::new(),
            schema: String::new(),
            ssl: false,
            ssh_tunnel: false,
            ssh_host: "localhost".into(),
            ssh_port: 22,
            ssh_user: String::new(),
            save_password: true,
            color: [kind.color().r(), kind.color().g(), kind.color().b()],
        };
        c.name = match kind {
            DbKind::Sqlite => "SQLite".into(),
            other => format!("{} localhost", other.label()),
        };
        c
    }
    pub fn color32(&self) -> Color32 {
        Color32::from_rgb(self.color[0], self.color[1], self.color[2])
    }
    pub fn target_desc(&self) -> String {
        if self.kind.is_sqlite() {
            if self.sqlite_path.is_empty() { "(memory)".into() } else { self.sqlite_path.clone() }
        } else {
            format!("{}:{}/{}", self.host, self.port, self.database)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ObjKind {
    Table,
    View,
    Index,
    Trigger,
}

impl ObjKind {
    pub fn label(&self) -> &'static str {
        match self {
            ObjKind::Table => "表",
            ObjKind::View => "视图",
            ObjKind::Index => "索引",
            ObjKind::Trigger => "触发器",
        }
    }
}

#[derive(Clone, Debug)]
pub struct DbObject {
    pub name: String,
    pub kind: ObjKind,
}

#[derive(Clone, Debug, Default)]
pub struct Column {
    pub cid: i64,
    pub name: String,
    pub decl_type: String,
    pub notnull: bool,
    pub default: Option<String>,
    pub pk: bool,
}

