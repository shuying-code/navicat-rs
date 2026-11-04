use crate::app::{short_id, Dialog, NavicatApp, TabKind}; // 引入应用层类型（Dialog/标签/应用对象）
use crate::model::DbKind; // 引入数据库引擎种类
use egui::{Color32, RichText, Stroke}; // 引入 egui 绘制所需类型

pub mod tree; // 对象树面板
pub mod table; // 数据表数据面板
pub mod query; // SQL 查询面板
pub mod designer; // 表结构设计器面板
pub mod er; // ER 图面板
pub mod qb; // 查询构建器面板
pub mod tools; // 各类工具面板集合

pub fn draw(app: &mut NavicatApp, ctx: &egui::Context) { // UI 总入口：绘制整个主界面
    top_menu(app, ctx); // 绘制顶部菜单
    toolbar(app, ctx); // 绘制工具栏
    status_bar(app, ctx); // 绘制状态栏
    if !app.focus_mode { // 非焦点模式时
        tree::draw(app, ctx); // 绘制左侧对象树
    } // if 结束

    egui::CentralPanel::default() // 中部主面板
        .frame(egui::Frame::none().fill(app.pal.bg)) // 设置面板背景填充色
        .show(ctx, |ui| { // 开始绘制中部内容
            tab_bar(app, ui); // 绘制标签页栏
            ui.add_space(2.0); // 留出少量间距
            let idx = app.active; // 当前激活标签索引
            if idx < app.tabs.len() { // 索引有效时
                content(app, ui, idx); // 绘制对应标签内容
            } else { // 否则
                welcome(app, ui); // 绘制欢迎页
            } // if 结束
        }); // CentralPanel 绘制结束

    dialogs(app, ctx); // 绘制弹窗对话框
} // draw 结束

fn top_menu(app: &mut NavicatApp, ctx: &egui::Context) { // 顶部菜单栏
    egui::TopBottomPanel::top("menubar") // 创建顶部面板
        .frame(egui::Frame::none().fill(app.pal.toolbar).inner_margin(egui::Margin::symmetric(8.0, 3.0))) // 设置工具栏底色与内边距
        .show(ctx, |ui| { // 开始绘制菜单
            egui::menu::bar(ui, |ui| { // 菜单条容器
                let sel_conn = app.selected_conn.clone().or_else(|| app.store.connections.first().map(|c| c.id.clone())); // 当前选中连接（缺省取第一个连接的 id）
                ui.menu_button(RichText::new("文件").color(app.pal.text), |ui| { // “文件”菜单
                    if ui.button("新建连接…").clicked() { // 点击“新建连接”
                        app.new_connection(DbKind::MySql); // 以 MySQL 打开新建连接
                        ui.close_menu(); // 关闭菜单
                    } // if 结束
                    if ui.button("新建 SQLite 本地库(示例)").clicked() { // 点击“新建 SQLite 本地库”
                        app.new_connection(DbKind::Sqlite); // 以 SQLite 打开新建连接
                        ui.close_menu(); // 关闭菜单
                    } // if 结束
                    ui.separator(); // 分隔线
                    if ui.button("新建查询").clicked() { // 点击“新建查询”
                        if let Some(id) = sel_conn.clone() { app.open_connection(id.clone()); app.new_query(id); } // 打开连接并新建查询标签
                        ui.close_menu(); // 关闭菜单
                    } // if 结束
                    if ui.button("退出").clicked() { ctx.send_viewport_cmd(egui::ViewportCommand::Close); } // 点击“退出”关闭窗口
                }); // “文件”菜单结束
                ui.menu_button(RichText::new("编辑").color(app.pal.text), |ui| { // “编辑”菜单
                    ui.label(RichText::new("（数据网格内可直接编辑单元格）").weak()); // 编辑提示文字
                }); // “编辑”菜单结束
                ui.menu_button(RichText::new("查看").color(app.pal.text), |ui| { // “查看”菜单
                    ui.menu_button("主题", |ui| { // “主题”子菜单
                        for t in crate::theme::THEMES { // 遍历所有主题
                            let selected = app.theme == t.key; // 当前主题是否被选中
                            if ui.selectable_label(selected, t.label).clicked() { // 点击某主题
                                app.set_theme(ctx, t.key); // 应用该主题
                                ui.close_menu(); // 关闭菜单
                            } // if 结束
                        } // for 结束
                    }); // 主题子菜单结束
                    if ui.button(if app.focus_mode { "退出焦点模式（显示侧栏）" } else { "焦点模式（隐藏侧栏）" }).clicked() { // 焦点模式切换按钮
                        app.focus_mode = !app.focus_mode; ui.close_menu(); // 切换焦点模式并关闭菜单
                    } // if 结束
                    if ui.button("消息日志").clicked() { app.show_log = !app.show_log; ui.close_menu(); } // 切换消息日志显示
                    if ui.button("设置").clicked() { app.open_settings(); ui.close_menu(); } // 打开设置
                }); // “查看”菜单结束
                ui.menu_button(RichText::new("工具").color(app.pal.text), |ui| { // “工具”菜单
                    let sc = sel_conn.clone(); // 缓存选中连接
                    if ui.button("SQL 查询编辑器").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.new_query(id);} ui.close_menu(); } // 打开 SQL 查询编辑器
                    if ui.button("查询构建器").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_query_builder(id);} ui.close_menu(); } // 打开查询构建器
                    if ui.button("ER 图 / 模型").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_er(id);} ui.close_menu(); } // 打开 ER 图 / 模型
                    ui.separator(); // 分隔线
                    if ui.button("数据传输").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_transfer(id);} ui.close_menu(); } // 打开数据传输
                    if ui.button("导入向导").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_import(id,String::new());} ui.close_menu(); } // 打开导入向导
                    if ui.button("导出向导").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_export(id,String::new());} ui.close_menu(); } // 打开导出向导
                    if ui.button("备份 / 还原").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_backup(id);} ui.close_menu(); } // 打开备份 / 还原
                    ui.separator(); // 分隔线
                    if ui.button("服务器监控").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_monitor(id);} ui.close_menu(); } // 打开服务器监控
                    if ui.button("数据剖析").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_profiling(id,String::new());} ui.close_menu(); } // 打开数据剖析
                    if ui.button("数据生成").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_datagen(id,String::new());} ui.close_menu(); } // 打开数据生成
                    if ui.button("数据脱敏").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_masking(id,String::new());} ui.close_menu(); } // 打开数据脱敏
                    if ui.button("数据同步 / 结构同步").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_sync(id);} ui.close_menu(); } // 打开数据同步 / 结构同步
                    if ui.button("对象管理（索引/触发器/视图）").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_objects(id,String::new());} ui.close_menu(); } // 打开对象管理
                    if ui.button("数据泵（SQL 导出/执行）").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_pump(id);} ui.close_menu(); } // 打开数据泵
                    if ui.button("模式分析").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_schema(id);} ui.close_menu(); } // 打开模式分析
                    if ui.button("SQL 大纲 / 重构").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_outline(id);} ui.close_menu(); } // 打开 SQL 大纲 / 重构
                    if ui.button("模型设计器").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_model(id);} ui.close_menu(); } // 打开模型设计器
                    if ui.button("查找于数据库").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_find(id);} ui.close_menu(); } // 打开查找于数据库
                    if ui.button("图表 / 仪表盘").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_chart(id);} ui.close_menu(); } // 打开图表 / 仪表盘
                    if ui.button("报表").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_report(id);} ui.close_menu(); } // 打开报表
                    if ui.button("自动化").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_automation(id);} ui.close_menu(); } // 打开自动化
                    ui.separator(); // 分隔线
                    if ui.button("AI 助手").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_ai(id);} ui.close_menu(); } // 打开 AI 助手
                    if ui.button("数据字典").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_dict(id,String::new());} ui.close_menu(); } // 打开数据字典
                    if ui.button("用户与权限").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_users(id);} ui.close_menu(); } // 打开用户与权限
                    if ui.button("代码片段").clicked() { app.open_snippets(); ui.close_menu(); } // 打开代码片段
                    if ui.button("查询历史").clicked() { app.open_history(); ui.close_menu(); } // 打开查询历史
                }); // “工具”菜单结束
                ui.menu_button(RichText::new("帮助").color(app.pal.text), |ui| { // “帮助”菜单
                    if ui.button("关于 Navicat-RS").clicked() { app.dialog = Some(Dialog::About); ui.close_menu(); } // 打开关于对话框
                }); // “帮助”菜单结束
            }); // 菜单条结束
        }); // 面板绘制结束
} // top_menu 结束

fn toolbar(app: &mut NavicatApp, ctx: &egui::Context) { // 工具栏
    egui::TopBottomPanel::top("toolbar") // 创建顶部工具栏面板
        .frame(egui::Frame::none().fill(app.pal.toolbar).stroke(Stroke::new(1.0, app.pal.border)).inner_margin(egui::Margin::symmetric(8.0, 5.0))) // 设置工具栏底色、描边与内边距
        .show(ctx, |ui| { // 开始绘制工具栏
            let sel = app.selected_conn.clone().or_else(|| app.store.connections.first().map(|c| c.id.clone())); // 当前选中连接
            ui.horizontal(|ui| { // 横向排列按钮
                if tb(ui, app.pal.text, "\u{2795}", "新建连接").clicked() { app.new_connection(DbKind::MySql); } // 新建连接按钮
                ui.separator(); // 分隔线
                if tb(ui, app.pal.text, "\u{1F4C4}", "查询").clicked() { if let Some(id)=sel.clone(){app.open_connection(id.clone());app.new_query(id);} } // 查询按钮
                if tb(ui, app.pal.text, "\u{1F5C2}", "表").clicked() { if let Some(id)=sel.clone(){app.open_connection(id.clone());app.open_designer(id,None);} } // 表按钮
                if tb(ui, app.pal.text, "\u{1F527}", "设计表").clicked() { if let Some(id)=sel.clone(){app.open_connection(id.clone());app.open_designer(id,None);} } // 设计表按钮
                if tb(ui, app.pal.text, "\u{1F5FA}", "ER 图").clicked() { if let Some(id)=sel.clone(){app.open_connection(id.clone());app.open_er(id);} } // ER 图按钮
                if tb(ui, app.pal.text, "\u{1F9F1}", "查询构建").clicked() { if let Some(id)=sel.clone(){app.open_connection(id.clone());app.open_query_builder(id);} } // 查询构建按钮
                ui.separator(); // 分隔线
                if tb(ui, app.pal.text, "\u{21C4}", "传输").clicked() { if let Some(id)=sel.clone(){app.open_connection(id.clone());app.open_transfer(id);} } // 数据传输按钮
                if tb(ui, app.pal.text, "\u{2B07}", "导入").clicked() { if let Some(id)=sel.clone(){app.open_connection(id.clone());app.open_import(id,String::new());} } // 导入按钮
                if tb(ui, app.pal.text, "\u{2B06}", "导出").clicked() { if let Some(id)=sel.clone(){app.open_connection(id.clone());app.open_export(id,String::new());} } // 导出按钮
                if tb(ui, app.pal.text, "\u{1F4BE}", "备份").clicked() { if let Some(id)=sel.clone(){app.open_connection(id.clone());app.open_backup(id);} } // 备份按钮
                ui.separator(); // 分隔线
                if tb(ui, app.pal.text, "\u{1F4C8}", "监控").clicked() { if let Some(id)=sel.clone(){app.open_connection(id.clone());app.open_monitor(id);} } // 监控按钮
                if tb(ui, app.pal.text, "\u{1F4CA}", "图表").clicked() { if let Some(id)=sel.clone(){app.open_connection(id.clone());app.open_chart(id);} } // 图表按钮
                if tb(ui, app.pal.text, "\u{1F50E}", "剖析").clicked() { if let Some(id)=sel.clone(){app.open_connection(id.clone());app.open_profiling(id,String::new());} } // 剖析按钮
                if tb(ui, app.pal.text, "\u{25D0}", "脱敏").clicked() { if let Some(id)=sel.clone(){app.open_connection(id.clone());app.open_masking(id,String::new());} } // 脱敏按钮
                if tb(ui, app.pal.text, "\u{21CB}", "同步").clicked() { if let Some(id)=sel.clone(){app.open_connection(id.clone());app.open_sync(id);} } // 同步按钮
                if tb(ui, app.pal.text, "\u{2728}", "AI").clicked() { if let Some(id)=sel.clone(){app.open_connection(id.clone());app.open_ai(id);} } // AI 按钮
            }); // 横向布局结束
        }); // 面板绘制结束
} // toolbar 结束

fn tb(ui: &mut egui::Ui, color: Color32, icon: &str, label: &str) -> egui::Response { // 工具栏按钮辅助函数
    let text = format!("{} {}", icon, label); // 拼接图标与文字
    ui.add(egui::Button::new(RichText::new(text).size(13.0).color(color)).frame(false)) // 绘制无边框按钮
        .on_hover_text(label) // 悬停提示文字
} // 返回响应

fn status_bar(app: &mut NavicatApp, ctx: &egui::Context) { // 状态栏
    egui::TopBottomPanel::bottom("statusbar") // 创建底部状态栏面板
        .frame(egui::Frame::none().fill(app.pal.toolbar).stroke(Stroke::new(1.0, app.pal.border)).inner_margin(egui::Margin::symmetric(10.0, 3.0))) // 设置底色、描边与内边距
        .show(ctx, |ui| { // 开始绘制
            ui.horizontal(|ui| { // 横向布局
                ui.label(RichText::new("\u{2714} 全部功能已解锁 · 免费开放 · 无任何权限限制").color(app.pal.success).size(12.0)); // “全部功能已解锁”成功提示
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| { // 右对齐布局
                    ui.label(RichText::new(&app.status).size(12.0).color(app.pal.text_dim)); // 当前状态文字
                    let open = app.engines.len(); // 打开的连接数
                    ui.label(RichText::new(format!("连接: {} 活动 | 标签: {}", open, app.tabs.len())).size(12.0).color(app.pal.text_dim)); // 连接 / 标签统计
                }); // 布局结束
            }); // 横向结束
        }); // 面板结束
} // status_bar 结束

fn tab_bar(app: &mut NavicatApp, ui: &mut egui::Ui) { // 标签页栏
    let mut close_idx: Option<usize> = None; // 待关闭的标签索引
    egui::ScrollArea::horizontal().id_source("tabscroll").max_height(34.0).show(ui, |ui| { // 横向滚动区域
        ui.horizontal(|ui| { // 横向布局
            for i in 0..app.tabs.len() { // 遍历所有标签
                let active = i == app.active; // 是否为激活标签
                let title = app.tabs[i].title(); // 取标签标题
                let icon = app.tabs[i].icon(); // 取标签图标
                let bg = if active { app.pal.tab_active } else { app.pal.tab_inactive }; // 根据激活状态取背景色
                let fg = if active { app.pal.accent } else { app.pal.text_dim }; // 根据激活状态取前景色
                egui::Frame::none() // 标签框架
                    .fill(bg) // 填充背景
                    .stroke(Stroke::new(1.0, app.pal.border)) // 描边
                    .inner_margin(egui::Margin::symmetric(9.0, 5.0)) // 内边距
                    .show(ui, |ui| { // 绘制标签内容
                        ui.horizontal(|ui| { // 横向布局
                            let resp = ui.add( // 可点击的标签文字
                                egui::Label::new(RichText::new(format!("{} {}", icon, title)).size(12.5).color(fg)) // 标签文字内容
                                    .sense(egui::Sense::click()), // 设置点击感应
                            ); // add 结束
                            if resp.clicked() { // 点击标签时
                                app.active = i; // 切换激活标签
                            } // if 结束
                            if app.tabs.len() > 1 && ui.small_button("\u{00D7}").clicked() { // 关闭按钮（多于一个标签时显示）
                                close_idx = Some(i); // 记录待关闭索引
                            } // if 结束
                        }); // 横向结束
                    }); // 框架结束
            } // for 结束
        }); // 横向结束
    }); // 滚动区结束
    if let Some(i) = close_idx { // 若有待关闭标签
        app.tabs.remove(i); // 移除该标签
        if app.active >= app.tabs.len() && !app.tabs.is_empty() { // 修正激活索引越界
            app.active = app.tabs.len() - 1; // 激活最后一个标签
        } // if 结束
    } // if 结束
} // tab_bar 结束

fn content(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) { // 按标签类型分发内容
    // Disambiguate borrow of app.tabs[idx].kind versus &mut app
    let is = match &app.tabs[idx].kind { // 取标签类型编号（避免借用冲突）
        TabKind::Welcome => 0, // Welcome → 0
        TabKind::Table(_) => 1, // Table → 1
        TabKind::Query(_) => 2, // Query → 2
        TabKind::Designer(_) => 3, // Designer → 3
        TabKind::Er(_) => 4, // Er → 4
        TabKind::QueryBuilder(_) => 5, // QueryBuilder → 5
        _ => 6, // 其它 → 6
    }; // match 结束
    match is { // 分发绘制
        0 => welcome(app, ui), // 欢迎页
        1 => table::draw(app, ui, idx), // 数据表
        2 => query::draw(app, ui, idx), // 查询
        3 => designer::draw(app, ui, idx), // 设计器
        4 => er::draw(app, ui, idx), // ER 图
        5 => qb::draw(app, ui, idx), // 查询构建器
        _ => tools::draw(app, ui, idx), // 工具面板
    } // match 结束
} // content 结束

pub fn card<'a>(app: &NavicatApp) -> egui::Frame { // 卡片样式（统一面板外观）
    egui::Frame::none() // 创建框架
        .fill(app.pal.panel) // 填充面板色
        .stroke(Stroke::new(1.0, app.pal.border)) // 描边
        .rounding(egui::Rounding::same(8.0)) // 圆角
        .inner_margin(egui::Margin::same(14.0)) // 内边距
} // 返回框架

fn welcome(app: &mut NavicatApp, ui: &mut egui::Ui) { // 欢迎页
    egui::ScrollArea::vertical().show(ui, |ui| { // 纵向滚动
        ui.add_space(12.0); // 间距
        ui.horizontal(|ui| { // 横向布局
            ui.add_space(12.0); // 左侧间距
            ui.vertical(|ui| { // 纵向布局
                ui.label(RichText::new("navicat-rs").size(26.0).strong().color(app.pal.accent)); // 标题文字
                ui.label(RichText::new("多引擎数据库管理 IDE · 全部付费功能免费开放 · 无登录 · 无试用 · 无权限限制").size(14.0).color(app.pal.text_dim)); // 副标题说明
                ui.add_space(10.0); // 间距
                card(app).show(ui, |ui| { // “已解锁功能”卡片
                    ui.horizontal(|ui| { // 横向
                        ui.label(RichText::new("\u{1F513} 已解锁").color(app.pal.success).strong()); // 已解锁标签
                        ui.label("数据剖析 · 数据生成 · 数据脱敏 · 查询构建器 · ER 模型 · 图表/仪表盘 · AI 助手 · 自动化 · 报表 · 服务器监控 · 备份/还原 · 数据传输 · 数据同步"); // 功能清单文字
                    }); // 横向结束
                }); // 卡片结束
                ui.add_space(14.0); // 间距
                ui.label(RichText::new("快速开始").size(16.0).strong()); // “快速开始”标题
                ui.add_space(6.0); // 间距
                let conn = app.store.connections.first().map(|c| c.id.clone()); // 取第一个连接 id
                ui.horizontal_wrapped(|ui| { // 自动换行布局
                    if big_btn(ui, "\u{2795}", "新建连接").clicked() { app.new_connection(DbKind::MySql); } // 新建连接大按钮
                    if big_btn(ui, "\u{1F4C4}", "新建查询").clicked() { // 新建查询大按钮
                        if let Some(id) = conn.clone() { app.open_connection(id.clone()); app.new_query(id); } // 打开连接并新建查询
                    } // if 结束
                    if big_btn(ui, "\u{1F5FA}", "ER 图").clicked() { // ER 图大按钮
                        if let Some(id) = conn.clone() { app.open_connection(id.clone()); app.open_er(id); } // 打开 ER 图
                    } // if 结束
                    if big_btn(ui, "\u{1F4CA}", "图表").clicked() { // 图表大按钮
                        if let Some(id) = conn.clone() { app.open_connection(id.clone()); app.open_chart(id); } // 打开图表
                    } // if 结束
                    if big_btn(ui, "\u{2728}", "AI 助手").clicked() { // AI 助手大按钮
                        if let Some(id) = conn.clone() { app.open_connection(id.clone()); app.open_ai(id); } // 打开 AI 助手
                    } // if 结束
                }); // 布局结束
                ui.add_space(14.0); // 间距
                ui.label(RichText::new("已保存的连接").size(16.0).strong()); // “已保存连接”标题
                ui.add_space(6.0); // 间距
                if app.store.connections.is_empty() { // 若无连接
                    ui.label(RichText::new("暂无连接。点击“新建连接”添加，或选择 SQLite 使用内置示例库。").color(app.pal.text_dim)); // 空态提示
                } else { // 否则
                    let conns: Vec<(String, String, String)> = app.store.connections.iter().map(|c| (c.id.clone(), c.name.clone(), format!("{} · {}", c.kind.label(), c.target_desc()))).collect(); // 收集连接信息为元组列表
                    for (id, name, desc) in conns { // 遍历连接
                        card(app).show(ui, |ui| { // 连接卡片
                            ui.horizontal(|ui| { // 横向
                                ui.label(RichText::new("\u{1F5C4}").size(18.0)); // 数据库图标
                                ui.vertical(|ui| { // 纵向
                                    ui.label(RichText::new(name).strong()); // 连接名（加粗）
                                    ui.label(RichText::new(desc).size(12.0).color(app.pal.text_dim)); // 连接描述（灰色小字）
                                }); // 纵向结束
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| { // 右对齐布局
                                    if ui.button("打开").clicked() { app.open_connection(id.clone()); } // “打开”按钮
                                    if ui.button("编辑").clicked() { app.edit_connection(id.clone()); } // “编辑”按钮
                                }); // 布局结束
                            }); // 横向结束
                        }); // 卡片结束
                        ui.add_space(6.0); // 间距
                    } // for 结束
                } // if 结束
            }); // 纵向结束
        }); // 横向结束
    }); // 滚动区结束
} // welcome 结束

fn big_btn(ui: &mut egui::Ui, icon: &str, label: &str) -> egui::Response { // 大按钮辅助函数
    ui.add_sized([150.0, 64.0], egui::Button::new(RichText::new(format!("{}\n{}", icon, label)).size(14.0))) // 绘制固定尺寸大按钮
} // 返回响应

fn dialogs(app: &mut NavicatApp, ctx: &egui::Context) { // 弹窗分发
    let dlg = match app.dialog.take() { // 取出待显示弹窗
        Some(d) => d, // 有弹窗
        None => return, // 无弹窗则直接返回
    }; // match 结束
    match dlg { // 按弹窗类型分发
        Dialog::Conn(mut form) => { // 连接表单弹窗
            let mut open = true; // 是否打开
            let mut act = FormAction::None; // 表单动作初值
            egui::Window::new(if form.is_new { "新建连接" } else { "编辑连接" }) // 窗口标题随新建/编辑变化
                .collapsible(false).resizable(false).open(&mut open) // 窗口属性
                .show(ctx, |ui| { // 绘制
                    act = connection_form_ui(app, &mut form, ui); // 渲染连接表单
                }); // 绘制结束
            match act { // 处理返回动作
                FormAction::Save => { // 保存
                    if form.conn.name.trim().is_empty() { // 名称非空校验
                        form.error = "连接名不能为空".into(); // 设置错误提示
                        app.dialog = Some(Dialog::Conn(form)); // 保持弹窗打开
                    } else { // 否则
                        app.commit_connection(form.conn, form.is_new); // 提交连接（新增或更新）
                    } // if 结束
                } // 取消：不处理
                FormAction::Cancel => {} // 无动作
                FormAction::None => { // 保持弹窗
                    if open { app.dialog = Some(Dialog::Conn(form)); } // if 结束
                } // match 结束
            } // 连接表单分支结束
        } // 块结束
        Dialog::Confirm(data) => { // 确认弹窗
            let mut open = true; // 是否打开
            let mut do_it = false; // 是否确认
            let mut cancel = false; // 是否取消
            egui::Window::new(data.title.clone()).collapsible(false).resizable(false).open(&mut open).show(ctx, |ui| { // 确认窗口
                ui.label(&data.msg); // 显示消息
                ui.add_space(10.0); // 间距
                ui.horizontal(|ui| { // 横向
                    if ui.button(RichText::new("确定").color(app.pal.danger)).clicked() { do_it = true; } // “确认”按钮
                    if ui.button("取消").clicked() { cancel = true; } // “取消”按钮
                }); // 横向结束
            }); // 绘制结束
            if do_it { // 若确认
                app.process_act(data.action.clone()); // 执行动作
            } else if open && !cancel { // 否则若仍打开且未取消
                app.dialog = Some(Dialog::Confirm(data)); // 保持弹窗
            } // if 结束
        } // 确认弹窗结束
        Dialog::Input(mut data) => { // 输入弹窗
            let mut open = true; // 是否打开
            let mut ok = false; // 是否确定
            let mut cancel = false; // 是否取消
            egui::Window::new(data.title.clone()).collapsible(false).resizable(false).open(&mut open).show(ctx, |ui| { // 输入窗口
                ui.label(&data.label); // 显示提示
                ui.add(egui::TextEdit::singleline(&mut data.value).desired_width(380.0)); // 单行输入框
                ui.add_space(8.0); // 间距
                ui.horizontal(|ui| { // 横向
                    if ui.button("确定").clicked() { ok = true; } // “确定”按钮
                    if ui.button("取消").clicked() { cancel = true; } // “取消”按钮
                }); // 横向结束
            }); // 绘制结束
            if ok { // 若确定
                app.process_act(data.action.clone()); // 执行动作
            } else if open && !cancel { // 否则若仍打开且未取消
                app.dialog = Some(Dialog::Input(data)); // 保持弹窗
            } // if 结束
        } // 输入弹窗结束
        Dialog::About => { // 关于弹窗
            let mut open = true; // 是否打开
            egui::Window::new("关于 Navicat-RS").collapsible(false).resizable(false).open(&mut open).show(ctx, |ui| { // 关于窗口
                ui.label(RichText::new("navicat-rs").size(18.0).strong().color(app.pal.accent)); // 标题文字
                ui.label("基于 eframe/egui 与 rusqlite 构建的本地数据库管理 IDE。"); // 项目说明
                ui.label("技术栈: Rust · eframe/egui · rusqlite · 纯 Rust PostgreSQL 驱动。"); // 技术栈说明
                ui.separator(); // 分隔线
                ui.label("所有付费/订阅功能均已免费开放，无登录、无试用期、无功能限制。"); // 版本/授权说明
            }); // 绘制结束
            if open { // 若仍打开
                app.dialog = Some(Dialog::About); // 保持弹窗
            } // if 结束
        } // 结束
    } // about 分支结束
} // dialogs 结束

enum FormAction { None, Save, Cancel } // 表单动作枚举

fn connection_form_ui(app: &mut NavicatApp, form: &mut crate::app::ConnectionForm, ui: &mut egui::Ui) -> FormAction { // 连接表单渲染函数
    ui.set_min_width(430.0); // 设置最小宽度
    egui::Grid::new("connform").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| { // 两列网格布局
        ui.label("连接名"); // “连接名”标签
        ui.add(egui::TextEdit::singleline(&mut form.conn.name).desired_width(300.0)); // 连接名输入框
        ui.end_row(); // 换行
        ui.label("类型"); // “引擎类型”标签
        egui::ComboBox::from_id_source("kindsel") // 类型下拉框
            .selected_text(form.conn.kind.label()) // 下拉显示的选中文本
            .show_ui(ui, |ui| { // 下拉内容
                let cur = form.conn.kind; // 当前类型
                for k in DbKind::all() { // 遍历全部引擎类型
                    if ui.selectable_label(cur == k, k.label()).clicked() { // 点击选择该类型
                        let was_sqlite = form.conn.kind.is_sqlite(); // 记录原类型是否为 SQLite
                        form.conn.kind = k; // 更新引擎类型
                        if k.default_port() != 0 { form.conn.port = k.default_port(); } // 若该类型有默认端口则更新端口
                        if k.is_sqlite() && form.conn.sqlite_path.is_empty() { // 若切到 SQLite 且路径为空
                            form.conn.sqlite_path = crate::app::default_export_dir().join("sampledb.sqlite").to_string_lossy().to_string(); // 填入默认示例库路径
                        } // if 结束
                        if !was_sqlite && k.is_sqlite() { form.conn.sqlite_path = crate::app::default_export_dir().join("sampledb.sqlite").to_string_lossy().to_string(); } // 从非 SQLite 切到 SQLite 时补默认路径
                    } // if 结束
                } // for 结束
            }); // 下拉结束
        ui.end_row(); // 换行
        if form.conn.kind.is_sqlite() { // 若为 SQLite 引擎
            ui.label("数据库文件"); // “数据库文件”标签
            ui.add(egui::TextEdit::singleline(&mut form.conn.sqlite_path).desired_width(300.0).hint_text("留空则使用内存示例库")); // 路径输入框（提示可留空用内存示例）
            ui.end_row(); // 换行
            ui.label(""); // 空白标签占位
            if ui.button("使用内置示例库 (内存)").clicked() { form.conn.sqlite_path.clear(); } // “使用内存示例库”按钮
            ui.end_row(); // 换行
        } else { // 否则（网络数据库）
            ui.label("主机"); // “主机”标签
            ui.add(egui::TextEdit::singleline(&mut form.conn.host).desired_width(300.0)); // 主机输入框
            ui.end_row(); // 换行
            ui.label("端口"); // “端口”标签
            let mut port = form.conn.port as i64; // 端口临时变量
            if ui.add(egui::DragValue::new(&mut port).range(0..=65535)).changed() { form.conn.port = port as u16; } // 端口拖拽输入框（0..65535）
            ui.end_row(); // 换行
            ui.label("用户名"); // “用户名”标签
            ui.add(egui::TextEdit::singleline(&mut form.conn.username).desired_width(300.0)); // 用户名输入框
            ui.end_row(); // 换行
            ui.label("密码"); // “密码”标签
            ui.add(egui::TextEdit::singleline(&mut form.conn.password).password(true).desired_width(300.0)); // 密码输入框（遮蔽显示）
            ui.end_row(); // 换行
            ui.label("数据库"); // “数据库”标签
            ui.add(egui::TextEdit::singleline(&mut form.conn.database).desired_width(300.0)); // 数据库名输入框
            ui.end_row(); // 换行
            ui.label("选项"); // “选项”标签
            ui.horizontal(|ui| { // 横向布局
                ui.checkbox(&mut form.conn.ssl, "SSL/TLS"); // SSL/TLS 勾选框
                ui.checkbox(&mut form.conn.ssh_tunnel, "SSH 隧道"); // SSH 隧道勾选框
            }); // 横向结束
            ui.end_row(); // 换行
            if form.conn.ssh_tunnel { // 若启用 SSH 隧道
                ui.label("SSH 主机"); // “SSH 主机”标签
                ui.horizontal(|ui| { // 横向布局
                    ui.add(egui::TextEdit::singleline(&mut form.conn.ssh_host).desired_width(180.0)); // SSH 主机输入框
                    ui.label("端口"); // “端口”标签
                    let mut p = form.conn.ssh_port as i64; // SSH 端口临时变量
                    if ui.add(egui::DragValue::new(&mut p).range(0..=65535)).changed() { form.conn.ssh_port = p as u16; } // SSH 端口拖拽输入框
                }); // 横向结束
                ui.end_row(); // 换行
                ui.label("SSH 用户"); // “SSH 用户”标签
                ui.add(egui::TextEdit::singleline(&mut form.conn.ssh_user).desired_width(300.0)); // SSH 用户输入框
                ui.end_row(); // 换行
            } // if 结束
            ui.label(""); // 空白标签
            ui.label(RichText::new("提示: 非 SQLite 引擎以离线演示库呈现（界面/工作流完整，无需真实服务器）。").size(11.0).color(app.pal.text_dim)); // SQLite 提示说明文字
            ui.end_row(); // 换行
        } // if 结束
    }); // 网格结束
    ui.add_space(4.0); // 间距
    ui.horizontal(|ui| { // 横向
        if ui.button("测试连接").clicked() { // “测试连接”按钮
            form.error = match form.conn.kind { // 执行测试并记录结果
                DbKind::PostgreSql => match crate::realdb::pg_test(&form.conn) { // PostgreSQL 走真实驱动测试
                    Ok(v) => format!("\u{2714} 连接成功: {}", v), // 成功提示
                    Err(e) => format!("\u{2716} 连接失败: {}", e), // 失败提示
                }, // match 结束
                DbKind::Sqlite => "\u{2714} SQLite 为嵌入式引擎，保存后直接可用。".to_string(), // SQLite 提示
                _ => "该引擎以离线演示库运行（界面与工作流完整）；当前已内置真实驱动的引擎为 PostgreSQL。".to_string(), // 其它引擎提示
            }; // 赋值结束
        } // if 结束
    }); // 横向结束
    if !form.error.is_empty() { // 若有错误信息
        ui.colored_label(app.pal.danger, &form.error); // 红色错误文字
    } // if 结束
    ui.add_space(8.0); // 间距
    let mut act = FormAction::None; // 返回动作初值
    ui.horizontal(|ui| { // 横向
        if ui.button(RichText::new("保存并连接").color(app.pal.contrast_text(app.pal.accent)).background_color(app.pal.accent)).clicked() { // “保存并连接”强调按钮
            act = FormAction::Save; // 设置动作为保存
        } // if 结束
        if ui.button("取消").clicked() { // “取消”按钮
            act = FormAction::Cancel; // 设置动作为取消
        } // if 结束
    }); // 横向结束
    act // 返回动作
} // 函数结束

pub fn mini_badge(ui: &mut egui::Ui, text: &str, color: Color32) { // 小徽标（带底色的小标签）
    egui::Frame::none().fill(color.gamma_multiply(0.15)).rounding(egui::Rounding::same(6.0)).inner_margin(egui::Margin::symmetric(6.0, 2.0)).show(ui, |ui| { // 创建底色半透明框架
        ui.label(RichText::new(text).size(11.0).color(color)); // 徽标文字
    }); // 框架绘制结束
} // 函数结束

pub fn conn_badge(ui: &mut egui::Ui, app: &NavicatApp, conn_id: &str) { // 连接徽标（引擎简写 + 短 id）
    if let Some(c) = app.conn(conn_id) { // 若连接存在
        egui::Frame::none().fill(c.color32().gamma_multiply(0.15)).rounding(egui::Rounding::same(5.0)).inner_margin(egui::Margin::symmetric(6.0, 2.0)).show(ui, |ui| { // 以引擎色半透明为底的框架
            ui.label(RichText::new(format!("{} {}", c.kind.short(), short_id(&c.id))).size(11.0).color(c.color32())); // 徽标文字（引擎简写 + 短 id）
        }); // 框架绘制结束
    } // if 结束
} // 函数结束
