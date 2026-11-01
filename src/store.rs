use crate::model::Connection;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Store {
    #[serde(default)]
    pub connections: Vec<Connection>,
    #[serde(default)]
    pub history: Vec<String>,
    #[serde(default)]
    pub snippets: Vec<Snippet>,
    #[serde(default)]
    pub favorites: Vec<String>,
    #[serde(default)]
    pub dark_mode: bool,
    #[serde(default)]
    pub settings: Settings,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Snippet {
    pub name: String,
    pub sql: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Settings {
    #[serde(default = "default_true")]
    pub confirm_delete: bool,
    #[serde(default = "default_page")]
    pub page_size: usize,
    #[serde(default)]
    pub auto_commit: bool,
    #[serde(default = "default_zoom")]
    pub tree_zoom: f32,
    #[serde(default)]
    pub theme: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { confirm_delete: true, page_size: 200, auto_commit: true, tree_zoom: 1.0, theme: String::new() }
    }
}
fn default_true() -> bool {
    true
}
fn default_page() -> usize {
    200
}
fn default_zoom() -> f32 {
    1.0
}

impl Store {
    pub fn dir() -> PathBuf {
        let base = dirs_data();
        std::fs::create_dir_all(&base).ok();
        base
    }
    pub fn path() -> PathBuf {
        Self::dir().join("navicat-rs.json")
    }
    pub fn load() -> Self {
        match std::fs::read_to_string(Self::path()) {
            Ok(s) => serde_json::from_str(&s).unwrap_or_default(),
            Err(_) => Store::default(),
        }
    }
    pub fn save(&self) {
        if let Ok(s) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(Self::path(), s);
        }
    }
}

fn dirs_data() -> PathBuf {
    // 跨平台配置目录：
    //  - Windows: %APPDATA%\navicat-rs
    //  - macOS:   ~/Library/Application Support/navicat-rs
    //  - Linux:   $XDG_CONFIG_HOME/navicat-rs 或 ~/.config/navicat-rs
    #[cfg(target_os = "windows")]
    {
        if let Ok(p) = std::env::var("APPDATA") {
            return PathBuf::from(p).join("navicat-rs");
        }
    }
    #[cfg(target_os = "macos")]
    {
        if let Ok(home) = std::env::var("HOME") {
            return PathBuf::from(home).join("Library").join("Application Support").join("navicat-rs");
        }
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
            return PathBuf::from(xdg).join("navicat-rs");
        }
        if let Ok(home) = std::env::var("HOME") {
            return PathBuf::from(home).join(".config").join("navicat-rs");
        }
    }
    PathBuf::from(".").join("navicat-rs-data")
}
