use crate::app::NavicatApp;
use crate::model::ObjKind;
use egui::{RichText, Stroke};

enum Pending {
    Open(String),
    Edit(String),
    Delete(String),
    Toggle(String),
    Table(String, String),
    Design(String, String),
    Profiling(String, String),
    Datagen(String, String),
    Masking(String, String),
    Dict(String, String),
    Export(String, String),
    DropTable(String, String),
    Favorite(String, String),
    Unfavorite(String, String),
}

pub fn draw(app: &mut NavicatApp, ctx: &egui::Context) {
    egui::SidePanel::left("tree")
        .resizable(true)
        .default_width(app.left_w)
        .width_range(200.0..=520.0)
        .frame(egui::Frame::none().fill(app.pal.panel_alt).inner_margin(egui::Margin::same(6.0)))
        .show(ctx, |ui| {
            let mut zoom = app.store.settings.tree_zoom.clamp(0.7, 1.9);
            let mut new_zoom: Option<f32> = None;
            ui.horizontal(|ui| {
                ui.label(RichText::new("连接").size(13.0 * zoom).strong());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.small_button("\u{2795}").on_hover_text("新建连接").clicked() {
                        app.new_connection(crate::model::DbKind::MySql);
                    }
                    if ui.small_button("\u{FF0B}").on_hover_text("放大树视图 (Ctrl+滚轮)").clicked() {
                        new_zoom = Some((zoom + 0.1).min(1.9));
                    }
                    if ui.small_button("\u{FF0D}").on_hover_text("缩小树视图 (Ctrl+滚轮)").clicked() {
                        new_zoom = Some((zoom - 0.1).max(0.7));
                    }
                    ui.label(RichText::new(format!("{:.0}%", zoom * 100.0)).size(11.0).color(app.pal.text_dim))
                        .on_hover_text("树形缩放（双击此处复位）");
                });
            });
            // Ctrl+mouse-wheel over the tree panel zooms the tree only.
            if ui.rect_contains_pointer(ui.max_rect()) {
                let (ctrl, scroll) = ui.input(|i| (i.modifiers.ctrl, i.raw_scroll_delta.y));
                if ctrl && scroll.abs() > 0.01 {
                    new_zoom = Some((zoom + scroll * 0.0012).clamp(0.7, 1.9));
                }
            }
            if let Some(z) = new_zoom {
                let z = (z * 10.0).round() / 10.0;
                app.store.settings.tree_zoom = z;
                app.store.save();
                zoom = z;
            }
            ui.add(egui::TextEdit::singleline(&mut app.tree_search).hint_text("\u{1F50D} 搜索连接/对象").desired_width(f32::INFINITY));
            ui.add_space(4.0);

            let mut pending: Option<Pending> = None;
            let search = app.tree_search.to_lowercase();
            egui::ScrollArea::vertical().id_source("treescroll").auto_shrink([false, false]).show(ui, |ui| {
                // Favorites section
                let favs: Vec<(String, String)> = app.store.favorites.iter()
                    .filter_map(|k| k.split_once("||").map(|(a, b)| (a.to_string(), b.to_string())))
                    .collect();
                if !favs.is_empty() {
                    ui.label(RichText::new(format!("\u{2605} 收藏 ({})", favs.len())).size(12.0 * zoom).color(app.pal.warn));
                    for (cid, t) in &favs {
                        let resp = ui.add(egui::Label::new(RichText::new(format!("\u{2605} {}", t)).size(12.5 * zoom).color(app.pal.text)).sense(egui::Sense::click()));
                        if resp.clicked() { pending = Some(Pending::Table(cid.clone(), t.clone())); }
                        let c2 = cid.clone(); let t2 = t.clone();
                        resp.context_menu(|ui| {
                            if ui.button("打开数据").clicked() { pending = Some(Pending::Table(c2.clone(), t2.clone())); ui.close_menu(); }
                            if ui.button("取消收藏").clicked() { pending = Some(Pending::Unfavorite(c2.clone(), t2.clone())); ui.close_menu(); }
                        });
                    }
                    ui.separator();
                }
                let conns: Vec<(String, String, crate::model::DbKind)> =
                    app.store.connections.iter().map(|c| (c.id.clone(), c.name.clone(), c.kind)).collect();
                if conns.is_empty() {
                    ui.add_space(10.0);
                    ui.label(RichText::new("尚无连接").color(app.pal.text_dim));
                    ui.label(RichText::new("点击右上角 + 新建").size(11.0 * zoom).color(app.pal.text_dim));
                }
                for (id, name, kind) in conns {
                    let is_open = app.engines.contains_key(&id);
                    let expanded = *app.expanded.get(&id).unwrap_or(&false);
                    let selected = app.selected_conn.as_deref() == Some(id.as_str());
                    let bg = if selected { app.pal.selection } else { egui::Color32::TRANSPARENT };
                    egui::Frame::none().fill(bg).rounding(egui::Rounding::same(5.0)).inner_margin(egui::Margin::symmetric(4.0, 2.0)).show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let arrow = if expanded { "\u{25BC}" } else { "\u{25B6}" };
                            if ui.small_button(arrow).clicked() {
                                if is_open { pending = Some(Pending::Toggle(id.clone())); }
                                else { pending = Some(Pending::Open(id.clone())); }
                            }
                            let dot = RichText::new("\u{25CF}").color(kind.color());
                            ui.label(dot);
                            let label = ui.add(egui::Label::new(RichText::new(&name).size(13.0 * zoom).color(app.pal.text)).sense(egui::Sense::click()));
                            if label.clicked() {
                                pending = Some(Pending::Open(id.clone()));
                            }
                            if label.double_clicked() {
                                pending = Some(Pending::Edit(id.clone()));
                            }
                            label.context_menu(|ui| {
                                if ui.button("打开连接").clicked() { pending = Some(Pending::Open(id.clone())); ui.close_menu(); }
                                if ui.button("编辑连接…").clicked() { pending = Some(Pending::Edit(id.clone())); ui.close_menu(); }
                                if ui.button("删除连接").clicked() { pending = Some(Pending::Delete(id.clone())); ui.close_menu(); }
                            });
                        });
                    });

                    if is_open && expanded {
                        let objs = app.objects_for(&id);
                        let tables: Vec<String> = objs.iter().filter(|o| o.kind == ObjKind::Table && (search.is_empty() || o.name.to_lowercase().contains(&search))).map(|o| o.name.clone()).collect();
                        let views: Vec<String> = objs.iter().filter(|o| o.kind == ObjKind::View && (search.is_empty() || o.name.to_lowercase().contains(&search))).map(|o| o.name.clone()).collect();
                        ui.indent(format!("obj{}", id), |ui| {
                            // 分组标题用 CollapsingHeader：自带绘制三角，点击只折叠/展开「本组」。
                            // （此前用的 ▾/▸ 字符本机字体缺失，会显示成空心方块，故改用绘制型折叠头。）
                            if !tables.is_empty() {
                                egui::CollapsingHeader::new(
                                    RichText::new(format!("\u{25A6} 表 ({})", tables.len()))
                                        .size(12.0 * zoom)
                                        .color(app.pal.text_dim),
                                )
                                .id_source(format!("{}::grp::tables", id))
                                .default_open(true)
                                .show(ui, |ui| {
                                    for t in &tables {
                                        tree_item(ui, app, &id, t, "\u{25A6}", zoom, &mut pending);
                                    }
                                });
                            }
                            if !views.is_empty() {
                                egui::CollapsingHeader::new(
                                    RichText::new(format!("\u{25C9} 视图 ({})", views.len()))
                                        .size(12.0 * zoom)
                                        .color(app.pal.text_dim),
                                )
                                .id_source(format!("{}::grp::views", id))
                                .default_open(true)
                                .show(ui, |ui| {
                                    for v in &views {
                                        tree_item(ui, app, &id, v, "\u{25C9}", zoom, &mut pending);
                                    }
                                });
                            }
                            // Feature folders (conceptual, mirror Navicat tree) — 各自可折叠。
                            for (fkey, flabel) in [("backup", "备份"), ("model", "模型"), ("users", "用户")] {
                                egui::CollapsingHeader::new(
                                    RichText::new(format!("\u{25A4} {}", flabel))
                                        .size(12.0 * zoom)
                                        .color(app.pal.text_dim),
                                )
                                .id_source(format!("{}::grp::{}", id, fkey))
                                .default_open(false)
                                .show(ui, |ui| {
                                    ui.label(
                                        RichText::new("（暂无内容）")
                                            .size(11.0 * zoom)
                                            .color(app.pal.text_dim),
                                    );
                                });
                            }
                        });
                    }
                }
            });

            if let Some(p) = pending {
                match p {
                    Pending::Open(id) => {
                        if app.engines.contains_key(&id) {
                            let cur = *app.expanded.get(&id).unwrap_or(&false);
                            app.expanded.insert(id.clone(), !cur);
                            app.selected_conn = Some(id);
                        } else { app.open_connection(id); }
                    }
                    Pending::Toggle(id) => {
                        let cur = *app.expanded.get(&id).unwrap_or(&false);
                        app.expanded.insert(id, !cur);
                    }
                    Pending::Edit(id) => app.edit_connection(id),
                    Pending::Delete(id) => app.request_delete_connection(id),
                    Pending::Table(id, t) => app.open_table(id, t),
                    Pending::Design(id, t) => app.open_designer(id, Some(t)),
                    Pending::Profiling(id, t) => app.open_profiling(id, t),
                    Pending::Datagen(id, t) => app.open_datagen(id, t),
                    Pending::Masking(id, t) => app.open_masking(id, t),
                    Pending::Dict(id, t) => app.open_dict(id, t),
                    Pending::Export(id, t) => app.open_export(id, t),
                    Pending::DropTable(id, t) => app.dialog = Some(crate::app::Dialog::Confirm(crate::app::ConfirmData {
                        title: "删除表".into(),
                        msg: format!("确定删除表 “{}” 吗？", t),
                        action: crate::app::Act::DropTable(id, t),
                    })),
                    Pending::Favorite(id, t) => {
                        let key = format!("{}||{}", id, t);
                        if !app.store.favorites.contains(&key) { app.store.favorites.push(key); app.store.save(); }
                    }
                    Pending::Unfavorite(id, t) => {
                        let key = format!("{}||{}", id, t);
                        app.store.favorites.retain(|k| k != &key); app.store.save();
                    }
                }
            }
        });
}

fn tree_item(ui: &mut egui::Ui, app: &NavicatApp, conn_id: &str, name: &str, icon: &str, zoom: f32, pending: &mut Option<Pending>) {
    let resp = ui.add(egui::Label::new(RichText::new(format!("{} {}", icon, name)).size(12.5 * zoom).color(app.pal.text)).sense(egui::Sense::click()));
    if resp.clicked() {
        *pending = Some(Pending::Table(conn_id.to_string(), name.to_string()));
    }
    let cid = conn_id.to_string();
    let nm = name.to_string();
    let key = format!("{}||{}", conn_id, name);
    let is_fav = app.store.favorites.contains(&key);
    resp.context_menu(|ui| {
        if ui.button("打开数据").clicked() { *pending = Some(Pending::Table(cid.clone(), nm.clone())); ui.close_menu(); }
        if ui.button("设计表").clicked() { *pending = Some(Pending::Design(cid.clone(), nm.clone())); ui.close_menu(); }
        if ui.button("数据剖析").clicked() { *pending = Some(Pending::Profiling(cid.clone(), nm.clone())); ui.close_menu(); }
        if ui.button("数据生成").clicked() { *pending = Some(Pending::Datagen(cid.clone(), nm.clone())); ui.close_menu(); }
        if ui.button("数据脱敏").clicked() { *pending = Some(Pending::Masking(cid.clone(), nm.clone())); ui.close_menu(); }
        if ui.button("数据字典").clicked() { *pending = Some(Pending::Dict(cid.clone(), nm.clone())); ui.close_menu(); }
        if ui.button("导出").clicked() { *pending = Some(Pending::Export(cid.clone(), nm.clone())); ui.close_menu(); }
        if is_fav {
            if ui.button("\u{2605} 取消收藏").clicked() { *pending = Some(Pending::Unfavorite(cid.clone(), nm.clone())); ui.close_menu(); }
        } else {
            if ui.button("\u{2606} 收藏此表").clicked() { *pending = Some(Pending::Favorite(cid.clone(), nm.clone())); ui.close_menu(); }
        }
        ui.separator();
        if ui.button("删除表").clicked() { *pending = Some(Pending::DropTable(cid.clone(), nm.clone())); ui.close_menu(); }
    });
    let _ = Stroke::NONE;
}
