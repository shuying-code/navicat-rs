use crate::app::{NavicatApp, TabKind};
use egui::{Color32, Pos2, Rect, RichText, Stroke, Vec2};

pub fn draw(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) {
    let pal = app.pal.clone();
    let mut refresh = false;
    let conn_id = match &app.tabs[idx].kind {
        TabKind::Er(e) => e.conn_id.clone(),
        _ => return,
    };
    egui::Frame::none().fill(pal.panel).inner_margin(egui::Margin::symmetric(8.0, 5.0)).show(ui, |ui| {
        ui.horizontal(|ui| {
            crate::ui::conn_badge(ui, app, &conn_id);
            ui.label(RichText::new("\u{1F5FA} ER 图 / 模型").strong().size(14.0));
            ui.separator();
            if ui.button("\u{1F504} 刷新").clicked() { refresh = true; }
            if let TabKind::Er(e) = &mut app.tabs[idx].kind {
                ui.label("缩放:");
                ui.add(egui::Slider::new(&mut e.zoom, 0.4..=2.0).show_value(false));
                ui.label(format!("{} 张表 · {} 个外键", e.tables.len(), e.fks.len()));
            }
        });
    });
    ui.separator();

    let avail = ui.available_rect_before_wrap();
    let painter = ui.painter_at(avail);
    painter.rect_filled(avail, 0.0, pal.bg);
    // grid dots
    let grid = 24.0;
    let mut gx = avail.left();
    while gx < avail.right() {
        let mut gy = avail.top();
        while gy < avail.bottom() {
            painter.circle_filled(Pos2::new(gx, gy), 0.6, pal.border);
            gy += grid;
        }
        gx += grid;
    }

    let e = match &mut app.tabs[idx].kind { TabKind::Er(e) => e, _ => return };
    let zoom = e.zoom;
    let origin = avail.min;
    let mut centers = std::collections::HashMap::new();
    // draw edges first
    for (child, _cc, parent, _pc) in &e.fks {
        if let (Some(cp), Some(pp)) = (e.pos.get(child), e.pos.get(parent)) {
            let c = origin + Vec2::new(cp.0 + 100.0, cp.1 + 20.0) * zoom;
            let p = origin + Vec2::new(pp.0 + 100.0, pp.1 + 20.0) * zoom;
            painter.line_segment([c, p], Stroke::new(1.4, pal.accent.gamma_multiply(0.7)));
            painter.circle_filled(p, 3.0, pal.accent);
        }
    }
    // draw boxes
    let mut i = 0;
    for (name, cols) in &e.tables {
        let p = e.pos.get(name).copied().unwrap_or((40.0, 40.0 + i as f32 * 30.0));
        i += 1;
        let w = 210.0 * zoom;
        let head_h = 26.0 * zoom;
        let row_h = 17.0 * zoom;
        let h = head_h + row_h * cols.len() as f32 + 6.0 * zoom;
        let rect = Rect::from_min_size(origin + Vec2::new(p.0, p.1) * zoom, Vec2::new(w, h));
        painter.rect_filled(rect, 6.0, pal.panel);
        painter.rect_stroke(rect, 6.0, Stroke::new(1.2, pal.border));
        let head = Rect::from_min_size(rect.min, Vec2::new(w, head_h));
        painter.rect_filled(head, 6.0, pal.accent);
        painter.text(head.center(), egui::Align2::CENTER_CENTER, name, egui::FontId::proportional(13.0 * zoom.min(1.3)), Color32::WHITE);
        for (ci, c) in cols.iter().enumerate() {
            let y = rect.min.y + head_h + row_h * ci as f32 + row_h * 0.5;
            painter.text(Pos2::new(rect.min.x + 10.0 * zoom, y), egui::Align2::LEFT_CENTER, c, egui::FontId::proportional(11.5), pal.text);
        }
        centers.insert(name.clone(), rect.center());
    }
    if e.tables.is_empty() {
        painter.text(avail.center(), egui::Align2::CENTER_CENTER, "无表可显示", egui::FontId::proportional(16.0), pal.text_dim);
    }
    if refresh {
        app.open_er(conn_id);
    }
}
