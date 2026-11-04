use crate::app::{NavicatApp, TabKind}; // 引入应用对象与标签类型
use egui::{Color32, Pos2, Rect, RichText, Stroke, Vec2}; // 引入 egui 几何/绘制类型

pub fn draw(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) { // 绘制 ER 图面板
    let pal = app.pal.clone(); // 克隆调色板
    let mut refresh = false; // 是否刷新
    let conn_id = match &app.tabs[idx].kind { // 取连接 id
        TabKind::Er(e) => e.conn_id.clone(), // Er 标签取连接 id
        _ => return, // 其它类型则返回
    }; // match 结束
    egui::Frame::none().fill(pal.panel).inner_margin(egui::Margin::symmetric(8.0, 5.0)).show(ui, |ui| { // 顶部工具条面板
        ui.horizontal(|ui| { // 横向布局
            crate::ui::conn_badge(ui, app, &conn_id); // 连接徽标
            ui.label(RichText::new("\u{1F5FA} ER 图 / 模型").strong().size(14.0)); // 标题文字
            ui.separator(); // 分隔线
            if ui.button("\u{1F504} 刷新").clicked() { refresh = true; } // 刷新按钮
            if let TabKind::Er(e) = &mut app.tabs[idx].kind { // 取可变的 Er 文档
                ui.label("缩放:"); // “缩放”标签
                ui.add(egui::Slider::new(&mut e.zoom, 0.4..=2.0).show_value(false)); // 缩放滑块
                ui.label(format!("{} 张表 · {} 个外键", e.tables.len(), e.fks.len())); // 表数与关系数统计
            } // if 结束
        }); // 横向结束
    }); // 面板结束
    ui.separator(); // 分隔线

    let avail = ui.available_rect_before_wrap(); // 可用绘制区域
    let painter = ui.painter_at(avail); // 创建画笔
    painter.rect_filled(avail, 0.0, pal.bg); // 以背景色填充整块区域
    // grid dots
    let grid = 24.0; // 网格间距
    let mut gx = avail.left(); // 网格起始 x
    while gx < avail.right() { // 横向循环
        let mut gy = avail.top(); // 网格起始 y
        while gy < avail.bottom() { // 纵向循环
            painter.circle_filled(Pos2::new(gx, gy), 0.6, pal.border); // 绘制网格点
            gy += grid; // y 步进
        } // 内层循环结束
        gx += grid; // x 步进
    } // 外层循环结束

    let e = match &mut app.tabs[idx].kind { TabKind::Er(e) => e, _ => return }; // 取可变的 Er 文档
    let zoom = e.zoom; // 缩放系数
    let origin = avail.min; // 绘制原点
    let mut centers = std::collections::HashMap::new(); // 表中心坐标缓存
    // draw edges first
    for (child, _cc, parent, _pc) in &e.fks { // 遍历外键关系
        if let (Some(cp), Some(pp)) = (e.pos.get(child), e.pos.get(parent)) { // 取两端坐标
            let c = origin + Vec2::new(cp.0 + 100.0, cp.1 + 20.0) * zoom; // 子表端坐标
            let p = origin + Vec2::new(pp.0 + 100.0, pp.1 + 20.0) * zoom; // 父表端坐标
            painter.line_segment([c, p], Stroke::new(1.4, pal.accent.gamma_multiply(0.7))); // 绘制关系连线
            painter.circle_filled(p, 3.0, pal.accent); // 在父端画圆点
        } // if 结束
    } // for 结束
    // draw boxes
    let mut i = 0; // 表序号
    for (name, cols) in &e.tables { // 遍历每张表
        let p = e.pos.get(name).copied().unwrap_or((40.0, 40.0 + i as f32 * 30.0)); // 取表位置（无则默认排布）
        i += 1; // 序号自增
        let w = 210.0 * zoom; // 框宽
        let head_h = 26.0 * zoom; // 表头高
        let row_h = 17.0 * zoom; // 行高
        let h = head_h + row_h * cols.len() as f32 + 6.0 * zoom; // 框总高
        let rect = Rect::from_min_size(origin + Vec2::new(p.0, p.1) * zoom, Vec2::new(w, h)); // 表框矩形
        painter.rect_filled(rect, 6.0, pal.panel); // 填充表框
        painter.rect_stroke(rect, 6.0, Stroke::new(1.2, pal.border)); // 描边
        let head = Rect::from_min_size(rect.min, Vec2::new(w, head_h)); // 表头矩形
        painter.rect_filled(head, 6.0, pal.accent); // 填充表头
        painter.text(head.center(), egui::Align2::CENTER_CENTER, name, egui::FontId::proportional(13.0 * zoom.min(1.3)), Color32::WHITE); // 绘制表名
        for (ci, c) in cols.iter().enumerate() { // 遍历列
            let y = rect.min.y + head_h + row_h * ci as f32 + row_h * 0.5; // 列行 y 坐标
            painter.text(Pos2::new(rect.min.x + 10.0 * zoom, y), egui::Align2::LEFT_CENTER, c, egui::FontId::proportional(11.5), pal.text); // 绘制列名
        } // 列遍历结束
        centers.insert(name.clone(), rect.center()); // 记录表中心
    } // 表遍历结束
    if e.tables.is_empty() { // 若无表
        painter.text(avail.center(), egui::Align2::CENTER_CENTER, "无表可显示", egui::FontId::proportional(16.0), pal.text_dim); // 空态提示
    } // if 结束
    if refresh { // 若点击刷新
        app.open_er(conn_id); // 重新打开 ER 图
    } // if 结束
} // draw 结束
