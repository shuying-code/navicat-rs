use crate::app::{NavicatApp, TabKind};
use egui::{RichText, Stroke};

pub fn draw(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) {
    let pal = app.pal.clone();
    let mut commit: Option<(i64, String, String, bool)> = None;
    let mut refresh = false;
    let mut new_row = false;
    let mut del_row: Option<i64> = None;
    let mut new_page: Option<usize> = None;
    let mut sort_req: Option<String> = None;
    let mut do_export = false;

    let (conn_id, table, has_rowid) = match &app.tabs[idx].kind {
        TabKind::Table(t) => (t.conn_id.clone(), t.table.clone(), t.has_rowid),
        _ => return,
    };

    // Toolbar
    egui::Frame::none().fill(pal.panel).inner_margin(egui::Margin::symmetric(8.0, 5.0)).show(ui, |ui| {
        ui.horizontal(|ui| {
            crate::ui::conn_badge(ui, app, &conn_id);
            ui.label(RichText::new("\u{1F5CB}").size(16.0));
            ui.label(RichText::new(&table).strong().size(14.0));
            ui.separator();
            if ui.button("\u{2795} 添加记录").clicked() { new_row = true; }
            if ui.button("\u{1F5D1} 删除记录").clicked() {
                if let TabKind::Table(t) = &app.tabs[idx].kind {
                    if let Some(r) = t.selected_row { if let Some((rowid, _)) = t.rows.get(r) { del_row = Some(*rowid); } }
                }
            }
            if ui.button("\u{1F504} 刷新").clicked() { refresh = true; }
            ui.separator();
            if ui.button("\u{2B06} 导出").clicked() { do_export = true; }
            if ui.button("\u{1F527} 设计表").clicked() {
                app.open_designer(conn_id.clone(), Some(table.clone()));
            }
        });
        ui.horizontal(|ui| {
            ui.label("过滤:");
            if let TabKind::Table(t) = &mut app.tabs[idx].kind {
                let resp = ui.add(egui::TextEdit::singleline(&mut t.filter).hint_text("如 country = 'China'").desired_width(320.0));
                if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) { refresh = true; }
                ui.label(RichText::new(format!("共 {} 行", t.total)).color(pal.text_dim).size(12.0));
                ui.separator();
                let pages = ((t.total as f64) / (t.page_size as f64)).ceil() as i64;
                let pages = pages.max(1);
                if ui.button("\u{25C0}").clicked() { if t.page > 0 { new_page = Some(t.page - 1); } }
                ui.label(format!("第 {}/{} 页", t.page + 1, pages));
                if ui.button("\u{25B6}").clicked() { if ((t.page + 1) as i64) < pages { new_page = Some(t.page + 1); } }
                let mut ps = t.page_size as i64;
                if ui.add(egui::DragValue::new(&mut ps).range(10..=5000).prefix("每页 ")).changed() {
                    t.page_size = ps as usize; t.page = 0; refresh = true;
                }
            }
        });
    });
    ui.separator();

    // Grid
    let t = match &mut app.tabs[idx].kind {
        TabKind::Table(t) => t,
        _ => return,
    };
    if !t.error.is_empty() {
        ui.colored_label(pal.danger, &t.error);
    }
    let cols = t.columns.clone();
    if cols.is_empty() {
        ui.label("（无列 / 视图无 schema）");
    } else {
        egui::ScrollArea::both().id_source("gridscroll").auto_shrink([false, false]).show(ui, |ui| {
            egui::Grid::new("datagrid").striped(true).min_col_width(60.0).spacing([1.0, 1.0]).show(ui, |ui| {
                // header
                ui.label(RichText::new("#").size(12.0).color(pal.text_dim));
                for c in &cols {
                    let arrow = match &t.order {
                        Some((oc, asc)) if oc == &c.name => if *asc { " \u{25B2}" } else { " \u{25BC}" },
                        _ => "",
                    };
                    let label = format!("{}{}", c.name, arrow);
                    let mut txt = RichText::new(label).size(12.5).strong();
                    if c.pk { txt = txt.color(pal.accent); }
                    let resp = ui.add(egui::Label::new(txt).sense(egui::Sense::click()));
                    if resp.clicked() { sort_req = Some(c.name.clone()); }
                    resp.on_hover_text(format!("{}{}", c.decl_type, if c.notnull {" NOT NULL"} else {""}));
                }
                ui.end_row();

                for (ri, (_rowid, vals)) in t.rows.iter().enumerate() {
                    let num = ui.add(egui::Label::new(RichText::new(format!("{}", ri + 1)).size(11.0).color(pal.text_dim)).sense(egui::Sense::click()));
                    if num.clicked() { t.selected_row = Some(ri); }
                    for ci in 0..cols.len() {
                        let val = vals.get(ci).cloned().flatten();
                        let editing = t.editing == Some((ri, ci));
                        if editing {
                            let id = ui.make_persistent_id(("cell", ri, ci));
                            let resp = ui.add(egui::TextEdit::singleline(&mut t.edit_buf).id(id).desired_width(120.0));
                            if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                commit = Some((_rowid_clone(&t.rows, ri), cols[ci].name.clone(), t.edit_buf.clone(), true));
                            } else if resp.lost_focus() {
                                commit = Some((_rowid_clone(&t.rows, ri), cols[ci].name.clone(), t.edit_buf.clone(), true));
                            }
                        } else {
                            let shown = match &val {
                                Some(s) => s.clone(),
                                None => "NULL".to_string(),
                            };
                            let color = if val.is_none() { pal.text_dim } else { pal.text };
                            let mut txt = RichText::new(shown).size(12.5).color(color);
                            if matches!(cols[ci].decl_type.to_uppercase().as_str(), "INTEGER" | "REAL" | "NUMERIC") { txt = txt.monospace(); }
                            let resp = ui.add(egui::Label::new(txt).sense(egui::Sense::click()));
                            if resp.clicked() { t.selected_row = Some(ri); }
                            if resp.double_clicked() {
                                t.editing = Some((ri, ci));
                                t.edit_buf = val.clone().unwrap_or_default();
                            }
                        }
                    }
                    ui.end_row();
                }
            });
        });
    }

    // Apply intents after borrows end
    if let Some((rowid, colname, val, _)) = commit {
        if let Some(eng) = app.engines.get(&conn_id) {
            let v = if val.is_empty() { None } else { Some(val.as_str()) };
            let _ = eng.update_cell(&table, rowid, &colname, v);
        }
        if let TabKind::Table(t) = &mut app.tabs[idx].kind { t.editing = None; }
        refresh = true;
    }
    if new_row {
        if let Some(eng) = app.engines.get(&conn_id) {
            let cols: Vec<crate::model::Column> = match &app.tabs[idx].kind {
                TabKind::Table(t) => t.columns.clone(),
                _ => Vec::new(),
            };
            // Skip auto-increment PK column(s); fill NOT NULL columns with a
            // type-appropriate placeholder so the insert satisfies constraints.
            let names: Vec<String> = cols.iter().filter(|c| !c.pk).map(|c| c.name.clone()).collect();
            let make_vals = |suffix: &str| -> Vec<Option<String>> {
                cols.iter()
                    .filter(|c| !c.pk)
                    .map(|c| {
                        if c.notnull {
                            let t = c.decl_type.to_uppercase();
                            if t.contains("INT") || t.contains("REAL") || t.contains("NUM") || t.contains("DEC") || t.contains("BIG") || t.contains("BOOL") {
                                Some("0".to_string())
                            } else {
                                Some(suffix.to_string())
                            }
                        } else {
                            None
                        }
                    })
                    .collect()
            };
            let mut result = eng.insert_row(&table, &names, &make_vals(""));
            if result.is_err() {
                // Retry with a unique suffix (handles UNIQUE NOT NULL columns).
                let suffix = format!("v{}", rand::random::<u32>() % 100000);
                result = eng.insert_row(&table, &names, &make_vals(&suffix));
            }
            match result {
                Ok(_) => app.log(format!("已添加记录到 {table}")),
                Err(e) => {
                    if let TabKind::Table(t) = &mut app.tabs[idx].kind { t.error = format!("添加失败: {e}"); }
                }
            }
        }
        refresh = true;
    }
    if let Some(rowid) = del_row {
        if app.store.settings.confirm_delete {
            app.dialog = Some(crate::app::Dialog::Confirm(crate::app::ConfirmData {
                title: "删除记录".into(),
                msg: format!("确定删除表 “{}” 中的这条记录吗？此操作不可撤销。", table),
                action: crate::app::Act::DeleteRow(conn_id.clone(), table.clone(), rowid),
            }));
        } else if let Some(eng) = app.engines.get(&conn_id) {
            let _ = eng.delete_row(&table, rowid);
            app.log("已删除记录");
            refresh = true;
        }
    }
    if let Some(p) = new_page {
        if let TabKind::Table(t) = &mut app.tabs[idx].kind { t.page = p; t.editing = None; }
        refresh = true;
    }
    if let Some(col) = sort_req {
        if let TabKind::Table(t) = &mut app.tabs[idx].kind {
            t.order = match &t.order {
                Some((oc, asc)) if oc == &col => Some((col.clone(), !*asc)),
                _ => Some((col.clone(), true)),
            };
        }
        refresh = true;
    }
    if do_export {
        app.open_export(conn_id.clone(), table.clone());
    }
    if refresh {
        app.refresh_table_doc(idx);
    }
    let _ = has_rowid;
}

fn _rowid_clone(rows: &[(i64, Vec<Option<String>>)], ri: usize) -> i64 {
    rows.get(ri).map(|r| r.0).unwrap_or(-1)
}
