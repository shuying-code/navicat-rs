use crate::app::{ChartKind, NavicatApp, TabKind};
use crate::db;
use egui::{Color32, Pos2, RichText, Stroke, Vec2};

pub fn draw(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) {
    let is = match &app.tabs[idx].kind {
        TabKind::Transfer(_) => "transfer",
        TabKind::Export(_) => "export",
        TabKind::Import(_) => "import",
        TabKind::Backup(_) => "backup",
        TabKind::Monitor(_) => "monitor",
        TabKind::Profiling(_) => "profiling",
        TabKind::DataGen(_) => "datagen",
        TabKind::Chart(_) => "chart",
        TabKind::Dict(_) => "dict",
        TabKind::Report(_) => "report",
        TabKind::Automation(_) => "automation",
        TabKind::Users(_) => "users",
        TabKind::Ai(_) => "ai",
        TabKind::Snippet(_) => "snippet",
        TabKind::Masking(_) => "masking",
        TabKind::Sync(_) => "sync",
        TabKind::History(_) => "history",
        TabKind::Objects(_) => "objects",
        TabKind::Pump(_) => "pump",
        TabKind::Schema(_) => "schema",
        TabKind::Outline(_) => "outline",
        TabKind::Model(_) => "model",
        TabKind::Find(_) => "find",
        TabKind::Settings => "settings",
        _ => "",
    };
    match is {
        "transfer" => transfer(app, ui, idx),
        "export" => export(app, ui, idx),
        "import" => import(app, ui, idx),
        "backup" => backup(app, ui, idx),
        "monitor" => monitor(app, ui, idx),
        "profiling" => profiling(app, ui, idx),
        "datagen" => datagen(app, ui, idx),
        "chart" => chart(app, ui, idx),
        "dict" => dict(app, ui, idx),
        "report" => report(app, ui, idx),
        "automation" => automation(app, ui, idx),
        "users" => users(app, ui, idx),
        "ai" => ai(app, ui, idx),
        "snippet" => snippet(app, ui, idx),
        "masking" => masking(app, ui, idx),
        "sync" => sync(app, ui, idx),
        "history" => history(app, ui, idx),
        "objects" => objects(app, ui, idx),
        "pump" => pump(app, ui, idx),
        "schema" => schema(app, ui, idx),
        "outline" => outline(app, ui, idx),
        "model" => model(app, ui, idx),
        "find" => find(app, ui, idx),
        "settings" => settings(app, ui),
        _ => { ui.label("未知页面"); }
    }
}

fn header(app: &NavicatApp, ui: &mut egui::Ui, conn_id: &str, title: &str) {
    egui::Frame::none().fill(app.pal.panel).inner_margin(egui::Margin::symmetric(8.0, 5.0)).show(ui, |ui| {
        ui.horizontal(|ui| {
            if !conn_id.is_empty() { crate::ui::conn_badge(ui, app, conn_id); }
            ui.label(RichText::new(title).strong().size(14.0));
        });
    });
    ui.separator();
}

fn tables_of(app: &NavicatApp, conn_id: &str) -> Vec<String> {
    app.engines.get(conn_id).map(|e| e.objects().into_iter().filter(|o| o.kind == crate::model::ObjKind::Table).map(|o| o.name).collect()).unwrap_or_default()
}

// ---------------- Transfer ----------------
fn transfer(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) {
    let pal = app.pal.clone();
    let mut go = false;
    let conn_id = match &app.tabs[idx].kind { TabKind::Transfer(t) => t.src_conn.clone(), _ => return };
    header(app, ui, &conn_id, "\u{21C4} 数据传输");
    let conns: Vec<(String, String)> = app.store.connections.iter().map(|c| (c.id.clone(), c.name.clone())).collect();
    let tables = tables_of(app, &conn_id);
    // reload columns when the source table changed
    let need_cols = matches!(&app.tabs[idx].kind, TabKind::Transfer(t) if t.cols.is_empty() && !t.table.is_empty());
    if need_cols {
        let tbl = match &app.tabs[idx].kind { TabKind::Transfer(t) => t.table.clone(), _ => String::new() };
        let cols: Vec<(String, bool)> = app.engines.get(&conn_id).map(|e| e.columns(&tbl).into_iter().map(|c| (c.name, true)).collect()).unwrap_or_default();
        if let TabKind::Transfer(t) = &mut app.tabs[idx].kind { t.cols = cols; }
    }
    crate::ui::card(&app).show(ui, |ui| {
        let t = match &mut app.tabs[idx].kind { TabKind::Transfer(t) => t, _ => return };
        egui::Grid::new("trgrid").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| {
            ui.label("源表");
            egui::ComboBox::from_id_source("trsrc").selected_text(if t.table.is_empty() { "选择表".to_string() } else { t.table.clone() })
                .show_ui(ui, |ui| { for tb in &tables { if ui.selectable_label(t.table == *tb, tb).clicked() { t.table = tb.clone(); t.dst_table = tb.clone(); t.cols.clear(); } } });
            ui.end_row();
            ui.label("目标连接");
            egui::ComboBox::from_id_source("trdst").selected_text(conns.iter().find(|(id, _)| id == &t.dst_conn).map(|(_, n)| n.clone()).unwrap_or("选择".into()))
                .show_ui(ui, |ui| { for (id, name) in &conns { if ui.selectable_label(&t.dst_conn == id, name).clicked() { t.dst_conn = id.clone(); } } });
            ui.end_row();
            ui.label("目标表名");
            ui.add(egui::TextEdit::singleline(&mut t.dst_table).desired_width(240.0));
            ui.end_row();
            ui.label("过滤条件 (WHERE)");
            ui.add(egui::TextEdit::singleline(&mut t.where_clause).desired_width(300.0).hint_text("如 country = 'China'（可留空）"));
            ui.end_row();
        });
        ui.add_space(4.0);
        ui.label(RichText::new("字段映射（勾选要传输的列）").strong());
        ui.horizontal_wrapped(|ui| {
            for (cname, sel) in t.cols.iter_mut() {
                ui.checkbox(sel, cname.as_str());
            }
        });
        ui.add_space(8.0);
        if ui.button(RichText::new("开始传输").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { go = true; }
        if !t.status.is_empty() { ui.add_space(6.0); ui.colored_label(pal.success, &t.status); }
    });
    if go {
        let (src, dst, table, dst_table, cols, whr) = match &app.tabs[idx].kind { TabKind::Transfer(t) => (t.src_conn.clone(), t.dst_conn.clone(), t.table.clone(), t.dst_table.clone(), t.cols.clone(), t.where_clause.clone()), _ => return };
        app.ensure_engine(&dst);
        let selected: Vec<String> = cols.into_iter().filter(|(_, s)| *s).map(|(c, _)| c).collect();
        let res = do_transfer(app, &src, &dst, &table, &dst_table, &selected, &whr);
        if let TabKind::Transfer(t) = &mut app.tabs[idx].kind { t.status = res.clone(); }
        app.log(res);
    }
}

fn do_transfer(app: &mut NavicatApp, src: &str, dst: &str, table: &str, dst_table: &str, selected: &[String], where_clause: &str) -> String {
    let dst_table = if dst_table.trim().is_empty() { table } else { dst_table };
    let (create_sql, columns, rows) = {
        let se = match app.engines.get(src) { Some(e) => e, None => return "源未连接".into() };
        let create_sql: String = se.conn.query_row(
            "SELECT sql FROM sqlite_master WHERE type='table' AND name=?1", [table], |r| r.get(0)).unwrap_or_default();
        let all = se.columns(table);
        let columns: Vec<crate::model::Column> = if selected.is_empty() {
            all
        } else {
            all.into_iter().filter(|c| selected.iter().any(|s| s == &c.name)).collect()
        };
        let whr = if where_clause.trim().is_empty() { String::new() } else { format!(" WHERE {}", where_clause) };
        let colsel: Vec<String> = columns.iter().map(|c| db::quote_ident(&c.name)).collect();
        let q = se.query(&format!("SELECT {} FROM {}{}", colsel.join(", "), db::quote_ident(table), whr));
        (create_sql, columns, q.rows)
    };
    let de = match app.engines.get(dst) { Some(e) => e, None => return "目标未连接".into() };
    let _ = de.conn.execute(&format!("DROP TABLE IF EXISTS {}", db::quote_ident(dst_table)), []);
    // create table under the target name
    let create_sql = rename_create_table(&create_sql, dst_table);
    if !create_sql.is_empty() {
        if let Err(e) = de.conn.execute(&create_sql, []) { return format!("建表失败: {e}"); }
    }
    let colnames: Vec<String> = columns.iter().map(|c| c.name.clone()).collect();
    let mut n = 0;
    for row in &rows {
        if de.insert_row(dst_table, &colnames, row).is_ok() { n += 1; }
    }
    format!("传输完成: {} 行 · {} 列 -> {}", n, colnames.len(), dst_table)
}

fn rename_create_table(create_sql: &str, new_name: &str) -> String {
    if create_sql.is_empty() { return String::new(); }
    // Replace the first occurrence of CREATE TABLE <name> with the new name.
    if let Some(pos) = create_sql.to_uppercase().find("CREATE TABLE") {
        let after = &create_sql[pos + "CREATE TABLE".len()..];
        let trimmed = after.trim_start();
        // find end of table token (up to '(' )
        if let Some(paren) = trimmed.find('(') {
            let rest = &trimmed[paren..];
            let head = &create_sql[..pos];
            return format!("{}CREATE TABLE {}{}", head, db::quote_ident(new_name), rest);
        }
    }
    create_sql.to_string()
}

// ---------------- Export ----------------
fn export(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) {
    let pal = app.pal.clone();
    let mut go = false;
    let conn_id = match &app.tabs[idx].kind { TabKind::Export(e) => e.conn_id.clone(), _ => return };
    header(app, ui, &conn_id, "\u{2B06} 导出向导");
    let tables = tables_of(app, &conn_id);
    crate::ui::card(&app).show(ui, |ui| {
        let e = match &mut app.tabs[idx].kind { TabKind::Export(e) => e, _ => return };
        egui::Grid::new("exgrid").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| {
            ui.label("表");
            egui::ComboBox::from_id_source("extable").selected_text(if e.table.is_empty() { "选择表".to_string() } else { e.table.clone() })
                .show_ui(ui, |ui| { for t in &tables { if ui.selectable_label(e.table == *t, t).clicked() { e.table = t.clone(); } } });
            ui.end_row();
            ui.label("格式");
            egui::ComboBox::from_id_source("exfmt").selected_text(e.format.clone()).show_ui(ui, |ui| {
                for f in ["CSV", "JSON", "SQL", "HTML"] { if ui.selectable_label(e.format == f, f).clicked() { e.format = f.to_string(); } }
            });
            ui.end_row();
            ui.label("导出路径");
            ui.add(egui::TextEdit::singleline(&mut e.path).desired_width(420.0));
            ui.end_row();
        });
        ui.add_space(8.0);
        if ui.button(RichText::new("开始导出").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { go = true; }
        if !e.status.is_empty() { ui.add_space(6.0); ui.colored_label(pal.success, &e.status); }
    });
    if go {
        let (table, fmt, path) = match &app.tabs[idx].kind { TabKind::Export(e) => (e.table.clone(), e.format.clone(), e.path.clone()), _ => return };
        let status = do_export(app, &conn_id, &table, &fmt, &path);
        if let TabKind::Export(e) = &mut app.tabs[idx].kind { e.status = status.clone(); }
        app.log(status);
    }
}

fn do_export(app: &NavicatApp, conn_id: &str, table: &str, fmt: &str, path: &str) -> String {
    let e = match app.engines.get(conn_id) { Some(e) => e, None => return "未连接".into() };
    if table.is_empty() { return "请选择表".into(); }
    let r = e.query(&format!("SELECT * FROM {}", db::quote_ident(table)));
    let content = match fmt {
        "JSON" => {
            let arr: Vec<serde_json::Value> = r.rows.iter().map(|row| {
                let mut o = serde_json::Map::new();
                for (i, c) in r.columns.iter().enumerate() { o.insert(c.clone(), opt_json(row.get(i).cloned().flatten())); }
                serde_json::Value::Object(o)
            }).collect();
            serde_json::to_string_pretty(&arr).unwrap_or_default()
        }
        "SQL" => {
            let create: String = e.conn.query_row("SELECT sql FROM sqlite_master WHERE type='table' AND name=?1", [table], |r| r.get(0)).unwrap_or_default();
            let mut s = format!("{}\n\n", create);
            for row in &r.rows {
                let vals: Vec<String> = row.iter().map(|v| match v { Some(s) => format!("'{}'", s.replace('\'', "''")), None => "NULL".into() }).collect();
                s.push_str(&format!("INSERT INTO {} VALUES ({});\n", db::quote_ident(table), vals.join(", ")));
            }
            s
        }
        "HTML" => {
            let mut s = String::new();
            s.push_str("<!DOCTYPE html>\n<html><head><meta charset=\"utf-8\"><title>");
            s.push_str(&html_escape(table));
            s.push_str("</title><style>body{font-family:Segoe UI,Arial,sans-serif;margin:24px;}table{border-collapse:collapse;}th,td{border:1px solid #ccc;padding:6px 10px;font-size:13px;}th{background:#2e86c1;color:#fff;}tr:nth-child(even){background:#f4f6f8;}</style></head><body>");
            s.push_str(&format!("<h2>{}</h2>", html_escape(table)));
            s.push_str("<table><thead><tr>");
            for c in &r.columns { s.push_str(&format!("<th>{}</th>", html_escape(c))); }
            s.push_str("</tr></thead><tbody>");
            for row in &r.rows {
                s.push_str("<tr>");
                for v in row { s.push_str(&format!("<td>{}</td>", html_escape(v.as_deref().unwrap_or("")))); }
                s.push_str("</tr>");
            }
            s.push_str("</tbody></table></body></html>");
            s
        }
        _ => {
            let mut s = String::new();
            // UTF-8 BOM so Excel opens Chinese text correctly
            s.push('\u{FEFF}');
            s.push_str(&r.columns.iter().map(|c| csv_escape(c)).collect::<Vec<_>>().join(","));
            s.push('\n');
            for row in &r.rows {
                let line: Vec<String> = row.iter().map(|v| csv_escape(&v.clone().unwrap_or_default())).collect();
                s.push_str(&line.join(","));
                s.push('\n');
            }
            s
        }
    };
    match std::fs::write(path, content) {
        Ok(_) => format!("已导出 {} 行 -> {}", r.rows.len(), path),
        Err(e) => format!("写入失败: {e}"),
    }
}

fn csv_escape(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else { s.to_string() }
}
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}
fn opt_json(v: Option<String>) -> serde_json::Value {
    match v {
        Some(s) => {
            if let Ok(i) = s.parse::<i64>() { serde_json::Value::from(i) }
            else if let Ok(f) = s.parse::<f64>() { serde_json::Value::from(f) }
            else { serde_json::Value::from(s) }
        }
        None => serde_json::Value::Null,
    }
}

// ---------------- Import ----------------
fn import(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) {
    let pal = app.pal.clone();
    let mut go = false;
    let conn_id = match &app.tabs[idx].kind { TabKind::Import(i) => i.conn_id.clone(), _ => return };
    header(app, ui, &conn_id, "\u{2B07} 导入向导");
    let tables = tables_of(app, &conn_id);
    crate::ui::card(&app).show(ui, |ui| {
        let d = match &mut app.tabs[idx].kind { TabKind::Import(i) => i, _ => return };
        egui::Grid::new("imgrid").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| {
            ui.label("目标表");
            egui::ComboBox::from_id_source("imtable").selected_text(if d.table.is_empty() { "选择表(需已存在)".to_string() } else { d.table.clone() })
                .show_ui(ui, |ui| { for t in &tables { if ui.selectable_label(d.table == *t, t).clicked() { d.table = t.clone(); } } });
            ui.end_row();
            ui.label("格式");
            egui::ComboBox::from_id_source("imfmt").selected_text(d.format.clone()).show_ui(ui, |ui| {
                for f in ["CSV", "JSON", "SQL"] { if ui.selectable_label(d.format == f, f).clicked() { d.format = f.to_string(); } }
            });
            ui.end_row();
            ui.label("文件路径");
            ui.add(egui::TextEdit::singleline(&mut d.path).desired_width(420.0).hint_text("文件路径：CSV 首行为列名；JSON 为对象数组；SQL 为可执行脚本"));
            ui.end_row();
        });
        ui.add_space(8.0);
        if ui.button(RichText::new("开始导入").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { go = true; }
        if !d.status.is_empty() { ui.add_space(6.0); ui.colored_label(pal.success, &d.status); }
    });
    if go {
        let (table, path, fmt) = match &app.tabs[idx].kind { TabKind::Import(i) => (i.table.clone(), i.path.clone(), i.format.clone()), _ => return };
        let status = do_import(app, &conn_id, &table, &path, &fmt);
        if let TabKind::Import(i) = &mut app.tabs[idx].kind { i.status = status.clone(); }
        app.log(status);
    }
}

fn do_import(app: &NavicatApp, conn_id: &str, table: &str, path: &str, fmt: &str) -> String {
    let e = match app.engines.get(conn_id) { Some(e) => e, None => return "未连接".into() };
    let content = match std::fs::read_to_string(path) { Ok(c) => c, Err(err) => return format!("读取失败: {err}") };
    match fmt {
        "SQL" => {
            let results = e.run_script(&content);
            let errs = results.iter().filter(|r| r.error.is_some()).count();
            if let Some(first) = results.iter().find(|r| r.error.is_some()) {
                return format!("SQL 导入：{} 条语句，{} 条出错：{}", results.len(), errs, first.error.clone().unwrap_or_default());
            }
            format!("SQL 导入完成: {} 条语句", results.len())
        }
        "JSON" => {
            if table.is_empty() { return "请选择目标表".into(); }
            match import_json_text(e, table, &content) {
                Ok(n) => format!("JSON 导入完成: {} 行 -> {}", n, table),
                Err(err) => err,
            }
        }
        _ => {
            if table.is_empty() { return "请选择目标表".into(); }
            let mut lines = content.lines();
            let header: Vec<String> = match lines.next() { Some(h) => parse_csv_line(h).into_iter().map(|s| s.trim().to_string()).collect(), None => return "空文件".into() };
            let mut n = 0;
            for line in lines {
                if line.trim().is_empty() { continue; }
                let vals: Vec<Option<String>> = parse_csv_line(line).into_iter().map(|v| if v.is_empty() { None } else { Some(v) }).collect();
                if e.insert_row(table, &header, &vals).is_ok() { n += 1; }
            }
            format!("导入完成: {} 行 -> {}", n, table)
        }
    }
}

fn json_val_to_string(v: &serde_json::Value) -> Option<String> {
    match v {
        serde_json::Value::Null => None,
        serde_json::Value::String(s) => Some(s.clone()),
        other => Some(other.to_string()),
    }
}

/// Import a JSON array of objects into a table. Column set is derived from the
/// union of keys in the first object; missing keys become NULL.
pub fn import_json_text(e: &crate::db::SqliteEngine, table: &str, content: &str) -> Result<usize, String> {
    let arr: serde_json::Value = serde_json::from_str(content).map_err(|err| format!("JSON 解析失败: {err}"))?;
    let items = arr.as_array().ok_or_else(|| "JSON 顶层需为对象数组".to_string())?;
    if items.is_empty() { return Ok(0); }
    let mut cols: Vec<String> = Vec::new();
    for item in items {
        if let Some(obj) = item.as_object() {
            for k in obj.keys() { if !cols.contains(k) { cols.push(k.clone()); } }
        }
    }
    let mut n = 0;
    for item in items {
        if let Some(obj) = item.as_object() {
            let vals: Vec<Option<String>> = cols.iter().map(|c| obj.get(c).and_then(json_val_to_string)).collect();
            if e.insert_row(table, &cols, &vals).is_ok() { n += 1; }
        }
    }
    Ok(n)
}

fn parse_csv_line(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_q = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if in_q {
            if c == '"' {
                if chars.peek() == Some(&'"') { cur.push('"'); chars.next(); } else { in_q = false; }
            } else { cur.push(c); }
        } else if c == '"' { in_q = true; }
        else if c == ',' { out.push(cur.clone()); cur.clear(); }
        else { cur.push(c); }
    }
    out.push(cur);
    out
}

// ---------------- Backup ----------------
fn backup(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) {
    let pal = app.pal.clone();
    let mut do_backup = false;
    let mut do_restore = false;
    let conn_id = match &app.tabs[idx].kind { TabKind::Backup(b) => b.conn_id.clone(), _ => return };
    header(app, ui, &conn_id, "\u{1F4BE} 备份 / 还原");
    let dbfile = app.conn(&conn_id).map(|c| c.sqlite_path.clone()).unwrap_or_default();
    crate::ui::card(&app).show(ui, |ui| {
        let b = match &mut app.tabs[idx].kind { TabKind::Backup(b) => b, _ => return };
        ui.label(format!("数据库文件: {}", if dbfile.is_empty() { "内存库（示例，支持 VACUUM INTO 备份）" } else { &dbfile }));
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.label("备份/还原路径:");
            ui.add(egui::TextEdit::singleline(&mut b.path).desired_width(420.0));
        });
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            if ui.button(RichText::new("备份到文件").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { do_backup = true; }
            if ui.button("从文件还原").clicked() { do_restore = true; }
        });
        if !b.status.is_empty() { ui.add_space(6.0); ui.colored_label(pal.success, &b.status); }
    });
    let path = match &app.tabs[idx].kind { TabKind::Backup(b) => b.path.clone(), _ => return };
    if do_backup {
        // Works for both file and in-memory databases via VACUUM INTO (consistent snapshot).
        let status = if let Some(e) = app.engines.get(&conn_id) {
            match e.vacuum_into(&path) { Ok(_) => format!("已备份 -> {path}"), Err(err) => format!("备份失败: {err}") }
        } else { "未连接".into() };
        if let TabKind::Backup(b) = &mut app.tabs[idx].kind { b.status = status.clone(); }
        app.log(status);
    }
    if do_restore {
        let status = if dbfile.is_empty() {
            // In-memory: load backup snapshot and replay its SQL into the live memory engine.
            restore_into_memory(app, &conn_id, &path)
        } else {
            app.engines.remove(&conn_id);
            match std::fs::copy(&path, &dbfile) { Ok(_) => { app.ensure_engine(&conn_id); format!("已从 {} 还原", path) } Err(e) => format!("还原失败: {e}") }
        };
        if let TabKind::Backup(b) = &mut app.tabs[idx].kind { b.status = status.clone(); }
        app.log(status);
    }
}

fn restore_into_memory(app: &mut NavicatApp, conn_id: &str, path: &str) -> String {
    let src = match crate::db::SqliteEngine::open(path) { Ok(e) => e, Err(err) => return format!("打开备份失败: {err}") };
    let script = src.dump_sql(None);
    let e = match app.engines.get(conn_id) { Some(e) => e, None => return "未连接".into() };
    // Drop existing schema in the live memory DB so restore reflects the backup exactly.
    let mut drops = Vec::new();
    if let Ok(mut st) = e.conn.prepare("SELECT type, name FROM sqlite_master WHERE type IN ('view','table') AND name NOT LIKE 'sqlite_%'") {
        if let Ok(mut rows) = st.query([]) {
            while let Ok(Some(r)) = rows.next() {
                let ty: String = r.get(0).unwrap_or_default();
                let nm: String = r.get(1).unwrap_or_default();
                drops.push(format!("DROP {} IF EXISTS {}", if ty == "view" { "VIEW" } else { "TABLE" }, db::quote_ident(&nm)));
            }
        }
    }
    for d in drops { let _ = e.conn.execute(&d, []); }
    let results = e.run_script(&script);
    let errs = results.iter().filter(|r| r.error.is_some()).count();
    if errs == 0 { format!("已从 {} 还原（内存库，{} 条语句）", path, results.len()) } else { format!("还原完成，{} 条语句出错", errs) }
}

// ---------------- Monitor ----------------
fn monitor(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) {
    let conn_id = match &app.tabs[idx].kind { TabKind::Monitor(m) => m.conn_id.clone(), _ => return };
    header(app, ui, &conn_id, "\u{1F4C8} 服务器监控");
    let stats: Vec<(String, String)> = if let Some(e) = app.engines.get(&conn_id) {
        let mut v = vec![
            ("SQLite 版本".into(), e.sqlite_version()),
            ("页大小".into(), format!("{} 字节", e.pragma_i64("page_size"))),
            ("页数量".into(), e.pragma_i64("page_count").to_string()),
            ("空闲页".into(), e.pragma_i64("freelist_count").to_string()),
            ("数据库大小".into(), format!("{} KB", e.db_file_size() / 1024)),
            ("完整性检查".into(), e.integrity_check()),
            ("journal_mode".into(), e.pragma_str("journal_mode")),
            ("表数量".into(), e.objects().iter().filter(|o| o.kind == crate::model::ObjKind::Table).count().to_string()),
        ];
        v.push(("连接数".into(), "1 (本进程)".into()));
        v
    } else { vec![("状态".into(), "未连接".into())] };
    crate::ui::card(&app).show(ui, |ui| {
        egui::Grid::new("mongrid").num_columns(2).striped(true).spacing([16.0, 6.0]).show(ui, |ui| {
            for (k, v) in &stats { ui.label(RichText::new(k).strong()); ui.label(v); ui.end_row(); }
        });
        ui.add_space(8.0);
        if ui.button("\u{1F504} 刷新").clicked() { /* auto next frame */ }
    });
}

// ---------------- Profiling ----------------
fn profiling(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) {
    let pal = app.pal.clone();
    let mut go = false;
    let conn_id = match &app.tabs[idx].kind { TabKind::Profiling(p) => p.conn_id.clone(), _ => return };
    header(app, ui, &conn_id, "\u{1F50E} 数据剖析");
    let tables = tables_of(app, &conn_id);
    crate::ui::card(&app).show(ui, |ui| {
        let d = match &mut app.tabs[idx].kind { TabKind::Profiling(p) => p, _ => return };
        ui.horizontal(|ui| {
            ui.label("表:");
            egui::ComboBox::from_id_source("prtable").selected_text(if d.table.is_empty() { "选择表".to_string() } else { d.table.clone() })
                .show_ui(ui, |ui| { for t in &tables { if ui.selectable_label(d.table == *t, t).clicked() { d.table = t.clone(); } } });
            if ui.button(RichText::new("计算统计").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { go = true; }
        });
        if !d.stats.is_empty() {
            ui.add_space(8.0);
            egui::ScrollArea::vertical().max_height(400.0).show(ui, |ui| {
                egui::Grid::new("prgrid").num_columns(2).striped(true).spacing([16.0, 4.0]).show(ui, |ui| {
                    for (k, v) in &d.stats { ui.label(RichText::new(k).strong().size(12.0)); ui.label(RichText::new(v).size(12.0)); ui.end_row(); }
                });
            });
        }
    });
    if go {
        let table = match &app.tabs[idx].kind { TabKind::Profiling(p) => p.table.clone(), _ => return };
        let mut stats = Vec::new();
        if let Some(e) = app.engines.get(&conn_id) {
            for c in e.columns(&table) {
                let cn = db::quote_ident(&c.name);
                let total = e.count(&table);
                let nulls: i64 = e.conn.query_row(&format!("SELECT COUNT(*) FROM {} WHERE {} IS NULL", db::quote_ident(&table), cn), [], |r| r.get(0)).unwrap_or(0);
                let distinct: i64 = e.conn.query_row(&format!("SELECT COUNT(DISTINCT {}) FROM {}", cn, db::quote_ident(&table)), [], |r| r.get(0)).unwrap_or(0);
                stats.push((format!("{} 行数", c.name), total.to_string()));
                stats.push((format!("{} 空值/唯一值", c.name), format!("{} / {}", nulls, distinct)));
            }
            // sample distinct values for first text column
            let top = e.query(&format!("SELECT * FROM {} LIMIT 3", db::quote_ident(&table)));
            stats.push(("样本".into(), format!("{} 行预览", top.rows.len())));
        }
        if let TabKind::Profiling(p) = &mut app.tabs[idx].kind { p.stats = stats; p.computed = true; }
        app.log("数据剖析完成");
    }
}

// ---------------- DataGen ----------------
fn datagen(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) {
    let pal = app.pal.clone();
    let mut go = false;
    let conn_id = match &app.tabs[idx].kind { TabKind::DataGen(g) => g.conn_id.clone(), _ => return };
    header(app, ui, &conn_id, "\u{1F9EA} 数据生成");
    crate::ui::card(&app).show(ui, |ui| {
        let d = match &mut app.tabs[idx].kind { TabKind::DataGen(g) => g, _ => return };
        ui.horizontal(|ui| {
            ui.label("目标表:");
            ui.add(egui::TextEdit::singleline(&mut d.table).desired_width(200.0));
            ui.label("生成行数:");
            let mut c = d.count as i64;
            if ui.add(egui::DragValue::new(&mut c).range(1..=100000)).changed() { d.count = c as usize; }
            if ui.button(RichText::new("生成数据").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { go = true; }
        });
        ui.add_space(6.0);
        ui.label(RichText::new("根据列名智能生成: name/email/city/country/phone/date/price/age 等；其余列生成随机值。").size(11.0).color(pal.text_dim));
        if !d.status.is_empty() { ui.add_space(6.0); ui.colored_label(pal.success, &d.status); }
    });
    if go {
        let (table, count) = match &app.tabs[idx].kind { TabKind::DataGen(g) => (g.table.clone(), g.count), _ => return };
        let status = do_datagen(app, &conn_id, &table, count);
        if let TabKind::DataGen(g) = &mut app.tabs[idx].kind { g.status = status.clone(); }
        app.log(status);
    }
}

fn do_datagen(app: &NavicatApp, conn_id: &str, table: &str, count: usize) -> String {
    let e = match app.engines.get(conn_id) { Some(e) => e, None => return "未连接".into() };
    let cols = e.columns(table);
    if cols.is_empty() { return "表不存在或无列".into(); }
    let first = ["Alice", "Bob", "Carol", "David", "Eve", "Frank", "Grace", "Henry", "Ivy", "Jack"];
    let last = ["Zhang", "Li", "Wang", "Chen", "Liu", "Wu", "Xu", "Sun"];
    let cities = ["Beijing", "Shanghai", "Shenzhen", "Hangzhou", "Chengdu", "Guangzhou"];
    let mut n = 0;
    for i in 0..count {
        let mut names = Vec::new();
        let mut vals = Vec::new();
        for c in &cols {
            let ln = c.name.to_lowercase();
            let v: Option<String> = if c.pk { None } else if ln.contains("mail") { Some(format!("user{}@example.com", rand::random::<u32>() % 100000)) }
            else if ln.contains("name") { Some(format!("{} {}", first[(i + 3) % first.len()], last[i % last.len()])) }
            else if ln.contains("city") { Some(cities[i % cities.len()].into()) }
            else if ln.contains("country") { Some(cities[i % cities.len()].into()) }
            else if ln.contains("phone") { Some(format!("138{:08}", rand::random::<u32>() % 100000000)) }
            else if ln.contains("date") || ln.contains("time") { Some(chrono::Local::now().format("%Y-%m-%d").to_string()) }
            else if ln.contains("price") || ln.contains("salary") || ln.contains("total") { Some(format!("{:.2}", (rand::random::<u32>() % 9000) as f64 / 10.0)) }
            else if ln.contains("age") || ln.contains("qty") || ln.contains("quantity") || ln.contains("stock") { Some((18 + (i % 40)).to_string()) }
            else { Some(format!("val_{}_{}", i, c.name)) };
            // skip autoincrement integer pk
            let skip = c.pk && (c.decl_type.to_uppercase().contains("INT"));
            if skip { continue; }
            names.push(c.name.clone());
            vals.push(v);
        }
        if e.insert_row(table, &names, &vals).is_ok() { n += 1; }
    }
    format!("已生成 {} 行 -> {}", n, table)
}

// ---------------- Chart ----------------
fn chart(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) {
    let pal = app.pal.clone();
    let mut go = false;
    let conn_id = match &app.tabs[idx].kind { TabKind::Chart(c) => c.conn_id.clone(), _ => return };
    header(app, ui, &conn_id, "\u{1F4CA} 图表 / 仪表盘");
    crate::ui::card(&app).show(ui, |ui| {
        let d = match &mut app.tabs[idx].kind { TabKind::Chart(c) => c, _ => return };
        ui.horizontal(|ui| {
            egui::ComboBox::from_id_source("chartkind").selected_text(match d.kind { ChartKind::Bar => "柱状图", ChartKind::Line => "折线图", ChartKind::Pie => "饼图" })
                .show_ui(ui, |ui| {
                    if ui.selectable_label(matches!(d.kind, ChartKind::Bar), "柱状图").clicked() { d.kind = ChartKind::Bar; }
                    if ui.selectable_label(matches!(d.kind, ChartKind::Line), "折线图").clicked() { d.kind = ChartKind::Line; }
                    if ui.selectable_label(matches!(d.kind, ChartKind::Pie), "饼图").clicked() { d.kind = ChartKind::Pie; }
                });
            if ui.button(RichText::new("\u{25B6} 生成图表").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { go = true; }
        });
        ui.add(egui::TextEdit::multiline(&mut d.sql).code_editor().desired_rows(2).desired_width(f32::INFINITY));
        if !d.status.is_empty() { ui.colored_label(pal.text_dim, &d.status); }
    });
    if go {
        let sql = match &app.tabs[idx].kind { TabKind::Chart(c) => c.sql.clone(), _ => return };
        let res = if let Some(e) = app.engines.get(&conn_id) { e.query(&sql) } else { Default::default() };
        if let TabKind::Chart(c) = &mut app.tabs[idx].kind {
            c.status = format!("{} 行", res.rows.len());
            c.data = res;
        }
        app.log("图表已生成");
    }
    // draw chart
    let (kind, data, lc, vc) = match &app.tabs[idx].kind { TabKind::Chart(c) => (c.kind, c.data.clone(), c.label_col, c.value_col), _ => return };
    if data.columns.is_empty() {
        ui.centered_and_justified(|ui| { ui.label(RichText::new("输入聚合 SQL 并点击“生成图表”").color(pal.text_dim)); });
        return;
    }
    let avail = ui.available_rect_before_wrap();
    let painter = ui.painter_at(avail);
    painter.rect_filled(avail, 0.0, pal.panel);
    if data.rows.is_empty() { return; }
    let vals: Vec<(String, f64)> = data.rows.iter().map(|r| {
        let label = r.get(lc).cloned().flatten().unwrap_or_default();
        let v = r.get(vc).cloned().flatten().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
        (label, v)
    }).collect();
    let maxv = vals.iter().map(|(_, v)| *v).fold(0.0f64, f64::max).max(1.0);
    let colors = [Color32::from_rgb(0x22,0x74,0xE8), Color32::from_rgb(0x1E,0x9E,0x5A), Color32::from_rgb(0xE0,0x8A,0x1E), Color32::from_rgb(0x8E,0x44,0xAD), Color32::from_rgb(0xC7,0x46,0x34), Color32::from_rgb(0x29,0xB5,0xE8)];
    match kind {
        ChartKind::Bar => {
            let n = vals.len();
            let bw = (avail.width() - 80.0) / n as f32 * 0.7;
            let gap = (avail.width() - 80.0) / n as f32;
            let base = avail.bottom() - 40.0;
            let hmax = avail.height() - 80.0;
            for (i, (label, v)) in vals.iter().enumerate() {
                let x = avail.left() + 50.0 + i as f32 * gap;
                let h = (v / maxv) as f32 * hmax;
                let rect = egui::Rect::from_min_size(Pos2::new(x, base - h), Vec2::new(bw, h));
                painter.rect_filled(rect, 3.0, colors[i % colors.len()]);
                painter.text(Pos2::new(x + bw / 2.0, base + 12.0), egui::Align2::CENTER_CENTER, label, egui::FontId::proportional(11.0), pal.text_dim);
                painter.text(Pos2::new(x + bw / 2.0, base - h - 10.0), egui::Align2::CENTER_CENTER, format!("{:.0}", v), egui::FontId::proportional(10.0), pal.text);
            }
            painter.line_segment([Pos2::new(avail.left()+45.0, base), Pos2::new(avail.right()-10.0, base)], Stroke::new(1.0, pal.border));
        }
        ChartKind::Line => {
            let n = vals.len();
            let w = avail.width() - 80.0;
            let h = avail.height() - 80.0;
            let base = avail.bottom() - 40.0;
            let step = if n > 1 { w / (n - 1) as f32 } else { w };
            let mut pts = Vec::new();
            for (i, (_, v)) in vals.iter().enumerate() {
                let x = avail.left() + 50.0 + i as f32 * step;
                let y = base - (v / maxv) as f32 * h;
                pts.push(Pos2::new(x, y));
            }
            for w2 in pts.windows(2) { painter.line_segment([w2[0], w2[1]], Stroke::new(2.0, pal.accent)); }
            for (i, p) in pts.iter().enumerate() { painter.circle_filled(*p, 3.5, colors[i % colors.len()]); }
        }
        ChartKind::Pie => {
            let total: f64 = vals.iter().map(|(_, v)| *v).sum::<f64>().max(1.0);
            let center = avail.center();
            let radius = (avail.height().min(avail.width()) * 0.4).min(200.0);
            let mut start = 0.0f64;
            for (i, (label, v)) in vals.iter().enumerate() {
                let frac = v / total;
                let end = start + frac * std::f64::consts::TAU;
                draw_wedge(&painter, center, radius, start, end, colors[i % colors.len()]);
                let mid = (start + end) / 2.0;
                let lp = center + Vec2::new((radius * 0.65 * mid.cos() as f32), (radius * 0.65 * mid.sin() as f32));
                painter.text(lp, egui::Align2::CENTER_CENTER, format!("{} {:.0}%", label, frac * 100.0), egui::FontId::proportional(10.0), Color32::WHITE);
                start = end;
            }
        }
    }
}

fn draw_wedge(p: &egui::Painter, c: Pos2, r: f32, a0: f64, a1: f64, color: Color32) {
    let steps = 24;
    let mut pts = vec![c];
    for i in 0..=steps {
        let a = a0 + (a1 - a0) * i as f64 / steps as f64;
        pts.push(c + Vec2::new((r * a.cos() as f32), (r * a.sin() as f32)));
    }
    p.add(egui::Shape::convex_polygon(pts, color, Stroke::NONE));
}

// ---------------- Dict ----------------
fn dict(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) {
    let pal = app.pal.clone();
    let conn_id = match &app.tabs[idx].kind { TabKind::Dict(d) => d.conn_id.clone(), _ => return };
    header(app, ui, &conn_id, "\u{1F4D6} 数据字典");
    let tables = tables_of(app, &conn_id);
    let sel = match &app.tabs[idx].kind { TabKind::Dict(d) => d.table.clone(), _ => String::new() };
    let mut newsel: Option<String> = None;
    crate::ui::card(&app).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label("表:");
            egui::ComboBox::from_id_source("dicttable").selected_text(if sel.is_empty() { "全部".to_string() } else { sel.clone() })
                .show_ui(ui, |ui| {
                    if ui.selectable_label(sel.is_empty(), "全部表").clicked() { newsel = Some(String::new()); }
                    for t in &tables { if ui.selectable_label(sel == *t, t).clicked() { newsel = Some(t.clone()); } }
                });
        });
        ui.add_space(6.0);
        if let Some(e) = app.engines.get(&conn_id) {
            let list: Vec<String> = if sel.is_empty() { tables.clone() } else { vec![sel.clone()] };
            egui::ScrollArea::vertical().max_height(520.0).show(ui, |ui| {
                for t in list {
                    ui.label(RichText::new(format!("\u{1F5C2} {}  ({} 行)", t, e.count(&t))).strong().color(pal.accent));
                    egui::Grid::new(format!("dg{}", t)).num_columns(4).striped(true).spacing([14.0, 3.0]).show(ui, |ui| {
                        for h in ["列名", "类型", "主键", "非空"] { ui.label(RichText::new(h).size(11.0).color(pal.text_dim)); }
                        ui.end_row();
                        for c in e.columns(&t) {
                            ui.label(RichText::new(&c.name).size(12.0));
                            ui.label(RichText::new(&c.decl_type).size(12.0).monospace());
                            ui.label(if c.pk { "\u{2714}" } else { "" });
                            ui.label(if c.notnull { "\u{2714}" } else { "" });
                            ui.end_row();
                        }
                    });
                    ui.add_space(8.0);
                }
            });
        }
    });
    if let Some(ns) = newsel {
        if let TabKind::Dict(d) = &mut app.tabs[idx].kind { d.table = ns; }
    }
}

// ---------------- Report ----------------
fn report(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) {
    let pal = app.pal.clone();
    let mut go = false;
    let mut export = false;
    let conn_id = match &app.tabs[idx].kind { TabKind::Report(r) => r.conn_id.clone(), _ => return };
    header(app, ui, &conn_id, "\u{1F4CB} 报表");
    crate::ui::card(&app).show(ui, |ui| {
        let d = match &mut app.tabs[idx].kind { TabKind::Report(r) => r, _ => return };
        ui.horizontal(|ui| {
            ui.label("标题:");
            ui.add(egui::TextEdit::singleline(&mut d.title).desired_width(300.0));
            if ui.button(RichText::new("生成报表").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { go = true; }
            if ui.button("导出 HTML").clicked() { export = true; }
        });
        ui.add(egui::TextEdit::multiline(&mut d.sql).code_editor().desired_rows(2).desired_width(f32::INFINITY));
        if !d.status.is_empty() { ui.colored_label(pal.text_dim, &d.status); }
    });
    if go {
        let sql = match &app.tabs[idx].kind { TabKind::Report(r) => r.sql.clone(), _ => return };
        let res = if let Some(e) = app.engines.get(&conn_id) { e.query(&sql) } else { Default::default() };
        if let TabKind::Report(r) = &mut app.tabs[idx].kind { r.status = format!("{} 行", res.rows.len()); }
        // store result in a temp via ChartDoc? Instead render inline: reuse preview by storing into status only.
        // Render below using a fresh query each frame is expensive; store into dict? We'll just render now.
        render_report(ui, &app.pal, &res);
        app.log("报表已生成");
        return;
    }
    // initial render attempt
    let sql = match &app.tabs[idx].kind { TabKind::Report(r) => r.sql.clone(), _ => return };
    if let Some(e) = app.engines.get(&conn_id) {
        let res = e.query(&sql);
        render_report(ui, &pal, &res);
    }
    if export {
        let title = match &app.tabs[idx].kind { TabKind::Report(r) => r.title.clone(), _ => "report".into() };
        let path = crate::app::default_export_dir().join(format!("{}.html", sanitize(&title)));
        if let Some(e) = app.engines.get(&conn_id) {
            let res = e.query(&sql);
            let html = report_html(&title, &res);
            let _ = std::fs::write(&path, html);
            app.log(format!("报表已导出: {}", path.to_string_lossy()));
        }
    }
}

fn sanitize(s: &str) -> String { s.chars().map(|c| if c.is_alphanumeric() { c } else { '_' }).collect() }

fn render_report(ui: &mut egui::Ui, pal: &crate::theme::Palette, res: &crate::db::QueryResult) {
    ui.separator();
    egui::ScrollArea::both().max_height(420.0).show(ui, |ui| { crate::ui::query::render_result(ui, pal, res); });
}

fn report_html(title: &str, res: &crate::db::QueryResult) -> String {
    let mut h = format!("<!doctype html><meta charset='utf-8'><title>{}</title><h2>{}</h2><table border=1 cellspacing=0 cellpadding=6><tr>", title, title);
    for c in &res.columns { h.push_str(&format!("<th>{}</th>", c)); }
    h.push_str("</tr>");
    for row in &res.rows {
        h.push_str("<tr>");
        for v in row { h.push_str(&format!("<td>{}</td>", v.clone().unwrap_or_default())); }
        h.push_str("</tr>");
    }
    h.push_str("</table>");
    h
}

// ---------------- Automation ----------------
fn automation(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) {
    let pal = app.pal.clone();
    let conn_id = match &app.tabs[idx].kind { TabKind::Automation(a) => a.conn_id.clone(), _ => return };
    header(app, ui, &conn_id, "\u{2699} 自动化 / 计划任务");
    let mut add = false;
    crate::ui::card(&app).show(ui, |ui| {
        let d = match &mut app.tabs[idx].kind { TabKind::Automation(a) => a, _ => return };
        ui.label("计划任务（本地执行，无云端依赖）:");
        ui.add_space(6.0);
        for t in &d.tasks { ui.label(RichText::new(format!("\u{23F0} {}", t)).size(13.0)); }
        ui.add_space(8.0);
        if ui.button(RichText::new("+ 新建计划任务").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { add = true; }
    });
    if add { if let TabKind::Automation(a) = &mut app.tabs[idx].kind { a.tasks.push(format!("备份任务 {}", a.tasks.len() + 1)); } }
}

// ---------------- Users ----------------
fn users(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) {
    let pal = app.pal.clone();
    let conn_id = match &app.tabs[idx].kind { TabKind::Users(u) => u.conn_id.clone(), _ => return };
    header(app, ui, &conn_id, "\u{1F465} 用户与权限");
    crate::ui::card(&app).show(ui, |ui| {
        ui.horizontal(|ui| {
            crate::ui::mini_badge(ui, "全部权限已开放", pal.success);
            ui.label(RichText::new("本应用不设任何用户、角色或权限限制。").color(pal.text_dim).size(12.0));
        });
        ui.add_space(8.0);
        egui::Grid::new("usersgrid").num_columns(4).striped(true).spacing([18.0, 5.0]).show(ui, |ui| {
            for h in ["用户", "主机", "角色", "权限"] { ui.label(RichText::new(h).strong().size(12.0)); }
            ui.end_row();
            for (u, hst, role) in [("root", "%", "DBA"), ("app", "10.0.%", "读写"), ("report", "%", "只读"), ("guest", "localhost", "受限")] {
                ui.label(u); ui.label(hst); ui.label(role); ui.label(RichText::new("全部").color(pal.success));
                ui.end_row();
            }
        });
        ui.add_space(8.0);
        ui.label(RichText::new("提示: SQLite 无内置用户/权限系统，此处为界面展示；所有对象的增删改查均无限制。").size(11.0).color(pal.text_dim));
    });
}

// ---------------- AI ----------------
fn ai(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) {
    let pal = app.pal.clone();
    let mut ask = false;
    let conn_id = match &app.tabs[idx].kind { TabKind::Ai(a) => a.conn_id.clone(), _ => return };
    header(app, ui, &conn_id, "\u{2728} AI 助手（本地离线）");
    crate::ui::card(&app).show(ui, |ui| {
        let d = match &mut app.tabs[idx].kind { TabKind::Ai(a) => a, _ => return };
        ui.horizontal(|ui| {
            if ui.button("解释 SQL").clicked() { d.prompt = format!("解释: {}", first_select(&d.prompt)); ask = true; }
            if ui.button("生成查询").clicked() { d.prompt = "生成查询: 按国家统计客户数量".into(); ask = true; }
            if ui.button("优化建议").clicked() { d.prompt = "优化: SELECT * FROM customers WHERE name LIKE '%a%'".into(); ask = true; }
        });
        ui.add_space(6.0);
        ui.add(egui::TextEdit::multiline(&mut d.prompt).desired_rows(3).desired_width(f32::INFINITY).hint_text("向 AI 提问，例如：生成查询，按国家统计客户数量"));
        if ui.button(RichText::new("提问").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { ask = true; }
        if !d.answer.is_empty() {
            ui.add_space(8.0);
            egui::Frame::none().fill(pal.bg).rounding(egui::Rounding::same(6.0)).inner_margin(egui::Margin::same(10.0)).show(ui, |ui| {
                ui.label(RichText::new(&d.answer).monospace().size(12.5));
            });
        }
    });
    if ask {
        let (prompt, schema) = match &app.tabs[idx].kind {
            TabKind::Ai(a) => (a.prompt.clone(), app.engines.get(&conn_id).map(|e| {
                e.objects().into_iter().filter(|o| o.kind == crate::model::ObjKind::Table).map(|o| {
                    format!("{}({})", o.name, e.columns(&o.name).iter().map(|c| c.name.clone()).collect::<Vec<_>>().join(", "))
                }).collect::<Vec<_>>().join("; ")
            }).unwrap_or_default()),
            _ => (String::new(), String::new()),
        };
        let ans = local_ai(&prompt, &schema);
        if let TabKind::Ai(a) = &mut app.tabs[idx].kind { a.answer = ans; }
    }
}

fn first_select(s: &str) -> String {
    let idx = s.to_uppercase().find("SELECT").unwrap_or(0);
    s[idx..].to_string()
}

fn local_ai(prompt: &str, schema: &str) -> String {
    let p = prompt.to_lowercase();
    let mut out = String::new();
    if p.contains("生成") || p.contains("查询") && (p.contains("统计") || p.contains("count")) {
        out.push_str("建议 SQL:\n");
        if p.contains("国家") || p.contains("country") {
            out.push_str("SELECT country, COUNT(*) AS cnt\nFROM customers\nGROUP BY country\nORDER BY cnt DESC;\n");
        } else if p.contains("销售") || p.contains("sales") {
            out.push_str("SELECT strftime('%Y-%m', order_date) AS month, SUM(total) AS total\nFROM orders\nGROUP BY month\nORDER BY month;\n");
        } else {
            out.push_str("SELECT * FROM customers LIMIT 100;\n");
        }
    } else if p.contains("解释") || p.contains("explain") {
        out.push_str("该语句解析:\n• SELECT 指定要返回的列\n• FROM 指定数据来源表/视图\n• WHERE 过滤行（可在过滤列上建索引加速）\n• GROUP BY 聚合分组\n• ORDER BY 排序\n• LIMIT 限制返回行数\n");
    } else if p.contains("优化") || p.contains("optimize") {
        out.push_str("优化建议:\n1) 避免 SELECT *，只取需要的列\n2) 对 WHERE/ORDER BY 高频列建立索引\n3) LIKE '%x%' 无法用索引，考虑前缀匹配或全文索引\n4) 大数据量分页使用键集分页 (WHERE id > ?)\n5) 用 EXPLAIN QUERY PLAN 验证执行计划\n");
    } else {
        out.push_str("可用能力: 解释 SQL / 生成查询 / 优化建议。\n当前库对象:\n");
        out.push_str(if schema.is_empty() { "(未连接)" } else { schema });
    }
    out
}

// ---------------- Snippet ----------------
fn snippet(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) {
    let pal = app.pal.clone();
    let mut save = false;
    let mut del: Option<usize> = None;
    let mut insert = false;
    header(app, ui, "", "\u{1F4CC} 代码片段");
    ui.columns(2, |cols| {
        egui::Frame::none().fill(pal.panel).stroke(Stroke::new(1.0, pal.border)).inner_margin(egui::Margin::same(10.0)).show(&mut cols[0], |ui| {
            ui.label(RichText::new("片段列表").strong());
            egui::ScrollArea::vertical().id_source("snipscroll").max_height(400.0).show(ui, |ui| {
                let snips: Vec<(String, String)> = app.store.snippets.iter().map(|s| (s.name.clone(), s.sql.clone())).collect();
                let mut pick: Option<usize> = None;
                for (i, (name, _)) in snips.iter().enumerate() {
                    let sel = matches!(&app.tabs[idx].kind, TabKind::Snippet(s) if s.selected == Some(i));
                    if ui.selectable_label(sel, name).clicked() { pick = Some(i); }
                }
                if let Some(i) = pick {
                    if let TabKind::Snippet(s) = &mut app.tabs[idx].kind {
                        s.selected = Some(i);
                        s.edit_name = snips[i].0.clone();
                        s.edit_sql = snips[i].1.clone();
                    }
                }
            });
        });
        egui::Frame::none().fill(pal.panel).stroke(Stroke::new(1.0, pal.border)).inner_margin(egui::Margin::same(10.0)).show(&mut cols[1], |ui| {
            let s = match &mut app.tabs[idx].kind { TabKind::Snippet(s) => s, _ => return };
            ui.horizontal(|ui| { ui.label("名称:"); ui.add(egui::TextEdit::singleline(&mut s.edit_name).desired_width(240.0)); });
            ui.add_space(4.0);
            ui.add(egui::TextEdit::multiline(&mut s.edit_sql).code_editor().desired_rows(10).desired_width(f32::INFINITY));
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                if ui.button(if s.selected.is_some() { "更新" } else { "保存为新片段" }).clicked() { save = true; }
                if s.selected.is_some() && ui.button("删除").clicked() { del = s.selected; }
                if ui.button("插入到查询编辑器").clicked() { insert = true; }
            });
        });
    });
    if save {
        if let TabKind::Snippet(s) = &app.tabs[idx].kind {
            let name = s.edit_name.clone(); let sql = s.edit_sql.clone(); let sel = s.selected;
            if let Some(i) = sel { if i < app.store.snippets.len() { app.store.snippets[i] = crate::store::Snippet { name, sql }; } }
            else { app.store.snippets.push(crate::store::Snippet { name, sql }); }
            app.store.save();
            app.log("片段已保存");
        }
    }
    if let Some(i) = del { if i < app.store.snippets.len() { app.store.snippets.remove(i); app.store.save(); } if let TabKind::Snippet(s) = &mut app.tabs[idx].kind { s.selected = None; s.edit_name.clear(); s.edit_sql.clear(); } }
    if insert {
        let sql = match &app.tabs[idx].kind { TabKind::Snippet(s) => s.edit_sql.clone(), _ => String::new() };
        if let Some(id) = app.store.connections.first().map(|c| c.id.clone()) {
            app.open_connection(id.clone());
            app.new_query(id);
            let ni = app.tabs.len() - 1;
            if let TabKind::Query(q) = &mut app.tabs[ni].kind { q.sql = sql; }
        }
    }
}

// ---------------- Data Masking ----------------
const MASK_RULES: &[&str] = &["全遮蔽 (***)", "姓名保留首字", "邮箱掩码", "手机掩码", "随机数字"];

fn masking(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) {
    let pal = app.pal.clone();
    let mut go = false;
    let conn_id = match &app.tabs[idx].kind { TabKind::Masking(m) => m.conn_id.clone(), _ => return };
    header(app, ui, &conn_id, "\u{25D0} 数据脱敏");
    let tables = tables_of(app, &conn_id);
    let cur_table = match &app.tabs[idx].kind { TabKind::Masking(m) => m.table.clone(), _ => String::new() };
    let cols: Vec<String> = app
        .engines
        .get(&conn_id)
        .map(|e| e.columns(&cur_table).into_iter().filter(|c| !c.pk).map(|c| c.name).collect())
        .unwrap_or_default();
    crate::ui::card(&app).show(ui, |ui| {
        let m = match &mut app.tabs[idx].kind { TabKind::Masking(m) => m, _ => return };
        egui::Grid::new("mkgrid").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| {
            ui.label("表");
            egui::ComboBox::from_id_source("mktable").selected_text(if m.table.is_empty() { "选择表".to_string() } else { m.table.clone() })
                .show_ui(ui, |ui| { for t in &tables { if ui.selectable_label(m.table == *t, t).clicked() { m.table = t.clone(); m.column.clear(); } } });
            ui.end_row();
            ui.label("列");
            egui::ComboBox::from_id_source("mkcol").selected_text(if m.column.is_empty() { "选择列".to_string() } else { m.column.clone() })
                .show_ui(ui, |ui| { for c in &cols { if ui.selectable_label(m.column == *c, c).clicked() { m.column = c.clone(); } } });
            ui.end_row();
            ui.label("脱敏规则");
            egui::ComboBox::from_id_source("mkrule").selected_text(m.rule.clone())
                .show_ui(ui, |ui| { for r in MASK_RULES { if ui.selectable_label(m.rule == *r, *r).clicked() { m.rule = r.to_string(); } } });
            ui.end_row();
        });
        ui.add_space(8.0);
        ui.label(RichText::new("对所选列就地脱敏（直接更新数据，无法撤销，请先备份）。").size(11.0).color(pal.text_dim));
        if ui.button(RichText::new("执行脱敏").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { go = true; }
        if !m.status.is_empty() { ui.add_space(6.0); ui.colored_label(pal.success, &m.status); }
    });
    if go {
        let (table, column, rule) = match &app.tabs[idx].kind { TabKind::Masking(m) => (m.table.clone(), m.column.clone(), m.rule.clone()), _ => return };
        let status = do_masking(app, &conn_id, &table, &column, &rule);
        if let TabKind::Masking(m) = &mut app.tabs[idx].kind { m.status = status.clone(); }
        app.log(status);
    }
}

pub(crate) fn mask_value(val: &str, rule: &str) -> String {
    match rule {
        "姓名保留首字" => match val.chars().next() { Some(first) => format!("{}**", first), None => "***".into() },
        "邮箱掩码" => match val.split_once('@') {
            Some((local, domain)) => {
                let first = local.chars().next().map(|c| c.to_string()).unwrap_or_default();
                format!("{}***@{}", first, domain)
            }
            None => "***".into(),
        },
        "手机掩码" => {
            let chars: Vec<char> = val.chars().collect();
            if chars.len() >= 7 {
                let head: String = chars[..3].iter().collect();
                let tail: String = chars[chars.len() - 2..].iter().collect();
                format!("{}****{}", head, tail)
            } else { "***".into() }
        }
        "随机数字" => {
            let len = val.chars().count().max(6);
            (0..len).map(|_| char::from_digit(rand::random::<u32>() % 10, 10).unwrap()).collect()
        }
        _ => "***".into(),
    }
}

fn do_masking(app: &NavicatApp, conn_id: &str, table: &str, column: &str, rule: &str) -> String {
    let e = match app.engines.get(conn_id) { Some(e) => e, None => return "未连接".into() };
    if table.is_empty() || column.is_empty() { return "请选择表与列".into(); }
    let q = e.query(&format!("SELECT rowid, {} FROM {}", db::quote_ident(column), db::quote_ident(table)));
    if let Some(err) = q.error.clone() { return format!("读取失败: {err}"); }
    let mut n = 0;
    for row in &q.rows {
        let rowid = row.get(0).cloned().flatten().and_then(|s| s.parse::<i64>().ok());
        let val = row.get(1).cloned().flatten().unwrap_or_default();
        if let Some(rid) = rowid {
            let masked = mask_value(&val, rule);
            if e.update_cell(table, rid, column, Some(&masked)).is_ok() { n += 1; }
        }
    }
    format!("已脱敏 {} 行 ({} . {}) 规则={}", n, table, column, rule)
}

// ---------------- Data / Structure Synchronization ----------------
fn sync(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) {
    let pal = app.pal.clone();
    let mut analyze = false;
    let mut run = false;
    let src = match &app.tabs[idx].kind { TabKind::Sync(s) => s.src_conn.clone(), _ => return };
    header(app, ui, &src, "\u{21CB} 同步（数据 / 结构）");
    let conns: Vec<(String, String)> = app.store.connections.iter().map(|c| (c.id.clone(), c.name.clone())).collect();
    let tables = tables_of(app, &src);
    let src_name = conns.iter().find(|(id, _)| id == &src).map(|(_, n)| n.clone()).unwrap_or_default();
    crate::ui::card(&app).show(ui, |ui| {
        let s = match &mut app.tabs[idx].kind { TabKind::Sync(s) => s, _ => return };
        egui::Grid::new("sygrid").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| {
            ui.label("源连接");
            ui.label(&src_name);
            ui.end_row();
            ui.label("目标连接");
            egui::ComboBox::from_id_source("sydst").selected_text(conns.iter().find(|(id, _)| id == &s.dst_conn).map(|(_, n)| n.clone()).unwrap_or("选择".into()))
                .show_ui(ui, |ui| { for (id, name) in &conns { if ui.selectable_label(&s.dst_conn == id, name).clicked() { s.dst_conn = id.clone(); } } });
            ui.end_row();
            ui.label("表");
            egui::ComboBox::from_id_source("sytable").selected_text(if s.table.is_empty() { "选择表".to_string() } else { s.table.clone() })
                .show_ui(ui, |ui| { for t in &tables { if ui.selectable_label(s.table == *t, t).clicked() { s.table = t.clone(); } } });
            ui.end_row();
            ui.label("模式");
            egui::ComboBox::from_id_source("symode").selected_text(s.mode.clone())
                .show_ui(ui, |ui| { for m in ["数据同步", "结构同步"] { if ui.selectable_label(s.mode == m, m).clicked() { s.mode = m.to_string(); } } });
            ui.end_row();
        });
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            if ui.button("分析差异").clicked() { analyze = true; }
            if ui.button(RichText::new("执行同步").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { run = true; }
        });
        if !s.status.is_empty() { ui.add_space(6.0); ui.colored_label(pal.success, &s.status); }
        if !s.diff.is_empty() {
            ui.add_space(8.0);
            ui.label(RichText::new("差异清单").strong());
            egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                for d in &s.diff { ui.label(RichText::new(format!("\u{2022} {}", d)).size(12.0)); }
            });
        }
    });
    if analyze {
        let (dst, table, mode) = match &app.tabs[idx].kind { TabKind::Sync(s) => (s.dst_conn.clone(), s.table.clone(), s.mode.clone()), _ => return };
        let (status, diff) = analyze_sync(app, &src, &dst, &table, &mode);
        if let TabKind::Sync(s) = &mut app.tabs[idx].kind { s.status = status; s.diff = diff; }
    }
    if run {
        let (dst, table, mode) = match &app.tabs[idx].kind { TabKind::Sync(s) => (s.dst_conn.clone(), s.table.clone(), s.mode.clone()), _ => return };
        app.ensure_engine(&dst);
        let (status, diff) = do_sync(app, &src, &dst, &table, &mode);
        if let TabKind::Sync(s) = &mut app.tabs[idx].kind { s.status = status.clone(); if !diff.is_empty() { s.diff = diff; } }
        app.log(status);
    }
}

fn table_create_sql(e: &crate::db::SqliteEngine, table: &str) -> String {
    e.conn.query_row("SELECT sql FROM sqlite_master WHERE type='table' AND name=?1", [table], |r| r.get(0)).unwrap_or_default()
}

fn analyze_sync(app: &NavicatApp, src: &str, dst: &str, table: &str, mode: &str) -> (String, Vec<String>) {
    if table.is_empty() { return ("请选择表".into(), Vec::new()); }
    let se = match app.engines.get(src) { Some(e) => e, None => return ("源未连接".into(), Vec::new()) };
    let de = match app.engines.get(dst) { Some(e) => e, None => return ("目标未连接".into(), Vec::new()) };
    let mut diff = Vec::new();
    if mode == "结构同步" {
        let sc = se.columns(table);
        let dc = de.columns(table);
        if dc.is_empty() {
            diff.push(format!("目标缺少表 {}，将按源结构创建", table));
        } else {
            for c in &sc { if !dc.iter().any(|d| d.name == c.name) { diff.push(format!("目标缺少列: {} {}", c.name, c.decl_type)); } }
            for d in &dc { if !sc.iter().any(|c| c.name == d.name) { diff.push(format!("目标多出列: {} {}（不会删除）", d.name, d.decl_type)); } }
        }
    } else {
        let sn = se.count(table);
        let dn = if de.columns(table).is_empty() { 0 } else { de.count(table) };
        diff.push(format!("源 {} 行 / 目标 {} 行", sn, dn));
    }
    if diff.is_empty() { diff.push("无差异".into()); }
    (format!("分析完成：{} 项", diff.len()), diff)
}

fn do_sync(app: &mut NavicatApp, src: &str, dst: &str, table: &str, mode: &str) -> (String, Vec<String>) {
    if table.is_empty() { return ("请选择表".into(), Vec::new()); }
    let (create_sql, src_cols, src_rows) = {
        let se = match app.engines.get(src) { Some(e) => e, None => return ("源未连接".into(), Vec::new()) };
        (table_create_sql(se, table), se.columns(table), se.query(&format!("SELECT * FROM {}", db::quote_ident(table))).rows)
    };
    let de = match app.engines.get(dst) { Some(e) => e, None => return ("目标未连接".into(), Vec::new()) };
    let dst_exists = !de.columns(table).is_empty();
    let mut diff = Vec::new();
    if !dst_exists {
        if create_sql.is_empty() { return ("无法读取源建表语句".into(), Vec::new()); }
        if let Err(e) = de.conn.execute(&create_sql, []) { return (format!("建表失败: {e}"), Vec::new()); }
        diff.push(format!("已在目标创建表 {}", table));
    }
    if mode == "结构同步" {
        let old = de.columns(table);
        let mut added = 0;
        for c in &src_cols {
            if !old.iter().any(|o| o.name == c.name) {
                let mut d = format!("ALTER TABLE {} ADD COLUMN {} {}", db::quote_ident(table), db::quote_ident(&c.name), if c.decl_type.is_empty() { "TEXT" } else { &c.decl_type });
                if let Some(def) = &c.default { if !def.is_empty() { d.push_str(&format!(" DEFAULT {}", def)); } }
                if de.conn.execute(&d, []).is_ok() { added += 1; diff.push(format!("新增列: {} {}", c.name, c.decl_type)); }
            }
        }
        return (format!("结构同步完成：新增 {} 列", added), diff);
    }
    let names: Vec<String> = src_cols.iter().map(|c| c.name.clone()).collect();
    let collist: Vec<String> = names.iter().map(|c| db::quote_ident(c)).collect();
    let ph: Vec<String> = (1..=names.len()).map(|i| format!("?{}", i)).collect();
    let sql = format!("INSERT OR REPLACE INTO {} ({}) VALUES ({})", db::quote_ident(table), collist.join(", "), ph.join(", "));
    let mut n = 0;
    for row in &src_rows {
        let params: Vec<rusqlite::types::Value> = row.iter().map(|v| match v {
            Some(s) => rusqlite::types::Value::Text(s.clone()),
            None => rusqlite::types::Value::Null,
        }).collect();
        if de.conn.execute(&sql, rusqlite::params_from_iter(params)).is_ok() { n += 1; }
    }
    diff.push(format!("已写入 {} 行", n));
    (format!("数据同步完成：{} 行 -> {}", n, table), diff)
}

// ---------------- Query History ----------------
fn history(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) {
    let pal = app.pal.clone();
    let _ = idx;
    header(app, ui, "", "\u{21BA} 查询历史");
    let mut open: Option<usize> = None;
    let mut clear = false;
    let items: Vec<String> = app.store.history.clone();
    crate::ui::card(&app).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new(format!("共 {} 条（最多保留 50 条）", items.len())).color(pal.text_dim).size(12.0));
            if !items.is_empty() && ui.small_button("清空历史").clicked() { clear = true; }
        });
        ui.add_space(6.0);
        if items.is_empty() {
            ui.label(RichText::new("暂无历史。在查询编辑器执行 SQL 后会自动记录。").color(pal.text_dim));
            return;
        }
        egui::ScrollArea::vertical().max_height(520.0).show(ui, |ui| {
            for (i, sql) in items.iter().enumerate() {
                let short: String = sql.lines().next().unwrap_or("").chars().take(90).collect();
                ui.horizontal(|ui| {
                    if ui.button("\u{25B6} 打开").clicked() { open = Some(i); }
                    ui.label(RichText::new(short).monospace().size(12.0).color(pal.text));
                });
                ui.separator();
            }
        });
    });
    if clear { app.store.history.clear(); app.store.save(); }
    if let Some(i) = open {
        if let Some(sql) = app.store.history.get(i).cloned() {
            if let Some(id) = app.selected_conn.clone().or_else(|| app.store.connections.first().map(|c| c.id.clone())) {
                app.open_connection(id.clone());
                app.new_query(id);
                let ni = app.tabs.len() - 1;
                if let TabKind::Query(q) = &mut app.tabs[ni].kind { q.sql = sql; }
            }
        }
    }
}

// ---------------- Structure Objects (index / trigger / view) ----------------
fn objects(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) {
    let pal = app.pal.clone();
    let conn_id = match &app.tabs[idx].kind { TabKind::Objects(o) => o.conn_id.clone(), _ => return };
    header(app, ui, &conn_id, "\u{25A4} 对象管理（索引 / 触发器 / 视图）");
    let tables = tables_of(app, &conn_id);
    let (cur_kind, cur_table) = match &app.tabs[idx].kind { TabKind::Objects(o) => (o.kind.clone(), o.table.clone()), _ => (String::new(), String::new()) };
    let list: Vec<(String, String)> = if let Some(e) = app.engines.get(&conn_id) {
        match cur_kind.as_str() {
            "索引" => e.indexes(&cur_table),
            "触发器" => e.triggers_of(&cur_table),
            _ => e.views_list(),
        }
    } else { Vec::new() };

    let mut create = false;
    let mut drop_obj: Option<(String, String)> = None; // (kind_cn, name)
    crate::ui::card(&app).show(ui, |ui| {
        let d = match &mut app.tabs[idx].kind { TabKind::Objects(o) => o, _ => return };
        ui.horizontal(|ui| {
            ui.label("类型:");
            egui::ComboBox::from_id_source("objkind").selected_text(d.kind.clone())
                .show_ui(ui, |ui| { for k in ["索引", "触发器", "视图"] { if ui.selectable_label(d.kind == k, k).clicked() { d.kind = k.to_string(); } } });
            if d.kind == "索引" || d.kind == "触发器" {
                ui.label("表:");
                egui::ComboBox::from_id_source("objtable").selected_text(if d.table.is_empty() { "选择表".to_string() } else { d.table.clone() })
                    .show_ui(ui, |ui| { for t in &tables { if ui.selectable_label(d.table == *t, t).clicked() { d.table = t.clone(); } } });
            }
        });
        ui.add_space(6.0);
        ui.label(RichText::new("现有对象").strong());
        egui::ScrollArea::vertical().max_height(220.0).show(ui, |ui| {
            if list.is_empty() { ui.label(RichText::new("（无）").color(pal.text_dim)); }
            for (name, sql) in &list {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(name).strong().size(12.5));
                    if ui.small_button("删除").clicked() { drop_obj = Some((d.kind.clone(), name.clone())); }
                });
                ui.label(RichText::new(sql).monospace().size(11.0).color(pal.text_dim));
                ui.separator();
            }
        });
        ui.add_space(6.0);
        ui.label(RichText::new("新建").strong());
        egui::Grid::new("objgrid").num_columns(2).spacing([10.0, 6.0]).show(ui, |ui| {
            ui.label("名称");
            ui.add(egui::TextEdit::singleline(&mut d.new_name).desired_width(240.0));
            ui.end_row();
            match d.kind.as_str() {
                "索引" => {
                    ui.label("列（逗号分隔）");
                    ui.add(egui::TextEdit::singleline(&mut d.new_sql).desired_width(400.0).hint_text("如 name, country"));
                    ui.end_row();
                    ui.label("");
                    ui.checkbox(&mut d.unique, "UNIQUE 唯一索引");
                    ui.end_row();
                }
                "触发器" => {
                    ui.label("定义 SQL");
                    ui.add(egui::TextEdit::multiline(&mut d.new_sql).code_editor().desired_rows(4).desired_width(460.0)
                        .hint_text("CREATE TRIGGER trg AFTER INSERT ON customers BEGIN ... END;"));
                    ui.end_row();
                }
                _ => {
                    ui.label("视图定义 SQL");
                    ui.add(egui::TextEdit::multiline(&mut d.new_sql).code_editor().desired_rows(3).desired_width(460.0)
                        .hint_text("SELECT ... FROM ..."));
                    ui.end_row();
                }
            }
        });
        ui.add_space(8.0);
        if ui.button(RichText::new("创建").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { create = true; }
        if !d.status.is_empty() { ui.add_space(6.0); ui.colored_label(pal.success, &d.status); }
    });

    if let Some((k, name)) = drop_obj {
        let sqlkind = match k.as_str() { "索引" => "INDEX", "触发器" => "TRIGGER", _ => "VIEW" };
        let msg = if let Some(e) = app.engines.get(&conn_id) {
            match e.drop_object(sqlkind, &name) { Ok(_) => format!("已删除 {}: {}", k, name), Err(err) => format!("删除失败: {err}") }
        } else { "未连接".into() };
        if let TabKind::Objects(o) = &mut app.tabs[idx].kind { o.status = msg.clone(); }
        app.log(msg);
    }
    if create {
        let (kind, table, name, sql, uniq) = match &app.tabs[idx].kind { TabKind::Objects(o) => (o.kind.clone(), o.table.clone(), o.new_name.clone(), o.new_sql.clone(), o.unique), _ => return };
        let msg = if let Some(e) = app.engines.get(&conn_id) {
            if name.trim().is_empty() { "请填写名称".to_string() }
            else {
                match kind.as_str() {
                    "索引" => {
                        let cols: Vec<String> = sql.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
                        match e.create_index(&name, &table, &cols, uniq) { Ok(_) => format!("已创建索引 {}", name), Err(err) => format!("创建失败: {err}") }
                    }
                    "触发器" => match e.execute_ddl(&sql) { Ok(_) => format!("已创建触发器 {}", name), Err(err) => format!("创建失败: {err}") },
                    _ => match e.create_view(&name, &sql) { Ok(_) => format!("已创建视图 {}", name), Err(err) => format!("创建失败: {err}") },
                }
            }
        } else { "未连接".into() };
        if let TabKind::Objects(o) = &mut app.tabs[idx].kind { o.status = msg.clone(); o.new_name.clear(); o.new_sql.clear(); }
        app.log(msg);
    }
}

// ---------------- Data Pump (SQL dump / load) ----------------
fn pump(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) {
    let pal = app.pal.clone();
    let mut dump = false;
    let mut load = false;
    let conn_id = match &app.tabs[idx].kind { TabKind::Pump(p) => p.conn_id.clone(), _ => return };
    header(app, ui, &conn_id, "\u{21E3} 数据泵（SQL 导出 / 执行）");
    let tables = tables_of(app, &conn_id);
    crate::ui::card(&app).show(ui, |ui| {
        let d = match &mut app.tabs[idx].kind { TabKind::Pump(p) => p, _ => return };
        egui::Grid::new("pumpgrid").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| {
            ui.label("范围");
            egui::ComboBox::from_id_source("pumptable").selected_text(if d.table.is_empty() { "整个数据库".to_string() } else { d.table.clone() })
                .show_ui(ui, |ui| {
                    if ui.selectable_label(d.table.is_empty(), "整个数据库").clicked() { d.table.clear(); }
                    for t in &tables { if ui.selectable_label(d.table == *t, t).clicked() { d.table = t.clone(); } }
                });
            ui.end_row();
            ui.label("SQL 文件");
            ui.add(egui::TextEdit::singleline(&mut d.path).desired_width(420.0));
            ui.end_row();
        });
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            if ui.button(RichText::new("导出 SQL 脚本").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { dump = true; }
            if ui.button("执行 SQL 脚本").clicked() { load = true; }
        });
        if !d.status.is_empty() { ui.add_space(6.0); ui.colored_label(pal.success, &d.status); }
    });
    if dump {
        let (table, path) = match &app.tabs[idx].kind { TabKind::Pump(p) => (p.table.clone(), p.path.clone()), _ => return };
        let msg = if let Some(e) = app.engines.get(&conn_id) {
            let script = e.dump_sql(if table.is_empty() { None } else { Some(table.as_str()) });
            match std::fs::write(&path, script) { Ok(_) => format!("已导出 SQL: {path}"), Err(err) => format!("写入失败: {err}") }
        } else { "未连接".into() };
        if let TabKind::Pump(p) = &mut app.tabs[idx].kind { p.status = msg.clone(); }
        app.log(msg);
    }
    if load {
        let path = match &app.tabs[idx].kind { TabKind::Pump(p) => p.path.clone(), _ => return };
        let msg = if let Some(e) = app.engines.get(&conn_id) {
            match std::fs::read_to_string(&path) {
                Ok(script) => {
                    let results = e.run_script(&script);
                    let errs = results.iter().filter(|r| r.error.is_some()).count();
                    if errs == 0 { format!("已执行 {} 条语句", results.len()) } else { format!("执行完成，{} 条出错", errs) }
                }
                Err(err) => format!("读取失败: {err}"),
            }
        } else { "未连接".into() };
        if let TabKind::Pump(p) = &mut app.tabs[idx].kind { p.status = msg.clone(); }
        app.log(msg);
    }
}

// ---------------- Schema Analysis ----------------
fn compute_schema(e: &crate::db::SqliteEngine) -> Vec<(String, String)> {
    let mut rows = Vec::new();
    let tables: Vec<String> = e.objects().into_iter().filter(|o| o.kind == crate::model::ObjKind::Table).map(|o| o.name).collect();
    rows.push(("数据库".into(), format!("{} 张表", tables.len())));
    for t in &tables {
        let cols = e.columns(t);
        let idx = e.indexes(t).len();
        let pk: Vec<String> = cols.iter().filter(|c| c.pk).map(|c| c.name.clone()).collect();
        let cnt = e.count(t);
        rows.push((format!("表 {}", t), format!("{} 行 · {} 列 · {} 索引 · 主键: {}", cnt, cols.len(), idx, if pk.is_empty() { "无".to_string() } else { pk.join(",") })));
        if pk.is_empty() { rows.push(("⚠ 警告".into(), format!("表 {} 没有主键（不利于更新/复制定位）", t))); }
        if cnt == 0 { rows.push(("提示".into(), format!("表 {} 为空", t))); }
    }
    for (child, _cc, parent, pc) in e.foreign_keys() {
        let has_idx = e.indexes(&child).iter().any(|(_, sql)| sql.to_lowercase().contains(&pc.to_lowercase()));
        if !has_idx { rows.push(("⚠ 提示".into(), format!("外键 {}.{} -> {} 可能缺少索引", child, pc, parent))); }
    }
    rows
}

fn schema(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) {
    let pal = app.pal.clone();
    let conn_id = match &app.tabs[idx].kind { TabKind::Schema(s) => s.conn_id.clone(), _ => return };
    header(app, ui, &conn_id, "\u{2317} 模式分析（Schema Analysis）");
    // auto-compute on first show
    let need = matches!(&app.tabs[idx].kind, TabKind::Schema(s) if !s.computed);
    if need {
        let rows = if let Some(e) = app.engines.get(&conn_id) { compute_schema(e) } else { Vec::new() };
        if let TabKind::Schema(s) = &mut app.tabs[idx].kind { s.rows = rows; s.computed = true; }
    }
    let mut re = false;
    crate::ui::card(&app).show(ui, |ui| {
        let s = match &mut app.tabs[idx].kind { TabKind::Schema(s) => s, _ => return };
        if ui.button("\u{1F504} 重新分析").clicked() { re = true; }
        ui.add_space(6.0);
        egui::ScrollArea::vertical().max_height(560.0).show(ui, |ui| {
            egui::Grid::new("schemagrid").num_columns(2).striped(true).spacing([16.0, 5.0]).show(ui, |ui| {
                for (k, v) in &s.rows {
                    let color = if k.starts_with('⚠') { pal.danger } else if k == "提示" { pal.text_dim } else { pal.text };
                    ui.label(RichText::new(k).strong().size(12.0).color(if k.starts_with('⚠') { pal.danger } else { pal.text }));
                    ui.label(RichText::new(v).size(12.0).color(color));
                    ui.end_row();
                }
            });
        });
    });
    if re {
        let rows = if let Some(e) = app.engines.get(&conn_id) { compute_schema(e) } else { Vec::new() };
        if let TabKind::Schema(s) = &mut app.tabs[idx].kind { s.rows = rows; }
    }
}

// ---------------- SQL Outline / Refactor ----------------
fn outline(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) {
    let pal = app.pal.clone();
    let conn_id = match &app.tabs[idx].kind { TabKind::Outline(o) => o.conn_id.clone(), _ => return };
    header(app, ui, &conn_id, "\u{2263} SQL 大纲 / 重构");
    let mut analyze = false;
    let mut reformat = false;
    // Auto-analyze on first show so the outline is immediately visible.
    let need = matches!(&app.tabs[idx].kind, TabKind::Outline(o) if o.message.is_empty() && !o.sql.trim().is_empty());
    if need {
        let sql = match &app.tabs[idx].kind { TabKind::Outline(o) => o.sql.clone(), _ => String::new() };
        let report = analyze_sql(&sql);
        if let TabKind::Outline(o) = &mut app.tabs[idx].kind { o.message = report; }
    }
    crate::ui::card(&app).show(ui, |ui| {
        let d = match &mut app.tabs[idx].kind { TabKind::Outline(o) => o, _ => return };
        ui.label("SQL 文本");
        ui.add(egui::TextEdit::multiline(&mut d.sql).code_editor().desired_rows(6).desired_width(f32::INFINITY));
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            if ui.button(RichText::new("\u{1F50E} 分析大纲").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { analyze = true; }
            if ui.button("\u{2728} 格式化为美化 SQL").clicked() { reformat = true; }
        });
        if !d.message.is_empty() {
            ui.add_space(6.0);
            ui.label(RichText::new("分析结果").strong());
            egui::ScrollArea::vertical().max_height(400.0).show(ui, |ui| {
                ui.label(RichText::new(&d.message).monospace().size(12.0));
            });
        }
    });
    if reformat {
        if let TabKind::Outline(d) = &mut app.tabs[idx].kind { d.sql = crate::ui::query::beautify_sql(&d.sql); }
    }
    if analyze {
        let sql = match &app.tabs[idx].kind { TabKind::Outline(d) => d.sql.clone(), _ => return };
        let report = analyze_sql(&sql);
        if let TabKind::Outline(d) = &mut app.tabs[idx].kind { d.message = report; }
    }
}

/// Produce a structural outline (statements, tables, columns, keywords) plus
/// lightweight refactor suggestions for a SQL script.
pub fn analyze_sql(sql: &str) -> String {
    let mut out = String::new();
    let stmts = db::split_sql(sql);
    out.push_str(&format!("语句数: {}\n", stmts.len()));
    let mut all_tables: Vec<String> = Vec::new();
    let mut all_cols: Vec<String> = Vec::new();
    let mut suggestions: Vec<String> = Vec::new();
    for (i, s) in stmts.iter().enumerate() {
        let upper = s.to_uppercase();
        let kind = ["SELECT", "INSERT", "UPDATE", "DELETE", "CREATE", "ALTER", "DROP", "WITH"]
            .iter().find(|k| upper.trim_start().starts_with(**k)).copied().unwrap_or("OTHER");
        out.push_str(&format!("\n[{:02}] 类型: {}\n", i + 1, kind));
        let mut first_line: String = s.lines().next().unwrap_or("").trim().chars().take(90).collect();
        if first_line.is_empty() { first_line = "(空)".into(); }
        out.push_str(&format!("     {}", first_line));
        if s.lines().count() > 1 { out.push_str(" ..."); }
        out.push('\n');
        let tables = extract_after_keywords(s, &["FROM", "JOIN", "INTO", "UPDATE"]);
        if !tables.is_empty() {
            out.push_str(&format!("     引用表: {}\n", tables.join(", ")));
            for t in &tables { if !all_tables.contains(t) { all_tables.push(t.clone()); } }
        }
        if kind == "SELECT" {
            if upper.contains("SELECT *") {
                suggestions.push(format!("语句 {}: 使用 SELECT * 会读取全部列，建议显式列出所需字段", i + 1));
            }
            if !upper.contains("WHERE") && upper.contains("FROM") && !upper.contains("LIMIT") {
                suggestions.push(format!("语句 {}: 缺少 WHERE/LIMIT，可能返回全表，请注意性能", i + 1));
            }
            let cols = extract_select_columns(s);
            if !cols.is_empty() { out.push_str(&format!("     输出列: {}\n", cols.join(", "))); for c in cols { if !all_cols.contains(&c) { all_cols.push(c); } } }
        }
        if kind == "UPDATE" && !upper.contains("WHERE") {
            suggestions.push(format!("语句 {}: UPDATE 无 WHERE，将更新整表", i + 1));
        }
        if kind == "DELETE" && !upper.contains("WHERE") {
            suggestions.push(format!("语句 {}: DELETE 无 WHERE，将删除整表数据", i + 1));
        }
        if upper.contains(", ") && upper.contains("FROM") && upper.contains("WHERE") && !upper.contains("JOIN") {
            suggestions.push(format!("语句 {}: 隐式连接写法，建议改用显式 JOIN", i + 1));
        }
    }
    out.push_str("\n---- 汇总 ----\n");
    out.push_str(&format!("涉及表 ({}): {}\n", all_tables.len(), if all_tables.is_empty() { "（无）".to_string() } else { all_tables.join(", ") }));
    out.push_str(&format!("涉及输出列 ({}): {}\n", all_cols.len(), if all_cols.is_empty() { "（无）".to_string() } else { all_cols.join(", ") }));
    out.push_str("\n---- 重构建议 ----\n");
    if suggestions.is_empty() {
        out.push_str("未发现明显问题。\n");
    } else {
        for s in suggestions { out.push_str(&format!("- {}\n", s)); }
    }
    out
}

fn extract_after_keywords(sql: &str, keywords: &[&str]) -> Vec<String> {
    let mut out = Vec::new();
    let tokens: Vec<&str> = sql.split_whitespace().collect();
    for (i, tok) in tokens.iter().enumerate() {
        if keywords.iter().any(|k| tok.eq_ignore_ascii_case(k)) {
            if let Some(next) = tokens.get(i + 1) {
                let t = next.trim_matches(|c: char| c == '(' || c == ')' || c == ',' || c == ';' || c == '"' || c == '`');
                if !t.is_empty() && !t.eq_ignore_ascii_case("SELECT") && !t.eq_ignore_ascii_case("ON") {
                    if !out.iter().any(|x: &String| x.eq_ignore_ascii_case(t)) {
                        out.push(t.to_string());
                    }
                }
            }
        }
    }
    out
}

fn extract_select_columns(sql: &str) -> Vec<String> {
    let upper = sql.to_uppercase();
    let Some(sel) = upper.find("SELECT") else { return Vec::new() };
    let Some(from) = upper.find("FROM") else { return Vec::new() };
    if from <= sel { return Vec::new(); }
    let raw = &sql[sel + 6..from];
    raw.split(',')
        .map(|c| {
            let c = c.trim();
            let c = c.split_whitespace().next().unwrap_or(c);
            c.trim_matches(|ch: char| ch == '(' || ch == ')').to_string()
        })
        .filter(|c| !c.is_empty() && c != "*" && !c.contains('('))
        .collect()
}

// ---------------- Model (conceptual / logical / physical) ----------------
fn model(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) {
    let pal = app.pal.clone();
    let mut reload = false;
    let mut export_sql = false;
    let mut compare = false;
    let mut sync_db = false;
    let mut import_sql = false;
    let conn_id = match &app.tabs[idx].kind { TabKind::Model(m) => m.conn_id.clone(), _ => return };
    header(app, ui, &conn_id, "\u{25A6} 模型设计器");
    let conns: Vec<(String, String)> = app.store.connections.iter().map(|c| (c.id.clone(), c.name.clone())).collect();
    crate::ui::card(&app).show(ui, |ui| {
        let d = match &mut app.tabs[idx].kind { TabKind::Model(m) => m, _ => return };
        ui.horizontal(|ui| {
            ui.label("视图:");
            for v in ["概念模型", "逻辑模型", "物理模型"] {
                if ui.selectable_label(d.view == v, v).clicked() { d.view = v.to_string(); }
            }
        });
        ui.add_space(4.0);
        ui.horizontal_wrapped(|ui| {
            if ui.button("\u{1F504} 反向工程（从数据库）").clicked() { reload = true; }
            if ui.button("\u{2B06} 导出模型为 SQL").clicked() { export_sql = true; }
            if ui.button("\u{1F504} 从模型同步到数据库").clicked() { sync_db = true; }
            if ui.button("\u{21C4} 模型对比").clicked() { compare = true; }
        });
        ui.horizontal(|ui| {
            ui.label("对比目标连接:");
            egui::ComboBox::from_id_source("cmpsel").selected_text(conns.iter().find(|(id, _)| id == &d.cmp_conn).map(|(_, n)| n.clone()).unwrap_or("选择".into()))
                .show_ui(ui, |ui| { for (id, name) in &conns { if ui.selectable_label(&d.cmp_conn == id, name).clicked() { d.cmp_conn = id.clone(); } } });
        });
        ui.horizontal(|ui| {
            ui.label("SQL 文件:");
            ui.add(egui::TextEdit::singleline(&mut d.sql_path).desired_width(360.0));
            if ui.button("导入 SQL 到模型").clicked() { import_sql = true; }
        });
        if !d.status.is_empty() { ui.add_space(4.0); ui.colored_label(pal.success, &d.status); }
    });
    ui.add_space(6.0);
    let (view, tables, fks, diff) = match &app.tabs[idx].kind { TabKind::Model(m) => (m.view.clone(), m.tables.clone(), m.fks.clone(), m.diff.clone()), _ => return };
    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        for (tname, cols) in &tables {
            egui::Frame::none().fill(pal.panel).stroke(Stroke::new(1.0, pal.accent)).rounding(egui::Rounding::same(8.0)).inner_margin(egui::Margin::same(10.0)).show(ui, |ui| {
                ui.set_width(ui.available_width().min(620.0));
                ui.horizontal(|ui| {
                    ui.label(RichText::new(tname).strong().size(13.5).color(pal.accent));
                    ui.label(RichText::new(format!("· {} 字段", cols.len())).size(11.0).color(pal.text_dim));
                });
                ui.separator();
                let body = match view.as_str() {
                    "概念模型" => format!("实体（{} 个字段）", cols.len()),
                    "物理模型" => cols.join("   \u{00B7}   "),
                    _ => cols.iter().map(|c| c.split_whitespace().next().unwrap_or(c.as_str()).to_string()).collect::<Vec<_>>().join("   \u{00B7}   "),
                };
                ui.add(egui::Label::new(RichText::new(body).size(11.0).color(pal.text)));
            });
            ui.add_space(6.0);
        }
        ui.add_space(8.0);
        if !fks.is_empty() {
            ui.label(RichText::new("关系 (外键)").strong());
            for (child, cc, parent, pc) in &fks {
                ui.label(RichText::new(format!("\u{2192} {}.{}  =  {}.{}", child, cc, parent, pc)).size(11.5).color(pal.text_dim));
            }
        }
        if !diff.is_empty() {
            ui.add_space(8.0);
            ui.label(RichText::new("模型对比结果").strong());
            for line in &diff { ui.label(RichText::new(line).size(11.5)); }
        }
    });

    if reload {
        let (tables, fks) = app.reload_model(&conn_id);
        let msg = format!("已反向工程: {} 张表", tables.len());
        if let TabKind::Model(m) = &mut app.tabs[idx].kind { m.tables = tables; m.fks = fks; m.status = msg.clone(); }
        app.log(msg);
    }
    if export_sql {
        let (path, tables) = match &app.tabs[idx].kind { TabKind::Model(m) => (m.sql_path.clone(), m.tables.clone()), _ => return };
        let mut sql = String::from("-- 由 Navicat-RS 模型导出\n");
        for (t, cols) in &tables {
            let defs: Vec<String> = cols.iter().map(|c| {
                let mut parts = c.splitn(2, ' ');
                let name = parts.next().unwrap_or("c");
                let ty = parts.next().unwrap_or("TEXT");
                format!("  {} {}", db::quote_ident(name), ty)
            }).collect();
            sql.push_str(&format!("CREATE TABLE {} (\n{}\n);\n", db::quote_ident(t), defs.join(",\n")));
        }
        let msg = match std::fs::write(&path, &sql) { Ok(_) => format!("已导出模型 SQL -> {path}"), Err(e) => format!("写入失败: {e}") };
        if let TabKind::Model(m) = &mut app.tabs[idx].kind { m.status = msg.clone(); }
        app.log(msg);
    }
    if compare {
        let (cmp, tables) = match &app.tabs[idx].kind { TabKind::Model(m) => (m.cmp_conn.clone(), m.tables.clone()), _ => return };
        let other = app.reload_model(&cmp).0;
        let mut diff: Vec<String> = Vec::new();
        let names_a: Vec<&String> = tables.iter().map(|(t, _)| t).collect();
        let names_b: Vec<&String> = other.iter().map(|(t, _)| t).collect();
        for t in &names_a { if !names_b.contains(t) { diff.push(format!("仅在源存在: 表 {}", t)); } }
        for t in &names_b { if !names_a.contains(t) { diff.push(format!("仅在目标存在: 表 {}", t)); } }
        for (t, cols) in &tables {
            if let Some((_, ocols)) = other.iter().find(|(ot, _)| ot == t) {
                for c in cols { if !ocols.contains(c) { diff.push(format!("表 {} 列差异(源): {}", t, c)); } }
                for c in ocols { if !cols.contains(c) { diff.push(format!("表 {} 列差异(目标): {}", t, c)); } }
            }
        }
        if diff.is_empty() { diff.push("两个模型结构一致（表与列相同）。".into()); }
        if let TabKind::Model(m) = &mut app.tabs[idx].kind { m.diff = diff.clone(); m.status = "模型对比完成".into(); }
    }
    if sync_db {
        app.open_sync(conn_id.clone());
    }
    if import_sql {
        let path = match &app.tabs[idx].kind { TabKind::Model(m) => m.sql_path.clone(), _ => return };
        let script = match std::fs::read_to_string(&path) { Ok(s) => s, Err(e) => { if let TabKind::Model(m) = &mut app.tabs[idx].kind { m.status = format!("读取失败: {e}"); } return; } };
        if let Some(e) = app.engines.get(&conn_id) { let _ = e.run_script(&script); }
        let (tables, fks) = app.reload_model(&conn_id);
        let msg = format!("已导入 SQL 到模型: 现有 {} 张表", tables.len());
        if let TabKind::Model(m) = &mut app.tabs[idx].kind { m.tables = tables; m.fks = fks; m.status = msg.clone(); }
        app.log(msg);
    }
}

// ---------------- Find in Database ----------------
fn find(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) {
    let pal = app.pal.clone();
    let mut go = false;
    let conn_id = match &app.tabs[idx].kind { TabKind::Find(f) => f.conn_id.clone(), _ => return };
    header(app, ui, &conn_id, "\u{2315} 查找于数据库");
    crate::ui::card(&app).show(ui, |ui| {
        let d = match &mut app.tabs[idx].kind { TabKind::Find(f) => f, _ => return };
        ui.horizontal(|ui| {
            ui.label("查找内容:");
            ui.add(egui::TextEdit::singleline(&mut d.needle).desired_width(320.0).hint_text("在全部表的文本/数值列中搜索"));
            if ui.button(RichText::new("\u{1F50E} 查找").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { go = true; }
        });
        if !d.status.is_empty() { ui.add_space(4.0); ui.colored_label(pal.success, &d.status); }
        ui.add_space(4.0);
        egui::ScrollArea::both().max_height(480.0).show(ui, |ui| {
            egui::Grid::new("findgrid").striped(true).num_columns(3).spacing([14.0, 4.0]).show(ui, |ui| {
                ui.label(RichText::new("表").strong()); ui.label(RichText::new("列").strong()); ui.label(RichText::new("值").strong()); ui.end_row();
                for (t, c, v) in &d.hits { ui.label(RichText::new(t).size(11.5)); ui.label(RichText::new(c).size(11.5)); ui.label(RichText::new(v).size(11.5)); ui.end_row(); }
            });
        });
    });
    if go {
        let needle = match &app.tabs[idx].kind { TabKind::Find(f) => f.needle.clone(), _ => return };
        let hits = if let Some(e) = app.engines.get(&conn_id) { find_in_db(e, &needle) } else { Vec::new() };
        let msg = format!("找到 {} 处匹配", hits.len());
        if let TabKind::Find(f) = &mut app.tabs[idx].kind { f.hits = hits; f.status = msg.clone(); }
        app.log(msg);
    }
}

pub fn find_in_db(e: &crate::db::SqliteEngine, needle: &str) -> Vec<(String, String, String)> {
    let mut hits = Vec::new();
    if needle.trim().is_empty() { return hits; }
    let tables: Vec<String> = e.objects().into_iter().filter(|o| o.kind == crate::model::ObjKind::Table).map(|o| o.name).collect();
    let esc = needle.replace('\'', "''");
    for t in tables {
        for c in e.columns(&t) {
            let sql = format!(
                "SELECT CAST({} AS TEXT) FROM {} WHERE CAST({} AS TEXT) LIKE '%{}%' LIMIT 20",
                db::quote_ident(&c.name), db::quote_ident(&t), db::quote_ident(&c.name), esc
            );
            let r = e.query(&sql);
            if r.error.is_none() {
                for row in r.rows {
                    if let Some(v) = row.into_iter().next().flatten() {
                        hits.push((t.clone(), c.name.clone(), v));
                    }
                }
            }
            if hits.len() >= 500 { return hits; }
        }
    }
    hits
}

// ---------------- Settings ----------------
fn settings(app: &mut NavicatApp, ui: &mut egui::Ui) {
    let pal = app.pal.clone();
    egui::Frame::none().fill(pal.panel).inner_margin(egui::Margin::symmetric(8.0, 5.0)).show(ui, |ui| {
        ui.label(RichText::new("\u{1F527} 设置").strong().size(14.0));
    });
    ui.separator();
    crate::ui::card(app).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label("主题:");
            let cur = crate::theme::label_of(&app.theme);
            egui::ComboBox::from_id_salt("theme_select")
                .selected_text(cur)
                .show_ui(ui, |ui| {
                    for t in crate::theme::THEMES {
                        if ui.selectable_label(app.theme == t.key, t.label).clicked() {
                            let key = t.key;
                            app.set_theme(ui.ctx(), key);
                        }
                    }
                });
        });
        ui.add_space(8.0);
        ui.label("每页行数:");
        let mut ps = app.store.settings.page_size as i64;
        if ui.add(egui::DragValue::new(&mut ps).range(10..=5000)).changed() { app.store.settings.page_size = ps as usize; app.store.save(); }
        ui.label("树形视图缩放:");
        if ui.add(egui::Slider::new(&mut app.store.settings.tree_zoom, 0.7..=1.9).text("倍")).changed() { app.store.save(); }
        ui.checkbox(&mut app.store.settings.confirm_delete, "删除前确认");
        ui.checkbox(&mut app.store.settings.auto_commit, "自动提交（关闭后查询编辑器将整段脚本包在事务中执行）")
            .on_hover_text("开启：每条语句独立自动提交；关闭：一次运行的整段脚本作为单个事务原子执行");
        app.store.save();
        ui.add_space(8.0);
        ui.label(format!("数据目录: {}", crate::store::Store::dir().to_string_lossy()));
        ui.label(format!("配置文件: {}", crate::store::Store::path().to_string_lossy()));
        ui.add_space(8.0);
        if ui.button("立即保存设置").clicked() { app.store.save(); app.log("设置已保存"); }
    });
}
