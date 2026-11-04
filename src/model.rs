//! 数据模型层：定义数据库引擎种类、连接配置、数据库对象与列等核心结构体。
//! 这些类型既用于程序的运行时内存状态，也用于 JSON 持久化（通过 serde 派生）。

use egui::Color32; // 引入 egui 的颜色类型，用于给不同数据库引擎分配标识色
use serde::{Deserialize, Serialize}; // 引入 serde 的序列化 / 反序列化派生宏

/// 支持的数据库引擎种类枚举。
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)] // 派生：可克隆、可拷贝、可相等比较、可序列化、可调试打印
pub enum DbKind { // 声明“数据库种类”枚举
    Sqlite, // SQLite：进程内嵌入式数据库，是本项目的默认引擎
    MySql, // MySQL
    MariaDb, // MariaDB
    PostgreSql, // PostgreSQL（已实现真实网络驱动）
    Oracle, // Oracle
    SqlServer, // SQL Server
    MongoDb, // MongoDB
    Redis, // Redis
    Snowflake, // Snowflake 云数据仓库
    Dameng, // 达梦（国产数据库）
    Odbc, // 通用 ODBC 数据源
} // 枚举定义结束

impl DbKind { // 为 DbKind 实现一组便捷方法
    /// 返回全部引擎种类（用于新建连接时的下拉列表）。
    pub fn all() -> Vec<DbKind> { // 返回一个按常用顺序排列的引擎列表
        vec![ // 构造向量
            DbKind::MySql, // 第 1 项：MySQL
            DbKind::MariaDb, // 第 2 项：MariaDB
            DbKind::PostgreSql, // 第 3 项：PostgreSQL
            DbKind::Oracle, // 第 4 项：Oracle
            DbKind::SqlServer, // 第 5 项：SQL Server
            DbKind::Sqlite, // 第 6 项：SQLite
            DbKind::MongoDb, // 第 7 项：MongoDB
            DbKind::Redis, // 第 8 项：Redis
            DbKind::Snowflake, // 第 9 项：Snowflake
            DbKind::Dameng, // 第 10 项：达梦
            DbKind::Odbc, // 第 11 项：ODBC
        ] // 向量构造结束
    } // all 方法结束
    /// 返回面向用户展示的完整名称。
    pub fn label(&self) -> &'static str { // 根据引擎种类返回显示名称
        match self { // 对当前引擎种类进行匹配
            DbKind::Sqlite => "SQLite", // SQLite 的显示名
            DbKind::MySql => "MySQL", // MySQL 的显示名
            DbKind::MariaDb => "MariaDB", // MariaDB 的显示名
            DbKind::PostgreSql => "PostgreSQL", // PostgreSQL 的显示名
            DbKind::Oracle => "Oracle", // Oracle 的显示名
            DbKind::SqlServer => "SQL Server", // SQL Server 的显示名
            DbKind::MongoDb => "MongoDB", // MongoDB 的显示名
            DbKind::Redis => "Redis", // Redis 的显示名
            DbKind::Snowflake => "Snowflake", // Snowflake 的显示名
            DbKind::Dameng => "达梦 Dameng", // 达梦的显示名（中英并列）
            DbKind::Odbc => "ODBC", // ODBC 的显示名
        } // match 结束
    } // label 方法结束
    /// 返回用于树形控件节点的两字缩写。
    pub fn short(&self) -> &'static str { // 返回连接树中显示的两字母简称
        match self { // 对当前引擎种类进行匹配
            DbKind::Sqlite => "SQ", // SQLite 的两字简称
            DbKind::MySql => "MY", // MySQL 的两字简称
            DbKind::MariaDb => "MA", // MariaDB 的两字简称
            DbKind::PostgreSql => "PG", // PostgreSQL 的两字简称
            DbKind::Oracle => "OR", // Oracle 的两字简称
            DbKind::SqlServer => "MS", // SQL Server 的两字简称
            DbKind::MongoDb => "MO", // MongoDB 的两字简称
            DbKind::Redis => "RE", // Redis 的两字简称
            DbKind::Snowflake => "SF", // Snowflake 的两字简称
            DbKind::Dameng => "DM", // 达梦的两字简称
            DbKind::Odbc => "OD", // ODBC 的两字简称
        } // match 结束
    } // short 方法结束
    /// 返回该引擎的默认端口号（0 表示不适用，如 SQLite）。
    pub fn default_port(&self) -> u16 { // 返回默认网络端口
        match self { // 对当前引擎种类进行匹配
            DbKind::MySql | DbKind::MariaDb => 3306, // MySQL/MariaDB 默认 3306
            DbKind::PostgreSql => 5432, // PostgreSQL 默认 5432
            DbKind::Oracle => 1521, // Oracle 默认 1521
            DbKind::SqlServer => 1433, // SQL Server 默认 1433
            DbKind::MongoDb => 27017, // MongoDB 默认 27017
            DbKind::Redis => 6379, // Redis 默认 6379
            DbKind::Snowflake => 443, // Snowflake 走 HTTPS，默认 443
            DbKind::Dameng => 5236, // 达梦默认 5236
            DbKind::Sqlite => 0, // SQLite 无端口
            DbKind::Odbc => 0, // ODBC 无固定端口
        } // match 结束
    } // default_port 方法结束
    /// 返回该引擎的品牌标识色。
    pub fn color(&self) -> Color32 { // 返回引擎对应颜色（用于连接徽标）
        match self { // 对当前引擎种类进行匹配
            DbKind::Sqlite => Color32::from_rgb(0x2E, 0x86, 0xC1), // SQLite 蓝
            DbKind::MySql => Color32::from_rgb(0xE0, 0x8A, 0x1E), // MySQL 橙
            DbKind::MariaDb => Color32::from_rgb(0x8E, 0x44, 0xAD), // MariaDB 紫
            DbKind::PostgreSql => Color32::from_rgb(0x33, 0x6F, 0x9E), // PostgreSQL 深蓝
            DbKind::Oracle => Color32::from_rgb(0xC7, 0x46, 0x34), // Oracle 红
            DbKind::SqlServer => Color32::from_rgb(0xB7, 0x2E, 0x2E), // SQL Server 暗红
            DbKind::MongoDb => Color32::from_rgb(0x4E, 0xA5, 0x38), // MongoDB 绿
            DbKind::Redis => Color32::from_rgb(0xD1, 0x3B, 0x35), // Redis 红
            DbKind::Snowflake => Color32::from_rgb(0x29, 0xB5, 0xE8), // Snowflake 天蓝
            DbKind::Dameng => Color32::from_rgb(0x2F, 0x9E, 0x8F), // 达梦 青绿
            DbKind::Odbc => Color32::from_rgb(0x7A, 0x7F, 0x88), // ODBC 灰
        } // match 结束
    } // color 方法结束
    /// 判断是否为 SQLite 引擎。
    pub fn is_sqlite(&self) -> bool { // 返回是否为 SQLite
        matches!(self, DbKind::Sqlite) // 使用 matches! 宏判断
    } // is_sqlite 方法结束
} // impl DbKind 结束

/// 一条数据库连接的完整配置。
#[derive(Clone, Serialize, Deserialize, Debug)] // 派生：可克隆、可序列化、可调试打印
pub struct Connection { // 连接配置结构体
    pub id: String, // 唯一标识（UUID）
    pub name: String, // 连接显示名称
    pub kind: DbKind, // 数据库引擎种类
    pub host: String, // 主机地址
    pub port: u16, // 端口
    pub username: String, // 用户名
    pub password: String, // 密码（明文保存于本地配置）
    pub database: String, // 目标数据库名
    #[serde(default)] // 缺失时使用默认值（兼容旧配置）
    pub sqlite_path: String, // SQLite 文件路径
    #[serde(default)] // 缺失时使用默认值
    pub schema: String, // 模式 / schema 名
    #[serde(default)] // 缺失时使用默认值
    pub ssl: bool, // 是否启用 SSL
    #[serde(default)] // 缺失时使用默认值
    pub ssh_tunnel: bool, // 是否启用 SSH 隧道
    #[serde(default)] // 缺失时使用默认值
    pub ssh_host: String, // SSH 跳板机地址
    #[serde(default)] // 缺失时使用默认值
    pub ssh_port: u16, // SSH 端口
    #[serde(default)] // 缺失时使用默认值
    pub ssh_user: String, // SSH 用户名
    #[serde(default)] // 缺失时使用默认值
    pub save_password: bool, // 是否保存密码
    #[serde(default)] // 缺失时使用默认值
    pub color: [u8; 3], // 自定义标识色 RGB
} // 结构体定义结束

impl Connection { // 为 Connection 实现构造与辅助方法
    /// 基于引擎种类创建一条带默认值的连接配置。
    pub fn new(kind: DbKind) -> Self { // 构造函数
        let mut c = Connection { // 初始化连接配置
            id: uuid::Uuid::new_v4().to_string(), // 生成随机 UUID 作为 id
            name: String::new(), // 名称先留空，下面再填充
            kind, // 记录引擎种类
            host: "localhost".into(), // 默认主机为 localhost
            port: kind.default_port(), // 使用引擎默认端口
            username: String::new(), // 用户名留空
            password: String::new(), // 密码留空
            database: String::new(), // 数据库名留空
            sqlite_path: String::new(), // SQLite 路径留空
            schema: String::new(), // schema 留空
            ssl: false, // 默认不启用 SSL
            ssh_tunnel: false, // 默认不启用 SSH 隧道
            ssh_host: "localhost".into(), // SSH 主机默认 localhost
            ssh_port: 22, // SSH 默认端口 22
            ssh_user: String::new(), // SSH 用户名留空
            save_password: true, // 默认保存密码
            color: [kind.color().r(), kind.color().g(), kind.color().b()], // 用引擎品牌色作为默认标识色
        }; // 初始化结束
        c.name = match kind { // 根据引擎种类设置默认连接名
            DbKind::Sqlite => "SQLite".into(), // SQLite 直接用引擎名
            other => format!("{} localhost", other.label()), // 其它引擎使用“引擎名 localhost”
        }; // 默认名设置结束
        c // 返回构造好的连接
    } // new 方法结束
    /// 返回当前连接的自定义标识色。
    pub fn color32(&self) -> Color32 { // 将 [u8;3] 转为 egui Color32
        Color32::from_rgb(self.color[0], self.color[1], self.color[2]) // 构造颜色
    } // color32 方法结束
    /// 返回连接目标的简短描述（用于标题 / 状态栏）。
    pub fn target_desc(&self) -> String { // 生成目标描述文本
        if self.kind.is_sqlite() { // 如果是 SQLite
            if self.sqlite_path.is_empty() { "(memory)".into() } else { self.sqlite_path.clone() } // 空路径表示内存库
        } else { // 其它引擎
            format!("{}:{}/{}", self.host, self.port, self.database) // 形如 host:port/db
        } // if 结束
    } // target_desc 方法结束
} // impl Connection 结束

/// 数据库对象类型（表 / 视图 / 索引 / 触发器）。
#[derive(Clone, Copy, PartialEq, Eq, Debug)] // 派生：可克隆、可拷贝、可相等比较、可调试打印
pub enum ObjKind { // 对象类型枚举
    Table, // 表
    View, // 视图
    Index, // 索引
    Trigger, // 触发器
} // 枚举定义结束

impl ObjKind { // 为 ObjKind 实现方法
    /// 返回对象类型的中文显示名。
    pub fn label(&self) -> &'static str { // 返回显示名
        match self { // 匹配对象类型
            ObjKind::Table => "表", // 表
            ObjKind::View => "视图", // 视图
            ObjKind::Index => "索引", // 索引
            ObjKind::Trigger => "触发器", // 触发器
        } // match 结束
    } // label 方法结束
} // impl ObjKind 结束

/// 一个数据库对象（名称 + 类型）。
#[derive(Clone, Debug)] // 派生：可克隆、可调试打印
pub struct DbObject { // 数据库对象结构体
    pub name: String, // 对象名称
    pub kind: ObjKind, // 对象类型
} // 结构体定义结束

/// 一张表的一列（来自 PRAGMA table_info）。
#[derive(Clone, Debug, Default)] // 派生：可克隆、可调试打印、可实现默认值
pub struct Column { // 列结构体
    pub cid: i64, // 列序号
    pub name: String, // 列名
    pub decl_type: String, // 声明类型
    pub notnull: bool, // 是否非空
    pub default: Option<String>, // 默认值（可空）
    pub pk: bool, // 是否主键
} // 结构体定义结束
