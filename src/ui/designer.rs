use crate::app::{NavicatApp, TabKind};
use crate::model::Column;
use egui::{RichText, Stroke};

pub fn draw(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) {
    let pal = app.pal.clone();
    let mut save = false;
    let mut add_col = false;
    let mut del_col: Option<usize> = None;

    let (conn_id, original) = match &app.tabs[idx].kind {
        TabKind::Designer(d) => (d.conn_id.clone(), d.original.clone()),
        _ => return,
    };

    egui::Frame::none().fill(pal.panel).inner_margin(egui::Margin::symmetric(8.0, 5.0)).show(ui, |ui| {
        ui.horizontal(|ui| {
            crate::ui::conn_badge(ui, app, &conn_id);
            ui.label(RichText::new("\u{1F527}").size(16.0));
            ui.label(RichText::new(match &original { Some(t) => format!("设计表: {t}"), None => "新建表".into() }).strong().size(14.0));
            ui.separator();
            if ui.button(RichText::new("\u{1F4BE} 保存").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { save = true; }
            if ui.button("\u{2795} 添加列").clicked() { add_col = true; }
        });
    });
    ui.separator();

    egui::Frame::none().fill(pal.panel).stroke(Stroke::new(1.0, pal.border)).inner_margin(egui::Margin::same(12.0)).show(ui, |ui| {
        let d = match &mut app.tabs[idx].kind { TabKind::Designer(d) => d, _ => return };
        ui.horizontal(|ui| {
            ui.label("表名:");
            ui.add(egui::TextEdit::singleline(&mut d.name).desired_width(240.0));
        });
        ui.add_space(8.0);
        if !d.error.is_empty() { ui.colored_label(pal.danger, &d.error); }
        egui::ScrollArea::vertical().id_source("designerscroll").auto_shrink([false, false]).show(ui, |ui| {
            egui::Grid::new("colsgrid").striped(true).spacing([8.0, 6.0]).show(ui, |ui| {
                for h in ["#", "名称", "类型", "非空", "主键", "默认值", ""] {
                    ui.label(RichText::new(h).size(12.0).strong());
                }
                ui.end_row();
                for (i, c) in d.columns.iter_mut().enumerate() {
                    ui.label(RichText::new(format!("{}", i + 1)).size(11.0).color(pal.text_dim));
                    ui.add(egui::TextEdit::singleline(&mut c.name).desired_width(140.0));
                    egui::ComboBox::from_id_source(("typ", i))
                        .selected_text(if c.decl_type.is_empty() { "TEXT" } else { &c.decl_type })
                        .width(130.0)
                        .show_ui(ui, |ui| {
                            for t in ["INTEGER", "TEXT", "REAL", "BLOB", "NUMERIC", "BOOLEAN", "DATE", "DATETIME", "VARCHAR(255)", "DECIMAL(10,2)", "BIGINT"] {
                                if ui.selectable_value(&mut c.decl_type, t.to_string(), t).clicked() {}
                            }
                        });
                    ui.checkbox(&mut c.notnull, "");
                    ui.checkbox(&mut c.pk, "");
                    let mut def = c.default.clone().unwrap_or_default();
                    if ui.add(egui::TextEdit::singleline(&mut def).desired_width(90.0)).changed() {
                        c.default = if def.is_empty() { None } else { Some(def) };
                    }
                    if ui.small_button("\u{2716}").clicked() { del_col = Some(i); }
                    ui.end_row();
                }
            });
        });
        ui.add_space(8.0);
        ui.label(RichText::new("提示: SQLite 修改表结构支持新增列；重命名/删除列请通过 SQL 或新建表完成。").size(11.0).color(pal.text_dim));
    });

    if add_col {
        if let TabKind::Designer(d) = &mut app.tabs[idx].kind {
            let cid = d.columns.len() as i64;
            d.columns.push(Column { cid, name: format!("column_{}", cid + 1), decl_type: "TEXT".into(), notnull: false, default: None, pk: false });
        }
    }
    if let Some(i) = del_col {
        if let TabKind::Designer(d) = &mut app.tabs[idx].kind {
            if i < d.columns.len() { d.columns.remove(i); }
        }
    }
    if save {
        app.save_designer(idx);
    }
}
