# navicat-rs

<p align="center">🌐 <b>Language</b>：<a href="README.md">简体中文</a> · <a href="README.en.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.ru.md">Русский</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.de.md">Deutsch</a> · <a href="README.pt-BR.md">Português</a></p>

一个用 **Rust + egui/eframe** 编写的**跨平台单文件**数据库管理桌面客户端，界面与工作流参考 Navicat Premium 18。
**所有功能全部免费开放，无登录、无试用、无授权/权限限制。**

<p>
  <img alt="CI" src="https://github.com/shuying-code/navicat-rs/actions/workflows/ci.yml/badge.svg">
  <img alt="build" src="https://github.com/shuying-code/navicat-rs/actions/workflows/build.yml/badge.svg">
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.75%2B-orange">
  <img alt="Platform" src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-2f6fed">
  <img alt="License" src="https://img.shields.io/badge/license-MIT-green">
  <img alt="Release" src="https://img.shields.io/github/v/release/shuying-code/navicat-rs?display_name=tag">
</p>

> 本项目为**本地优先**的独立实现，不连接任何云端、不含登录、订阅、试用、遥测。**所有功能对任何人无条件可用。**

![欢迎页](assets/screenshot-welcome.png)

---

## 为什么「无限制」

Navicat 的付费与登录主要约束**云端 / 订阅**能力（云同步、团队协作、云端自动化等）。本工具只实现**不依赖云端**的本地数据库管理能力，因此天然不需要账号、不需要付费、没有配额。设计上移除了登录 / 订阅 / 授权限制，所有本地功能无条件可用。

---

## 功能

- **数据表**：分页浏览、过滤、内联编辑、增删行、排序（点击表头）、导出。
- **SQL 查询**：多语句执行、`Ctrl+Enter`、结果多网格、耗时 / 行数、Explain 执行计划树、SQL 美化、SQL 大纲 / 重构建议、查询历史。
- **设计与建模**：表结构设计器（列编辑 → DDL）、ER 图、查询构建器、模型设计器（概念 / 逻辑 / 物理 · 反向工程 · 对比 · 同步）。
- **数据搬运**：数据传输（列映射 + WHERE 过滤）、数据同步 / 结构同步、导入向导（CSV / JSON / SQL）、导出向导（CSV / JSON / SQL / HTML）、数据泵（整库 / 单表 SQL）。
- **运维与增强**：备份 / 还原（含内存库 `VACUUM INTO`）、服务器监控、数据剖析、模式分析、数据生成、数据脱敏、图表、数据字典、报表。
- **对象与查找**：对象管理（索引 / 触发器 / 视图）、跨全表列值搜索（查找于数据库）、对象树缩放与分组独立折叠。
- **其他**：AI 助手、自动化 / 计划任务、代码片段、用户与权限（全部开放）、设置（多主题 / 树缩放 / 自动提交 / 删除确认）。
- **多主题**：浅色 / 深色 / 午夜蓝 / 森林 / 暖沙 / 石墨，任意主题下正文与按钮文字均保持高对比可读。

### 与 Navicat 付费 / 云端能力的差异（有意不实现）

- 云端同步 / 团队协作 / 云端自动化 / 云端报表等需要账号或付费的**云端能力**。
- 本工具刻意去除登录、订阅、试用、会员等限制，所有**本地**功能无条件可用。

## 图形界面（GUI）

- **纯 Rust + egui/eframe**：单文件 exe，无需 WebView，双击即用；内嵌中文字体，无缺字方框。
- **三区布局**：菜单栏 / 工具栏 / 标签栏 / 左侧连接与对象树 / 右侧功能页 / 状态栏。
- **对象树**：连接 → 表 / 视图 / 备份 / 模型 / 用户；可缩放（`− 100% ＋`、`Ctrl+滚轮`、设置页滑杆）、可分组独立折叠。
- **多标签**：数据表、查询、设计、ER、构建器及各工具页并行打开，互不干扰。
- **多主题**：`查看 → 主题` 或 `设置 → 主题` 切换，选择持久化。

## 支持的平台

| 平台 | 状态 | 说明 |
|---|---|---|
| Windows x64 | ✅ 已构建验证 | 推荐 MSVC 工具链；Release 提供预编译可执行文件 |
| macOS（Intel / Apple Silicon） | ✅ CI 构建（universal） | 由 `build.yml` 产出通用二进制 |
| Linux（X11 / Wayland） | ✅ CI 构建 | 由 `build.yml` 产出 x64 压缩包（需系统 GUI 依赖） |

程序基于 `eframe/egui`，三大桌面平台使用同一套源码；配置文件按平台写入各自的标准目录（见「工作区」）。

## 构建

需要 Rust 1.75+（推荐 stable）。

```bash
cargo build --release
```

### Windows（MSVC）

```powershell
# 需已安装 Visual Studio Build Tools（含 C++ 与 Windows SDK）
cargo build --release
```

### macOS

```bash
xcode-select --install   # 首次可需安装命令行工具
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

## 预编译 / 跨平台产物

- **Releases**：<https://github.com/shuying-code/navicat-rs/releases>（Windows x64 可执行文件/zip、macOS universal tar.gz、Linux x64 tar.gz；由 `build.yml` 跨平台构建并附加）。
- **CI 构建矩阵**：`.github/workflows/ci.yml` 在 `ubuntu-latest / macos-latest / windows-latest` 三平台构建并运行 `--selftest`。
- **容器镜像（GHCR）**：

```bash
docker pull ghcr.io/shuying-code/navicat-rs:latest
docker run --rm ghcr.io/shuying-code/navicat-rs:latest --selftest
```

## 使用

```bash
navicat-rs              # 启动 GUI
navicat-rs --selftest   # 无头自检（SQLite 引擎层 + 关键功能，22 项）
```

首次启动会自动创建「示例数据库 (SQLite)」（4 张表 + 1 个视图 + 演示数据）。左侧对象树双击表即可浏览；顶部工具条可打开查询 / 设计表 / ER 图 / 查询构建 / 传输 / 导入 / 导出 / 备份 / 监控 / 图表 / 剖析 / AI 等全部页面。

## 工作区（本地配置）

配置与状态（连接、历史、收藏、主题、设置）保存在各平台的用户配置目录：

| 平台 | 路径 |
|---|---|
| Windows | `%APPDATA%\navicat-rs\navicat-rs.json` |
| macOS | `~/Library/Application Support/navicat-rs/navicat-rs.json` |
| Linux | `$XDG_CONFIG_HOME/navicat-rs/navicat-rs.json` 或 `~/.config/navicat-rs/navicat-rs.json` |

## 验收

- `--selftest` 覆盖引擎与关键功能共 **22 项，全部通过**。
- CI 三平台（Windows / macOS / Ubuntu）构建 + 自检 **全部 success**。
- 6 套主题逐一截图核对正文 / 加粗标题 / 表头 / 按钮文字可读性（`work/theme_*.png`）。

## 数据库后端

| 引擎 | 状态 |
|---|---|
| SQLite | **完全真实**：连接、查询、CRUD、DDL、脚本、外键、PRAGMA（`rusqlite` bundled，跨平台） |
| PostgreSQL | **真实网络协议**：连接测试 + 真实查询（纯 Rust `postgres` 驱动） |
| MySQL / MariaDB / Oracle / SQL Server / MongoDB / Redis / Snowflake / 达梦 / ODBC | 连接配置 + 离线演示 schema（界面 / 工作流完整；驱动层已抽象，可按 `src/realdb.rs` 模式逐个接入） |

## 命令行参数

| 参数 | 说明 |
|---|---|
| （无） | 启动 GUI |
| `--selftest` | 无头运行功能自检，打印 `[PASS]/[FAIL]` 并以退出码返回结果（0=全通过） |

## 环境变量（可选）

| 变量 | 说明 |
|---|---|
| `NAVICAT_RS_THEME` | 启动时指定主题：`light`/`dark`/`midnight`/`forest`/`sand`/`graphite` |
| `NAVICAT_RS_DEMO` | `1` 打开示例查询与表；`all` 打开全部功能页（用于截图/验收） |
| `NAVICAT_RS_TAB` | 配合 demo，切换到标题包含该子串的标签页 |

## 常见问题（FAQ）

**Q：需要 Navicat 账号或授权吗？**
不需要。程序启动即进主界面，无登录 / 激活 / 试用提示。

**Q：数据存在哪里？**
见「工作区」；Windows 为 `%APPDATA%\navicat-rs\navicat-rs.json`，数据库本身由你连接的 SQLite 文件决定。

**Q：为什么 macOS 二进制在 CI 里构建？**
本机为 Windows，macOS 通用二进制由 GitHub Actions 的 `macos-latest` 产出（见 `build.yml`）。

**Q：某个非 SQLite 引擎能真正连接吗？**
当前 SQLite 完全真实、PostgreSQL 为真实网络协议；其余引擎为连接配置 + 离线演示库，可按 `src/realdb.rs` 扩展。

**Q：暗色主题下文字看不清？**
已修复：所有主题下正文、加粗标题、表头、按钮文字均按对比度取色；如仍有异常请附截图反馈。

## 问题反馈（Issues）

遇到问题或有建议？欢迎通过 GitHub Issues 反馈：<https://github.com/shuying-code/navicat-rs/issues/new/choose>。提交前请先阅读 **[使用反馈指南](docs/FEEDBACK.md)**。

- **🐞 Bug / 问题反馈**：可复现的功能异常、崩溃、报错
- **💡 功能建议**：新功能或改进
- **❓ 使用咨询 / 反馈**：用法、配置与一般性反馈

> 维护者通常**每周集中处理一次** issue。反馈越具体（版本、系统、复现步骤、报错、最小样例），修复越快。

## 项目结构

```
navicat-rs/
├─ src/
│  ├─ main.rs      程序入口、窗口配置、--selftest 自检
│  ├─ app.rs       应用状态、标签页路由、动作处理
│  ├─ model.rs     数据模型（连接 / 列 / 对象）
│  ├─ store.rs     跨平台配置持久化（JSON）
│  ├─ db.rs        SQLite 引擎（查询 / CRUD / DDL / 备份 / 数据泵）
│  ├─ realdb.rs    PostgreSQL 真实网络协议驱动
│  ├─ theme.rs     多主题配色与样式
│  └─ ui/          各功能页面（table / query / designer / er / qb / tools / tree）
├─ assets/         内嵌中文字体与截图
├─ docs/           逆向分析、功能矩阵、Rust 复刻架构、验收与发布说明（见 docs/README.md）
├─ .github/        Issue 模板与 CI / build / Docker 工作流
├─ Dockerfile      容器镜像构建
├─ .editorconfig / .gitattributes / rustfmt.toml   代码风格与换行统一
├─ CONTRIBUTING.md / SECURITY.md / CODE_OF_CONDUCT.md   协作、安全与行为准则
└─ dist/           预编译产物（git 忽略）
```

## 本次更新摘要

- 页面品牌统一为 **navicat-rs**，去除「复刻」相关文案。
- 跨平台配置目录 + Windows/macOS/Linux CI 构建矩阵。
- 多主题（6 套）与全主题文字可读性修复。
- 发布 `v0.1.0`：Release 预编译产物 + GHCR 容器镜像。
- 详见根目录 `CHANGELOG.md` 与 `docs/06_十次迭代记录.md`。

## 文档

| 文件 | 内容 |
|---|---|
| `docs/用户操作手册.md` | 面向用户的完整操作手册（启动 / 连接 / 数据表 / 查询 / 设计 / 数据搬运 / 运维 / FAQ） |
| `docs/01_逆向分析总报告.md` | 安装包解包、技术栈与特性清单 |
| `docs/02_功能矩阵与业务逻辑.md` | 能力域 → 实现状态的完整矩阵 |
| `docs/03_收费与权限去除说明.md` | 全部功能开放的实现与代码级核验 |
| `docs/04_Rust复刻架构与映射.md` | 工程结构与模块映射 |
| `docs/05_验收报告.md` | 构建 / 自检 / 截图 / 覆盖度验收 |
| `docs/06_十次迭代记录.md` | 迭代过程与关键修复 |
| `docs/07_功能覆盖与差距清单.md` | 覆盖度核对与后续项 |
| `docs/FEEDBACK.md` | 使用反馈指南 |
| `docs/RELEASE_NOTES_v0.1.0.md` | v0.1.0 发布说明 |

## 说明

- 本项目为独立的界面与工作流实现，不包含、不调用任何第三方商业软件的二进制或许可证。
- 「全部功能免费开放」指本工程自身不设任何授权或权限限制。
- SQLite 为完全真实的本地后端；PostgreSQL 已接入真实网络协议；其余引擎当前为连接配置 + 离线演示数据，可按 `src/realdb.rs` 扩展。

## 许可

[MIT](LICENSE)
