use crate::app::{NavicatApp, TabKind}; // 引入应用对象与标签类型
use egui::{RichText, Stroke}; // 引入 egui 类型

pub fn draw(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) { // 绘制查询面板
    let pal = app.pal.clone(); // 克隆调色板
    let mut run = false; // 是否运行
    let mut explain = false; // 是否执行 EXPLAIN
    let mut beautify = false; // 是否美化 SQL
    let mut save_snippet = false; // 是否保存为片段
    let mut active_result: Option<usize> = None; // 待激活的结果索引

    let conn_id = match &app.tabs[idx].kind { // 取连接 id
        TabKind::Query(q) => q.conn_id.clone(), // 查询标签取连接 id
        _ => return, // 其它类型则返回
    }; // match 结束

    egui::Frame::none().fill(pal.panel).inner_margin(egui::Margin::symmetric(8.0, 5.0)).show(ui, |ui| { // 顶部工具条面板
        ui.horizontal(|ui| { // 横向布局
            crate::ui::conn_badge(ui, app, &conn_id); // 连接徽标
            if ui.button(RichText::new("\u{25B6} 运行 (Ctrl+Enter)").color(pal.contrast_text(pal.success)).background_color(pal.success)).clicked() { run = true; } // 运行按钮（Ctrl+Enter）
            if ui.button("\u{1F50E} Explain").clicked() { explain = true; } // Explain 按钮
            if ui.button("\u{2728} 美化 SQL").clicked() { beautify = true; } // 美化 SQL 按钮
            if ui.button("\u{1F4CC} 保存为片段").clicked() { save_snippet = true; } // 保存为片段按钮
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| { // 右对齐布局
                if let TabKind::Query(q) = &app.tabs[idx].kind { // 取查询标签
                    ui.label(RichText::new(&q.message).size(12.0).color(pal.text_dim)); // 显示消息文字
                } // if 结束
            }); // 布局结束
        }); // 横向结束
    }); // 面板结束
    ui.separator(); // 分隔线

    let avail_h = ui.available_height(); // 可用高度
    let editor_h = (avail_h * 0.40).max(120.0); // 编辑器高度（占 40%，至少 120）

    // Editor
    egui::Frame::none().fill(pal.panel).stroke(Stroke::new(1.0, pal.border)).show(ui, |ui| { // 编辑器容器
        if let TabKind::Query(q) = &mut app.tabs[idx].kind { // 取可变查询标签
            let resp = ui.add_sized( // 多行代码编辑器
                [ui.available_width(), editor_h - 8.0], // 尺寸
                egui::TextEdit::multiline(&mut q.sql).code_editor().desired_width(f32::INFINITY).hint_text("-- 输入 SQL"), // 编辑器控件
            ); // add_sized 结束
            if resp.has_focus() && ui.input(|i| i.modifiers.command && i.key_pressed(egui::Key::Enter)) { // 若聚焦并按下 Ctrl+Enter
                run = true; // 触发运行
            } // if 结束
        } // if 结束
    }); // 容器结束
    ui.add_space(4.0); // 间距

    // Results
    let (results_len, msg) = match &app.tabs[idx].kind { // 取结果数量与消息
        TabKind::Query(q) => (q.results.len(), q.message.clone()), // 查询标签
        _ => (0, String::new()), // 否则
    }; // match 结束
    if results_len == 0 { // 若无结果
        ui.centered_and_justified(|ui| { // 居中显示
            ui.label(RichText::new("按 “运行” 或 Ctrl+Enter 执行 SQL，结果将显示在此处。").color(pal.text_dim)); // 空态提示
        }); // 居中结束
    } else { // 否则
        ui.horizontal(|ui| { // 横向布局
            if let TabKind::Query(q) = &app.tabs[idx].kind { // 取查询标签
                for (i, r) in q.results.iter().enumerate() { // 遍历结果集
                    let label = if let Some(e) = &r.error { // 结果标签文字
                        format!("结果 {}  \u{2716} 错误", i + 1) // 错误结果标签
                    } else if r.is_select { // SELECT 结果标签
                        format!("结果 {}  ({} 行)", i + 1, r.rows.len()) // DML 结果标签
                    } else { // 否则
                        format!("语句 {}  ({} 行)", i + 1, r.affected) // if 结束
                    }; // 是否当前激活结果
                    let sel = i == q.active_result; // 选中态
                    if ui.selectable_label(sel, RichText::new(label).size(12.0)).clicked() { // 点击切换激活结果
                        active_result = Some(i); // if 结束
                    } // for 结束
                } // if 结束
            } // 右对齐布局
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| { // 消息文字
                ui.label(RichText::new(msg).size(12.0).color(pal.text_dim)); // 布局结束
            }); // 横向结束
        }); // 分隔线
        ui.separator(); // 结果容器
        egui::Frame::none().fill(pal.panel).show(ui, |ui| { // 取查询标签
            if let TabKind::Query(q) = &app.tabs[idx].kind { // 激活索引限幅
                let ar = q.active_result.min(q.results.len().saturating_sub(1)); // 取激活结果
                if let Some(r) = q.results.get(ar) { // 渲染结果表
                    render_result(ui, &pal, r); // if 结束
                } // if 结束
            } // 容器结束
        }); // if 结束
    } // 块结束

    if run { // 若点击运行
        app.run_query_at(idx, None); // 执行当前标签查询
        if let TabKind::Query(q) = &app.tabs[idx].kind { // 取查询标签
            // remember in history
            let sql = q.sql.clone(); // 取 SQL
            if !sql.trim().is_empty() && !app.store.history.contains(&sql) { // 若非空且不在历史中
                app.store.history.insert(0, sql); // 插入历史首部
                if app.store.history.len() > 50 { app.store.history.truncate(50); } // 限制历史 50 条
                app.store.save(); // 保存历史
            } // if 结束
        } // if 结束
    } // if 结束
    if explain { // 若点击 Explain
        if let TabKind::Query(q) = &mut app.tabs[idx].kind { // 取可变查询标签
            let first: String = q.sql.split(';').next().unwrap_or("").to_string(); // 取第一条语句
            let sql = format!("EXPLAIN QUERY PLAN {}", first); // 构造 EXPLAIN QUERY PLAN
            let res = if let Some(eng) = app.engines.get(&q.conn_id) { eng.query(&sql) } else { Default::default() }; // 执行计划查询
            q.results = vec![explain_tree(&res)]; // 以计划树作为结果
            q.active_result = 0; // 激活结果 0
            q.message = "执行计划（可视化）".into(); // 设置消息
        } // if 结束
    } // if 结束
    if beautify { // 若点击美化
        if let TabKind::Query(q) = &mut app.tabs[idx].kind { // 取可变查询标签
            q.sql = beautify_sql(&q.sql); // 美化 SQL
        } // if 结束
    } // if 结束
    if save_snippet { // 若点击保存片段
        if let TabKind::Query(q) = &app.tabs[idx].kind { // 取查询标签
            let sql = q.sql.clone(); // 取 SQL
            app.add_snippet(format!("片段 {}", app.store.snippets.len() + 1), sql); // 添加为片段
            app.log("已保存为代码片段"); // 记录日志
        } // if 结束
    } // if 结束
    if let Some(i) = active_result { // 若切换激活结果
        if let TabKind::Query(q) = &mut app.tabs[idx].kind { q.active_result = i; } // 写回激活索引
    } // if 结束
} // draw 结束

/// Turn `EXPLAIN QUERY PLAN` rows (id, parent, notused, detail) into an
/// indented single-column tree view.
fn explain_tree(res: &crate::db::QueryResult) -> crate::db::QueryResult { // 将 EXPLAIN 行转为缩进单列树
    if res.error.is_some() || res.columns.is_empty() { // 若出错或列为空
        return res.clone(); // 直接返回
    } // if 结束
    let find = |name: &str| res.columns.iter().position(|c| c.eq_ignore_ascii_case(name)); // 列名查找闭包
    let (id_i, par_i, det_i) = match (find("id"), find("parent"), find("detail")) { // 取 id/parent/detail 列索引
        (Some(a), Some(b), Some(c)) => (a, b, c), // 若三列齐全
        _ => { // 匹配分支
            // Not the expected shape; show detail column (last) as a flat list.
            let last = res.columns.len() - 1; // 取最后一列
            let rows: Vec<Vec<Option<String>>> = res.rows.iter().map(|r| vec![r.get(last).cloned().flatten()]).collect(); // 扁平化为单列
            return crate::db::QueryResult { columns: vec![res.columns[last].clone()], rows, is_select: true, ..Default::default() }; // 返回单列结果
        } // 分支结束
    }; // match 结束
    // Build children map: parent_id -> Vec<(id, detail)>
    use std::collections::HashMap; // 引入 HashMap
    let mut children: HashMap<String, Vec<(String, String)>> = HashMap::new(); // 子节点映射：父 id → 子列表
    let mut roots: Vec<(String, String)> = Vec::new(); // 根节点列表
    for r in &res.rows { // 遍历结果行
        let id = r.get(id_i).cloned().flatten().unwrap_or_default(); // 取 id
        let parent = r.get(par_i).cloned().flatten().unwrap_or_default(); // 取 parent
        let detail = r.get(det_i).cloned().flatten().unwrap_or_default(); // 取 detail
        if parent.is_empty() || parent == "0" { // 若无父或父为 0
            roots.push((id, detail)); // 加入根
        } else { // 否则
            children.entry(parent).or_default().push((id, detail)); // 加入对应父的子列表
        } // if 结束
    } // for 结束
    let mut out: Vec<Vec<Option<String>>> = Vec::new(); // 输出行
    fn walk(id: &str, detail: &str, depth: usize, children: &std::collections::HashMap<String, Vec<(String, String)>>, out: &mut Vec<Vec<Option<String>>>) { // 递归遍历子节点
        let prefix = if depth == 0 { String::new() } else { format!("{}|- ", "   ".repeat(depth - 1)) }; // 计算缩进前缀
        out.push(vec![Some(format!("{}{}", prefix, detail))]); // 输出当前行
        if let Some(kids) = children.get(id) { // 若有子节点
            for (cid, cdet) in kids { walk(cid, cdet, depth + 1, children, out); } // 递归
        } // if 结束
    } // 递归函数结束
    for (id, detail) in &roots { walk(id, detail, 0, &children, &mut out); } // 从各根遍历
    if out.is_empty() { // 若输出为空
        // fallback flat
        for r in &res.rows { out.push(vec![r.get(det_i).cloned().flatten()]); } // 扁平回退
    } // if 结束
    crate::db::QueryResult { columns: vec!["执行计划".into()], rows: out, is_select: true, statement: res.statement.clone(), ..Default::default() } // 返回计划树结果
} // 函数结束

pub fn render_result(ui: &mut egui::Ui, pal: &crate::theme::Palette, r: &crate::db::QueryResult) { // 渲染查询结果为网格
    if let Some(e) = &r.error { // 若有错误
        ui.colored_label(pal.danger, format!("\u{2716} {}", e)); // 红色错误文字
        return; // 返回
    } // if 结束
    if !r.is_select { // 若非 SELECT
        ui.label(RichText::new(format!("\u{2714} 执行成功，{} 行受影响，用时 {:.1} ms", r.affected, r.elapsed_ms)).color(pal.success)); // 显示受影响行数与耗时
        return; // 返回
    } // if 结束
    egui::ScrollArea::both().id_source(("res", r.statement.len())).auto_shrink([false, false]).show(ui, |ui| { // 双向滚动区
        egui::Grid::new("resgrid").striped(true).min_col_width(50.0).spacing([1.0, 1.0]).show(ui, |ui| { // 结果网格
            for c in &r.columns { // 遍历列
                ui.label(RichText::new(c).size(12.0).strong().color(pal.text)); // 列名
            } // for 结束
            ui.end_row(); // 换行
            for row in r.rows.iter() { // 遍历行
                for v in row { // 遍历单元格
                    match v { // 匹配单元格值
                        Some(s) => { ui.label(RichText::new(s).size(12.0)); } // 有值则显示
                        None => { ui.label(RichText::new("NULL").size(12.0).color(pal.text_dim)); } // 空则显示 NULL
                    } // match 结束
                } // for 结束
                ui.end_row(); // 换行
            } // for 结束
        }); // 网格结束
    }); // 滚动区结束
    let _ = pal; // 忽略未使用变量
} // 函数结束

/// Non-destructive SQL pretty-printer: uppercases keywords *outside* string
/// literals / quoted identifiers and inserts line breaks before major clauses.
/// String literals and quoted identifiers are preserved verbatim.
pub fn beautify_sql(sql: &str) -> String { // 非破坏性 SQL 美化函数
    const KEYWORDS: &[&str] = &[ // 关键字常量
        "SELECT", "FROM", "WHERE", "GROUP", "BY", "ORDER", "HAVING", "LIMIT", "OFFSET", // 语句
        "JOIN", "LEFT", "RIGHT", "INNER", "OUTER", "ON", "AND", "OR", "NOT", "INSERT", // 语句
        "INTO", "VALUES", "UPDATE", "SET", "DELETE", "CREATE", "TABLE", "ALTER", "DROP", // 语句
        "AS", "DISTINCT", "UNION", "ALL", "CASE", "WHEN", "THEN", "ELSE", "END", "IS", // 语句
        "NULL", "IN", "LIKE", "BETWEEN", "ASC", "DESC", "COUNT", "SUM", "AVG", "MIN", "MAX", // 语句
    ]; // 常量结束

    // Pass 1: uppercase keyword words outside quotes.
    let mut up = String::new(); // 结果字符串
    let mut chars = sql.chars().peekable(); // 字符迭代器
    let (mut in_single, mut in_double) = (false, false); // 单/双引号状态
    while let Some(c) = chars.next() { // 遍历字符
        if in_single { // 若在单引号内
            up.push(c); // 原样推入
            if c == '\'' { in_single = false; } // 遇单引号则退出
            continue; // continue
        } // if 结束
        if in_double { // 若在双引号内
            up.push(c); // 原样推入
            if c == '"' { in_double = false; } // 遇双引号则退出
            continue; // continue
        } // if 结束
        match c { // 匹配当前字符
            '\'' => { in_single = true; up.push(c); } // 遇单引号进入单引号态
            '"' => { in_double = true; up.push(c); } // 遇双引号进入双引号态
            _ if c.is_alphabetic() => { // 若为字母
                let mut w = String::from(c); // 开始收集单词
                while let Some(&n) = chars.peek() { // 继续读字母/数字/下划线
                    if n.is_alphanumeric() || n == '_' { w.push(n); chars.next(); } else { break; } // 组成完整单词
                } // 循环结束
                let upper = w.to_uppercase(); // 转大写
                if KEYWORDS.contains(&upper.as_str()) { up.push_str(&upper); } else { up.push_str(&w); } // 是关键字则大写，否则原样
            } // 分支结束
            _ => up.push(c), // 其它字符原样推入
        } // match 结束
    } // while 结束

    // Pass 2: line breaks before major clauses.
    for k in ["FROM", "WHERE", "GROUP BY", "ORDER BY", "HAVING", "LIMIT", "UNION", "LEFT JOIN", "RIGHT JOIN", "INNER JOIN", "JOIN"] { // 遍历主要子句
        up = up.replace(&format!(" {} ", k), &format!("\n{} ", k)); // 在子句前插入换行
    } // for 结束
    up.trim().to_string() // 去空白并返回
} // beautify_sql 结束
