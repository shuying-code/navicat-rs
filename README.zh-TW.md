<p align="center">🌐 <b>語言</b>：<a href="README.md">简体中文</a> · <a href="README.en.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.ru.md">Русский</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.de.md">Deutsch</a> · <a href="README.pt-BR.md">Português</a></p>

# navicat-rs

一套以 **Rust + egui/eframe** 撰寫的**跨平台單一檔案**資料庫管理桌面用戶端，介面與工作流程參考 Navicat Premium 18。
**所有功能全部免費開放，無登入、無試用、無授權/權限限制。**

<p>
  <img alt="CI" src="https://github.com/shuying-code/navicat-rs/actions/workflows/ci.yml/badge.svg">
  <img alt="build" src="https://github.com/shuying-code/navicat-rs/actions/workflows/build.yml/badge.svg">
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.75%2B-orange">
  <img alt="Platform" src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-2f6fed">
  <img alt="License" src="https://img.shields.io/badge/license-MIT-green">
  <img alt="Release" src="https://img.shields.io/github/v/release/shuying-code/navicat-rs?display_name=tag">
</p>

> 本專案為**本地優先**的獨立實作，不連接任何雲端、不含登入、訂閱、試用、遙測。**所有功能對任何人無條件可用。**

![歡迎頁](assets/screenshot-welcome.png)

---

## 為何「無限制」

Navicat 的付費與登入主要約束的是**雲端 / 訂閱**能力（雲端同步、團隊協作、雲端自動化等）。本工具只實作**不依賴雲端**的本地資料庫管理能力，因此天生不需要帳號、不需要付費、沒有配額。設計上移除了登入 / 訂閱 / 授權限制，所有本地功能無條件可用。

---

## 功能

- **資料表**：分頁瀏覽、過濾、內聯編輯、增刪列、排序（點擊表頭）、匯出。
- **SQL 查詢**：多語句執行、`Ctrl+Enter`、多網格結果、耗時 / 列數、Explain 執行計畫樹、SQL 美化、SQL 大綱 / 重構建議、查詢歷史。
- **設計與建模**：表結構設計器（欄位編輯 → DDL）、ER 圖、查詢建構器、模型設計器（概念/邏輯/實體 · 反向工程 · 比較 · 同步）。
- **資料搬移**：資料傳輸（欄位對映 + WHERE 過濾）、資料/結構同步、匯入精靈（CSV/JSON/SQL）、匯出精靈（CSV/JSON/SQL/HTML）、資料泵。
- **維運與增強**：備份 / 還原（含記憶體庫 `VACUUM INTO`）、伺服器監控、資料剖析、模式分析、資料產生、資料脫敏、圖表、資料字典、報表。
- **物件與搜尋**：物件管理（索引/觸發器/檢視）、跨全表欄位值搜尋（尋找於資料庫）、物件樹縮放與分組獨立折疊。
- **其他**：AI 助手、自動化 / 排程任務、程式碼片段、使用者與權限（全部開放）、設定（多主題 / 樹縮放 / 自動提交 / 刪除確認）。
- **多主題**：淺色 / 深色 / 午夜藍 / 森林 / 暖沙 / 石墨；任意主題下正文與按鈕文字皆保持高對比可讀。

## 圖形介面（GUI）

- **純 Rust + egui/eframe**：單檔 exe，無需 WebView，雙擊即用；內嵌中文字型，無缺字方框。
- **三區佈局**：選單列 / 工具列 / 分頁列 / 左側連線與物件樹 / 右側功能頁 / 狀態列。
- **物件樹**：連線 → 表 / 檢視 / 備份 / 模型 / 使用者；可縮放（`− 100% ＋`、`Ctrl+滾輪`、設定頁滑桿）、可分組獨立折疊。
- **多分頁**：資料表、查詢、設計、ER、建構器及各工具頁並行開啟。
- **多主題**：`檢視 → 主題` 或 `設定 → 主題` 切換，選擇持久化。

## 支援平台

| 平台 | 狀態 | 說明 |
|---|---|---|
| Windows x64 | ✅ 已建置驗證 | 建議 MSVC 工具鏈；Release 提供預編譯執行檔 |
| macOS（Intel / Apple Silicon） | ✅ CI 建置（universal） | 由 `build.yml` 產出通用二進位檔 |
| Linux（X11 / Wayland） | ✅ CI 建置 | 由 `build.yml` 產出 x64 壓縮包（需系統 GUI 依賴） |

## 建置

需要 Rust 1.75+（建議 stable）。

```bash
cargo build --release
```

### Windows（MSVC）

```powershell
# 需已安裝 Visual Studio Build Tools（含 C++ 與 Windows SDK）
cargo build --release
```

### macOS

```bash
xcode-select --install   # 首次可能需要
cargo build --release
```

### Linux（Debian/Ubuntu）

```bash
sudo apt-get install -y build-essential pkg-config \
  libxkbcommon-dev libgtk-3-dev libgl1-mesa-dev \
  libx11-dev libxi-dev libxrandr-dev libxcursor-dev libxinerama-dev \
  libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
cargo build --release
```

## 預編譯 / 跨平台產物

- **Releases**：<https://github.com/shuying-code/navicat-rs/releases>（Windows x64 執行檔/zip、macOS universal tar.gz、Linux x64 tar.gz；由 `build.yml` 建置）。
- **CI 建置矩陣**：`.github/workflows/ci.yml` 在 `ubuntu-latest / macos-latest / windows-latest` 建置並執行 `--selftest`。
- **容器映像（GHCR）**：

```bash
docker pull ghcr.io/shuying-code/navicat-rs:latest
docker run --rm ghcr.io/shuying-code/navicat-rs:latest --selftest
```

## 使用

```bash
navicat-rs              # 啟動 GUI
navicat-rs --selftest   # 無頭自檢（SQLite 引擎 + 關鍵功能，22 項）
```

首次啟動會自動建立「範例資料庫 (SQLite)」（4 張表 + 1 個檢視 + 示範資料）。左側物件樹雙擊表即可瀏覽；頂部工具列可開啟查詢 / 設計表 / ER 圖 / 查詢建構 / 傳輸 / 匯入 / 匯出 / 備份 / 監控 / 圖表 / 剖析 / AI 等全部頁面。

## 工作區（本地設定）

設定與狀態（連線、歷史、收藏、主題、設定）保存在各平台的使用者設定目錄：

| 平台 | 路徑 |
|---|---|
| Windows | `%APPDATA%\navicat-rs\navicat-rs.json` |
| macOS | `~/Library/Application Support/navicat-rs/navicat-rs.json` |
| Linux | `$XDG_CONFIG_HOME/navicat-rs/navicat-rs.json` 或 `~/.config/navicat-rs/navicat-rs.json` |

## 驗收

- `--selftest` 覆蓋引擎與關鍵功能共 **22 項，全部通過**。
- CI 三平台（Windows / macOS / Ubuntu）建置 + 自檢 **全部 success**。
- 6 套主題逐一截圖核對正文 / 加粗標題 / 表頭 / 按鈕文字可讀性。

## 資料庫後端

| 引擎 | 狀態 |
|---|---|
| SQLite | **完全真實**：連線、查詢、CRUD、DDL、腳本、外鍵、PRAGMA（`rusqlite` bundled，跨平台） |
| PostgreSQL | **真實網路協定**：連線測試 + 真實查詢（純 Rust `postgres` 驅動） |
| MySQL / MariaDB / Oracle / SQL Server / MongoDB / Redis / Snowflake / 達夢 / ODBC | 連線設定 + 離線示範 schema（介面/工作流完整；驅動層已抽象，可依 `src/realdb.rs` 逐一接入） |

## 命令列參數

| 參數 | 說明 |
|---|---|
| （無） | 啟動 GUI |
| `--selftest` | 無頭執行功能自檢，列印 `[PASS]/[FAIL]` 並以退出碼回傳結果（0=全通過） |

## 環境變數（可選）

| 變數 | 說明 |
|---|---|
| `NAVICAT_RS_THEME` | 啟動時指定主題：`light`/`dark`/`midnight`/`forest`/`sand`/`graphite` |
| `NAVICAT_RS_DEMO` | `1` 開啟範例查詢與表；`all` 開啟全部功能頁（用於截圖/驗收） |
| `NAVICAT_RS_TAB` | 搭配 demo，切換到標題含該子串的分頁 |

## 常見問題（FAQ）

**Q：需要 Navicat 帳號或授權嗎？** 不需要。程式啟動即進主介面，無登入 / 啟用 / 試用提示。

**Q：資料存在哪裡？** 見「工作區」；Windows 為 `%APPDATA%\navicat-rs\navicat-rs.json`，資料庫本身由你連線的 SQLite 檔案決定。

**Q：為何 macOS 二進位在 CI 裡建置？** 本機為 Windows，macOS 通用二進位由 GitHub Actions 的 `macos-latest` 產出（見 `build.yml`）。

**Q：某個非 SQLite 引擎能真正連線嗎？** 目前 SQLite 完全真實、PostgreSQL 為真實網路協定；其餘引擎為連線設定 + 離線示範庫，可依 `src/realdb.rs` 擴充。

## 問題回報（Issues）

請使用 GitHub Issues：<https://github.com/shuying-code/navicat-rs/issues/new/choose>。提交前請先閱讀 **[使用回報指南](docs/FEEDBACK.md)**。維護者通常每週集中處理一次 issue。

## 授權

[MIT](LICENSE)
