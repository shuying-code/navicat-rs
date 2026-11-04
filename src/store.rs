//! 本地配置持久化层：把连接、历史、片段、收藏与设置保存为 JSON 文件。
//! 配置文件位置遵循各平台约定（见 `dirs_data`）。

use crate::model::Connection; // 引入连接配置类型
use serde::{Deserialize, Serialize}; // 引入 serde 序列化/反序列化派生宏
use std::path::PathBuf; // 引入路径类型

/// 应用的全部可持久化状态。
#[derive(Clone, Serialize, Deserialize, Debug, Default)] // 派生：可克隆、可序列化、可调试、可默认
pub struct Store { // 顶层存储结构
    #[serde(default)] // 缺失字段时使用默认值（兼容旧配置）
    pub connections: Vec<Connection>, // 连接列表
    #[serde(default)] // 缺失字段时使用默认值
    pub history: Vec<String>, // SQL 执行历史
    #[serde(default)] // 缺失字段时使用默认值
    pub snippets: Vec<Snippet>, // 代码片段
    #[serde(default)] // 缺失字段时使用默认值
    pub favorites: Vec<String>, // 收藏项（对象名/连接 id）
    #[serde(default)] // 缺失字段时使用默认值
    pub dark_mode: bool, // 是否深色模式（旧字段，兼容用）
    #[serde(default)] // 缺失字段时使用默认值
    pub settings: Settings, // 应用设置
} // 结构体结束

/// 一条可复用的 SQL 代码片段。
#[derive(Clone, Serialize, Deserialize, Debug)] // 派生：可克隆、可序列化、可调试
pub struct Snippet { // 片段结构体
    pub name: String, // 片段名称
    pub sql: String, // 片段内容（SQL 文本）
} // 结构体结束

/// 应用级设置项。
#[derive(Clone, Serialize, Deserialize, Debug)] // 派生：可克隆、可序列化、可调试
pub struct Settings { // 设置结构体
    #[serde(default = "default_true")] // 缺失时使用 default_true
    pub confirm_delete: bool, // 删除前是否二次确认
    #[serde(default = "default_page")] // 缺失时使用 default_page
    pub page_size: usize, // 每页显示行数
    #[serde(default)] // 缺失时使用默认值
    pub auto_commit: bool, // 是否自动提交
    #[serde(default = "default_zoom")] // 缺失时使用 default_zoom
    pub tree_zoom: f32, // 对象树缩放比例
    #[serde(default)] // 缺失时使用默认值
    pub theme: String, // 主题 key
} // 结构体结束

impl Default for Settings { // 为 Settings 提供默认实现
    fn default() -> Self { // 返回默认设置
        Settings { confirm_delete: true, page_size: 200, auto_commit: true, tree_zoom: 1.0, theme: String::new() } // 各字段默认值
    } // default 结束
} // impl 结束
fn default_true() -> bool { // serde 默认值函数：布尔默认 true
    true // 返回 true
} // 结束
fn default_page() -> usize { // serde 默认值函数：每页 200 行
    200 // 返回 200
} // 结束
fn default_zoom() -> f32 { // serde 默认值函数：缩放系数 1.0
    1.0 // 返回 1.0
} // 结束

impl Store { // 为 Store 实现读写方法
    /// 返回配置目录路径，并确保其存在。
    pub fn dir() -> PathBuf { // 计算并创建配置目录
        let base = dirs_data(); // 计算平台相关目录
        std::fs::create_dir_all(&base).ok(); // 尝试创建目录（忽略失败）
        base // 返回目录路径
    } // dir 结束
    /// 返回配置文件的完整路径。
    pub fn path() -> PathBuf { // 计算配置文件路径
        Self::dir().join("navicat-rs.json") // 目录 + 固定文件名
    } // path 结束
    /// 从磁盘读取配置；文件不存在或解析失败时返回默认值。
    pub fn load() -> Self { // 载入配置
        match std::fs::read_to_string(Self::path()) { // 读取文件内容
            Ok(s) => serde_json::from_str(&s).unwrap_or_default(), // 解析 JSON，失败则用默认值
            Err(_) => Store::default(), // 文件不存在则用默认值
        } // match 结束
    } // load 结束
    /// 将配置美化后写回磁盘。
    pub fn save(&self) { // 保存配置
        if let Ok(s) = serde_json::to_string_pretty(self) { // 序列化为美化 JSON
            let _ = std::fs::write(Self::path(), s); // 写入文件（忽略失败）
        } // if 结束
    } // save 结束
} // impl 结束

/// 计算跨平台配置目录。
fn dirs_data() -> PathBuf { // 依据平台返回配置目录
    // 跨平台配置目录：
    //  - Windows: %APPDATA%\navicat-rs
    //  - macOS:   ~/Library/Application Support/navicat-rs
    //  - Linux:   $XDG_CONFIG_HOME/navicat-rs 或 ~/.config/navicat-rs
    #[cfg(target_os = "windows")] // Windows 分支
    { // 语句
        if let Ok(p) = std::env::var("APPDATA") { // 读取 APPDATA 环境变量
            return PathBuf::from(p).join("navicat-rs"); // 返回 %APPDATA%\navicat-rs
        } // if 结束
    } // windows 分支结束
    #[cfg(target_os = "macos")] // macOS 分支
    { // 语句
        if let Ok(home) = std::env::var("HOME") { // 读取 HOME 环境变量
            return PathBuf::from(home).join("Library").join("Application Support").join("navicat-rs"); // 返回标准应用支持目录
        } // if 结束
    } // macos 分支结束
    #[cfg(all(unix, not(target_os = "macos")))] // Linux 及其它 unix 分支
    { // 语句
        if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") { // 优先 XDG_CONFIG_HOME
            return PathBuf::from(xdg).join("navicat-rs"); // 返回 XDG 配置目录
        } // if 结束
        if let Ok(home) = std::env::var("HOME") { // 其次 HOME
            return PathBuf::from(home).join(".config").join("navicat-rs"); // 返回 ~/.config/navicat-rs
        } // if 结束
    } // unix 分支结束
    PathBuf::from(".").join("navicat-rs-data") // 兜底：当前目录下
} // dirs_data 结束
