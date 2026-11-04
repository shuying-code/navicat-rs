use crate::app::{NavicatApp, TabKind}; // 引入应用对象与标签类型
use crate::model::Column; // 引入列模型
use egui::{RichText, Stroke}; // 引入 egui 类型

pub fn draw(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) { // 绘制表设计器面板
    let pal = app.pal.clone(); // 克隆调色板
    let mut save = false; // 是否触发保存
    let mut add_col = false; // 是否新增列
    let mut del_col: Option<usize> = None; // 待删除的列索引

    let (conn_id, original) = match &app.tabs[idx].kind { // 读取设计器文档
        TabKind::Designer(d) => (d.conn_id.clone(), d.original.clone()), // 取连接 id 与原表名
        _ => return, // 非设计器标签则返回
    }; // match 结束

    egui::Frame::none().fill(pal.panel).inner_margin(egui::Margin::symmetric(8.0, 5.0)).show(ui, |ui| { // 顶部工具条面板
        ui.horizontal(|ui| { // 横向布局
            crate::ui::conn_badge(ui, app, &conn_id); // 连接徽标
            ui.label(RichText::new("\u{1F527}").size(16.0)); // 扳手图标
            ui.label(RichText::new(match &original { Some(t) => format!("设计表: {t}"), None => "新建表".into() }).strong().size(14.0)); // 标题（原表名或新建表）
            ui.separator(); // 分隔线
            if ui.button(RichText::new("\u{1F4BE} 保存").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { save = true; } // 保存按钮
            if ui.button("\u{2795} 添加列").clicked() { add_col = true; } // 新增列按钮
        }); // 横向结束
    }); // 工具条面板结束
    ui.separator(); // 分隔线

    egui::Frame::none().fill(pal.panel).stroke(Stroke::new(1.0, pal.border)).inner_margin(egui::Margin::same(12.0)).show(ui, |ui| { // 主体编辑面板
        let d = match &mut app.tabs[idx].kind { TabKind::Designer(d) => d, _ => return }; // 取可变的设计器文档
        ui.horizontal(|ui| { // 横向布局
            ui.label("表名:"); // “表名”标签
            ui.add(egui::TextEdit::singleline(&mut d.name).desired_width(240.0)); // 表名输入框
        }); // 横向结束
        ui.add_space(8.0); // 间距
        if !d.error.is_empty() { ui.colored_label(pal.danger, &d.error); } // 若有错误则红色显示
        egui::ScrollArea::vertical().id_source("designerscroll").auto_shrink([false, false]).show(ui, |ui| { // 纵向滚动区
            egui::Grid::new("colsgrid").striped(true).spacing([8.0, 6.0]).show(ui, |ui| { // 多列表格
                for h in ["#", "名称", "类型", "非空", "主键", "默认值", ""] { // 表头列名
                    ui.label(RichText::new(h).size(12.0).strong()); // 表头文字
                } // 表头行结束
                ui.end_row(); // 换行
                for (i, c) in d.columns.iter_mut().enumerate() { // 遍历每一列
                    ui.label(RichText::new(format!("{}", i + 1)).size(11.0).color(pal.text_dim)); // 行号
                    ui.add(egui::TextEdit::singleline(&mut c.name).desired_width(140.0)); // 列名输入框
                    egui::ComboBox::from_id_source(("typ", i)) // 类型下拉框
                        .selected_text(if c.decl_type.is_empty() { "TEXT" } else { &c.decl_type }) // 显示当前类型（空则 TEXT）
                        .width(130.0) // 下拉宽度
                        .show_ui(ui, |ui| { // 下拉内容
                            for t in ["INTEGER", "TEXT", "REAL", "BLOB", "NUMERIC", "BOOLEAN", "DATE", "DATETIME", "VARCHAR(255)", "DECIMAL(10,2)", "BIGINT"] { // 类型候选列表
                                if ui.selectable_value(&mut c.decl_type, t.to_string(), t).clicked() {} // 选择类型
                            } // 下拉结束
                        }); // 下拉结束
                    ui.checkbox(&mut c.notnull, ""); // 非空勾选框
                    ui.checkbox(&mut c.pk, ""); // 主键勾选框
                    let mut def = c.default.clone().unwrap_or_default(); // 默认值临时变量
                    if ui.add(egui::TextEdit::singleline(&mut def).desired_width(90.0)).changed() { // 默认值输入框
                        c.default = if def.is_empty() { None } else { Some(def) }; // 更新默认值（空则为 None）
                    } // if 结束
                    if ui.small_button("\u{2716}").clicked() { del_col = Some(i); } // 删除本列按钮
                    ui.end_row(); // 换行
                } // 列遍历结束
            }); // 表格结束
        }); // 滚动区结束
        ui.add_space(8.0); // 间距
        ui.label(RichText::new("提示: SQLite 修改表结构支持新增列；重命名/删除列请通过 SQL 或新建表完成。").size(11.0).color(pal.text_dim)); // SQLite 改列结构提示
    }); // 主体面板结束

    if add_col { // 若点击了新增列
        if let TabKind::Designer(d) = &mut app.tabs[idx].kind { // 取可变文档
            let cid = d.columns.len() as i64; // 新列序号
            d.columns.push(Column { cid, name: format!("column_{}", cid + 1), decl_type: "TEXT".into(), notnull: false, default: None, pk: false }); // 追加一列默认定义
        } // if 结束
    } // if 结束
    if let Some(i) = del_col { // 若点击了删除列
        if let TabKind::Designer(d) = &mut app.tabs[idx].kind { // 取可变文档
            if i < d.columns.len() { d.columns.remove(i); } // 索引有效则移除该列
        } // if 结束
    } // if 结束
    if save { // 若点击保存
        app.save_designer(idx); // 执行保存设计器
    } // if 结束
} // draw 结束
