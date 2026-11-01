use crate::app::{short_id, Dialog, NavicatApp, TabKind};
use crate::model::DbKind;
use egui::{Color32, RichText, Stroke};

pub mod tree;
pub mod table;
pub mod query;
pub mod designer;
pub mod er;
pub mod qb;
pub mod tools;

pub fn draw(app: &mut NavicatApp, ctx: &egui::Context) {
    top_menu(app, ctx);
    toolbar(app, ctx);
    status_bar(app, ctx);
    if !app.focus_mode {
        tree::draw(app, ctx);
    }

    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(app.pal.bg))
        .show(ctx, |ui| {
            tab_bar(app, ui);
            ui.add_space(2.0);
            let idx = app.active;
            if idx < app.tabs.len() {
                content(app, ui, idx);
            } else {
                welcome(app, ui);
            }
        });

    dialogs(app, ctx);
}

fn top_menu(app: &mut NavicatApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("menubar")
        .frame(egui::Frame::none().fill(app.pal.toolbar).inner_margin(egui::Margin::symmetric(8.0, 3.0)))
        .show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let sel_conn = app.selected_conn.clone().or_else(|| app.store.connections.first().map(|c| c.id.clone()));
                ui.menu_button(RichText::new("文件").color(app.pal.text), |ui| {
                    if ui.button("新建连接…").clicked() {
                        app.new_connection(DbKind::MySql);
                        ui.close_menu();
                    }
                    if ui.button("新建 SQLite 本地库(示例)").clicked() {
                        app.new_connection(DbKind::Sqlite);
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("新建查询").clicked() {
                        if let Some(id) = sel_conn.clone() { app.open_connection(id.clone()); app.new_query(id); }
                        ui.close_menu();
                    }
                    if ui.button("退出").clicked() { ctx.send_viewport_cmd(egui::ViewportCommand::Close); }
                });
                ui.menu_button(RichText::new("编辑").color(app.pal.text), |ui| {
                    ui.label(RichText::new("（数据网格内可直接编辑单元格）").weak());
                });
                ui.menu_button(RichText::new("查看").color(app.pal.text), |ui| {
                    ui.menu_button("主题", |ui| {
                        for t in crate::theme::THEMES {
                            let selected = app.theme == t.key;
                            if ui.selectable_label(selected, t.label).clicked() {
                                app.set_theme(ctx, t.key);
                                ui.close_menu();
                            }
                        }
                    });
                    if ui.button(if app.focus_mode { "退出焦点模式（显示侧栏）" } else { "焦点模式（隐藏侧栏）" }).clicked() {
                        app.focus_mode = !app.focus_mode; ui.close_menu();
                    }
                    if ui.button("消息日志").clicked() { app.show_log = !app.show_log; ui.close_menu(); }
                    if ui.button("设置").clicked() { app.open_settings(); ui.close_menu(); }
                });
                ui.menu_button(RichText::new("工具").color(app.pal.text), |ui| {
                    let sc = sel_conn.clone();
                    if ui.button("SQL 查询编辑器").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.new_query(id);} ui.close_menu(); }
                    if ui.button("查询构建器").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_query_builder(id);} ui.close_menu(); }
                    if ui.button("ER 图 / 模型").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_er(id);} ui.close_menu(); }
                    ui.separator();
                    if ui.button("数据传输").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_transfer(id);} ui.close_menu(); }
                    if ui.button("导入向导").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_import(id,String::new());} ui.close_menu(); }
                    if ui.button("导出向导").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_export(id,String::new());} ui.close_menu(); }
                    if ui.button("备份 / 还原").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_backup(id);} ui.close_menu(); }
                    ui.separator();
                    if ui.button("服务器监控").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_monitor(id);} ui.close_menu(); }
                    if ui.button("数据剖析").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_profiling(id,String::new());} ui.close_menu(); }
                    if ui.button("数据生成").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_datagen(id,String::new());} ui.close_menu(); }
                    if ui.button("数据脱敏").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_masking(id,String::new());} ui.close_menu(); }
                    if ui.button("数据同步 / 结构同步").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_sync(id);} ui.close_menu(); }
                    if ui.button("对象管理（索引/触发器/视图）").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_objects(id,String::new());} ui.close_menu(); }
                    if ui.button("数据泵（SQL 导出/执行）").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_pump(id);} ui.close_menu(); }
                    if ui.button("模式分析").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_schema(id);} ui.close_menu(); }
                    if ui.button("SQL 大纲 / 重构").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_outline(id);} ui.close_menu(); }
                    if ui.button("模型设计器").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_model(id);} ui.close_menu(); }
                    if ui.button("查找于数据库").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_find(id);} ui.close_menu(); }
                    if ui.button("图表 / 仪表盘").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_chart(id);} ui.close_menu(); }
                    if ui.button("报表").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_report(id);} ui.close_menu(); }
                    if ui.button("自动化").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_automation(id);} ui.close_menu(); }
                    ui.separator();
                    if ui.button("AI 助手").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_ai(id);} ui.close_menu(); }
                    if ui.button("数据字典").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_dict(id,String::new());} ui.close_menu(); }
                    if ui.button("用户与权限").clicked() { if let Some(id)=sc.clone(){app.open_connection(id.clone());app.open_users(id);} ui.close_menu(); }
                    if ui.button("代码片段").clicked() { app.open_snippets(); ui.close_menu(); }
                    if ui.button("查询历史").clicked() { app.open_history(); ui.close_menu(); }
                });
                ui.menu_button(RichText::new("帮助").color(app.pal.text), |ui| {
                    if ui.button("关于 Navicat-RS").clicked() { app.dialog = Some(Dialog::About); ui.close_menu(); }
                });
            });
        });
}

fn toolbar(app: &mut NavicatApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("toolbar")
        .frame(egui::Frame::none().fill(app.pal.toolbar).stroke(Stroke::new(1.0, app.pal.border)).inner_margin(egui::Margin::symmetric(8.0, 5.0)))
        .show(ctx, |ui| {
            let sel = app.selected_conn.clone().or_else(|| app.store.connections.first().map(|c| c.id.clone()));
            ui.horizontal(|ui| {
                if tb(ui, app.pal.text, "\u{2795}", "新建连接").clicked() { app.new_connection(DbKind::MySql); }
                ui.separator();
                if tb(ui, app.pal.text, "\u{1F4C4}", "查询").clicked() { if let Some(id)=sel.clone(){app.open_connection(id.clone());app.new_query(id);} }
                if tb(ui, app.pal.text, "\u{1F5C2}", "表").clicked() { if let Some(id)=sel.clone(){app.open_connection(id.clone());app.open_designer(id,None);} }
                if tb(ui, app.pal.text, "\u{1F527}", "设计表").clicked() { if let Some(id)=sel.clone(){app.open_connection(id.clone());app.open_designer(id,None);} }
                if tb(ui, app.pal.text, "\u{1F5FA}", "ER 图").clicked() { if let Some(id)=sel.clone(){app.open_connection(id.clone());app.open_er(id);} }
                if tb(ui, app.pal.text, "\u{1F9F1}", "查询构建").clicked() { if let Some(id)=sel.clone(){app.open_connection(id.clone());app.open_query_builder(id);} }
                ui.separator();
                if tb(ui, app.pal.text, "\u{21C4}", "传输").clicked() { if let Some(id)=sel.clone(){app.open_connection(id.clone());app.open_transfer(id);} }
                if tb(ui, app.pal.text, "\u{2B07}", "导入").clicked() { if let Some(id)=sel.clone(){app.open_connection(id.clone());app.open_import(id,String::new());} }
                if tb(ui, app.pal.text, "\u{2B06}", "导出").clicked() { if let Some(id)=sel.clone(){app.open_connection(id.clone());app.open_export(id,String::new());} }
                if tb(ui, app.pal.text, "\u{1F4BE}", "备份").clicked() { if let Some(id)=sel.clone(){app.open_connection(id.clone());app.open_backup(id);} }
                ui.separator();
                if tb(ui, app.pal.text, "\u{1F4C8}", "监控").clicked() { if let Some(id)=sel.clone(){app.open_connection(id.clone());app.open_monitor(id);} }
                if tb(ui, app.pal.text, "\u{1F4CA}", "图表").clicked() { if let Some(id)=sel.clone(){app.open_connection(id.clone());app.open_chart(id);} }
                if tb(ui, app.pal.text, "\u{1F50E}", "剖析").clicked() { if let Some(id)=sel.clone(){app.open_connection(id.clone());app.open_profiling(id,String::new());} }
                if tb(ui, app.pal.text, "\u{25D0}", "脱敏").clicked() { if let Some(id)=sel.clone(){app.open_connection(id.clone());app.open_masking(id,String::new());} }
                if tb(ui, app.pal.text, "\u{21CB}", "同步").clicked() { if let Some(id)=sel.clone(){app.open_connection(id.clone());app.open_sync(id);} }
                if tb(ui, app.pal.text, "\u{2728}", "AI").clicked() { if let Some(id)=sel.clone(){app.open_connection(id.clone());app.open_ai(id);} }
            });
        });
}

fn tb(ui: &mut egui::Ui, color: Color32, icon: &str, label: &str) -> egui::Response {
    let text = format!("{} {}", icon, label);
    ui.add(egui::Button::new(RichText::new(text).size(13.0).color(color)).frame(false))
        .on_hover_text(label)
}

fn status_bar(app: &mut NavicatApp, ctx: &egui::Context) {
    egui::TopBottomPanel::bottom("statusbar")
        .frame(egui::Frame::none().fill(app.pal.toolbar).stroke(Stroke::new(1.0, app.pal.border)).inner_margin(egui::Margin::symmetric(10.0, 3.0)))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("\u{2714} 全部功能已解锁 · 免费开放 · 无任何权限限制").color(app.pal.success).size(12.0));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new(&app.status).size(12.0).color(app.pal.text_dim));
                    let open = app.engines.len();
                    ui.label(RichText::new(format!("连接: {} 活动 | 标签: {}", open, app.tabs.len())).size(12.0).color(app.pal.text_dim));
                });
            });
        });
}

fn tab_bar(app: &mut NavicatApp, ui: &mut egui::Ui) {
    let mut close_idx: Option<usize> = None;
    egui::ScrollArea::horizontal().id_source("tabscroll").max_height(34.0).show(ui, |ui| {
        ui.horizontal(|ui| {
            for i in 0..app.tabs.len() {
                let active = i == app.active;
                let title = app.tabs[i].title();
                let icon = app.tabs[i].icon();
                let bg = if active { app.pal.tab_active } else { app.pal.tab_inactive };
                let fg = if active { app.pal.accent } else { app.pal.text_dim };
                egui::Frame::none()
                    .fill(bg)
                    .stroke(Stroke::new(1.0, app.pal.border))
                    .inner_margin(egui::Margin::symmetric(9.0, 5.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let resp = ui.add(
                                egui::Label::new(RichText::new(format!("{} {}", icon, title)).size(12.5).color(fg))
                                    .sense(egui::Sense::click()),
                            );
                            if resp.clicked() {
                                app.active = i;
                            }
                            if app.tabs.len() > 1 && ui.small_button("\u{00D7}").clicked() {
                                close_idx = Some(i);
                            }
                        });
                    });
            }
        });
    });
    if let Some(i) = close_idx {
        app.tabs.remove(i);
        if app.active >= app.tabs.len() && !app.tabs.is_empty() {
            app.active = app.tabs.len() - 1;
        }
    }
}

fn content(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) {
    // Disambiguate borrow of app.tabs[idx].kind versus &mut app
    let is = match &app.tabs[idx].kind {
        TabKind::Welcome => 0,
        TabKind::Table(_) => 1,
        TabKind::Query(_) => 2,
        TabKind::Designer(_) => 3,
        TabKind::Er(_) => 4,
        TabKind::QueryBuilder(_) => 5,
        _ => 6,
    };
    match is {
        0 => welcome(app, ui),
        1 => table::draw(app, ui, idx),
        2 => query::draw(app, ui, idx),
        3 => designer::draw(app, ui, idx),
        4 => er::draw(app, ui, idx),
        5 => qb::draw(app, ui, idx),
        _ => tools::draw(app, ui, idx),
    }
}

pub fn card<'a>(app: &NavicatApp) -> egui::Frame {
    egui::Frame::none()
        .fill(app.pal.panel)
        .stroke(Stroke::new(1.0, app.pal.border))
        .rounding(egui::Rounding::same(8.0))
        .inner_margin(egui::Margin::same(14.0))
}

fn welcome(app: &mut NavicatApp, ui: &mut egui::Ui) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(12.0);
        ui.horizontal(|ui| {
            ui.add_space(12.0);
            ui.vertical(|ui| {
                ui.label(RichText::new("navicat-rs").size(26.0).strong().color(app.pal.accent));
                ui.label(RichText::new("多引擎数据库管理 IDE · 全部付费功能免费开放 · 无登录 · 无试用 · 无权限限制").size(14.0).color(app.pal.text_dim));
                ui.add_space(10.0);
                card(app).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("\u{1F513} 已解锁").color(app.pal.success).strong());
                        ui.label("数据剖析 · 数据生成 · 数据脱敏 · 查询构建器 · ER 模型 · 图表/仪表盘 · AI 助手 · 自动化 · 报表 · 服务器监控 · 备份/还原 · 数据传输 · 数据同步");
                    });
                });
                ui.add_space(14.0);
                ui.label(RichText::new("快速开始").size(16.0).strong());
                ui.add_space(6.0);
                let conn = app.store.connections.first().map(|c| c.id.clone());
                ui.horizontal_wrapped(|ui| {
                    if big_btn(ui, "\u{2795}", "新建连接").clicked() { app.new_connection(DbKind::MySql); }
                    if big_btn(ui, "\u{1F4C4}", "新建查询").clicked() {
                        if let Some(id) = conn.clone() { app.open_connection(id.clone()); app.new_query(id); }
                    }
                    if big_btn(ui, "\u{1F5FA}", "ER 图").clicked() {
                        if let Some(id) = conn.clone() { app.open_connection(id.clone()); app.open_er(id); }
                    }
                    if big_btn(ui, "\u{1F4CA}", "图表").clicked() {
                        if let Some(id) = conn.clone() { app.open_connection(id.clone()); app.open_chart(id); }
                    }
                    if big_btn(ui, "\u{2728}", "AI 助手").clicked() {
                        if let Some(id) = conn.clone() { app.open_connection(id.clone()); app.open_ai(id); }
                    }
                });
                ui.add_space(14.0);
                ui.label(RichText::new("已保存的连接").size(16.0).strong());
                ui.add_space(6.0);
                if app.store.connections.is_empty() {
                    ui.label(RichText::new("暂无连接。点击“新建连接”添加，或选择 SQLite 使用内置示例库。").color(app.pal.text_dim));
                } else {
                    let conns: Vec<(String, String, String)> = app.store.connections.iter().map(|c| (c.id.clone(), c.name.clone(), format!("{} · {}", c.kind.label(), c.target_desc()))).collect();
                    for (id, name, desc) in conns {
                        card(app).show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("\u{1F5C4}").size(18.0));
                                ui.vertical(|ui| {
                                    ui.label(RichText::new(name).strong());
                                    ui.label(RichText::new(desc).size(12.0).color(app.pal.text_dim));
                                });
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if ui.button("打开").clicked() { app.open_connection(id.clone()); }
                                    if ui.button("编辑").clicked() { app.edit_connection(id.clone()); }
                                });
                            });
                        });
                        ui.add_space(6.0);
                    }
                }
            });
        });
    });
}

fn big_btn(ui: &mut egui::Ui, icon: &str, label: &str) -> egui::Response {
    ui.add_sized([150.0, 64.0], egui::Button::new(RichText::new(format!("{}\n{}", icon, label)).size(14.0)))
}

fn dialogs(app: &mut NavicatApp, ctx: &egui::Context) {
    let dlg = match app.dialog.take() {
        Some(d) => d,
        None => return,
    };
    match dlg {
        Dialog::Conn(mut form) => {
            let mut open = true;
            let mut act = FormAction::None;
            egui::Window::new(if form.is_new { "新建连接" } else { "编辑连接" })
                .collapsible(false).resizable(false).open(&mut open)
                .show(ctx, |ui| {
                    act = connection_form_ui(app, &mut form, ui);
                });
            match act {
                FormAction::Save => {
                    if form.conn.name.trim().is_empty() {
                        form.error = "连接名不能为空".into();
                        app.dialog = Some(Dialog::Conn(form));
                    } else {
                        app.commit_connection(form.conn, form.is_new);
                    }
                }
                FormAction::Cancel => {}
                FormAction::None => {
                    if open { app.dialog = Some(Dialog::Conn(form)); }
                }
            }
        }
        Dialog::Confirm(data) => {
            let mut open = true;
            let mut do_it = false;
            let mut cancel = false;
            egui::Window::new(data.title.clone()).collapsible(false).resizable(false).open(&mut open).show(ctx, |ui| {
                ui.label(&data.msg);
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if ui.button(RichText::new("确定").color(app.pal.danger)).clicked() { do_it = true; }
                    if ui.button("取消").clicked() { cancel = true; }
                });
            });
            if do_it {
                app.process_act(data.action.clone());
            } else if open && !cancel {
                app.dialog = Some(Dialog::Confirm(data));
            }
        }
        Dialog::Input(mut data) => {
            let mut open = true;
            let mut ok = false;
            let mut cancel = false;
            egui::Window::new(data.title.clone()).collapsible(false).resizable(false).open(&mut open).show(ctx, |ui| {
                ui.label(&data.label);
                ui.add(egui::TextEdit::singleline(&mut data.value).desired_width(380.0));
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if ui.button("确定").clicked() { ok = true; }
                    if ui.button("取消").clicked() { cancel = true; }
                });
            });
            if ok {
                app.process_act(data.action.clone());
            } else if open && !cancel {
                app.dialog = Some(Dialog::Input(data));
            }
        }
        Dialog::About => {
            let mut open = true;
            egui::Window::new("关于 Navicat-RS").collapsible(false).resizable(false).open(&mut open).show(ctx, |ui| {
                ui.label(RichText::new("navicat-rs").size(18.0).strong().color(app.pal.accent));
                ui.label("基于 eframe/egui 与 rusqlite 构建的本地数据库管理 IDE。");
                ui.label("技术栈: Rust · eframe/egui · rusqlite · 纯 Rust PostgreSQL 驱动。");
                ui.separator();
                ui.label("所有付费/订阅功能均已免费开放，无登录、无试用期、无功能限制。");
            });
            if open {
                app.dialog = Some(Dialog::About);
            }
        }
    }
}

enum FormAction { None, Save, Cancel }

fn connection_form_ui(app: &mut NavicatApp, form: &mut crate::app::ConnectionForm, ui: &mut egui::Ui) -> FormAction {
    ui.set_min_width(430.0);
    egui::Grid::new("connform").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| {
        ui.label("连接名");
        ui.add(egui::TextEdit::singleline(&mut form.conn.name).desired_width(300.0));
        ui.end_row();
        ui.label("类型");
        egui::ComboBox::from_id_source("kindsel")
            .selected_text(form.conn.kind.label())
            .show_ui(ui, |ui| {
                let cur = form.conn.kind;
                for k in DbKind::all() {
                    if ui.selectable_label(cur == k, k.label()).clicked() {
                        let was_sqlite = form.conn.kind.is_sqlite();
                        form.conn.kind = k;
                        if k.default_port() != 0 { form.conn.port = k.default_port(); }
                        if k.is_sqlite() && form.conn.sqlite_path.is_empty() {
                            form.conn.sqlite_path = crate::app::default_export_dir().join("sampledb.sqlite").to_string_lossy().to_string();
                        }
                        if !was_sqlite && k.is_sqlite() { form.conn.sqlite_path = crate::app::default_export_dir().join("sampledb.sqlite").to_string_lossy().to_string(); }
                    }
                }
            });
        ui.end_row();
        if form.conn.kind.is_sqlite() {
            ui.label("数据库文件");
            ui.add(egui::TextEdit::singleline(&mut form.conn.sqlite_path).desired_width(300.0).hint_text("留空则使用内存示例库"));
            ui.end_row();
            ui.label("");
            if ui.button("使用内置示例库 (内存)").clicked() { form.conn.sqlite_path.clear(); }
            ui.end_row();
        } else {
            ui.label("主机");
            ui.add(egui::TextEdit::singleline(&mut form.conn.host).desired_width(300.0));
            ui.end_row();
            ui.label("端口");
            let mut port = form.conn.port as i64;
            if ui.add(egui::DragValue::new(&mut port).range(0..=65535)).changed() { form.conn.port = port as u16; }
            ui.end_row();
            ui.label("用户名");
            ui.add(egui::TextEdit::singleline(&mut form.conn.username).desired_width(300.0));
            ui.end_row();
            ui.label("密码");
            ui.add(egui::TextEdit::singleline(&mut form.conn.password).password(true).desired_width(300.0));
            ui.end_row();
            ui.label("数据库");
            ui.add(egui::TextEdit::singleline(&mut form.conn.database).desired_width(300.0));
            ui.end_row();
            ui.label("选项");
            ui.horizontal(|ui| {
                ui.checkbox(&mut form.conn.ssl, "SSL/TLS");
                ui.checkbox(&mut form.conn.ssh_tunnel, "SSH 隧道");
            });
            ui.end_row();
            if form.conn.ssh_tunnel {
                ui.label("SSH 主机");
                ui.horizontal(|ui| {
                    ui.add(egui::TextEdit::singleline(&mut form.conn.ssh_host).desired_width(180.0));
                    ui.label("端口");
                    let mut p = form.conn.ssh_port as i64;
                    if ui.add(egui::DragValue::new(&mut p).range(0..=65535)).changed() { form.conn.ssh_port = p as u16; }
                });
                ui.end_row();
                ui.label("SSH 用户");
                ui.add(egui::TextEdit::singleline(&mut form.conn.ssh_user).desired_width(300.0));
                ui.end_row();
            }
            ui.label("");
            ui.label(RichText::new("提示: 非 SQLite 引擎以离线演示库呈现（界面/工作流完整，无需真实服务器）。").size(11.0).color(app.pal.text_dim));
            ui.end_row();
        }
    });
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        if ui.button("测试连接").clicked() {
            form.error = match form.conn.kind {
                DbKind::PostgreSql => match crate::realdb::pg_test(&form.conn) {
                    Ok(v) => format!("\u{2714} 连接成功: {}", v),
                    Err(e) => format!("\u{2716} 连接失败: {}", e),
                },
                DbKind::Sqlite => "\u{2714} SQLite 为嵌入式引擎，保存后直接可用。".to_string(),
                _ => "该引擎以离线演示库运行（界面与工作流完整）；当前已内置真实驱动的引擎为 PostgreSQL。".to_string(),
            };
        }
    });
    if !form.error.is_empty() {
        ui.colored_label(app.pal.danger, &form.error);
    }
    ui.add_space(8.0);
    let mut act = FormAction::None;
    ui.horizontal(|ui| {
        if ui.button(RichText::new("保存并连接").color(app.pal.contrast_text(app.pal.accent)).background_color(app.pal.accent)).clicked() {
            act = FormAction::Save;
        }
        if ui.button("取消").clicked() {
            act = FormAction::Cancel;
        }
    });
    act
}

pub fn mini_badge(ui: &mut egui::Ui, text: &str, color: Color32) {
    egui::Frame::none().fill(color.gamma_multiply(0.15)).rounding(egui::Rounding::same(6.0)).inner_margin(egui::Margin::symmetric(6.0, 2.0)).show(ui, |ui| {
        ui.label(RichText::new(text).size(11.0).color(color));
    });
}

pub fn conn_badge(ui: &mut egui::Ui, app: &NavicatApp, conn_id: &str) {
    if let Some(c) = app.conn(conn_id) {
        egui::Frame::none().fill(c.color32().gamma_multiply(0.15)).rounding(egui::Rounding::same(5.0)).inner_margin(egui::Margin::symmetric(6.0, 2.0)).show(ui, |ui| {
            ui.label(RichText::new(format!("{} {}", c.kind.short(), short_id(&c.id))).size(11.0).color(c.color32()));
        });
    }
}
