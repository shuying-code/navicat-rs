# Rust 复刻架构与原程序映射

> 工程：`F:\codex工作区\navicat-rs`
> 技术栈：Rust 2021 + eframe/egui 0.29（glow 后端）+ rusqlite(bundled)
> 目标：同功能、同页面观感的桌面数据库客户端，全部功能免费开放。

---

## 1. 技术选型对比（为何不是"逐字节移植"）

原程序是 **Delphi/C++Builder FMX**，无可靠源码。逐字节移植不可行，也不必要——用户要的是**同功能、同页面观感**的可用产品。故采用 **Rust + egui 立即模式 GUI** 重建：跨平台、单文件可执行、依赖少、可完全离线。

| 维度 | 原版 | 复刻 |
|---|---|---|
| 语言/框架 | Delphi/C++Builder FMX | Rust + egui/eframe |
| 渲染 | 原生 FMX 控件 | GPU(glow) 立即模式绘制 |
| 数据库访问 | 各引擎原生客户端 DLL | rusqlite(bundled) 真实 SQLite；其余引擎配置化 |
| 配置存储 | 注册表/自有格式 | `%APPDATA%\navicat-rs\navicat-rs.json` |
| 授权 | 许可证/试用/特性开关 | 无（全开放） |

---

## 2. 工程结构

```
navicat-rs/
├─ Cargo.toml            # 依赖声明
├─ .cargo/config.toml    # target-dir 指向 C:\cargo-target\navicat（C 盘空间充足）
├─ assets/NotoSansSC-Regular.ttf   # 内嵌中文字体（include_bytes!）
├─ src/
│  ├─ main.rs            # 入口；--selftest 无头自检；eframe 启动
│  ├─ app.rs             # NavicatApp：状态、TabKind、动作、连接、各 open_* 页面构造
│  ├─ model.rs           # DbKind/Connection/Column/DbObject/ObjKind 数据模型
│  ├─ store.rs           # JSON 持久化（连接、片段、设置）
│  ├─ db.rs              # SqliteEngine：连接/对象/列/查询/执行/脚本/外键/CRUD/PRAGMA/建表/分号切分
│  ├─ realdb.rs          # 真实网络协议驱动层：PostgreSQL（postgres 纯 Rust）；has_real_driver 判定
│  ├─ theme.rs           # Palette（浅色/深色 + Navicat 蓝强调色）、字体安装
│  └─ ui/
│     ├─ mod.rs          # 菜单栏、工具栏、标签栏、状态栏、欢迎页、对话框、卡片/徽章
│     ├─ tree.rs         # 左侧连接与对象树、搜索过滤
│     ├─ table.rs        # 数据网格（分页/过滤/内联编辑/增删行/导出）
│     ├─ query.rs        # SQL 编辑器 + 结果网格（多语句）
│     ├─ designer.rs     # 表设计器（列编辑 → DDL）
│     ├─ er.rs           # ER 图（实体卡 + 外键连线 + 缩放）
│     ├─ qb.rs           # 可视化查询构建器（选表/列/条件 → SQL）
│     └─ tools.rs        # 传输/导入/导出/备份/监控/剖析/数据生成/图表/字典/报表/自动化/用户/AI/片段/设置
├─ work/                 # 截图脚本与验收截图
├─ dist/                 # Release 可执行产物
└─ docs/                 # 分析/矩阵/架构/验收/迭代/差距文档
```

---

## 3. 依赖（Cargo.toml 摘要）

| crate | 版本 | 用途 |
|---|---|---|
| eframe | 0.29 | 窗口/事件循环（glow + default_fonts） |
| egui | 0.29 | 立即模式 UI（**必须作为直接依赖**，与 eframe 版本严格一致） |
| egui_extras | 0.29 | 网格/图片等扩展 |
| rusqlite | 0.31 (`bundled`) | 内置编译 SQLite，无需系统库 |
| serde / serde_json | 1 | 配置持久化 |
| chrono | 0.4 | 时间戳/日期 |
| uuid | 1 (v4) | 连接 ID |
| rand | 0.8 | 数据生成随机值 |
| csv | 1 | CSV 导入导出 |
| anyhow | 1 | 错误传播 |

> 关键工程细节：`rusqlite` 的 `bundled` 特性需 C 编译器；本机用 MSVC Build Tools 的 `vcvars64.bat` 初始化环境后 `cargo build`。`egui` 必须与 `eframe` 同为 0.29，否则类型不兼容。

---

## 4. 核心模块与原程序映射

| 原程序概念 | 复刻实现 |
|---|---|
| `TNavicatMainForm` 主窗体 | `NavicatApp` + `ui::mod`（菜单/工具栏/标签/状态栏三区布局） |
| 连接 profile（`cc*`） | `model::Connection` + `DbKind` 枚举（11 引擎 + ODBC） |
| 对象树 | `ui::tree`（按 表/视图/备份/模型/用户 分组；`db.objects()` 驱动） |
| 表数据窗体 | `ui::table`（分页/过滤/编辑/增删/导出） |
| SQL 编辑器/结果 | `ui::query`（多语句 `run_script`） |
| 表设计器（`DesignForm`） | `ui::designer` + `db::build_create_table` |
| ER 图（`ModelERDiagram`） | `ui::er` + `db::foreign_keys()` 绘制连线 |
| 查询构建器（`libqb`） | `ui::qb` |
| 传输/导入/导出/备份/监控/剖析/数据生成/图表/字典/报表/自动化/用户/AI/片段/设置 | `ui::tools`（每页一个渲染函数） |
| `ac*` 动作 | `app.rs` 的 `act_*`/`open_*` 方法 + 菜单/工具栏绑定 |
| 属性/设置存储 | `store::Store` → `%APPDATA%\navicat-rs\navicat-rs.json` |
| 内嵌字体 | `assets/NotoSansSC-Regular.ttf`，`theme::install_fonts` 注入 egui |

---

## 5. 数据层设计（`db.rs`）

- `SqliteEngine` 封装 `rusqlite::Connection` + 路径。`open(path)` 打开真实库；`open_memory_seeded()` 建内存库并 `seed_demo()`。
- `seed_demo()` 建 4 表（customers/products/orders/employees）+ 1 视图（v_order_summary）+ 演示数据 + 外键，作为**所有引擎离线演示的标准 schema**。
- 查询：`query()`/`execute()` 统一返回 `QueryResult{columns, rows, affected, elapsed_ms, error, is_select, ...}`，值以 `Option<String>` 承载（NULL 语义清晰）。
- 脚本：`run_script()` 依 `split_sql()` 切分多语句，逐条执行返回多个结果。
- CRUD：`update_cell(table,rowid,col,val)` / `delete_row` / `insert_row`，按 `rowid` 精确定位。
- 结构：`objects()`/`columns()`/`foreign_keys()`/`build_create_table()`。
- 运维：`pragma_i64/str`、`integrity_check`、`sqlite_version`、`db_file_size`。

---

## 6. UI 状态与交互设计

- **TabKind**：`Welcome/Table/Query/Designer/Er/QueryBuilder/Transfer/Export/Import/Backup/Monitor/Profiling/DataGen/Chart/Ai/Dict/Snippet/Report/Automation/Users/Settings/About`——对应原程序主要功能页。
- **Dialog**：`Conn(ConnectionForm)/Confirm/Input/About`，采用"take-占用再放回"的模式避免借用冲突（`self.dialog.take()`）。
- **工具栏/菜单**：与顶部工具条一一绑定到 `open_*`。所有入口默认可见（无灰显/无锁）。
- **主题**：`Palette` 浅/深两套，Navicat 蓝作为强调色；`theme::apply_style` 设置控件圆角/间距/描边。
- **中文字体**：内嵌 Noto Sans SC，保证中文 UI 与树/表格中文正常渲染（不依赖系统字体）。

---

## 7. 构建与运行

```powershell
# Debug 运行
cmd /c 'call "D:\app\Microsoft Visual Studio\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul 2>&1 && cargo build'
C:\cargo-target\navicat\debug\navicat-rs.exe

# Release 打包
cmd /c 'call "D:\app\Microsoft Visual Studio\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul 2>&1 && cargo build --release'
# 产物：C:\cargo-target\navicat\release\navicat-rs.exe → 复制到 dist\

# 无头功能自检（不进 GUI）
C:\cargo-target\navicat\debug\navicat-rs.exe --selftest
```

环境变量（用于验收截图）：`NAVICAT_RS_DEMO=1|all`（自动打开示例库/全部页面）、`NAVICAT_RS_TAB=<子串>`（激活某页）。

---

## 8. 与"同功能同页面"的达成度

- **页面观感**：主窗体采用"菜单+工具条+标签页+左树+主区+状态栏"与 Navicat 一致的经典布局；浅/深主题、蓝色强调、中文界面。
- **功能**：SQLite 全链路真实可用；PostgreSQL 已接入真实网络协议（`src/realdb.rs`，测试连接 + 真实查询）；其余引擎以连接配置+离线演示 schema 提供一致交互页面。详见 `05`（自检 22/22）与 `07`。
- **"全开放"**：无任何登录/许可/试用/权限/配额判断（见 `03`）。
