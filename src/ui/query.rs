use crate::app::{NavicatApp, TabKind};
use egui::{RichText, Stroke};

pub fn draw(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) {
    let pal = app.pal.clone();
    let mut run = false;
    let mut explain = false;
    let mut beautify = false;
    let mut save_snippet = false;
    let mut active_result: Option<usize> = None;

    let conn_id = match &app.tabs[idx].kind {
        TabKind::Query(q) => q.conn_id.clone(),
        _ => return,
    };

    egui::Frame::none().fill(pal.panel).inner_margin(egui::Margin::symmetric(8.0, 5.0)).show(ui, |ui| {
        ui.horizontal(|ui| {
            crate::ui::conn_badge(ui, app, &conn_id);
            if ui.button(RichText::new("\u{25B6} 运行 (Ctrl+Enter)").color(pal.contrast_text(pal.success)).background_color(pal.success)).clicked() { run = true; }
            if ui.button("\u{1F50E} Explain").clicked() { explain = true; }
            if ui.button("\u{2728} 美化 SQL").clicked() { beautify = true; }
            if ui.button("\u{1F4CC} 保存为片段").clicked() { save_snippet = true; }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if let TabKind::Query(q) = &app.tabs[idx].kind {
                    ui.label(RichText::new(&q.message).size(12.0).color(pal.text_dim));
                }
            });
        });
    });
    ui.separator();

    let avail_h = ui.available_height();
    let editor_h = (avail_h * 0.40).max(120.0);

    // Editor
    egui::Frame::none().fill(pal.panel).stroke(Stroke::new(1.0, pal.border)).show(ui, |ui| {
        if let TabKind::Query(q) = &mut app.tabs[idx].kind {
            let resp = ui.add_sized(
                [ui.available_width(), editor_h - 8.0],
                egui::TextEdit::multiline(&mut q.sql).code_editor().desired_width(f32::INFINITY).hint_text("-- 输入 SQL"),
            );
            if resp.has_focus() && ui.input(|i| i.modifiers.command && i.key_pressed(egui::Key::Enter)) {
                run = true;
            }
        }
    });
    ui.add_space(4.0);

    // Results
    let (results_len, msg) = match &app.tabs[idx].kind {
        TabKind::Query(q) => (q.results.len(), q.message.clone()),
        _ => (0, String::new()),
    };
    if results_len == 0 {
        ui.centered_and_justified(|ui| {
            ui.label(RichText::new("按 “运行” 或 Ctrl+Enter 执行 SQL，结果将显示在此处。").color(pal.text_dim));
        });
    } else {
        ui.horizontal(|ui| {
            if let TabKind::Query(q) = &app.tabs[idx].kind {
                for (i, r) in q.results.iter().enumerate() {
                    let label = if let Some(e) = &r.error {
                        format!("结果 {}  \u{2716} 错误", i + 1)
                    } else if r.is_select {
                        format!("结果 {}  ({} 行)", i + 1, r.rows.len())
                    } else {
                        format!("语句 {}  ({} 行)", i + 1, r.affected)
                    };
                    let sel = i == q.active_result;
                    if ui.selectable_label(sel, RichText::new(label).size(12.0)).clicked() {
                        active_result = Some(i);
                    }
                }
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(RichText::new(msg).size(12.0).color(pal.text_dim));
            });
        });
        ui.separator();
        egui::Frame::none().fill(pal.panel).show(ui, |ui| {
            if let TabKind::Query(q) = &app.tabs[idx].kind {
                let ar = q.active_result.min(q.results.len().saturating_sub(1));
                if let Some(r) = q.results.get(ar) {
                    render_result(ui, &pal, r);
                }
            }
        });
    }

    if run {
        app.run_query_at(idx, None);
        if let TabKind::Query(q) = &app.tabs[idx].kind {
            // remember in history
            let sql = q.sql.clone();
            if !sql.trim().is_empty() && !app.store.history.contains(&sql) {
                app.store.history.insert(0, sql);
                if app.store.history.len() > 50 { app.store.history.truncate(50); }
                app.store.save();
            }
        }
    }
    if explain {
        if let TabKind::Query(q) = &mut app.tabs[idx].kind {
            let first: String = q.sql.split(';').next().unwrap_or("").to_string();
            let sql = format!("EXPLAIN QUERY PLAN {}", first);
            let res = if let Some(eng) = app.engines.get(&q.conn_id) { eng.query(&sql) } else { Default::default() };
            q.results = vec![explain_tree(&res)];
            q.active_result = 0;
            q.message = "执行计划（可视化）".into();
        }
    }
    if beautify {
        if let TabKind::Query(q) = &mut app.tabs[idx].kind {
            q.sql = beautify_sql(&q.sql);
        }
    }
    if save_snippet {
        if let TabKind::Query(q) = &app.tabs[idx].kind {
            let sql = q.sql.clone();
            app.add_snippet(format!("片段 {}", app.store.snippets.len() + 1), sql);
            app.log("已保存为代码片段");
        }
    }
    if let Some(i) = active_result {
        if let TabKind::Query(q) = &mut app.tabs[idx].kind { q.active_result = i; }
    }
}

/// Turn `EXPLAIN QUERY PLAN` rows (id, parent, notused, detail) into an
/// indented single-column tree view.
fn explain_tree(res: &crate::db::QueryResult) -> crate::db::QueryResult {
    if res.error.is_some() || res.columns.is_empty() {
        return res.clone();
    }
    let find = |name: &str| res.columns.iter().position(|c| c.eq_ignore_ascii_case(name));
    let (id_i, par_i, det_i) = match (find("id"), find("parent"), find("detail")) {
        (Some(a), Some(b), Some(c)) => (a, b, c),
        _ => {
            // Not the expected shape; show detail column (last) as a flat list.
            let last = res.columns.len() - 1;
            let rows: Vec<Vec<Option<String>>> = res.rows.iter().map(|r| vec![r.get(last).cloned().flatten()]).collect();
            return crate::db::QueryResult { columns: vec![res.columns[last].clone()], rows, is_select: true, ..Default::default() };
        }
    };
    // Build children map: parent_id -> Vec<(id, detail)>
    use std::collections::HashMap;
    let mut children: HashMap<String, Vec<(String, String)>> = HashMap::new();
    let mut roots: Vec<(String, String)> = Vec::new();
    for r in &res.rows {
        let id = r.get(id_i).cloned().flatten().unwrap_or_default();
        let parent = r.get(par_i).cloned().flatten().unwrap_or_default();
        let detail = r.get(det_i).cloned().flatten().unwrap_or_default();
        if parent.is_empty() || parent == "0" {
            roots.push((id, detail));
        } else {
            children.entry(parent).or_default().push((id, detail));
        }
    }
    let mut out: Vec<Vec<Option<String>>> = Vec::new();
    fn walk(id: &str, detail: &str, depth: usize, children: &std::collections::HashMap<String, Vec<(String, String)>>, out: &mut Vec<Vec<Option<String>>>) {
        let prefix = if depth == 0 { String::new() } else { format!("{}|- ", "   ".repeat(depth - 1)) };
        out.push(vec![Some(format!("{}{}", prefix, detail))]);
        if let Some(kids) = children.get(id) {
            for (cid, cdet) in kids { walk(cid, cdet, depth + 1, children, out); }
        }
    }
    for (id, detail) in &roots { walk(id, detail, 0, &children, &mut out); }
    if out.is_empty() {
        // fallback flat
        for r in &res.rows { out.push(vec![r.get(det_i).cloned().flatten()]); }
    }
    crate::db::QueryResult { columns: vec!["执行计划".into()], rows: out, is_select: true, statement: res.statement.clone(), ..Default::default() }
}

pub fn render_result(ui: &mut egui::Ui, pal: &crate::theme::Palette, r: &crate::db::QueryResult) {
    if let Some(e) = &r.error {
        ui.colored_label(pal.danger, format!("\u{2716} {}", e));
        return;
    }
    if !r.is_select {
        ui.label(RichText::new(format!("\u{2714} 执行成功，{} 行受影响，用时 {:.1} ms", r.affected, r.elapsed_ms)).color(pal.success));
        return;
    }
    egui::ScrollArea::both().id_source(("res", r.statement.len())).auto_shrink([false, false]).show(ui, |ui| {
        egui::Grid::new("resgrid").striped(true).min_col_width(50.0).spacing([1.0, 1.0]).show(ui, |ui| {
            for c in &r.columns {
                ui.label(RichText::new(c).size(12.0).strong().color(pal.text));
            }
            ui.end_row();
            for row in r.rows.iter() {
                for v in row {
                    match v {
                        Some(s) => { ui.label(RichText::new(s).size(12.0)); }
                        None => { ui.label(RichText::new("NULL").size(12.0).color(pal.text_dim)); }
                    }
                }
                ui.end_row();
            }
        });
    });
    let _ = pal;
}

/// Non-destructive SQL pretty-printer: uppercases keywords *outside* string
/// literals / quoted identifiers and inserts line breaks before major clauses.
/// String literals and quoted identifiers are preserved verbatim.
pub fn beautify_sql(sql: &str) -> String {
    const KEYWORDS: &[&str] = &[
        "SELECT", "FROM", "WHERE", "GROUP", "BY", "ORDER", "HAVING", "LIMIT", "OFFSET",
        "JOIN", "LEFT", "RIGHT", "INNER", "OUTER", "ON", "AND", "OR", "NOT", "INSERT",
        "INTO", "VALUES", "UPDATE", "SET", "DELETE", "CREATE", "TABLE", "ALTER", "DROP",
        "AS", "DISTINCT", "UNION", "ALL", "CASE", "WHEN", "THEN", "ELSE", "END", "IS",
        "NULL", "IN", "LIKE", "BETWEEN", "ASC", "DESC", "COUNT", "SUM", "AVG", "MIN", "MAX",
    ];

    // Pass 1: uppercase keyword words outside quotes.
    let mut up = String::new();
    let mut chars = sql.chars().peekable();
    let (mut in_single, mut in_double) = (false, false);
    while let Some(c) = chars.next() {
        if in_single {
            up.push(c);
            if c == '\'' { in_single = false; }
            continue;
        }
        if in_double {
            up.push(c);
            if c == '"' { in_double = false; }
            continue;
        }
        match c {
            '\'' => { in_single = true; up.push(c); }
            '"' => { in_double = true; up.push(c); }
            _ if c.is_alphabetic() => {
                let mut w = String::from(c);
                while let Some(&n) = chars.peek() {
                    if n.is_alphanumeric() || n == '_' { w.push(n); chars.next(); } else { break; }
                }
                let upper = w.to_uppercase();
                if KEYWORDS.contains(&upper.as_str()) { up.push_str(&upper); } else { up.push_str(&w); }
            }
            _ => up.push(c),
        }
    }

    // Pass 2: line breaks before major clauses.
    for k in ["FROM", "WHERE", "GROUP BY", "ORDER BY", "HAVING", "LIMIT", "UNION", "LEFT JOIN", "RIGHT JOIN", "INNER JOIN", "JOIN"] {
        up = up.replace(&format!(" {} ", k), &format!("\n{} ", k));
    }
    up.trim().to_string()
}
