use crate::app::{ChartKind, NavicatApp, TabKind}; // 引入依赖
use crate::db; // 引入依赖
use egui::{Color32, Pos2, RichText, Stroke, Vec2}; // 引入依赖

pub fn draw(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) { // 函数 draw
    let is = match &app.tabs[idx].kind { // 变量 is
        TabKind::Transfer(_) => "transfer", // 匹配分支
        TabKind::Export(_) => "export", // 匹配分支
        TabKind::Import(_) => "import", // 匹配分支
        TabKind::Backup(_) => "backup", // 匹配分支
        TabKind::Monitor(_) => "monitor", // 匹配分支
        TabKind::Profiling(_) => "profiling", // 匹配分支
        TabKind::DataGen(_) => "datagen", // 匹配分支
        TabKind::Chart(_) => "chart", // 匹配分支
        TabKind::Dict(_) => "dict", // 匹配分支
        TabKind::Report(_) => "report", // 匹配分支
        TabKind::Automation(_) => "automation", // 匹配分支
        TabKind::Users(_) => "users", // 匹配分支
        TabKind::Ai(_) => "ai", // 匹配分支
        TabKind::Snippet(_) => "snippet", // 匹配分支
        TabKind::Masking(_) => "masking", // 匹配分支
        TabKind::Sync(_) => "sync", // 匹配分支
        TabKind::History(_) => "history", // 匹配分支
        TabKind::Objects(_) => "objects", // 匹配分支
        TabKind::Pump(_) => "pump", // 匹配分支
        TabKind::Schema(_) => "schema", // 匹配分支
        TabKind::Outline(_) => "outline", // 匹配分支
        TabKind::Model(_) => "model", // 匹配分支
        TabKind::Find(_) => "find", // 匹配分支
        TabKind::Settings => "settings", // 匹配分支
        _ => "", // 匹配分支
    }; // 块结束
    match is { // 分支匹配
        "transfer" => transfer(app, ui, idx), // 匹配分支
        "export" => export(app, ui, idx), // 匹配分支
        "import" => import(app, ui, idx), // 匹配分支
        "backup" => backup(app, ui, idx), // 匹配分支
        "monitor" => monitor(app, ui, idx), // 匹配分支
        "profiling" => profiling(app, ui, idx), // 匹配分支
        "datagen" => datagen(app, ui, idx), // 匹配分支
        "chart" => chart(app, ui, idx), // 匹配分支
        "dict" => dict(app, ui, idx), // 匹配分支
        "report" => report(app, ui, idx), // 匹配分支
        "automation" => automation(app, ui, idx), // 匹配分支
        "users" => users(app, ui, idx), // 匹配分支
        "ai" => ai(app, ui, idx), // 匹配分支
        "snippet" => snippet(app, ui, idx), // 匹配分支
        "masking" => masking(app, ui, idx), // 匹配分支
        "sync" => sync(app, ui, idx), // 匹配分支
        "history" => history(app, ui, idx), // 匹配分支
        "objects" => objects(app, ui, idx), // 匹配分支
        "pump" => pump(app, ui, idx), // 匹配分支
        "schema" => schema(app, ui, idx), // 匹配分支
        "outline" => outline(app, ui, idx), // 匹配分支
        "model" => model(app, ui, idx), // 匹配分支
        "find" => find(app, ui, idx), // 匹配分支
        "settings" => settings(app, ui), // 匹配分支
        _ => { ui.label("未知页面"); } // 匹配分支
    } // 块结束
} // 块结束

fn header(app: &NavicatApp, ui: &mut egui::Ui, conn_id: &str, title: &str) { // 函数 header
    egui::Frame::none().fill(app.pal.panel).inner_margin(egui::Margin::symmetric(8.0, 5.0)).show(ui, |ui| { // 调用 none
        ui.horizontal(|ui| { // 语句
            if !conn_id.is_empty() { crate::ui::conn_badge(ui, app, conn_id); } // 条件分支
            ui.label(RichText::new(title).strong().size(14.0)); // 语句
        }); // 语句
    }); // 语句
    ui.separator(); // 语句
} // 块结束

fn tables_of(app: &NavicatApp, conn_id: &str) -> Vec<String> { // 函数 tables_of
    app.engines.get(conn_id).map(|e| e.objects().into_iter().filter(|o| o.kind == crate::model::ObjKind::Table).map(|o| o.name).collect()).unwrap_or_default() // 语句
} // 块结束

// ---------------- Transfer ----------------
fn transfer(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) { // 函数 transfer
    let pal = app.pal.clone(); // 变量 pal
    let mut go = false; // 变量 go
    let conn_id = match &app.tabs[idx].kind { TabKind::Transfer(t) => t.src_conn.clone(), _ => return }; // 变量 conn_id
    header(app, ui, &conn_id, "\u{21C4} 数据传输"); // 调用 header
    let conns: Vec<(String, String)> = app.store.connections.iter().map(|c| (c.id.clone(), c.name.clone())).collect(); // 变量 conns
    let tables = tables_of(app, &conn_id); // 变量 tables
    // reload columns when the source table changed
    let need_cols = matches!(&app.tabs[idx].kind, TabKind::Transfer(t) if t.cols.is_empty() && !t.table.is_empty()); // 变量 need_cols
    if need_cols { // 条件分支
        let tbl = match &app.tabs[idx].kind { TabKind::Transfer(t) => t.table.clone(), _ => String::new() }; // 变量 tbl
        let cols: Vec<(String, bool)> = app.engines.get(&conn_id).map(|e| e.columns(&tbl).into_iter().map(|c| (c.name, true)).collect()).unwrap_or_default(); // 变量 cols
        if let TabKind::Transfer(t) = &mut app.tabs[idx].kind { t.cols = cols; } // 条件分支
    } // 块结束
    crate::ui::card(&app).show(ui, |ui| { // 调用 card
        let t = match &mut app.tabs[idx].kind { TabKind::Transfer(t) => t, _ => return }; // 变量 t
        egui::Grid::new("trgrid").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| { // 调用 new
            ui.label("源表"); // 语句
            egui::ComboBox::from_id_source("trsrc").selected_text(if t.table.is_empty() { "选择表".to_string() } else { t.table.clone() }) // 调用 from_id_source
                .show_ui(ui, |ui| { for tb in &tables { if ui.selectable_label(t.table == *tb, tb).clicked() { t.table = tb.clone(); t.dst_table = tb.clone(); t.cols.clear(); } } }); // 语句
            ui.end_row(); // 语句
            ui.label("目标连接"); // 语句
            egui::ComboBox::from_id_source("trdst").selected_text(conns.iter().find(|(id, _)| id == &t.dst_conn).map(|(_, n)| n.clone()).unwrap_or("选择".into())) // 调用 from_id_source
                .show_ui(ui, |ui| { for (id, name) in &conns { if ui.selectable_label(&t.dst_conn == id, name).clicked() { t.dst_conn = id.clone(); } } }); // 语句
            ui.end_row(); // 语句
            ui.label("目标表名"); // 语句
            ui.add(egui::TextEdit::singleline(&mut t.dst_table).desired_width(240.0)); // 语句
            ui.end_row(); // 语句
            ui.label("过滤条件 (WHERE)"); // 语句
            ui.add(egui::TextEdit::singleline(&mut t.where_clause).desired_width(300.0).hint_text("如 country = 'China'（可留空）")); // 语句
            ui.end_row(); // 语句
        }); // 语句
        ui.add_space(4.0); // 语句
        ui.label(RichText::new("字段映射（勾选要传输的列）").strong()); // 语句
        ui.horizontal_wrapped(|ui| { // 语句
            for (cname, sel) in t.cols.iter_mut() { // 循环
                ui.checkbox(sel, cname.as_str()); // 语句
            } // 块结束
        }); // 语句
        ui.add_space(8.0); // 语句
        if ui.button(RichText::new("开始传输").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { go = true; } // 条件分支
        if !t.status.is_empty() { ui.add_space(6.0); ui.colored_label(pal.success, &t.status); } // 条件分支
    }); // 语句
    if go { // 条件分支
        let (src, dst, table, dst_table, cols, whr) = match &app.tabs[idx].kind { TabKind::Transfer(t) => (t.src_conn.clone(), t.dst_conn.clone(), t.table.clone(), t.dst_table.clone(), t.cols.clone(), t.where_clause.clone()), _ => return }; // 匹配分支
        app.ensure_engine(&dst); // 语句
        let selected: Vec<String> = cols.into_iter().filter(|(_, s)| *s).map(|(c, _)| c).collect(); // 变量 selected
        let res = do_transfer(app, &src, &dst, &table, &dst_table, &selected, &whr); // 变量 res
        if let TabKind::Transfer(t) = &mut app.tabs[idx].kind { t.status = res.clone(); } // 条件分支
        app.log(res); // 语句
    } // 块结束
} // 块结束

fn do_transfer(app: &mut NavicatApp, src: &str, dst: &str, table: &str, dst_table: &str, selected: &[String], where_clause: &str) -> String { // 函数 do_transfer
    let dst_table = if dst_table.trim().is_empty() { table } else { dst_table }; // 变量 dst_table
    let (create_sql, columns, rows) = { // 调用 let
        let se = match app.engines.get(src) { Some(e) => e, None => return "源未连接".into() }; // 变量 se
        let create_sql: String = se.conn.query_row( // 变量 create_sql
            "SELECT sql FROM sqlite_master WHERE type='table' AND name=?1", [table], |r| r.get(0)).unwrap_or_default(); // 语句
        let all = se.columns(table); // 变量 all
        let columns: Vec<crate::model::Column> = if selected.is_empty() { // 变量 columns
            all // 语句
        } else { // 语句
            all.into_iter().filter(|c| selected.iter().any(|s| s == &c.name)).collect() // 语句
        }; // 块结束
        let whr = if where_clause.trim().is_empty() { String::new() } else { format!(" WHERE {}", where_clause) }; // 变量 whr
        let colsel: Vec<String> = columns.iter().map(|c| db::quote_ident(&c.name)).collect(); // 变量 colsel
        let q = se.query(&format!("SELECT {} FROM {}{}", colsel.join(", "), db::quote_ident(table), whr)); // 变量 q
        (create_sql, columns, q.rows) // 语句
    }; // 块结束
    let de = match app.engines.get(dst) { Some(e) => e, None => return "目标未连接".into() }; // 变量 de
    let _ = de.conn.execute(&format!("DROP TABLE IF EXISTS {}", db::quote_ident(dst_table)), []); // 变量 _
    // create table under the target name
    let create_sql = rename_create_table(&create_sql, dst_table); // 变量 create_sql
    if !create_sql.is_empty() { // 条件分支
        if let Err(e) = de.conn.execute(&create_sql, []) { return format!("建表失败: {e}"); } // 条件分支
    } // 块结束
    let colnames: Vec<String> = columns.iter().map(|c| c.name.clone()).collect(); // 变量 colnames
    let mut n = 0; // 变量 n
    for row in &rows { // 循环
        if de.insert_row(dst_table, &colnames, row).is_ok() { n += 1; } // 条件分支
    } // 块结束
    format!("传输完成: {} 行 · {} 列 -> {}", n, colnames.len(), dst_table) // 语句
} // 块结束

fn rename_create_table(create_sql: &str, new_name: &str) -> String { // 函数 rename_create_table
    if create_sql.is_empty() { return String::new(); } // 条件分支
    // Replace the first occurrence of CREATE TABLE <name> with the new name.
    if let Some(pos) = create_sql.to_uppercase().find("CREATE TABLE") { // 条件分支
        let after = &create_sql[pos + "CREATE TABLE".len()..]; // 变量 after
        let trimmed = after.trim_start(); // 变量 trimmed
        // find end of table token (up to '(' )
        if let Some(paren) = trimmed.find('(') { // 条件分支
            let rest = &trimmed[paren..]; // 变量 rest
            let head = &create_sql[..pos]; // 变量 head
            return format!("{}CREATE TABLE {}{}", head, db::quote_ident(new_name), rest); // 返回
        } // 块结束
    } // 块结束
    create_sql.to_string() // 语句
} // 块结束

// ---------------- Export ----------------
fn export(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) { // 函数 export
    let pal = app.pal.clone(); // 变量 pal
    let mut go = false; // 变量 go
    let conn_id = match &app.tabs[idx].kind { TabKind::Export(e) => e.conn_id.clone(), _ => return }; // 变量 conn_id
    header(app, ui, &conn_id, "\u{2B06} 导出向导"); // 调用 header
    let tables = tables_of(app, &conn_id); // 变量 tables
    crate::ui::card(&app).show(ui, |ui| { // 调用 card
        let e = match &mut app.tabs[idx].kind { TabKind::Export(e) => e, _ => return }; // 变量 e
        egui::Grid::new("exgrid").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| { // 调用 new
            ui.label("表"); // 语句
            egui::ComboBox::from_id_source("extable").selected_text(if e.table.is_empty() { "选择表".to_string() } else { e.table.clone() }) // 调用 from_id_source
                .show_ui(ui, |ui| { for t in &tables { if ui.selectable_label(e.table == *t, t).clicked() { e.table = t.clone(); } } }); // 语句
            ui.end_row(); // 语句
            ui.label("格式"); // 语句
            egui::ComboBox::from_id_source("exfmt").selected_text(e.format.clone()).show_ui(ui, |ui| { // 调用 from_id_source
                for f in ["CSV", "JSON", "SQL", "HTML"] { if ui.selectable_label(e.format == f, f).clicked() { e.format = f.to_string(); } } // 循环
            }); // 语句
            ui.end_row(); // 语句
            ui.label("导出路径"); // 语句
            ui.add(egui::TextEdit::singleline(&mut e.path).desired_width(420.0)); // 语句
            ui.end_row(); // 语句
        }); // 语句
        ui.add_space(8.0); // 语句
        if ui.button(RichText::new("开始导出").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { go = true; } // 条件分支
        if !e.status.is_empty() { ui.add_space(6.0); ui.colored_label(pal.success, &e.status); } // 条件分支
    }); // 语句
    if go { // 条件分支
        let (table, fmt, path) = match &app.tabs[idx].kind { TabKind::Export(e) => (e.table.clone(), e.format.clone(), e.path.clone()), _ => return }; // 匹配分支
        let status = do_export(app, &conn_id, &table, &fmt, &path); // 变量 status
        if let TabKind::Export(e) = &mut app.tabs[idx].kind { e.status = status.clone(); } // 条件分支
        app.log(status); // 语句
    } // 块结束
} // 块结束

fn do_export(app: &NavicatApp, conn_id: &str, table: &str, fmt: &str, path: &str) -> String { // 函数 do_export
    let e = match app.engines.get(conn_id) { Some(e) => e, None => return "未连接".into() }; // 变量 e
    if table.is_empty() { return "请选择表".into(); } // 条件分支
    let r = e.query(&format!("SELECT * FROM {}", db::quote_ident(table))); // 变量 r
    let content = match fmt { // 变量 content
        "JSON" => { // 匹配分支
            let arr: Vec<serde_json::Value> = r.rows.iter().map(|row| { // 变量 arr
                let mut o = serde_json::Map::new(); // 变量 o
                for (i, c) in r.columns.iter().enumerate() { o.insert(c.clone(), opt_json(row.get(i).cloned().flatten())); } // 循环
                serde_json::Value::Object(o) // 调用 Object
            }).collect(); // 语句
            serde_json::to_string_pretty(&arr).unwrap_or_default() // 调用 to_string_pretty
        } // 块结束
        "SQL" => { // 匹配分支
            let create: String = e.conn.query_row("SELECT sql FROM sqlite_master WHERE type='table' AND name=?1", [table], |r| r.get(0)).unwrap_or_default(); // 变量 create
            let mut s = format!("{}\n\n", create); // 变量 s
            for row in &r.rows { // 循环
                let vals: Vec<String> = row.iter().map(|v| match v { Some(s) => format!("'{}'", s.replace('\'', "''")), None => "NULL".into() }).collect(); // 变量 vals
                s.push_str(&format!("INSERT INTO {} VALUES ({});\n", db::quote_ident(table), vals.join(", "))); // 语句
            } // 块结束
            s // 语句
        } // 块结束
        "HTML" => { // 匹配分支
            let mut s = String::new(); // 变量 s
            s.push_str("<!DOCTYPE html>\n<html><head><meta charset=\"utf-8\"><title>"); // 语句
            s.push_str(&html_escape(table)); // 语句
            s.push_str("</title><style>body{font-family:Segoe UI,Arial,sans-serif;margin:24px;}table{border-collapse:collapse;}th,td{border:1px solid #ccc;padding:6px 10px;font-size:13px;}th{background:#2e86c1;color:#fff;}tr:nth-child(even){background:#f4f6f8;}</style></head><body>"); // 语句
            s.push_str(&format!("<h2>{}</h2>", html_escape(table))); // 语句
            s.push_str("<table><thead><tr>"); // 语句
            for c in &r.columns { s.push_str(&format!("<th>{}</th>", html_escape(c))); } // 循环
            s.push_str("</tr></thead><tbody>"); // 语句
            for row in &r.rows { // 循环
                s.push_str("<tr>"); // 语句
                for v in row { s.push_str(&format!("<td>{}</td>", html_escape(v.as_deref().unwrap_or("")))); } // 循环
                s.push_str("</tr>"); // 语句
            } // 块结束
            s.push_str("</tbody></table></body></html>"); // 语句
            s // 语句
        } // 块结束
        _ => { // 匹配分支
            let mut s = String::new(); // 变量 s
            // UTF-8 BOM so Excel opens Chinese text correctly
            s.push('\u{FEFF}'); // 语句
            s.push_str(&r.columns.iter().map(|c| csv_escape(c)).collect::<Vec<_>>().join(",")); // 语句
            s.push('\n'); // 语句
            for row in &r.rows { // 循环
                let line: Vec<String> = row.iter().map(|v| csv_escape(&v.clone().unwrap_or_default())).collect(); // 变量 line
                s.push_str(&line.join(",")); // 语句
                s.push('\n'); // 语句
            } // 块结束
            s // 语句
        } // 块结束
    }; // 块结束
    match std::fs::write(path, content) { // 分支匹配
        Ok(_) => format!("已导出 {} 行 -> {}", r.rows.len(), path), // 匹配分支
        Err(e) => format!("写入失败: {e}"), // 匹配分支
    } // 块结束
} // 块结束

fn csv_escape(s: &str) -> String { // 函数 csv_escape
    if s.contains(',') || s.contains('"') || s.contains('\n') { // 条件分支
        format!("\"{}\"", s.replace('"', "\"\"")) // 语句
    } else { s.to_string() } // 语句
} // 块结束
fn html_escape(s: &str) -> String { // 函数 html_escape
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;") // 语句
} // 块结束
fn opt_json(v: Option<String>) -> serde_json::Value { // 函数 opt_json
    match v { // 分支匹配
        Some(s) => { // 匹配分支
            if let Ok(i) = s.parse::<i64>() { serde_json::Value::from(i) } // 条件分支
            else if let Ok(f) = s.parse::<f64>() { serde_json::Value::from(f) } // 条件分支
            else { serde_json::Value::from(s) } // 否则分支
        } // 块结束
        None => serde_json::Value::Null, // 匹配分支
    } // 块结束
} // 块结束

// ---------------- Import ----------------
fn import(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) { // 函数 import
    let pal = app.pal.clone(); // 变量 pal
    let mut go = false; // 变量 go
    let conn_id = match &app.tabs[idx].kind { TabKind::Import(i) => i.conn_id.clone(), _ => return }; // 变量 conn_id
    header(app, ui, &conn_id, "\u{2B07} 导入向导"); // 调用 header
    let tables = tables_of(app, &conn_id); // 变量 tables
    crate::ui::card(&app).show(ui, |ui| { // 调用 card
        let d = match &mut app.tabs[idx].kind { TabKind::Import(i) => i, _ => return }; // 变量 d
        egui::Grid::new("imgrid").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| { // 调用 new
            ui.label("目标表"); // 语句
            egui::ComboBox::from_id_source("imtable").selected_text(if d.table.is_empty() { "选择表(需已存在)".to_string() } else { d.table.clone() }) // 调用 from_id_source
                .show_ui(ui, |ui| { for t in &tables { if ui.selectable_label(d.table == *t, t).clicked() { d.table = t.clone(); } } }); // 语句
            ui.end_row(); // 语句
            ui.label("格式"); // 语句
            egui::ComboBox::from_id_source("imfmt").selected_text(d.format.clone()).show_ui(ui, |ui| { // 调用 from_id_source
                for f in ["CSV", "JSON", "SQL"] { if ui.selectable_label(d.format == f, f).clicked() { d.format = f.to_string(); } } // 循环
            }); // 语句
            ui.end_row(); // 语句
            ui.label("文件路径"); // 语句
            ui.add(egui::TextEdit::singleline(&mut d.path).desired_width(420.0).hint_text("文件路径：CSV 首行为列名；JSON 为对象数组；SQL 为可执行脚本")); // 语句
            ui.end_row(); // 语句
        }); // 语句
        ui.add_space(8.0); // 语句
        if ui.button(RichText::new("开始导入").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { go = true; } // 条件分支
        if !d.status.is_empty() { ui.add_space(6.0); ui.colored_label(pal.success, &d.status); } // 条件分支
    }); // 语句
    if go { // 条件分支
        let (table, path, fmt) = match &app.tabs[idx].kind { TabKind::Import(i) => (i.table.clone(), i.path.clone(), i.format.clone()), _ => return }; // 匹配分支
        let status = do_import(app, &conn_id, &table, &path, &fmt); // 变量 status
        if let TabKind::Import(i) = &mut app.tabs[idx].kind { i.status = status.clone(); } // 条件分支
        app.log(status); // 语句
    } // 块结束
} // 块结束

fn do_import(app: &NavicatApp, conn_id: &str, table: &str, path: &str, fmt: &str) -> String { // 函数 do_import
    let e = match app.engines.get(conn_id) { Some(e) => e, None => return "未连接".into() }; // 变量 e
    let content = match std::fs::read_to_string(path) { Ok(c) => c, Err(err) => return format!("读取失败: {err}") }; // 变量 content
    match fmt { // 分支匹配
        "SQL" => { // 匹配分支
            let results = e.run_script(&content); // 变量 results
            let errs = results.iter().filter(|r| r.error.is_some()).count(); // 变量 errs
            if let Some(first) = results.iter().find(|r| r.error.is_some()) { // 条件分支
                return format!("SQL 导入：{} 条语句，{} 条出错：{}", results.len(), errs, first.error.clone().unwrap_or_default()); // 返回
            } // 块结束
            format!("SQL 导入完成: {} 条语句", results.len()) // 语句
        } // 块结束
        "JSON" => { // 匹配分支
            if table.is_empty() { return "请选择目标表".into(); } // 条件分支
            match import_json_text(e, table, &content) { // 分支匹配
                Ok(n) => format!("JSON 导入完成: {} 行 -> {}", n, table), // 匹配分支
                Err(err) => err, // 匹配分支
            } // 块结束
        } // 块结束
        _ => { // 匹配分支
            if table.is_empty() { return "请选择目标表".into(); } // 条件分支
            let mut lines = content.lines(); // 变量 lines
            let header: Vec<String> = match lines.next() { Some(h) => parse_csv_line(h).into_iter().map(|s| s.trim().to_string()).collect(), None => return "空文件".into() }; // 变量 header
            let mut n = 0; // 变量 n
            for line in lines { // 循环
                if line.trim().is_empty() { continue; } // 条件分支
                let vals: Vec<Option<String>> = parse_csv_line(line).into_iter().map(|v| if v.is_empty() { None } else { Some(v) }).collect(); // 变量 vals
                if e.insert_row(table, &header, &vals).is_ok() { n += 1; } // 条件分支
            } // 块结束
            format!("导入完成: {} 行 -> {}", n, table) // 语句
        } // 块结束
    } // 块结束
} // 块结束

fn json_val_to_string(v: &serde_json::Value) -> Option<String> { // 函数 json_val_to_string
    match v { // 分支匹配
        serde_json::Value::Null => None, // 匹配分支
        serde_json::Value::String(s) => Some(s.clone()), // 匹配分支
        other => Some(other.to_string()), // 匹配分支
    } // 块结束
} // 块结束

/// Import a JSON array of objects into a table. Column set is derived from the
/// union of keys in the first object; missing keys become NULL.
pub fn import_json_text(e: &crate::db::SqliteEngine, table: &str, content: &str) -> Result<usize, String> { // 函数 import_json_text
    let arr: serde_json::Value = serde_json::from_str(content).map_err(|err| format!("JSON 解析失败: {err}"))?; // 变量 arr
    let items = arr.as_array().ok_or_else(|| "JSON 顶层需为对象数组".to_string())?; // 变量 items
    if items.is_empty() { return Ok(0); } // 条件分支
    let mut cols: Vec<String> = Vec::new(); // 变量 cols
    for item in items { // 循环
        if let Some(obj) = item.as_object() { // 条件分支
            for k in obj.keys() { if !cols.contains(k) { cols.push(k.clone()); } } // 循环
        } // 块结束
    } // 块结束
    let mut n = 0; // 变量 n
    for item in items { // 循环
        if let Some(obj) = item.as_object() { // 条件分支
            let vals: Vec<Option<String>> = cols.iter().map(|c| obj.get(c).and_then(json_val_to_string)).collect(); // 变量 vals
            if e.insert_row(table, &cols, &vals).is_ok() { n += 1; } // 条件分支
        } // 块结束
    } // 块结束
    Ok(n) // 调用 Ok
} // 块结束

fn parse_csv_line(line: &str) -> Vec<String> { // 函数 parse_csv_line
    let mut out = Vec::new(); // 变量 out
    let mut cur = String::new(); // 变量 cur
    let mut in_q = false; // 变量 in_q
    let mut chars = line.chars().peekable(); // 变量 chars
    while let Some(c) = chars.next() { // 循环
        if in_q { // 条件分支
            if c == '"' { // 条件分支
                if chars.peek() == Some(&'"') { cur.push('"'); chars.next(); } else { in_q = false; } // 条件分支
            } else { cur.push(c); } // 语句
        } else if c == '"' { in_q = true; } // 条件分支
        else if c == ',' { out.push(cur.clone()); cur.clear(); } // 条件分支
        else { cur.push(c); } // 否则分支
    } // 块结束
    out.push(cur); // 语句
    out // 语句
} // 块结束

// ---------------- Backup ----------------
fn backup(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) { // 函数 backup
    let pal = app.pal.clone(); // 变量 pal
    let mut do_backup = false; // 变量 do_backup
    let mut do_restore = false; // 变量 do_restore
    let conn_id = match &app.tabs[idx].kind { TabKind::Backup(b) => b.conn_id.clone(), _ => return }; // 变量 conn_id
    header(app, ui, &conn_id, "\u{1F4BE} 备份 / 还原"); // 调用 header
    let dbfile = app.conn(&conn_id).map(|c| c.sqlite_path.clone()).unwrap_or_default(); // 变量 dbfile
    crate::ui::card(&app).show(ui, |ui| { // 调用 card
        let b = match &mut app.tabs[idx].kind { TabKind::Backup(b) => b, _ => return }; // 变量 b
        ui.label(format!("数据库文件: {}", if dbfile.is_empty() { "内存库（示例，支持 VACUUM INTO 备份）" } else { &dbfile })); // 语句
        ui.add_space(6.0); // 语句
        ui.horizontal(|ui| { // 语句
            ui.label("备份/还原路径:"); // 语句
            ui.add(egui::TextEdit::singleline(&mut b.path).desired_width(420.0)); // 语句
        }); // 语句
        ui.add_space(8.0); // 语句
        ui.horizontal(|ui| { // 语句
            if ui.button(RichText::new("备份到文件").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { do_backup = true; } // 条件分支
            if ui.button("从文件还原").clicked() { do_restore = true; } // 条件分支
        }); // 语句
        if !b.status.is_empty() { ui.add_space(6.0); ui.colored_label(pal.success, &b.status); } // 条件分支
    }); // 语句
    let path = match &app.tabs[idx].kind { TabKind::Backup(b) => b.path.clone(), _ => return }; // 变量 path
    if do_backup { // 条件分支
        // Works for both file and in-memory databases via VACUUM INTO (consistent snapshot).
        let status = if let Some(e) = app.engines.get(&conn_id) { // 变量 status
            match e.vacuum_into(&path) { Ok(_) => format!("已备份 -> {path}"), Err(err) => format!("备份失败: {err}") } // 分支匹配
        } else { "未连接".into() }; // 语句
        if let TabKind::Backup(b) = &mut app.tabs[idx].kind { b.status = status.clone(); } // 条件分支
        app.log(status); // 语句
    } // 块结束
    if do_restore { // 条件分支
        let status = if dbfile.is_empty() { // 变量 status
            // In-memory: load backup snapshot and replay its SQL into the live memory engine.
            restore_into_memory(app, &conn_id, &path) // 调用 restore_into_memory
        } else { // 语句
            app.engines.remove(&conn_id); // 语句
            match std::fs::copy(&path, &dbfile) { Ok(_) => { app.ensure_engine(&conn_id); format!("已从 {} 还原", path) } Err(e) => format!("还原失败: {e}") } // 分支匹配
        }; // 块结束
        if let TabKind::Backup(b) = &mut app.tabs[idx].kind { b.status = status.clone(); } // 条件分支
        app.log(status); // 语句
    } // 块结束
} // 块结束

fn restore_into_memory(app: &mut NavicatApp, conn_id: &str, path: &str) -> String { // 函数 restore_into_memory
    let src = match crate::db::SqliteEngine::open(path) { Ok(e) => e, Err(err) => return format!("打开备份失败: {err}") }; // 变量 src
    let script = src.dump_sql(None); // 变量 script
    let e = match app.engines.get(conn_id) { Some(e) => e, None => return "未连接".into() }; // 变量 e
    // Drop existing schema in the live memory DB so restore reflects the backup exactly.
    let mut drops = Vec::new(); // 变量 drops
    if let Ok(mut st) = e.conn.prepare("SELECT type, name FROM sqlite_master WHERE type IN ('view','table') AND name NOT LIKE 'sqlite_%'") { // 条件分支
        if let Ok(mut rows) = st.query([]) { // 条件分支
            while let Ok(Some(r)) = rows.next() { // 循环
                let ty: String = r.get(0).unwrap_or_default(); // 变量 ty
                let nm: String = r.get(1).unwrap_or_default(); // 变量 nm
                drops.push(format!("DROP {} IF EXISTS {}", if ty == "view" { "VIEW" } else { "TABLE" }, db::quote_ident(&nm))); // 语句
            } // 块结束
        } // 块结束
    } // 块结束
    for d in drops { let _ = e.conn.execute(&d, []); } // 循环
    let results = e.run_script(&script); // 变量 results
    let errs = results.iter().filter(|r| r.error.is_some()).count(); // 变量 errs
    if errs == 0 { format!("已从 {} 还原（内存库，{} 条语句）", path, results.len()) } else { format!("还原完成，{} 条语句出错", errs) } // 条件分支
} // 块结束

// ---------------- Monitor ----------------
fn monitor(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) { // 函数 monitor
    let conn_id = match &app.tabs[idx].kind { TabKind::Monitor(m) => m.conn_id.clone(), _ => return }; // 变量 conn_id
    header(app, ui, &conn_id, "\u{1F4C8} 服务器监控"); // 调用 header
    let stats: Vec<(String, String)> = if let Some(e) = app.engines.get(&conn_id) { // 变量 stats
        let mut v = vec![ // 变量 v
            ("SQLite 版本".into(), e.sqlite_version()), // 语句
            ("页大小".into(), format!("{} 字节", e.pragma_i64("page_size"))), // 语句
            ("页数量".into(), e.pragma_i64("page_count").to_string()), // 语句
            ("空闲页".into(), e.pragma_i64("freelist_count").to_string()), // 语句
            ("数据库大小".into(), format!("{} KB", e.db_file_size() / 1024)), // 语句
            ("完整性检查".into(), e.integrity_check()), // 语句
            ("journal_mode".into(), e.pragma_str("journal_mode")), // 语句
            ("表数量".into(), e.objects().iter().filter(|o| o.kind == crate::model::ObjKind::Table).count().to_string()), // 语句
        ]; // 语句
        v.push(("连接数".into(), "1 (本进程)".into())); // 语句
        v // 语句
    } else { vec![("状态".into(), "未连接".into())] }; // 语句
    crate::ui::card(&app).show(ui, |ui| { // 调用 card
        egui::Grid::new("mongrid").num_columns(2).striped(true).spacing([16.0, 6.0]).show(ui, |ui| { // 调用 new
            for (k, v) in &stats { ui.label(RichText::new(k).strong()); ui.label(v); ui.end_row(); } // 循环
        }); // 语句
        ui.add_space(8.0); // 语句
        if ui.button("\u{1F504} 刷新").clicked() { /* auto next frame */ } // 条件分支
    }); // 语句
} // 块结束

// ---------------- Profiling ----------------
fn profiling(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) { // 函数 profiling
    let pal = app.pal.clone(); // 变量 pal
    let mut go = false; // 变量 go
    let conn_id = match &app.tabs[idx].kind { TabKind::Profiling(p) => p.conn_id.clone(), _ => return }; // 变量 conn_id
    header(app, ui, &conn_id, "\u{1F50E} 数据剖析"); // 调用 header
    let tables = tables_of(app, &conn_id); // 变量 tables
    crate::ui::card(&app).show(ui, |ui| { // 调用 card
        let d = match &mut app.tabs[idx].kind { TabKind::Profiling(p) => p, _ => return }; // 变量 d
        ui.horizontal(|ui| { // 语句
            ui.label("表:"); // 语句
            egui::ComboBox::from_id_source("prtable").selected_text(if d.table.is_empty() { "选择表".to_string() } else { d.table.clone() }) // 调用 from_id_source
                .show_ui(ui, |ui| { for t in &tables { if ui.selectable_label(d.table == *t, t).clicked() { d.table = t.clone(); } } }); // 语句
            if ui.button(RichText::new("计算统计").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { go = true; } // 条件分支
        }); // 语句
        if !d.stats.is_empty() { // 条件分支
            ui.add_space(8.0); // 语句
            egui::ScrollArea::vertical().max_height(400.0).show(ui, |ui| { // 调用 vertical
                egui::Grid::new("prgrid").num_columns(2).striped(true).spacing([16.0, 4.0]).show(ui, |ui| { // 调用 new
                    for (k, v) in &d.stats { ui.label(RichText::new(k).strong().size(12.0)); ui.label(RichText::new(v).size(12.0)); ui.end_row(); } // 循环
                }); // 语句
            }); // 语句
        } // 块结束
    }); // 语句
    if go { // 条件分支
        let table = match &app.tabs[idx].kind { TabKind::Profiling(p) => p.table.clone(), _ => return }; // 变量 table
        let mut stats = Vec::new(); // 变量 stats
        if let Some(e) = app.engines.get(&conn_id) { // 条件分支
            for c in e.columns(&table) { // 循环
                let cn = db::quote_ident(&c.name); // 变量 cn
                let total = e.count(&table); // 变量 total
                let nulls: i64 = e.conn.query_row(&format!("SELECT COUNT(*) FROM {} WHERE {} IS NULL", db::quote_ident(&table), cn), [], |r| r.get(0)).unwrap_or(0); // 变量 nulls
                let distinct: i64 = e.conn.query_row(&format!("SELECT COUNT(DISTINCT {}) FROM {}", cn, db::quote_ident(&table)), [], |r| r.get(0)).unwrap_or(0); // 变量 distinct
                stats.push((format!("{} 行数", c.name), total.to_string())); // 语句
                stats.push((format!("{} 空值/唯一值", c.name), format!("{} / {}", nulls, distinct))); // 语句
            } // 块结束
            // sample distinct values for first text column
            let top = e.query(&format!("SELECT * FROM {} LIMIT 3", db::quote_ident(&table))); // 变量 top
            stats.push(("样本".into(), format!("{} 行预览", top.rows.len()))); // 语句
        } // 块结束
        if let TabKind::Profiling(p) = &mut app.tabs[idx].kind { p.stats = stats; p.computed = true; } // 条件分支
        app.log("数据剖析完成"); // 语句
    } // 块结束
} // 块结束

// ---------------- DataGen ----------------
fn datagen(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) { // 函数 datagen
    let pal = app.pal.clone(); // 变量 pal
    let mut go = false; // 变量 go
    let conn_id = match &app.tabs[idx].kind { TabKind::DataGen(g) => g.conn_id.clone(), _ => return }; // 变量 conn_id
    header(app, ui, &conn_id, "\u{1F9EA} 数据生成"); // 调用 header
    crate::ui::card(&app).show(ui, |ui| { // 调用 card
        let d = match &mut app.tabs[idx].kind { TabKind::DataGen(g) => g, _ => return }; // 变量 d
        ui.horizontal(|ui| { // 语句
            ui.label("目标表:"); // 语句
            ui.add(egui::TextEdit::singleline(&mut d.table).desired_width(200.0)); // 语句
            ui.label("生成行数:"); // 语句
            let mut c = d.count as i64; // 变量 c
            if ui.add(egui::DragValue::new(&mut c).range(1..=100000)).changed() { d.count = c as usize; } // 条件分支
            if ui.button(RichText::new("生成数据").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { go = true; } // 条件分支
        }); // 语句
        ui.add_space(6.0); // 语句
        ui.label(RichText::new("根据列名智能生成: name/email/city/country/phone/date/price/age 等；其余列生成随机值。").size(11.0).color(pal.text_dim)); // 语句
        if !d.status.is_empty() { ui.add_space(6.0); ui.colored_label(pal.success, &d.status); } // 条件分支
    }); // 语句
    if go { // 条件分支
        let (table, count) = match &app.tabs[idx].kind { TabKind::DataGen(g) => (g.table.clone(), g.count), _ => return }; // 匹配分支
        let status = do_datagen(app, &conn_id, &table, count); // 变量 status
        if let TabKind::DataGen(g) = &mut app.tabs[idx].kind { g.status = status.clone(); } // 条件分支
        app.log(status); // 语句
    } // 块结束
} // 块结束

fn do_datagen(app: &NavicatApp, conn_id: &str, table: &str, count: usize) -> String { // 函数 do_datagen
    let e = match app.engines.get(conn_id) { Some(e) => e, None => return "未连接".into() }; // 变量 e
    let cols = e.columns(table); // 变量 cols
    if cols.is_empty() { return "表不存在或无列".into(); } // 条件分支
    let first = ["Alice", "Bob", "Carol", "David", "Eve", "Frank", "Grace", "Henry", "Ivy", "Jack"]; // 变量 first
    let last = ["Zhang", "Li", "Wang", "Chen", "Liu", "Wu", "Xu", "Sun"]; // 变量 last
    let cities = ["Beijing", "Shanghai", "Shenzhen", "Hangzhou", "Chengdu", "Guangzhou"]; // 变量 cities
    let mut n = 0; // 变量 n
    for i in 0..count { // 循环
        let mut names = Vec::new(); // 变量 names
        let mut vals = Vec::new(); // 变量 vals
        for c in &cols { // 循环
            let ln = c.name.to_lowercase(); // 变量 ln
            let v: Option<String> = if c.pk { None } else if ln.contains("mail") { Some(format!("user{}@example.com", rand::random::<u32>() % 100000)) } // 变量 v
            else if ln.contains("name") { Some(format!("{} {}", first[(i + 3) % first.len()], last[i % last.len()])) } // 条件分支
            else if ln.contains("city") { Some(cities[i % cities.len()].into()) } // 条件分支
            else if ln.contains("country") { Some(cities[i % cities.len()].into()) } // 条件分支
            else if ln.contains("phone") { Some(format!("138{:08}", rand::random::<u32>() % 100000000)) } // 条件分支
            else if ln.contains("date") || ln.contains("time") { Some(chrono::Local::now().format("%Y-%m-%d").to_string()) } // 条件分支
            else if ln.contains("price") || ln.contains("salary") || ln.contains("total") { Some(format!("{:.2}", (rand::random::<u32>() % 9000) as f64 / 10.0)) } // 条件分支
            else if ln.contains("age") || ln.contains("qty") || ln.contains("quantity") || ln.contains("stock") { Some((18 + (i % 40)).to_string()) } // 条件分支
            else { Some(format!("val_{}_{}", i, c.name)) }; // 否则分支
            // skip autoincrement integer pk
            let skip = c.pk && (c.decl_type.to_uppercase().contains("INT")); // 变量 skip
            if skip { continue; } // 条件分支
            names.push(c.name.clone()); // 语句
            vals.push(v); // 语句
        } // 块结束
        if e.insert_row(table, &names, &vals).is_ok() { n += 1; } // 条件分支
    } // 块结束
    format!("已生成 {} 行 -> {}", n, table) // 语句
} // 块结束

// ---------------- Chart ----------------
fn chart(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) { // 函数 chart
    let pal = app.pal.clone(); // 变量 pal
    let mut go = false; // 变量 go
    let conn_id = match &app.tabs[idx].kind { TabKind::Chart(c) => c.conn_id.clone(), _ => return }; // 变量 conn_id
    header(app, ui, &conn_id, "\u{1F4CA} 图表 / 仪表盘"); // 调用 header
    crate::ui::card(&app).show(ui, |ui| { // 调用 card
        let d = match &mut app.tabs[idx].kind { TabKind::Chart(c) => c, _ => return }; // 变量 d
        ui.horizontal(|ui| { // 语句
            egui::ComboBox::from_id_source("chartkind").selected_text(match d.kind { ChartKind::Bar => "柱状图", ChartKind::Line => "折线图", ChartKind::Pie => "饼图" }) // 匹配分支
                .show_ui(ui, |ui| { // 语句
                    if ui.selectable_label(matches!(d.kind, ChartKind::Bar), "柱状图").clicked() { d.kind = ChartKind::Bar; } // 条件分支
                    if ui.selectable_label(matches!(d.kind, ChartKind::Line), "折线图").clicked() { d.kind = ChartKind::Line; } // 条件分支
                    if ui.selectable_label(matches!(d.kind, ChartKind::Pie), "饼图").clicked() { d.kind = ChartKind::Pie; } // 条件分支
                }); // 语句
            if ui.button(RichText::new("\u{25B6} 生成图表").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { go = true; } // 条件分支
        }); // 语句
        ui.add(egui::TextEdit::multiline(&mut d.sql).code_editor().desired_rows(2).desired_width(f32::INFINITY)); // 语句
        if !d.status.is_empty() { ui.colored_label(pal.text_dim, &d.status); } // 条件分支
    }); // 语句
    if go { // 条件分支
        let sql = match &app.tabs[idx].kind { TabKind::Chart(c) => c.sql.clone(), _ => return }; // 变量 sql
        let res = if let Some(e) = app.engines.get(&conn_id) { e.query(&sql) } else { Default::default() }; // 变量 res
        if let TabKind::Chart(c) = &mut app.tabs[idx].kind { // 条件分支
            c.status = format!("{} 行", res.rows.len()); // 语句
            c.data = res; // 语句
        } // 块结束
        app.log("图表已生成"); // 语句
    } // 块结束
    // draw chart
    let (kind, data, lc, vc) = match &app.tabs[idx].kind { TabKind::Chart(c) => (c.kind, c.data.clone(), c.label_col, c.value_col), _ => return }; // 匹配分支
    if data.columns.is_empty() { // 条件分支
        ui.centered_and_justified(|ui| { ui.label(RichText::new("输入聚合 SQL 并点击“生成图表”").color(pal.text_dim)); }); // 语句
        return; // 返回
    } // 块结束
    let avail = ui.available_rect_before_wrap(); // 变量 avail
    let painter = ui.painter_at(avail); // 变量 painter
    painter.rect_filled(avail, 0.0, pal.panel); // 语句
    if data.rows.is_empty() { return; } // 条件分支
    let vals: Vec<(String, f64)> = data.rows.iter().map(|r| { // 变量 vals
        let label = r.get(lc).cloned().flatten().unwrap_or_default(); // 变量 label
        let v = r.get(vc).cloned().flatten().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0); // 变量 v
        (label, v) // 语句
    }).collect(); // 语句
    let maxv = vals.iter().map(|(_, v)| *v).fold(0.0f64, f64::max).max(1.0); // 变量 maxv
    let colors = [Color32::from_rgb(0x22,0x74,0xE8), Color32::from_rgb(0x1E,0x9E,0x5A), Color32::from_rgb(0xE0,0x8A,0x1E), Color32::from_rgb(0x8E,0x44,0xAD), Color32::from_rgb(0xC7,0x46,0x34), Color32::from_rgb(0x29,0xB5,0xE8)]; // 变量 colors
    match kind { // 分支匹配
        ChartKind::Bar => { // 匹配分支
            let n = vals.len(); // 变量 n
            let bw = (avail.width() - 80.0) / n as f32 * 0.7; // 变量 bw
            let gap = (avail.width() - 80.0) / n as f32; // 变量 gap
            let base = avail.bottom() - 40.0; // 变量 base
            let hmax = avail.height() - 80.0; // 变量 hmax
            for (i, (label, v)) in vals.iter().enumerate() { // 循环
                let x = avail.left() + 50.0 + i as f32 * gap; // 变量 x
                let h = (v / maxv) as f32 * hmax; // 变量 h
                let rect = egui::Rect::from_min_size(Pos2::new(x, base - h), Vec2::new(bw, h)); // 变量 rect
                painter.rect_filled(rect, 3.0, colors[i % colors.len()]); // 语句
                painter.text(Pos2::new(x + bw / 2.0, base + 12.0), egui::Align2::CENTER_CENTER, label, egui::FontId::proportional(11.0), pal.text_dim); // 语句
                painter.text(Pos2::new(x + bw / 2.0, base - h - 10.0), egui::Align2::CENTER_CENTER, format!("{:.0}", v), egui::FontId::proportional(10.0), pal.text); // 语句
            } // 块结束
            painter.line_segment([Pos2::new(avail.left()+45.0, base), Pos2::new(avail.right()-10.0, base)], Stroke::new(1.0, pal.border)); // 语句
        } // 块结束
        ChartKind::Line => { // 匹配分支
            let n = vals.len(); // 变量 n
            let w = avail.width() - 80.0; // 变量 w
            let h = avail.height() - 80.0; // 变量 h
            let base = avail.bottom() - 40.0; // 变量 base
            let step = if n > 1 { w / (n - 1) as f32 } else { w }; // 变量 step
            let mut pts = Vec::new(); // 变量 pts
            for (i, (_, v)) in vals.iter().enumerate() { // 循环
                let x = avail.left() + 50.0 + i as f32 * step; // 变量 x
                let y = base - (v / maxv) as f32 * h; // 变量 y
                pts.push(Pos2::new(x, y)); // 语句
            } // 块结束
            for w2 in pts.windows(2) { painter.line_segment([w2[0], w2[1]], Stroke::new(2.0, pal.accent)); } // 循环
            for (i, p) in pts.iter().enumerate() { painter.circle_filled(*p, 3.5, colors[i % colors.len()]); } // 循环
        } // 块结束
        ChartKind::Pie => { // 匹配分支
            let total: f64 = vals.iter().map(|(_, v)| *v).sum::<f64>().max(1.0); // 变量 total
            let center = avail.center(); // 变量 center
            let radius = (avail.height().min(avail.width()) * 0.4).min(200.0); // 变量 radius
            let mut start = 0.0f64; // 变量 start
            for (i, (label, v)) in vals.iter().enumerate() { // 循环
                let frac = v / total; // 变量 frac
                let end = start + frac * std::f64::consts::TAU; // 变量 end
                draw_wedge(&painter, center, radius, start, end, colors[i % colors.len()]); // 调用 draw_wedge
                let mid = (start + end) / 2.0; // 变量 mid
                let lp = center + Vec2::new((radius * 0.65 * mid.cos() as f32), (radius * 0.65 * mid.sin() as f32)); // 变量 lp
                painter.text(lp, egui::Align2::CENTER_CENTER, format!("{} {:.0}%", label, frac * 100.0), egui::FontId::proportional(10.0), Color32::WHITE); // 语句
                start = end; // 语句
            } // 块结束
        } // 块结束
    } // 块结束
} // 块结束

fn draw_wedge(p: &egui::Painter, c: Pos2, r: f32, a0: f64, a1: f64, color: Color32) { // 函数 draw_wedge
    let steps = 24; // 变量 steps
    let mut pts = vec![c]; // 变量 pts
    for i in 0..=steps { // 循环
        let a = a0 + (a1 - a0) * i as f64 / steps as f64; // 变量 a
        pts.push(c + Vec2::new((r * a.cos() as f32), (r * a.sin() as f32))); // 语句
    } // 块结束
    p.add(egui::Shape::convex_polygon(pts, color, Stroke::NONE)); // 语句
} // 块结束

// ---------------- Dict ----------------
fn dict(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) { // 函数 dict
    let pal = app.pal.clone(); // 变量 pal
    let conn_id = match &app.tabs[idx].kind { TabKind::Dict(d) => d.conn_id.clone(), _ => return }; // 变量 conn_id
    header(app, ui, &conn_id, "\u{1F4D6} 数据字典"); // 调用 header
    let tables = tables_of(app, &conn_id); // 变量 tables
    let sel = match &app.tabs[idx].kind { TabKind::Dict(d) => d.table.clone(), _ => String::new() }; // 变量 sel
    let mut newsel: Option<String> = None; // 变量 newsel
    crate::ui::card(&app).show(ui, |ui| { // 调用 card
        ui.horizontal(|ui| { // 语句
            ui.label("表:"); // 语句
            egui::ComboBox::from_id_source("dicttable").selected_text(if sel.is_empty() { "全部".to_string() } else { sel.clone() }) // 调用 from_id_source
                .show_ui(ui, |ui| { // 语句
                    if ui.selectable_label(sel.is_empty(), "全部表").clicked() { newsel = Some(String::new()); } // 条件分支
                    for t in &tables { if ui.selectable_label(sel == *t, t).clicked() { newsel = Some(t.clone()); } } // 循环
                }); // 语句
        }); // 语句
        ui.add_space(6.0); // 语句
        if let Some(e) = app.engines.get(&conn_id) { // 条件分支
            let list: Vec<String> = if sel.is_empty() { tables.clone() } else { vec![sel.clone()] }; // 变量 list
            egui::ScrollArea::vertical().max_height(520.0).show(ui, |ui| { // 调用 vertical
                for t in list { // 循环
                    ui.label(RichText::new(format!("\u{1F5C2} {}  ({} 行)", t, e.count(&t))).strong().color(pal.accent)); // 语句
                    egui::Grid::new(format!("dg{}", t)).num_columns(4).striped(true).spacing([14.0, 3.0]).show(ui, |ui| { // 调用 new
                        for h in ["列名", "类型", "主键", "非空"] { ui.label(RichText::new(h).size(11.0).color(pal.text_dim)); } // 循环
                        ui.end_row(); // 语句
                        for c in e.columns(&t) { // 循环
                            ui.label(RichText::new(&c.name).size(12.0)); // 语句
                            ui.label(RichText::new(&c.decl_type).size(12.0).monospace()); // 语句
                            ui.label(if c.pk { "\u{2714}" } else { "" }); // 语句
                            ui.label(if c.notnull { "\u{2714}" } else { "" }); // 语句
                            ui.end_row(); // 语句
                        } // 块结束
                    }); // 语句
                    ui.add_space(8.0); // 语句
                } // 块结束
            }); // 语句
        } // 块结束
    }); // 语句
    if let Some(ns) = newsel { // 条件分支
        if let TabKind::Dict(d) = &mut app.tabs[idx].kind { d.table = ns; } // 条件分支
    } // 块结束
} // 块结束

// ---------------- Report ----------------
fn report(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) { // 函数 report
    let pal = app.pal.clone(); // 变量 pal
    let mut go = false; // 变量 go
    let mut export = false; // 变量 export
    let conn_id = match &app.tabs[idx].kind { TabKind::Report(r) => r.conn_id.clone(), _ => return }; // 变量 conn_id
    header(app, ui, &conn_id, "\u{1F4CB} 报表"); // 调用 header
    crate::ui::card(&app).show(ui, |ui| { // 调用 card
        let d = match &mut app.tabs[idx].kind { TabKind::Report(r) => r, _ => return }; // 变量 d
        ui.horizontal(|ui| { // 语句
            ui.label("标题:"); // 语句
            ui.add(egui::TextEdit::singleline(&mut d.title).desired_width(300.0)); // 语句
            if ui.button(RichText::new("生成报表").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { go = true; } // 条件分支
            if ui.button("导出 HTML").clicked() { export = true; } // 条件分支
        }); // 语句
        ui.add(egui::TextEdit::multiline(&mut d.sql).code_editor().desired_rows(2).desired_width(f32::INFINITY)); // 语句
        if !d.status.is_empty() { ui.colored_label(pal.text_dim, &d.status); } // 条件分支
    }); // 语句
    if go { // 条件分支
        let sql = match &app.tabs[idx].kind { TabKind::Report(r) => r.sql.clone(), _ => return }; // 变量 sql
        let res = if let Some(e) = app.engines.get(&conn_id) { e.query(&sql) } else { Default::default() }; // 变量 res
        if let TabKind::Report(r) = &mut app.tabs[idx].kind { r.status = format!("{} 行", res.rows.len()); } // 条件分支
        // store result in a temp via ChartDoc? Instead render inline: reuse preview by storing into status only.
        // Render below using a fresh query each frame is expensive; store into dict? We'll just render now.
        render_report(ui, &app.pal, &res); // 调用 render_report
        app.log("报表已生成"); // 语句
        return; // 返回
    } // 块结束
    // initial render attempt
    let sql = match &app.tabs[idx].kind { TabKind::Report(r) => r.sql.clone(), _ => return }; // 变量 sql
    if let Some(e) = app.engines.get(&conn_id) { // 条件分支
        let res = e.query(&sql); // 变量 res
        render_report(ui, &pal, &res); // 调用 render_report
    } // 块结束
    if export { // 条件分支
        let title = match &app.tabs[idx].kind { TabKind::Report(r) => r.title.clone(), _ => "report".into() }; // 变量 title
        let path = crate::app::default_export_dir().join(format!("{}.html", sanitize(&title))); // 变量 path
        if let Some(e) = app.engines.get(&conn_id) { // 条件分支
            let res = e.query(&sql); // 变量 res
            let html = report_html(&title, &res); // 变量 html
            let _ = std::fs::write(&path, html); // 变量 _
            app.log(format!("报表已导出: {}", path.to_string_lossy())); // 语句
        } // 块结束
    } // 块结束
} // 块结束

fn sanitize(s: &str) -> String { s.chars().map(|c| if c.is_alphanumeric() { c } else { '_' }).collect() } // 函数 sanitize

fn render_report(ui: &mut egui::Ui, pal: &crate::theme::Palette, res: &crate::db::QueryResult) { // 函数 render_report
    ui.separator(); // 语句
    egui::ScrollArea::both().max_height(420.0).show(ui, |ui| { crate::ui::query::render_result(ui, pal, res); }); // 调用 both
} // 块结束

fn report_html(title: &str, res: &crate::db::QueryResult) -> String { // 函数 report_html
    let mut h = format!("<!doctype html><meta charset='utf-8'><title>{}</title><h2>{}</h2><table border=1 cellspacing=0 cellpadding=6><tr>", title, title); // 变量 h
    for c in &res.columns { h.push_str(&format!("<th>{}</th>", c)); } // 循环
    h.push_str("</tr>"); // 语句
    for row in &res.rows { // 循环
        h.push_str("<tr>"); // 语句
        for v in row { h.push_str(&format!("<td>{}</td>", v.clone().unwrap_or_default())); } // 循环
        h.push_str("</tr>"); // 语句
    } // 块结束
    h.push_str("</table>"); // 语句
    h // 语句
} // 块结束

// ---------------- Automation ----------------
fn automation(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) { // 函数 automation
    let pal = app.pal.clone(); // 变量 pal
    let conn_id = match &app.tabs[idx].kind { TabKind::Automation(a) => a.conn_id.clone(), _ => return }; // 变量 conn_id
    header(app, ui, &conn_id, "\u{2699} 自动化 / 计划任务"); // 调用 header
    let mut add = false; // 变量 add
    crate::ui::card(&app).show(ui, |ui| { // 调用 card
        let d = match &mut app.tabs[idx].kind { TabKind::Automation(a) => a, _ => return }; // 变量 d
        ui.label("计划任务（本地执行，无云端依赖）:"); // 语句
        ui.add_space(6.0); // 语句
        for t in &d.tasks { ui.label(RichText::new(format!("\u{23F0} {}", t)).size(13.0)); } // 循环
        ui.add_space(8.0); // 语句
        if ui.button(RichText::new("+ 新建计划任务").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { add = true; } // 条件分支
    }); // 语句
    if add { if let TabKind::Automation(a) = &mut app.tabs[idx].kind { a.tasks.push(format!("备份任务 {}", a.tasks.len() + 1)); } } // 条件分支
} // 块结束

// ---------------- Users ----------------
fn users(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) { // 函数 users
    let pal = app.pal.clone(); // 变量 pal
    let conn_id = match &app.tabs[idx].kind { TabKind::Users(u) => u.conn_id.clone(), _ => return }; // 变量 conn_id
    header(app, ui, &conn_id, "\u{1F465} 用户与权限"); // 调用 header
    crate::ui::card(&app).show(ui, |ui| { // 调用 card
        ui.horizontal(|ui| { // 语句
            crate::ui::mini_badge(ui, "全部权限已开放", pal.success); // 调用 mini_badge
            ui.label(RichText::new("本应用不设任何用户、角色或权限限制。").color(pal.text_dim).size(12.0)); // 语句
        }); // 语句
        ui.add_space(8.0); // 语句
        egui::Grid::new("usersgrid").num_columns(4).striped(true).spacing([18.0, 5.0]).show(ui, |ui| { // 调用 new
            for h in ["用户", "主机", "角色", "权限"] { ui.label(RichText::new(h).strong().size(12.0)); } // 循环
            ui.end_row(); // 语句
            for (u, hst, role) in [("root", "%", "DBA"), ("app", "10.0.%", "读写"), ("report", "%", "只读"), ("guest", "localhost", "受限")] { // 循环
                ui.label(u); ui.label(hst); ui.label(role); ui.label(RichText::new("全部").color(pal.success)); // 语句
                ui.end_row(); // 语句
            } // 块结束
        }); // 语句
        ui.add_space(8.0); // 语句
        ui.label(RichText::new("提示: SQLite 无内置用户/权限系统，此处为界面展示；所有对象的增删改查均无限制。").size(11.0).color(pal.text_dim)); // 语句
    }); // 语句
} // 块结束

// ---------------- AI ----------------
fn ai(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) { // 函数 ai
    let pal = app.pal.clone(); // 变量 pal
    let mut ask = false; // 变量 ask
    let conn_id = match &app.tabs[idx].kind { TabKind::Ai(a) => a.conn_id.clone(), _ => return }; // 变量 conn_id
    header(app, ui, &conn_id, "\u{2728} AI 助手（本地离线）"); // 调用 header
    crate::ui::card(&app).show(ui, |ui| { // 调用 card
        let d = match &mut app.tabs[idx].kind { TabKind::Ai(a) => a, _ => return }; // 变量 d
        ui.horizontal(|ui| { // 语句
            if ui.button("解释 SQL").clicked() { d.prompt = format!("解释: {}", first_select(&d.prompt)); ask = true; } // 条件分支
            if ui.button("生成查询").clicked() { d.prompt = "生成查询: 按国家统计客户数量".into(); ask = true; } // 条件分支
            if ui.button("优化建议").clicked() { d.prompt = "优化: SELECT * FROM customers WHERE name LIKE '%a%'".into(); ask = true; } // 条件分支
        }); // 语句
        ui.add_space(6.0); // 语句
        ui.add(egui::TextEdit::multiline(&mut d.prompt).desired_rows(3).desired_width(f32::INFINITY).hint_text("向 AI 提问，例如：生成查询，按国家统计客户数量")); // 语句
        if ui.button(RichText::new("提问").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { ask = true; } // 条件分支
        if !d.answer.is_empty() { // 条件分支
            ui.add_space(8.0); // 语句
            egui::Frame::none().fill(pal.bg).rounding(egui::Rounding::same(6.0)).inner_margin(egui::Margin::same(10.0)).show(ui, |ui| { // 调用 none
                ui.label(RichText::new(&d.answer).monospace().size(12.5)); // 语句
            }); // 语句
        } // 块结束
    }); // 语句
    if ask { // 条件分支
        let (prompt, schema) = match &app.tabs[idx].kind { // 调用 let
            TabKind::Ai(a) => (a.prompt.clone(), app.engines.get(&conn_id).map(|e| { // 匹配分支
                e.objects().into_iter().filter(|o| o.kind == crate::model::ObjKind::Table).map(|o| { // 语句
                    format!("{}({})", o.name, e.columns(&o.name).iter().map(|c| c.name.clone()).collect::<Vec<_>>().join(", ")) // 语句
                }).collect::<Vec<_>>().join("; ") // 语句
            }).unwrap_or_default()), // 语句
            _ => (String::new(), String::new()), // 匹配分支
        }; // 块结束
        let ans = local_ai(&prompt, &schema); // 变量 ans
        if let TabKind::Ai(a) = &mut app.tabs[idx].kind { a.answer = ans; } // 条件分支
    } // 块结束
} // 块结束

fn first_select(s: &str) -> String { // 函数 first_select
    let idx = s.to_uppercase().find("SELECT").unwrap_or(0); // 变量 idx
    s[idx..].to_string() // 语句
} // 块结束

fn local_ai(prompt: &str, schema: &str) -> String { // 函数 local_ai
    let p = prompt.to_lowercase(); // 变量 p
    let mut out = String::new(); // 变量 out
    if p.contains("生成") || p.contains("查询") && (p.contains("统计") || p.contains("count")) { // 条件分支
        out.push_str("建议 SQL:\n"); // 语句
        if p.contains("国家") || p.contains("country") { // 条件分支
            out.push_str("SELECT country, COUNT(*) AS cnt\nFROM customers\nGROUP BY country\nORDER BY cnt DESC;\n"); // 语句
        } else if p.contains("销售") || p.contains("sales") { // 条件分支
            out.push_str("SELECT strftime('%Y-%m', order_date) AS month, SUM(total) AS total\nFROM orders\nGROUP BY month\nORDER BY month;\n"); // 语句
        } else { // 语句
            out.push_str("SELECT * FROM customers LIMIT 100;\n"); // 语句
        } // 块结束
    } else if p.contains("解释") || p.contains("explain") { // 条件分支
        out.push_str("该语句解析:\n• SELECT 指定要返回的列\n• FROM 指定数据来源表/视图\n• WHERE 过滤行（可在过滤列上建索引加速）\n• GROUP BY 聚合分组\n• ORDER BY 排序\n• LIMIT 限制返回行数\n"); // 语句
    } else if p.contains("优化") || p.contains("optimize") { // 条件分支
        out.push_str("优化建议:\n1) 避免 SELECT *，只取需要的列\n2) 对 WHERE/ORDER BY 高频列建立索引\n3) LIKE '%x%' 无法用索引，考虑前缀匹配或全文索引\n4) 大数据量分页使用键集分页 (WHERE id > ?)\n5) 用 EXPLAIN QUERY PLAN 验证执行计划\n"); // 语句
    } else { // 语句
        out.push_str("可用能力: 解释 SQL / 生成查询 / 优化建议。\n当前库对象:\n"); // 语句
        out.push_str(if schema.is_empty() { "(未连接)" } else { schema }); // 语句
    } // 块结束
    out // 语句
} // 块结束

// ---------------- Snippet ----------------
fn snippet(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) { // 函数 snippet
    let pal = app.pal.clone(); // 变量 pal
    let mut save = false; // 变量 save
    let mut del: Option<usize> = None; // 变量 del
    let mut insert = false; // 变量 insert
    header(app, ui, "", "\u{1F4CC} 代码片段"); // 调用 header
    ui.columns(2, |cols| { // 语句
        egui::Frame::none().fill(pal.panel).stroke(Stroke::new(1.0, pal.border)).inner_margin(egui::Margin::same(10.0)).show(&mut cols[0], |ui| { // 调用 none
            ui.label(RichText::new("片段列表").strong()); // 语句
            egui::ScrollArea::vertical().id_source("snipscroll").max_height(400.0).show(ui, |ui| { // 调用 vertical
                let snips: Vec<(String, String)> = app.store.snippets.iter().map(|s| (s.name.clone(), s.sql.clone())).collect(); // 变量 snips
                let mut pick: Option<usize> = None; // 变量 pick
                for (i, (name, _)) in snips.iter().enumerate() { // 循环
                    let sel = matches!(&app.tabs[idx].kind, TabKind::Snippet(s) if s.selected == Some(i)); // 变量 sel
                    if ui.selectable_label(sel, name).clicked() { pick = Some(i); } // 条件分支
                } // 块结束
                if let Some(i) = pick { // 条件分支
                    if let TabKind::Snippet(s) = &mut app.tabs[idx].kind { // 条件分支
                        s.selected = Some(i); // 语句
                        s.edit_name = snips[i].0.clone(); // 语句
                        s.edit_sql = snips[i].1.clone(); // 语句
                    } // 块结束
                } // 块结束
            }); // 语句
        }); // 语句
        egui::Frame::none().fill(pal.panel).stroke(Stroke::new(1.0, pal.border)).inner_margin(egui::Margin::same(10.0)).show(&mut cols[1], |ui| { // 调用 none
            let s = match &mut app.tabs[idx].kind { TabKind::Snippet(s) => s, _ => return }; // 变量 s
            ui.horizontal(|ui| { ui.label("名称:"); ui.add(egui::TextEdit::singleline(&mut s.edit_name).desired_width(240.0)); }); // 语句
            ui.add_space(4.0); // 语句
            ui.add(egui::TextEdit::multiline(&mut s.edit_sql).code_editor().desired_rows(10).desired_width(f32::INFINITY)); // 语句
            ui.add_space(6.0); // 语句
            ui.horizontal(|ui| { // 语句
                if ui.button(if s.selected.is_some() { "更新" } else { "保存为新片段" }).clicked() { save = true; } // 条件分支
                if s.selected.is_some() && ui.button("删除").clicked() { del = s.selected; } // 条件分支
                if ui.button("插入到查询编辑器").clicked() { insert = true; } // 条件分支
            }); // 语句
        }); // 语句
    }); // 语句
    if save { // 条件分支
        if let TabKind::Snippet(s) = &app.tabs[idx].kind { // 条件分支
            let name = s.edit_name.clone(); let sql = s.edit_sql.clone(); let sel = s.selected; // 变量 name
            if let Some(i) = sel { if i < app.store.snippets.len() { app.store.snippets[i] = crate::store::Snippet { name, sql }; } } // 条件分支
            else { app.store.snippets.push(crate::store::Snippet { name, sql }); } // 否则分支
            app.store.save(); // 语句
            app.log("片段已保存"); // 语句
        } // 块结束
    } // 块结束
    if let Some(i) = del { if i < app.store.snippets.len() { app.store.snippets.remove(i); app.store.save(); } if let TabKind::Snippet(s) = &mut app.tabs[idx].kind { s.selected = None; s.edit_name.clear(); s.edit_sql.clear(); } } // 条件分支
    if insert { // 条件分支
        let sql = match &app.tabs[idx].kind { TabKind::Snippet(s) => s.edit_sql.clone(), _ => String::new() }; // 变量 sql
        if let Some(id) = app.store.connections.first().map(|c| c.id.clone()) { // 条件分支
            app.open_connection(id.clone()); // 语句
            app.new_query(id); // 语句
            let ni = app.tabs.len() - 1; // 变量 ni
            if let TabKind::Query(q) = &mut app.tabs[ni].kind { q.sql = sql; } // 条件分支
        } // 块结束
    } // 块结束
} // 块结束

// ---------------- Data Masking ----------------
const MASK_RULES: &[&str] = &["全遮蔽 (***)", "姓名保留首字", "邮箱掩码", "手机掩码", "随机数字"]; // 常量 MASK_RULES

fn masking(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) { // 函数 masking
    let pal = app.pal.clone(); // 变量 pal
    let mut go = false; // 变量 go
    let conn_id = match &app.tabs[idx].kind { TabKind::Masking(m) => m.conn_id.clone(), _ => return }; // 变量 conn_id
    header(app, ui, &conn_id, "\u{25D0} 数据脱敏"); // 调用 header
    let tables = tables_of(app, &conn_id); // 变量 tables
    let cur_table = match &app.tabs[idx].kind { TabKind::Masking(m) => m.table.clone(), _ => String::new() }; // 变量 cur_table
    let cols: Vec<String> = app // 变量 cols
        .engines // 语句
        .get(&conn_id) // 语句
        .map(|e| e.columns(&cur_table).into_iter().filter(|c| !c.pk).map(|c| c.name).collect()) // 语句
        .unwrap_or_default(); // 语句
    crate::ui::card(&app).show(ui, |ui| { // 调用 card
        let m = match &mut app.tabs[idx].kind { TabKind::Masking(m) => m, _ => return }; // 变量 m
        egui::Grid::new("mkgrid").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| { // 调用 new
            ui.label("表"); // 语句
            egui::ComboBox::from_id_source("mktable").selected_text(if m.table.is_empty() { "选择表".to_string() } else { m.table.clone() }) // 调用 from_id_source
                .show_ui(ui, |ui| { for t in &tables { if ui.selectable_label(m.table == *t, t).clicked() { m.table = t.clone(); m.column.clear(); } } }); // 语句
            ui.end_row(); // 语句
            ui.label("列"); // 语句
            egui::ComboBox::from_id_source("mkcol").selected_text(if m.column.is_empty() { "选择列".to_string() } else { m.column.clone() }) // 调用 from_id_source
                .show_ui(ui, |ui| { for c in &cols { if ui.selectable_label(m.column == *c, c).clicked() { m.column = c.clone(); } } }); // 语句
            ui.end_row(); // 语句
            ui.label("脱敏规则"); // 语句
            egui::ComboBox::from_id_source("mkrule").selected_text(m.rule.clone()) // 调用 from_id_source
                .show_ui(ui, |ui| { for r in MASK_RULES { if ui.selectable_label(m.rule == *r, *r).clicked() { m.rule = r.to_string(); } } }); // 语句
            ui.end_row(); // 语句
        }); // 语句
        ui.add_space(8.0); // 语句
        ui.label(RichText::new("对所选列就地脱敏（直接更新数据，无法撤销，请先备份）。").size(11.0).color(pal.text_dim)); // 语句
        if ui.button(RichText::new("执行脱敏").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { go = true; } // 条件分支
        if !m.status.is_empty() { ui.add_space(6.0); ui.colored_label(pal.success, &m.status); } // 条件分支
    }); // 语句
    if go { // 条件分支
        let (table, column, rule) = match &app.tabs[idx].kind { TabKind::Masking(m) => (m.table.clone(), m.column.clone(), m.rule.clone()), _ => return }; // 匹配分支
        let status = do_masking(app, &conn_id, &table, &column, &rule); // 变量 status
        if let TabKind::Masking(m) = &mut app.tabs[idx].kind { m.status = status.clone(); } // 条件分支
        app.log(status); // 语句
    } // 块结束
} // 块结束

pub(crate) fn mask_value(val: &str, rule: &str) -> String { // 调用 pub
    match rule { // 分支匹配
        "姓名保留首字" => match val.chars().next() { Some(first) => format!("{}**", first), None => "***".into() }, // 匹配分支
        "邮箱掩码" => match val.split_once('@') { // 匹配分支
            Some((local, domain)) => { // 匹配分支
                let first = local.chars().next().map(|c| c.to_string()).unwrap_or_default(); // 变量 first
                format!("{}***@{}", first, domain) // 语句
            } // 块结束
            None => "***".into(), // 匹配分支
        }, // 块结束
        "手机掩码" => { // 匹配分支
            let chars: Vec<char> = val.chars().collect(); // 变量 chars
            if chars.len() >= 7 { // 条件分支
                let head: String = chars[..3].iter().collect(); // 变量 head
                let tail: String = chars[chars.len() - 2..].iter().collect(); // 变量 tail
                format!("{}****{}", head, tail) // 语句
            } else { "***".into() } // 语句
        } // 块结束
        "随机数字" => { // 匹配分支
            let len = val.chars().count().max(6); // 变量 len
            (0..len).map(|_| char::from_digit(rand::random::<u32>() % 10, 10).unwrap()).collect() // 语句
        } // 块结束
        _ => "***".into(), // 匹配分支
    } // 块结束
} // 块结束

fn do_masking(app: &NavicatApp, conn_id: &str, table: &str, column: &str, rule: &str) -> String { // 函数 do_masking
    let e = match app.engines.get(conn_id) { Some(e) => e, None => return "未连接".into() }; // 变量 e
    if table.is_empty() || column.is_empty() { return "请选择表与列".into(); } // 条件分支
    let q = e.query(&format!("SELECT rowid, {} FROM {}", db::quote_ident(column), db::quote_ident(table))); // 变量 q
    if let Some(err) = q.error.clone() { return format!("读取失败: {err}"); } // 条件分支
    let mut n = 0; // 变量 n
    for row in &q.rows { // 循环
        let rowid = row.get(0).cloned().flatten().and_then(|s| s.parse::<i64>().ok()); // 变量 rowid
        let val = row.get(1).cloned().flatten().unwrap_or_default(); // 变量 val
        if let Some(rid) = rowid { // 条件分支
            let masked = mask_value(&val, rule); // 变量 masked
            if e.update_cell(table, rid, column, Some(&masked)).is_ok() { n += 1; } // 条件分支
        } // 块结束
    } // 块结束
    format!("已脱敏 {} 行 ({} . {}) 规则={}", n, table, column, rule) // 语句
} // 块结束

// ---------------- Data / Structure Synchronization ----------------
fn sync(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) { // 函数 sync
    let pal = app.pal.clone(); // 变量 pal
    let mut analyze = false; // 变量 analyze
    let mut run = false; // 变量 run
    let src = match &app.tabs[idx].kind { TabKind::Sync(s) => s.src_conn.clone(), _ => return }; // 变量 src
    header(app, ui, &src, "\u{21CB} 同步（数据 / 结构）"); // 调用 header
    let conns: Vec<(String, String)> = app.store.connections.iter().map(|c| (c.id.clone(), c.name.clone())).collect(); // 变量 conns
    let tables = tables_of(app, &src); // 变量 tables
    let src_name = conns.iter().find(|(id, _)| id == &src).map(|(_, n)| n.clone()).unwrap_or_default(); // 变量 src_name
    crate::ui::card(&app).show(ui, |ui| { // 调用 card
        let s = match &mut app.tabs[idx].kind { TabKind::Sync(s) => s, _ => return }; // 变量 s
        egui::Grid::new("sygrid").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| { // 调用 new
            ui.label("源连接"); // 语句
            ui.label(&src_name); // 语句
            ui.end_row(); // 语句
            ui.label("目标连接"); // 语句
            egui::ComboBox::from_id_source("sydst").selected_text(conns.iter().find(|(id, _)| id == &s.dst_conn).map(|(_, n)| n.clone()).unwrap_or("选择".into())) // 调用 from_id_source
                .show_ui(ui, |ui| { for (id, name) in &conns { if ui.selectable_label(&s.dst_conn == id, name).clicked() { s.dst_conn = id.clone(); } } }); // 语句
            ui.end_row(); // 语句
            ui.label("表"); // 语句
            egui::ComboBox::from_id_source("sytable").selected_text(if s.table.is_empty() { "选择表".to_string() } else { s.table.clone() }) // 调用 from_id_source
                .show_ui(ui, |ui| { for t in &tables { if ui.selectable_label(s.table == *t, t).clicked() { s.table = t.clone(); } } }); // 语句
            ui.end_row(); // 语句
            ui.label("模式"); // 语句
            egui::ComboBox::from_id_source("symode").selected_text(s.mode.clone()) // 调用 from_id_source
                .show_ui(ui, |ui| { for m in ["数据同步", "结构同步"] { if ui.selectable_label(s.mode == m, m).clicked() { s.mode = m.to_string(); } } }); // 语句
            ui.end_row(); // 语句
        }); // 语句
        ui.add_space(8.0); // 语句
        ui.horizontal(|ui| { // 语句
            if ui.button("分析差异").clicked() { analyze = true; } // 条件分支
            if ui.button(RichText::new("执行同步").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { run = true; } // 条件分支
        }); // 语句
        if !s.status.is_empty() { ui.add_space(6.0); ui.colored_label(pal.success, &s.status); } // 条件分支
        if !s.diff.is_empty() { // 条件分支
            ui.add_space(8.0); // 语句
            ui.label(RichText::new("差异清单").strong()); // 语句
            egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| { // 调用 vertical
                for d in &s.diff { ui.label(RichText::new(format!("\u{2022} {}", d)).size(12.0)); } // 循环
            }); // 语句
        } // 块结束
    }); // 语句
    if analyze { // 条件分支
        let (dst, table, mode) = match &app.tabs[idx].kind { TabKind::Sync(s) => (s.dst_conn.clone(), s.table.clone(), s.mode.clone()), _ => return }; // 匹配分支
        let (status, diff) = analyze_sync(app, &src, &dst, &table, &mode); // 调用 let
        if let TabKind::Sync(s) = &mut app.tabs[idx].kind { s.status = status; s.diff = diff; } // 条件分支
    } // 块结束
    if run { // 条件分支
        let (dst, table, mode) = match &app.tabs[idx].kind { TabKind::Sync(s) => (s.dst_conn.clone(), s.table.clone(), s.mode.clone()), _ => return }; // 匹配分支
        app.ensure_engine(&dst); // 语句
        let (status, diff) = do_sync(app, &src, &dst, &table, &mode); // 调用 let
        if let TabKind::Sync(s) = &mut app.tabs[idx].kind { s.status = status.clone(); if !diff.is_empty() { s.diff = diff; } } // 条件分支
        app.log(status); // 语句
    } // 块结束
} // 块结束

fn table_create_sql(e: &crate::db::SqliteEngine, table: &str) -> String { // 函数 table_create_sql
    e.conn.query_row("SELECT sql FROM sqlite_master WHERE type='table' AND name=?1", [table], |r| r.get(0)).unwrap_or_default() // 语句
} // 块结束

fn analyze_sync(app: &NavicatApp, src: &str, dst: &str, table: &str, mode: &str) -> (String, Vec<String>) { // 函数 analyze_sync
    if table.is_empty() { return ("请选择表".into(), Vec::new()); } // 条件分支
    let se = match app.engines.get(src) { Some(e) => e, None => return ("源未连接".into(), Vec::new()) }; // 变量 se
    let de = match app.engines.get(dst) { Some(e) => e, None => return ("目标未连接".into(), Vec::new()) }; // 变量 de
    let mut diff = Vec::new(); // 变量 diff
    if mode == "结构同步" { // 条件分支
        let sc = se.columns(table); // 变量 sc
        let dc = de.columns(table); // 变量 dc
        if dc.is_empty() { // 条件分支
            diff.push(format!("目标缺少表 {}，将按源结构创建", table)); // 语句
        } else { // 语句
            for c in &sc { if !dc.iter().any(|d| d.name == c.name) { diff.push(format!("目标缺少列: {} {}", c.name, c.decl_type)); } } // 循环
            for d in &dc { if !sc.iter().any(|c| c.name == d.name) { diff.push(format!("目标多出列: {} {}（不会删除）", d.name, d.decl_type)); } } // 循环
        } // 块结束
    } else { // 语句
        let sn = se.count(table); // 变量 sn
        let dn = if de.columns(table).is_empty() { 0 } else { de.count(table) }; // 变量 dn
        diff.push(format!("源 {} 行 / 目标 {} 行", sn, dn)); // 语句
    } // 块结束
    if diff.is_empty() { diff.push("无差异".into()); } // 条件分支
    (format!("分析完成：{} 项", diff.len()), diff) // 语句
} // 块结束

fn do_sync(app: &mut NavicatApp, src: &str, dst: &str, table: &str, mode: &str) -> (String, Vec<String>) { // 函数 do_sync
    if table.is_empty() { return ("请选择表".into(), Vec::new()); } // 条件分支
    let (create_sql, src_cols, src_rows) = { // 调用 let
        let se = match app.engines.get(src) { Some(e) => e, None => return ("源未连接".into(), Vec::new()) }; // 变量 se
        (table_create_sql(se, table), se.columns(table), se.query(&format!("SELECT * FROM {}", db::quote_ident(table))).rows) // 语句
    }; // 块结束
    let de = match app.engines.get(dst) { Some(e) => e, None => return ("目标未连接".into(), Vec::new()) }; // 变量 de
    let dst_exists = !de.columns(table).is_empty(); // 变量 dst_exists
    let mut diff = Vec::new(); // 变量 diff
    if !dst_exists { // 条件分支
        if create_sql.is_empty() { return ("无法读取源建表语句".into(), Vec::new()); } // 条件分支
        if let Err(e) = de.conn.execute(&create_sql, []) { return (format!("建表失败: {e}"), Vec::new()); } // 条件分支
        diff.push(format!("已在目标创建表 {}", table)); // 语句
    } // 块结束
    if mode == "结构同步" { // 条件分支
        let old = de.columns(table); // 变量 old
        let mut added = 0; // 变量 added
        for c in &src_cols { // 循环
            if !old.iter().any(|o| o.name == c.name) { // 条件分支
                let mut d = format!("ALTER TABLE {} ADD COLUMN {} {}", db::quote_ident(table), db::quote_ident(&c.name), if c.decl_type.is_empty() { "TEXT" } else { &c.decl_type }); // 变量 d
                if let Some(def) = &c.default { if !def.is_empty() { d.push_str(&format!(" DEFAULT {}", def)); } } // 条件分支
                if de.conn.execute(&d, []).is_ok() { added += 1; diff.push(format!("新增列: {} {}", c.name, c.decl_type)); } // 条件分支
            } // 块结束
        } // 块结束
        return (format!("结构同步完成：新增 {} 列", added), diff); // 返回
    } // 块结束
    let names: Vec<String> = src_cols.iter().map(|c| c.name.clone()).collect(); // 变量 names
    let collist: Vec<String> = names.iter().map(|c| db::quote_ident(c)).collect(); // 变量 collist
    let ph: Vec<String> = (1..=names.len()).map(|i| format!("?{}", i)).collect(); // 变量 ph
    let sql = format!("INSERT OR REPLACE INTO {} ({}) VALUES ({})", db::quote_ident(table), collist.join(", "), ph.join(", ")); // 变量 sql
    let mut n = 0; // 变量 n
    for row in &src_rows { // 循环
        let params: Vec<rusqlite::types::Value> = row.iter().map(|v| match v { // 变量 params
            Some(s) => rusqlite::types::Value::Text(s.clone()), // 匹配分支
            None => rusqlite::types::Value::Null, // 匹配分支
        }).collect(); // 语句
        if de.conn.execute(&sql, rusqlite::params_from_iter(params)).is_ok() { n += 1; } // 条件分支
    } // 块结束
    diff.push(format!("已写入 {} 行", n)); // 语句
    (format!("数据同步完成：{} 行 -> {}", n, table), diff) // 语句
} // 块结束

// ---------------- Query History ----------------
fn history(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) { // 函数 history
    let pal = app.pal.clone(); // 变量 pal
    let _ = idx; // 变量 _
    header(app, ui, "", "\u{21BA} 查询历史"); // 调用 header
    let mut open: Option<usize> = None; // 变量 open
    let mut clear = false; // 变量 clear
    let items: Vec<String> = app.store.history.clone(); // 变量 items
    crate::ui::card(&app).show(ui, |ui| { // 调用 card
        ui.horizontal(|ui| { // 语句
            ui.label(RichText::new(format!("共 {} 条（最多保留 50 条）", items.len())).color(pal.text_dim).size(12.0)); // 语句
            if !items.is_empty() && ui.small_button("清空历史").clicked() { clear = true; } // 条件分支
        }); // 语句
        ui.add_space(6.0); // 语句
        if items.is_empty() { // 条件分支
            ui.label(RichText::new("暂无历史。在查询编辑器执行 SQL 后会自动记录。").color(pal.text_dim)); // 语句
            return; // 返回
        } // 块结束
        egui::ScrollArea::vertical().max_height(520.0).show(ui, |ui| { // 调用 vertical
            for (i, sql) in items.iter().enumerate() { // 循环
                let short: String = sql.lines().next().unwrap_or("").chars().take(90).collect(); // 变量 short
                ui.horizontal(|ui| { // 语句
                    if ui.button("\u{25B6} 打开").clicked() { open = Some(i); } // 条件分支
                    ui.label(RichText::new(short).monospace().size(12.0).color(pal.text)); // 语句
                }); // 语句
                ui.separator(); // 语句
            } // 块结束
        }); // 语句
    }); // 语句
    if clear { app.store.history.clear(); app.store.save(); } // 条件分支
    if let Some(i) = open { // 条件分支
        if let Some(sql) = app.store.history.get(i).cloned() { // 条件分支
            if let Some(id) = app.selected_conn.clone().or_else(|| app.store.connections.first().map(|c| c.id.clone())) { // 条件分支
                app.open_connection(id.clone()); // 语句
                app.new_query(id); // 语句
                let ni = app.tabs.len() - 1; // 变量 ni
                if let TabKind::Query(q) = &mut app.tabs[ni].kind { q.sql = sql; } // 条件分支
            } // 块结束
        } // 块结束
    } // 块结束
} // 块结束

// ---------------- Structure Objects (index / trigger / view) ----------------
fn objects(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) { // 函数 objects
    let pal = app.pal.clone(); // 变量 pal
    let conn_id = match &app.tabs[idx].kind { TabKind::Objects(o) => o.conn_id.clone(), _ => return }; // 变量 conn_id
    header(app, ui, &conn_id, "\u{25A4} 对象管理（索引 / 触发器 / 视图）"); // 调用 header
    let tables = tables_of(app, &conn_id); // 变量 tables
    let (cur_kind, cur_table) = match &app.tabs[idx].kind { TabKind::Objects(o) => (o.kind.clone(), o.table.clone()), _ => (String::new(), String::new()) }; // 匹配分支
    let list: Vec<(String, String)> = if let Some(e) = app.engines.get(&conn_id) { // 变量 list
        match cur_kind.as_str() { // 分支匹配
            "索引" => e.indexes(&cur_table), // 匹配分支
            "触发器" => e.triggers_of(&cur_table), // 匹配分支
            _ => e.views_list(), // 匹配分支
        } // 块结束
    } else { Vec::new() }; // 语句

    let mut create = false; // 变量 create
    let mut drop_obj: Option<(String, String)> = None; // (kind_cn, name)
    crate::ui::card(&app).show(ui, |ui| { // 调用 card
        let d = match &mut app.tabs[idx].kind { TabKind::Objects(o) => o, _ => return }; // 变量 d
        ui.horizontal(|ui| { // 语句
            ui.label("类型:"); // 语句
            egui::ComboBox::from_id_source("objkind").selected_text(d.kind.clone()) // 调用 from_id_source
                .show_ui(ui, |ui| { for k in ["索引", "触发器", "视图"] { if ui.selectable_label(d.kind == k, k).clicked() { d.kind = k.to_string(); } } }); // 语句
            if d.kind == "索引" || d.kind == "触发器" { // 条件分支
                ui.label("表:"); // 语句
                egui::ComboBox::from_id_source("objtable").selected_text(if d.table.is_empty() { "选择表".to_string() } else { d.table.clone() }) // 调用 from_id_source
                    .show_ui(ui, |ui| { for t in &tables { if ui.selectable_label(d.table == *t, t).clicked() { d.table = t.clone(); } } }); // 语句
            } // 块结束
        }); // 语句
        ui.add_space(6.0); // 语句
        ui.label(RichText::new("现有对象").strong()); // 语句
        egui::ScrollArea::vertical().max_height(220.0).show(ui, |ui| { // 调用 vertical
            if list.is_empty() { ui.label(RichText::new("（无）").color(pal.text_dim)); } // 条件分支
            for (name, sql) in &list { // 循环
                ui.horizontal(|ui| { // 语句
                    ui.label(RichText::new(name).strong().size(12.5)); // 语句
                    if ui.small_button("删除").clicked() { drop_obj = Some((d.kind.clone(), name.clone())); } // 条件分支
                }); // 语句
                ui.label(RichText::new(sql).monospace().size(11.0).color(pal.text_dim)); // 语句
                ui.separator(); // 语句
            } // 块结束
        }); // 语句
        ui.add_space(6.0); // 语句
        ui.label(RichText::new("新建").strong()); // 语句
        egui::Grid::new("objgrid").num_columns(2).spacing([10.0, 6.0]).show(ui, |ui| { // 调用 new
            ui.label("名称"); // 语句
            ui.add(egui::TextEdit::singleline(&mut d.new_name).desired_width(240.0)); // 语句
            ui.end_row(); // 语句
            match d.kind.as_str() { // 分支匹配
                "索引" => { // 匹配分支
                    ui.label("列（逗号分隔）"); // 语句
                    ui.add(egui::TextEdit::singleline(&mut d.new_sql).desired_width(400.0).hint_text("如 name, country")); // 语句
                    ui.end_row(); // 语句
                    ui.label(""); // 语句
                    ui.checkbox(&mut d.unique, "UNIQUE 唯一索引"); // 语句
                    ui.end_row(); // 语句
                } // 块结束
                "触发器" => { // 匹配分支
                    ui.label("定义 SQL"); // 语句
                    ui.add(egui::TextEdit::multiline(&mut d.new_sql).code_editor().desired_rows(4).desired_width(460.0) // 语句
                        .hint_text("CREATE TRIGGER trg AFTER INSERT ON customers BEGIN ... END;")); // 语句
                    ui.end_row(); // 语句
                } // 块结束
                _ => { // 匹配分支
                    ui.label("视图定义 SQL"); // 语句
                    ui.add(egui::TextEdit::multiline(&mut d.new_sql).code_editor().desired_rows(3).desired_width(460.0) // 语句
                        .hint_text("SELECT ... FROM ...")); // 语句
                    ui.end_row(); // 语句
                } // 块结束
            } // 块结束
        }); // 语句
        ui.add_space(8.0); // 语句
        if ui.button(RichText::new("创建").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { create = true; } // 条件分支
        if !d.status.is_empty() { ui.add_space(6.0); ui.colored_label(pal.success, &d.status); } // 条件分支
    }); // 语句

    if let Some((k, name)) = drop_obj { // 条件分支
        let sqlkind = match k.as_str() { "索引" => "INDEX", "触发器" => "TRIGGER", _ => "VIEW" }; // 变量 sqlkind
        let msg = if let Some(e) = app.engines.get(&conn_id) { // 变量 msg
            match e.drop_object(sqlkind, &name) { Ok(_) => format!("已删除 {}: {}", k, name), Err(err) => format!("删除失败: {err}") } // 分支匹配
        } else { "未连接".into() }; // 语句
        if let TabKind::Objects(o) = &mut app.tabs[idx].kind { o.status = msg.clone(); } // 条件分支
        app.log(msg); // 语句
    } // 块结束
    if create { // 条件分支
        let (kind, table, name, sql, uniq) = match &app.tabs[idx].kind { TabKind::Objects(o) => (o.kind.clone(), o.table.clone(), o.new_name.clone(), o.new_sql.clone(), o.unique), _ => return }; // 匹配分支
        let msg = if let Some(e) = app.engines.get(&conn_id) { // 变量 msg
            if name.trim().is_empty() { "请填写名称".to_string() } // 条件分支
            else { // 否则分支
                match kind.as_str() { // 分支匹配
                    "索引" => { // 匹配分支
                        let cols: Vec<String> = sql.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect(); // 变量 cols
                        match e.create_index(&name, &table, &cols, uniq) { Ok(_) => format!("已创建索引 {}", name), Err(err) => format!("创建失败: {err}") } // 分支匹配
                    } // 块结束
                    "触发器" => match e.execute_ddl(&sql) { Ok(_) => format!("已创建触发器 {}", name), Err(err) => format!("创建失败: {err}") }, // 匹配分支
                    _ => match e.create_view(&name, &sql) { Ok(_) => format!("已创建视图 {}", name), Err(err) => format!("创建失败: {err}") }, // 匹配分支
                } // 块结束
            } // 块结束
        } else { "未连接".into() }; // 语句
        if let TabKind::Objects(o) = &mut app.tabs[idx].kind { o.status = msg.clone(); o.new_name.clear(); o.new_sql.clear(); } // 条件分支
        app.log(msg); // 语句
    } // 块结束
} // 块结束

// ---------------- Data Pump (SQL dump / load) ----------------
fn pump(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) { // 函数 pump
    let pal = app.pal.clone(); // 变量 pal
    let mut dump = false; // 变量 dump
    let mut load = false; // 变量 load
    let conn_id = match &app.tabs[idx].kind { TabKind::Pump(p) => p.conn_id.clone(), _ => return }; // 变量 conn_id
    header(app, ui, &conn_id, "\u{21E3} 数据泵（SQL 导出 / 执行）"); // 调用 header
    let tables = tables_of(app, &conn_id); // 变量 tables
    crate::ui::card(&app).show(ui, |ui| { // 调用 card
        let d = match &mut app.tabs[idx].kind { TabKind::Pump(p) => p, _ => return }; // 变量 d
        egui::Grid::new("pumpgrid").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| { // 调用 new
            ui.label("范围"); // 语句
            egui::ComboBox::from_id_source("pumptable").selected_text(if d.table.is_empty() { "整个数据库".to_string() } else { d.table.clone() }) // 调用 from_id_source
                .show_ui(ui, |ui| { // 语句
                    if ui.selectable_label(d.table.is_empty(), "整个数据库").clicked() { d.table.clear(); } // 条件分支
                    for t in &tables { if ui.selectable_label(d.table == *t, t).clicked() { d.table = t.clone(); } } // 循环
                }); // 语句
            ui.end_row(); // 语句
            ui.label("SQL 文件"); // 语句
            ui.add(egui::TextEdit::singleline(&mut d.path).desired_width(420.0)); // 语句
            ui.end_row(); // 语句
        }); // 语句
        ui.add_space(8.0); // 语句
        ui.horizontal(|ui| { // 语句
            if ui.button(RichText::new("导出 SQL 脚本").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { dump = true; } // 条件分支
            if ui.button("执行 SQL 脚本").clicked() { load = true; } // 条件分支
        }); // 语句
        if !d.status.is_empty() { ui.add_space(6.0); ui.colored_label(pal.success, &d.status); } // 条件分支
    }); // 语句
    if dump { // 条件分支
        let (table, path) = match &app.tabs[idx].kind { TabKind::Pump(p) => (p.table.clone(), p.path.clone()), _ => return }; // 匹配分支
        let msg = if let Some(e) = app.engines.get(&conn_id) { // 变量 msg
            let script = e.dump_sql(if table.is_empty() { None } else { Some(table.as_str()) }); // 变量 script
            match std::fs::write(&path, script) { Ok(_) => format!("已导出 SQL: {path}"), Err(err) => format!("写入失败: {err}") } // 分支匹配
        } else { "未连接".into() }; // 语句
        if let TabKind::Pump(p) = &mut app.tabs[idx].kind { p.status = msg.clone(); } // 条件分支
        app.log(msg); // 语句
    } // 块结束
    if load { // 条件分支
        let path = match &app.tabs[idx].kind { TabKind::Pump(p) => p.path.clone(), _ => return }; // 变量 path
        let msg = if let Some(e) = app.engines.get(&conn_id) { // 变量 msg
            match std::fs::read_to_string(&path) { // 分支匹配
                Ok(script) => { // 匹配分支
                    let results = e.run_script(&script); // 变量 results
                    let errs = results.iter().filter(|r| r.error.is_some()).count(); // 变量 errs
                    if errs == 0 { format!("已执行 {} 条语句", results.len()) } else { format!("执行完成，{} 条出错", errs) } // 条件分支
                } // 块结束
                Err(err) => format!("读取失败: {err}"), // 匹配分支
            } // 块结束
        } else { "未连接".into() }; // 语句
        if let TabKind::Pump(p) = &mut app.tabs[idx].kind { p.status = msg.clone(); } // 条件分支
        app.log(msg); // 语句
    } // 块结束
} // 块结束

// ---------------- Schema Analysis ----------------
fn compute_schema(e: &crate::db::SqliteEngine) -> Vec<(String, String)> { // 函数 compute_schema
    let mut rows = Vec::new(); // 变量 rows
    let tables: Vec<String> = e.objects().into_iter().filter(|o| o.kind == crate::model::ObjKind::Table).map(|o| o.name).collect(); // 变量 tables
    rows.push(("数据库".into(), format!("{} 张表", tables.len()))); // 语句
    for t in &tables { // 循环
        let cols = e.columns(t); // 变量 cols
        let idx = e.indexes(t).len(); // 变量 idx
        let pk: Vec<String> = cols.iter().filter(|c| c.pk).map(|c| c.name.clone()).collect(); // 变量 pk
        let cnt = e.count(t); // 变量 cnt
        rows.push((format!("表 {}", t), format!("{} 行 · {} 列 · {} 索引 · 主键: {}", cnt, cols.len(), idx, if pk.is_empty() { "无".to_string() } else { pk.join(",") }))); // 语句
        if pk.is_empty() { rows.push(("⚠ 警告".into(), format!("表 {} 没有主键（不利于更新/复制定位）", t))); } // 条件分支
        if cnt == 0 { rows.push(("提示".into(), format!("表 {} 为空", t))); } // 条件分支
    } // 块结束
    for (child, _cc, parent, pc) in e.foreign_keys() { // 循环
        let has_idx = e.indexes(&child).iter().any(|(_, sql)| sql.to_lowercase().contains(&pc.to_lowercase())); // 变量 has_idx
        if !has_idx { rows.push(("⚠ 提示".into(), format!("外键 {}.{} -> {} 可能缺少索引", child, pc, parent))); } // 条件分支
    } // 块结束
    rows // 语句
} // 块结束

fn schema(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) { // 函数 schema
    let pal = app.pal.clone(); // 变量 pal
    let conn_id = match &app.tabs[idx].kind { TabKind::Schema(s) => s.conn_id.clone(), _ => return }; // 变量 conn_id
    header(app, ui, &conn_id, "\u{2317} 模式分析（Schema Analysis）"); // 调用 header
    // auto-compute on first show
    let need = matches!(&app.tabs[idx].kind, TabKind::Schema(s) if !s.computed); // 变量 need
    if need { // 条件分支
        let rows = if let Some(e) = app.engines.get(&conn_id) { compute_schema(e) } else { Vec::new() }; // 变量 rows
        if let TabKind::Schema(s) = &mut app.tabs[idx].kind { s.rows = rows; s.computed = true; } // 条件分支
    } // 块结束
    let mut re = false; // 变量 re
    crate::ui::card(&app).show(ui, |ui| { // 调用 card
        let s = match &mut app.tabs[idx].kind { TabKind::Schema(s) => s, _ => return }; // 变量 s
        if ui.button("\u{1F504} 重新分析").clicked() { re = true; } // 条件分支
        ui.add_space(6.0); // 语句
        egui::ScrollArea::vertical().max_height(560.0).show(ui, |ui| { // 调用 vertical
            egui::Grid::new("schemagrid").num_columns(2).striped(true).spacing([16.0, 5.0]).show(ui, |ui| { // 调用 new
                for (k, v) in &s.rows { // 循环
                    let color = if k.starts_with('⚠') { pal.danger } else if k == "提示" { pal.text_dim } else { pal.text }; // 变量 color
                    ui.label(RichText::new(k).strong().size(12.0).color(if k.starts_with('⚠') { pal.danger } else { pal.text })); // 语句
                    ui.label(RichText::new(v).size(12.0).color(color)); // 语句
                    ui.end_row(); // 语句
                } // 块结束
            }); // 语句
        }); // 语句
    }); // 语句
    if re { // 条件分支
        let rows = if let Some(e) = app.engines.get(&conn_id) { compute_schema(e) } else { Vec::new() }; // 变量 rows
        if let TabKind::Schema(s) = &mut app.tabs[idx].kind { s.rows = rows; } // 条件分支
    } // 块结束
} // 块结束

// ---------------- SQL Outline / Refactor ----------------
fn outline(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) { // 函数 outline
    let pal = app.pal.clone(); // 变量 pal
    let conn_id = match &app.tabs[idx].kind { TabKind::Outline(o) => o.conn_id.clone(), _ => return }; // 变量 conn_id
    header(app, ui, &conn_id, "\u{2263} SQL 大纲 / 重构"); // 调用 header
    let mut analyze = false; // 变量 analyze
    let mut reformat = false; // 变量 reformat
    // Auto-analyze on first show so the outline is immediately visible.
    let need = matches!(&app.tabs[idx].kind, TabKind::Outline(o) if o.message.is_empty() && !o.sql.trim().is_empty()); // 变量 need
    if need { // 条件分支
        let sql = match &app.tabs[idx].kind { TabKind::Outline(o) => o.sql.clone(), _ => String::new() }; // 变量 sql
        let report = analyze_sql(&sql); // 变量 report
        if let TabKind::Outline(o) = &mut app.tabs[idx].kind { o.message = report; } // 条件分支
    } // 块结束
    crate::ui::card(&app).show(ui, |ui| { // 调用 card
        let d = match &mut app.tabs[idx].kind { TabKind::Outline(o) => o, _ => return }; // 变量 d
        ui.label("SQL 文本"); // 语句
        ui.add(egui::TextEdit::multiline(&mut d.sql).code_editor().desired_rows(6).desired_width(f32::INFINITY)); // 语句
        ui.add_space(6.0); // 语句
        ui.horizontal(|ui| { // 语句
            if ui.button(RichText::new("\u{1F50E} 分析大纲").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { analyze = true; } // 条件分支
            if ui.button("\u{2728} 格式化为美化 SQL").clicked() { reformat = true; } // 条件分支
        }); // 语句
        if !d.message.is_empty() { // 条件分支
            ui.add_space(6.0); // 语句
            ui.label(RichText::new("分析结果").strong()); // 语句
            egui::ScrollArea::vertical().max_height(400.0).show(ui, |ui| { // 调用 vertical
                ui.label(RichText::new(&d.message).monospace().size(12.0)); // 语句
            }); // 语句
        } // 块结束
    }); // 语句
    if reformat { // 条件分支
        if let TabKind::Outline(d) = &mut app.tabs[idx].kind { d.sql = crate::ui::query::beautify_sql(&d.sql); } // 条件分支
    } // 块结束
    if analyze { // 条件分支
        let sql = match &app.tabs[idx].kind { TabKind::Outline(d) => d.sql.clone(), _ => return }; // 变量 sql
        let report = analyze_sql(&sql); // 变量 report
        if let TabKind::Outline(d) = &mut app.tabs[idx].kind { d.message = report; } // 条件分支
    } // 块结束
} // 块结束

/// Produce a structural outline (statements, tables, columns, keywords) plus
/// lightweight refactor suggestions for a SQL script.
pub fn analyze_sql(sql: &str) -> String { // 函数 analyze_sql
    let mut out = String::new(); // 变量 out
    let stmts = db::split_sql(sql); // 变量 stmts
    out.push_str(&format!("语句数: {}\n", stmts.len())); // 语句
    let mut all_tables: Vec<String> = Vec::new(); // 变量 all_tables
    let mut all_cols: Vec<String> = Vec::new(); // 变量 all_cols
    let mut suggestions: Vec<String> = Vec::new(); // 变量 suggestions
    for (i, s) in stmts.iter().enumerate() { // 循环
        let upper = s.to_uppercase(); // 变量 upper
        let kind = ["SELECT", "INSERT", "UPDATE", "DELETE", "CREATE", "ALTER", "DROP", "WITH"] // 变量 kind
            .iter().find(|k| upper.trim_start().starts_with(**k)).copied().unwrap_or("OTHER"); // 语句
        out.push_str(&format!("\n[{:02}] 类型: {}\n", i + 1, kind)); // 语句
        let mut first_line: String = s.lines().next().unwrap_or("").trim().chars().take(90).collect(); // 变量 first_line
        if first_line.is_empty() { first_line = "(空)".into(); } // 条件分支
        out.push_str(&format!("     {}", first_line)); // 语句
        if s.lines().count() > 1 { out.push_str(" ..."); } // 条件分支
        out.push('\n'); // 语句
        let tables = extract_after_keywords(s, &["FROM", "JOIN", "INTO", "UPDATE"]); // 变量 tables
        if !tables.is_empty() { // 条件分支
            out.push_str(&format!("     引用表: {}\n", tables.join(", "))); // 语句
            for t in &tables { if !all_tables.contains(t) { all_tables.push(t.clone()); } } // 循环
        } // 块结束
        if kind == "SELECT" { // 条件分支
            if upper.contains("SELECT *") { // 条件分支
                suggestions.push(format!("语句 {}: 使用 SELECT * 会读取全部列，建议显式列出所需字段", i + 1)); // 语句
            } // 块结束
            if !upper.contains("WHERE") && upper.contains("FROM") && !upper.contains("LIMIT") { // 条件分支
                suggestions.push(format!("语句 {}: 缺少 WHERE/LIMIT，可能返回全表，请注意性能", i + 1)); // 语句
            } // 块结束
            let cols = extract_select_columns(s); // 变量 cols
            if !cols.is_empty() { out.push_str(&format!("     输出列: {}\n", cols.join(", "))); for c in cols { if !all_cols.contains(&c) { all_cols.push(c); } } } // 条件分支
        } // 块结束
        if kind == "UPDATE" && !upper.contains("WHERE") { // 条件分支
            suggestions.push(format!("语句 {}: UPDATE 无 WHERE，将更新整表", i + 1)); // 语句
        } // 块结束
        if kind == "DELETE" && !upper.contains("WHERE") { // 条件分支
            suggestions.push(format!("语句 {}: DELETE 无 WHERE，将删除整表数据", i + 1)); // 语句
        } // 块结束
        if upper.contains(", ") && upper.contains("FROM") && upper.contains("WHERE") && !upper.contains("JOIN") { // 条件分支
            suggestions.push(format!("语句 {}: 隐式连接写法，建议改用显式 JOIN", i + 1)); // 语句
        } // 块结束
    } // 块结束
    out.push_str("\n---- 汇总 ----\n"); // 语句
    out.push_str(&format!("涉及表 ({}): {}\n", all_tables.len(), if all_tables.is_empty() { "（无）".to_string() } else { all_tables.join(", ") })); // 语句
    out.push_str(&format!("涉及输出列 ({}): {}\n", all_cols.len(), if all_cols.is_empty() { "（无）".to_string() } else { all_cols.join(", ") })); // 语句
    out.push_str("\n---- 重构建议 ----\n"); // 语句
    if suggestions.is_empty() { // 条件分支
        out.push_str("未发现明显问题。\n"); // 语句
    } else { // 语句
        for s in suggestions { out.push_str(&format!("- {}\n", s)); } // 循环
    } // 块结束
    out // 语句
} // 块结束

fn extract_after_keywords(sql: &str, keywords: &[&str]) -> Vec<String> { // 函数 extract_after_keywords
    let mut out = Vec::new(); // 变量 out
    let tokens: Vec<&str> = sql.split_whitespace().collect(); // 变量 tokens
    for (i, tok) in tokens.iter().enumerate() { // 循环
        if keywords.iter().any(|k| tok.eq_ignore_ascii_case(k)) { // 条件分支
            if let Some(next) = tokens.get(i + 1) { // 条件分支
                let t = next.trim_matches(|c: char| c == '(' || c == ')' || c == ',' || c == ';' || c == '"' || c == '`'); // 变量 t
                if !t.is_empty() && !t.eq_ignore_ascii_case("SELECT") && !t.eq_ignore_ascii_case("ON") { // 条件分支
                    if !out.iter().any(|x: &String| x.eq_ignore_ascii_case(t)) { // 条件分支
                        out.push(t.to_string()); // 语句
                    } // 块结束
                } // 块结束
            } // 块结束
        } // 块结束
    } // 块结束
    out // 语句
} // 块结束

fn extract_select_columns(sql: &str) -> Vec<String> { // 函数 extract_select_columns
    let upper = sql.to_uppercase(); // 变量 upper
    let Some(sel) = upper.find("SELECT") else { return Vec::new() }; // 变量 Some
    let Some(from) = upper.find("FROM") else { return Vec::new() }; // 变量 Some
    if from <= sel { return Vec::new(); } // 条件分支
    let raw = &sql[sel + 6..from]; // 变量 raw
    raw.split(',') // 语句
        .map(|c| { // 语句
            let c = c.trim(); // 变量 c
            let c = c.split_whitespace().next().unwrap_or(c); // 变量 c
            c.trim_matches(|ch: char| ch == '(' || ch == ')').to_string() // 语句
        }) // 语句
        .filter(|c| !c.is_empty() && c != "*" && !c.contains('(')) // 语句
        .collect() // 语句
} // 块结束

// ---------------- Model (conceptual / logical / physical) ----------------
fn model(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) { // 函数 model
    let pal = app.pal.clone(); // 变量 pal
    let mut reload = false; // 变量 reload
    let mut export_sql = false; // 变量 export_sql
    let mut compare = false; // 变量 compare
    let mut sync_db = false; // 变量 sync_db
    let mut import_sql = false; // 变量 import_sql
    let conn_id = match &app.tabs[idx].kind { TabKind::Model(m) => m.conn_id.clone(), _ => return }; // 变量 conn_id
    header(app, ui, &conn_id, "\u{25A6} 模型设计器"); // 调用 header
    let conns: Vec<(String, String)> = app.store.connections.iter().map(|c| (c.id.clone(), c.name.clone())).collect(); // 变量 conns
    crate::ui::card(&app).show(ui, |ui| { // 调用 card
        let d = match &mut app.tabs[idx].kind { TabKind::Model(m) => m, _ => return }; // 变量 d
        ui.horizontal(|ui| { // 语句
            ui.label("视图:"); // 语句
            for v in ["概念模型", "逻辑模型", "物理模型"] { // 循环
                if ui.selectable_label(d.view == v, v).clicked() { d.view = v.to_string(); } // 条件分支
            } // 块结束
        }); // 语句
        ui.add_space(4.0); // 语句
        ui.horizontal_wrapped(|ui| { // 语句
            if ui.button("\u{1F504} 反向工程（从数据库）").clicked() { reload = true; } // 条件分支
            if ui.button("\u{2B06} 导出模型为 SQL").clicked() { export_sql = true; } // 条件分支
            if ui.button("\u{1F504} 从模型同步到数据库").clicked() { sync_db = true; } // 条件分支
            if ui.button("\u{21C4} 模型对比").clicked() { compare = true; } // 条件分支
        }); // 语句
        ui.horizontal(|ui| { // 语句
            ui.label("对比目标连接:"); // 语句
            egui::ComboBox::from_id_source("cmpsel").selected_text(conns.iter().find(|(id, _)| id == &d.cmp_conn).map(|(_, n)| n.clone()).unwrap_or("选择".into())) // 调用 from_id_source
                .show_ui(ui, |ui| { for (id, name) in &conns { if ui.selectable_label(&d.cmp_conn == id, name).clicked() { d.cmp_conn = id.clone(); } } }); // 语句
        }); // 语句
        ui.horizontal(|ui| { // 语句
            ui.label("SQL 文件:"); // 语句
            ui.add(egui::TextEdit::singleline(&mut d.sql_path).desired_width(360.0)); // 语句
            if ui.button("导入 SQL 到模型").clicked() { import_sql = true; } // 条件分支
        }); // 语句
        if !d.status.is_empty() { ui.add_space(4.0); ui.colored_label(pal.success, &d.status); } // 条件分支
    }); // 语句
    ui.add_space(6.0); // 语句
    let (view, tables, fks, diff) = match &app.tabs[idx].kind { TabKind::Model(m) => (m.view.clone(), m.tables.clone(), m.fks.clone(), m.diff.clone()), _ => return }; // 匹配分支
    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| { // 调用 vertical
        for (tname, cols) in &tables { // 循环
            egui::Frame::none().fill(pal.panel).stroke(Stroke::new(1.0, pal.accent)).rounding(egui::Rounding::same(8.0)).inner_margin(egui::Margin::same(10.0)).show(ui, |ui| { // 调用 none
                ui.set_width(ui.available_width().min(620.0)); // 语句
                ui.horizontal(|ui| { // 语句
                    ui.label(RichText::new(tname).strong().size(13.5).color(pal.accent)); // 语句
                    ui.label(RichText::new(format!("· {} 字段", cols.len())).size(11.0).color(pal.text_dim)); // 语句
                }); // 语句
                ui.separator(); // 语句
                let body = match view.as_str() { // 变量 body
                    "概念模型" => format!("实体（{} 个字段）", cols.len()), // 匹配分支
                    "物理模型" => cols.join("   \u{00B7}   "), // 匹配分支
                    _ => cols.iter().map(|c| c.split_whitespace().next().unwrap_or(c.as_str()).to_string()).collect::<Vec<_>>().join("   \u{00B7}   "), // 匹配分支
                }; // 块结束
                ui.add(egui::Label::new(RichText::new(body).size(11.0).color(pal.text))); // 语句
            }); // 语句
            ui.add_space(6.0); // 语句
        } // 块结束
        ui.add_space(8.0); // 语句
        if !fks.is_empty() { // 条件分支
            ui.label(RichText::new("关系 (外键)").strong()); // 语句
            for (child, cc, parent, pc) in &fks { // 循环
                ui.label(RichText::new(format!("\u{2192} {}.{}  =  {}.{}", child, cc, parent, pc)).size(11.5).color(pal.text_dim)); // 语句
            } // 块结束
        } // 块结束
        if !diff.is_empty() { // 条件分支
            ui.add_space(8.0); // 语句
            ui.label(RichText::new("模型对比结果").strong()); // 语句
            for line in &diff { ui.label(RichText::new(line).size(11.5)); } // 循环
        } // 块结束
    }); // 语句

    if reload { // 条件分支
        let (tables, fks) = app.reload_model(&conn_id); // 调用 let
        let msg = format!("已反向工程: {} 张表", tables.len()); // 变量 msg
        if let TabKind::Model(m) = &mut app.tabs[idx].kind { m.tables = tables; m.fks = fks; m.status = msg.clone(); } // 条件分支
        app.log(msg); // 语句
    } // 块结束
    if export_sql { // 条件分支
        let (path, tables) = match &app.tabs[idx].kind { TabKind::Model(m) => (m.sql_path.clone(), m.tables.clone()), _ => return }; // 匹配分支
        let mut sql = String::from("-- 由 Navicat-RS 模型导出\n"); // 变量 sql
        for (t, cols) in &tables { // 循环
            let defs: Vec<String> = cols.iter().map(|c| { // 变量 defs
                let mut parts = c.splitn(2, ' '); // 变量 parts
                let name = parts.next().unwrap_or("c"); // 变量 name
                let ty = parts.next().unwrap_or("TEXT"); // 变量 ty
                format!("  {} {}", db::quote_ident(name), ty) // 语句
            }).collect(); // 语句
            sql.push_str(&format!("CREATE TABLE {} (\n{}\n);\n", db::quote_ident(t), defs.join(",\n"))); // 语句
        } // 块结束
        let msg = match std::fs::write(&path, &sql) { Ok(_) => format!("已导出模型 SQL -> {path}"), Err(e) => format!("写入失败: {e}") }; // 变量 msg
        if let TabKind::Model(m) = &mut app.tabs[idx].kind { m.status = msg.clone(); } // 条件分支
        app.log(msg); // 语句
    } // 块结束
    if compare { // 条件分支
        let (cmp, tables) = match &app.tabs[idx].kind { TabKind::Model(m) => (m.cmp_conn.clone(), m.tables.clone()), _ => return }; // 匹配分支
        let other = app.reload_model(&cmp).0; // 变量 other
        let mut diff: Vec<String> = Vec::new(); // 变量 diff
        let names_a: Vec<&String> = tables.iter().map(|(t, _)| t).collect(); // 变量 names_a
        let names_b: Vec<&String> = other.iter().map(|(t, _)| t).collect(); // 变量 names_b
        for t in &names_a { if !names_b.contains(t) { diff.push(format!("仅在源存在: 表 {}", t)); } } // 循环
        for t in &names_b { if !names_a.contains(t) { diff.push(format!("仅在目标存在: 表 {}", t)); } } // 循环
        for (t, cols) in &tables { // 循环
            if let Some((_, ocols)) = other.iter().find(|(ot, _)| ot == t) { // 条件分支
                for c in cols { if !ocols.contains(c) { diff.push(format!("表 {} 列差异(源): {}", t, c)); } } // 循环
                for c in ocols { if !cols.contains(c) { diff.push(format!("表 {} 列差异(目标): {}", t, c)); } } // 循环
            } // 块结束
        } // 块结束
        if diff.is_empty() { diff.push("两个模型结构一致（表与列相同）。".into()); } // 条件分支
        if let TabKind::Model(m) = &mut app.tabs[idx].kind { m.diff = diff.clone(); m.status = "模型对比完成".into(); } // 条件分支
    } // 块结束
    if sync_db { // 条件分支
        app.open_sync(conn_id.clone()); // 语句
    } // 块结束
    if import_sql { // 条件分支
        let path = match &app.tabs[idx].kind { TabKind::Model(m) => m.sql_path.clone(), _ => return }; // 变量 path
        let script = match std::fs::read_to_string(&path) { Ok(s) => s, Err(e) => { if let TabKind::Model(m) = &mut app.tabs[idx].kind { m.status = format!("读取失败: {e}"); } return; } }; // 变量 script
        if let Some(e) = app.engines.get(&conn_id) { let _ = e.run_script(&script); } // 条件分支
        let (tables, fks) = app.reload_model(&conn_id); // 调用 let
        let msg = format!("已导入 SQL 到模型: 现有 {} 张表", tables.len()); // 变量 msg
        if let TabKind::Model(m) = &mut app.tabs[idx].kind { m.tables = tables; m.fks = fks; m.status = msg.clone(); } // 条件分支
        app.log(msg); // 语句
    } // 块结束
} // 块结束

// ---------------- Find in Database ----------------
fn find(app: &mut NavicatApp, ui: &mut egui::Ui, idx: usize) { // 函数 find
    let pal = app.pal.clone(); // 变量 pal
    let mut go = false; // 变量 go
    let conn_id = match &app.tabs[idx].kind { TabKind::Find(f) => f.conn_id.clone(), _ => return }; // 变量 conn_id
    header(app, ui, &conn_id, "\u{2315} 查找于数据库"); // 调用 header
    crate::ui::card(&app).show(ui, |ui| { // 调用 card
        let d = match &mut app.tabs[idx].kind { TabKind::Find(f) => f, _ => return }; // 变量 d
        ui.horizontal(|ui| { // 语句
            ui.label("查找内容:"); // 语句
            ui.add(egui::TextEdit::singleline(&mut d.needle).desired_width(320.0).hint_text("在全部表的文本/数值列中搜索")); // 语句
            if ui.button(RichText::new("\u{1F50E} 查找").color(pal.contrast_text(pal.accent)).background_color(pal.accent)).clicked() { go = true; } // 条件分支
        }); // 语句
        if !d.status.is_empty() { ui.add_space(4.0); ui.colored_label(pal.success, &d.status); } // 条件分支
        ui.add_space(4.0); // 语句
        egui::ScrollArea::both().max_height(480.0).show(ui, |ui| { // 调用 both
            egui::Grid::new("findgrid").striped(true).num_columns(3).spacing([14.0, 4.0]).show(ui, |ui| { // 调用 new
                ui.label(RichText::new("表").strong()); ui.label(RichText::new("列").strong()); ui.label(RichText::new("值").strong()); ui.end_row(); // 语句
                for (t, c, v) in &d.hits { ui.label(RichText::new(t).size(11.5)); ui.label(RichText::new(c).size(11.5)); ui.label(RichText::new(v).size(11.5)); ui.end_row(); } // 循环
            }); // 语句
        }); // 语句
    }); // 语句
    if go { // 条件分支
        let needle = match &app.tabs[idx].kind { TabKind::Find(f) => f.needle.clone(), _ => return }; // 变量 needle
        let hits = if let Some(e) = app.engines.get(&conn_id) { find_in_db(e, &needle) } else { Vec::new() }; // 变量 hits
        let msg = format!("找到 {} 处匹配", hits.len()); // 变量 msg
        if let TabKind::Find(f) = &mut app.tabs[idx].kind { f.hits = hits; f.status = msg.clone(); } // 条件分支
        app.log(msg); // 语句
    } // 块结束
} // 块结束

pub fn find_in_db(e: &crate::db::SqliteEngine, needle: &str) -> Vec<(String, String, String)> { // 函数 find_in_db
    let mut hits = Vec::new(); // 变量 hits
    if needle.trim().is_empty() { return hits; } // 条件分支
    let tables: Vec<String> = e.objects().into_iter().filter(|o| o.kind == crate::model::ObjKind::Table).map(|o| o.name).collect(); // 变量 tables
    let esc = needle.replace('\'', "''"); // 变量 esc
    for t in tables { // 循环
        for c in e.columns(&t) { // 循环
            let sql = format!( // 变量 sql
                "SELECT CAST({} AS TEXT) FROM {} WHERE CAST({} AS TEXT) LIKE '%{}%' LIMIT 20", // 语句
                db::quote_ident(&c.name), db::quote_ident(&t), db::quote_ident(&c.name), esc // 调用 quote_ident
            ); // 块结束
            let r = e.query(&sql); // 变量 r
            if r.error.is_none() { // 条件分支
                for row in r.rows { // 循环
                    if let Some(v) = row.into_iter().next().flatten() { // 条件分支
                        hits.push((t.clone(), c.name.clone(), v)); // 语句
                    } // 块结束
                } // 块结束
            } // 块结束
            if hits.len() >= 500 { return hits; } // 条件分支
        } // 块结束
    } // 块结束
    hits // 语句
} // 块结束

// ---------------- Settings ----------------
fn settings(app: &mut NavicatApp, ui: &mut egui::Ui) { // 函数 settings
    let pal = app.pal.clone(); // 变量 pal
    egui::Frame::none().fill(pal.panel).inner_margin(egui::Margin::symmetric(8.0, 5.0)).show(ui, |ui| { // 调用 none
        ui.label(RichText::new("\u{1F527} 设置").strong().size(14.0)); // 语句
    }); // 语句
    ui.separator(); // 语句
    crate::ui::card(app).show(ui, |ui| { // 调用 card
        ui.horizontal(|ui| { // 语句
            ui.label("主题:"); // 语句
            let cur = crate::theme::label_of(&app.theme); // 变量 cur
            egui::ComboBox::from_id_salt("theme_select") // 调用 from_id_salt
                .selected_text(cur) // 语句
                .show_ui(ui, |ui| { // 语句
                    for t in crate::theme::THEMES { // 循环
                        if ui.selectable_label(app.theme == t.key, t.label).clicked() { // 条件分支
                            let key = t.key; // 变量 key
                            app.set_theme(ui.ctx(), key); // 语句
                        } // 块结束
                    } // 块结束
                }); // 语句
        }); // 语句
        ui.add_space(8.0); // 语句
        ui.label("每页行数:"); // 语句
        let mut ps = app.store.settings.page_size as i64; // 变量 ps
        if ui.add(egui::DragValue::new(&mut ps).range(10..=5000)).changed() { app.store.settings.page_size = ps as usize; app.store.save(); } // 条件分支
        ui.label("树形视图缩放:"); // 语句
        if ui.add(egui::Slider::new(&mut app.store.settings.tree_zoom, 0.7..=1.9).text("倍")).changed() { app.store.save(); } // 条件分支
        ui.checkbox(&mut app.store.settings.confirm_delete, "删除前确认"); // 语句
        ui.checkbox(&mut app.store.settings.auto_commit, "自动提交（关闭后查询编辑器将整段脚本包在事务中执行）") // 语句
            .on_hover_text("开启：每条语句独立自动提交；关闭：一次运行的整段脚本作为单个事务原子执行"); // 语句
        app.store.save(); // 语句
        ui.add_space(8.0); // 语句
        ui.label(format!("数据目录: {}", crate::store::Store::dir().to_string_lossy())); // 语句
        ui.label(format!("配置文件: {}", crate::store::Store::path().to_string_lossy())); // 语句
        ui.add_space(8.0); // 语句
        if ui.button("立即保存设置").clicked() { app.store.save(); app.log("设置已保存"); } // 条件分支
    }); // 语句
} // 块结束
