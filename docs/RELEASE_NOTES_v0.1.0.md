# navicat-rs v0.1.0

首个正式版本：用 **Rust + egui/eframe** 编写的跨平台数据库管理桌面客户端。
所有功能全部免费开放，无登录、无试用、无授权/权限限制。

## 亮点

- **跨平台**：Windows / macOS / Linux 三平台同源代码（CI 三平台构建通过）。
- **多主题**：浅色 / 深色 / 午夜蓝 / 森林 / 暖沙 / 石墨，任意主题下文字对比清晰可读。
- **完整功能页**：数据表、SQL 查询（多语句 / Explain / 美化 / 大纲）、表设计器、ER 图、
  查询构建器、模型设计器、数据传输/同步、导入/导出、数据泵、备份/还原、服务器监控、
  数据剖析、模式分析、数据生成、数据脱敏、图表、数据字典、报表、自动化、AI 助手、
  代码片段、对象管理、查找于数据库、用户与权限、设置。
- **真实后端**：SQLite（bundled，全平台）；PostgreSQL 真实网络协议驱动。
- **树形缩放与分组折叠**、`--selftest` 无头自检 22 项。

## 下载

| 平台 | 文件 |
|---|---|
| Windows x64 | `navicat-rs-windows-x64.exe` / `navicat-rs-windows-x64.zip` |
| macOS（Universal, Intel + Apple Silicon） | `navicat-rs-macos-universal.tar.gz` |
| Linux x64 | `navicat-rs-linux-x64.tar.gz` |

> Windows 直接双击 exe 运行；macOS/Linux 解压后 `chmod +x` 运行。也可使用 `ghcr.io/shuying-code/navicat-rs` 容器镜像。

## 使用

```bash
navicat-rs            # 启动 GUI
navicat-rs --selftest # 无头自检
```

首次启动会自动创建“示例数据库 (SQLite)”。

## 容器镜像

```bash
docker pull ghcr.io/shuying-code/navicat-rs:latest
docker run --rm ghcr.io/shuying-code/navicat-rs:latest --selftest
```

完整变更见 [CHANGELOG](../CHANGELOG.md)。
