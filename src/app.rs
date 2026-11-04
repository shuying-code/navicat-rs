use crate::db::{self, QueryResult, SqliteEngine}; // 引入数据库类型
use crate::model::{Column, Connection, DbKind}; // 引入模型类型
use crate::store::{Snippet, Store}; // 引入存储类型
use crate::theme::{self, Palette}; // 引入主题类型
use std::collections::HashMap; // 引入 HashMap

/// Project display name, shown as the tree's root connection name.
pub const PROJECT_NAME: &str = "navicat-rs"; // 项目显示名常量

#[derive(Clone, Copy, PartialEq, Eq, Debug)] // 图表类型枚举
pub enum ChartKind { // 柱状图
    Bar, // 折线图
    Line, // 饼图
    Pie, // 语句
} // 枚举结束

pub struct TableDoc { // 表数据文档
    pub conn_id: String, // 连接 id
    pub table: String, // 表名
    pub columns: Vec<Column>, // 列定义
    pub has_rowid: bool, // 是否有 rowid
    pub rows: Vec<(i64, Vec<Option<String>>)>, // 行数据（rowid + 值）
    pub total: i64, // 总行数
    pub page: usize, // 当前页
    pub page_size: usize, // 每页行数
    pub filter: String, // 过滤条件
    pub order: Option<(String, bool)>, // 排序
    pub editing: Option<(usize, usize)>, // 编辑中的单元格
    pub edit_buf: String, // 编辑缓冲
    pub error: String, // 错误信息
    pub selected_row: Option<usize>, // 选中行
} // 结构结束

pub struct QueryDoc { // 查询文档
    pub conn_id: String, // 连接 id
    pub sql: String, // SQL 文本
    pub results: Vec<QueryResult>, // 结果集
    pub active_result: usize, // 激活结果索引
    pub message: String, // 消息
} // 结构结束

pub struct DesignerDoc { // 设计器文档
    pub conn_id: String, // 连接 id
    pub original: Option<String>, // 原表名
    pub name: String, // 表名
    pub columns: Vec<Column>, // 列
    pub error: String, // 错误
} // 结构结束

pub struct ErDoc { // ER 文档
    pub conn_id: String, // 连接 id
    pub tables: Vec<(String, Vec<String>)>, // 表及其列
    pub fks: Vec<(String, String, String, String)>, // 外键
    pub pos: HashMap<String, (f32, f32)>, // 表位置
    pub zoom: f32, // 缩放
} // 结构结束

pub struct QbDoc { // 查询构建文档
    pub conn_id: String, // 连接 id
    pub table: String, // 表名
    pub cols: Vec<(String, bool)>, // 列及勾选
    pub where_col: String, // 条件列
    pub where_op: String, // 运算符
    pub where_val: String, // 条件值
    pub order_col: String, // 排序列
    pub desc: bool, // 降序
    pub limit: String, // 限制
    pub preview: QueryResult, // 预览结果
    pub status: String, // 状态
} // 结构结束

pub struct TransferDoc { // 数据传输文档
    pub src_conn: String, // 源连接
    pub dst_conn: String, // 目标连接
    pub table: String, // 源表
    pub dst_table: String, // 目标表
    pub cols: Vec<(String, bool)>, // 列
    pub where_clause: String, // WHERE 子句
    pub status: String, // 状态
} // 结构结束

pub struct ExportDoc { // 导出文档
    pub conn_id: String, // 连接 id
    pub table: String, // 表名
    pub format: String, // 格式
    pub path: String, // 路径
    pub status: String, // 状态
} // 结构结束

pub struct ImportDoc { // 导入文档
    pub conn_id: String, // 连接 id
    pub table: String, // 表名
    pub format: String, // 格式
    pub path: String, // 路径
    pub status: String, // 状态
} // 结构结束

pub struct BackupDoc { // 备份文档
    pub conn_id: String, // 连接 id
    pub path: String, // 路径
    pub status: String, // 状态
} // 结构结束

pub struct MonitorDoc { // 监控文档
    pub conn_id: String, // 连接 id
    pub auto_refresh: bool, // 自动刷新
} // 结构结束

pub struct ProfileDoc { // 剖析文档
    pub conn_id: String, // 连接 id
    pub table: String, // 表名
    pub stats: Vec<(String, String)>, // 统计
    pub computed: bool, // 是否已计算
} // 结构结束

pub struct GenDoc { // 数据生成文档
    pub conn_id: String, // 连接 id
    pub table: String, // 表名
    pub count: usize, // 行数
    pub status: String, // 状态
} // 结构结束

pub struct ChartDoc { // 图表文档
    pub conn_id: String, // 连接 id
    pub sql: String, // SQL
    pub data: QueryResult, // 数据
    pub kind: ChartKind, // 图表类型
    pub label_col: usize, // 标签列
    pub value_col: usize, // 数值列
    pub status: String, // 状态
} // 结构结束

pub struct AiDoc { // AI 文档
    pub conn_id: String, // 连接 id
    pub prompt: String, // 提问
    pub answer: String, // 回答
} // 结构结束

pub struct DictDoc { // 数据字典文档
    pub conn_id: String, // 连接 id
    pub table: String, // 表名
} // 结构结束

pub struct SnippetDoc { // 代码片段文档
    pub selected: Option<usize>, // 选中索引
    pub edit_name: String, // 编辑名称
    pub edit_sql: String, // 编辑 SQL
} // 结构结束

pub struct ReportDoc { // 报表文档
    pub conn_id: String, // 连接 id
    pub title: String, // 标题
    pub sql: String, // SQL
    pub status: String, // 状态
} // 结构结束

pub struct AutoDoc { // 自动化文档
    pub conn_id: String, // 连接 id
    pub tasks: Vec<String>, // 任务
} // 结构结束

pub struct UsersDoc { // 用户文档
    pub conn_id: String, // 连接 id
} // 结构结束

pub struct MaskingDoc { // 脱敏文档
    pub conn_id: String, // 连接 id
    pub table: String, // 表名
    pub column: String, // 列名
    pub rule: String, // 规则
    pub status: String, // 状态
} // 结构结束

pub struct SyncDoc { // 同步文档
    pub src_conn: String, // 源连接
    pub dst_conn: String, // 目标连接
    pub table: String, // 表名
    pub mode: String, // 模式
    pub diff: Vec<String>, // 差异
    pub status: String, // 状态
} // 结构结束

pub struct HistoryDoc { // 历史文档
    pub selected: Option<usize>, // 选中索引
} // 结构结束

pub struct ObjectsDoc { // 对象管理文档
    pub conn_id: String, // 连接 id
    pub table: String, // 表名
    pub kind: String,   // "索引" | "触发器" | "视图"
    pub new_name: String, // 新对象名
    pub new_sql: String, // 新对象 SQL
    pub unique: bool, // 是否唯一
    pub status: String, // 状态
} // 结构结束

pub struct PumpDoc { // 数据泵文档
    pub conn_id: String, // 连接 id
    pub table: String,   // empty = whole database // 表名
    pub path: String, // 路径
    pub status: String, // 状态
} // 结构结束

pub struct SchemaDoc { // 模式分析文档
    pub conn_id: String, // 连接 id
    pub rows: Vec<(String, String)>, // 行数据
    pub computed: bool, // 是否已计算
} // 结构结束

pub struct OutlineDoc { // 大纲文档
    pub conn_id: String, // 连接 id
    pub sql: String, // SQL
    pub message: String, // 消息
} // 结构结束

pub struct ModelDoc { // 模型文档
    pub conn_id: String, // 连接 id
    pub view: String, // 概念模型 / 逻辑模型 / 物理模型
    pub tables: Vec<(String, Vec<String>)>, // 表及其列
    pub fks: Vec<(String, String, String, String)>, // 外键
    pub cmp_conn: String, // 对比连接
    pub diff: Vec<String>, // 差异
    pub status: String, // 状态
    pub sql_path: String, // SQL 路径
} // 结构结束

pub struct FindDoc { // 查找文档
    pub conn_id: String, // 连接 id
    pub needle: String, // 关键词
    pub hits: Vec<(String, String, String)>, // 命中
    pub status: String, // 状态
} // 结构结束

pub enum TabKind { // 标签类型枚举
    Welcome, // 欢迎
    Table(TableDoc), // 调用 Table
    Query(QueryDoc), // 调用 Query
    Designer(DesignerDoc), // 调用 Designer
    Er(ErDoc), // 调用 Er
    QueryBuilder(QbDoc), // 调用 QueryBuilder
    Transfer(TransferDoc), // 调用 Transfer
    Export(ExportDoc), // 调用 Export
    Import(ImportDoc), // 调用 Import
    Backup(BackupDoc), // 调用 Backup
    Monitor(MonitorDoc), // 调用 Monitor
    Profiling(ProfileDoc), // 调用 Profiling
    DataGen(GenDoc), // 调用 DataGen
    Chart(ChartDoc), // 调用 Chart
    Ai(AiDoc), // 调用 Ai
    Dict(DictDoc), // 调用 Dict
    Snippet(SnippetDoc), // 调用 Snippet
    Report(ReportDoc), // 调用 Report
    Automation(AutoDoc), // 调用 Automation
    Users(UsersDoc), // 调用 Users
    Masking(MaskingDoc), // 调用 Masking
    Sync(SyncDoc), // 调用 Sync
    History(HistoryDoc), // 调用 History
    Objects(ObjectsDoc), // 调用 Objects
    Pump(PumpDoc), // 调用 Pump
    Schema(SchemaDoc), // 调用 Schema
    Outline(OutlineDoc), // 调用 Outline
    Model(ModelDoc), // 调用 Model
    Find(FindDoc), // 调用 Find
    Settings, // 设置
    About, // 关于
} // 枚举结束

pub struct Tab { // 标签结构
    pub id: u64, // 标签 id
    pub kind: TabKind, // 标签类型
} // 结构结束

impl Tab { // 为 Tab 实现
    pub fn title(&self) -> String { // 返回标签标题
        match &self.kind { // 匹配类型
            TabKind::Welcome => "欢迎".into(), // 欢迎
            TabKind::Table(t) => format!("{} [{}]", t.table, short_id(&t.conn_id)), // 表：表名 + 短 id
            TabKind::Query(q) => { // 查询：取首行
                let first = q.sql.lines().next().unwrap_or("Query").trim(); // 首行文字
                let s: String = first.chars().take(24).collect(); // 截断 24 字符
                if s.is_empty() { "查询".into() } else { s } // 空则显示“查询”
            } // 块结束
            TabKind::Designer(d) => match &d.original { // 设计器
                Some(t) => format!("设计: {}", t), // 有原表名
                None => "新建表".into(), // 设计: 表名
            }, // 否则
            TabKind::Er(_) => "ER 图".into(), // 新建表
            TabKind::QueryBuilder(_) => "查询构建器".into(), // match 结束
            TabKind::Transfer(_) => "数据传输".into(), // ER 图
            TabKind::Export(_) => "导出向导".into(), // 查询构建器
            TabKind::Import(_) => "导入向导".into(), // 数据传输
            TabKind::Backup(_) => "备份/还原".into(), // 导出向导
            TabKind::Monitor(_) => "服务器监控".into(), // 导入向导
            TabKind::Profiling(_) => "数据剖析".into(), // 备份/还原
            TabKind::DataGen(_) => "数据生成".into(), // 服务器监控
            TabKind::Chart(_) => "图表".into(), // 数据剖析
            TabKind::Ai(_) => "AI 助手".into(), // 数据生成
            TabKind::Dict(_) => "数据字典".into(), // 图表
            TabKind::Snippet(_) => "代码片段".into(), // AI 助手
            TabKind::Report(_) => "报表".into(), // 数据字典
            TabKind::Automation(_) => "自动化".into(), // 代码片段
            TabKind::Users(_) => "用户与权限".into(), // 报表
            TabKind::Masking(_) => "数据脱敏".into(), // 自动化
            TabKind::Sync(_) => "同步".into(), // 用户与权限
            TabKind::History(_) => "查询历史".into(), // 数据脱敏
            TabKind::Objects(_) => "对象管理".into(), // 同步
            TabKind::Pump(_) => "数据泵".into(), // 查询历史
            TabKind::Schema(_) => "模式分析".into(), // 对象管理
            TabKind::Outline(_) => "SQL 大纲/重构".into(), // 数据泵
            TabKind::Model(_) => "模型".into(), // 模式分析
            TabKind::Find(_) => "查找于数据库".into(), // SQL 大纲/重构
            TabKind::Settings => "设置".into(), // 模型
            TabKind::About => "关于".into(), // 查找于数据库
        } // 设置
    } // 关于
    pub fn icon(&self) -> &'static str { // match 结束
        match &self.kind { // 函数结束
            TabKind::Welcome => "\u{1F3E0}", // 返回标签图标
            TabKind::Table(_) => "\u{1F5C2}", // 匹配类型
            TabKind::Query(_) => "\u{1F4C4}", // 欢迎图标
            TabKind::Designer(_) => "\u{1F527}", // 表图标
            TabKind::Er(_) => "\u{1F5FA}", // 查询图标
            TabKind::QueryBuilder(_) => "\u{1F9F1}", // 设计器图标
            TabKind::Transfer(_) => "\u{21C4}", // ER 图标
            TabKind::Export(_) => "\u{2B06}", // 查询构建图标
            TabKind::Import(_) => "\u{2B07}", // 传输图标
            TabKind::Backup(_) => "\u{1F4BE}", // 导出图标
            TabKind::Monitor(_) => "\u{1F4C8}", // 导入图标
            TabKind::Profiling(_) => "\u{1F50E}", // 备份图标
            TabKind::DataGen(_) => "\u{1F9EA}", // 监控图标
            TabKind::Chart(_) => "\u{1F4CA}", // 剖析图标
            TabKind::Ai(_) => "\u{2728}", // 数据生成图标
            TabKind::Dict(_) => "\u{1F4D6}", // 图表图标
            TabKind::Snippet(_) => "\u{1F4CC}", // AI 图标
            TabKind::Report(_) => "\u{1F4CB}", // 字典图标
            TabKind::Automation(_) => "\u{2699}", // 片段图标
            TabKind::Users(_) => "\u{1F465}", // 报表图标
            TabKind::Masking(_) => "\u{25D0}", // 自动化图标
            TabKind::Sync(_) => "\u{21CB}", // 用户图标
            TabKind::History(_) => "\u{21BA}", // 脱敏图标
            TabKind::Objects(_) => "\u{25A4}", // 同步图标
            TabKind::Pump(_) => "\u{21E3}", // 历史图标
            TabKind::Schema(_) => "\u{2317}", // 对象图标
            TabKind::Outline(_) => "\u{2263}", // 数据泵图标
            TabKind::Model(_) => "\u{25A6}", // 模式图标
            TabKind::Find(_) => "\u{2315}", // 大纲图标
            TabKind::Settings => "\u{1F527}", // 模型图标
            TabKind::About => "\u{2139}", // 查找图标
        } // 设置图标
    } // 关于图标
} // match 结束

pub struct ConnectionForm { // 连接表单结构
    pub conn: Connection, // 连接
    pub is_new: bool, // 是否新建
    pub error: String, // 错误
} // 结构结束

pub struct ConfirmData { // 确认数据
    pub title: String, // 标题
    pub msg: String, // 消息
    pub action: Act, // 动作
} // 结构结束

pub struct InputData { // 输入数据
    pub title: String, // 标题
    pub label: String, // 提示
    pub value: String, // 值
    pub action: Act, // 动作
} // 结构结束

pub enum Dialog { // 对话框枚举
    Conn(ConnectionForm), // 连接表单
    Confirm(ConfirmData), // 确认
    Input(InputData), // 输入
    About, // 关于
} // 枚举结束

#[derive(Clone)] // 可克隆
pub enum Act { // 动作枚举
    DeleteConnection(String), // 删除连接
    DropTable(String, String), // 删表
    DeleteRow(String, String, i64), // 删行
    OpenSqliteFile(String), // 打开 SQLite 文件
    ExportPath(u64), // 导出路径
    BackupPath(String), // 备份路径
    Noop, // 空操作
} // 枚举结束

pub struct NavicatApp { // 应用主结构
    pub store: Store, // 存储
    pub pal: Palette, // 调色板
    pub engines: HashMap<String, SqliteEngine>, // 引擎表
    pub status: String, // 状态栏文字
    pub tabs: Vec<Tab>, // 标签集合
    pub active: usize, // 激活标签索引
    pub next_tab_id: u64, // 下一个标签 id
    pub dialog: Option<Dialog>, // 当前对话框
    pub tree_search: String, // 树搜索词
    pub expanded: HashMap<String, bool>, // 连接展开状态
    pub selected_conn: Option<String>, // 选中连接
    pub left_w: f32, // 左栏宽度
    pub log: Vec<String>, // 日志
    pub show_log: bool, // 是否显示日志
    pub dark: bool, // 是否深色
    pub theme: String, // 主题 key
    pub focus_mode: bool, // 焦点模式
} // 结构结束

impl NavicatApp { // 为 NavicatApp 实现
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self { // 构造应用
        let store = Store::load(); // 载入存储
        install_fonts(&cc.egui_ctx); // 安装中文字体
        // 主题：环境变量（截图/验收用）优先，其次 settings.theme，最后按旧 dark_mode 兼容推导。
        let theme_key = if let Ok(t) = std::env::var("NAVICAT_RS_THEME") { // 计算主题 key（环境变量优先）
            let t = t.trim().to_string(); // 读取环境变量
            if t.is_empty() { String::new() } else { t } // 空则空串
        } else if store.settings.theme.trim().is_empty() { // 否则若设置为空
            if store.dark_mode { "dark".to_string() } else { "light".to_string() } // 按旧 dark_mode 推导
        } else { // 否则
            store.settings.theme.trim().to_string() // 用设置的主题
        }; // if 结束
        let theme_key = if theme_key.is_empty() { // 若 key 仍为空
            if store.dark_mode { "dark".to_string() } else { "light".to_string() } // 再次推导
        } else { theme_key }; // 否则
        let dark = theme::is_dark(&theme_key); // 用已有 key
        let pal = theme::apply_theme(&cc.egui_ctx, &theme_key); // if 结束
        let mut app = NavicatApp { // 是否深色主题
            store, // 应用主题
            pal, // 构造应用对象
            engines: HashMap::new(), // 存储
            status: "就绪 · 全部功能已解锁（免费开放，无权限限制）".into(), // 调色板
            tabs: Vec::new(), // 空引擎表
            active: 0, // 初始状态文字
            next_tab_id: 1, // 空标签集
            dialog: None, // 激活 0
            tree_search: String::new(), // 下一个 id=1
            expanded: HashMap::new(), // 无对话框
            selected_conn: None, // 空搜索
            left_w: 290.0, // 空展开
            log: Vec::new(), // 无选中连接
            show_log: false, // 左栏宽 290
            dark, // 空日志
            theme: theme_key.clone(), // 不显示日志
            focus_mode: false, // dark
        }; // theme
        app.push_tab(TabKind::Welcome); // 非焦点模式
        if app.store.connections.is_empty() { // 结构结束
            let mut c = Connection::new(DbKind::Sqlite); // 推入欢迎标签
            c.name = "示例数据库 (SQLite)".into(); // 若无连接
            c.sqlite_path.clear(); // 新建 SQLite 连接
            app.store.connections.push(c); // 设默认名称
            app.store.save(); // 清空路径（内存库）
        } // 加入连接
        // Optional demo autoload for screenshots / acceptance (NAVICAT_RS_DEMO=1) // 保存
        let demo = std::env::var("NAVICAT_RS_DEMO").ok(); // 读取演示环境变量
        if let Some((id, table)) = app.store.connections.first().map(|c| (c.id.clone(), "customers".to_string())) { // 若存在首个连接
            match demo.as_deref() { // 匹配演示模式
                Some("1") => { // demo=1
                    app.open_connection(id.clone()); // 打开连接
                    app.new_query(id.clone()); // 新建查询
                    app.open_table(id.clone(), table); // 打开表
                } // 分支结束
                Some("all") => { // demo=all
                    app.open_connection(id.clone()); // 打开连接
                    app.open_table(id.clone(), table.clone()); // 打开表
                    app.new_query(id.clone()); // 新建查询
                    app.open_designer(id.clone(), Some(table.clone())); // 打开设计器
                    app.open_er(id.clone()); // 打开 ER 图
                    app.open_query_builder(id.clone()); // 打开查询构建器
                    app.open_transfer(id.clone()); // 打开数据传输
                    app.open_export(id.clone(), table.clone()); // 打开导出
                    app.open_import(id.clone(), table.clone()); // 打开导入
                    app.open_backup(id.clone()); // 打开备份
                    app.open_monitor(id.clone()); // 打开监控
                    app.open_profiling(id.clone(), table.clone()); // 打开剖析
                    app.open_datagen(id.clone(), table.clone()); // 打开数据生成
                    app.open_chart(id.clone()); // 打开图表
                    app.open_ai(id.clone()); // 打开 AI
                    app.open_dict(id.clone(), table.clone()); // 打开数据字典
                    app.open_snippets(); // 打开代码片段
                    app.open_report(id.clone()); // 打开报表
                    app.open_automation(id.clone()); // 打开自动化
                    app.open_users(id.clone()); // 打开用户
                    app.open_masking(id.clone(), table.clone()); // 打开脱敏
                    app.open_sync(id.clone()); // 打开同步
                    app.open_history(); // 打开历史
                    app.open_objects(id.clone(), table.clone()); // 打开对象管理
                    app.open_pump(id.clone()); // 打开数据泵
                    app.open_schema(id.clone()); // 打开模式分析
                    app.open_outline(id.clone()); // 打开大纲
                    app.open_model(id.clone()); // 打开模型
                    app.open_find(id.clone()); // 打开查找
                    app.open_settings(); // 打开设置
                } // 分支结束
                _ => {} // 默认无操作
            } // match 结束
            // Optional: bring a specific tab to front for screenshots (NAVICAT_RS_TAB=<substring>)
            if let Ok(needle) = std::env::var("NAVICAT_RS_TAB") { // 读取 TAB 环境变量
                let n = needle.to_lowercase(); // 转小写
                if let Some(idx) = app.tabs.iter().position(|t| t.title().to_lowercase().contains(&n)) { // 查找匹配标签
                    app.active = idx; // 设为激活
                } // if 结束
            } // if 结束
        } // if 结束
        app // 返回应用
    } // 函数结束

    pub fn push_tab(&mut self, kind: TabKind) { // 推入新标签
        let id = self.next_tab_id; // 分配新 id
        self.next_tab_id += 1; // id 自增
        self.tabs.push(Tab { id, kind }); // 加入标签
        self.active = self.tabs.len() - 1; // 激活新标签
    } // 函数结束

    pub fn tab_by_id(&self, id: u64) -> Option<usize> { // 按 id 查标签索引
        self.tabs.iter().position(|t| t.id == id) // 定位
    } // 函数结束

    pub fn log(&mut self, msg: impl Into<String>) { // 记录日志 / 更新状态
        let m = msg.into(); // 转字符串
        self.status = m.clone(); // 更新状态栏
        self.log.push(format!("{}  {}", chrono::Local::now().format("%H:%M:%S"), m)); // 追加带时间戳日志
        if self.log.len() > 500 { // 若超过 500 条
            self.log.remove(0); // 移除最旧一条
        } // if 结束
    } // 函数结束

    pub fn conn(&self, id: &str) -> Option<&Connection> { // 按 id 取连接
        self.store.connections.iter().find(|c| c.id == id) // 查找连接
    } // 函数结束

    pub fn ensure_engine(&mut self, id: &str) -> bool { // 确保引擎就绪
        if self.engines.contains_key(id) { // 若引擎已存在
            return true; // 直接返回 true
        } // if 结束
        let conn = match self.conn(id) { // 取连接配置
            Some(c) => c.clone(), // 存在则克隆
            None => return false, // 否则返回 false
        }; // match 结束
        let eng = if conn.kind.is_sqlite() { // 若为 SQLite
            if conn.sqlite_path.is_empty() { // 若路径为空
                SqliteEngine::open_memory_seeded() // 打开内存示例库
            } else { // 否则
                SqliteEngine::open(&conn.sqlite_path) // 打开文件库
            } // if 结束
        } else { // 否则（其它引擎）
            // Non-SQLite engines: provide a functional demo schema so every page works offline.
            SqliteEngine::open_memory_seeded() // 打开内存示例库（离线演示）
        }; // if 结束
        match eng { // 匹配引擎结果
            Ok(e) => { // 成功
                self.engines.insert(id.to_string(), e); // 加入引擎表
                self.log(format!("已连接: {}", conn.name)); // 记录已连接
                true // 返回 true
            } // 分支结束
            Err(e) => { // 失败
                self.log(format!("连接失败: {e}")); // 记录失败
                false // 返回 false
            } // 分支结束
        } // match 结束
    } // 函数结束

    pub fn open_connection(&mut self, id: String) { // 打开连接
        if self.ensure_engine(&id) { // 若引擎就绪
            self.expanded.insert(id.clone(), true); // 标记展开
            self.selected_conn = Some(id); // 设为选中
        } // if 结束
    } // 函数结束

    pub fn objects_for(&mut self, id: &str) -> Vec<crate::model::DbObject> { // 取连接的对象列表
        if !self.engines.contains_key(id) { // 若引擎未就绪
            return Vec::new(); // 返回空
        } // if 结束
        self.engines.get(id).map(|e| e.objects()).unwrap_or_default() // 返回对象列表
    } // 函数结束

    pub fn fetch_table( // 抓取表数据（分页）
        eng: &SqliteEngine, // 字段 eng
        table: &str, // 字段 table
        page: usize, // 字段 page
        page_size: usize, // 字段 page_size
        filter: &str, // 字段 filter
        order: &Option<(String, bool)>, // 字段 order
    ) -> (Vec<Column>, bool, Vec<(i64, Vec<Option<String>>)>, i64) { // 语句
        let columns = eng.columns(table); // 取列定义
        let has_rowid = eng.table_has_rowid(table); // 是否有 rowid
        let where_clause = if filter.trim().is_empty() { // 构造 WHERE 子句
            String::new() // 无过滤
        } else { // 否则
            format!(" WHERE {}", filter) // 带过滤
        }; // if 结束
        let order_clause = match order { // 构造 ORDER BY 子句
            Some((c, asc)) => format!(" ORDER BY {} {}", db::quote_ident(c), if *asc { "ASC" } else { "DESC" }), // 有排序
            None => String::new(), // 无排序
        }; // match 结束
        let total: i64 = eng // 查询总行数
            .conn // 连接
            .query_row( // 查询
                &format!("SELECT COUNT(*) FROM {}{}", db::quote_ident(table), where_clause), // COUNT 语句
                [], // 无参数
                |r| r.get(0), // 取第一列
            ) // query_row 结束
            .unwrap_or(0); // 默认 0
        let collist = if has_rowid { // 列清单
            "rowid, *".to_string() // 有 rowid 时含 rowid
        } else { // 否则
            "*".to_string() // 全部列
        }; // if 结束
        let sql = format!( // 构造分页查询 SQL
            "SELECT {} FROM {}{}{} LIMIT {} OFFSET {}", // 格式串
            collist, // 列清单
            db::quote_ident(table), // 表名
            where_clause, // where
            order_clause, // order
            page_size, // 每页行数
            page * page_size // 偏移量
        ); // format 结束
        let mut rows = Vec::new(); // 结果行集合
        if let Ok(mut st) = eng.conn.prepare(&sql) { // 准备语句
            let ncol = st.column_count(); // 列数
            if let Ok(mut q) = st.query([]) { // 执行查询
                while let Ok(Some(row)) = q.next() { // 遍历行
                    let offset = if has_rowid { 1 } else { 0 }; // 偏移（有 rowid 则从 1 开始）
                    let rowid = if has_rowid { // 取 rowid
                        row.get::<_, i64>(0).unwrap_or(-1) // 读第 0 列
                    } else { // 否则
                        -1 // -1
                    }; // if 结束
                    let mut rec = Vec::with_capacity(ncol - offset); // 预分配记录
                    for i in offset..ncol { // 遍历列
                        rec.push(match row.get_ref(i) { // 转换值引用
                            Ok(rusqlite::types::ValueRef::Null) => None, // NULL
                            Ok(rusqlite::types::ValueRef::Integer(v)) => Some(v.to_string()), // 整数
                            Ok(rusqlite::types::ValueRef::Real(v)) => Some(format!("{}", v)), // 实数
                            Ok(rusqlite::types::ValueRef::Text(v)) => Some(String::from_utf8_lossy(v).to_string()), // 文本
                            Ok(rusqlite::types::ValueRef::Blob(v)) => Some(format!("[BLOB {} bytes]", v.len())), // BLOB 摘要
                            Err(_) => None, // 出错取 None
                        }); // 匹配结束
                    } // for 结束
                    rows.push((rowid, rec)); // 加入行
                } // while 结束
            } // if 结束
        } // if 结束
        (columns, has_rowid, rows, total) // 返回列/rowid/行/总数
    } // 函数结束

    pub fn open_table(&mut self, conn_id: String, table: String) { // 打开表标签
        if let Some(pos) = self.tabs.iter().position(|t| { // 若已存在同表标签
            matches!(&t.kind, TabKind::Table(td) if td.conn_id == conn_id && td.table == table) // 条件匹配
        }) { // 闭包结束
            self.active = pos; // 激活并返回
            return; // if 结束
        } // 若引擎未就绪
        if !self.ensure_engine(&conn_id) { // 返回
            return; // if 结束
        } // 取引擎
        let eng = self.engines.get(&conn_id).unwrap(); // 抓取首页数据
        let (columns, has_rowid, rows, total) = Self::fetch_table(eng, &table, 0, self.store.settings.page_size, "", &None); // 构造文档
        let doc = TableDoc { // 连接 id
            conn_id, // 表名
            table, // 列
            columns, // rowid 标志
            has_rowid, // 行
            rows, // 总数
            total, // 页 0
            page: 0, // 每页行数
            page_size: self.store.settings.page_size, // 空过滤
            filter: String::new(), // 无排序
            order: None, // 无编辑
            editing: None, // 空编辑缓冲
            edit_buf: String::new(), // 空错误
            error: String::new(), // 无选中行
            selected_row: None, // 结构结束
        }; // 块结束
        self.push_tab(TabKind::Table(doc)); // 推入表标签
    } // 函数结束

    pub fn refresh_table_doc(&mut self, idx: usize) { // 刷新表文档
        let (conn_id, table, page, page_size, filter, order) = match &self.tabs[idx].kind { // 读取表标签字段
            TabKind::Table(t) => (t.conn_id.clone(), t.table.clone(), t.page, t.page_size, t.filter.clone(), t.order.clone()), // 表标签
            _ => return, // 否则返回
        }; // match 结束
        if let Some(eng) = self.engines.get(&conn_id) { // 若引擎存在
            let (columns, has_rowid, rows, total) = Self::fetch_table(eng, &table, page, page_size, &filter, &order); // 抓取数据
            if let TabKind::Table(t) = &mut self.tabs[idx].kind { // 写回文档
                t.columns = columns; // 列
                t.has_rowid = has_rowid; // rowid 标志
                t.rows = rows; // 行
                t.total = total; // 总数
                t.editing = None; // 清编辑态
                t.selected_row = None; // 清选中行
            } // if 结束
        } // if 结束
    } // 函数结束

    pub fn new_query(&mut self, conn_id: String) { // 新建查询标签
        let sql = "-- 在此输入 SQL，Ctrl+Enter 执行\n".to_string(); // 默认 SQL 提示
        self.push_tab(TabKind::Query(QueryDoc { // 推入查询标签
            conn_id, // 连接 id
            sql, // SQL
            results: Vec::new(), // 空结果
            active_result: 0, // 激活 0
            message: String::new(), // 空消息
        })); // 结构结束
    } // 函数结束

    pub fn run_query_at(&mut self, idx: usize, only_selected: Option<String>) { // 执行指定标签的查询
        let (conn_id, sql) = match &self.tabs[idx].kind { // 取连接 id 与待执行 SQL
            TabKind::Query(q) => (q.conn_id.clone(), only_selected.unwrap_or_else(|| q.sql.clone())), // 查询标签
            _ => return, // 否则返回
        }; // match 结束
        if !self.ensure_engine(&conn_id) { // 若引擎未就绪
            return; // 返回
        } // if 结束
        // When 自动提交 is off, wrap the whole script in a single transaction so
        // the run is atomic (all-or-nothing) instead of per-statement autocommit.
        let script = if !self.store.settings.auto_commit && !sql.trim().is_empty() { // 若非自动提交且 SQL 非空
            format!("BEGIN;\n{}\nCOMMIT;", sql) // 包裹为单事务
        } else { // 否则
            sql // 原样
        }; // if 结束
        // PostgreSQL uses a real wire-protocol driver; SQLite uses the embedded engine.
        let kind = self.conn(&conn_id).map(|c| c.kind); // 取引擎类型
        let results = if kind == Some(DbKind::PostgreSql) { // 若为 PostgreSQL
            let conn = self.conn(&conn_id).cloned().unwrap(); // 克隆连接
            crate::realdb::pg_run(&conn, &script) // 走真实网络驱动
        } else { // 否则
            self.engines.get(&conn_id).unwrap().run_script(&script) // 走内嵌 SQLite 引擎
        }; // if 结束
        let msg = summarize(&results); // 汇总消息
        if let TabKind::Query(q) = &mut self.tabs[idx].kind { // 写回查询标签
            q.results = results; // 结果
            q.active_result = 0; // 激活 0
            q.message = msg.clone(); // 消息
        } // if 结束
        self.log(msg); // 记录日志
    } // 函数结束

    pub fn open_designer(&mut self, conn_id: String, table: Option<String>) { // 打开表设计器
        if !self.ensure_engine(&conn_id) { // 若引擎未就绪
            return; // 返回
        } // if 结束
        let eng = self.engines.get(&conn_id).unwrap(); // 取引擎
        let (name, columns) = match &table { // 取名称与列
            Some(t) => (t.clone(), eng.columns(t)), // 已有表
            None => ("new_table".to_string(), vec![ // 否则默认新表两列
                Column { cid: 0, name: "id".into(), decl_type: "INTEGER".into(), notnull: true, default: None, pk: true }, // 列 id
                Column { cid: 1, name: "name".into(), decl_type: "TEXT".into(), notnull: false, default: None, pk: false }, // 列 name
            ]), // match 结束
        }; // 推入设计器标签
        self.push_tab(TabKind::Designer(DesignerDoc { // 连接 id
            conn_id, // 原表名
            original: table, // 名称
            name, // 列
            columns, // 空错误
            error: String::new(), // 结构结束
        })); // 函数结束
    } // 块结束

    pub fn save_designer(&mut self, idx: usize) { // 保存表设计器
        let (conn_id, original, name, columns) = match &self.tabs[idx].kind { // 读取设计器字段
            TabKind::Designer(d) => (d.conn_id.clone(), d.original.clone(), d.name.clone(), d.columns.clone()), // 设计器标签
            _ => return, // 否则返回
        }; // match 结束
        if name.trim().is_empty() { // 若表名为空
            if let TabKind::Designer(d) = &mut self.tabs[idx].kind { d.error = "表名不能为空".into(); } // 设错误并返回
            return; // 返回
        } // if 结束
        if !self.ensure_engine(&conn_id) { return; } // 若引擎未就绪返回
        let eng = self.engines.get(&conn_id).unwrap(); // 取引擎
        let res = (|| -> anyhow::Result<()> { // 结果闭包
            let colsdef = db::build_create_table(&columns)?; // 生成列定义
            if original.is_some() { // 若为已存在表
                // simple ALTER: add missing columns (SQLite limited). Rename not handled fully.
                let old = eng.columns(&name); // 取旧列
                for c in &columns { // 遍历新列
                    if !old.iter().any(|o| o.name == c.name) { // 若旧表中不存在
                        let mut d = format!("ALTER TABLE {} ADD COLUMN {} {}", db::quote_ident(&name), db::quote_ident(&c.name), if c.decl_type.is_empty() { "TEXT" } else { &c.decl_type }); // 构造 ALTER ADD COLUMN
                        if let Some(def) = &c.default { if !def.is_empty() { d.push_str(&format!(" DEFAULT {}", def)); } } // 追加默认值
                        eng.conn.execute(&d, [])?; // 执行
                    } // if 结束
                } // for 结束
            } else { // 否则（新表）
                let sql = format!("CREATE TABLE {} ({})", db::quote_ident(&name), colsdef); // 构造 CREATE TABLE
                eng.conn.execute(&sql, [])?; // 执行
            } // if 结束
            Ok(()) // 返回 Ok
        })(); // 执行闭包
        match res { // 匹配结果
            Ok(_) => { // 成功
                self.log(format!("已保存表结构: {}", name)); // 记录日志
                if let TabKind::Designer(d) = &mut self.tabs[idx].kind { // 取设计器标签
                    d.error.clear(); // 清错误
                    d.original = Some(name.clone()); // 更新原表名
                } // if 结束
                self.open_table(conn_id, name); // 打开表标签
            } // 分支结束
            Err(e) => { // 失败
                if let TabKind::Designer(d) = &mut self.tabs[idx].kind { d.error = e.to_string(); } // 设错误信息
            } // 分支结束
        } // match 结束
    } // 函数结束

    pub fn open_er(&mut self, conn_id: String) { // 打开 ER 图
        if !self.ensure_engine(&conn_id) { return; } // 若引擎未就绪返回
        let eng = self.engines.get(&conn_id).unwrap(); // 取引擎
        let objs = eng.objects(); // 对象列表
        let mut tables = Vec::new(); // 表集合
        for o in objs.iter().filter(|o| o.kind == crate::model::ObjKind::Table) { // 遍历表
            let cols = eng.columns(&o.name).into_iter().map(|c| c.name).collect(); // 取列名
            tables.push((o.name.clone(), cols)); // 加入表集合
        } // for 结束
        let fks = eng.foreign_keys(); // 取外键
        let mut pos = HashMap::new(); // 位置映射
        let mut x = 40.0; // 初始 x
        let mut y = 40.0; // 初始 y
        for (i, (name, _)) in tables.iter().enumerate() { // 遍历表布局
            pos.insert(name.clone(), (x, y)); // 记录位置
            x += 260.0; // x 步进
            if (i + 1) % 3 == 0 { x = 40.0; y += 220.0; } // 每 3 个换行
        } // if 结束
        self.push_tab(TabKind::Er(ErDoc { conn_id, tables, fks, pos, zoom: 1.0 })); // 推入 ER 标签
    } // 函数结束

    pub fn open_query_builder(&mut self, conn_id: String) { // 打开查询构建器
        if !self.ensure_engine(&conn_id) { return; } // 若引擎未就绪返回
        let eng = self.engines.get(&conn_id).unwrap(); // 取引擎
        let tables = eng.objects(); // 对象列表
        let first = tables.first().map(|t| t.name.clone()).unwrap_or_default(); // 取首表名
        let cols = if first.is_empty() { Vec::new() } else { eng.columns(&first).into_iter().map(|c| (c.name, true)).collect() }; // 取列
        self.push_tab(TabKind::QueryBuilder(QbDoc { // 推入查询构建标签
            conn_id, // 连接 id
            table: first, // 表
            cols, // 列
            where_col: String::new(), // 空条件列
            where_op: "=".into(), // 默认运算符
            where_val: String::new(), // 空条件值
            order_col: String::new(), // 空排序列
            desc: false, // 非降序
            limit: "100".into(), // 限制 100
            preview: QueryResult::default(), // 空预览
            status: String::new(), // 空状态
        })); // 结构结束
    } // 函数结束

    pub fn open_export(&mut self, conn_id: String, table: String) { // 打开导出面板
        self.push_tab(TabKind::Export(ExportDoc { conn_id, table, format: "CSV".into(), path: default_export_path(), status: String::new() })); // 推入导出标签
    } // 函数结束
    pub fn open_import(&mut self, conn_id: String, table: String) { // 打开导入面板
        self.push_tab(TabKind::Import(ImportDoc { conn_id, table, format: "CSV".into(), path: String::new(), status: String::new() })); // 推入导入标签
    } // 函数结束
    pub fn open_backup(&mut self, conn_id: String) { // 打开备份面板
        self.push_tab(TabKind::Backup(BackupDoc { conn_id, path: default_backup_path(), status: String::new() })); // 推入备份标签
    } // 函数结束
    pub fn open_monitor(&mut self, conn_id: String) { // 打开监控面板
        self.push_tab(TabKind::Monitor(MonitorDoc { conn_id, auto_refresh: true })); // 推入监控标签
    } // 函数结束
    pub fn open_profiling(&mut self, conn_id: String, table: String) { // 打开剖析面板
        let stats = self.profile_stats(&conn_id, &table); // 计算统计
        let computed = !stats.is_empty(); // 是否已计算
        self.push_tab(TabKind::Profiling(ProfileDoc { conn_id, table, stats, computed })); // 推入剖析标签
    } // 函数结束

    /// Column-level statistics used by both the Profiling page and the demo autoload.
    fn profile_stats(&self, conn_id: &str, table: &str) -> Vec<(String, String)> { // 列级统计函数
        let mut stats = Vec::new(); // 统计集合
        if let Some(e) = self.engines.get(conn_id) { // 若引擎存在
            let total = e.count(table); // 总行数
            for c in e.columns(table) { // 遍历列
                let cn = crate::db::quote_ident(&c.name); // 列名
                let nulls: i64 = e // 查询空值数
                    .conn // 查询
                    .query_row(&format!("SELECT COUNT(*) FROM {} WHERE {} IS NULL", crate::db::quote_ident(table), cn), [], |r| r.get(0)) // 默认 0
                    .unwrap_or(0); // 查询唯一值数
                let distinct: i64 = e // 查询
                    .conn // 默认 0
                    .query_row(&format!("SELECT COUNT(DISTINCT {}) FROM {}", cn, crate::db::quote_ident(table)), [], |r| r.get(0)) // 加入行数统计
                    .unwrap_or(0); // 加入空值/唯一值统计
                stats.push((format!("{} 行数", c.name), total.to_string())); // for 结束
                stats.push((format!("{} 空值/唯一值", c.name), format!("{} / {}", nulls, distinct))); // 取样本
            } // 加入样本统计
            let top = e.query(&format!("SELECT * FROM {} LIMIT 3", crate::db::quote_ident(table))); // if 结束
            stats.push(("样本".into(), format!("{} 行预览", top.rows.len()))); // if 结束
        } // 返回统计
        stats // 函数结束
    } // 块结束
    pub fn open_datagen(&mut self, conn_id: String, table: String) { // 打开数据生成
        self.push_tab(TabKind::DataGen(GenDoc { conn_id, table, count: 50, status: String::new() })); // 推入标签
    } // 函数结束
    pub fn open_chart(&mut self, conn_id: String) { // 打开图表
        let sql = "SELECT country, COUNT(*) AS cnt FROM customers GROUP BY country ORDER BY cnt DESC".to_string(); // 默认聚合 SQL
        // Pre-compute so the chart renders immediately (acceptance/screenshot friendly).
        let data = if let Some(e) = self.engines.get(&conn_id) { e.query(&sql) } else { QueryResult::default() }; // 预计算数据以便立即渲染
        self.push_tab(TabKind::Chart(ChartDoc { // 推入图表标签
            conn_id, // 连接 id
            sql, // SQL
            data, // 数据
            kind: ChartKind::Bar, // 默认柱状图
            label_col: 0, // 标签列 0
            value_col: 1, // 数值列 1
            status: String::new(), // 空状态
        })); // 结构结束
    } // 函数结束
    pub fn open_ai(&mut self, conn_id: String) { // 打开 AI 助手
        self.push_tab(TabKind::Ai(AiDoc { conn_id, prompt: String::new(), answer: String::new() })); // 推入 AI 标签
    } // 函数结束
    pub fn open_dict(&mut self, conn_id: String, table: String) { // 打开数据字典
        self.push_tab(TabKind::Dict(DictDoc { conn_id, table })); // 推入字典标签
    } // 函数结束
    pub fn open_snippets(&mut self) { // 打开代码片段
        self.push_tab(TabKind::Snippet(SnippetDoc { selected: None, edit_name: String::new(), edit_sql: String::new() })); // 推入片段标签
    } // 函数结束
    pub fn open_report(&mut self, conn_id: String) { // 打开报表
        self.push_tab(TabKind::Report(ReportDoc { conn_id, title: "销售报表".into(), sql: "SELECT * FROM v_order_summary".into(), status: String::new() })); // 推入报表标签
    } // 函数结束
    pub fn open_automation(&mut self, conn_id: String) { // 打开自动化
        self.push_tab(TabKind::Automation(AutoDoc { conn_id, tasks: vec!["每日备份 (每日 02:00)".into(), "数据同步 customers -> warehouse".into()] })); // 推入自动化标签
    } // 函数结束
    pub fn open_users(&mut self, conn_id: String) { // 打开用户与权限
        self.push_tab(TabKind::Users(UsersDoc { conn_id })); // 推入用户标签
    } // 函数结束
    pub fn open_masking(&mut self, conn_id: String, table: String) { // 打开数据脱敏
        if !self.ensure_engine(&conn_id) { // 若引擎未就绪
            return; // 返回
        } // if 结束
        let column = self // 取首个非主键列
            .engines // 引擎表
            .get(&conn_id) // 取列
            .map(|e| e.columns(&table).into_iter().find(|c| !c.pk).map(|c| c.name).unwrap_or_default()) // 否则空
            .unwrap_or_default(); // 默认空
        if table.is_empty() { // 若表名为空
            let t = self // 取首个表
                .engines // 引擎表
                .get(&conn_id) // 取
                .and_then(|e| e.objects().into_iter().find(|o| o.kind == crate::model::ObjKind::Table).map(|o| o.name)) // 默认空
                .unwrap_or_default(); // 若仍为空
            if t.is_empty() { // 返回
                return; // if 结束
            } // 递归打开
            return self.open_masking(conn_id, t); // if 结束
        } // 推入脱敏标签
        self.push_tab(TabKind::Masking(MaskingDoc { conn_id, table, column, rule: "全遮蔽 (***)".into(), status: String::new() })); // 结构结束
    } // 函数结束
    pub fn open_sync(&mut self, conn_id: String) { // 打开同步
        let dst = self.store.connections.iter().find(|c| c.id != conn_id).map(|c| c.id.clone()).unwrap_or_else(|| conn_id.clone()); // 目标连接
        let table = self // 取首表
            .engines // 引擎表
            .get(&conn_id) // 取
            .and_then(|e| e.objects().into_iter().find(|o| o.kind == crate::model::ObjKind::Table).map(|o| o.name)) // 默认空
            .unwrap_or_default(); // 推入同步标签
        self.push_tab(TabKind::Sync(SyncDoc { src_conn: conn_id, dst_conn: dst, table, mode: "数据同步".into(), diff: Vec::new(), status: String::new() })); // 结构结束
    } // 函数结束
    pub fn open_history(&mut self) { // 打开查询历史
        if let Some(p) = self.tabs.iter().position(|t| matches!(t.kind, TabKind::History(_))) { // 若已存在历史标签
            self.active = p; // 激活
        } else { // 否则
            self.push_tab(TabKind::History(HistoryDoc { selected: None })); // 推入历史标签
        } // if 结束
    } // 函数结束
    pub fn open_objects(&mut self, conn_id: String, table: String) { // 打开对象管理
        if !self.ensure_engine(&conn_id) { return; } // 若引擎未就绪返回
        let table = if table.is_empty() { // 若表名为空
            self.engines // 引擎表
                .get(&conn_id) // 取
                .and_then(|e| e.objects().into_iter().find(|o| o.kind == crate::model::ObjKind::Table).map(|o| o.name)) // 默认空
                .unwrap_or_default() // 否则保持
        } else { table }; // if 结束
        self.push_tab(TabKind::Objects(ObjectsDoc { conn_id, table, kind: "索引".into(), new_name: String::new(), new_sql: String::new(), unique: false, status: String::new() })); // 推入对象标签
    } // 函数结束
    pub fn open_pump(&mut self, conn_id: String) { // 打开数据泵
        let path = default_export_dir().join("dump.sql").to_string_lossy().to_string(); // 默认导出路径
        self.push_tab(TabKind::Pump(PumpDoc { conn_id, table: String::new(), path, status: String::new() })); // 推入数据泵标签
    } // 函数结束
    pub fn open_schema(&mut self, conn_id: String) { // 打开模式分析
        self.push_tab(TabKind::Schema(SchemaDoc { conn_id, rows: Vec::new(), computed: false })); // 推入标签
    } // 函数结束
    pub fn open_outline(&mut self, conn_id: String) { // 打开 SQL 大纲
        self.push_tab(TabKind::Outline(OutlineDoc { // 推入标签
            conn_id, // 连接 id
            sql: "SELECT c.name, c.country, COUNT(o.id) AS orders\nFROM customers c\nLEFT JOIN orders o ON o.customer_id = c.id\nGROUP BY c.id\nORDER BY orders DESC;".into(), // 默认分析 SQL
            message: String::new(), // 空消息
        })); // 结构结束
    } // 函数结束
    pub fn open_model(&mut self, conn_id: String) { // 打开模型设计器
        if !self.ensure_engine(&conn_id) { return; } // 若引擎未就绪返回
        let (tables, fks) = self.model_tables(&conn_id); // 取表与外键
        let cmp = self.store.connections.iter().find(|c| c.id != conn_id).map(|c| c.id.clone()).unwrap_or_else(|| conn_id.clone()); // 对比连接
        let sql_path = default_export_dir().join("model.sql").to_string_lossy().to_string(); // SQL 路径
        self.push_tab(TabKind::Model(ModelDoc { // 推入模型标签
            conn_id, // 连接 id
            view: "逻辑模型".into(), // 默认逻辑模型
            tables, // 表
            fks, // 外键
            cmp_conn: cmp, // 对比连接
            diff: Vec::new(), // 空差异
            status: String::new(), // 空状态
            sql_path, // SQL 路径
        })); // 结构结束
    } // 函数结束
    pub fn reload_model(&self, conn_id: &str) -> (Vec<(String, Vec<String>)>, Vec<(String, String, String, String)>) { // 重载模型
        self.model_tables(conn_id) // 返回表与外键
    } // 函数结束
    fn model_tables(&self, conn_id: &str) -> (Vec<(String, Vec<String>)>, Vec<(String, String, String, String)>) { // 取模型的表与外键
        if let Some(eng) = self.engines.get(conn_id) { // 若引擎存在
            let mut tables = Vec::new(); // 表集合
            for o in eng.objects().into_iter().filter(|o| o.kind == crate::model::ObjKind::Table) { // 遍历表
                let cols = eng.columns(&o.name).into_iter().map(|c| format!("{} {}", c.name, c.decl_type)).collect(); // 取列（名 类型）
                tables.push((o.name, cols)); // 加入
            } // for 结束
            (tables, eng.foreign_keys()) // 返回表与外键
        } else { // 否则
            (Vec::new(), Vec::new()) // 返回空
        } // if 结束
    } // 函数结束
    pub fn open_find(&mut self, conn_id: String) { // 打开查找于数据库
        self.push_tab(TabKind::Find(FindDoc { conn_id, needle: String::new(), hits: Vec::new(), status: String::new() })); // 推入标签
    } // 函数结束
    pub fn open_transfer(&mut self, conn_id: String) { // 打开数据传输
        let src = conn_id; // 源连接
        let dst = self.store.connections.first().map(|c| c.id.clone()).unwrap_or(src.clone()); // 目标连接
        let table = if let Some(eng) = self.engines.get(&src) { // 取首表
            eng.objects().into_iter().find(|o| o.kind == crate::model::ObjKind::Table).map(|o| o.name).unwrap_or_default() // 表名
        } else { String::new() }; // 否则空
        let cols = if let Some(eng) = self.engines.get(&src) { // 列
            if table.is_empty() { Vec::new() } else { eng.columns(&table).into_iter().map(|c| (c.name, true)).collect() } // 若表名为空
        } else { Vec::new() }; // 取列或空
        let dst_table = table.clone(); // 否则
        self.push_tab(TabKind::Transfer(TransferDoc { src_conn: src, dst_conn: dst, table, dst_table, cols, where_clause: String::new(), status: String::new() })); // 目标表与源表同名
    } // 推入传输标签
// 结构结束
    pub fn open_settings(&mut self) { // 函数结束
        if let Some(p) = self.tabs.iter().position(|t| matches!(t.kind, TabKind::Settings)) { // 打开设置
            self.active = p; // 若已存在设置标签
        } else { // 激活
            self.push_tab(TabKind::Settings); // 否则
        } // 推入设置标签
    } // if 结束
// 函数结束
    pub fn new_connection(&mut self, kind: DbKind) { // 新建连接
        self.dialog = Some(Dialog::Conn(ConnectionForm { conn: Connection::new(kind), is_new: true, error: String::new() })); // 打开新建连接对话框
    } // 函数结束
    pub fn edit_connection(&mut self, id: String) { // 编辑连接
        if let Some(c) = self.conn(&id) { // 若连接存在
            self.dialog = Some(Dialog::Conn(ConnectionForm { conn: c.clone(), is_new: false, error: String::new() })); // 打开编辑对话框
        } // if 结束
    } // 函数结束

    pub fn commit_connection(&mut self, conn: Connection, is_new: bool) { // 提交连接（新增或更新）
        if conn.name.trim().is_empty() { // 名称为空则返回
            return; // 返回
        } // if 结束
        if is_new { // 若为新建
            self.store.connections.push(conn.clone()); // 追加连接
        } else if let Some(existing) = self.store.connections.iter_mut().find(|c| c.id == conn.id) { // 否则若找到已有
            *existing = conn.clone(); // 覆盖
        } // if 结束
        self.engines.remove(&conn.id); // 移除旧引擎缓存
        self.store.save(); // 保存
        self.log(format!("已保存连接: {}", conn.name)); // 记录日志
        self.open_connection(conn.id); // 打开连接
    } // 函数结束

    pub fn delete_connection(&mut self, id: String) { // 删除连接
        self.engines.remove(&id); // 移除引擎
        self.store.connections.retain(|c| c.id != id); // 移除连接
        self.store.save(); // 保存
        self.log("已删除连接"); // 记录日志
    } // 函数结束

    pub fn request_delete_connection(&mut self, id: String) { // 请求删除连接（确认框）
        let name = self.conn(&id).map(|c| c.name.clone()).unwrap_or_default(); // 取连接名
        self.dialog = Some(Dialog::Confirm(ConfirmData { // 弹出确认对话框
            title: "删除连接".into(), // 标题
            msg: format!("确定要删除连接 “{}” 吗？此操作不可撤销。", name), // 消息
            action: Act::DeleteConnection(id), // 动作
        })); // 结构结束
    } // 函数结束

    pub fn process_act(&mut self, act: Act) { // 处理动作
        match act { // 匹配动作
            Act::DeleteConnection(id) => self.delete_connection(id), // 删除连接
            Act::DropTable(conn_id, table) => { // 删表
                if let Some(eng) = self.engines.get(&conn_id) { // 若引擎存在
                    let _ = eng.conn.execute(&format!("DROP TABLE IF EXISTS {}", db::quote_ident(&table)), []); // 执行 DROP TABLE
                } // if 结束
                self.log(format!("已删除表: {table}")); // 记录日志
                self.tabs.retain(|t| !matches!(&t.kind, TabKind::Table(td) if td.conn_id==conn_id && td.table==table)); // 移除相关表标签
            } // 分支结束
            Act::DeleteRow(conn_id, table, rowid) => { // 删行
                if let Some(eng) = self.engines.get(&conn_id) { // 若引擎存在
                    let _ = eng.delete_row(&table, rowid); // 删除行
                } // if 结束
                for i in 0..self.tabs.len() { // 遍历标签
                    if matches!(&self.tabs[i].kind, TabKind::Table(td) if td.conn_id == conn_id && td.table == table) { // 若匹配则刷新
                        self.refresh_table_doc(i); // 刷新
                    } // if 结束
                } // for 结束
                self.log("已删除行"); // 记录日志
            } // 分支结束
            Act::OpenSqliteFile(conn_id) => { // 打开 SQLite 文件
                // default sample file in work dir
                let path = default_export_dir().join("sampledb.sqlite").to_string_lossy().to_string(); // 默认示例文件路径
                if !std::path::Path::new(&path).exists() { // 若不存在
                    if let Ok(e) = SqliteEngine::open(&path) { let _ = e.seed_demo(); } // 创建并填充数据
                } // if 结束
                if let Some(c) = self.store.connections.iter_mut().find(|c| c.id == conn_id) { // 若找到连接
                    c.sqlite_path = path.clone(); // 设置路径
                } // if 结束
                self.engines.remove(&conn_id); // 移除引擎缓存
                self.store.save(); // 保存
                self.open_connection(conn_id); // 打开连接
                self.log(format!("SQLite 文件: {path}")); // 记录日志
            } // 分支结束
            Act::ExportPath(_) | Act::BackupPath(_) | Act::Noop => {} // 其它动作无操作
        } // match 结束
        self.dialog = None; // 关闭对话框
    } // 函数结束

    pub fn save_snippets(&mut self) { // 保存代码片段
        self.store.save(); // 保存
    } // 函数结束
    pub fn add_snippet(&mut self, name: String, sql: String) { // 添加代码片段
        if name.trim().is_empty() { return; } // 名称为空则返回
        self.store.snippets.push(Snippet { name, sql }); // 追加片段
        self.store.save(); // 保存
    } // 函数结束

    pub fn toggle_theme(&mut self, ctx: &egui::Context) { // 切换主题
        let next = if self.dark { "light" } else { "dark" }; // 计算下一个主题
        self.set_theme(ctx, next); // 应用主题
    } // 函数结束

    /// 切换并持久化指定主题（key 见 `theme::THEMES`）。
    pub fn set_theme(&mut self, ctx: &egui::Context, key: &str) { // 设置并持久化主题
        self.theme = key.to_string(); // 更新主题 key
        self.dark = theme::is_dark(key); // 是否深色
        self.store.settings.theme = key.to_string(); // 写设置
        self.store.dark_mode = self.dark; // 写 dark_mode
        self.pal = theme::apply_theme(ctx, key); // 应用主题
        self.store.save(); // 保存
    } // 函数结束
} // impl 结束

fn summarize(results: &[QueryResult]) -> String { // 结果摘要函数
    let mut lines = Vec::new(); // 行集合
    for r in results { // 遍历结果
        if let Some(e) = &r.error { // 若有错误
            lines.push(format!("错误: {}", e)); // 错误行
        } else if r.is_select { // 否则若为 SELECT
            lines.push(format!("查询成功: {} 行 ({:.1} ms)", r.rows.len(), r.elapsed_ms)); // 查询成功行
        } else { // 否则
            lines.push(format!("执行成功: {} 行受影响 ({:.1} ms)", r.affected, r.elapsed_ms)); // 执行成功行
        } // if 结束
    } // for 结束
    if lines.is_empty() { "无语句".into() } else { lines.join(" | ") } // 空则“无语句”
} // 函数结束

fn install_fonts(ctx: &egui::Context) { // 安装中文字体
    let mut fonts = egui::FontDefinitions::default(); // 字体定义
    let cjk = egui::FontData::from_owned(include_bytes!("../assets/NotoSansSC-Regular.ttf").to_vec()); // 载入内置中文字体
    fonts.font_data.insert("noto_sc".to_owned(), cjk); // 插入字体数据
    if let Some(fam) = fonts.families.get_mut(&egui::FontFamily::Proportional) { // 若存在比例字体族
        fam.push("noto_sc".to_owned()); // 追加
    } // if 结束
    if let Some(fam) = fonts.families.get_mut(&egui::FontFamily::Monospace) { // 若存在等宽字体族
        fam.push("noto_sc".to_owned()); // 追加
    } // if 结束
    ctx.set_fonts(fonts); // 设置字体
} // 函数结束

pub fn short_id(id: &str) -> String { // 取短 id
    id.chars().take(4).collect() // 取前 4 个字符
} // 函数结束

pub fn default_export_dir() -> std::path::PathBuf { // 默认导出目录
    let d = std::env::var("USERPROFILE").map(|p| std::path::PathBuf::from(p).join("Documents").join("navicat-rs")).unwrap_or_else(|_| std::path::PathBuf::from(".")); // 计算目录路径
    std::fs::create_dir_all(&d).ok(); // 创建目录
    d // 返回目录
} // 函数结束
pub fn default_export_path() -> String { // 默认导出文件路径
    default_export_dir().join("export.csv").to_string_lossy().to_string() // 拼接路径
} // 函数结束
pub fn default_backup_path() -> String { // 默认备份文件路径
    default_export_dir().join("backup.sqlite").to_string_lossy().to_string() // 拼接路径
} // 函数结束

impl eframe::App for NavicatApp { // 实现 eframe::App
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) { // 每帧更新
        crate::ui::draw(self, ctx); // 委托给 UI 绘制
    } // 函数结束
} // impl 结束
