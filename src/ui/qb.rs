use crate::app::{NavicatApp, TabKind};
use egui::{RichText, Stroke};

pub fn draw(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) {
    let pal = app.pal.clone();
    let mut reload_cols = false;
    let mut run = false;
    let mut open_as_query = false;
    let conn_id = match &app.tabs[idx].kind {
        TabKind::QueryBuilder(q) => q.conn_id.clone(),
        _ => return,
    };

    egui::Frame::none().fill(pal.panel).inner_margin(egui::Margin::symmetric(8.0, 5.0)).show(ui, |ui| {
        ui.horizontal(|ui| {
            crate::ui::conn_badge(ui, app, &conn_id);
            ui.label(RichText::new("\u{1F9F1} 查询构建器").strong().size(14.0));
            ui.separator();
            if ui.button(RichText::new("\u{25B6} 运行").color(pal.contrast_text(pal.success)).background_color(pal.success)).clicked() { run = true; }
            if ui.button("\u{1F4C4} 在查询编辑器中打开").clicked() { open_as_query = true; }
        });
    });
    ui.separator();

    ui.columns(2, |cols| {
        // left controls
        egui::Frame::none().fill(pal.panel).stroke(Stroke::new(1.0, pal.border)).inner_margin(egui::Margin::same(12.0)).show(&mut cols[0], |ui| {
            let q = match &mut app.tabs[idx].kind { TabKind::QueryBuilder(q) => q, _ => return };
            ui.horizontal(|ui| {
                ui.label("表:");
                let names: Vec<String> = app.engines.get(&q.conn_id).map(|e| e.objects().into_iter().filter(|o| o.kind == crate::model::ObjKind::Table).map(|o| o.name).collect()).unwrap_or_default();
                egui::ComboBox::from_id_source("qbtable")
                    .selected_text(if q.table.is_empty() { "选择表".to_string() } else { q.table.clone() })
                    .show_ui(ui, |ui| {
                        for n in names {
                            if ui.selectable_label(q.table == n, &n).clicked() { q.table = n.clone(); reload_cols = true; }
                        }
                    });
            });
            ui.add_space(6.0);
            ui.label(RichText::new("选择列").strong());
            egui::ScrollArea::vertical().id_source("qbcols").max_height(220.0).show(ui, |ui| {
                for (c, on) in q.cols.iter_mut() {
                    ui.checkbox(on, c.as_str());
                }
            });
            ui.add_space(6.0);
            ui.label(RichText::new("筛选条件").strong());
            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut q.where_col).desired_width(90.0).hint_text("列"));
                egui::ComboBox::from_id_source("qbop").selected_text(q.where_op.clone()).width(60.0).show_ui(ui, |ui| {
                    for op in ["=", ">", "<", ">=", "<=", "LIKE", "!="] {
                        if ui.selectable_value(&mut q.where_op, op.to_string(), op).clicked() {}
                    }
                });
                ui.add(egui::TextEdit::singleline(&mut q.where_val).desired_width(110.0).hint_text("值"));
            });
            ui.add_space(6.0);
            ui.label(RichText::new("排序 / 限制").strong());
            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut q.order_col).desired_width(90.0).hint_text("排序列"));
                ui.checkbox(&mut q.desc, "降序");
                ui.label("LIMIT");
                ui.add(egui::TextEdit::singleline(&mut q.limit).desired_width(60.0));
            });
        });
        // right preview
        egui::Frame::none().fill(pal.panel).stroke(Stroke::new(1.0, pal.border)).inner_margin(egui::Margin::same(12.0)).show(&mut cols[1], |ui| {
            let sql = {
                let q = match &app.tabs[idx].kind { TabKind::QueryBuilder(q) => q, _ => return };
                build_sql(q)
            };
            ui.label(RichText::new("生成的 SQL").strong());
            egui::Frame::none().fill(pal.bg).rounding(egui::Rounding::same(6.0)).inner_margin(egui::Margin::same(8.0)).show(ui, |ui| {
                ui.label(RichText::new(&sql).monospace().size(12.5));
            });
            ui.add_space(6.0);
            if let TabKind::QueryBuilder(q) = &app.tabs[idx].kind {
                if q.preview.error.is_some() || !q.preview.columns.is_empty() {
                    crate::ui::query::render_result(ui, &pal, &q.preview);
                } else {
                    ui.label(RichText::new("点击“运行”查看结果").color(pal.text_dim));
                }
            }
        });
    });

    if reload_cols {
        if let TabKind::QueryBuilder(q) = &mut app.tabs[idx].kind {
            let tbl = q.table.clone();
            if let Some(eng) = app.engines.get(&q.conn_id) {
                q.cols = eng.columns(&tbl).into_iter().map(|c| (c.name, true)).collect();
                q.where_col.clear(); q.order_col.clear();
            }
        }
    }
    if run {
        let sql = match &app.tabs[idx].kind { TabKind::QueryBuilder(q) => build_sql(q), _ => String::new() };
        let res = if let TabKind::QueryBuilder(q) = &app.tabs[idx].kind {
            app.engines.get(&q.conn_id).map(|e| e.query(&sql)).unwrap_or_default()
        } else { Default::default() };
        if let TabKind::QueryBuilder(q) = &mut app.tabs[idx].kind {
            q.status = format!("{} 行", res.rows.len());
            q.preview = res;
        }
        app.log("查询构建器已执行");
    }
    if open_as_query {
        let sql = match &app.tabs[idx].kind { TabKind::QueryBuilder(q) => build_sql(q), _ => String::new() };
        app.new_query(conn_id.clone());
        let ni = app.tabs.len() - 1;
        if let TabKind::Query(q) = &mut app.tabs[ni].kind { q.sql = sql; }
    }
}

fn build_sql(q: &crate::app::QbDoc) -> String {
    let cols: Vec<String> = q.cols.iter().filter(|(_, on)| *on).map(|(c, _)| format!("\"{}\"", c)).collect();
    let collist = if cols.is_empty() { "*".to_string() } else { cols.join(", ") };
    let mut sql = format!("SELECT {} FROM \"{}\"", collist, q.table);
    if !q.where_col.trim().is_empty() {
        let val = if q.where_op == "LIKE" {
            format!("'%{}%'", q.where_val)
        } else if q.where_val.parse::<f64>().is_ok() {
            q.where_val.clone()
        } else {
            format!("'{}'", q.where_val.replace('\'', "''"))
        };
        sql.push_str(&format!(" WHERE \"{}\" {} {}", q.where_col, q.where_op, val));
    }
    if !q.order_col.trim().is_empty() {
        sql.push_str(&format!(" ORDER BY \"{}\" {}", q.order_col, if q.desc { "DESC" } else { "ASC" }));
    }
    if let Ok(n) = q.limit.parse::<i64>() { sql.push_str(&format!(" LIMIT {}", n)); }
    sql
}
