use crate::app::{NavicatApp, TabKind}; // 引入应用对象与标签类型
use egui::{RichText, Stroke}; // 引入 egui 类型

pub fn draw(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) { // 绘制查询构建器面板
    let pal = app.pal.clone(); // 克隆调色板
    let mut reload_cols = false; // 是否重载列列表
    let mut run = false; // 是否运行查询
    let mut open_as_query = false; // 是否在查询编辑器中打开
    let conn_id = match &app.tabs[idx].kind { // 取连接 id
        TabKind::QueryBuilder(q) => q.conn_id.clone(), // 取查询构建文档
        _ => return, // 其它类型则返回
    }; // match 结束

    egui::Frame::none().fill(pal.panel).inner_margin(egui::Margin::symmetric(8.0, 5.0)).show(ui, |ui| { // 顶部工具条面板
        ui.horizontal(|ui| { // 横向布局
            crate::ui::conn_badge(ui, app, &conn_id); // 连接徽标
            ui.label(RichText::new("\u{1F9F1} 查询构建器").strong().size(14.0)); // 标题文字
            ui.separator(); // 分隔线
            if ui.button(RichText::new("\u{25B6} 运行").color(pal.contrast_text(pal.success)).background_color(pal.success)).clicked() { run = true; } // 运行按钮
            if ui.button("\u{1F4C4} 在查询编辑器中打开").clicked() { open_as_query = true; } // 在查询编辑器中打开按钮
        }); // 横向结束
    }); // 面板结束
    ui.separator(); // 分隔线

    ui.columns(2, |cols| { // 左右两列布局
        // left controls
        egui::Frame::none().fill(pal.panel).stroke(Stroke::new(1.0, pal.border)).inner_margin(egui::Margin::same(12.0)).show(&mut cols[0], |ui| { // 左列：查询条件面板
            let q = match &mut app.tabs[idx].kind { TabKind::QueryBuilder(q) => q, _ => return }; // 取可变的查询构建文档
            ui.horizontal(|ui| { // 横向布局
                ui.label("表:"); // “表”标签
                let names: Vec<String> = app.engines.get(&q.conn_id).map(|e| e.objects().into_iter().filter(|o| o.kind == crate::model::ObjKind::Table).map(|o| o.name).collect()).unwrap_or_default(); // 取该连接下的所有表名
                egui::ComboBox::from_id_source("qbtable") // 表下拉框
                    .selected_text(if q.table.is_empty() { "选择表".to_string() } else { q.table.clone() }) // 选中文本（空则提示选择表）
                    .show_ui(ui, |ui| { // 下拉内容
                        for n in names { // 遍历表名
                            if ui.selectable_label(q.table == n, &n).clicked() { q.table = n.clone(); reload_cols = true; } // 选择该表并触发重载列
                        } // for 结束
                    }); // 下拉结束
            }); // 横向结束
            ui.add_space(6.0); // 间距
            ui.label(RichText::new("选择列").strong()); // “选择列”标题
            egui::ScrollArea::vertical().id_source("qbcols").max_height(220.0).show(ui, |ui| { // 纵向滚动区
                for (c, on) in q.cols.iter_mut() { // 遍历列的勾选状态
                    ui.checkbox(on, c.as_str()); // 列勾选框
                } // for 结束
            }); // 滚动区结束
            ui.add_space(6.0); // 间距
            ui.label(RichText::new("筛选条件").strong()); // “筛选条件”标题
            ui.horizontal(|ui| { // 横向布局
                ui.add(egui::TextEdit::singleline(&mut q.where_col).desired_width(90.0).hint_text("列")); // 条件列输入框
                egui::ComboBox::from_id_source("qbop").selected_text(q.where_op.clone()).width(60.0).show_ui(ui, |ui| { // 运算符下拉框
                    for op in ["=", ">", "<", ">=", "<=", "LIKE", "!="] { // 运算符候选
                        if ui.selectable_value(&mut q.where_op, op.to_string(), op).clicked() {} // 选择运算符
                    } // for 结束
                }); // 下拉结束
                ui.add(egui::TextEdit::singleline(&mut q.where_val).desired_width(110.0).hint_text("值")); // 条件值输入框
            }); // 横向结束
            ui.add_space(6.0); // 间距
            ui.label(RichText::new("排序 / 限制").strong()); // “排序 / 分页”标题
            ui.horizontal(|ui| { // 横向布局
                ui.add(egui::TextEdit::singleline(&mut q.order_col).desired_width(90.0).hint_text("排序列")); // 排序列输入框
                ui.checkbox(&mut q.desc, "降序"); // 降序勾选框
                ui.label("LIMIT"); // LIMIT 标签
                ui.add(egui::TextEdit::singleline(&mut q.limit).desired_width(60.0)); // 限制条数输入框
            }); // 横向结束
        }); // 左侧面板结束
        // right preview
        egui::Frame::none().fill(pal.panel).stroke(Stroke::new(1.0, pal.border)).inner_margin(egui::Margin::same(12.0)).show(&mut cols[1], |ui| { // 右列：SQL 预览面板
            let sql = { // 计算生成的 SQL
                let q = match &app.tabs[idx].kind { TabKind::QueryBuilder(q) => q, _ => return }; // 取查询构建文档
                build_sql(q) // 构建 SQL 文本
            }; // 块结束
            ui.label(RichText::new("生成的 SQL").strong()); // “生成的 SQL”标题
            egui::Frame::none().fill(pal.bg).rounding(egui::Rounding::same(6.0)).inner_margin(egui::Margin::same(8.0)).show(ui, |ui| { // SQL 展示框
                ui.label(RichText::new(&sql).monospace().size(12.5)); // SQL 等宽文字
            }); // 展示框结束
            ui.add_space(6.0); // 间距
            if let TabKind::QueryBuilder(q) = &app.tabs[idx].kind { // 取查询构建文档
                if q.preview.error.is_some() || !q.preview.columns.is_empty() { // 若有错误或已有结果
                    crate::ui::query::render_result(ui, &pal, &q.preview); // 渲染结果表
                } else { // 否则
                    ui.label(RichText::new("点击“运行”查看结果").color(pal.text_dim)); // 空态提示
                } // if 结束
            } // if 结束
        }); // 右侧面板结束
    }); // 两列布局结束

    if reload_cols { // 若需重载列
        if let TabKind::QueryBuilder(q) = &mut app.tabs[idx].kind { // 取可变文档
            let tbl = q.table.clone(); // 表名
            if let Some(eng) = app.engines.get(&q.conn_id) { // 若引擎存在
                q.cols = eng.columns(&tbl).into_iter().map(|c| (c.name, true)).collect(); // 重载列并默认全选
                q.where_col.clear(); q.order_col.clear(); // 清空条件与排序
            } // if 结束
        } // if 结束
    } // if 结束
    if run { // 若点击运行
        let sql = match &app.tabs[idx].kind { TabKind::QueryBuilder(q) => build_sql(q), _ => String::new() }; // 构建 SQL
        let res = if let TabKind::QueryBuilder(q) = &app.tabs[idx].kind { // 执行查询
            app.engines.get(&q.conn_id).map(|e| e.query(&sql)).unwrap_or_default() // 取查询结果
        } else { Default::default() }; // 否则用默认结果
        if let TabKind::QueryBuilder(q) = &mut app.tabs[idx].kind { // if 结束
            q.status = format!("{} 行", res.rows.len()); // 取可变文档写回
            q.preview = res; // 状态：结果行数
        } // 预览结果
        app.log("查询构建器已执行"); // if 结束
    } // 记录日志
    if open_as_query { // if 结束
        let sql = match &app.tabs[idx].kind { TabKind::QueryBuilder(q) => build_sql(q), _ => String::new() }; // 若在查询编辑器中打开
        app.new_query(conn_id.clone()); // 构建 SQL
        let ni = app.tabs.len() - 1; // 新建查询标签
        if let TabKind::Query(q) = &mut app.tabs[ni].kind { q.sql = sql; } // 新标签索引
    } // 写入 SQL 文本
} // if 结束
// draw 结束
fn build_sql(q: &crate::app::QbDoc) -> String { // 函数 build_sql
    let cols: Vec<String> = q.cols.iter().filter(|(_, on)| *on).map(|(c, _)| format!("\"{}\"", c)).collect(); // 选中的列（加引号）
    let collist = if cols.is_empty() { "*".to_string() } else { cols.join(", ") }; // 列清单或 *
    let mut sql = format!("SELECT {} FROM \"{}\"", collist, q.table); // 基础 SELECT 语句
    if !q.where_col.trim().is_empty() { // 若有筛选列
        let val = if q.where_op == "LIKE" { // 条件值处理
            format!("'%{}%'", q.where_val) // LIKE：两端加 %
        } else if q.where_val.parse::<f64>().is_ok() { // 否则若可解析为数字
            q.where_val.clone() // 直接使用数字
        } else { // 否则
            format!("'{}'", q.where_val.replace('\'', "''")) // 加引号并转义单引号
        }; // 条件值结束
        sql.push_str(&format!(" WHERE \"{}\" {} {}", q.where_col, q.where_op, val)); // 追加 WHERE 子句
    } // if 结束
    if !q.order_col.trim().is_empty() { // 若有排序列
        sql.push_str(&format!(" ORDER BY \"{}\" {}", q.order_col, if q.desc { "DESC" } else { "ASC" })); // 追加 ORDER BY
    } // if 结束
    if let Ok(n) = q.limit.parse::<i64>() { sql.push_str(&format!(" LIMIT {}", n)); } // 若有 LIMIT 则追加
    sql // 返回 SQL
} // build_sql 结束
