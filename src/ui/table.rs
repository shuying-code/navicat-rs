use crate::app::{NavicatApp, TabKind}; // 引入应用对象与标签类型
use egui::{RichText, Stroke}; // 引入 egui 类型

pub fn draw(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) { // 绘制数据表面板
    let pal = app.pal.clone(); // 克隆调色板
    let mut commit: Option<(i64, String, String, bool)> = None; // 待提交的单元格编辑
    let mut refresh = false; // 是否刷新
    let mut new_row = false; // 是否新增行
    let mut del_row: Option<i64> = None; // 待删除行 id
    let mut new_page: Option<usize> = None; // 待切换页码
    let mut sort_req: Option<String> = None; // 待排序请求
    let mut do_export = false; // 是否导出

    let (conn_id, table, has_rowid) = match &app.tabs[idx].kind { // 读取表标签信息
        TabKind::Table(t) => (t.conn_id.clone(), t.table.clone(), t.has_rowid), // 取连接 id、表名、是否有 rowid
        _ => return, // 非表标签则返回
    }; // match 结束

    // Toolbar
    egui::Frame::none().fill(pal.panel).inner_margin(egui::Margin::symmetric(8.0, 5.0)).show(ui, |ui| { // 工具栏面板
        ui.horizontal(|ui| { // 横向布局
            crate::ui::conn_badge(ui, app, &conn_id); // 连接徽标
            ui.label(RichText::new("\u{1F5CB}").size(16.0)); // 表图标
            ui.label(RichText::new(&table).strong().size(14.0)); // 表名
            ui.separator(); // 分隔线
            if ui.button("\u{2795} 添加记录").clicked() { new_row = true; } // 新增记录按钮
            if ui.button("\u{1F5D1} 删除记录").clicked() { // 删除记录按钮
                if let TabKind::Table(t) = &app.tabs[idx].kind { // 取表标签
                    if let Some(r) = t.selected_row { if let Some((rowid, _)) = t.rows.get(r) { del_row = Some(*rowid); } } // 取选中行的 rowid
                } // if 结束
            } // if 结束
            if ui.button("\u{1F504} 刷新").clicked() { refresh = true; } // 刷新按钮
            ui.separator(); // 分隔线
            if ui.button("\u{2B06} 导出").clicked() { do_export = true; } // 导出按钮
            if ui.button("\u{1F527} 设计表").clicked() { // 设计表按钮
                app.open_designer(conn_id.clone(), Some(table.clone())); // 打开设计器
            } // if 结束
        }); // 横向结束
        ui.horizontal(|ui| { // 横向布局
            ui.label("过滤:"); // “过滤”标签
            if let TabKind::Table(t) = &mut app.tabs[idx].kind { // 取可变表标签
                let resp = ui.add(egui::TextEdit::singleline(&mut t.filter).hint_text("如 country = 'China'").desired_width(320.0)); // 过滤输入框（回车触发刷新）
                if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) { refresh = true; } // 回车时刷新
                ui.label(RichText::new(format!("共 {} 行", t.total)).color(pal.text_dim).size(12.0)); // 总行数
                ui.separator(); // 分隔线
                let pages = ((t.total as f64) / (t.page_size as f64)).ceil() as i64; // 总页数
                let pages = pages.max(1); // 至少为 1 页
                if ui.button("\u{25C0}").clicked() { if t.page > 0 { new_page = Some(t.page - 1); } } // 上一页按钮
                ui.label(format!("第 {}/{} 页", t.page + 1, pages)); // 页码显示
                if ui.button("\u{25B6}").clicked() { if ((t.page + 1) as i64) < pages { new_page = Some(t.page + 1); } } // 下一页按钮
                let mut ps = t.page_size as i64; // 每页行数变量
                if ui.add(egui::DragValue::new(&mut ps).range(10..=5000).prefix("每页 ")).changed() { // 每页行数拖拽框
                    t.page_size = ps as usize; t.page = 0; refresh = true; // 更新每页并刷新
                } // if 结束
            } // if 结束
        }); // 横向结束
    }); // 面板结束
    ui.separator(); // 分隔线

    // Grid
    let t = match &mut app.tabs[idx].kind { // 取可变表标签
        TabKind::Table(t) => t, // 表标签匹配
        _ => return, // 非表标签则返回
    }; // match 结束
    if !t.error.is_empty() { // 若有错误
        ui.colored_label(pal.danger, &t.error); // 红色显示错误
    } // if 结束
    let cols = t.columns.clone(); // 克隆列定义
    if cols.is_empty() { // 若无列
        ui.label("（无列 / 视图无 schema）"); // 空态提示
    } else { // 否则
        egui::ScrollArea::both().id_source("gridscroll").auto_shrink([false, false]).show(ui, |ui| { // 双向滚动区
            egui::Grid::new("datagrid").striped(true).min_col_width(60.0).spacing([1.0, 1.0]).show(ui, |ui| { // 数据网格
                // header
                ui.label(RichText::new("#").size(12.0).color(pal.text_dim)); // 行号表头
                for c in &cols { // 遍历列
                    let arrow = match &t.order { // 排序箭头
                        Some((oc, asc)) if oc == &c.name => if *asc { " \u{25B2}" } else { " \u{25BC}" }, // 当前排序列
                        _ => "", // 其它列无箭头
                    }; // arm 结束
                    let label = format!("{}{}", c.name, arrow); // 表头文字
                    let mut txt = RichText::new(label).size(12.5).strong(); // 加粗
                    if c.pk { txt = txt.color(pal.accent); } // 主键用强调色
                    let resp = ui.add(egui::Label::new(txt).sense(egui::Sense::click())); // 可点击表头
                    if resp.clicked() { sort_req = Some(c.name.clone()); } // 点击触发排序
                    resp.on_hover_text(format!("{}{}", c.decl_type, if c.notnull {" NOT NULL"} else {""})); // 悬停显示类型与约束
                } // for 结束
                ui.end_row(); // 换行

                for (ri, (_rowid, vals)) in t.rows.iter().enumerate() { // 遍历数据行
                    let num = ui.add(egui::Label::new(RichText::new(format!("{}", ri + 1)).size(11.0).color(pal.text_dim)).sense(egui::Sense::click())); // 行号（可点击选中）
                    if num.clicked() { t.selected_row = Some(ri); } // 点击选中该行
                    for ci in 0..cols.len() { // 遍历列
                        let val = vals.get(ci).cloned().flatten(); // 取值
                        let editing = t.editing == Some((ri, ci)); // 是否处于编辑态
                        if editing { // 若处于编辑态
                            let id = ui.make_persistent_id(("cell", ri, ci)); // 单元格持久 id
                            let resp = ui.add(egui::TextEdit::singleline(&mut t.edit_buf).id(id).desired_width(120.0)); // 编辑输入框
                            if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) { // 回车提交
                                commit = Some((_rowid_clone(&t.rows, ri), cols[ci].name.clone(), t.edit_buf.clone(), true)); // 提交编辑
                            } else if resp.lost_focus() { // 失焦也提交
                                commit = Some((_rowid_clone(&t.rows, ri), cols[ci].name.clone(), t.edit_buf.clone(), true)); // 提交编辑
                            } // if 结束
                        } else { // 否则
                            let shown = match &val { // 显示值
                                Some(s) => s.clone(), // 有值
                                None => "NULL".to_string(), // 空显示 NULL
                            }; // match 结束
                            let color = if val.is_none() { pal.text_dim } else { pal.text }; // 空值用灰色
                            let mut txt = RichText::new(shown).size(12.5).color(color); // 构造文字
                            if matches!(cols[ci].decl_type.to_uppercase().as_str(), "INTEGER" | "REAL" | "NUMERIC") { txt = txt.monospace(); } // 数值类型用等宽字体
                            let resp = ui.add(egui::Label::new(txt).sense(egui::Sense::click())); // 可点击标签
                            if resp.clicked() { t.selected_row = Some(ri); } // 点击选中该行
                            if resp.double_clicked() { // 双击进入编辑
                                t.editing = Some((ri, ci)); // 设编辑单元格
                                t.edit_buf = val.clone().unwrap_or_default(); // 填充编辑缓冲
                            } // if 结束
                        } // if 结束
                    } // 列遍历结束
                    ui.end_row(); // 换行
                } // 行遍历结束
            }); // 网格结束
        }); // 滚动区结束
    } // if 结束

    // Apply intents after borrows end
    if let Some((rowid, colname, val, _)) = commit { // 若有待提交编辑
        if let Some(eng) = app.engines.get(&conn_id) { // 若引擎存在
            let v = if val.is_empty() { None } else { Some(val.as_str()) }; // 空值转 None
            let _ = eng.update_cell(&table, rowid, &colname, v); // 更新单元格
        } // if 结束
        if let TabKind::Table(t) = &mut app.tabs[idx].kind { t.editing = None; } // 结束编辑态
        refresh = true; // 触发刷新
    } // if 结束
    if new_row { // 若新增行
        if let Some(eng) = app.engines.get(&conn_id) { // 若引擎存在
            let cols: Vec<crate::model::Column> = match &app.tabs[idx].kind { // 取列定义
                TabKind::Table(t) => t.columns.clone(), // 表标签的列
                _ => Vec::new(), // 否则空
            }; // match 结束
            // Skip auto-increment PK column(s); fill NOT NULL columns with a
            // type-appropriate placeholder so the insert satisfies constraints.
            let names: Vec<String> = cols.iter().filter(|c| !c.pk).map(|c| c.name.clone()).collect(); // 非主键列名
            let make_vals = |suffix: &str| -> Vec<Option<String>> { // 生成值的闭包（含唯一后缀）
                cols.iter() // 遍历列
                    .filter(|c| !c.pk) // 过滤非主键列
                    .map(|c| { // 映射为值
                        if c.notnull { // 若为非空约束
                            let t = c.decl_type.to_uppercase(); // 类型转大写
                            if t.contains("INT") || t.contains("REAL") || t.contains("NUM") || t.contains("DEC") || t.contains("BIG") || t.contains("BOOL") { // 数值类型填 0
                                Some("0".to_string()) // 返回 "0"
                            } else { // 否则
                                Some(suffix.to_string()) // 填入后缀文本
                            } // if 结束
                        } else { // 否则
                            None // 可空则 None
                        } // if 结束
                    }) // 闭包结束
                    .collect() // 收集
            }; // 闭包结束
            let mut result = eng.insert_row(&table, &names, &make_vals("")); // 先用空后缀插入
            if result.is_err() { // 若插入失败
                // Retry with a unique suffix (handles UNIQUE NOT NULL columns).
                let suffix = format!("v{}", rand::random::<u32>() % 100000); // 生成唯一后缀
                result = eng.insert_row(&table, &names, &make_vals(&suffix)); // 带后缀重试插入
            } // if 结束
            match result { // 匹配插入结果
                Ok(_) => app.log(format!("已添加记录到 {table}")), // 成功则记录日志
                Err(e) => { // 失败
                    if let TabKind::Table(t) = &mut app.tabs[idx].kind { t.error = format!("添加失败: {e}"); } // 显示错误
                } // 分支结束
            } // match 结束
        } // if 结束
        refresh = true; // 触发刷新
    } // if 结束
    if let Some(rowid) = del_row { // 若删除行
        if app.store.settings.confirm_delete { // 若启用删除确认
            app.dialog = Some(crate::app::Dialog::Confirm(crate::app::ConfirmData { // 弹出确认对话框
                title: "删除记录".into(), // 标题
                msg: format!("确定删除表 “{}” 中的这条记录吗？此操作不可撤销。", table), // 消息
                action: crate::app::Act::DeleteRow(conn_id.clone(), table.clone(), rowid), // 动作
            })); // 构造结束
        } else if let Some(eng) = app.engines.get(&conn_id) { // 否则若引擎存在
            let _ = eng.delete_row(&table, rowid); // 直接删除该行
            app.log("已删除记录"); // 记录日志
            refresh = true; // 触发刷新
        } // if 结束
    } // if 结束
    if let Some(p) = new_page { // 若切换页码
        if let TabKind::Table(t) = &mut app.tabs[idx].kind { t.page = p; t.editing = None; } // 更新页码并清编辑态
        refresh = true; // 触发刷新
    } // if 结束
    if let Some(col) = sort_req { // 若排序请求
        if let TabKind::Table(t) = &mut app.tabs[idx].kind { // 取可变表标签
            t.order = match &t.order { // 切换排序状态
                Some((oc, asc)) if oc == &col => Some((col.clone(), !*asc)), // 同列则方向取反
                _ => Some((col.clone(), true)), // 否则升序
            }; // match 结束
        } // if 结束
        refresh = true; // 触发刷新
    } // if 结束
    if do_export { // 若导出
        app.open_export(conn_id.clone(), table.clone()); // 打开导出面板
    } // if 结束
    if refresh { // 若刷新
        app.refresh_table_doc(idx); // 刷新表文档
    } // if 结束
    let _ = has_rowid; // 忽略未使用变量
} // draw 结束

fn _rowid_clone(rows: &[(i64, Vec<Option<String>>)], ri: usize) -> i64 { // rowid 辅助函数
    rows.get(ri).map(|r| r.0).unwrap_or(-1) // 取指定行的 rowid（无则 -1）
} // 函数结束
